use bitfield::Bit;

use cortex_m::interrupt::Mutex;
use heapless::Vec;
use usb_device::bus::PollResult;
use usb_device::class_prelude::{EndpointAddress, EndpointType};
use usb_device::UsbDirection;
use usb_device::UsbError::{EndpointMemoryOverflow, EndpointOverflow, InvalidEndpoint, BufferOverflow, WouldBlock};

use m451::{CLK, SYS, USBD};
use m451::usbd::ep::cfg::STATE_A;
use m451::usbd::epsts::EPSTS0_A::{IN_ACK, OUT_DATA0_ACK, OUT_DATA1_ACK, SETUP_ACK};
use core::cell::RefCell;
use m451::generic::Reg;
use m451::usbd::intsts::INTSTS_SPEC;

pub fn unlock_registers<F, R>(sys: &mut SYS, f: F) -> R
    where F: FnOnce(&mut SYS) -> R  {
    while sys.sys_reglctl.read().reglctl().is_0() {
        unsafe {
            sys.sys_reglctl.write(|w| w.reglctl().bits(0x59));
            sys.sys_reglctl.write(|w| w.reglctl().bits(0x16));
            sys.sys_reglctl.write(|w| w.reglctl().bits(0x88));
        }
    }

    let r = f(sys);

    unsafe {
        sys.sys_reglctl.write(|w| w.reglctl().bits(0));
    }

    r
}

