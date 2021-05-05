#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Interrupt Enable Register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x04 - USB Device Interrupt Event Status Register"]
    pub intsts: crate::Reg<intsts::INTSTS_SPEC>,
    #[doc = "0x08 - USB Device Function Address Register"]
    pub faddr: crate::Reg<faddr::FADDR_SPEC>,
    #[doc = "0x0c - USB Device Endpoint Status Register"]
    pub epsts: crate::Reg<epsts::EPSTS_SPEC>,
    #[doc = "0x10 - USB Device Bus Status and Attribution Register"]
    pub attr: crate::Reg<attr::ATTR_SPEC>,
    #[doc = "0x14 - USB Device VBUS Detection Register"]
    pub vbusdet: crate::Reg<vbusdet::VBUSDET_SPEC>,
    #[doc = "0x18 - USB Setup Token Buffer Segmentation Register"]
    pub stbufseg: crate::Reg<stbufseg::STBUFSEG_SPEC>,
    _reserved7: [u8; 116usize],
    #[doc = "0x90 - USB Device Drive SE0 Control Register"]
    pub se0: crate::Reg<se0::SE0_SPEC>,
    _reserved8: [u8; 1132usize],
    #[doc = "0x500 - Cluster EP%s, containing BUFSEG*, MXPLD*, CFG\\[01234567\\], CFGP*"]
    pub ep: [EP; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EP {
    #[doc = "0x00 - USB Endpoint 0 Buffer Segmentation Register"]
    pub bufseg: crate::Reg<self::ep::bufseg::BUFSEG_SPEC>,
    #[doc = "0x04 - USB Endpoint 0 Maximal Payload Register"]
    pub mxpld: crate::Reg<self::ep::mxpld::MXPLD_SPEC>,
    #[doc = "0x08 - USB Endpoint 0 Configuration Register"]
    pub cfg: crate::Reg<self::ep::cfg::CFG_SPEC>,
    #[doc = "0x0c - USB Endpoint 0 Set Stall and Clear In/Out Ready Control Register"]
    pub cfgp: crate::Reg<self::ep::cfgp::CFGP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster EP%s, containing BUFSEG*, MXPLD*, CFG\\[01234567\\], CFGP*"]
pub mod ep;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "USB Device Interrupt Enable Register"]
pub mod inten;
#[doc = "INTSTS register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "USB Device Interrupt Event Status Register"]
pub mod intsts;
#[doc = "FADDR register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "USB Device Function Address Register"]
pub mod faddr;
#[doc = "EPSTS register accessor: an alias for `Reg<EPSTS_SPEC>`"]
pub type EPSTS = crate::Reg<epsts::EPSTS_SPEC>;
#[doc = "USB Device Endpoint Status Register"]
pub mod epsts;
#[doc = "ATTR register accessor: an alias for `Reg<ATTR_SPEC>`"]
pub type ATTR = crate::Reg<attr::ATTR_SPEC>;
#[doc = "USB Device Bus Status and Attribution Register"]
pub mod attr;
#[doc = "VBUSDET register accessor: an alias for `Reg<VBUSDET_SPEC>`"]
pub type VBUSDET = crate::Reg<vbusdet::VBUSDET_SPEC>;
#[doc = "USB Device VBUS Detection Register"]
pub mod vbusdet;
#[doc = "STBUFSEG register accessor: an alias for `Reg<STBUFSEG_SPEC>`"]
pub type STBUFSEG = crate::Reg<stbufseg::STBUFSEG_SPEC>;
#[doc = "USB Setup Token Buffer Segmentation Register"]
pub mod stbufseg;
#[doc = "SE0 register accessor: an alias for `Reg<SE0_SPEC>`"]
pub type SE0 = crate::Reg<se0::SE0_SPEC>;
#[doc = "USB Device Drive SE0 Control Register"]
pub mod se0;
