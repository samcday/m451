#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PA I/O Mode Control"]
    pub pa_mode: crate::Reg<pa_mode::PA_MODE_SPEC>,
    #[doc = "0x04 - PA Digital Input Path Disable Control"]
    pub pa_dinoff: crate::Reg<pa_dinoff::PA_DINOFF_SPEC>,
    #[doc = "0x08 - PA Data Output Value"]
    pub pa_dout: crate::Reg<pa_dout::PA_DOUT_SPEC>,
    #[doc = "0x0c - PA Data Output Write Mask"]
    pub pa_datmsk: crate::Reg<pa_datmsk::PA_DATMSK_SPEC>,
    #[doc = "0x10 - PA Pin Value"]
    pub pa_pin: crate::Reg<pa_pin::PA_PIN_SPEC>,
    #[doc = "0x14 - PA De-bounce Enable Control Register"]
    pub pa_dben: crate::Reg<pa_dben::PA_DBEN_SPEC>,
    #[doc = "0x18 - PA Interrupt Trigger Type Control"]
    pub pa_inttype: crate::Reg<pa_inttype::PA_INTTYPE_SPEC>,
    #[doc = "0x1c - PA Interrupt Enable Control Register"]
    pub pa_inten: crate::Reg<pa_inten::PA_INTEN_SPEC>,
    #[doc = "0x20 - PA Interrupt Source Flag"]
    pub pa_intsrc: crate::Reg<pa_intsrc::PA_INTSRC_SPEC>,
    #[doc = "0x24 - PA Input Schmitt Trigger Enable Register"]
    pub pa_smten: crate::Reg<pa_smten::PA_SMTEN_SPEC>,
    #[doc = "0x28 - PA High Slew Rate Control Register"]
    pub pa_slewctl: crate::Reg<pa_slewctl::PA_SLEWCTL_SPEC>,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - PB I/O Mode Control"]
    pub pb_mode: crate::Reg<pb_mode::PB_MODE_SPEC>,
    #[doc = "0x44 - PB Digital Input Path Disable Control"]
    pub pb_dinoff: crate::Reg<pb_dinoff::PB_DINOFF_SPEC>,
    #[doc = "0x48 - PB Data Output Value"]
    pub pb_dout: crate::Reg<pb_dout::PB_DOUT_SPEC>,
    #[doc = "0x4c - PB Data Output Write Mask"]
    pub pb_datmsk: crate::Reg<pb_datmsk::PB_DATMSK_SPEC>,
    #[doc = "0x50 - PB Pin Value"]
    pub pb_pin: crate::Reg<pb_pin::PB_PIN_SPEC>,
    #[doc = "0x54 - PB De-bounce Enable Control Register"]
    pub pb_dben: crate::Reg<pb_dben::PB_DBEN_SPEC>,
    #[doc = "0x58 - PB Interrupt Trigger Type Control"]
    pub pb_inttype: crate::Reg<pb_inttype::PB_INTTYPE_SPEC>,
    #[doc = "0x5c - PB Interrupt Enable Control Register"]
    pub pb_inten: crate::Reg<pb_inten::PB_INTEN_SPEC>,
    #[doc = "0x60 - PB Interrupt Source Flag"]
    pub pb_intsrc: crate::Reg<pb_intsrc::PB_INTSRC_SPEC>,
    #[doc = "0x64 - PB Input Schmitt Trigger Enable Register"]
    pub pb_smten: crate::Reg<pb_smten::PB_SMTEN_SPEC>,
    #[doc = "0x68 - PB High Slew Rate Control Register"]
    pub pb_slewctl: crate::Reg<pb_slewctl::PB_SLEWCTL_SPEC>,
    _reserved22: [u8; 20usize],
    #[doc = "0x80 - PC I/O Mode Control"]
    pub pc_mode: crate::Reg<pc_mode::PC_MODE_SPEC>,
    #[doc = "0x84 - PC Digital Input Path Disable Control"]
    pub pc_dinoff: crate::Reg<pc_dinoff::PC_DINOFF_SPEC>,
    #[doc = "0x88 - PC Data Output Value"]
    pub pc_dout: crate::Reg<pc_dout::PC_DOUT_SPEC>,
    #[doc = "0x8c - PC Data Output Write Mask"]
    pub pc_datmsk: crate::Reg<pc_datmsk::PC_DATMSK_SPEC>,
    #[doc = "0x90 - PC Pin Value"]
    pub pc_pin: crate::Reg<pc_pin::PC_PIN_SPEC>,
    #[doc = "0x94 - PC De-bounce Enable Control Register"]
    pub pc_dben: crate::Reg<pc_dben::PC_DBEN_SPEC>,
    #[doc = "0x98 - PC Interrupt Trigger Type Control"]
    pub pc_inttype: crate::Reg<pc_inttype::PC_INTTYPE_SPEC>,
    #[doc = "0x9c - PC Interrupt Enable Control Register"]
    pub pc_inten: crate::Reg<pc_inten::PC_INTEN_SPEC>,
    #[doc = "0xa0 - PC Interrupt Source Flag"]
    pub pc_intsrc: crate::Reg<pc_intsrc::PC_INTSRC_SPEC>,
    #[doc = "0xa4 - PC Input Schmitt Trigger Enable Register"]
    pub pc_smten: crate::Reg<pc_smten::PC_SMTEN_SPEC>,
    #[doc = "0xa8 - PC High Slew Rate Control Register"]
    pub pc_slewctl: crate::Reg<pc_slewctl::PC_SLEWCTL_SPEC>,
    _reserved33: [u8; 20usize],
    #[doc = "0xc0 - PD I/O Mode Control"]
    pub pd_mode: crate::Reg<pd_mode::PD_MODE_SPEC>,
    #[doc = "0xc4 - PD Digital Input Path Disable Control"]
    pub pd_dinoff: crate::Reg<pd_dinoff::PD_DINOFF_SPEC>,
    #[doc = "0xc8 - PD Data Output Value"]
    pub pd_dout: crate::Reg<pd_dout::PD_DOUT_SPEC>,
    #[doc = "0xcc - PD Data Output Write Mask"]
    pub pd_datmsk: crate::Reg<pd_datmsk::PD_DATMSK_SPEC>,
    #[doc = "0xd0 - PD Pin Value"]
    pub pd_pin: crate::Reg<pd_pin::PD_PIN_SPEC>,
    #[doc = "0xd4 - PD De-bounce Enable Control Register"]
    pub pd_dben: crate::Reg<pd_dben::PD_DBEN_SPEC>,
    #[doc = "0xd8 - PD Interrupt Trigger Type Control"]
    pub pd_inttype: crate::Reg<pd_inttype::PD_INTTYPE_SPEC>,
    #[doc = "0xdc - PD Interrupt Enable Control Register"]
    pub pd_inten: crate::Reg<pd_inten::PD_INTEN_SPEC>,
    #[doc = "0xe0 - PD Interrupt Source Flag"]
    pub pd_intsrc: crate::Reg<pd_intsrc::PD_INTSRC_SPEC>,
    #[doc = "0xe4 - PD Input Schmitt Trigger Enable Register"]
    pub pd_smten: crate::Reg<pd_smten::PD_SMTEN_SPEC>,
    #[doc = "0xe8 - PD High Slew Rate Control Register"]
    pub pd_slewctl: crate::Reg<pd_slewctl::PD_SLEWCTL_SPEC>,
    _reserved44: [u8; 20usize],
    #[doc = "0x100 - PE I/O Mode Control"]
    pub pe_mode: crate::Reg<pe_mode::PE_MODE_SPEC>,
    #[doc = "0x104 - PE Digital Input Path Disable Control"]
    pub pe_dinoff: crate::Reg<pe_dinoff::PE_DINOFF_SPEC>,
    #[doc = "0x108 - PE Data Output Value"]
    pub pe_dout: crate::Reg<pe_dout::PE_DOUT_SPEC>,
    #[doc = "0x10c - PE Data Output Write Mask"]
    pub pe_datmsk: crate::Reg<pe_datmsk::PE_DATMSK_SPEC>,
    #[doc = "0x110 - PE Pin Value"]
    pub pe_pin: crate::Reg<pe_pin::PE_PIN_SPEC>,
    #[doc = "0x114 - PE De-bounce Enable Control Register"]
    pub pe_dben: crate::Reg<pe_dben::PE_DBEN_SPEC>,
    #[doc = "0x118 - PE Interrupt Trigger Type Control"]
    pub pe_inttype: crate::Reg<pe_inttype::PE_INTTYPE_SPEC>,
    #[doc = "0x11c - PE Interrupt Enable Control Register"]
    pub pe_inten: crate::Reg<pe_inten::PE_INTEN_SPEC>,
    #[doc = "0x120 - PE Interrupt Source Flag"]
    pub pe_intsrc: crate::Reg<pe_intsrc::PE_INTSRC_SPEC>,
    #[doc = "0x124 - PE Input Schmitt Trigger Enable Register"]
    pub pe_smten: crate::Reg<pe_smten::PE_SMTEN_SPEC>,
    #[doc = "0x128 - PE High Slew Rate Control Register"]
    pub pe_slewctl: crate::Reg<pe_slewctl::PE_SLEWCTL_SPEC>,
    #[doc = "0x12c - PE High Drive Strength Control Register"]
    pub pe_drvctl: crate::Reg<pe_drvctl::PE_DRVCTL_SPEC>,
    _reserved56: [u8; 16usize],
    #[doc = "0x140 - PF I/O Mode Control"]
    pub pf_mode: crate::Reg<pf_mode::PF_MODE_SPEC>,
    #[doc = "0x144 - PF Digital Input Path Disable Control"]
    pub pf_dinoff: crate::Reg<pf_dinoff::PF_DINOFF_SPEC>,
    #[doc = "0x148 - PF Data Output Value"]
    pub pf_dout: crate::Reg<pf_dout::PF_DOUT_SPEC>,
    #[doc = "0x14c - PF Data Output Write Mask"]
    pub pf_datmsk: crate::Reg<pf_datmsk::PF_DATMSK_SPEC>,
    #[doc = "0x150 - PF Pin Value"]
    pub pf_pin: crate::Reg<pf_pin::PF_PIN_SPEC>,
    #[doc = "0x154 - PF De-bounce Enable Control Register"]
    pub pf_dben: crate::Reg<pf_dben::PF_DBEN_SPEC>,
    #[doc = "0x158 - PF Interrupt Trigger Type Control"]
    pub pf_inttype: crate::Reg<pf_inttype::PF_INTTYPE_SPEC>,
    #[doc = "0x15c - PF Interrupt Enable Control Register"]
    pub pf_inten: crate::Reg<pf_inten::PF_INTEN_SPEC>,
    #[doc = "0x160 - PF Interrupt Source Flag"]
    pub pf_intsrc: crate::Reg<pf_intsrc::PF_INTSRC_SPEC>,
    #[doc = "0x164 - PF Input Schmitt Trigger Enable Register"]
    pub pf_smten: crate::Reg<pf_smten::PF_SMTEN_SPEC>,
    #[doc = "0x168 - PF High Slew Rate Control Register"]
    pub pf_slewctl: crate::Reg<pf_slewctl::PF_SLEWCTL_SPEC>,
    _reserved67: [u8; 724usize],
    #[doc = "0x440 - Interrupt De-bounce Control Register"]
    pub gpio_dbctl: crate::Reg<gpio_dbctl::GPIO_DBCTL_SPEC>,
    _reserved68: [u8; 956usize],
    #[doc = "0x800 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa0_pdio: crate::Reg<pa0_pdio::PA0_PDIO_SPEC>,
    #[doc = "0x804 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa1_pdio: crate::Reg<pa1_pdio::PA1_PDIO_SPEC>,
    #[doc = "0x808 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa2_pdio: crate::Reg<pa2_pdio::PA2_PDIO_SPEC>,
    #[doc = "0x80c - GPIO PA.n Pin Data Input/Output Register"]
    pub pa3_pdio: crate::Reg<pa3_pdio::PA3_PDIO_SPEC>,
    #[doc = "0x810 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa4_pdio: crate::Reg<pa4_pdio::PA4_PDIO_SPEC>,
    #[doc = "0x814 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa5_pdio: crate::Reg<pa5_pdio::PA5_PDIO_SPEC>,
    #[doc = "0x818 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa6_pdio: crate::Reg<pa6_pdio::PA6_PDIO_SPEC>,
    #[doc = "0x81c - GPIO PA.n Pin Data Input/Output Register"]
    pub pa7_pdio: crate::Reg<pa7_pdio::PA7_PDIO_SPEC>,
    #[doc = "0x820 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa8_pdio: crate::Reg<pa8_pdio::PA8_PDIO_SPEC>,
    #[doc = "0x824 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa9_pdio: crate::Reg<pa9_pdio::PA9_PDIO_SPEC>,
    #[doc = "0x828 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa10_pdio: crate::Reg<pa10_pdio::PA10_PDIO_SPEC>,
    #[doc = "0x82c - GPIO PA.n Pin Data Input/Output Register"]
    pub pa11_pdio: crate::Reg<pa11_pdio::PA11_PDIO_SPEC>,
    #[doc = "0x830 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa12_pdio: crate::Reg<pa12_pdio::PA12_PDIO_SPEC>,
    #[doc = "0x834 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa13_pdio: crate::Reg<pa13_pdio::PA13_PDIO_SPEC>,
    #[doc = "0x838 - GPIO PA.n Pin Data Input/Output Register"]
    pub pa14_pdio: crate::Reg<pa14_pdio::PA14_PDIO_SPEC>,
    #[doc = "0x83c - GPIO PA.n Pin Data Input/Output Register"]
    pub pa15_pdio: crate::Reg<pa15_pdio::PA15_PDIO_SPEC>,
    #[doc = "0x840 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb0_pdio: crate::Reg<pb0_pdio::PB0_PDIO_SPEC>,
    #[doc = "0x844 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb1_pdio: crate::Reg<pb1_pdio::PB1_PDIO_SPEC>,
    #[doc = "0x848 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb2_pdio: crate::Reg<pb2_pdio::PB2_PDIO_SPEC>,
    #[doc = "0x84c - GPIO PB.n Pin Data Input/Output Register"]
    pub pb3_pdio: crate::Reg<pb3_pdio::PB3_PDIO_SPEC>,
    #[doc = "0x850 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb4_pdio: crate::Reg<pb4_pdio::PB4_PDIO_SPEC>,
    #[doc = "0x854 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb5_pdio: crate::Reg<pb5_pdio::PB5_PDIO_SPEC>,
    #[doc = "0x858 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb6_pdio: crate::Reg<pb6_pdio::PB6_PDIO_SPEC>,
    #[doc = "0x85c - GPIO PB.n Pin Data Input/Output Register"]
    pub pb7_pdio: crate::Reg<pb7_pdio::PB7_PDIO_SPEC>,
    #[doc = "0x860 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb8_pdio: crate::Reg<pb8_pdio::PB8_PDIO_SPEC>,
    #[doc = "0x864 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb9_pdio: crate::Reg<pb9_pdio::PB9_PDIO_SPEC>,
    #[doc = "0x868 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb10_pdio: crate::Reg<pb10_pdio::PB10_PDIO_SPEC>,
    #[doc = "0x86c - GPIO PB.n Pin Data Input/Output Register"]
    pub pb11_pdio: crate::Reg<pb11_pdio::PB11_PDIO_SPEC>,
    #[doc = "0x870 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb12_pdio: crate::Reg<pb12_pdio::PB12_PDIO_SPEC>,
    #[doc = "0x874 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb13_pdio: crate::Reg<pb13_pdio::PB13_PDIO_SPEC>,
    #[doc = "0x878 - GPIO PB.n Pin Data Input/Output Register"]
    pub pb14_pdio: crate::Reg<pb14_pdio::PB14_PDIO_SPEC>,
    #[doc = "0x87c - GPIO PB.n Pin Data Input/Output Register"]
    pub pb15_pdio: crate::Reg<pb15_pdio::PB15_PDIO_SPEC>,
    #[doc = "0x880 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc0_pdio: crate::Reg<pc0_pdio::PC0_PDIO_SPEC>,
    #[doc = "0x884 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc1_pdio: crate::Reg<pc1_pdio::PC1_PDIO_SPEC>,
    #[doc = "0x888 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc2_pdio: crate::Reg<pc2_pdio::PC2_PDIO_SPEC>,
    #[doc = "0x88c - GPIO PC.n Pin Data Input/Output Register"]
    pub pc3_pdio: crate::Reg<pc3_pdio::PC3_PDIO_SPEC>,
    #[doc = "0x890 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc4_pdio: crate::Reg<pc4_pdio::PC4_PDIO_SPEC>,
    #[doc = "0x894 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc5_pdio: crate::Reg<pc5_pdio::PC5_PDIO_SPEC>,
    #[doc = "0x898 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc6_pdio: crate::Reg<pc6_pdio::PC6_PDIO_SPEC>,
    #[doc = "0x89c - GPIO PC.n Pin Data Input/Output Register"]
    pub pc7_pdio: crate::Reg<pc7_pdio::PC7_PDIO_SPEC>,
    #[doc = "0x8a0 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc8_pdio: crate::Reg<pc8_pdio::PC8_PDIO_SPEC>,
    #[doc = "0x8a4 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc9_pdio: crate::Reg<pc9_pdio::PC9_PDIO_SPEC>,
    #[doc = "0x8a8 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc10_pdio: crate::Reg<pc10_pdio::PC10_PDIO_SPEC>,
    #[doc = "0x8ac - GPIO PC.n Pin Data Input/Output Register"]
    pub pc11_pdio: crate::Reg<pc11_pdio::PC11_PDIO_SPEC>,
    #[doc = "0x8b0 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc12_pdio: crate::Reg<pc12_pdio::PC12_PDIO_SPEC>,
    #[doc = "0x8b4 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc13_pdio: crate::Reg<pc13_pdio::PC13_PDIO_SPEC>,
    #[doc = "0x8b8 - GPIO PC.n Pin Data Input/Output Register"]
    pub pc14_pdio: crate::Reg<pc14_pdio::PC14_PDIO_SPEC>,
    #[doc = "0x8bc - GPIO PC.n Pin Data Input/Output Register"]
    pub pc15_pdio: crate::Reg<pc15_pdio::PC15_PDIO_SPEC>,
    #[doc = "0x8c0 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd0_pdio: crate::Reg<pd0_pdio::PD0_PDIO_SPEC>,
    #[doc = "0x8c4 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd1_pdio: crate::Reg<pd1_pdio::PD1_PDIO_SPEC>,
    #[doc = "0x8c8 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd2_pdio: crate::Reg<pd2_pdio::PD2_PDIO_SPEC>,
    #[doc = "0x8cc - GPIO PD.n Pin Data Input/Output Register"]
    pub pd3_pdio: crate::Reg<pd3_pdio::PD3_PDIO_SPEC>,
    #[doc = "0x8d0 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd4_pdio: crate::Reg<pd4_pdio::PD4_PDIO_SPEC>,
    #[doc = "0x8d4 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd5_pdio: crate::Reg<pd5_pdio::PD5_PDIO_SPEC>,
    #[doc = "0x8d8 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd6_pdio: crate::Reg<pd6_pdio::PD6_PDIO_SPEC>,
    #[doc = "0x8dc - GPIO PD.n Pin Data Input/Output Register"]
    pub pd7_pdio: crate::Reg<pd7_pdio::PD7_PDIO_SPEC>,
    #[doc = "0x8e0 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd8_pdio: crate::Reg<pd8_pdio::PD8_PDIO_SPEC>,
    #[doc = "0x8e4 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd9_pdio: crate::Reg<pd9_pdio::PD9_PDIO_SPEC>,
    #[doc = "0x8e8 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd10_pdio: crate::Reg<pd10_pdio::PD10_PDIO_SPEC>,
    #[doc = "0x8ec - GPIO PD.n Pin Data Input/Output Register"]
    pub pd11_pdio: crate::Reg<pd11_pdio::PD11_PDIO_SPEC>,
    #[doc = "0x8f0 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd12_pdio: crate::Reg<pd12_pdio::PD12_PDIO_SPEC>,
    #[doc = "0x8f4 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd13_pdio: crate::Reg<pd13_pdio::PD13_PDIO_SPEC>,
    #[doc = "0x8f8 - GPIO PD.n Pin Data Input/Output Register"]
    pub pd14_pdio: crate::Reg<pd14_pdio::PD14_PDIO_SPEC>,
    #[doc = "0x8fc - GPIO PD.n Pin Data Input/Output Register"]
    pub pd15_pdio: crate::Reg<pd15_pdio::PD15_PDIO_SPEC>,
    #[doc = "0x900 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe0_pdio: crate::Reg<pe0_pdio::PE0_PDIO_SPEC>,
    #[doc = "0x904 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe1_pdio: crate::Reg<pe1_pdio::PE1_PDIO_SPEC>,
    #[doc = "0x908 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe2_pdio: crate::Reg<pe2_pdio::PE2_PDIO_SPEC>,
    #[doc = "0x90c - GPIO PE.n Pin Data Input/Output Register"]
    pub pe3_pdio: crate::Reg<pe3_pdio::PE3_PDIO_SPEC>,
    #[doc = "0x910 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe4_pdio: crate::Reg<pe4_pdio::PE4_PDIO_SPEC>,
    #[doc = "0x914 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe5_pdio: crate::Reg<pe5_pdio::PE5_PDIO_SPEC>,
    #[doc = "0x918 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe6_pdio: crate::Reg<pe6_pdio::PE6_PDIO_SPEC>,
    #[doc = "0x91c - GPIO PE.n Pin Data Input/Output Register"]
    pub pe7_pdio: crate::Reg<pe7_pdio::PE7_PDIO_SPEC>,
    #[doc = "0x920 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe8_pdio: crate::Reg<pe8_pdio::PE8_PDIO_SPEC>,
    #[doc = "0x924 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe9_pdio: crate::Reg<pe9_pdio::PE9_PDIO_SPEC>,
    #[doc = "0x928 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe10_pdio: crate::Reg<pe10_pdio::PE10_PDIO_SPEC>,
    #[doc = "0x92c - GPIO PE.n Pin Data Input/Output Register"]
    pub pe11_pdio: crate::Reg<pe11_pdio::PE11_PDIO_SPEC>,
    #[doc = "0x930 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe12_pdio: crate::Reg<pe12_pdio::PE12_PDIO_SPEC>,
    #[doc = "0x934 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe13_pdio: crate::Reg<pe13_pdio::PE13_PDIO_SPEC>,
    #[doc = "0x938 - GPIO PE.n Pin Data Input/Output Register"]
    pub pe14_pdio: crate::Reg<pe14_pdio::PE14_PDIO_SPEC>,
    _reserved147: [u8; 4usize],
    #[doc = "0x940 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf0_pdio: crate::Reg<pf0_pdio::PF0_PDIO_SPEC>,
    #[doc = "0x944 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf1_pdio: crate::Reg<pf1_pdio::PF1_PDIO_SPEC>,
    #[doc = "0x948 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf2_pdio: crate::Reg<pf2_pdio::PF2_PDIO_SPEC>,
    #[doc = "0x94c - GPIO PF.n Pin Data Input/Output Register"]
    pub pf3_pdio: crate::Reg<pf3_pdio::PF3_PDIO_SPEC>,
    #[doc = "0x950 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf4_pdio: crate::Reg<pf4_pdio::PF4_PDIO_SPEC>,
    #[doc = "0x954 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf5_pdio: crate::Reg<pf5_pdio::PF5_PDIO_SPEC>,
    #[doc = "0x958 - GPIO PF.n Pin Data Input/Output Register"]
    pub pf6_pdio: crate::Reg<pf6_pdio::PF6_PDIO_SPEC>,
    #[doc = "0x95c - GPIO PF.n Pin Data Input/Output Register"]
    pub pf7_pdio: crate::Reg<pf7_pdio::PF7_PDIO_SPEC>,
}
#[doc = "PA_MODE register accessor: an alias for `Reg<PA_MODE_SPEC>`"]
pub type PA_MODE = crate::Reg<pa_mode::PA_MODE_SPEC>;
#[doc = "PA I/O Mode Control"]
pub mod pa_mode;
#[doc = "PA_DINOFF register accessor: an alias for `Reg<PA_DINOFF_SPEC>`"]
pub type PA_DINOFF = crate::Reg<pa_dinoff::PA_DINOFF_SPEC>;
#[doc = "PA Digital Input Path Disable Control"]
pub mod pa_dinoff;
#[doc = "PA_DOUT register accessor: an alias for `Reg<PA_DOUT_SPEC>`"]
pub type PA_DOUT = crate::Reg<pa_dout::PA_DOUT_SPEC>;
#[doc = "PA Data Output Value"]
pub mod pa_dout;
#[doc = "PA_DATMSK register accessor: an alias for `Reg<PA_DATMSK_SPEC>`"]
pub type PA_DATMSK = crate::Reg<pa_datmsk::PA_DATMSK_SPEC>;
#[doc = "PA Data Output Write Mask"]
pub mod pa_datmsk;
#[doc = "PA_PIN register accessor: an alias for `Reg<PA_PIN_SPEC>`"]
pub type PA_PIN = crate::Reg<pa_pin::PA_PIN_SPEC>;
#[doc = "PA Pin Value"]
pub mod pa_pin;
#[doc = "PA_DBEN register accessor: an alias for `Reg<PA_DBEN_SPEC>`"]
pub type PA_DBEN = crate::Reg<pa_dben::PA_DBEN_SPEC>;
#[doc = "PA De-bounce Enable Control Register"]
pub mod pa_dben;
#[doc = "PA_INTTYPE register accessor: an alias for `Reg<PA_INTTYPE_SPEC>`"]
pub type PA_INTTYPE = crate::Reg<pa_inttype::PA_INTTYPE_SPEC>;
#[doc = "PA Interrupt Trigger Type Control"]
pub mod pa_inttype;
#[doc = "PA_INTEN register accessor: an alias for `Reg<PA_INTEN_SPEC>`"]
pub type PA_INTEN = crate::Reg<pa_inten::PA_INTEN_SPEC>;
#[doc = "PA Interrupt Enable Control Register"]
pub mod pa_inten;
#[doc = "PA_INTSRC register accessor: an alias for `Reg<PA_INTSRC_SPEC>`"]
pub type PA_INTSRC = crate::Reg<pa_intsrc::PA_INTSRC_SPEC>;
#[doc = "PA Interrupt Source Flag"]
pub mod pa_intsrc;
#[doc = "PA_SMTEN register accessor: an alias for `Reg<PA_SMTEN_SPEC>`"]
pub type PA_SMTEN = crate::Reg<pa_smten::PA_SMTEN_SPEC>;
#[doc = "PA Input Schmitt Trigger Enable Register"]
pub mod pa_smten;
#[doc = "PA_SLEWCTL register accessor: an alias for `Reg<PA_SLEWCTL_SPEC>`"]
pub type PA_SLEWCTL = crate::Reg<pa_slewctl::PA_SLEWCTL_SPEC>;
#[doc = "PA High Slew Rate Control Register"]
pub mod pa_slewctl;
#[doc = "PB_MODE register accessor: an alias for `Reg<PB_MODE_SPEC>`"]
pub type PB_MODE = crate::Reg<pb_mode::PB_MODE_SPEC>;
#[doc = "PB I/O Mode Control"]
pub mod pb_mode;
#[doc = "PB_DINOFF register accessor: an alias for `Reg<PB_DINOFF_SPEC>`"]
pub type PB_DINOFF = crate::Reg<pb_dinoff::PB_DINOFF_SPEC>;
#[doc = "PB Digital Input Path Disable Control"]
pub mod pb_dinoff;
#[doc = "PB_DOUT register accessor: an alias for `Reg<PB_DOUT_SPEC>`"]
pub type PB_DOUT = crate::Reg<pb_dout::PB_DOUT_SPEC>;
#[doc = "PB Data Output Value"]
pub mod pb_dout;
#[doc = "PB_DATMSK register accessor: an alias for `Reg<PB_DATMSK_SPEC>`"]
pub type PB_DATMSK = crate::Reg<pb_datmsk::PB_DATMSK_SPEC>;
#[doc = "PB Data Output Write Mask"]
pub mod pb_datmsk;
#[doc = "PB_PIN register accessor: an alias for `Reg<PB_PIN_SPEC>`"]
pub type PB_PIN = crate::Reg<pb_pin::PB_PIN_SPEC>;
#[doc = "PB Pin Value"]
pub mod pb_pin;
#[doc = "PB_DBEN register accessor: an alias for `Reg<PB_DBEN_SPEC>`"]
pub type PB_DBEN = crate::Reg<pb_dben::PB_DBEN_SPEC>;
#[doc = "PB De-bounce Enable Control Register"]
pub mod pb_dben;
#[doc = "PB_INTTYPE register accessor: an alias for `Reg<PB_INTTYPE_SPEC>`"]
pub type PB_INTTYPE = crate::Reg<pb_inttype::PB_INTTYPE_SPEC>;
#[doc = "PB Interrupt Trigger Type Control"]
pub mod pb_inttype;
#[doc = "PB_INTEN register accessor: an alias for `Reg<PB_INTEN_SPEC>`"]
pub type PB_INTEN = crate::Reg<pb_inten::PB_INTEN_SPEC>;
#[doc = "PB Interrupt Enable Control Register"]
pub mod pb_inten;
#[doc = "PB_INTSRC register accessor: an alias for `Reg<PB_INTSRC_SPEC>`"]
pub type PB_INTSRC = crate::Reg<pb_intsrc::PB_INTSRC_SPEC>;
#[doc = "PB Interrupt Source Flag"]
pub mod pb_intsrc;
#[doc = "PB_SMTEN register accessor: an alias for `Reg<PB_SMTEN_SPEC>`"]
pub type PB_SMTEN = crate::Reg<pb_smten::PB_SMTEN_SPEC>;
#[doc = "PB Input Schmitt Trigger Enable Register"]
pub mod pb_smten;
#[doc = "PB_SLEWCTL register accessor: an alias for `Reg<PB_SLEWCTL_SPEC>`"]
pub type PB_SLEWCTL = crate::Reg<pb_slewctl::PB_SLEWCTL_SPEC>;
#[doc = "PB High Slew Rate Control Register"]
pub mod pb_slewctl;
#[doc = "PC_MODE register accessor: an alias for `Reg<PC_MODE_SPEC>`"]
pub type PC_MODE = crate::Reg<pc_mode::PC_MODE_SPEC>;
#[doc = "PC I/O Mode Control"]
pub mod pc_mode;
#[doc = "PC_DINOFF register accessor: an alias for `Reg<PC_DINOFF_SPEC>`"]
pub type PC_DINOFF = crate::Reg<pc_dinoff::PC_DINOFF_SPEC>;
#[doc = "PC Digital Input Path Disable Control"]
pub mod pc_dinoff;
#[doc = "PC_DOUT register accessor: an alias for `Reg<PC_DOUT_SPEC>`"]
pub type PC_DOUT = crate::Reg<pc_dout::PC_DOUT_SPEC>;
#[doc = "PC Data Output Value"]
pub mod pc_dout;
#[doc = "PC_DATMSK register accessor: an alias for `Reg<PC_DATMSK_SPEC>`"]
pub type PC_DATMSK = crate::Reg<pc_datmsk::PC_DATMSK_SPEC>;
#[doc = "PC Data Output Write Mask"]
pub mod pc_datmsk;
#[doc = "PC_PIN register accessor: an alias for `Reg<PC_PIN_SPEC>`"]
pub type PC_PIN = crate::Reg<pc_pin::PC_PIN_SPEC>;
#[doc = "PC Pin Value"]
pub mod pc_pin;
#[doc = "PC_DBEN register accessor: an alias for `Reg<PC_DBEN_SPEC>`"]
pub type PC_DBEN = crate::Reg<pc_dben::PC_DBEN_SPEC>;
#[doc = "PC De-bounce Enable Control Register"]
pub mod pc_dben;
#[doc = "PC_INTTYPE register accessor: an alias for `Reg<PC_INTTYPE_SPEC>`"]
pub type PC_INTTYPE = crate::Reg<pc_inttype::PC_INTTYPE_SPEC>;
#[doc = "PC Interrupt Trigger Type Control"]
pub mod pc_inttype;
#[doc = "PC_INTEN register accessor: an alias for `Reg<PC_INTEN_SPEC>`"]
pub type PC_INTEN = crate::Reg<pc_inten::PC_INTEN_SPEC>;
#[doc = "PC Interrupt Enable Control Register"]
pub mod pc_inten;
#[doc = "PC_INTSRC register accessor: an alias for `Reg<PC_INTSRC_SPEC>`"]
pub type PC_INTSRC = crate::Reg<pc_intsrc::PC_INTSRC_SPEC>;
#[doc = "PC Interrupt Source Flag"]
pub mod pc_intsrc;
#[doc = "PC_SMTEN register accessor: an alias for `Reg<PC_SMTEN_SPEC>`"]
pub type PC_SMTEN = crate::Reg<pc_smten::PC_SMTEN_SPEC>;
#[doc = "PC Input Schmitt Trigger Enable Register"]
pub mod pc_smten;
#[doc = "PC_SLEWCTL register accessor: an alias for `Reg<PC_SLEWCTL_SPEC>`"]
pub type PC_SLEWCTL = crate::Reg<pc_slewctl::PC_SLEWCTL_SPEC>;
#[doc = "PC High Slew Rate Control Register"]
pub mod pc_slewctl;
#[doc = "PD_MODE register accessor: an alias for `Reg<PD_MODE_SPEC>`"]
pub type PD_MODE = crate::Reg<pd_mode::PD_MODE_SPEC>;
#[doc = "PD I/O Mode Control"]
pub mod pd_mode;
#[doc = "PD_DINOFF register accessor: an alias for `Reg<PD_DINOFF_SPEC>`"]
pub type PD_DINOFF = crate::Reg<pd_dinoff::PD_DINOFF_SPEC>;
#[doc = "PD Digital Input Path Disable Control"]
pub mod pd_dinoff;
#[doc = "PD_DOUT register accessor: an alias for `Reg<PD_DOUT_SPEC>`"]
pub type PD_DOUT = crate::Reg<pd_dout::PD_DOUT_SPEC>;
#[doc = "PD Data Output Value"]
pub mod pd_dout;
#[doc = "PD_DATMSK register accessor: an alias for `Reg<PD_DATMSK_SPEC>`"]
pub type PD_DATMSK = crate::Reg<pd_datmsk::PD_DATMSK_SPEC>;
#[doc = "PD Data Output Write Mask"]
pub mod pd_datmsk;
#[doc = "PD_PIN register accessor: an alias for `Reg<PD_PIN_SPEC>`"]
pub type PD_PIN = crate::Reg<pd_pin::PD_PIN_SPEC>;
#[doc = "PD Pin Value"]
pub mod pd_pin;
#[doc = "PD_DBEN register accessor: an alias for `Reg<PD_DBEN_SPEC>`"]
pub type PD_DBEN = crate::Reg<pd_dben::PD_DBEN_SPEC>;
#[doc = "PD De-bounce Enable Control Register"]
pub mod pd_dben;
#[doc = "PD_INTTYPE register accessor: an alias for `Reg<PD_INTTYPE_SPEC>`"]
pub type PD_INTTYPE = crate::Reg<pd_inttype::PD_INTTYPE_SPEC>;
#[doc = "PD Interrupt Trigger Type Control"]
pub mod pd_inttype;
#[doc = "PD_INTEN register accessor: an alias for `Reg<PD_INTEN_SPEC>`"]
pub type PD_INTEN = crate::Reg<pd_inten::PD_INTEN_SPEC>;
#[doc = "PD Interrupt Enable Control Register"]
pub mod pd_inten;
#[doc = "PD_INTSRC register accessor: an alias for `Reg<PD_INTSRC_SPEC>`"]
pub type PD_INTSRC = crate::Reg<pd_intsrc::PD_INTSRC_SPEC>;
#[doc = "PD Interrupt Source Flag"]
pub mod pd_intsrc;
#[doc = "PD_SMTEN register accessor: an alias for `Reg<PD_SMTEN_SPEC>`"]
pub type PD_SMTEN = crate::Reg<pd_smten::PD_SMTEN_SPEC>;
#[doc = "PD Input Schmitt Trigger Enable Register"]
pub mod pd_smten;
#[doc = "PD_SLEWCTL register accessor: an alias for `Reg<PD_SLEWCTL_SPEC>`"]
pub type PD_SLEWCTL = crate::Reg<pd_slewctl::PD_SLEWCTL_SPEC>;
#[doc = "PD High Slew Rate Control Register"]
pub mod pd_slewctl;
#[doc = "PE_MODE register accessor: an alias for `Reg<PE_MODE_SPEC>`"]
pub type PE_MODE = crate::Reg<pe_mode::PE_MODE_SPEC>;
#[doc = "PE I/O Mode Control"]
pub mod pe_mode;
#[doc = "PE_DINOFF register accessor: an alias for `Reg<PE_DINOFF_SPEC>`"]
pub type PE_DINOFF = crate::Reg<pe_dinoff::PE_DINOFF_SPEC>;
#[doc = "PE Digital Input Path Disable Control"]
pub mod pe_dinoff;
#[doc = "PE_DOUT register accessor: an alias for `Reg<PE_DOUT_SPEC>`"]
pub type PE_DOUT = crate::Reg<pe_dout::PE_DOUT_SPEC>;
#[doc = "PE Data Output Value"]
pub mod pe_dout;
#[doc = "PE_DATMSK register accessor: an alias for `Reg<PE_DATMSK_SPEC>`"]
pub type PE_DATMSK = crate::Reg<pe_datmsk::PE_DATMSK_SPEC>;
#[doc = "PE Data Output Write Mask"]
pub mod pe_datmsk;
#[doc = "PE_PIN register accessor: an alias for `Reg<PE_PIN_SPEC>`"]
pub type PE_PIN = crate::Reg<pe_pin::PE_PIN_SPEC>;
#[doc = "PE Pin Value"]
pub mod pe_pin;
#[doc = "PE_DBEN register accessor: an alias for `Reg<PE_DBEN_SPEC>`"]
pub type PE_DBEN = crate::Reg<pe_dben::PE_DBEN_SPEC>;
#[doc = "PE De-bounce Enable Control Register"]
pub mod pe_dben;
#[doc = "PE_INTTYPE register accessor: an alias for `Reg<PE_INTTYPE_SPEC>`"]
pub type PE_INTTYPE = crate::Reg<pe_inttype::PE_INTTYPE_SPEC>;
#[doc = "PE Interrupt Trigger Type Control"]
pub mod pe_inttype;
#[doc = "PE_INTEN register accessor: an alias for `Reg<PE_INTEN_SPEC>`"]
pub type PE_INTEN = crate::Reg<pe_inten::PE_INTEN_SPEC>;
#[doc = "PE Interrupt Enable Control Register"]
pub mod pe_inten;
#[doc = "PE_INTSRC register accessor: an alias for `Reg<PE_INTSRC_SPEC>`"]
pub type PE_INTSRC = crate::Reg<pe_intsrc::PE_INTSRC_SPEC>;
#[doc = "PE Interrupt Source Flag"]
pub mod pe_intsrc;
#[doc = "PE_SMTEN register accessor: an alias for `Reg<PE_SMTEN_SPEC>`"]
pub type PE_SMTEN = crate::Reg<pe_smten::PE_SMTEN_SPEC>;
#[doc = "PE Input Schmitt Trigger Enable Register"]
pub mod pe_smten;
#[doc = "PE_SLEWCTL register accessor: an alias for `Reg<PE_SLEWCTL_SPEC>`"]
pub type PE_SLEWCTL = crate::Reg<pe_slewctl::PE_SLEWCTL_SPEC>;
#[doc = "PE High Slew Rate Control Register"]
pub mod pe_slewctl;
#[doc = "PE_DRVCTL register accessor: an alias for `Reg<PE_DRVCTL_SPEC>`"]
pub type PE_DRVCTL = crate::Reg<pe_drvctl::PE_DRVCTL_SPEC>;
#[doc = "PE High Drive Strength Control Register"]
pub mod pe_drvctl;
#[doc = "PF_MODE register accessor: an alias for `Reg<PF_MODE_SPEC>`"]
pub type PF_MODE = crate::Reg<pf_mode::PF_MODE_SPEC>;
#[doc = "PF I/O Mode Control"]
pub mod pf_mode;
#[doc = "PF_DINOFF register accessor: an alias for `Reg<PF_DINOFF_SPEC>`"]
pub type PF_DINOFF = crate::Reg<pf_dinoff::PF_DINOFF_SPEC>;
#[doc = "PF Digital Input Path Disable Control"]
pub mod pf_dinoff;
#[doc = "PF_DOUT register accessor: an alias for `Reg<PF_DOUT_SPEC>`"]
pub type PF_DOUT = crate::Reg<pf_dout::PF_DOUT_SPEC>;
#[doc = "PF Data Output Value"]
pub mod pf_dout;
#[doc = "PF_DATMSK register accessor: an alias for `Reg<PF_DATMSK_SPEC>`"]
pub type PF_DATMSK = crate::Reg<pf_datmsk::PF_DATMSK_SPEC>;
#[doc = "PF Data Output Write Mask"]
pub mod pf_datmsk;
#[doc = "PF_PIN register accessor: an alias for `Reg<PF_PIN_SPEC>`"]
pub type PF_PIN = crate::Reg<pf_pin::PF_PIN_SPEC>;
#[doc = "PF Pin Value"]
pub mod pf_pin;
#[doc = "PF_DBEN register accessor: an alias for `Reg<PF_DBEN_SPEC>`"]
pub type PF_DBEN = crate::Reg<pf_dben::PF_DBEN_SPEC>;
#[doc = "PF De-bounce Enable Control Register"]
pub mod pf_dben;
#[doc = "PF_INTTYPE register accessor: an alias for `Reg<PF_INTTYPE_SPEC>`"]
pub type PF_INTTYPE = crate::Reg<pf_inttype::PF_INTTYPE_SPEC>;
#[doc = "PF Interrupt Trigger Type Control"]
pub mod pf_inttype;
#[doc = "PF_INTEN register accessor: an alias for `Reg<PF_INTEN_SPEC>`"]
pub type PF_INTEN = crate::Reg<pf_inten::PF_INTEN_SPEC>;
#[doc = "PF Interrupt Enable Control Register"]
pub mod pf_inten;
#[doc = "PF_INTSRC register accessor: an alias for `Reg<PF_INTSRC_SPEC>`"]
pub type PF_INTSRC = crate::Reg<pf_intsrc::PF_INTSRC_SPEC>;
#[doc = "PF Interrupt Source Flag"]
pub mod pf_intsrc;
#[doc = "PF_SMTEN register accessor: an alias for `Reg<PF_SMTEN_SPEC>`"]
pub type PF_SMTEN = crate::Reg<pf_smten::PF_SMTEN_SPEC>;
#[doc = "PF Input Schmitt Trigger Enable Register"]
pub mod pf_smten;
#[doc = "PF_SLEWCTL register accessor: an alias for `Reg<PF_SLEWCTL_SPEC>`"]
pub type PF_SLEWCTL = crate::Reg<pf_slewctl::PF_SLEWCTL_SPEC>;
#[doc = "PF High Slew Rate Control Register"]
pub mod pf_slewctl;
#[doc = "GPIO_DBCTL register accessor: an alias for `Reg<GPIO_DBCTL_SPEC>`"]
pub type GPIO_DBCTL = crate::Reg<gpio_dbctl::GPIO_DBCTL_SPEC>;
#[doc = "Interrupt De-bounce Control Register"]
pub mod gpio_dbctl;
#[doc = "PA0_PDIO register accessor: an alias for `Reg<PA0_PDIO_SPEC>`"]
pub type PA0_PDIO = crate::Reg<pa0_pdio::PA0_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa0_pdio;
#[doc = "PA1_PDIO register accessor: an alias for `Reg<PA1_PDIO_SPEC>`"]
pub type PA1_PDIO = crate::Reg<pa1_pdio::PA1_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa1_pdio;
#[doc = "PA2_PDIO register accessor: an alias for `Reg<PA2_PDIO_SPEC>`"]
pub type PA2_PDIO = crate::Reg<pa2_pdio::PA2_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa2_pdio;
#[doc = "PA3_PDIO register accessor: an alias for `Reg<PA3_PDIO_SPEC>`"]
pub type PA3_PDIO = crate::Reg<pa3_pdio::PA3_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa3_pdio;
#[doc = "PA4_PDIO register accessor: an alias for `Reg<PA4_PDIO_SPEC>`"]
pub type PA4_PDIO = crate::Reg<pa4_pdio::PA4_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa4_pdio;
#[doc = "PA5_PDIO register accessor: an alias for `Reg<PA5_PDIO_SPEC>`"]
pub type PA5_PDIO = crate::Reg<pa5_pdio::PA5_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa5_pdio;
#[doc = "PA6_PDIO register accessor: an alias for `Reg<PA6_PDIO_SPEC>`"]
pub type PA6_PDIO = crate::Reg<pa6_pdio::PA6_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa6_pdio;
#[doc = "PA7_PDIO register accessor: an alias for `Reg<PA7_PDIO_SPEC>`"]
pub type PA7_PDIO = crate::Reg<pa7_pdio::PA7_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa7_pdio;
#[doc = "PA8_PDIO register accessor: an alias for `Reg<PA8_PDIO_SPEC>`"]
pub type PA8_PDIO = crate::Reg<pa8_pdio::PA8_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa8_pdio;
#[doc = "PA9_PDIO register accessor: an alias for `Reg<PA9_PDIO_SPEC>`"]
pub type PA9_PDIO = crate::Reg<pa9_pdio::PA9_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa9_pdio;
#[doc = "PA10_PDIO register accessor: an alias for `Reg<PA10_PDIO_SPEC>`"]
pub type PA10_PDIO = crate::Reg<pa10_pdio::PA10_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa10_pdio;
#[doc = "PA11_PDIO register accessor: an alias for `Reg<PA11_PDIO_SPEC>`"]
pub type PA11_PDIO = crate::Reg<pa11_pdio::PA11_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa11_pdio;
#[doc = "PA12_PDIO register accessor: an alias for `Reg<PA12_PDIO_SPEC>`"]
pub type PA12_PDIO = crate::Reg<pa12_pdio::PA12_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa12_pdio;
#[doc = "PA13_PDIO register accessor: an alias for `Reg<PA13_PDIO_SPEC>`"]
pub type PA13_PDIO = crate::Reg<pa13_pdio::PA13_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa13_pdio;
#[doc = "PA14_PDIO register accessor: an alias for `Reg<PA14_PDIO_SPEC>`"]
pub type PA14_PDIO = crate::Reg<pa14_pdio::PA14_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa14_pdio;
#[doc = "PA15_PDIO register accessor: an alias for `Reg<PA15_PDIO_SPEC>`"]
pub type PA15_PDIO = crate::Reg<pa15_pdio::PA15_PDIO_SPEC>;
#[doc = "GPIO PA.n Pin Data Input/Output Register"]
pub mod pa15_pdio;
#[doc = "PB0_PDIO register accessor: an alias for `Reg<PB0_PDIO_SPEC>`"]
pub type PB0_PDIO = crate::Reg<pb0_pdio::PB0_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb0_pdio;
#[doc = "PB1_PDIO register accessor: an alias for `Reg<PB1_PDIO_SPEC>`"]
pub type PB1_PDIO = crate::Reg<pb1_pdio::PB1_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb1_pdio;
#[doc = "PB2_PDIO register accessor: an alias for `Reg<PB2_PDIO_SPEC>`"]
pub type PB2_PDIO = crate::Reg<pb2_pdio::PB2_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb2_pdio;
#[doc = "PB3_PDIO register accessor: an alias for `Reg<PB3_PDIO_SPEC>`"]
pub type PB3_PDIO = crate::Reg<pb3_pdio::PB3_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb3_pdio;
#[doc = "PB4_PDIO register accessor: an alias for `Reg<PB4_PDIO_SPEC>`"]
pub type PB4_PDIO = crate::Reg<pb4_pdio::PB4_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb4_pdio;
#[doc = "PB5_PDIO register accessor: an alias for `Reg<PB5_PDIO_SPEC>`"]
pub type PB5_PDIO = crate::Reg<pb5_pdio::PB5_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb5_pdio;
#[doc = "PB6_PDIO register accessor: an alias for `Reg<PB6_PDIO_SPEC>`"]
pub type PB6_PDIO = crate::Reg<pb6_pdio::PB6_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb6_pdio;
#[doc = "PB7_PDIO register accessor: an alias for `Reg<PB7_PDIO_SPEC>`"]
pub type PB7_PDIO = crate::Reg<pb7_pdio::PB7_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb7_pdio;
#[doc = "PB8_PDIO register accessor: an alias for `Reg<PB8_PDIO_SPEC>`"]
pub type PB8_PDIO = crate::Reg<pb8_pdio::PB8_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb8_pdio;
#[doc = "PB9_PDIO register accessor: an alias for `Reg<PB9_PDIO_SPEC>`"]
pub type PB9_PDIO = crate::Reg<pb9_pdio::PB9_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb9_pdio;
#[doc = "PB10_PDIO register accessor: an alias for `Reg<PB10_PDIO_SPEC>`"]
pub type PB10_PDIO = crate::Reg<pb10_pdio::PB10_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb10_pdio;
#[doc = "PB11_PDIO register accessor: an alias for `Reg<PB11_PDIO_SPEC>`"]
pub type PB11_PDIO = crate::Reg<pb11_pdio::PB11_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb11_pdio;
#[doc = "PB12_PDIO register accessor: an alias for `Reg<PB12_PDIO_SPEC>`"]
pub type PB12_PDIO = crate::Reg<pb12_pdio::PB12_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb12_pdio;
#[doc = "PB13_PDIO register accessor: an alias for `Reg<PB13_PDIO_SPEC>`"]
pub type PB13_PDIO = crate::Reg<pb13_pdio::PB13_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb13_pdio;
#[doc = "PB14_PDIO register accessor: an alias for `Reg<PB14_PDIO_SPEC>`"]
pub type PB14_PDIO = crate::Reg<pb14_pdio::PB14_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb14_pdio;
#[doc = "PB15_PDIO register accessor: an alias for `Reg<PB15_PDIO_SPEC>`"]
pub type PB15_PDIO = crate::Reg<pb15_pdio::PB15_PDIO_SPEC>;
#[doc = "GPIO PB.n Pin Data Input/Output Register"]
pub mod pb15_pdio;
#[doc = "PC0_PDIO register accessor: an alias for `Reg<PC0_PDIO_SPEC>`"]
pub type PC0_PDIO = crate::Reg<pc0_pdio::PC0_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc0_pdio;
#[doc = "PC1_PDIO register accessor: an alias for `Reg<PC1_PDIO_SPEC>`"]
pub type PC1_PDIO = crate::Reg<pc1_pdio::PC1_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc1_pdio;
#[doc = "PC2_PDIO register accessor: an alias for `Reg<PC2_PDIO_SPEC>`"]
pub type PC2_PDIO = crate::Reg<pc2_pdio::PC2_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc2_pdio;
#[doc = "PC3_PDIO register accessor: an alias for `Reg<PC3_PDIO_SPEC>`"]
pub type PC3_PDIO = crate::Reg<pc3_pdio::PC3_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc3_pdio;
#[doc = "PC4_PDIO register accessor: an alias for `Reg<PC4_PDIO_SPEC>`"]
pub type PC4_PDIO = crate::Reg<pc4_pdio::PC4_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc4_pdio;
#[doc = "PC5_PDIO register accessor: an alias for `Reg<PC5_PDIO_SPEC>`"]
pub type PC5_PDIO = crate::Reg<pc5_pdio::PC5_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc5_pdio;
#[doc = "PC6_PDIO register accessor: an alias for `Reg<PC6_PDIO_SPEC>`"]
pub type PC6_PDIO = crate::Reg<pc6_pdio::PC6_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc6_pdio;
#[doc = "PC7_PDIO register accessor: an alias for `Reg<PC7_PDIO_SPEC>`"]
pub type PC7_PDIO = crate::Reg<pc7_pdio::PC7_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc7_pdio;
#[doc = "PC8_PDIO register accessor: an alias for `Reg<PC8_PDIO_SPEC>`"]
pub type PC8_PDIO = crate::Reg<pc8_pdio::PC8_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc8_pdio;
#[doc = "PC9_PDIO register accessor: an alias for `Reg<PC9_PDIO_SPEC>`"]
pub type PC9_PDIO = crate::Reg<pc9_pdio::PC9_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc9_pdio;
#[doc = "PC10_PDIO register accessor: an alias for `Reg<PC10_PDIO_SPEC>`"]
pub type PC10_PDIO = crate::Reg<pc10_pdio::PC10_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc10_pdio;
#[doc = "PC11_PDIO register accessor: an alias for `Reg<PC11_PDIO_SPEC>`"]
pub type PC11_PDIO = crate::Reg<pc11_pdio::PC11_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc11_pdio;
#[doc = "PC12_PDIO register accessor: an alias for `Reg<PC12_PDIO_SPEC>`"]
pub type PC12_PDIO = crate::Reg<pc12_pdio::PC12_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc12_pdio;
#[doc = "PC13_PDIO register accessor: an alias for `Reg<PC13_PDIO_SPEC>`"]
pub type PC13_PDIO = crate::Reg<pc13_pdio::PC13_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc13_pdio;
#[doc = "PC14_PDIO register accessor: an alias for `Reg<PC14_PDIO_SPEC>`"]
pub type PC14_PDIO = crate::Reg<pc14_pdio::PC14_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc14_pdio;
#[doc = "PC15_PDIO register accessor: an alias for `Reg<PC15_PDIO_SPEC>`"]
pub type PC15_PDIO = crate::Reg<pc15_pdio::PC15_PDIO_SPEC>;
#[doc = "GPIO PC.n Pin Data Input/Output Register"]
pub mod pc15_pdio;
#[doc = "PD0_PDIO register accessor: an alias for `Reg<PD0_PDIO_SPEC>`"]
pub type PD0_PDIO = crate::Reg<pd0_pdio::PD0_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd0_pdio;
#[doc = "PD1_PDIO register accessor: an alias for `Reg<PD1_PDIO_SPEC>`"]
pub type PD1_PDIO = crate::Reg<pd1_pdio::PD1_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd1_pdio;
#[doc = "PD2_PDIO register accessor: an alias for `Reg<PD2_PDIO_SPEC>`"]
pub type PD2_PDIO = crate::Reg<pd2_pdio::PD2_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd2_pdio;
#[doc = "PD3_PDIO register accessor: an alias for `Reg<PD3_PDIO_SPEC>`"]
pub type PD3_PDIO = crate::Reg<pd3_pdio::PD3_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd3_pdio;
#[doc = "PD4_PDIO register accessor: an alias for `Reg<PD4_PDIO_SPEC>`"]
pub type PD4_PDIO = crate::Reg<pd4_pdio::PD4_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd4_pdio;
#[doc = "PD5_PDIO register accessor: an alias for `Reg<PD5_PDIO_SPEC>`"]
pub type PD5_PDIO = crate::Reg<pd5_pdio::PD5_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd5_pdio;
#[doc = "PD6_PDIO register accessor: an alias for `Reg<PD6_PDIO_SPEC>`"]
pub type PD6_PDIO = crate::Reg<pd6_pdio::PD6_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd6_pdio;
#[doc = "PD7_PDIO register accessor: an alias for `Reg<PD7_PDIO_SPEC>`"]
pub type PD7_PDIO = crate::Reg<pd7_pdio::PD7_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd7_pdio;
#[doc = "PD8_PDIO register accessor: an alias for `Reg<PD8_PDIO_SPEC>`"]
pub type PD8_PDIO = crate::Reg<pd8_pdio::PD8_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd8_pdio;
#[doc = "PD9_PDIO register accessor: an alias for `Reg<PD9_PDIO_SPEC>`"]
pub type PD9_PDIO = crate::Reg<pd9_pdio::PD9_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd9_pdio;
#[doc = "PD10_PDIO register accessor: an alias for `Reg<PD10_PDIO_SPEC>`"]
pub type PD10_PDIO = crate::Reg<pd10_pdio::PD10_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd10_pdio;
#[doc = "PD11_PDIO register accessor: an alias for `Reg<PD11_PDIO_SPEC>`"]
pub type PD11_PDIO = crate::Reg<pd11_pdio::PD11_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd11_pdio;
#[doc = "PD12_PDIO register accessor: an alias for `Reg<PD12_PDIO_SPEC>`"]
pub type PD12_PDIO = crate::Reg<pd12_pdio::PD12_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd12_pdio;
#[doc = "PD13_PDIO register accessor: an alias for `Reg<PD13_PDIO_SPEC>`"]
pub type PD13_PDIO = crate::Reg<pd13_pdio::PD13_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd13_pdio;
#[doc = "PD14_PDIO register accessor: an alias for `Reg<PD14_PDIO_SPEC>`"]
pub type PD14_PDIO = crate::Reg<pd14_pdio::PD14_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd14_pdio;
#[doc = "PD15_PDIO register accessor: an alias for `Reg<PD15_PDIO_SPEC>`"]
pub type PD15_PDIO = crate::Reg<pd15_pdio::PD15_PDIO_SPEC>;
#[doc = "GPIO PD.n Pin Data Input/Output Register"]
pub mod pd15_pdio;
#[doc = "PE0_PDIO register accessor: an alias for `Reg<PE0_PDIO_SPEC>`"]
pub type PE0_PDIO = crate::Reg<pe0_pdio::PE0_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe0_pdio;
#[doc = "PE1_PDIO register accessor: an alias for `Reg<PE1_PDIO_SPEC>`"]
pub type PE1_PDIO = crate::Reg<pe1_pdio::PE1_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe1_pdio;
#[doc = "PE2_PDIO register accessor: an alias for `Reg<PE2_PDIO_SPEC>`"]
pub type PE2_PDIO = crate::Reg<pe2_pdio::PE2_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe2_pdio;
#[doc = "PE3_PDIO register accessor: an alias for `Reg<PE3_PDIO_SPEC>`"]
pub type PE3_PDIO = crate::Reg<pe3_pdio::PE3_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe3_pdio;
#[doc = "PE4_PDIO register accessor: an alias for `Reg<PE4_PDIO_SPEC>`"]
pub type PE4_PDIO = crate::Reg<pe4_pdio::PE4_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe4_pdio;
#[doc = "PE5_PDIO register accessor: an alias for `Reg<PE5_PDIO_SPEC>`"]
pub type PE5_PDIO = crate::Reg<pe5_pdio::PE5_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe5_pdio;
#[doc = "PE6_PDIO register accessor: an alias for `Reg<PE6_PDIO_SPEC>`"]
pub type PE6_PDIO = crate::Reg<pe6_pdio::PE6_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe6_pdio;
#[doc = "PE7_PDIO register accessor: an alias for `Reg<PE7_PDIO_SPEC>`"]
pub type PE7_PDIO = crate::Reg<pe7_pdio::PE7_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe7_pdio;
#[doc = "PE8_PDIO register accessor: an alias for `Reg<PE8_PDIO_SPEC>`"]
pub type PE8_PDIO = crate::Reg<pe8_pdio::PE8_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe8_pdio;
#[doc = "PE9_PDIO register accessor: an alias for `Reg<PE9_PDIO_SPEC>`"]
pub type PE9_PDIO = crate::Reg<pe9_pdio::PE9_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe9_pdio;
#[doc = "PE10_PDIO register accessor: an alias for `Reg<PE10_PDIO_SPEC>`"]
pub type PE10_PDIO = crate::Reg<pe10_pdio::PE10_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe10_pdio;
#[doc = "PE11_PDIO register accessor: an alias for `Reg<PE11_PDIO_SPEC>`"]
pub type PE11_PDIO = crate::Reg<pe11_pdio::PE11_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe11_pdio;
#[doc = "PE12_PDIO register accessor: an alias for `Reg<PE12_PDIO_SPEC>`"]
pub type PE12_PDIO = crate::Reg<pe12_pdio::PE12_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe12_pdio;
#[doc = "PE13_PDIO register accessor: an alias for `Reg<PE13_PDIO_SPEC>`"]
pub type PE13_PDIO = crate::Reg<pe13_pdio::PE13_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe13_pdio;
#[doc = "PE14_PDIO register accessor: an alias for `Reg<PE14_PDIO_SPEC>`"]
pub type PE14_PDIO = crate::Reg<pe14_pdio::PE14_PDIO_SPEC>;
#[doc = "GPIO PE.n Pin Data Input/Output Register"]
pub mod pe14_pdio;
#[doc = "PF0_PDIO register accessor: an alias for `Reg<PF0_PDIO_SPEC>`"]
pub type PF0_PDIO = crate::Reg<pf0_pdio::PF0_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf0_pdio;
#[doc = "PF1_PDIO register accessor: an alias for `Reg<PF1_PDIO_SPEC>`"]
pub type PF1_PDIO = crate::Reg<pf1_pdio::PF1_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf1_pdio;
#[doc = "PF2_PDIO register accessor: an alias for `Reg<PF2_PDIO_SPEC>`"]
pub type PF2_PDIO = crate::Reg<pf2_pdio::PF2_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf2_pdio;
#[doc = "PF3_PDIO register accessor: an alias for `Reg<PF3_PDIO_SPEC>`"]
pub type PF3_PDIO = crate::Reg<pf3_pdio::PF3_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf3_pdio;
#[doc = "PF4_PDIO register accessor: an alias for `Reg<PF4_PDIO_SPEC>`"]
pub type PF4_PDIO = crate::Reg<pf4_pdio::PF4_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf4_pdio;
#[doc = "PF5_PDIO register accessor: an alias for `Reg<PF5_PDIO_SPEC>`"]
pub type PF5_PDIO = crate::Reg<pf5_pdio::PF5_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf5_pdio;
#[doc = "PF6_PDIO register accessor: an alias for `Reg<PF6_PDIO_SPEC>`"]
pub type PF6_PDIO = crate::Reg<pf6_pdio::PF6_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf6_pdio;
#[doc = "PF7_PDIO register accessor: an alias for `Reg<PF7_PDIO_SPEC>`"]
pub type PF7_PDIO = crate::Reg<pf7_pdio::PF7_PDIO_SPEC>;
#[doc = "GPIO PF.n Pin Data Input/Output Register"]
pub mod pf7_pdio;
