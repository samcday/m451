#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NMI Source Interrupt Enable Register"]
    pub nmien: crate::Reg<nmien::NMIEN_SPEC>,
    #[doc = "0x04 - NMI Source Interrupt Status Register"]
    pub nmists: crate::Reg<nmists::NMISTS_SPEC>,
}
#[doc = "NMIEN register accessor: an alias for `Reg<NMIEN_SPEC>`"]
pub type NMIEN = crate::Reg<nmien::NMIEN_SPEC>;
#[doc = "NMI Source Interrupt Enable Register"]
pub mod nmien;
#[doc = "NMISTS register accessor: an alias for `Reg<NMISTS_SPEC>`"]
pub type NMISTS = crate::Reg<nmists::NMISTS_SPEC>;
#[doc = "NMI Source Interrupt Status Register"]
pub mod nmists;
