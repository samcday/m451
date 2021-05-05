#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct0_sa: crate::Reg<pdma_dsct0_sa::PDMA_DSCT0_SA_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct1_sa: crate::Reg<pdma_dsct1_sa::PDMA_DSCT1_SA_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct2_sa: crate::Reg<pdma_dsct2_sa::PDMA_DSCT2_SA_SPEC>,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct3_sa: crate::Reg<pdma_dsct3_sa::PDMA_DSCT3_SA_SPEC>,
    _reserved4: [u8; 12usize],
    #[doc = "0x40 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct4_sa: crate::Reg<pdma_dsct4_sa::PDMA_DSCT4_SA_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct5_sa: crate::Reg<pdma_dsct5_sa::PDMA_DSCT5_SA_SPEC>,
    _reserved6: [u8; 12usize],
    #[doc = "0x60 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct6_sa: crate::Reg<pdma_dsct6_sa::PDMA_DSCT6_SA_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x70 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct7_sa: crate::Reg<pdma_dsct7_sa::PDMA_DSCT7_SA_SPEC>,
    _reserved8: [u8; 12usize],
    #[doc = "0x80 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct8_sa: crate::Reg<pdma_dsct8_sa::PDMA_DSCT8_SA_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x90 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct9_sa: crate::Reg<pdma_dsct9_sa::PDMA_DSCT9_SA_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0xa0 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct10_sa: crate::Reg<pdma_dsct10_sa::PDMA_DSCT10_SA_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0xb0 - Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct11_sa: crate::Reg<pdma_dsct11_sa::PDMA_DSCT11_SA_SPEC>,
}
#[doc = "PDMA_DSCT0_SA register accessor: an alias for `Reg<PDMA_DSCT0_SA_SPEC>`"]
pub type PDMA_DSCT0_SA = crate::Reg<pdma_dsct0_sa::PDMA_DSCT0_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct0_sa;
#[doc = "PDMA_DSCT1_SA register accessor: an alias for `Reg<PDMA_DSCT1_SA_SPEC>`"]
pub type PDMA_DSCT1_SA = crate::Reg<pdma_dsct1_sa::PDMA_DSCT1_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct1_sa;
#[doc = "PDMA_DSCT2_SA register accessor: an alias for `Reg<PDMA_DSCT2_SA_SPEC>`"]
pub type PDMA_DSCT2_SA = crate::Reg<pdma_dsct2_sa::PDMA_DSCT2_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct2_sa;
#[doc = "PDMA_DSCT3_SA register accessor: an alias for `Reg<PDMA_DSCT3_SA_SPEC>`"]
pub type PDMA_DSCT3_SA = crate::Reg<pdma_dsct3_sa::PDMA_DSCT3_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct3_sa;
#[doc = "PDMA_DSCT4_SA register accessor: an alias for `Reg<PDMA_DSCT4_SA_SPEC>`"]
pub type PDMA_DSCT4_SA = crate::Reg<pdma_dsct4_sa::PDMA_DSCT4_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct4_sa;
#[doc = "PDMA_DSCT5_SA register accessor: an alias for `Reg<PDMA_DSCT5_SA_SPEC>`"]
pub type PDMA_DSCT5_SA = crate::Reg<pdma_dsct5_sa::PDMA_DSCT5_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct5_sa;
#[doc = "PDMA_DSCT6_SA register accessor: an alias for `Reg<PDMA_DSCT6_SA_SPEC>`"]
pub type PDMA_DSCT6_SA = crate::Reg<pdma_dsct6_sa::PDMA_DSCT6_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct6_sa;
#[doc = "PDMA_DSCT7_SA register accessor: an alias for `Reg<PDMA_DSCT7_SA_SPEC>`"]
pub type PDMA_DSCT7_SA = crate::Reg<pdma_dsct7_sa::PDMA_DSCT7_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct7_sa;
#[doc = "PDMA_DSCT8_SA register accessor: an alias for `Reg<PDMA_DSCT8_SA_SPEC>`"]
pub type PDMA_DSCT8_SA = crate::Reg<pdma_dsct8_sa::PDMA_DSCT8_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct8_sa;
#[doc = "PDMA_DSCT9_SA register accessor: an alias for `Reg<PDMA_DSCT9_SA_SPEC>`"]
pub type PDMA_DSCT9_SA = crate::Reg<pdma_dsct9_sa::PDMA_DSCT9_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct9_sa;
#[doc = "PDMA_DSCT10_SA register accessor: an alias for `Reg<PDMA_DSCT10_SA_SPEC>`"]
pub type PDMA_DSCT10_SA = crate::Reg<pdma_dsct10_sa::PDMA_DSCT10_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct10_sa;
#[doc = "PDMA_DSCT11_SA register accessor: an alias for `Reg<PDMA_DSCT11_SA_SPEC>`"]
pub type PDMA_DSCT11_SA = crate::Reg<pdma_dsct11_sa::PDMA_DSCT11_SA_SPEC>;
#[doc = "Source Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct11_sa;