pub struct UsbBus {
    usbd: Mutex<RefCell<USBD>>,
    eps: Mutex<RefCell<Vec<Endpoint, 8>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Endpoint {
    addr: EndpointAddress,
    typ: EndpointType,
    buf_sz: u16,
    write_pending: bool,
}

impl UsbBus {
    pub fn new(_sys: &mut SYS, _clk: &mut CLK, usbd: USBD) -> UsbBus {
        // TODO: clock stuff
        // unlock_registers(sys, |sys| {
        //     // TRM 6.17.4
        //     sys.sys_usbphy.modify(|r, w| w.usbrole()._0());
        //     clk.clk_apbclk0.modify(|r, w| w.usbdcken()._1());
        // });
        // clk.clk_clkdiv0.modify(|r, w| w.usbdiv())

        UsbBus {
            usbd: Mutex::new(RefCell::new(usbd)),
            eps: Mutex::new(RefCell::new(Vec::new())),
        }
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    fn alloc_ep(&mut self, ep_dir: UsbDirection, ep_addr: Option<EndpointAddress>, ep_type: EndpointType,
                max_packet_size: u16, _interval: u8)  -> usb_device::Result<EndpointAddress> {
        cortex_m::interrupt::free(|cs| {
            let mut eps = self.eps.borrow(cs).borrow_mut();

            if eps.is_full() {
                return Err(EndpointOverflow);
            }

            // Iterate through existing endpoints in order to:
            // 1) make sure the requested address (if provided) is not in use.
            // 2) calculate the BUFSEG offset in SRAM for this new endpoint.
            // 3) determine which endpoint numbers are still available for automatic allocation.
            let mut buf_tot = 0;
            let mut in_free = 0xFFFEu16;
            let mut out_free = 0xFFFEu16;

            for ep in eps.iter() {
                if Some(ep.addr) == ep_addr {
                    return Err(InvalidEndpoint);
                }
                buf_tot += ep.buf_sz;
                (if ep.addr.is_in() { &mut in_free } else { &mut out_free }).set_bit(ep.addr.index(), false);
            }

            if buf_tot + max_packet_size > 512 {
                return Err(EndpointMemoryOverflow);
            }

            let addr = match ep_addr {
                Some(x) => x,
                None => {
                    let free = match ep_dir {
                        UsbDirection::In => in_free,
                        UsbDirection::Out => out_free,
                    };

                    EndpointAddress::from_parts(
                        (0..16usize).find(|idx| free.bit(*idx)).unwrap(), ep_dir
                    )
                }
            };

            // Safe to unwrap here - we did bounds check as part of Err(EndpointOverflow) check.
            eps.push(Endpoint{addr, buf_sz: max_packet_size, typ: ep_type, write_pending: false }).unwrap();

            Ok(addr)
        })
    }

    fn enable(&mut self) {
        cortex_m::interrupt::free(|cs| {
            let usbd = self.usbd.borrow(cs).borrow();
            usbd.attr.write(|w| {
                w
                    .phyen()._1()
                    .usben()._1()
                    .dppuen()._1()
                    .pwrdn()._1()
                    .bytem()._1()
            });
            usbd.se0.reset();

            usbd.inten.write(|w| {
                w
                    .busien().set_bit()
                    .usbien().set_bit()
                    .vbdetien().set_bit()
            });
        });
    }

    fn reset(&self) {
        cortex_m::interrupt::free(|cs| {
            let usbd = self.usbd.borrow(cs).borrow();
            let eps = self.eps.borrow(cs).borrow();

            usbd.faddr.reset();
            usbd.stbufseg.reset();
            for reg in usbd.ep.iter() {
                reg.cfg.reset();
                reg.cfgp.reset();
                reg.bufseg.reset();
            }

            let mut buf_idx = 8u16; // 8byte setup token at 0x00.

            for (ep, ep_regs) in eps.iter().zip(&usbd.ep) {
                ep_regs.cfg.modify(|_, mut w| {
                    if ep.typ == EndpointType::Control {
                        w = w.cstall().set_bit();
                    }
                    // Unsafe because field is 4bits wide, I guess?
                    w = unsafe { w.epnum().bits(ep.addr.index() as u8) };

                    if ep.typ == EndpointType::Isochronous {
                        w = w.isoch().set_bit();
                    }
                    w.state().variant(if ep.addr.is_in() { STATE_A::IN } else { STATE_A::OUT })
                });

                ep_regs.bufseg.write(|w| unsafe { w.bits(buf_idx.into()) });
                if ep.addr.is_out() {
                    ep_regs.mxpld.write(|w| unsafe { w.mxpld().bits(ep.buf_sz) });
                }
                buf_idx += ep.buf_sz;
            }

            usbd.se0.write(|w| w.se0().clear_bit());
        });
    }

    fn set_device_address(&self, addr: u8) {
        cortex_m::interrupt::free(|cs| {
            self.usbd.borrow(cs).borrow().faddr.write(|w| unsafe { w.faddr().bits(addr.into()) });
        });
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> usb_device::Result<usize> {
        if !ep_addr.is_in() {
            return Err(InvalidEndpoint);
        }
        cortex_m::interrupt::free(|cs| {
            if let Some((idx, ep)) = self.eps.borrow(cs).borrow_mut().iter_mut().enumerate().find(|(_, ep)| ep.addr == ep_addr) {
                if ep.write_pending {
                    return Err(WouldBlock);
                }

                let regs = &self.usbd.borrow(cs).borrow().ep[idx];
                if buf.len() > ep.buf_sz.into() {
                    return Err(BufferOverflow);
                }

                let sram = unsafe {
                    core::slice::from_raw_parts_mut(
                        (USBD::ptr() as *mut u8).offset(0x100 + regs.bufseg.read().bits() as isize),
                        buf.len()
                    )
                };
                sram.copy_from_slice(buf);
                regs.mxpld.write(|w| unsafe { w.mxpld().bits(ep.buf_sz) });
                ep.write_pending = true;

                Ok(buf.len())
            } else {
                return Err(InvalidEndpoint);
            }
        })
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize> {
        if !ep_addr.is_out() {
            return Err(InvalidEndpoint);
        }
        cortex_m::interrupt::free(|cs| {
            if let Some((idx, ep)) = self.eps.borrow(cs).borrow().iter().enumerate().find(|(_, ep)| ep.addr == ep_addr) {
                let ep_sts = self.usbd.borrow(cs).borrow().epsts.read();
                let mut is_setup = false;
                match [
                    ep_sts.epsts0().variant(),
                    ep_sts.epsts1().variant(),
                    ep_sts.epsts2().variant(),
                    ep_sts.epsts3().variant(),
                    ep_sts.epsts4().variant(),
                    ep_sts.epsts5().variant(),
                    ep_sts.epsts6().variant(),
                    ep_sts.epsts7().variant(),
                ][idx] {
                    Some(OUT_DATA0_ACK)|Some(OUT_DATA1_ACK) => {},
                    Some(SETUP_ACK) => is_setup = true,
                    _ => return Err(WouldBlock),
                }

                let regs = &self.usbd.borrow(cs).borrow().ep[idx];
                let (len, offset): (u32, isize) =
                    if is_setup { (8, 0) }
                    else { (regs.mxpld.read().bits(), regs.bufseg.read().bits() as isize) };
                // (regs.mxpld.read().bits(), regs.bufseg.read().bits() as isize);

                if len > buf.len() as u32 {
                    return Err(BufferOverflow)
                }

                let sram = unsafe {
                    core::slice::from_raw_parts_mut(
                        (USBD::ptr() as *mut u8).offset(0x100 + offset),
                        len as usize
                    )
                };

                buf.copy_from_slice(sram);

                unsafe { *regs.mxpld.as_ptr() = ep.buf_sz.into(); }

                let usbd = self.usbd.borrow(cs).borrow();
                clear_epevt(&usbd.intsts, idx);

                Ok(sram.len())
            } else {
                Err(InvalidEndpoint)
            }
        })
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        cortex_m::interrupt::free(|cs| {
            if let Some((idx, _)) = self.eps.borrow(cs).borrow().iter().enumerate().find(|(_, ep)| ep.addr == ep_addr) {
                self.usbd.borrow(cs).borrow().ep[idx].cfgp.modify(|_, w| w.sstall().bit(stalled));
            }
        });
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        cortex_m::interrupt::free(|cs| {
            if let Some((idx, _)) = self.eps.borrow(cs).borrow().iter().enumerate().find(|(_, ep)| ep.addr == ep_addr) {
                self.usbd.borrow(cs).borrow().ep[idx].cfgp.read().sstall().bit()
            } else {
                false
            }
        })
    }

    fn suspend(&self) {
        cortex_m::interrupt::free(|cs| {
            self.usbd.borrow(cs).borrow().attr.modify(|_, w| w.phyen().clear_bit());
        });
    }

    fn resume(&self) {
        cortex_m::interrupt::free(|cs| {
            self.usbd.borrow(cs).borrow().attr.modify(|_, w|
                w.usben().set_bit().phyen().set_bit());
        });
    }

    fn poll(&self) -> PollResult {
        cortex_m::interrupt::free(|cs| {
            let usbd = self.usbd.borrow(cs).borrow();

            let intsts = usbd.intsts.read();

            let plugged_in = usbd.vbusdet.read().vbusdet().bit_is_set();

            if intsts.vbdetif().bit_is_set() {
                let pr = if plugged_in { PollResult::Resume } else { PollResult::Suspend };
                usbd.intsts.write(|w| w.vbdetif().set_bit());
                return pr;
            }

            // if !plugged_in {
            //     return PollResult::None;
            // }

            if intsts.nevwkif().bit_is_set() {
                usbd.intsts.write(|w| w.nevwkif().set_bit());
            }

            if intsts.busif().bit_is_set() {
                usbd.intsts.modify(|_, w| w.busif().set_bit());

                return if usbd.attr.read().usbrst().bit_is_set() {
                    PollResult::Reset
                } else if usbd.attr.read().resume().bit_is_set() {
                    PollResult::Resume
                } else if usbd.attr.read().suspend().bit_is_set() {
                    PollResult::Suspend
                } else {
                    unreachable!("BUSIF interrupt but no corresponding event detected.");
                };
            }

            if usbd.intsts.read().usbif().bit_is_set() {
                let mut ep_in_complete = 0;
                let mut ep_out = 0;
                let mut ep_setup = 0;

                if usbd.intsts.read().setup().bit_is_set() {
                    ep_out.set_bit(0, true);
                    ep_setup.set_bit(0, true);
                }

                let ep_sts = usbd.epsts.read();
                for (idx, (sts_reg, ep)) in [
                    ep_sts.epsts0().variant(),
                    ep_sts.epsts1().variant(),
                    ep_sts.epsts2().variant(),
                    ep_sts.epsts3().variant(),
                    ep_sts.epsts4().variant(),
                    ep_sts.epsts5().variant(),
                    ep_sts.epsts6().variant(),
                    ep_sts.epsts7().variant(),
                ].iter().zip(self.eps.borrow(cs).borrow_mut().iter_mut()).enumerate() {
                    let epnum = ep.addr.index();
                    if let Some(IN_ACK) = sts_reg {
                        // Clear the IN ACK event.
                        ep.write_pending = false;
                        clear_epevt(&usbd.intsts, idx);

                        ep_in_complete.set_bit(epnum, true);
                    } else if let Some(OUT_DATA0_ACK) = sts_reg {
                        ep_out.set_bit(epnum, true);
                    } else if let Some(OUT_DATA1_ACK) = sts_reg {
                        ep_out.set_bit(epnum, true);
                    } else if let Some(SETUP_ACK) = sts_reg {
                        ep_setup.set_bit(epnum, true);
                    }
                }

                return PollResult::Data { ep_in_complete, ep_out, ep_setup };
            }

            PollResult::None
        })
    }
}

fn clear_epevt(reg: &Reg<INTSTS_SPEC>, idx: usize) {
    match idx {
        0 => reg.write(|w| w.epevt0().set_bit()),
        1 => reg.write(|w| w.epevt1().set_bit()),
        2 => reg.write(|w| w.epevt2().set_bit()),
        3 => reg.write(|w| w.epevt3().set_bit()),
        4 => reg.write(|w| w.epevt4().set_bit()),
        5 => reg.write(|w| w.epevt5().set_bit()),
        6 => reg.write(|w| w.epevt6().set_bit()),
        7 => reg.write(|w| w.epevt7().set_bit()),
        _ => unreachable!(),
    }
}
