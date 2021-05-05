#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct0_da: crate::Reg<pdma_dsct0_da::PDMA_DSCT0_DA_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct1_da: crate::Reg<pdma_dsct1_da::PDMA_DSCT1_DA_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct2_da: crate::Reg<pdma_dsct2_da::PDMA_DSCT2_DA_SPEC>,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct3_da: crate::Reg<pdma_dsct3_da::PDMA_DSCT3_DA_SPEC>,
    _reserved4: [u8; 12usize],
    #[doc = "0x40 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct4_da: crate::Reg<pdma_dsct4_da::PDMA_DSCT4_DA_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct5_da: crate::Reg<pdma_dsct5_da::PDMA_DSCT5_DA_SPEC>,
    _reserved6: [u8; 12usize],
    #[doc = "0x60 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct6_da: crate::Reg<pdma_dsct6_da::PDMA_DSCT6_DA_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x70 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct7_da: crate::Reg<pdma_dsct7_da::PDMA_DSCT7_DA_SPEC>,
    _reserved8: [u8; 12usize],
    #[doc = "0x80 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct8_da: crate::Reg<pdma_dsct8_da::PDMA_DSCT8_DA_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x90 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct9_da: crate::Reg<pdma_dsct9_da::PDMA_DSCT9_DA_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0xa0 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct10_da: crate::Reg<pdma_dsct10_da::PDMA_DSCT10_DA_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0xb0 - Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct11_da: crate::Reg<pdma_dsct11_da::PDMA_DSCT11_DA_SPEC>,
}
#[doc = "PDMA_DSCT0_DA register accessor: an alias for `Reg<PDMA_DSCT0_DA_SPEC>`"]
pub type PDMA_DSCT0_DA = crate::Reg<pdma_dsct0_da::PDMA_DSCT0_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct0_da;
#[doc = "PDMA_DSCT1_DA register accessor: an alias for `Reg<PDMA_DSCT1_DA_SPEC>`"]
pub type PDMA_DSCT1_DA = crate::Reg<pdma_dsct1_da::PDMA_DSCT1_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct1_da;
#[doc = "PDMA_DSCT2_DA register accessor: an alias for `Reg<PDMA_DSCT2_DA_SPEC>`"]
pub type PDMA_DSCT2_DA = crate::Reg<pdma_dsct2_da::PDMA_DSCT2_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct2_da;
#[doc = "PDMA_DSCT3_DA register accessor: an alias for `Reg<PDMA_DSCT3_DA_SPEC>`"]
pub type PDMA_DSCT3_DA = crate::Reg<pdma_dsct3_da::PDMA_DSCT3_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct3_da;
#[doc = "PDMA_DSCT4_DA register accessor: an alias for `Reg<PDMA_DSCT4_DA_SPEC>`"]
pub type PDMA_DSCT4_DA = crate::Reg<pdma_dsct4_da::PDMA_DSCT4_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct4_da;
#[doc = "PDMA_DSCT5_DA register accessor: an alias for `Reg<PDMA_DSCT5_DA_SPEC>`"]
pub type PDMA_DSCT5_DA = crate::Reg<pdma_dsct5_da::PDMA_DSCT5_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct5_da;
#[doc = "PDMA_DSCT6_DA register accessor: an alias for `Reg<PDMA_DSCT6_DA_SPEC>`"]
pub type PDMA_DSCT6_DA = crate::Reg<pdma_dsct6_da::PDMA_DSCT6_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct6_da;
#[doc = "PDMA_DSCT7_DA register accessor: an alias for `Reg<PDMA_DSCT7_DA_SPEC>`"]
pub type PDMA_DSCT7_DA = crate::Reg<pdma_dsct7_da::PDMA_DSCT7_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct7_da;
#[doc = "PDMA_DSCT8_DA register accessor: an alias for `Reg<PDMA_DSCT8_DA_SPEC>`"]
pub type PDMA_DSCT8_DA = crate::Reg<pdma_dsct8_da::PDMA_DSCT8_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct8_da;
#[doc = "PDMA_DSCT9_DA register accessor: an alias for `Reg<PDMA_DSCT9_DA_SPEC>`"]
pub type PDMA_DSCT9_DA = crate::Reg<pdma_dsct9_da::PDMA_DSCT9_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct9_da;
#[doc = "PDMA_DSCT10_DA register accessor: an alias for `Reg<PDMA_DSCT10_DA_SPEC>`"]
pub type PDMA_DSCT10_DA = crate::Reg<pdma_dsct10_da::PDMA_DSCT10_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct10_da;
#[doc = "PDMA_DSCT11_DA register accessor: an alias for `Reg<PDMA_DSCT11_DA_SPEC>`"]
pub type PDMA_DSCT11_DA = crate::Reg<pdma_dsct11_da::PDMA_DSCT11_DA_SPEC>;
#[doc = "Destination Address Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct11_da;
