#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WWDT Reload Counter Register"]
    pub wwdt_rldcnt: crate::Reg<wwdt_rldcnt::WWDT_RLDCNT_SPEC>,
    #[doc = "0x04 - WWDT Control Register"]
    pub wwdt_ctl: crate::Reg<wwdt_ctl::WWDT_CTL_SPEC>,
    #[doc = "0x08 - WWDT Status Register"]
    pub wwdt_status: crate::Reg<wwdt_status::WWDT_STATUS_SPEC>,
    #[doc = "0x0c - WWDT Counter Value Register"]
    pub wwdt_cnt: crate::Reg<wwdt_cnt::WWDT_CNT_SPEC>,
}
#[doc = "WWDT_RLDCNT register accessor: an alias for `Reg<WWDT_RLDCNT_SPEC>`"]
pub type WWDT_RLDCNT = crate::Reg<wwdt_rldcnt::WWDT_RLDCNT_SPEC>;
#[doc = "WWDT Reload Counter Register"]
pub mod wwdt_rldcnt;
#[doc = "WWDT_CTL register accessor: an alias for `Reg<WWDT_CTL_SPEC>`"]
pub type WWDT_CTL = crate::Reg<wwdt_ctl::WWDT_CTL_SPEC>;
#[doc = "WWDT Control Register"]
pub mod wwdt_ctl;
#[doc = "WWDT_STATUS register accessor: an alias for `Reg<WWDT_STATUS_SPEC>`"]
pub type WWDT_STATUS = crate::Reg<wwdt_status::WWDT_STATUS_SPEC>;
#[doc = "WWDT Status Register"]
pub mod wwdt_status;
#[doc = "WWDT_CNT register accessor: an alias for `Reg<WWDT_CNT_SPEC>`"]
pub type WWDT_CNT = crate::Reg<wwdt_cnt::WWDT_CNT_SPEC>;
#[doc = "WWDT Counter Value Register"]
pub mod wwdt_cnt;
