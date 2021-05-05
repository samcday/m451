#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct0_ctl: crate::Reg<pdma_dsct0_ctl::PDMA_DSCT0_CTL_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct1_ctl: crate::Reg<pdma_dsct1_ctl::PDMA_DSCT1_CTL_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct2_ctl: crate::Reg<pdma_dsct2_ctl::PDMA_DSCT2_CTL_SPEC>,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct3_ctl: crate::Reg<pdma_dsct3_ctl::PDMA_DSCT3_CTL_SPEC>,
    _reserved4: [u8; 12usize],
    #[doc = "0x40 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct4_ctl: crate::Reg<pdma_dsct4_ctl::PDMA_DSCT4_CTL_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct5_ctl: crate::Reg<pdma_dsct5_ctl::PDMA_DSCT5_CTL_SPEC>,
    _reserved6: [u8; 12usize],
    #[doc = "0x60 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct6_ctl: crate::Reg<pdma_dsct6_ctl::PDMA_DSCT6_CTL_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x70 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct7_ctl: crate::Reg<pdma_dsct7_ctl::PDMA_DSCT7_CTL_SPEC>,
    _reserved8: [u8; 12usize],
    #[doc = "0x80 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct8_ctl: crate::Reg<pdma_dsct8_ctl::PDMA_DSCT8_CTL_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x90 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct9_ctl: crate::Reg<pdma_dsct9_ctl::PDMA_DSCT9_CTL_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0xa0 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct10_ctl: crate::Reg<pdma_dsct10_ctl::PDMA_DSCT10_CTL_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0xb0 - Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct11_ctl: crate::Reg<pdma_dsct11_ctl::PDMA_DSCT11_CTL_SPEC>,
}
#[doc = "PDMA_DSCT0_CTL register accessor: an alias for `Reg<PDMA_DSCT0_CTL_SPEC>`"]
pub type PDMA_DSCT0_CTL = crate::Reg<pdma_dsct0_ctl::PDMA_DSCT0_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct0_ctl;
#[doc = "PDMA_DSCT1_CTL register accessor: an alias for `Reg<PDMA_DSCT1_CTL_SPEC>`"]
pub type PDMA_DSCT1_CTL = crate::Reg<pdma_dsct1_ctl::PDMA_DSCT1_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct1_ctl;
#[doc = "PDMA_DSCT2_CTL register accessor: an alias for `Reg<PDMA_DSCT2_CTL_SPEC>`"]
pub type PDMA_DSCT2_CTL = crate::Reg<pdma_dsct2_ctl::PDMA_DSCT2_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct2_ctl;
#[doc = "PDMA_DSCT3_CTL register accessor: an alias for `Reg<PDMA_DSCT3_CTL_SPEC>`"]
pub type PDMA_DSCT3_CTL = crate::Reg<pdma_dsct3_ctl::PDMA_DSCT3_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct3_ctl;
#[doc = "PDMA_DSCT4_CTL register accessor: an alias for `Reg<PDMA_DSCT4_CTL_SPEC>`"]
pub type PDMA_DSCT4_CTL = crate::Reg<pdma_dsct4_ctl::PDMA_DSCT4_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct4_ctl;
#[doc = "PDMA_DSCT5_CTL register accessor: an alias for `Reg<PDMA_DSCT5_CTL_SPEC>`"]
pub type PDMA_DSCT5_CTL = crate::Reg<pdma_dsct5_ctl::PDMA_DSCT5_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct5_ctl;
#[doc = "PDMA_DSCT6_CTL register accessor: an alias for `Reg<PDMA_DSCT6_CTL_SPEC>`"]
pub type PDMA_DSCT6_CTL = crate::Reg<pdma_dsct6_ctl::PDMA_DSCT6_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct6_ctl;
#[doc = "PDMA_DSCT7_CTL register accessor: an alias for `Reg<PDMA_DSCT7_CTL_SPEC>`"]
pub type PDMA_DSCT7_CTL = crate::Reg<pdma_dsct7_ctl::PDMA_DSCT7_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct7_ctl;
#[doc = "PDMA_DSCT8_CTL register accessor: an alias for `Reg<PDMA_DSCT8_CTL_SPEC>`"]
pub type PDMA_DSCT8_CTL = crate::Reg<pdma_dsct8_ctl::PDMA_DSCT8_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct8_ctl;
#[doc = "PDMA_DSCT9_CTL register accessor: an alias for `Reg<PDMA_DSCT9_CTL_SPEC>`"]
pub type PDMA_DSCT9_CTL = crate::Reg<pdma_dsct9_ctl::PDMA_DSCT9_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct9_ctl;
#[doc = "PDMA_DSCT10_CTL register accessor: an alias for `Reg<PDMA_DSCT10_CTL_SPEC>`"]
pub type PDMA_DSCT10_CTL = crate::Reg<pdma_dsct10_ctl::PDMA_DSCT10_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct10_ctl;
#[doc = "PDMA_DSCT11_CTL register accessor: an alias for `Reg<PDMA_DSCT11_CTL_SPEC>`"]
pub type PDMA_DSCT11_CTL = crate::Reg<pdma_dsct11_ctl::PDMA_DSCT11_CTL_SPEC>;
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct11_ctl;
