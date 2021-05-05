#![no_std]

use bitfield::Bit;

use cortex_m::interrupt::Mutex;
use heapless::Vec;
use usb_device::bus::PollResult;
use usb_device::class_prelude::{EndpointAddress, EndpointType};
use usb_device::UsbDirection;
use usb_device::UsbError::{EndpointMemoryOverflow, EndpointOverflow, InvalidEndpoint};

use m451::{CLK, SYS, USBD};
use m451::usbd::ep::cfg::STATE_A;
use m451::usbd::epsts::EPSTS0_A::{IN_ACK, OUT_DATA0_ACK, SETUP_ACK};
use core::cell::RefCell;

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

            if eps.len() == 8 {
                return Err(EndpointOverflow);
            }

            // Iterate through existing endpoints in order to:
            // 1) make sure the requested address (if provided) is not in use.
            // 2) calculate the BUFSEG offset in SRAM for this new endpoint.
            // 3) determine which endpoint numbers are still available for automatic allocation.
            let mut buf_tot = 0;
            let mut in_free = 0xFFFFu16;
            let mut out_free = 0xFFFFu16;

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

            let free = match ep_dir {
                UsbDirection::In => in_free,
                UsbDirection::Out => out_free,
            };

            let addr = EndpointAddress::from_parts(
                (0..16usize).find(|idx| free.bit(*idx)).unwrap(), ep_dir
            );

            // Safe to unwrap here - we did bounds check as part of Err(EndpointOverflow) check.
            eps.push(Endpoint{addr, buf_sz: max_packet_size, typ: ep_type }).unwrap();

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
            })
        });
    }

    fn reset(&self) {
        cortex_m::interrupt::free(|cs| {
            let usbd = self.usbd.borrow(cs).borrow();
            let eps = self.eps.borrow(cs).borrow();

            usbd.stbufseg.reset();
            for reg in usbd.ep.iter() {
                reg.cfg.reset();
                reg.cfgp.reset();
                reg.bufseg.reset();
                reg.mxpld.reset();
            }

            let mut buf_idx = 8u16; // 8byte setup token at 0x00.

            for (ep, ep_regs) in eps.iter().zip(&usbd.ep) {
                ep_regs.cfg.modify(|_, mut w| {
                    // Unsafe because field is 4bits wide, I guess?
                    w = unsafe { w.epnum().bits(ep.addr.index() as u8) };

                    if ep.typ == EndpointType::Isochronous {
                        w = w.isoch().set_bit();
                    }
                    w.state().variant(if ep.addr.is_in() { STATE_A::IN } else { STATE_A::OUT })
                });

                ep_regs.bufseg.write(|w| unsafe { w.bits(buf_idx.into()) });
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

    fn write(&self, _ep_addr: EndpointAddress, _buf: &[u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn read(&self, _ep_addr: EndpointAddress, _buf: &mut [u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn set_stalled(&self, _ep_addr: EndpointAddress, _stalled: bool) {
        todo!()
    }

    fn is_stalled(&self, _ep_addr: EndpointAddress) -> bool {
        todo!()
    }

    fn suspend(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn poll(&self) -> PollResult {
        cortex_m::interrupt::free(|cs| {
            let usbd = self.usbd.borrow(cs).borrow();

            if usbd.vbusdet.read().vbusdet().bit_is_clear() {
                // Device isn't plugged in, nothing interesting happening here, folks.
                return PollResult::None;
            }

            if usbd.attr.read().suspend().bit_is_set() {
                return PollResult::Suspend;
            }

            if usbd.intsts.read().busif().bit_is_set() {
                if usbd.attr.read().usbrst().bit_is_set() {
                    usbd.intsts.modify(|_, w| w.busif().set_bit());
                    return PollResult::Reset;
                }
                if usbd.attr.read().resume().bit_is_set() {
                    usbd.intsts.modify(|_, w| w.busif().set_bit());
                    return PollResult::Resume;
                }
                // warn: "BUSIF interrupt but no corresponding event detected."
            }

            if usbd.intsts.read().usbif().bit_is_set() {
                let mut ep_in_complete = 0;
                let mut ep_out = 0;
                let mut ep_setup = 0;

                let ep_sts = usbd.epsts.read();

                for (idx, sts_reg) in [
                    ep_sts.epsts0().variant(),
                    ep_sts.epsts1().variant(),
                    ep_sts.epsts2().variant(),
                    ep_sts.epsts3().variant(),
                    ep_sts.epsts4().variant(),
                    ep_sts.epsts5().variant(),
                    ep_sts.epsts6().variant(),
                    ep_sts.epsts7().variant(),
                ].iter().enumerate() {
                    if let Some(IN_ACK) = sts_reg {
                        ep_in_complete |= 1 << idx;
                    } else if let Some(OUT_DATA0_ACK) = sts_reg {
                        ep_out |= 1 << idx;
                    } else if let Some(SETUP_ACK) = sts_reg {
                        ep_setup |= 1 << idx;
                    }
                }
                return PollResult::Data {ep_in_complete, ep_out, ep_setup};
            }

            PollResult::None
        })
    }
}
