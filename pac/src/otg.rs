#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG Control Register"]
    pub otg_ctl: crate::Reg<otg_ctl::OTG_CTL_SPEC>,
    #[doc = "0x04 - OTG PHY Control Register"]
    pub otg_phyctl: crate::Reg<otg_phyctl::OTG_PHYCTL_SPEC>,
    #[doc = "0x08 - OTG Interrupt Enable Register"]
    pub otg_inten: crate::Reg<otg_inten::OTG_INTEN_SPEC>,
    #[doc = "0x0c - OTG Interrupt Status Register"]
    pub otg_intsts: crate::Reg<otg_intsts::OTG_INTSTS_SPEC>,
    #[doc = "0x10 - OTG Status Register"]
    pub otg_status: crate::Reg<otg_status::OTG_STATUS_SPEC>,
}
#[doc = "OTG_CTL register accessor: an alias for `Reg<OTG_CTL_SPEC>`"]
pub type OTG_CTL = crate::Reg<otg_ctl::OTG_CTL_SPEC>;
#[doc = "OTG Control Register"]
pub mod otg_ctl;
#[doc = "OTG_PHYCTL register accessor: an alias for `Reg<OTG_PHYCTL_SPEC>`"]
pub type OTG_PHYCTL = crate::Reg<otg_phyctl::OTG_PHYCTL_SPEC>;
#[doc = "OTG PHY Control Register"]
pub mod otg_phyctl;
#[doc = "OTG_INTEN register accessor: an alias for `Reg<OTG_INTEN_SPEC>`"]
pub type OTG_INTEN = crate::Reg<otg_inten::OTG_INTEN_SPEC>;
#[doc = "OTG Interrupt Enable Register"]
pub mod otg_inten;
#[doc = "OTG_INTSTS register accessor: an alias for `Reg<OTG_INTSTS_SPEC>`"]
pub type OTG_INTSTS = crate::Reg<otg_intsts::OTG_INTSTS_SPEC>;
#[doc = "OTG Interrupt Status Register"]
pub mod otg_intsts;
#[doc = "OTG_STATUS register accessor: an alias for `Reg<OTG_STATUS_SPEC>`"]
pub type OTG_STATUS = crate::Reg<otg_status::OTG_STATUS_SPEC>;
#[doc = "OTG Status Register"]
pub mod otg_status;
