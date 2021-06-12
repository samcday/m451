use m451::WWDT;
use embedded_hal::watchdog::{Enable, Watchdog};
use core::convert::Infallible;

// Writing this value to the down-counting WWDT_RLDCNT register resets it to 0x3F.
const WWDT_RLDCNT_MAGIC: u32 = 0x5aa5;

pub struct WindowWatchdog {
    wwdt: WWDT,
}

impl WindowWatchdog {
    pub fn new(wwdt: WWDT) -> WindowWatchdog { // Soon: mutref to Clock controller to enable the WWDT peripheral clock.
        WindowWatchdog { wwdt }
    }
}

impl Enable for WindowWatchdog {
    type Error = Infallible;
    type Time = embedded_time::duration::Nanoseconds;
    type Target = WindowWatchdog;

    fn try_start<T>(self, period: T) -> Result<Self::Target, Self::Error> where T: Into<Self::Time> {
        self.wwdt.wwdt_ctl.write(|w| {
            w.wwdten().set_bit()
        });
    }
}

impl Watchdog for WindowWatchdog {
    type Error = Infallible;

    fn try_feed(&mut self) -> Result<(), Self::Error> {

    }
}