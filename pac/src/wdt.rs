#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT Control Register"]
    pub wdt_ctl: crate::Reg<wdt_ctl::WDT_CTL_SPEC>,
    #[doc = "0x04 - WDT Alternative Control Register"]
    pub wdt_altctl: crate::Reg<wdt_altctl::WDT_ALTCTL_SPEC>,
}
#[doc = "WDT_CTL register accessor: an alias for `Reg<WDT_CTL_SPEC>`"]
pub type WDT_CTL = crate::Reg<wdt_ctl::WDT_CTL_SPEC>;
#[doc = "WDT Control Register"]
pub mod wdt_ctl;
#[doc = "WDT_ALTCTL register accessor: an alias for `Reg<WDT_ALTCTL_SPEC>`"]
pub type WDT_ALTCTL = crate::Reg<wdt_altctl::WDT_ALTCTL_SPEC>;
#[doc = "WDT Alternative Control Register"]
pub mod wdt_altctl;
