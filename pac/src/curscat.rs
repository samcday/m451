#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat0: crate::Reg<pdma_curscat0::PDMA_CURSCAT0_SPEC>,
    #[doc = "0x04 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat1: crate::Reg<pdma_curscat1::PDMA_CURSCAT1_SPEC>,
    #[doc = "0x08 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat2: crate::Reg<pdma_curscat2::PDMA_CURSCAT2_SPEC>,
    #[doc = "0x0c - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat3: crate::Reg<pdma_curscat3::PDMA_CURSCAT3_SPEC>,
    #[doc = "0x10 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat4: crate::Reg<pdma_curscat4::PDMA_CURSCAT4_SPEC>,
    #[doc = "0x14 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat5: crate::Reg<pdma_curscat5::PDMA_CURSCAT5_SPEC>,
    #[doc = "0x18 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat6: crate::Reg<pdma_curscat6::PDMA_CURSCAT6_SPEC>,
    #[doc = "0x1c - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat7: crate::Reg<pdma_curscat7::PDMA_CURSCAT7_SPEC>,
    #[doc = "0x20 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat8: crate::Reg<pdma_curscat8::PDMA_CURSCAT8_SPEC>,
    #[doc = "0x24 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat9: crate::Reg<pdma_curscat9::PDMA_CURSCAT9_SPEC>,
    #[doc = "0x28 - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat10: crate::Reg<pdma_curscat10::PDMA_CURSCAT10_SPEC>,
    #[doc = "0x2c - Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
    pub pdma_curscat11: crate::Reg<pdma_curscat11::PDMA_CURSCAT11_SPEC>,
}
#[doc = "PDMA_CURSCAT0 register accessor: an alias for `Reg<PDMA_CURSCAT0_SPEC>`"]
pub type PDMA_CURSCAT0 = crate::Reg<pdma_curscat0::PDMA_CURSCAT0_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat0;
#[doc = "PDMA_CURSCAT1 register accessor: an alias for `Reg<PDMA_CURSCAT1_SPEC>`"]
pub type PDMA_CURSCAT1 = crate::Reg<pdma_curscat1::PDMA_CURSCAT1_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat1;
#[doc = "PDMA_CURSCAT2 register accessor: an alias for `Reg<PDMA_CURSCAT2_SPEC>`"]
pub type PDMA_CURSCAT2 = crate::Reg<pdma_curscat2::PDMA_CURSCAT2_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat2;
#[doc = "PDMA_CURSCAT3 register accessor: an alias for `Reg<PDMA_CURSCAT3_SPEC>`"]
pub type PDMA_CURSCAT3 = crate::Reg<pdma_curscat3::PDMA_CURSCAT3_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat3;
#[doc = "PDMA_CURSCAT4 register accessor: an alias for `Reg<PDMA_CURSCAT4_SPEC>`"]
pub type PDMA_CURSCAT4 = crate::Reg<pdma_curscat4::PDMA_CURSCAT4_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat4;
#[doc = "PDMA_CURSCAT5 register accessor: an alias for `Reg<PDMA_CURSCAT5_SPEC>`"]
pub type PDMA_CURSCAT5 = crate::Reg<pdma_curscat5::PDMA_CURSCAT5_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat5;
#[doc = "PDMA_CURSCAT6 register accessor: an alias for `Reg<PDMA_CURSCAT6_SPEC>`"]
pub type PDMA_CURSCAT6 = crate::Reg<pdma_curscat6::PDMA_CURSCAT6_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat6;
#[doc = "PDMA_CURSCAT7 register accessor: an alias for `Reg<PDMA_CURSCAT7_SPEC>`"]
pub type PDMA_CURSCAT7 = crate::Reg<pdma_curscat7::PDMA_CURSCAT7_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat7;
#[doc = "PDMA_CURSCAT8 register accessor: an alias for `Reg<PDMA_CURSCAT8_SPEC>`"]
pub type PDMA_CURSCAT8 = crate::Reg<pdma_curscat8::PDMA_CURSCAT8_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat8;
#[doc = "PDMA_CURSCAT9 register accessor: an alias for `Reg<PDMA_CURSCAT9_SPEC>`"]
pub type PDMA_CURSCAT9 = crate::Reg<pdma_curscat9::PDMA_CURSCAT9_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat9;
#[doc = "PDMA_CURSCAT10 register accessor: an alias for `Reg<PDMA_CURSCAT10_SPEC>`"]
pub type PDMA_CURSCAT10 = crate::Reg<pdma_curscat10::PDMA_CURSCAT10_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat10;
#[doc = "PDMA_CURSCAT11 register accessor: an alias for `Reg<PDMA_CURSCAT11_SPEC>`"]
pub type PDMA_CURSCAT11 = crate::Reg<pdma_curscat11::PDMA_CURSCAT11_SPEC>;
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)"]
pub mod pdma_curscat11;
