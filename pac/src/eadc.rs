#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Data Register 0 for Sample Module 0"]
    pub eadc_dat0: crate::Reg<eadc_dat0::EADC_DAT0_SPEC>,
    #[doc = "0x04 - A/D Data Register 1 for Sample Module 1"]
    pub eadc_dat1: crate::Reg<eadc_dat1::EADC_DAT1_SPEC>,
    #[doc = "0x08 - A/D Data Register 2 for Sample Module 2"]
    pub eadc_dat2: crate::Reg<eadc_dat2::EADC_DAT2_SPEC>,
    #[doc = "0x0c - A/D Data Register 3 for Sample Module 3"]
    pub eadc_dat3: crate::Reg<eadc_dat3::EADC_DAT3_SPEC>,
    #[doc = "0x10 - A/D Data Register 4 for Sample Module 4"]
    pub eadc_dat4: crate::Reg<eadc_dat4::EADC_DAT4_SPEC>,
    #[doc = "0x14 - A/D Data Register 5 for Sample Module 5"]
    pub eadc_dat5: crate::Reg<eadc_dat5::EADC_DAT5_SPEC>,
    #[doc = "0x18 - A/D Data Register 6 for Sample Module 6"]
    pub eadc_dat6: crate::Reg<eadc_dat6::EADC_DAT6_SPEC>,
    #[doc = "0x1c - A/D Data Register 7 for Sample Module 7"]
    pub eadc_dat7: crate::Reg<eadc_dat7::EADC_DAT7_SPEC>,
    #[doc = "0x20 - A/D Data Register 8 for Sample Module 8"]
    pub eadc_dat8: crate::Reg<eadc_dat8::EADC_DAT8_SPEC>,
    #[doc = "0x24 - A/D Data Register 9 for Sample Module 9"]
    pub eadc_dat9: crate::Reg<eadc_dat9::EADC_DAT9_SPEC>,
    #[doc = "0x28 - A/D Data Register 10 for Sample Module 10"]
    pub eadc_dat10: crate::Reg<eadc_dat10::EADC_DAT10_SPEC>,
    #[doc = "0x2c - A/D Data Register 11 for Sample Module 11"]
    pub eadc_dat11: crate::Reg<eadc_dat11::EADC_DAT11_SPEC>,
    #[doc = "0x30 - A/D Data Register 12 for Sample Module 12"]
    pub eadc_dat12: crate::Reg<eadc_dat12::EADC_DAT12_SPEC>,
    #[doc = "0x34 - A/D Data Register 13 for Sample Module 13"]
    pub eadc_dat13: crate::Reg<eadc_dat13::EADC_DAT13_SPEC>,
    #[doc = "0x38 - A/D Data Register 14 for Sample Module 14"]
    pub eadc_dat14: crate::Reg<eadc_dat14::EADC_DAT14_SPEC>,
    #[doc = "0x3c - A/D Data Register 15 for Sample Module 15"]
    pub eadc_dat15: crate::Reg<eadc_dat15::EADC_DAT15_SPEC>,
    #[doc = "0x40 - A/D Data Register 16 for Sample Module 16"]
    pub eadc_dat16: crate::Reg<eadc_dat16::EADC_DAT16_SPEC>,
    #[doc = "0x44 - A/D Data Register 17 for Sample Module 17"]
    pub eadc_dat17: crate::Reg<eadc_dat17::EADC_DAT17_SPEC>,
    #[doc = "0x48 - A/D Data Register 18 for Sample Module 18"]
    pub eadc_dat18: crate::Reg<eadc_dat18::EADC_DAT18_SPEC>,
    #[doc = "0x4c - EADC PDMA Current Transfer Data Register"]
    pub eadc_curdat: crate::Reg<eadc_curdat::EADC_CURDAT_SPEC>,
    #[doc = "0x50 - A/D Control Register"]
    pub eadc_ctl: crate::Reg<eadc_ctl::EADC_CTL_SPEC>,
    #[doc = "0x54 - A/D Sample Module Software Start Register"]
    pub eadc_swtrg: crate::Reg<eadc_swtrg::EADC_SWTRG_SPEC>,
    #[doc = "0x58 - A/D Start of Conversion Pending Flag Register"]
    pub eadc_pendsts: crate::Reg<eadc_pendsts::EADC_PENDSTS_SPEC>,
    #[doc = "0x5c - A/D Sample Module Start of Conversion Overrun Flag Register"]
    pub eadc_ovsts: crate::Reg<eadc_ovsts::EADC_OVSTS_SPEC>,
    _reserved24: [u8; 32usize],
    #[doc = "0x80 - A/D Sample Module 0 Control Register"]
    pub eadc_sctl0: crate::Reg<eadc_sctl0::EADC_SCTL0_SPEC>,
    #[doc = "0x84 - A/D Sample Module 1 Control Register"]
    pub eadc_sctl1: crate::Reg<eadc_sctl1::EADC_SCTL1_SPEC>,
    #[doc = "0x88 - A/D Sample Module 2 Control Register"]
    pub eadc_sctl2: crate::Reg<eadc_sctl2::EADC_SCTL2_SPEC>,
    #[doc = "0x8c - A/D Sample Module 3 Control Register"]
    pub eadc_sctl3: crate::Reg<eadc_sctl3::EADC_SCTL3_SPEC>,
    #[doc = "0x90 - A/D Sample Module 4 Control Register"]
    pub eadc_sctl4: crate::Reg<eadc_sctl4::EADC_SCTL4_SPEC>,
    #[doc = "0x94 - A/D Sample Module 5 Control Register"]
    pub eadc_sctl5: crate::Reg<eadc_sctl5::EADC_SCTL5_SPEC>,
    #[doc = "0x98 - A/D Sample Module 6 Control Register"]
    pub eadc_sctl6: crate::Reg<eadc_sctl6::EADC_SCTL6_SPEC>,
    #[doc = "0x9c - A/D Sample Module 7 Control Register"]
    pub eadc_sctl7: crate::Reg<eadc_sctl7::EADC_SCTL7_SPEC>,
    #[doc = "0xa0 - A/D Sample Module 8 Control Register"]
    pub eadc_sctl8: crate::Reg<eadc_sctl8::EADC_SCTL8_SPEC>,
    #[doc = "0xa4 - A/D Sample Module 9 Control Register"]
    pub eadc_sctl9: crate::Reg<eadc_sctl9::EADC_SCTL9_SPEC>,
    #[doc = "0xa8 - A/D Sample Module 10 Control Register"]
    pub eadc_sctl10: crate::Reg<eadc_sctl10::EADC_SCTL10_SPEC>,
    #[doc = "0xac - A/D Sample Module 11 Control Register"]
    pub eadc_sctl11: crate::Reg<eadc_sctl11::EADC_SCTL11_SPEC>,
    #[doc = "0xb0 - A/D Sample Module 12 Control Register"]
    pub eadc_sctl12: crate::Reg<eadc_sctl12::EADC_SCTL12_SPEC>,
    #[doc = "0xb4 - A/D Sample Module 13 Control Register"]
    pub eadc_sctl13: crate::Reg<eadc_sctl13::EADC_SCTL13_SPEC>,
    #[doc = "0xb8 - A/D Sample Module 14 Control Register"]
    pub eadc_sctl14: crate::Reg<eadc_sctl14::EADC_SCTL14_SPEC>,
    #[doc = "0xbc - A/D Sample Module 15 Control Register"]
    pub eadc_sctl15: crate::Reg<eadc_sctl15::EADC_SCTL15_SPEC>,
    #[doc = "0xc0 - A/D Sample Module 16 Control Register"]
    pub eadc_sctl16: crate::Reg<eadc_sctl16::EADC_SCTL16_SPEC>,
    #[doc = "0xc4 - A/D Sample Module 17 Control Register"]
    pub eadc_sctl17: crate::Reg<eadc_sctl17::EADC_SCTL17_SPEC>,
    #[doc = "0xc8 - A/D Sample Module 18 Control Register"]
    pub eadc_sctl18: crate::Reg<eadc_sctl18::EADC_SCTL18_SPEC>,
    _reserved43: [u8; 4usize],
    #[doc = "0xd0 - ADC Interrupt 0 Source Enable Control Register."]
    pub eadc_intsrc0: crate::Reg<eadc_intsrc0::EADC_INTSRC0_SPEC>,
    #[doc = "0xd4 - ADC Interrupt 1 Source Enable Control Register."]
    pub eadc_intsrc1: crate::Reg<eadc_intsrc1::EADC_INTSRC1_SPEC>,
    #[doc = "0xd8 - ADC Interrupt 2 Source Enable Control Register."]
    pub eadc_intsrc2: crate::Reg<eadc_intsrc2::EADC_INTSRC2_SPEC>,
    #[doc = "0xdc - ADC Interrupt 3 Source Enable Control Register."]
    pub eadc_intsrc3: crate::Reg<eadc_intsrc3::EADC_INTSRC3_SPEC>,
    #[doc = "0xe0 - A/D Result Compare Register 0"]
    pub eadc_cmp0: crate::Reg<eadc_cmp0::EADC_CMP0_SPEC>,
    #[doc = "0xe4 - A/D Result Compare Register 1"]
    pub eadc_cmp1: crate::Reg<eadc_cmp1::EADC_CMP1_SPEC>,
    #[doc = "0xe8 - A/D Result Compare Register 2"]
    pub eadc_cmp2: crate::Reg<eadc_cmp2::EADC_CMP2_SPEC>,
    #[doc = "0xec - A/D Result Compare Register 3"]
    pub eadc_cmp3: crate::Reg<eadc_cmp3::EADC_CMP3_SPEC>,
    #[doc = "0xf0 - A/D Status Register 0"]
    pub eadc_status0: crate::Reg<eadc_status0::EADC_STATUS0_SPEC>,
    #[doc = "0xf4 - A/D Status Register 1"]
    pub eadc_status1: crate::Reg<eadc_status1::EADC_STATUS1_SPEC>,
    #[doc = "0xf8 - A/D Status Register 2"]
    pub eadc_status2: crate::Reg<eadc_status2::EADC_STATUS2_SPEC>,
    #[doc = "0xfc - A/D Status Register 3"]
    pub eadc_status3: crate::Reg<eadc_status3::EADC_STATUS3_SPEC>,
    #[doc = "0x100 - A/D Double Data Register 0 for Sample Module 0"]
    pub eadc_ddat0: crate::Reg<eadc_ddat0::EADC_DDAT0_SPEC>,
    #[doc = "0x104 - A/D Double Data Register 1 for Sample Module 1"]
    pub eadc_ddat1: crate::Reg<eadc_ddat1::EADC_DDAT1_SPEC>,
    #[doc = "0x108 - A/D Double Data Register 2 for Sample Module 2"]
    pub eadc_ddat2: crate::Reg<eadc_ddat2::EADC_DDAT2_SPEC>,
    #[doc = "0x10c - A/D Double Data Register 3 for Sample Module 3"]
    pub eadc_ddat3: crate::Reg<eadc_ddat3::EADC_DDAT3_SPEC>,
}
#[doc = "EADC_DAT0 register accessor: an alias for `Reg<EADC_DAT0_SPEC>`"]
pub type EADC_DAT0 = crate::Reg<eadc_dat0::EADC_DAT0_SPEC>;
#[doc = "A/D Data Register 0 for Sample Module 0"]
pub mod eadc_dat0;
#[doc = "EADC_DAT1 register accessor: an alias for `Reg<EADC_DAT1_SPEC>`"]
pub type EADC_DAT1 = crate::Reg<eadc_dat1::EADC_DAT1_SPEC>;
#[doc = "A/D Data Register 1 for Sample Module 1"]
pub mod eadc_dat1;
#[doc = "EADC_DAT2 register accessor: an alias for `Reg<EADC_DAT2_SPEC>`"]
pub type EADC_DAT2 = crate::Reg<eadc_dat2::EADC_DAT2_SPEC>;
#[doc = "A/D Data Register 2 for Sample Module 2"]
pub mod eadc_dat2;
#[doc = "EADC_DAT3 register accessor: an alias for `Reg<EADC_DAT3_SPEC>`"]
pub type EADC_DAT3 = crate::Reg<eadc_dat3::EADC_DAT3_SPEC>;
#[doc = "A/D Data Register 3 for Sample Module 3"]
pub mod eadc_dat3;
#[doc = "EADC_DAT4 register accessor: an alias for `Reg<EADC_DAT4_SPEC>`"]
pub type EADC_DAT4 = crate::Reg<eadc_dat4::EADC_DAT4_SPEC>;
#[doc = "A/D Data Register 4 for Sample Module 4"]
pub mod eadc_dat4;
#[doc = "EADC_DAT5 register accessor: an alias for `Reg<EADC_DAT5_SPEC>`"]
pub type EADC_DAT5 = crate::Reg<eadc_dat5::EADC_DAT5_SPEC>;
#[doc = "A/D Data Register 5 for Sample Module 5"]
pub mod eadc_dat5;
#[doc = "EADC_DAT6 register accessor: an alias for `Reg<EADC_DAT6_SPEC>`"]
pub type EADC_DAT6 = crate::Reg<eadc_dat6::EADC_DAT6_SPEC>;
#[doc = "A/D Data Register 6 for Sample Module 6"]
pub mod eadc_dat6;
#[doc = "EADC_DAT7 register accessor: an alias for `Reg<EADC_DAT7_SPEC>`"]
pub type EADC_DAT7 = crate::Reg<eadc_dat7::EADC_DAT7_SPEC>;
#[doc = "A/D Data Register 7 for Sample Module 7"]
pub mod eadc_dat7;
#[doc = "EADC_DAT8 register accessor: an alias for `Reg<EADC_DAT8_SPEC>`"]
pub type EADC_DAT8 = crate::Reg<eadc_dat8::EADC_DAT8_SPEC>;
#[doc = "A/D Data Register 8 for Sample Module 8"]
pub mod eadc_dat8;
#[doc = "EADC_DAT9 register accessor: an alias for `Reg<EADC_DAT9_SPEC>`"]
pub type EADC_DAT9 = crate::Reg<eadc_dat9::EADC_DAT9_SPEC>;
#[doc = "A/D Data Register 9 for Sample Module 9"]
pub mod eadc_dat9;
#[doc = "EADC_DAT10 register accessor: an alias for `Reg<EADC_DAT10_SPEC>`"]
pub type EADC_DAT10 = crate::Reg<eadc_dat10::EADC_DAT10_SPEC>;
#[doc = "A/D Data Register 10 for Sample Module 10"]
pub mod eadc_dat10;
#[doc = "EADC_DAT11 register accessor: an alias for `Reg<EADC_DAT11_SPEC>`"]
pub type EADC_DAT11 = crate::Reg<eadc_dat11::EADC_DAT11_SPEC>;
#[doc = "A/D Data Register 11 for Sample Module 11"]
pub mod eadc_dat11;
#[doc = "EADC_DAT12 register accessor: an alias for `Reg<EADC_DAT12_SPEC>`"]
pub type EADC_DAT12 = crate::Reg<eadc_dat12::EADC_DAT12_SPEC>;
#[doc = "A/D Data Register 12 for Sample Module 12"]
pub mod eadc_dat12;
#[doc = "EADC_DAT13 register accessor: an alias for `Reg<EADC_DAT13_SPEC>`"]
pub type EADC_DAT13 = crate::Reg<eadc_dat13::EADC_DAT13_SPEC>;
#[doc = "A/D Data Register 13 for Sample Module 13"]
pub mod eadc_dat13;
#[doc = "EADC_DAT14 register accessor: an alias for `Reg<EADC_DAT14_SPEC>`"]
pub type EADC_DAT14 = crate::Reg<eadc_dat14::EADC_DAT14_SPEC>;
#[doc = "A/D Data Register 14 for Sample Module 14"]
pub mod eadc_dat14;
#[doc = "EADC_DAT15 register accessor: an alias for `Reg<EADC_DAT15_SPEC>`"]
pub type EADC_DAT15 = crate::Reg<eadc_dat15::EADC_DAT15_SPEC>;
#[doc = "A/D Data Register 15 for Sample Module 15"]
pub mod eadc_dat15;
#[doc = "EADC_DAT16 register accessor: an alias for `Reg<EADC_DAT16_SPEC>`"]
pub type EADC_DAT16 = crate::Reg<eadc_dat16::EADC_DAT16_SPEC>;
#[doc = "A/D Data Register 16 for Sample Module 16"]
pub mod eadc_dat16;
#[doc = "EADC_DAT17 register accessor: an alias for `Reg<EADC_DAT17_SPEC>`"]
pub type EADC_DAT17 = crate::Reg<eadc_dat17::EADC_DAT17_SPEC>;
#[doc = "A/D Data Register 17 for Sample Module 17"]
pub mod eadc_dat17;
#[doc = "EADC_DAT18 register accessor: an alias for `Reg<EADC_DAT18_SPEC>`"]
pub type EADC_DAT18 = crate::Reg<eadc_dat18::EADC_DAT18_SPEC>;
#[doc = "A/D Data Register 18 for Sample Module 18"]
pub mod eadc_dat18;
#[doc = "EADC_CURDAT register accessor: an alias for `Reg<EADC_CURDAT_SPEC>`"]
pub type EADC_CURDAT = crate::Reg<eadc_curdat::EADC_CURDAT_SPEC>;
#[doc = "EADC PDMA Current Transfer Data Register"]
pub mod eadc_curdat;
#[doc = "EADC_CTL register accessor: an alias for `Reg<EADC_CTL_SPEC>`"]
pub type EADC_CTL = crate::Reg<eadc_ctl::EADC_CTL_SPEC>;
#[doc = "A/D Control Register"]
pub mod eadc_ctl;
#[doc = "EADC_SWTRG register accessor: an alias for `Reg<EADC_SWTRG_SPEC>`"]
pub type EADC_SWTRG = crate::Reg<eadc_swtrg::EADC_SWTRG_SPEC>;
#[doc = "A/D Sample Module Software Start Register"]
pub mod eadc_swtrg;
#[doc = "EADC_PENDSTS register accessor: an alias for `Reg<EADC_PENDSTS_SPEC>`"]
pub type EADC_PENDSTS = crate::Reg<eadc_pendsts::EADC_PENDSTS_SPEC>;
#[doc = "A/D Start of Conversion Pending Flag Register"]
pub mod eadc_pendsts;
#[doc = "EADC_OVSTS register accessor: an alias for `Reg<EADC_OVSTS_SPEC>`"]
pub type EADC_OVSTS = crate::Reg<eadc_ovsts::EADC_OVSTS_SPEC>;
#[doc = "A/D Sample Module Start of Conversion Overrun Flag Register"]
pub mod eadc_ovsts;
#[doc = "EADC_SCTL0 register accessor: an alias for `Reg<EADC_SCTL0_SPEC>`"]
pub type EADC_SCTL0 = crate::Reg<eadc_sctl0::EADC_SCTL0_SPEC>;
#[doc = "A/D Sample Module 0 Control Register"]
pub mod eadc_sctl0;
#[doc = "EADC_SCTL1 register accessor: an alias for `Reg<EADC_SCTL1_SPEC>`"]
pub type EADC_SCTL1 = crate::Reg<eadc_sctl1::EADC_SCTL1_SPEC>;
#[doc = "A/D Sample Module 1 Control Register"]
pub mod eadc_sctl1;
#[doc = "EADC_SCTL2 register accessor: an alias for `Reg<EADC_SCTL2_SPEC>`"]
pub type EADC_SCTL2 = crate::Reg<eadc_sctl2::EADC_SCTL2_SPEC>;
#[doc = "A/D Sample Module 2 Control Register"]
pub mod eadc_sctl2;
#[doc = "EADC_SCTL3 register accessor: an alias for `Reg<EADC_SCTL3_SPEC>`"]
pub type EADC_SCTL3 = crate::Reg<eadc_sctl3::EADC_SCTL3_SPEC>;
#[doc = "A/D Sample Module 3 Control Register"]
pub mod eadc_sctl3;
#[doc = "EADC_SCTL4 register accessor: an alias for `Reg<EADC_SCTL4_SPEC>`"]
pub type EADC_SCTL4 = crate::Reg<eadc_sctl4::EADC_SCTL4_SPEC>;
#[doc = "A/D Sample Module 4 Control Register"]
pub mod eadc_sctl4;
#[doc = "EADC_SCTL5 register accessor: an alias for `Reg<EADC_SCTL5_SPEC>`"]
pub type EADC_SCTL5 = crate::Reg<eadc_sctl5::EADC_SCTL5_SPEC>;
#[doc = "A/D Sample Module 5 Control Register"]
pub mod eadc_sctl5;
#[doc = "EADC_SCTL6 register accessor: an alias for `Reg<EADC_SCTL6_SPEC>`"]
pub type EADC_SCTL6 = crate::Reg<eadc_sctl6::EADC_SCTL6_SPEC>;
#[doc = "A/D Sample Module 6 Control Register"]
pub mod eadc_sctl6;
#[doc = "EADC_SCTL7 register accessor: an alias for `Reg<EADC_SCTL7_SPEC>`"]
pub type EADC_SCTL7 = crate::Reg<eadc_sctl7::EADC_SCTL7_SPEC>;
#[doc = "A/D Sample Module 7 Control Register"]
pub mod eadc_sctl7;
#[doc = "EADC_SCTL8 register accessor: an alias for `Reg<EADC_SCTL8_SPEC>`"]
pub type EADC_SCTL8 = crate::Reg<eadc_sctl8::EADC_SCTL8_SPEC>;
#[doc = "A/D Sample Module 8 Control Register"]
pub mod eadc_sctl8;
#[doc = "EADC_SCTL9 register accessor: an alias for `Reg<EADC_SCTL9_SPEC>`"]
pub type EADC_SCTL9 = crate::Reg<eadc_sctl9::EADC_SCTL9_SPEC>;
#[doc = "A/D Sample Module 9 Control Register"]
pub mod eadc_sctl9;
#[doc = "EADC_SCTL10 register accessor: an alias for `Reg<EADC_SCTL10_SPEC>`"]
pub type EADC_SCTL10 = crate::Reg<eadc_sctl10::EADC_SCTL10_SPEC>;
#[doc = "A/D Sample Module 10 Control Register"]
pub mod eadc_sctl10;
#[doc = "EADC_SCTL11 register accessor: an alias for `Reg<EADC_SCTL11_SPEC>`"]
pub type EADC_SCTL11 = crate::Reg<eadc_sctl11::EADC_SCTL11_SPEC>;
#[doc = "A/D Sample Module 11 Control Register"]
pub mod eadc_sctl11;
#[doc = "EADC_SCTL12 register accessor: an alias for `Reg<EADC_SCTL12_SPEC>`"]
pub type EADC_SCTL12 = crate::Reg<eadc_sctl12::EADC_SCTL12_SPEC>;
#[doc = "A/D Sample Module 12 Control Register"]
pub mod eadc_sctl12;
#[doc = "EADC_SCTL13 register accessor: an alias for `Reg<EADC_SCTL13_SPEC>`"]
pub type EADC_SCTL13 = crate::Reg<eadc_sctl13::EADC_SCTL13_SPEC>;
#[doc = "A/D Sample Module 13 Control Register"]
pub mod eadc_sctl13;
#[doc = "EADC_SCTL14 register accessor: an alias for `Reg<EADC_SCTL14_SPEC>`"]
pub type EADC_SCTL14 = crate::Reg<eadc_sctl14::EADC_SCTL14_SPEC>;
#[doc = "A/D Sample Module 14 Control Register"]
pub mod eadc_sctl14;
#[doc = "EADC_SCTL15 register accessor: an alias for `Reg<EADC_SCTL15_SPEC>`"]
pub type EADC_SCTL15 = crate::Reg<eadc_sctl15::EADC_SCTL15_SPEC>;
#[doc = "A/D Sample Module 15 Control Register"]
pub mod eadc_sctl15;
#[doc = "EADC_SCTL16 register accessor: an alias for `Reg<EADC_SCTL16_SPEC>`"]
pub type EADC_SCTL16 = crate::Reg<eadc_sctl16::EADC_SCTL16_SPEC>;
#[doc = "A/D Sample Module 16 Control Register"]
pub mod eadc_sctl16;
#[doc = "EADC_SCTL17 register accessor: an alias for `Reg<EADC_SCTL17_SPEC>`"]
pub type EADC_SCTL17 = crate::Reg<eadc_sctl17::EADC_SCTL17_SPEC>;
#[doc = "A/D Sample Module 17 Control Register"]
pub mod eadc_sctl17;
#[doc = "EADC_SCTL18 register accessor: an alias for `Reg<EADC_SCTL18_SPEC>`"]
pub type EADC_SCTL18 = crate::Reg<eadc_sctl18::EADC_SCTL18_SPEC>;
#[doc = "A/D Sample Module 18 Control Register"]
pub mod eadc_sctl18;
#[doc = "EADC_INTSRC0 register accessor: an alias for `Reg<EADC_INTSRC0_SPEC>`"]
pub type EADC_INTSRC0 = crate::Reg<eadc_intsrc0::EADC_INTSRC0_SPEC>;
#[doc = "ADC Interrupt 0 Source Enable Control Register."]
pub mod eadc_intsrc0;
#[doc = "EADC_INTSRC1 register accessor: an alias for `Reg<EADC_INTSRC1_SPEC>`"]
pub type EADC_INTSRC1 = crate::Reg<eadc_intsrc1::EADC_INTSRC1_SPEC>;
#[doc = "ADC Interrupt 1 Source Enable Control Register."]
pub mod eadc_intsrc1;
#[doc = "EADC_INTSRC2 register accessor: an alias for `Reg<EADC_INTSRC2_SPEC>`"]
pub type EADC_INTSRC2 = crate::Reg<eadc_intsrc2::EADC_INTSRC2_SPEC>;
#[doc = "ADC Interrupt 2 Source Enable Control Register."]
pub mod eadc_intsrc2;
#[doc = "EADC_INTSRC3 register accessor: an alias for `Reg<EADC_INTSRC3_SPEC>`"]
pub type EADC_INTSRC3 = crate::Reg<eadc_intsrc3::EADC_INTSRC3_SPEC>;
#[doc = "ADC Interrupt 3 Source Enable Control Register."]
pub mod eadc_intsrc3;
#[doc = "EADC_CMP0 register accessor: an alias for `Reg<EADC_CMP0_SPEC>`"]
pub type EADC_CMP0 = crate::Reg<eadc_cmp0::EADC_CMP0_SPEC>;
#[doc = "A/D Result Compare Register 0"]
pub mod eadc_cmp0;
#[doc = "EADC_CMP1 register accessor: an alias for `Reg<EADC_CMP1_SPEC>`"]
pub type EADC_CMP1 = crate::Reg<eadc_cmp1::EADC_CMP1_SPEC>;
#[doc = "A/D Result Compare Register 1"]
pub mod eadc_cmp1;
#[doc = "EADC_CMP2 register accessor: an alias for `Reg<EADC_CMP2_SPEC>`"]
pub type EADC_CMP2 = crate::Reg<eadc_cmp2::EADC_CMP2_SPEC>;
#[doc = "A/D Result Compare Register 2"]
pub mod eadc_cmp2;
#[doc = "EADC_CMP3 register accessor: an alias for `Reg<EADC_CMP3_SPEC>`"]
pub type EADC_CMP3 = crate::Reg<eadc_cmp3::EADC_CMP3_SPEC>;
#[doc = "A/D Result Compare Register 3"]
pub mod eadc_cmp3;
#[doc = "EADC_STATUS0 register accessor: an alias for `Reg<EADC_STATUS0_SPEC>`"]
pub type EADC_STATUS0 = crate::Reg<eadc_status0::EADC_STATUS0_SPEC>;
#[doc = "A/D Status Register 0"]
pub mod eadc_status0;
#[doc = "EADC_STATUS1 register accessor: an alias for `Reg<EADC_STATUS1_SPEC>`"]
pub type EADC_STATUS1 = crate::Reg<eadc_status1::EADC_STATUS1_SPEC>;
#[doc = "A/D Status Register 1"]
pub mod eadc_status1;
#[doc = "EADC_STATUS2 register accessor: an alias for `Reg<EADC_STATUS2_SPEC>`"]
pub type EADC_STATUS2 = crate::Reg<eadc_status2::EADC_STATUS2_SPEC>;
#[doc = "A/D Status Register 2"]
pub mod eadc_status2;
#[doc = "EADC_STATUS3 register accessor: an alias for `Reg<EADC_STATUS3_SPEC>`"]
pub type EADC_STATUS3 = crate::Reg<eadc_status3::EADC_STATUS3_SPEC>;
#[doc = "A/D Status Register 3"]
pub mod eadc_status3;
#[doc = "EADC_DDAT0 register accessor: an alias for `Reg<EADC_DDAT0_SPEC>`"]
pub type EADC_DDAT0 = crate::Reg<eadc_ddat0::EADC_DDAT0_SPEC>;
#[doc = "A/D Double Data Register 0 for Sample Module 0"]
pub mod eadc_ddat0;
#[doc = "EADC_DDAT1 register accessor: an alias for `Reg<EADC_DDAT1_SPEC>`"]
pub type EADC_DDAT1 = crate::Reg<eadc_ddat1::EADC_DDAT1_SPEC>;
#[doc = "A/D Double Data Register 1 for Sample Module 1"]
pub mod eadc_ddat1;
#[doc = "EADC_DDAT2 register accessor: an alias for `Reg<EADC_DDAT2_SPEC>`"]
pub type EADC_DDAT2 = crate::Reg<eadc_ddat2::EADC_DDAT2_SPEC>;
#[doc = "A/D Double Data Register 2 for Sample Module 2"]
pub mod eadc_ddat2;
#[doc = "EADC_DDAT3 register accessor: an alias for `Reg<EADC_DDAT3_SPEC>`"]
pub type EADC_DDAT3 = crate::Reg<eadc_ddat3::EADC_DDAT3_SPEC>;
#[doc = "A/D Double Data Register 3 for Sample Module 3"]
pub mod eadc_ddat3;
