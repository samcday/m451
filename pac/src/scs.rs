#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - SysTick Control and Status Register"]
    pub syst_ctrl: crate::Reg<syst_ctrl::SYST_CTRL_SPEC>,
    #[doc = "0x14 - SysTick Reload Value Register"]
    pub syst_load: crate::Reg<syst_load::SYST_LOAD_SPEC>,
    #[doc = "0x18 - SysTick Current Value Register"]
    pub syst_val: crate::Reg<syst_val::SYST_VAL_SPEC>,
    _reserved3: [u8; 3304usize],
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: crate::Reg<aircr::AIRCR_SPEC>,
    #[doc = "0xd10 - System Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: crate::Reg<shpr1::SHPR1_SPEC>,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
}
#[doc = "SYST_CTRL register accessor: an alias for `Reg<SYST_CTRL_SPEC>`"]
pub type SYST_CTRL = crate::Reg<syst_ctrl::SYST_CTRL_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_ctrl;
#[doc = "SYST_LOAD register accessor: an alias for `Reg<SYST_LOAD_SPEC>`"]
pub type SYST_LOAD = crate::Reg<syst_load::SYST_LOAD_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_load;
#[doc = "SYST_VAL register accessor: an alias for `Reg<SYST_VAL_SPEC>`"]
pub type SYST_VAL = crate::Reg<syst_val::SYST_VAL_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod syst_val;
#[doc = "ICSR register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "AIRCR register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "SHPR1 register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
