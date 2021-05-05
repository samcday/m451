#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct0_next: crate::Reg<pdma_dsct0_next::PDMA_DSCT0_NEXT_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct1_next: crate::Reg<pdma_dsct1_next::PDMA_DSCT1_NEXT_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct2_next: crate::Reg<pdma_dsct2_next::PDMA_DSCT2_NEXT_SPEC>,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct3_next: crate::Reg<pdma_dsct3_next::PDMA_DSCT3_NEXT_SPEC>,
    _reserved4: [u8; 12usize],
    #[doc = "0x40 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct4_next: crate::Reg<pdma_dsct4_next::PDMA_DSCT4_NEXT_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct5_next: crate::Reg<pdma_dsct5_next::PDMA_DSCT5_NEXT_SPEC>,
    _reserved6: [u8; 12usize],
    #[doc = "0x60 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct6_next: crate::Reg<pdma_dsct6_next::PDMA_DSCT6_NEXT_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x70 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct7_next: crate::Reg<pdma_dsct7_next::PDMA_DSCT7_NEXT_SPEC>,
    _reserved8: [u8; 12usize],
    #[doc = "0x80 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct8_next: crate::Reg<pdma_dsct8_next::PDMA_DSCT8_NEXT_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x90 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct9_next: crate::Reg<pdma_dsct9_next::PDMA_DSCT9_NEXT_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0xa0 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct10_next: crate::Reg<pdma_dsct10_next::PDMA_DSCT10_NEXT_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0xb0 - First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_dsct11_next: crate::Reg<pdma_dsct11_next::PDMA_DSCT11_NEXT_SPEC>,
}
#[doc = "PDMA_DSCT0_NEXT register accessor: an alias for `Reg<PDMA_DSCT0_NEXT_SPEC>`"]
pub type PDMA_DSCT0_NEXT = crate::Reg<pdma_dsct0_next::PDMA_DSCT0_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct0_next;
#[doc = "PDMA_DSCT1_NEXT register accessor: an alias for `Reg<PDMA_DSCT1_NEXT_SPEC>`"]
pub type PDMA_DSCT1_NEXT = crate::Reg<pdma_dsct1_next::PDMA_DSCT1_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct1_next;
#[doc = "PDMA_DSCT2_NEXT register accessor: an alias for `Reg<PDMA_DSCT2_NEXT_SPEC>`"]
pub type PDMA_DSCT2_NEXT = crate::Reg<pdma_dsct2_next::PDMA_DSCT2_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct2_next;
#[doc = "PDMA_DSCT3_NEXT register accessor: an alias for `Reg<PDMA_DSCT3_NEXT_SPEC>`"]
pub type PDMA_DSCT3_NEXT = crate::Reg<pdma_dsct3_next::PDMA_DSCT3_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct3_next;
#[doc = "PDMA_DSCT4_NEXT register accessor: an alias for `Reg<PDMA_DSCT4_NEXT_SPEC>`"]
pub type PDMA_DSCT4_NEXT = crate::Reg<pdma_dsct4_next::PDMA_DSCT4_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct4_next;
#[doc = "PDMA_DSCT5_NEXT register accessor: an alias for `Reg<PDMA_DSCT5_NEXT_SPEC>`"]
pub type PDMA_DSCT5_NEXT = crate::Reg<pdma_dsct5_next::PDMA_DSCT5_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct5_next;
#[doc = "PDMA_DSCT6_NEXT register accessor: an alias for `Reg<PDMA_DSCT6_NEXT_SPEC>`"]
pub type PDMA_DSCT6_NEXT = crate::Reg<pdma_dsct6_next::PDMA_DSCT6_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct6_next;
#[doc = "PDMA_DSCT7_NEXT register accessor: an alias for `Reg<PDMA_DSCT7_NEXT_SPEC>`"]
pub type PDMA_DSCT7_NEXT = crate::Reg<pdma_dsct7_next::PDMA_DSCT7_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct7_next;
#[doc = "PDMA_DSCT8_NEXT register accessor: an alias for `Reg<PDMA_DSCT8_NEXT_SPEC>`"]
pub type PDMA_DSCT8_NEXT = crate::Reg<pdma_dsct8_next::PDMA_DSCT8_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct8_next;
#[doc = "PDMA_DSCT9_NEXT register accessor: an alias for `Reg<PDMA_DSCT9_NEXT_SPEC>`"]
pub type PDMA_DSCT9_NEXT = crate::Reg<pdma_dsct9_next::PDMA_DSCT9_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct9_next;
#[doc = "PDMA_DSCT10_NEXT register accessor: an alias for `Reg<PDMA_DSCT10_NEXT_SPEC>`"]
pub type PDMA_DSCT10_NEXT = crate::Reg<pdma_dsct10_next::PDMA_DSCT10_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct10_next;
#[doc = "PDMA_DSCT11_NEXT register accessor: an alias for `Reg<PDMA_DSCT11_NEXT_SPEC>`"]
pub type PDMA_DSCT11_NEXT = crate::Reg<pdma_dsct11_next::PDMA_DSCT11_NEXT_SPEC>;
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_dsct11_next;
