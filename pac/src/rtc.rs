#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Initiation Register"]
    pub rtc_init: crate::Reg<rtc_init::RTC_INIT_SPEC>,
    #[doc = "0x04 - RTC Access Enable Register"]
    pub rtc_rwen: crate::Reg<rtc_rwen::RTC_RWEN_SPEC>,
    #[doc = "0x08 - RTC Frequency Compensation Register"]
    pub rtc_freqadj: crate::Reg<rtc_freqadj::RTC_FREQADJ_SPEC>,
    #[doc = "0x0c - RTC Time Loading Register"]
    pub rtc_time: crate::Reg<rtc_time::RTC_TIME_SPEC>,
    #[doc = "0x10 - RTC Calendar Loading Register"]
    pub rtc_cal: crate::Reg<rtc_cal::RTC_CAL_SPEC>,
    #[doc = "0x14 - RTC Time Scale Selection Register"]
    pub rtc_clkfmt: crate::Reg<rtc_clkfmt::RTC_CLKFMT_SPEC>,
    #[doc = "0x18 - RTC Day of the Week Register"]
    pub rtc_weekday: crate::Reg<rtc_weekday::RTC_WEEKDAY_SPEC>,
    #[doc = "0x1c - RTC Time Alarm Register"]
    pub rtc_talm: crate::Reg<rtc_talm::RTC_TALM_SPEC>,
    #[doc = "0x20 - RTC Calendar Alarm Register"]
    pub rtc_calm: crate::Reg<rtc_calm::RTC_CALM_SPEC>,
    #[doc = "0x24 - RTC Leap Year Indicator Register"]
    pub rtc_leapyear: crate::Reg<rtc_leapyear::RTC_LEAPYEAR_SPEC>,
    #[doc = "0x28 - RTC Interrupt Enable Register"]
    pub rtc_inten: crate::Reg<rtc_inten::RTC_INTEN_SPEC>,
    #[doc = "0x2c - RTC Interrupt Indicator Register"]
    pub rtc_intsts: crate::Reg<rtc_intsts::RTC_INTSTS_SPEC>,
    #[doc = "0x30 - RTC Time Tick Register"]
    pub rtc_tick: crate::Reg<rtc_tick::RTC_TICK_SPEC>,
    #[doc = "0x34 - RTC Time Alarm Mask Register"]
    pub rtc_tamsk: crate::Reg<rtc_tamsk::RTC_TAMSK_SPEC>,
    #[doc = "0x38 - RTC Calendar Alarm Mask Register"]
    pub rtc_camsk: crate::Reg<rtc_camsk::RTC_CAMSK_SPEC>,
    #[doc = "0x3c - RTC Spare Functional Control Register"]
    pub rtc_sprctl: crate::Reg<rtc_sprctl::RTC_SPRCTL_SPEC>,
    #[doc = "0x40 - RTC Spare Register 0"]
    pub rtc_spr0: crate::Reg<rtc_spr0::RTC_SPR0_SPEC>,
    #[doc = "0x44 - RTC Spare Register 1"]
    pub rtc_spr1: crate::Reg<rtc_spr1::RTC_SPR1_SPEC>,
    #[doc = "0x48 - RTC Spare Register 2"]
    pub rtc_spr2: crate::Reg<rtc_spr2::RTC_SPR2_SPEC>,
    #[doc = "0x4c - RTC Spare Register 3"]
    pub rtc_spr3: crate::Reg<rtc_spr3::RTC_SPR3_SPEC>,
    #[doc = "0x50 - RTC Spare Register 4"]
    pub rtc_spr4: crate::Reg<rtc_spr4::RTC_SPR4_SPEC>,
    #[doc = "0x54 - RTC Spare Register 5"]
    pub rtc_spr5: crate::Reg<rtc_spr5::RTC_SPR5_SPEC>,
    #[doc = "0x58 - RTC Spare Register 6"]
    pub rtc_spr6: crate::Reg<rtc_spr6::RTC_SPR6_SPEC>,
    #[doc = "0x5c - RTC Spare Register 7"]
    pub rtc_spr7: crate::Reg<rtc_spr7::RTC_SPR7_SPEC>,
    #[doc = "0x60 - RTC Spare Register 8"]
    pub rtc_spr8: crate::Reg<rtc_spr8::RTC_SPR8_SPEC>,
    #[doc = "0x64 - RTC Spare Register 9"]
    pub rtc_spr9: crate::Reg<rtc_spr9::RTC_SPR9_SPEC>,
    #[doc = "0x68 - RTC Spare Register 10"]
    pub rtc_spr10: crate::Reg<rtc_spr10::RTC_SPR10_SPEC>,
    #[doc = "0x6c - RTC Spare Register 11"]
    pub rtc_spr11: crate::Reg<rtc_spr11::RTC_SPR11_SPEC>,
    #[doc = "0x70 - RTC Spare Register 12"]
    pub rtc_spr12: crate::Reg<rtc_spr12::RTC_SPR12_SPEC>,
    #[doc = "0x74 - RTC Spare Register 13"]
    pub rtc_spr13: crate::Reg<rtc_spr13::RTC_SPR13_SPEC>,
    #[doc = "0x78 - RTC Spare Register 14"]
    pub rtc_spr14: crate::Reg<rtc_spr14::RTC_SPR14_SPEC>,
    #[doc = "0x7c - RTC Spare Register 15"]
    pub rtc_spr15: crate::Reg<rtc_spr15::RTC_SPR15_SPEC>,
    #[doc = "0x80 - RTC Spare Register 16"]
    pub rtc_spr16: crate::Reg<rtc_spr16::RTC_SPR16_SPEC>,
    #[doc = "0x84 - RTC Spare Register 17"]
    pub rtc_spr17: crate::Reg<rtc_spr17::RTC_SPR17_SPEC>,
    #[doc = "0x88 - RTC Spare Register 18"]
    pub rtc_spr18: crate::Reg<rtc_spr18::RTC_SPR18_SPEC>,
    #[doc = "0x8c - RTC Spare Register 19"]
    pub rtc_spr19: crate::Reg<rtc_spr19::RTC_SPR19_SPEC>,
    _reserved36: [u8; 112usize],
    #[doc = "0x100 - RTC 32.768 KHz Oscillator Control Register"]
    pub rtc_lxtctl: crate::Reg<rtc_lxtctl::RTC_LXTCTL_SPEC>,
    #[doc = "0x104 - X32KO Pin Control Register"]
    pub rtc_lxtoctl: crate::Reg<rtc_lxtoctl::RTC_LXTOCTL_SPEC>,
    #[doc = "0x108 - X32KI Pin Control Register"]
    pub rtc_lxtictl: crate::Reg<rtc_lxtictl::RTC_LXTICTL_SPEC>,
    #[doc = "0x10c - TAMPER Pin Control Register"]
    pub rtc_tampctl: crate::Reg<rtc_tampctl::RTC_TAMPCTL_SPEC>,
}
#[doc = "RTC_INIT register accessor: an alias for `Reg<RTC_INIT_SPEC>`"]
pub type RTC_INIT = crate::Reg<rtc_init::RTC_INIT_SPEC>;
#[doc = "RTC Initiation Register"]
pub mod rtc_init;
#[doc = "RTC_RWEN register accessor: an alias for `Reg<RTC_RWEN_SPEC>`"]
pub type RTC_RWEN = crate::Reg<rtc_rwen::RTC_RWEN_SPEC>;
#[doc = "RTC Access Enable Register"]
pub mod rtc_rwen;
#[doc = "RTC_FREQADJ register accessor: an alias for `Reg<RTC_FREQADJ_SPEC>`"]
pub type RTC_FREQADJ = crate::Reg<rtc_freqadj::RTC_FREQADJ_SPEC>;
#[doc = "RTC Frequency Compensation Register"]
pub mod rtc_freqadj;
#[doc = "RTC_TIME register accessor: an alias for `Reg<RTC_TIME_SPEC>`"]
pub type RTC_TIME = crate::Reg<rtc_time::RTC_TIME_SPEC>;
#[doc = "RTC Time Loading Register"]
pub mod rtc_time;
#[doc = "RTC_CAL register accessor: an alias for `Reg<RTC_CAL_SPEC>`"]
pub type RTC_CAL = crate::Reg<rtc_cal::RTC_CAL_SPEC>;
#[doc = "RTC Calendar Loading Register"]
pub mod rtc_cal;
#[doc = "RTC_CLKFMT register accessor: an alias for `Reg<RTC_CLKFMT_SPEC>`"]
pub type RTC_CLKFMT = crate::Reg<rtc_clkfmt::RTC_CLKFMT_SPEC>;
#[doc = "RTC Time Scale Selection Register"]
pub mod rtc_clkfmt;
#[doc = "RTC_WEEKDAY register accessor: an alias for `Reg<RTC_WEEKDAY_SPEC>`"]
pub type RTC_WEEKDAY = crate::Reg<rtc_weekday::RTC_WEEKDAY_SPEC>;
#[doc = "RTC Day of the Week Register"]
pub mod rtc_weekday;
#[doc = "RTC_TALM register accessor: an alias for `Reg<RTC_TALM_SPEC>`"]
pub type RTC_TALM = crate::Reg<rtc_talm::RTC_TALM_SPEC>;
#[doc = "RTC Time Alarm Register"]
pub mod rtc_talm;
#[doc = "RTC_CALM register accessor: an alias for `Reg<RTC_CALM_SPEC>`"]
pub type RTC_CALM = crate::Reg<rtc_calm::RTC_CALM_SPEC>;
#[doc = "RTC Calendar Alarm Register"]
pub mod rtc_calm;
#[doc = "RTC_LEAPYEAR register accessor: an alias for `Reg<RTC_LEAPYEAR_SPEC>`"]
pub type RTC_LEAPYEAR = crate::Reg<rtc_leapyear::RTC_LEAPYEAR_SPEC>;
#[doc = "RTC Leap Year Indicator Register"]
pub mod rtc_leapyear;
#[doc = "RTC_INTEN register accessor: an alias for `Reg<RTC_INTEN_SPEC>`"]
pub type RTC_INTEN = crate::Reg<rtc_inten::RTC_INTEN_SPEC>;
#[doc = "RTC Interrupt Enable Register"]
pub mod rtc_inten;
#[doc = "RTC_INTSTS register accessor: an alias for `Reg<RTC_INTSTS_SPEC>`"]
pub type RTC_INTSTS = crate::Reg<rtc_intsts::RTC_INTSTS_SPEC>;
#[doc = "RTC Interrupt Indicator Register"]
pub mod rtc_intsts;
#[doc = "RTC_TICK register accessor: an alias for `Reg<RTC_TICK_SPEC>`"]
pub type RTC_TICK = crate::Reg<rtc_tick::RTC_TICK_SPEC>;
#[doc = "RTC Time Tick Register"]
pub mod rtc_tick;
#[doc = "RTC_TAMSK register accessor: an alias for `Reg<RTC_TAMSK_SPEC>`"]
pub type RTC_TAMSK = crate::Reg<rtc_tamsk::RTC_TAMSK_SPEC>;
#[doc = "RTC Time Alarm Mask Register"]
pub mod rtc_tamsk;
#[doc = "RTC_CAMSK register accessor: an alias for `Reg<RTC_CAMSK_SPEC>`"]
pub type RTC_CAMSK = crate::Reg<rtc_camsk::RTC_CAMSK_SPEC>;
#[doc = "RTC Calendar Alarm Mask Register"]
pub mod rtc_camsk;
#[doc = "RTC_SPRCTL register accessor: an alias for `Reg<RTC_SPRCTL_SPEC>`"]
pub type RTC_SPRCTL = crate::Reg<rtc_sprctl::RTC_SPRCTL_SPEC>;
#[doc = "RTC Spare Functional Control Register"]
pub mod rtc_sprctl;
#[doc = "RTC_SPR0 register accessor: an alias for `Reg<RTC_SPR0_SPEC>`"]
pub type RTC_SPR0 = crate::Reg<rtc_spr0::RTC_SPR0_SPEC>;
#[doc = "RTC Spare Register 0"]
pub mod rtc_spr0;
#[doc = "RTC_SPR1 register accessor: an alias for `Reg<RTC_SPR1_SPEC>`"]
pub type RTC_SPR1 = crate::Reg<rtc_spr1::RTC_SPR1_SPEC>;
#[doc = "RTC Spare Register 1"]
pub mod rtc_spr1;
#[doc = "RTC_SPR2 register accessor: an alias for `Reg<RTC_SPR2_SPEC>`"]
pub type RTC_SPR2 = crate::Reg<rtc_spr2::RTC_SPR2_SPEC>;
#[doc = "RTC Spare Register 2"]
pub mod rtc_spr2;
#[doc = "RTC_SPR3 register accessor: an alias for `Reg<RTC_SPR3_SPEC>`"]
pub type RTC_SPR3 = crate::Reg<rtc_spr3::RTC_SPR3_SPEC>;
#[doc = "RTC Spare Register 3"]
pub mod rtc_spr3;
#[doc = "RTC_SPR4 register accessor: an alias for `Reg<RTC_SPR4_SPEC>`"]
pub type RTC_SPR4 = crate::Reg<rtc_spr4::RTC_SPR4_SPEC>;
#[doc = "RTC Spare Register 4"]
pub mod rtc_spr4;
#[doc = "RTC_SPR5 register accessor: an alias for `Reg<RTC_SPR5_SPEC>`"]
pub type RTC_SPR5 = crate::Reg<rtc_spr5::RTC_SPR5_SPEC>;
#[doc = "RTC Spare Register 5"]
pub mod rtc_spr5;
#[doc = "RTC_SPR6 register accessor: an alias for `Reg<RTC_SPR6_SPEC>`"]
pub type RTC_SPR6 = crate::Reg<rtc_spr6::RTC_SPR6_SPEC>;
#[doc = "RTC Spare Register 6"]
pub mod rtc_spr6;
#[doc = "RTC_SPR7 register accessor: an alias for `Reg<RTC_SPR7_SPEC>`"]
pub type RTC_SPR7 = crate::Reg<rtc_spr7::RTC_SPR7_SPEC>;
#[doc = "RTC Spare Register 7"]
pub mod rtc_spr7;
#[doc = "RTC_SPR8 register accessor: an alias for `Reg<RTC_SPR8_SPEC>`"]
pub type RTC_SPR8 = crate::Reg<rtc_spr8::RTC_SPR8_SPEC>;
#[doc = "RTC Spare Register 8"]
pub mod rtc_spr8;
#[doc = "RTC_SPR9 register accessor: an alias for `Reg<RTC_SPR9_SPEC>`"]
pub type RTC_SPR9 = crate::Reg<rtc_spr9::RTC_SPR9_SPEC>;
#[doc = "RTC Spare Register 9"]
pub mod rtc_spr9;
#[doc = "RTC_SPR10 register accessor: an alias for `Reg<RTC_SPR10_SPEC>`"]
pub type RTC_SPR10 = crate::Reg<rtc_spr10::RTC_SPR10_SPEC>;
#[doc = "RTC Spare Register 10"]
pub mod rtc_spr10;
#[doc = "RTC_SPR11 register accessor: an alias for `Reg<RTC_SPR11_SPEC>`"]
pub type RTC_SPR11 = crate::Reg<rtc_spr11::RTC_SPR11_SPEC>;
#[doc = "RTC Spare Register 11"]
pub mod rtc_spr11;
#[doc = "RTC_SPR12 register accessor: an alias for `Reg<RTC_SPR12_SPEC>`"]
pub type RTC_SPR12 = crate::Reg<rtc_spr12::RTC_SPR12_SPEC>;
#[doc = "RTC Spare Register 12"]
pub mod rtc_spr12;
#[doc = "RTC_SPR13 register accessor: an alias for `Reg<RTC_SPR13_SPEC>`"]
pub type RTC_SPR13 = crate::Reg<rtc_spr13::RTC_SPR13_SPEC>;
#[doc = "RTC Spare Register 13"]
pub mod rtc_spr13;
#[doc = "RTC_SPR14 register accessor: an alias for `Reg<RTC_SPR14_SPEC>`"]
pub type RTC_SPR14 = crate::Reg<rtc_spr14::RTC_SPR14_SPEC>;
#[doc = "RTC Spare Register 14"]
pub mod rtc_spr14;
#[doc = "RTC_SPR15 register accessor: an alias for `Reg<RTC_SPR15_SPEC>`"]
pub type RTC_SPR15 = crate::Reg<rtc_spr15::RTC_SPR15_SPEC>;
#[doc = "RTC Spare Register 15"]
pub mod rtc_spr15;
#[doc = "RTC_SPR16 register accessor: an alias for `Reg<RTC_SPR16_SPEC>`"]
pub type RTC_SPR16 = crate::Reg<rtc_spr16::RTC_SPR16_SPEC>;
#[doc = "RTC Spare Register 16"]
pub mod rtc_spr16;
#[doc = "RTC_SPR17 register accessor: an alias for `Reg<RTC_SPR17_SPEC>`"]
pub type RTC_SPR17 = crate::Reg<rtc_spr17::RTC_SPR17_SPEC>;
#[doc = "RTC Spare Register 17"]
pub mod rtc_spr17;
#[doc = "RTC_SPR18 register accessor: an alias for `Reg<RTC_SPR18_SPEC>`"]
pub type RTC_SPR18 = crate::Reg<rtc_spr18::RTC_SPR18_SPEC>;
#[doc = "RTC Spare Register 18"]
pub mod rtc_spr18;
#[doc = "RTC_SPR19 register accessor: an alias for `Reg<RTC_SPR19_SPEC>`"]
pub type RTC_SPR19 = crate::Reg<rtc_spr19::RTC_SPR19_SPEC>;
#[doc = "RTC Spare Register 19"]
pub mod rtc_spr19;
#[doc = "RTC_LXTCTL register accessor: an alias for `Reg<RTC_LXTCTL_SPEC>`"]
pub type RTC_LXTCTL = crate::Reg<rtc_lxtctl::RTC_LXTCTL_SPEC>;
#[doc = "RTC 32.768 KHz Oscillator Control Register"]
pub mod rtc_lxtctl;
#[doc = "RTC_LXTOCTL register accessor: an alias for `Reg<RTC_LXTOCTL_SPEC>`"]
pub type RTC_LXTOCTL = crate::Reg<rtc_lxtoctl::RTC_LXTOCTL_SPEC>;
#[doc = "X32KO Pin Control Register"]
pub mod rtc_lxtoctl;
#[doc = "RTC_LXTICTL register accessor: an alias for `Reg<RTC_LXTICTL_SPEC>`"]
pub type RTC_LXTICTL = crate::Reg<rtc_lxtictl::RTC_LXTICTL_SPEC>;
#[doc = "X32KI Pin Control Register"]
pub mod rtc_lxtictl;
#[doc = "RTC_TAMPCTL register accessor: an alias for `Reg<RTC_TAMPCTL_SPEC>`"]
pub type RTC_TAMPCTL = crate::Reg<rtc_tampctl::RTC_TAMPCTL_SPEC>;
#[doc = "TAMPER Pin Control Register"]
pub mod rtc_tampctl;
