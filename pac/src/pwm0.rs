#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control Register 0"]
    pub pwm_ctl0: crate::Reg<pwm_ctl0::PWM_CTL0_SPEC>,
    #[doc = "0x04 - PWM Control Register 1"]
    pub pwm_ctl1: crate::Reg<pwm_ctl1::PWM_CTL1_SPEC>,
    #[doc = "0x08 - PWM Synchronization Register"]
    pub pwm_sync: crate::Reg<pwm_sync::PWM_SYNC_SPEC>,
    #[doc = "0x0c - PWM Software Control Synchronization Register"]
    pub pwm_swsync: crate::Reg<pwm_swsync::PWM_SWSYNC_SPEC>,
    #[doc = "0x10 - PWM Clock Source Register"]
    pub pwm_clksrc: crate::Reg<pwm_clksrc::PWM_CLKSRC_SPEC>,
    #[doc = "0x14 - PWM Clock Pre-scale Register 0"]
    pub pwm_clkpsc0_1: crate::Reg<pwm_clkpsc0_1::PWM_CLKPSC0_1_SPEC>,
    #[doc = "0x18 - PWM Clock Pre-scale Register 2"]
    pub pwm_clkpsc2_3: crate::Reg<pwm_clkpsc2_3::PWM_CLKPSC2_3_SPEC>,
    #[doc = "0x1c - PWM Clock Pre-scale Register 4"]
    pub pwm_clkpsc4_5: crate::Reg<pwm_clkpsc4_5::PWM_CLKPSC4_5_SPEC>,
    #[doc = "0x20 - PWM Counter Enable Register"]
    pub pwm_cnten: crate::Reg<pwm_cnten::PWM_CNTEN_SPEC>,
    #[doc = "0x24 - PWM Clear Counter Register"]
    pub pwm_cntclr: crate::Reg<pwm_cntclr::PWM_CNTCLR_SPEC>,
    #[doc = "0x28 - PWM Load Register"]
    pub pwm_load: crate::Reg<pwm_load::PWM_LOAD_SPEC>,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - PWM Period Register 0"]
    pub pwm_period0: crate::Reg<pwm_period0::PWM_PERIOD0_SPEC>,
    #[doc = "0x34 - PWM Period Register 1"]
    pub pwm_period1: crate::Reg<pwm_period1::PWM_PERIOD1_SPEC>,
    #[doc = "0x38 - PWM Period Register 2"]
    pub pwm_period2: crate::Reg<pwm_period2::PWM_PERIOD2_SPEC>,
    #[doc = "0x3c - PWM Period Register 3"]
    pub pwm_period3: crate::Reg<pwm_period3::PWM_PERIOD3_SPEC>,
    #[doc = "0x40 - PWM Period Register 4"]
    pub pwm_period4: crate::Reg<pwm_period4::PWM_PERIOD4_SPEC>,
    #[doc = "0x44 - PWM Period Register 5"]
    pub pwm_period5: crate::Reg<pwm_period5::PWM_PERIOD5_SPEC>,
    _reserved17: [u8; 8usize],
    #[doc = "0x50 - PWM Comparator Register 0"]
    pub pwm_cmpdat0: crate::Reg<pwm_cmpdat0::PWM_CMPDAT0_SPEC>,
    #[doc = "0x54 - PWM Comparator Register 1"]
    pub pwm_cmpdat1: crate::Reg<pwm_cmpdat1::PWM_CMPDAT1_SPEC>,
    #[doc = "0x58 - PWM Comparator Register 2"]
    pub pwm_cmpdat2: crate::Reg<pwm_cmpdat2::PWM_CMPDAT2_SPEC>,
    #[doc = "0x5c - PWM Comparator Register 3"]
    pub pwm_cmpdat3: crate::Reg<pwm_cmpdat3::PWM_CMPDAT3_SPEC>,
    #[doc = "0x60 - PWM Comparator Register 4"]
    pub pwm_cmpdat4: crate::Reg<pwm_cmpdat4::PWM_CMPDAT4_SPEC>,
    #[doc = "0x64 - PWM Comparator Register 5"]
    pub pwm_cmpdat5: crate::Reg<pwm_cmpdat5::PWM_CMPDAT5_SPEC>,
    _reserved23: [u8; 8usize],
    #[doc = "0x70 - PWM Dead-time Control Register 0"]
    pub pwm_dtctl0_1: crate::Reg<pwm_dtctl0_1::PWM_DTCTL0_1_SPEC>,
    #[doc = "0x74 - PWM Dead-time Control Register 2"]
    pub pwm_dtctl2_3: crate::Reg<pwm_dtctl2_3::PWM_DTCTL2_3_SPEC>,
    #[doc = "0x78 - PWM Dead-time Control Register 4"]
    pub pwm_dtctl4_5: crate::Reg<pwm_dtctl4_5::PWM_DTCTL4_5_SPEC>,
    _reserved26: [u8; 4usize],
    #[doc = "0x80 - PWM Counter Phase Register 0"]
    pub pwm_phs0_1: crate::Reg<pwm_phs0_1::PWM_PHS0_1_SPEC>,
    #[doc = "0x84 - PWM Counter Phase Register 2"]
    pub pwm_phs2_3: crate::Reg<pwm_phs2_3::PWM_PHS2_3_SPEC>,
    #[doc = "0x88 - PWM Counter Phase Register 4"]
    pub pwm_phs4_5: crate::Reg<pwm_phs4_5::PWM_PHS4_5_SPEC>,
    _reserved29: [u8; 4usize],
    #[doc = "0x90 - PWM Counter Register 0"]
    pub pwm_cnt0: crate::Reg<pwm_cnt0::PWM_CNT0_SPEC>,
    #[doc = "0x94 - PWM Counter Register 1"]
    pub pwm_cnt1: crate::Reg<pwm_cnt1::PWM_CNT1_SPEC>,
    #[doc = "0x98 - PWM Counter Register 2"]
    pub pwm_cnt2: crate::Reg<pwm_cnt2::PWM_CNT2_SPEC>,
    #[doc = "0x9c - PWM Counter Register 3"]
    pub pwm_cnt3: crate::Reg<pwm_cnt3::PWM_CNT3_SPEC>,
    #[doc = "0xa0 - PWM Counter Register 4"]
    pub pwm_cnt4: crate::Reg<pwm_cnt4::PWM_CNT4_SPEC>,
    #[doc = "0xa4 - PWM Counter Register 5"]
    pub pwm_cnt5: crate::Reg<pwm_cnt5::PWM_CNT5_SPEC>,
    _reserved35: [u8; 8usize],
    #[doc = "0xb0 - PWM Generation Register 0"]
    pub pwm_wgctl0: crate::Reg<pwm_wgctl0::PWM_WGCTL0_SPEC>,
    #[doc = "0xb4 - PWM Generation Register 1"]
    pub pwm_wgctl1: crate::Reg<pwm_wgctl1::PWM_WGCTL1_SPEC>,
    #[doc = "0xb8 - PWM Mask Enable Register"]
    pub pwm_msken: crate::Reg<pwm_msken::PWM_MSKEN_SPEC>,
    #[doc = "0xbc - PWM Mask Data Register"]
    pub pwm_msk: crate::Reg<pwm_msk::PWM_MSK_SPEC>,
    #[doc = "0xc0 - PWM Brake Noise Filter Register"]
    pub pwm_bnf: crate::Reg<pwm_bnf::PWM_BNF_SPEC>,
    #[doc = "0xc4 - PWM System Fail Brake Control Register"]
    pub pwm_failbrk: crate::Reg<pwm_failbrk::PWM_FAILBRK_SPEC>,
    #[doc = "0xc8 - PWM Brake Edge Detect Control Register 0"]
    pub pwm_brkctl0_1: crate::Reg<pwm_brkctl0_1::PWM_BRKCTL0_1_SPEC>,
    #[doc = "0xcc - PWM Brake Edge Detect Control Register 2"]
    pub pwm_brkctl2_3: crate::Reg<pwm_brkctl2_3::PWM_BRKCTL2_3_SPEC>,
    #[doc = "0xd0 - PWM Brake Edge Detect Control Register 4"]
    pub pwm_brkctl4_5: crate::Reg<pwm_brkctl4_5::PWM_BRKCTL4_5_SPEC>,
    #[doc = "0xd4 - PWM Pin Polar Inverse Register"]
    pub pwm_polctl: crate::Reg<pwm_polctl::PWM_POLCTL_SPEC>,
    #[doc = "0xd8 - PWM Output Enable Register"]
    pub pwm_poen: crate::Reg<pwm_poen::PWM_POEN_SPEC>,
    #[doc = "0xdc - PWM Software Brake Control Register"]
    pub pwm_swbrk: crate::Reg<pwm_swbrk::PWM_SWBRK_SPEC>,
    #[doc = "0xe0 - PWM Interrupt Enable Register 0"]
    pub pwm_inten0: crate::Reg<pwm_inten0::PWM_INTEN0_SPEC>,
    #[doc = "0xe4 - PWM Interrupt Enable Register 1"]
    pub pwm_inten1: crate::Reg<pwm_inten1::PWM_INTEN1_SPEC>,
    #[doc = "0xe8 - PWM Interrupt Flag Register 0"]
    pub pwm_intsts0: crate::Reg<pwm_intsts0::PWM_INTSTS0_SPEC>,
    #[doc = "0xec - PWM Interrupt Flag Register 1"]
    pub pwm_intsts1: crate::Reg<pwm_intsts1::PWM_INTSTS1_SPEC>,
    #[doc = "0xf0 - PWM Interrupt Flag Accumulator Register"]
    pub pwm_ifa: crate::Reg<pwm_ifa::PWM_IFA_SPEC>,
    #[doc = "0xf4 - PWM Trigger DAC Enable Register"]
    pub pwm_dactrgen: crate::Reg<pwm_dactrgen::PWM_DACTRGEN_SPEC>,
    #[doc = "0xf8 - PWM Trigger EADC Source Select Register 0"]
    pub pwm_eadcts0: crate::Reg<pwm_eadcts0::PWM_EADCTS0_SPEC>,
    #[doc = "0xfc - PWM Trigger EADC Source Select Register 1"]
    pub pwm_eadcts1: crate::Reg<pwm_eadcts1::PWM_EADCTS1_SPEC>,
    #[doc = "0x100 - PWM Free Trigger Compare Register 0"]
    pub pwm_ftcmpdat0_1: crate::Reg<pwm_ftcmpdat0_1::PWM_FTCMPDAT0_1_SPEC>,
    #[doc = "0x104 - PWM Free Trigger Compare Register 2"]
    pub pwm_ftcmpdat2_3: crate::Reg<pwm_ftcmpdat2_3::PWM_FTCMPDAT2_3_SPEC>,
    #[doc = "0x108 - PWM Free Trigger Compare Register 4"]
    pub pwm_ftcmpdat4_5: crate::Reg<pwm_ftcmpdat4_5::PWM_FTCMPDAT4_5_SPEC>,
    _reserved58: [u8; 4usize],
    #[doc = "0x110 - PWM Synchronous Start Control Register"]
    pub pwm_ssctl: crate::Reg<pwm_ssctl::PWM_SSCTL_SPEC>,
    #[doc = "0x114 - PWM Synchronous Start Trigger Register"]
    pub pwm_sstrg: crate::Reg<pwm_sstrg::PWM_SSTRG_SPEC>,
    _reserved60: [u8; 8usize],
    #[doc = "0x120 - PWM Status Register"]
    pub pwm_status: crate::Reg<pwm_status::PWM_STATUS_SPEC>,
    _reserved61: [u8; 220usize],
    #[doc = "0x200 - PWM Capture Input Enable Register"]
    pub pwm_capinen: crate::Reg<pwm_capinen::PWM_CAPINEN_SPEC>,
    #[doc = "0x204 - PWM Capture Control Register"]
    pub pwm_capctl: crate::Reg<pwm_capctl::PWM_CAPCTL_SPEC>,
    #[doc = "0x208 - PWM Capture Status Register"]
    pub pwm_capsts: crate::Reg<pwm_capsts::PWM_CAPSTS_SPEC>,
    #[doc = "0x20c - PWM Rising Capture Data Register 0"]
    pub pwm_rcapdat0: crate::Reg<pwm_rcapdat0::PWM_RCAPDAT0_SPEC>,
    #[doc = "0x210 - PWM Falling Capture Data Register 0"]
    pub pwm_fcapdat0: crate::Reg<pwm_fcapdat0::PWM_FCAPDAT0_SPEC>,
    #[doc = "0x214 - PWM Rising Capture Data Register 1"]
    pub pwm_rcapdat1: crate::Reg<pwm_rcapdat1::PWM_RCAPDAT1_SPEC>,
    #[doc = "0x218 - PWM Falling Capture Data Register 1"]
    pub pwm_fcapdat1: crate::Reg<pwm_fcapdat1::PWM_FCAPDAT1_SPEC>,
    #[doc = "0x21c - PWM Rising Capture Data Register 2"]
    pub pwm_rcapdat2: crate::Reg<pwm_rcapdat2::PWM_RCAPDAT2_SPEC>,
    #[doc = "0x220 - PWM Falling Capture Data Register 2"]
    pub pwm_fcapdat2: crate::Reg<pwm_fcapdat2::PWM_FCAPDAT2_SPEC>,
    #[doc = "0x224 - PWM Rising Capture Data Register 3"]
    pub pwm_rcapdat3: crate::Reg<pwm_rcapdat3::PWM_RCAPDAT3_SPEC>,
    #[doc = "0x228 - PWM Falling Capture Data Register 3"]
    pub pwm_fcapdat3: crate::Reg<pwm_fcapdat3::PWM_FCAPDAT3_SPEC>,
    #[doc = "0x22c - PWM Rising Capture Data Register 4"]
    pub pwm_rcapdat4: crate::Reg<pwm_rcapdat4::PWM_RCAPDAT4_SPEC>,
    #[doc = "0x230 - PWM Falling Capture Data Register 4"]
    pub pwm_fcapdat4: crate::Reg<pwm_fcapdat4::PWM_FCAPDAT4_SPEC>,
    #[doc = "0x234 - PWM Rising Capture Data Register 5"]
    pub pwm_rcapdat5: crate::Reg<pwm_rcapdat5::PWM_RCAPDAT5_SPEC>,
    #[doc = "0x238 - PWM Falling Capture Data Register 5"]
    pub pwm_fcapdat5: crate::Reg<pwm_fcapdat5::PWM_FCAPDAT5_SPEC>,
    #[doc = "0x23c - PWM PDMA Control Register"]
    pub pwm_pdmactl: crate::Reg<pwm_pdmactl::PWM_PDMACTL_SPEC>,
    #[doc = "0x240 - PWM Capture Channel 01 PDMA Register"]
    pub pwm_pdmacap0_1: crate::Reg<pwm_pdmacap0_1::PWM_PDMACAP0_1_SPEC>,
    #[doc = "0x244 - PWM Capture Channel 23 PDMA Register"]
    pub pwm_pdmacap2_3: crate::Reg<pwm_pdmacap2_3::PWM_PDMACAP2_3_SPEC>,
    #[doc = "0x248 - PWM Capture Channel 45 PDMA Register"]
    pub pwm_pdmacap4_5: crate::Reg<pwm_pdmacap4_5::PWM_PDMACAP4_5_SPEC>,
    _reserved80: [u8; 4usize],
    #[doc = "0x250 - PWM Capture Interrupt Enable Register"]
    pub pwm_capien: crate::Reg<pwm_capien::PWM_CAPIEN_SPEC>,
    #[doc = "0x254 - PWM Capture Interrupt Flag Register"]
    pub pwm_capif: crate::Reg<pwm_capif::PWM_CAPIF_SPEC>,
    _reserved82: [u8; 172usize],
    #[doc = "0x304 - PWM PERIOD0 Buffer"]
    pub pwm_pbuf0: crate::Reg<pwm_pbuf0::PWM_PBUF0_SPEC>,
    #[doc = "0x308 - PWM PERIOD1 Buffer"]
    pub pwm_pbuf1: crate::Reg<pwm_pbuf1::PWM_PBUF1_SPEC>,
    #[doc = "0x30c - PWM PERIOD2 Buffer"]
    pub pwm_pbuf2: crate::Reg<pwm_pbuf2::PWM_PBUF2_SPEC>,
    #[doc = "0x310 - PWM PERIOD3 Buffer"]
    pub pwm_pbuf3: crate::Reg<pwm_pbuf3::PWM_PBUF3_SPEC>,
    #[doc = "0x314 - PWM PERIOD4 Buffer"]
    pub pwm_pbuf4: crate::Reg<pwm_pbuf4::PWM_PBUF4_SPEC>,
    #[doc = "0x318 - PWM PERIOD5 Buffer"]
    pub pwm_pbuf5: crate::Reg<pwm_pbuf5::PWM_PBUF5_SPEC>,
    #[doc = "0x31c - PWM CMPDAT0 Buffer"]
    pub pwm_cmpbuf0: crate::Reg<pwm_cmpbuf0::PWM_CMPBUF0_SPEC>,
    #[doc = "0x320 - PWM CMPDAT1 Buffer"]
    pub pwm_cmpbuf1: crate::Reg<pwm_cmpbuf1::PWM_CMPBUF1_SPEC>,
    #[doc = "0x324 - PWM CMPDAT2 Buffer"]
    pub pwm_cmpbuf2: crate::Reg<pwm_cmpbuf2::PWM_CMPBUF2_SPEC>,
    #[doc = "0x328 - PWM CMPDAT3 Buffer"]
    pub pwm_cmpbuf3: crate::Reg<pwm_cmpbuf3::PWM_CMPBUF3_SPEC>,
    #[doc = "0x32c - PWM CMPDAT4 Buffer"]
    pub pwm_cmpbuf4: crate::Reg<pwm_cmpbuf4::PWM_CMPBUF4_SPEC>,
    #[doc = "0x330 - PWM CMPDAT5 Buffer"]
    pub pwm_cmpbuf5: crate::Reg<pwm_cmpbuf5::PWM_CMPBUF5_SPEC>,
    _reserved94: [u8; 12usize],
    #[doc = "0x340 - PWM FTCMPDAT0_1 Buffer"]
    pub pwm_ftcbuf0_1: crate::Reg<pwm_ftcbuf0_1::PWM_FTCBUF0_1_SPEC>,
    #[doc = "0x344 - PWM FTCMPDAT2_3 Buffer"]
    pub pwm_ftcbuf2_3: crate::Reg<pwm_ftcbuf2_3::PWM_FTCBUF2_3_SPEC>,
    #[doc = "0x348 - PWM FTCMPDAT4_5 Buffer"]
    pub pwm_ftcbuf4_5: crate::Reg<pwm_ftcbuf4_5::PWM_FTCBUF4_5_SPEC>,
    #[doc = "0x34c - PWM FTCMPDAT Indicator Register"]
    pub pwm_ftci: crate::Reg<pwm_ftci::PWM_FTCI_SPEC>,
}
#[doc = "PWM_CTL0 register accessor: an alias for `Reg<PWM_CTL0_SPEC>`"]
pub type PWM_CTL0 = crate::Reg<pwm_ctl0::PWM_CTL0_SPEC>;
#[doc = "PWM Control Register 0"]
pub mod pwm_ctl0;
#[doc = "PWM_CTL1 register accessor: an alias for `Reg<PWM_CTL1_SPEC>`"]
pub type PWM_CTL1 = crate::Reg<pwm_ctl1::PWM_CTL1_SPEC>;
#[doc = "PWM Control Register 1"]
pub mod pwm_ctl1;
#[doc = "PWM_SYNC register accessor: an alias for `Reg<PWM_SYNC_SPEC>`"]
pub type PWM_SYNC = crate::Reg<pwm_sync::PWM_SYNC_SPEC>;
#[doc = "PWM Synchronization Register"]
pub mod pwm_sync;
#[doc = "PWM_SWSYNC register accessor: an alias for `Reg<PWM_SWSYNC_SPEC>`"]
pub type PWM_SWSYNC = crate::Reg<pwm_swsync::PWM_SWSYNC_SPEC>;
#[doc = "PWM Software Control Synchronization Register"]
pub mod pwm_swsync;
#[doc = "PWM_CLKSRC register accessor: an alias for `Reg<PWM_CLKSRC_SPEC>`"]
pub type PWM_CLKSRC = crate::Reg<pwm_clksrc::PWM_CLKSRC_SPEC>;
#[doc = "PWM Clock Source Register"]
pub mod pwm_clksrc;
#[doc = "PWM_CLKPSC0_1 register accessor: an alias for `Reg<PWM_CLKPSC0_1_SPEC>`"]
pub type PWM_CLKPSC0_1 = crate::Reg<pwm_clkpsc0_1::PWM_CLKPSC0_1_SPEC>;
#[doc = "PWM Clock Pre-scale Register 0"]
pub mod pwm_clkpsc0_1;
#[doc = "PWM_CLKPSC2_3 register accessor: an alias for `Reg<PWM_CLKPSC2_3_SPEC>`"]
pub type PWM_CLKPSC2_3 = crate::Reg<pwm_clkpsc2_3::PWM_CLKPSC2_3_SPEC>;
#[doc = "PWM Clock Pre-scale Register 2"]
pub mod pwm_clkpsc2_3;
#[doc = "PWM_CLKPSC4_5 register accessor: an alias for `Reg<PWM_CLKPSC4_5_SPEC>`"]
pub type PWM_CLKPSC4_5 = crate::Reg<pwm_clkpsc4_5::PWM_CLKPSC4_5_SPEC>;
#[doc = "PWM Clock Pre-scale Register 4"]
pub mod pwm_clkpsc4_5;
#[doc = "PWM_CNTEN register accessor: an alias for `Reg<PWM_CNTEN_SPEC>`"]
pub type PWM_CNTEN = crate::Reg<pwm_cnten::PWM_CNTEN_SPEC>;
#[doc = "PWM Counter Enable Register"]
pub mod pwm_cnten;
#[doc = "PWM_CNTCLR register accessor: an alias for `Reg<PWM_CNTCLR_SPEC>`"]
pub type PWM_CNTCLR = crate::Reg<pwm_cntclr::PWM_CNTCLR_SPEC>;
#[doc = "PWM Clear Counter Register"]
pub mod pwm_cntclr;
#[doc = "PWM_LOAD register accessor: an alias for `Reg<PWM_LOAD_SPEC>`"]
pub type PWM_LOAD = crate::Reg<pwm_load::PWM_LOAD_SPEC>;
#[doc = "PWM Load Register"]
pub mod pwm_load;
#[doc = "PWM_PERIOD0 register accessor: an alias for `Reg<PWM_PERIOD0_SPEC>`"]
pub type PWM_PERIOD0 = crate::Reg<pwm_period0::PWM_PERIOD0_SPEC>;
#[doc = "PWM Period Register 0"]
pub mod pwm_period0;
#[doc = "PWM_PERIOD1 register accessor: an alias for `Reg<PWM_PERIOD1_SPEC>`"]
pub type PWM_PERIOD1 = crate::Reg<pwm_period1::PWM_PERIOD1_SPEC>;
#[doc = "PWM Period Register 1"]
pub mod pwm_period1;
#[doc = "PWM_PERIOD2 register accessor: an alias for `Reg<PWM_PERIOD2_SPEC>`"]
pub type PWM_PERIOD2 = crate::Reg<pwm_period2::PWM_PERIOD2_SPEC>;
#[doc = "PWM Period Register 2"]
pub mod pwm_period2;
#[doc = "PWM_PERIOD3 register accessor: an alias for `Reg<PWM_PERIOD3_SPEC>`"]
pub type PWM_PERIOD3 = crate::Reg<pwm_period3::PWM_PERIOD3_SPEC>;
#[doc = "PWM Period Register 3"]
pub mod pwm_period3;
#[doc = "PWM_PERIOD4 register accessor: an alias for `Reg<PWM_PERIOD4_SPEC>`"]
pub type PWM_PERIOD4 = crate::Reg<pwm_period4::PWM_PERIOD4_SPEC>;
#[doc = "PWM Period Register 4"]
pub mod pwm_period4;
#[doc = "PWM_PERIOD5 register accessor: an alias for `Reg<PWM_PERIOD5_SPEC>`"]
pub type PWM_PERIOD5 = crate::Reg<pwm_period5::PWM_PERIOD5_SPEC>;
#[doc = "PWM Period Register 5"]
pub mod pwm_period5;
#[doc = "PWM_CMPDAT0 register accessor: an alias for `Reg<PWM_CMPDAT0_SPEC>`"]
pub type PWM_CMPDAT0 = crate::Reg<pwm_cmpdat0::PWM_CMPDAT0_SPEC>;
#[doc = "PWM Comparator Register 0"]
pub mod pwm_cmpdat0;
#[doc = "PWM_CMPDAT1 register accessor: an alias for `Reg<PWM_CMPDAT1_SPEC>`"]
pub type PWM_CMPDAT1 = crate::Reg<pwm_cmpdat1::PWM_CMPDAT1_SPEC>;
#[doc = "PWM Comparator Register 1"]
pub mod pwm_cmpdat1;
#[doc = "PWM_CMPDAT2 register accessor: an alias for `Reg<PWM_CMPDAT2_SPEC>`"]
pub type PWM_CMPDAT2 = crate::Reg<pwm_cmpdat2::PWM_CMPDAT2_SPEC>;
#[doc = "PWM Comparator Register 2"]
pub mod pwm_cmpdat2;
#[doc = "PWM_CMPDAT3 register accessor: an alias for `Reg<PWM_CMPDAT3_SPEC>`"]
pub type PWM_CMPDAT3 = crate::Reg<pwm_cmpdat3::PWM_CMPDAT3_SPEC>;
#[doc = "PWM Comparator Register 3"]
pub mod pwm_cmpdat3;
#[doc = "PWM_CMPDAT4 register accessor: an alias for `Reg<PWM_CMPDAT4_SPEC>`"]
pub type PWM_CMPDAT4 = crate::Reg<pwm_cmpdat4::PWM_CMPDAT4_SPEC>;
#[doc = "PWM Comparator Register 4"]
pub mod pwm_cmpdat4;
#[doc = "PWM_CMPDAT5 register accessor: an alias for `Reg<PWM_CMPDAT5_SPEC>`"]
pub type PWM_CMPDAT5 = crate::Reg<pwm_cmpdat5::PWM_CMPDAT5_SPEC>;
#[doc = "PWM Comparator Register 5"]
pub mod pwm_cmpdat5;
#[doc = "PWM_DTCTL0_1 register accessor: an alias for `Reg<PWM_DTCTL0_1_SPEC>`"]
pub type PWM_DTCTL0_1 = crate::Reg<pwm_dtctl0_1::PWM_DTCTL0_1_SPEC>;
#[doc = "PWM Dead-time Control Register 0"]
pub mod pwm_dtctl0_1;
#[doc = "PWM_DTCTL2_3 register accessor: an alias for `Reg<PWM_DTCTL2_3_SPEC>`"]
pub type PWM_DTCTL2_3 = crate::Reg<pwm_dtctl2_3::PWM_DTCTL2_3_SPEC>;
#[doc = "PWM Dead-time Control Register 2"]
pub mod pwm_dtctl2_3;
#[doc = "PWM_DTCTL4_5 register accessor: an alias for `Reg<PWM_DTCTL4_5_SPEC>`"]
pub type PWM_DTCTL4_5 = crate::Reg<pwm_dtctl4_5::PWM_DTCTL4_5_SPEC>;
#[doc = "PWM Dead-time Control Register 4"]
pub mod pwm_dtctl4_5;
#[doc = "PWM_PHS0_1 register accessor: an alias for `Reg<PWM_PHS0_1_SPEC>`"]
pub type PWM_PHS0_1 = crate::Reg<pwm_phs0_1::PWM_PHS0_1_SPEC>;
#[doc = "PWM Counter Phase Register 0"]
pub mod pwm_phs0_1;
#[doc = "PWM_PHS2_3 register accessor: an alias for `Reg<PWM_PHS2_3_SPEC>`"]
pub type PWM_PHS2_3 = crate::Reg<pwm_phs2_3::PWM_PHS2_3_SPEC>;
#[doc = "PWM Counter Phase Register 2"]
pub mod pwm_phs2_3;
#[doc = "PWM_PHS4_5 register accessor: an alias for `Reg<PWM_PHS4_5_SPEC>`"]
pub type PWM_PHS4_5 = crate::Reg<pwm_phs4_5::PWM_PHS4_5_SPEC>;
#[doc = "PWM Counter Phase Register 4"]
pub mod pwm_phs4_5;
#[doc = "PWM_CNT0 register accessor: an alias for `Reg<PWM_CNT0_SPEC>`"]
pub type PWM_CNT0 = crate::Reg<pwm_cnt0::PWM_CNT0_SPEC>;
#[doc = "PWM Counter Register 0"]
pub mod pwm_cnt0;
#[doc = "PWM_CNT1 register accessor: an alias for `Reg<PWM_CNT1_SPEC>`"]
pub type PWM_CNT1 = crate::Reg<pwm_cnt1::PWM_CNT1_SPEC>;
#[doc = "PWM Counter Register 1"]
pub mod pwm_cnt1;
#[doc = "PWM_CNT2 register accessor: an alias for `Reg<PWM_CNT2_SPEC>`"]
pub type PWM_CNT2 = crate::Reg<pwm_cnt2::PWM_CNT2_SPEC>;
#[doc = "PWM Counter Register 2"]
pub mod pwm_cnt2;
#[doc = "PWM_CNT3 register accessor: an alias for `Reg<PWM_CNT3_SPEC>`"]
pub type PWM_CNT3 = crate::Reg<pwm_cnt3::PWM_CNT3_SPEC>;
#[doc = "PWM Counter Register 3"]
pub mod pwm_cnt3;
#[doc = "PWM_CNT4 register accessor: an alias for `Reg<PWM_CNT4_SPEC>`"]
pub type PWM_CNT4 = crate::Reg<pwm_cnt4::PWM_CNT4_SPEC>;
#[doc = "PWM Counter Register 4"]
pub mod pwm_cnt4;
#[doc = "PWM_CNT5 register accessor: an alias for `Reg<PWM_CNT5_SPEC>`"]
pub type PWM_CNT5 = crate::Reg<pwm_cnt5::PWM_CNT5_SPEC>;
#[doc = "PWM Counter Register 5"]
pub mod pwm_cnt5;
#[doc = "PWM_WGCTL0 register accessor: an alias for `Reg<PWM_WGCTL0_SPEC>`"]
pub type PWM_WGCTL0 = crate::Reg<pwm_wgctl0::PWM_WGCTL0_SPEC>;
#[doc = "PWM Generation Register 0"]
pub mod pwm_wgctl0;
#[doc = "PWM_WGCTL1 register accessor: an alias for `Reg<PWM_WGCTL1_SPEC>`"]
pub type PWM_WGCTL1 = crate::Reg<pwm_wgctl1::PWM_WGCTL1_SPEC>;
#[doc = "PWM Generation Register 1"]
pub mod pwm_wgctl1;
#[doc = "PWM_MSKEN register accessor: an alias for `Reg<PWM_MSKEN_SPEC>`"]
pub type PWM_MSKEN = crate::Reg<pwm_msken::PWM_MSKEN_SPEC>;
#[doc = "PWM Mask Enable Register"]
pub mod pwm_msken;
#[doc = "PWM_MSK register accessor: an alias for `Reg<PWM_MSK_SPEC>`"]
pub type PWM_MSK = crate::Reg<pwm_msk::PWM_MSK_SPEC>;
#[doc = "PWM Mask Data Register"]
pub mod pwm_msk;
#[doc = "PWM_BNF register accessor: an alias for `Reg<PWM_BNF_SPEC>`"]
pub type PWM_BNF = crate::Reg<pwm_bnf::PWM_BNF_SPEC>;
#[doc = "PWM Brake Noise Filter Register"]
pub mod pwm_bnf;
#[doc = "PWM_FAILBRK register accessor: an alias for `Reg<PWM_FAILBRK_SPEC>`"]
pub type PWM_FAILBRK = crate::Reg<pwm_failbrk::PWM_FAILBRK_SPEC>;
#[doc = "PWM System Fail Brake Control Register"]
pub mod pwm_failbrk;
#[doc = "PWM_BRKCTL0_1 register accessor: an alias for `Reg<PWM_BRKCTL0_1_SPEC>`"]
pub type PWM_BRKCTL0_1 = crate::Reg<pwm_brkctl0_1::PWM_BRKCTL0_1_SPEC>;
#[doc = "PWM Brake Edge Detect Control Register 0"]
pub mod pwm_brkctl0_1;
#[doc = "PWM_BRKCTL2_3 register accessor: an alias for `Reg<PWM_BRKCTL2_3_SPEC>`"]
pub type PWM_BRKCTL2_3 = crate::Reg<pwm_brkctl2_3::PWM_BRKCTL2_3_SPEC>;
#[doc = "PWM Brake Edge Detect Control Register 2"]
pub mod pwm_brkctl2_3;
#[doc = "PWM_BRKCTL4_5 register accessor: an alias for `Reg<PWM_BRKCTL4_5_SPEC>`"]
pub type PWM_BRKCTL4_5 = crate::Reg<pwm_brkctl4_5::PWM_BRKCTL4_5_SPEC>;
#[doc = "PWM Brake Edge Detect Control Register 4"]
pub mod pwm_brkctl4_5;
#[doc = "PWM_POLCTL register accessor: an alias for `Reg<PWM_POLCTL_SPEC>`"]
pub type PWM_POLCTL = crate::Reg<pwm_polctl::PWM_POLCTL_SPEC>;
#[doc = "PWM Pin Polar Inverse Register"]
pub mod pwm_polctl;
#[doc = "PWM_POEN register accessor: an alias for `Reg<PWM_POEN_SPEC>`"]
pub type PWM_POEN = crate::Reg<pwm_poen::PWM_POEN_SPEC>;
#[doc = "PWM Output Enable Register"]
pub mod pwm_poen;
#[doc = "PWM_SWBRK register accessor: an alias for `Reg<PWM_SWBRK_SPEC>`"]
pub type PWM_SWBRK = crate::Reg<pwm_swbrk::PWM_SWBRK_SPEC>;
#[doc = "PWM Software Brake Control Register"]
pub mod pwm_swbrk;
#[doc = "PWM_INTEN0 register accessor: an alias for `Reg<PWM_INTEN0_SPEC>`"]
pub type PWM_INTEN0 = crate::Reg<pwm_inten0::PWM_INTEN0_SPEC>;
#[doc = "PWM Interrupt Enable Register 0"]
pub mod pwm_inten0;
#[doc = "PWM_INTEN1 register accessor: an alias for `Reg<PWM_INTEN1_SPEC>`"]
pub type PWM_INTEN1 = crate::Reg<pwm_inten1::PWM_INTEN1_SPEC>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod pwm_inten1;
#[doc = "PWM_INTSTS0 register accessor: an alias for `Reg<PWM_INTSTS0_SPEC>`"]
pub type PWM_INTSTS0 = crate::Reg<pwm_intsts0::PWM_INTSTS0_SPEC>;
#[doc = "PWM Interrupt Flag Register 0"]
pub mod pwm_intsts0;
#[doc = "PWM_INTSTS1 register accessor: an alias for `Reg<PWM_INTSTS1_SPEC>`"]
pub type PWM_INTSTS1 = crate::Reg<pwm_intsts1::PWM_INTSTS1_SPEC>;
#[doc = "PWM Interrupt Flag Register 1"]
pub mod pwm_intsts1;
#[doc = "PWM_IFA register accessor: an alias for `Reg<PWM_IFA_SPEC>`"]
pub type PWM_IFA = crate::Reg<pwm_ifa::PWM_IFA_SPEC>;
#[doc = "PWM Interrupt Flag Accumulator Register"]
pub mod pwm_ifa;
#[doc = "PWM_DACTRGEN register accessor: an alias for `Reg<PWM_DACTRGEN_SPEC>`"]
pub type PWM_DACTRGEN = crate::Reg<pwm_dactrgen::PWM_DACTRGEN_SPEC>;
#[doc = "PWM Trigger DAC Enable Register"]
pub mod pwm_dactrgen;
#[doc = "PWM_EADCTS0 register accessor: an alias for `Reg<PWM_EADCTS0_SPEC>`"]
pub type PWM_EADCTS0 = crate::Reg<pwm_eadcts0::PWM_EADCTS0_SPEC>;
#[doc = "PWM Trigger EADC Source Select Register 0"]
pub mod pwm_eadcts0;
#[doc = "PWM_EADCTS1 register accessor: an alias for `Reg<PWM_EADCTS1_SPEC>`"]
pub type PWM_EADCTS1 = crate::Reg<pwm_eadcts1::PWM_EADCTS1_SPEC>;
#[doc = "PWM Trigger EADC Source Select Register 1"]
pub mod pwm_eadcts1;
#[doc = "PWM_FTCMPDAT0_1 register accessor: an alias for `Reg<PWM_FTCMPDAT0_1_SPEC>`"]
pub type PWM_FTCMPDAT0_1 = crate::Reg<pwm_ftcmpdat0_1::PWM_FTCMPDAT0_1_SPEC>;
#[doc = "PWM Free Trigger Compare Register 0"]
pub mod pwm_ftcmpdat0_1;
#[doc = "PWM_FTCMPDAT2_3 register accessor: an alias for `Reg<PWM_FTCMPDAT2_3_SPEC>`"]
pub type PWM_FTCMPDAT2_3 = crate::Reg<pwm_ftcmpdat2_3::PWM_FTCMPDAT2_3_SPEC>;
#[doc = "PWM Free Trigger Compare Register 2"]
pub mod pwm_ftcmpdat2_3;
#[doc = "PWM_FTCMPDAT4_5 register accessor: an alias for `Reg<PWM_FTCMPDAT4_5_SPEC>`"]
pub type PWM_FTCMPDAT4_5 = crate::Reg<pwm_ftcmpdat4_5::PWM_FTCMPDAT4_5_SPEC>;
#[doc = "PWM Free Trigger Compare Register 4"]
pub mod pwm_ftcmpdat4_5;
#[doc = "PWM_SSCTL register accessor: an alias for `Reg<PWM_SSCTL_SPEC>`"]
pub type PWM_SSCTL = crate::Reg<pwm_ssctl::PWM_SSCTL_SPEC>;
#[doc = "PWM Synchronous Start Control Register"]
pub mod pwm_ssctl;
#[doc = "PWM_SSTRG register accessor: an alias for `Reg<PWM_SSTRG_SPEC>`"]
pub type PWM_SSTRG = crate::Reg<pwm_sstrg::PWM_SSTRG_SPEC>;
#[doc = "PWM Synchronous Start Trigger Register"]
pub mod pwm_sstrg;
#[doc = "PWM_STATUS register accessor: an alias for `Reg<PWM_STATUS_SPEC>`"]
pub type PWM_STATUS = crate::Reg<pwm_status::PWM_STATUS_SPEC>;
#[doc = "PWM Status Register"]
pub mod pwm_status;
#[doc = "PWM_CAPINEN register accessor: an alias for `Reg<PWM_CAPINEN_SPEC>`"]
pub type PWM_CAPINEN = crate::Reg<pwm_capinen::PWM_CAPINEN_SPEC>;
#[doc = "PWM Capture Input Enable Register"]
pub mod pwm_capinen;
#[doc = "PWM_CAPCTL register accessor: an alias for `Reg<PWM_CAPCTL_SPEC>`"]
pub type PWM_CAPCTL = crate::Reg<pwm_capctl::PWM_CAPCTL_SPEC>;
#[doc = "PWM Capture Control Register"]
pub mod pwm_capctl;
#[doc = "PWM_CAPSTS register accessor: an alias for `Reg<PWM_CAPSTS_SPEC>`"]
pub type PWM_CAPSTS = crate::Reg<pwm_capsts::PWM_CAPSTS_SPEC>;
#[doc = "PWM Capture Status Register"]
pub mod pwm_capsts;
#[doc = "PWM_RCAPDAT0 register accessor: an alias for `Reg<PWM_RCAPDAT0_SPEC>`"]
pub type PWM_RCAPDAT0 = crate::Reg<pwm_rcapdat0::PWM_RCAPDAT0_SPEC>;
#[doc = "PWM Rising Capture Data Register 0"]
pub mod pwm_rcapdat0;
#[doc = "PWM_FCAPDAT0 register accessor: an alias for `Reg<PWM_FCAPDAT0_SPEC>`"]
pub type PWM_FCAPDAT0 = crate::Reg<pwm_fcapdat0::PWM_FCAPDAT0_SPEC>;
#[doc = "PWM Falling Capture Data Register 0"]
pub mod pwm_fcapdat0;
#[doc = "PWM_RCAPDAT1 register accessor: an alias for `Reg<PWM_RCAPDAT1_SPEC>`"]
pub type PWM_RCAPDAT1 = crate::Reg<pwm_rcapdat1::PWM_RCAPDAT1_SPEC>;
#[doc = "PWM Rising Capture Data Register 1"]
pub mod pwm_rcapdat1;
#[doc = "PWM_FCAPDAT1 register accessor: an alias for `Reg<PWM_FCAPDAT1_SPEC>`"]
pub type PWM_FCAPDAT1 = crate::Reg<pwm_fcapdat1::PWM_FCAPDAT1_SPEC>;
#[doc = "PWM Falling Capture Data Register 1"]
pub mod pwm_fcapdat1;
#[doc = "PWM_RCAPDAT2 register accessor: an alias for `Reg<PWM_RCAPDAT2_SPEC>`"]
pub type PWM_RCAPDAT2 = crate::Reg<pwm_rcapdat2::PWM_RCAPDAT2_SPEC>;
#[doc = "PWM Rising Capture Data Register 2"]
pub mod pwm_rcapdat2;
#[doc = "PWM_FCAPDAT2 register accessor: an alias for `Reg<PWM_FCAPDAT2_SPEC>`"]
pub type PWM_FCAPDAT2 = crate::Reg<pwm_fcapdat2::PWM_FCAPDAT2_SPEC>;
#[doc = "PWM Falling Capture Data Register 2"]
pub mod pwm_fcapdat2;
#[doc = "PWM_RCAPDAT3 register accessor: an alias for `Reg<PWM_RCAPDAT3_SPEC>`"]
pub type PWM_RCAPDAT3 = crate::Reg<pwm_rcapdat3::PWM_RCAPDAT3_SPEC>;
#[doc = "PWM Rising Capture Data Register 3"]
pub mod pwm_rcapdat3;
#[doc = "PWM_FCAPDAT3 register accessor: an alias for `Reg<PWM_FCAPDAT3_SPEC>`"]
pub type PWM_FCAPDAT3 = crate::Reg<pwm_fcapdat3::PWM_FCAPDAT3_SPEC>;
#[doc = "PWM Falling Capture Data Register 3"]
pub mod pwm_fcapdat3;
#[doc = "PWM_RCAPDAT4 register accessor: an alias for `Reg<PWM_RCAPDAT4_SPEC>`"]
pub type PWM_RCAPDAT4 = crate::Reg<pwm_rcapdat4::PWM_RCAPDAT4_SPEC>;
#[doc = "PWM Rising Capture Data Register 4"]
pub mod pwm_rcapdat4;
#[doc = "PWM_FCAPDAT4 register accessor: an alias for `Reg<PWM_FCAPDAT4_SPEC>`"]
pub type PWM_FCAPDAT4 = crate::Reg<pwm_fcapdat4::PWM_FCAPDAT4_SPEC>;
#[doc = "PWM Falling Capture Data Register 4"]
pub mod pwm_fcapdat4;
#[doc = "PWM_RCAPDAT5 register accessor: an alias for `Reg<PWM_RCAPDAT5_SPEC>`"]
pub type PWM_RCAPDAT5 = crate::Reg<pwm_rcapdat5::PWM_RCAPDAT5_SPEC>;
#[doc = "PWM Rising Capture Data Register 5"]
pub mod pwm_rcapdat5;
#[doc = "PWM_FCAPDAT5 register accessor: an alias for `Reg<PWM_FCAPDAT5_SPEC>`"]
pub type PWM_FCAPDAT5 = crate::Reg<pwm_fcapdat5::PWM_FCAPDAT5_SPEC>;
#[doc = "PWM Falling Capture Data Register 5"]
pub mod pwm_fcapdat5;
#[doc = "PWM_PDMACTL register accessor: an alias for `Reg<PWM_PDMACTL_SPEC>`"]
pub type PWM_PDMACTL = crate::Reg<pwm_pdmactl::PWM_PDMACTL_SPEC>;
#[doc = "PWM PDMA Control Register"]
pub mod pwm_pdmactl;
#[doc = "PWM_PDMACAP0_1 register accessor: an alias for `Reg<PWM_PDMACAP0_1_SPEC>`"]
pub type PWM_PDMACAP0_1 = crate::Reg<pwm_pdmacap0_1::PWM_PDMACAP0_1_SPEC>;
#[doc = "PWM Capture Channel 01 PDMA Register"]
pub mod pwm_pdmacap0_1;
#[doc = "PWM_PDMACAP2_3 register accessor: an alias for `Reg<PWM_PDMACAP2_3_SPEC>`"]
pub type PWM_PDMACAP2_3 = crate::Reg<pwm_pdmacap2_3::PWM_PDMACAP2_3_SPEC>;
#[doc = "PWM Capture Channel 23 PDMA Register"]
pub mod pwm_pdmacap2_3;
#[doc = "PWM_PDMACAP4_5 register accessor: an alias for `Reg<PWM_PDMACAP4_5_SPEC>`"]
pub type PWM_PDMACAP4_5 = crate::Reg<pwm_pdmacap4_5::PWM_PDMACAP4_5_SPEC>;
#[doc = "PWM Capture Channel 45 PDMA Register"]
pub mod pwm_pdmacap4_5;
#[doc = "PWM_CAPIEN register accessor: an alias for `Reg<PWM_CAPIEN_SPEC>`"]
pub type PWM_CAPIEN = crate::Reg<pwm_capien::PWM_CAPIEN_SPEC>;
#[doc = "PWM Capture Interrupt Enable Register"]
pub mod pwm_capien;
#[doc = "PWM_CAPIF register accessor: an alias for `Reg<PWM_CAPIF_SPEC>`"]
pub type PWM_CAPIF = crate::Reg<pwm_capif::PWM_CAPIF_SPEC>;
#[doc = "PWM Capture Interrupt Flag Register"]
pub mod pwm_capif;
#[doc = "PWM_PBUF0 register accessor: an alias for `Reg<PWM_PBUF0_SPEC>`"]
pub type PWM_PBUF0 = crate::Reg<pwm_pbuf0::PWM_PBUF0_SPEC>;
#[doc = "PWM PERIOD0 Buffer"]
pub mod pwm_pbuf0;
#[doc = "PWM_PBUF1 register accessor: an alias for `Reg<PWM_PBUF1_SPEC>`"]
pub type PWM_PBUF1 = crate::Reg<pwm_pbuf1::PWM_PBUF1_SPEC>;
#[doc = "PWM PERIOD1 Buffer"]
pub mod pwm_pbuf1;
#[doc = "PWM_PBUF2 register accessor: an alias for `Reg<PWM_PBUF2_SPEC>`"]
pub type PWM_PBUF2 = crate::Reg<pwm_pbuf2::PWM_PBUF2_SPEC>;
#[doc = "PWM PERIOD2 Buffer"]
pub mod pwm_pbuf2;
#[doc = "PWM_PBUF3 register accessor: an alias for `Reg<PWM_PBUF3_SPEC>`"]
pub type PWM_PBUF3 = crate::Reg<pwm_pbuf3::PWM_PBUF3_SPEC>;
#[doc = "PWM PERIOD3 Buffer"]
pub mod pwm_pbuf3;
#[doc = "PWM_PBUF4 register accessor: an alias for `Reg<PWM_PBUF4_SPEC>`"]
pub type PWM_PBUF4 = crate::Reg<pwm_pbuf4::PWM_PBUF4_SPEC>;
#[doc = "PWM PERIOD4 Buffer"]
pub mod pwm_pbuf4;
#[doc = "PWM_PBUF5 register accessor: an alias for `Reg<PWM_PBUF5_SPEC>`"]
pub type PWM_PBUF5 = crate::Reg<pwm_pbuf5::PWM_PBUF5_SPEC>;
#[doc = "PWM PERIOD5 Buffer"]
pub mod pwm_pbuf5;
#[doc = "PWM_CMPBUF0 register accessor: an alias for `Reg<PWM_CMPBUF0_SPEC>`"]
pub type PWM_CMPBUF0 = crate::Reg<pwm_cmpbuf0::PWM_CMPBUF0_SPEC>;
#[doc = "PWM CMPDAT0 Buffer"]
pub mod pwm_cmpbuf0;
#[doc = "PWM_CMPBUF1 register accessor: an alias for `Reg<PWM_CMPBUF1_SPEC>`"]
pub type PWM_CMPBUF1 = crate::Reg<pwm_cmpbuf1::PWM_CMPBUF1_SPEC>;
#[doc = "PWM CMPDAT1 Buffer"]
pub mod pwm_cmpbuf1;
#[doc = "PWM_CMPBUF2 register accessor: an alias for `Reg<PWM_CMPBUF2_SPEC>`"]
pub type PWM_CMPBUF2 = crate::Reg<pwm_cmpbuf2::PWM_CMPBUF2_SPEC>;
#[doc = "PWM CMPDAT2 Buffer"]
pub mod pwm_cmpbuf2;
#[doc = "PWM_CMPBUF3 register accessor: an alias for `Reg<PWM_CMPBUF3_SPEC>`"]
pub type PWM_CMPBUF3 = crate::Reg<pwm_cmpbuf3::PWM_CMPBUF3_SPEC>;
#[doc = "PWM CMPDAT3 Buffer"]
pub mod pwm_cmpbuf3;
#[doc = "PWM_CMPBUF4 register accessor: an alias for `Reg<PWM_CMPBUF4_SPEC>`"]
pub type PWM_CMPBUF4 = crate::Reg<pwm_cmpbuf4::PWM_CMPBUF4_SPEC>;
#[doc = "PWM CMPDAT4 Buffer"]
pub mod pwm_cmpbuf4;
#[doc = "PWM_CMPBUF5 register accessor: an alias for `Reg<PWM_CMPBUF5_SPEC>`"]
pub type PWM_CMPBUF5 = crate::Reg<pwm_cmpbuf5::PWM_CMPBUF5_SPEC>;
#[doc = "PWM CMPDAT5 Buffer"]
pub mod pwm_cmpbuf5;
#[doc = "PWM_FTCBUF0_1 register accessor: an alias for `Reg<PWM_FTCBUF0_1_SPEC>`"]
pub type PWM_FTCBUF0_1 = crate::Reg<pwm_ftcbuf0_1::PWM_FTCBUF0_1_SPEC>;
#[doc = "PWM FTCMPDAT0_1 Buffer"]
pub mod pwm_ftcbuf0_1;
#[doc = "PWM_FTCBUF2_3 register accessor: an alias for `Reg<PWM_FTCBUF2_3_SPEC>`"]
pub type PWM_FTCBUF2_3 = crate::Reg<pwm_ftcbuf2_3::PWM_FTCBUF2_3_SPEC>;
#[doc = "PWM FTCMPDAT2_3 Buffer"]
pub mod pwm_ftcbuf2_3;
#[doc = "PWM_FTCBUF4_5 register accessor: an alias for `Reg<PWM_FTCBUF4_5_SPEC>`"]
pub type PWM_FTCBUF4_5 = crate::Reg<pwm_ftcbuf4_5::PWM_FTCBUF4_5_SPEC>;
#[doc = "PWM FTCMPDAT4_5 Buffer"]
pub mod pwm_ftcbuf4_5;
#[doc = "PWM_FTCI register accessor: an alias for `Reg<PWM_FTCI_SPEC>`"]
pub type PWM_FTCI = crate::Reg<pwm_ftci::PWM_FTCI_SPEC>;
#[doc = "PWM FTCMPDAT Indicator Register"]
pub mod pwm_ftci;
