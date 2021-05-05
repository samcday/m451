#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - PDMA Channel Control Register"]
    pub pdma_chctl: crate::Reg<pdma_chctl::PDMA_CHCTL_SPEC>,
    #[doc = "0x404 - PDMA Transfer Stop Control Register"]
    pub pdma_stop: crate::Reg<pdma_stop::PDMA_STOP_SPEC>,
    #[doc = "0x408 - PDMA Software Request Register"]
    pub pdma_swreq: crate::Reg<pdma_swreq::PDMA_SWREQ_SPEC>,
    #[doc = "0x40c - PDMA Channel Request Status Register"]
    pub pdma_trgsts: crate::Reg<pdma_trgsts::PDMA_TRGSTS_SPEC>,
    #[doc = "0x410 - PDMA Fixed Priority Setting Register"]
    pub pdma_priset: crate::Reg<pdma_priset::PDMA_PRISET_SPEC>,
    #[doc = "0x414 - PDMA Fixed Priority Clear Register"]
    pub pdma_priclr: crate::Reg<pdma_priclr::PDMA_PRICLR_SPEC>,
    #[doc = "0x418 - PDMA Interrupt Enable Register"]
    pub pdma_inten: crate::Reg<pdma_inten::PDMA_INTEN_SPEC>,
    #[doc = "0x41c - PDMA Interrupt Status Register"]
    pub pdma_intsts: crate::Reg<pdma_intsts::PDMA_INTSTS_SPEC>,
    #[doc = "0x420 - PDMA Channel Read/Write Target Abort Flag Register"]
    pub pdma_abtsts: crate::Reg<pdma_abtsts::PDMA_ABTSTS_SPEC>,
    #[doc = "0x424 - PDMA Channel Transfer Done Flag Register"]
    pub pdma_tdsts: crate::Reg<pdma_tdsts::PDMA_TDSTS_SPEC>,
    #[doc = "0x428 - PDMA Scatter-gather Table Empty Status Register"]
    pub pdma_scatsts: crate::Reg<pdma_scatsts::PDMA_SCATSTS_SPEC>,
    #[doc = "0x42c - PDMA Transfer Active Flag Register"]
    pub pdma_tactsts: crate::Reg<pdma_tactsts::PDMA_TACTSTS_SPEC>,
    _reserved12: [u8; 4usize],
    #[doc = "0x434 - PDMA Time-out Enable Register (M45xD/M45xC Only)"]
    pub pdma_touten: crate::Reg<pdma_touten::PDMA_TOUTEN_SPEC>,
    #[doc = "0x438 - PDMA Time-out Interrupt Enable Register (M45xD/M45xC Only)"]
    pub pdma_toutien: crate::Reg<pdma_toutien::PDMA_TOUTIEN_SPEC>,
    #[doc = "0x43c - PDMA Scatter-gather Descriptor Table Base Address Register"]
    pub pdma_scatba: crate::Reg<pdma_scatba::PDMA_SCATBA_SPEC>,
    #[doc = "0x440 - PDMA Time-out Counter Ch1 and Ch0 Register (M45xD/M45xC Only)"]
    pub pdma_toc0_1: crate::Reg<pdma_toc0_1::PDMA_TOC0_1_SPEC>,
    #[doc = "0x444 - PDMA Time-out Counter Ch3 and Ch2 Register (M45xD/M45xC Only)"]
    pub pdma_toc2_3: crate::Reg<pdma_toc2_3::PDMA_TOC2_3_SPEC>,
    #[doc = "0x448 - PDMA Time-out Counter Ch5 and Ch4 Register (M45xD/M45xC Only)"]
    pub pdma_toc4_5: crate::Reg<pdma_toc4_5::PDMA_TOC4_5_SPEC>,
    #[doc = "0x44c - PDMA Time-out Counter Ch7 and Ch6 Register (M45xD/M45xC Only)"]
    pub pdma_toc6_7: crate::Reg<pdma_toc6_7::PDMA_TOC6_7_SPEC>,
    _reserved19: [u8; 48usize],
    #[doc = "0x480 - PDMA Request Source Select Register 0"]
    pub pdma_reqsel0_3: crate::Reg<pdma_reqsel0_3::PDMA_REQSEL0_3_SPEC>,
    #[doc = "0x484 - PDMA Request Source Select Register 1"]
    pub pdma_reqsel4_7: crate::Reg<pdma_reqsel4_7::PDMA_REQSEL4_7_SPEC>,
    #[doc = "0x488 - PDMA Request Source Select Register 2"]
    pub pdma_reqsel8_11: crate::Reg<pdma_reqsel8_11::PDMA_REQSEL8_11_SPEC>,
}
#[doc = "PDMA_CHCTL register accessor: an alias for `Reg<PDMA_CHCTL_SPEC>`"]
pub type PDMA_CHCTL = crate::Reg<pdma_chctl::PDMA_CHCTL_SPEC>;
#[doc = "PDMA Channel Control Register"]
pub mod pdma_chctl;
#[doc = "PDMA_STOP register accessor: an alias for `Reg<PDMA_STOP_SPEC>`"]
pub type PDMA_STOP = crate::Reg<pdma_stop::PDMA_STOP_SPEC>;
#[doc = "PDMA Transfer Stop Control Register"]
pub mod pdma_stop;
#[doc = "PDMA_SWREQ register accessor: an alias for `Reg<PDMA_SWREQ_SPEC>`"]
pub type PDMA_SWREQ = crate::Reg<pdma_swreq::PDMA_SWREQ_SPEC>;
#[doc = "PDMA Software Request Register"]
pub mod pdma_swreq;
#[doc = "PDMA_TRGSTS register accessor: an alias for `Reg<PDMA_TRGSTS_SPEC>`"]
pub type PDMA_TRGSTS = crate::Reg<pdma_trgsts::PDMA_TRGSTS_SPEC>;
#[doc = "PDMA Channel Request Status Register"]
pub mod pdma_trgsts;
#[doc = "PDMA_PRISET register accessor: an alias for `Reg<PDMA_PRISET_SPEC>`"]
pub type PDMA_PRISET = crate::Reg<pdma_priset::PDMA_PRISET_SPEC>;
#[doc = "PDMA Fixed Priority Setting Register"]
pub mod pdma_priset;
#[doc = "PDMA_PRICLR register accessor: an alias for `Reg<PDMA_PRICLR_SPEC>`"]
pub type PDMA_PRICLR = crate::Reg<pdma_priclr::PDMA_PRICLR_SPEC>;
#[doc = "PDMA Fixed Priority Clear Register"]
pub mod pdma_priclr;
#[doc = "PDMA_INTEN register accessor: an alias for `Reg<PDMA_INTEN_SPEC>`"]
pub type PDMA_INTEN = crate::Reg<pdma_inten::PDMA_INTEN_SPEC>;
#[doc = "PDMA Interrupt Enable Register"]
pub mod pdma_inten;
#[doc = "PDMA_INTSTS register accessor: an alias for `Reg<PDMA_INTSTS_SPEC>`"]
pub type PDMA_INTSTS = crate::Reg<pdma_intsts::PDMA_INTSTS_SPEC>;
#[doc = "PDMA Interrupt Status Register"]
pub mod pdma_intsts;
#[doc = "PDMA_ABTSTS register accessor: an alias for `Reg<PDMA_ABTSTS_SPEC>`"]
pub type PDMA_ABTSTS = crate::Reg<pdma_abtsts::PDMA_ABTSTS_SPEC>;
#[doc = "PDMA Channel Read/Write Target Abort Flag Register"]
pub mod pdma_abtsts;
#[doc = "PDMA_TDSTS register accessor: an alias for `Reg<PDMA_TDSTS_SPEC>`"]
pub type PDMA_TDSTS = crate::Reg<pdma_tdsts::PDMA_TDSTS_SPEC>;
#[doc = "PDMA Channel Transfer Done Flag Register"]
pub mod pdma_tdsts;
#[doc = "PDMA_SCATSTS register accessor: an alias for `Reg<PDMA_SCATSTS_SPEC>`"]
pub type PDMA_SCATSTS = crate::Reg<pdma_scatsts::PDMA_SCATSTS_SPEC>;
#[doc = "PDMA Scatter-gather Table Empty Status Register"]
pub mod pdma_scatsts;
#[doc = "PDMA_TACTSTS register accessor: an alias for `Reg<PDMA_TACTSTS_SPEC>`"]
pub type PDMA_TACTSTS = crate::Reg<pdma_tactsts::PDMA_TACTSTS_SPEC>;
#[doc = "PDMA Transfer Active Flag Register"]
pub mod pdma_tactsts;
#[doc = "PDMA_TOUTEN register accessor: an alias for `Reg<PDMA_TOUTEN_SPEC>`"]
pub type PDMA_TOUTEN = crate::Reg<pdma_touten::PDMA_TOUTEN_SPEC>;
#[doc = "PDMA Time-out Enable Register (M45xD/M45xC Only)"]
pub mod pdma_touten;
#[doc = "PDMA_TOUTIEN register accessor: an alias for `Reg<PDMA_TOUTIEN_SPEC>`"]
pub type PDMA_TOUTIEN = crate::Reg<pdma_toutien::PDMA_TOUTIEN_SPEC>;
#[doc = "PDMA Time-out Interrupt Enable Register (M45xD/M45xC Only)"]
pub mod pdma_toutien;
#[doc = "PDMA_SCATBA register accessor: an alias for `Reg<PDMA_SCATBA_SPEC>`"]
pub type PDMA_SCATBA = crate::Reg<pdma_scatba::PDMA_SCATBA_SPEC>;
#[doc = "PDMA Scatter-gather Descriptor Table Base Address Register"]
pub mod pdma_scatba;
#[doc = "PDMA_TOC0_1 register accessor: an alias for `Reg<PDMA_TOC0_1_SPEC>`"]
pub type PDMA_TOC0_1 = crate::Reg<pdma_toc0_1::PDMA_TOC0_1_SPEC>;
#[doc = "PDMA Time-out Counter Ch1 and Ch0 Register (M45xD/M45xC Only)"]
pub mod pdma_toc0_1;
#[doc = "PDMA_TOC2_3 register accessor: an alias for `Reg<PDMA_TOC2_3_SPEC>`"]
pub type PDMA_TOC2_3 = crate::Reg<pdma_toc2_3::PDMA_TOC2_3_SPEC>;
#[doc = "PDMA Time-out Counter Ch3 and Ch2 Register (M45xD/M45xC Only)"]
pub mod pdma_toc2_3;
#[doc = "PDMA_TOC4_5 register accessor: an alias for `Reg<PDMA_TOC4_5_SPEC>`"]
pub type PDMA_TOC4_5 = crate::Reg<pdma_toc4_5::PDMA_TOC4_5_SPEC>;
#[doc = "PDMA Time-out Counter Ch5 and Ch4 Register (M45xD/M45xC Only)"]
pub mod pdma_toc4_5;
#[doc = "PDMA_TOC6_7 register accessor: an alias for `Reg<PDMA_TOC6_7_SPEC>`"]
pub type PDMA_TOC6_7 = crate::Reg<pdma_toc6_7::PDMA_TOC6_7_SPEC>;
#[doc = "PDMA Time-out Counter Ch7 and Ch6 Register (M45xD/M45xC Only)"]
pub mod pdma_toc6_7;
#[doc = "PDMA_REQSEL0_3 register accessor: an alias for `Reg<PDMA_REQSEL0_3_SPEC>`"]
pub type PDMA_REQSEL0_3 = crate::Reg<pdma_reqsel0_3::PDMA_REQSEL0_3_SPEC>;
#[doc = "PDMA Request Source Select Register 0"]
pub mod pdma_reqsel0_3;
#[doc = "PDMA_REQSEL4_7 register accessor: an alias for `Reg<PDMA_REQSEL4_7_SPEC>`"]
pub type PDMA_REQSEL4_7 = crate::Reg<pdma_reqsel4_7::PDMA_REQSEL4_7_SPEC>;
#[doc = "PDMA Request Source Select Register 1"]
pub mod pdma_reqsel4_7;
#[doc = "PDMA_REQSEL8_11 register accessor: an alias for `Reg<PDMA_REQSEL8_11_SPEC>`"]
pub type PDMA_REQSEL8_11 = crate::Reg<pdma_reqsel8_11::PDMA_REQSEL8_11_SPEC>;
#[doc = "PDMA Request Source Select Register 2"]
pub mod pdma_reqsel8_11;
