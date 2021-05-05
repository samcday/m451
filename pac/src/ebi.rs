#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Bus Interface Bank0 Control Register"]
    pub ebi_ctl0: crate::Reg<ebi_ctl0::EBI_CTL0_SPEC>,
    #[doc = "0x04 - External Bus Interface Bank0 Timing Control Register"]
    pub ebi_tctl0: crate::Reg<ebi_tctl0::EBI_TCTL0_SPEC>,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - External Bus Interface Bank1 Control Register"]
    pub ebi_ctl1: crate::Reg<ebi_ctl1::EBI_CTL1_SPEC>,
    #[doc = "0x14 - External Bus Interface Bank1 Timing Control Register"]
    pub ebi_tctl1: crate::Reg<ebi_tctl1::EBI_TCTL1_SPEC>,
}
#[doc = "EBI_CTL0 register accessor: an alias for `Reg<EBI_CTL0_SPEC>`"]
pub type EBI_CTL0 = crate::Reg<ebi_ctl0::EBI_CTL0_SPEC>;
#[doc = "External Bus Interface Bank0 Control Register"]
pub mod ebi_ctl0;
#[doc = "EBI_TCTL0 register accessor: an alias for `Reg<EBI_TCTL0_SPEC>`"]
pub type EBI_TCTL0 = crate::Reg<ebi_tctl0::EBI_TCTL0_SPEC>;
#[doc = "External Bus Interface Bank0 Timing Control Register"]
pub mod ebi_tctl0;
#[doc = "EBI_CTL1 register accessor: an alias for `Reg<EBI_CTL1_SPEC>`"]
pub type EBI_CTL1 = crate::Reg<ebi_ctl1::EBI_CTL1_SPEC>;
#[doc = "External Bus Interface Bank1 Control Register"]
pub mod ebi_ctl1;
#[doc = "EBI_TCTL1 register accessor: an alias for `Reg<EBI_TCTL1_SPEC>`"]
pub type EBI_TCTL1 = crate::Reg<ebi_tctl1::EBI_TCTL1_SPEC>;
#[doc = "External Bus Interface Bank1 Timing Control Register"]
pub mod ebi_tctl1;
