#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Power-down Control Register"]
    pub clk_pwrctl: crate::Reg<clk_pwrctl::CLK_PWRCTL_SPEC>,
    #[doc = "0x04 - AHB Devices Clock Enable Control Register"]
    pub clk_ahbclk: crate::Reg<clk_ahbclk::CLK_AHBCLK_SPEC>,
    #[doc = "0x08 - APB Devices Clock Enable Control Register 0"]
    pub clk_apbclk0: crate::Reg<clk_apbclk0::CLK_APBCLK0_SPEC>,
    #[doc = "0x0c - APB Devices Clock Enable Control Register 1"]
    pub clk_apbclk1: crate::Reg<clk_apbclk1::CLK_APBCLK1_SPEC>,
    #[doc = "0x10 - Clock Source Select Control Register 0"]
    pub clk_clksel0: crate::Reg<clk_clksel0::CLK_CLKSEL0_SPEC>,
    #[doc = "0x14 - Clock Source Select Control Register 1"]
    pub clk_clksel1: crate::Reg<clk_clksel1::CLK_CLKSEL1_SPEC>,
    #[doc = "0x18 - Clock Source Select Control Register 2"]
    pub clk_clksel2: crate::Reg<clk_clksel2::CLK_CLKSEL2_SPEC>,
    #[doc = "0x1c - Clock Source Select Control Register 3"]
    pub clk_clksel3: crate::Reg<clk_clksel3::CLK_CLKSEL3_SPEC>,
    #[doc = "0x20 - Clock Divider Number Register 0"]
    pub clk_clkdiv0: crate::Reg<clk_clkdiv0::CLK_CLKDIV0_SPEC>,
    #[doc = "0x24 - Clock Divider Number Register 1"]
    pub clk_clkdiv1: crate::Reg<clk_clkdiv1::CLK_CLKDIV1_SPEC>,
    _reserved10: [u8; 24usize],
    #[doc = "0x40 - PLL Control Register"]
    pub clk_pllctl: crate::Reg<clk_pllctl::CLK_PLLCTL_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0x50 - Clock Status Monitor Register"]
    pub clk_status: crate::Reg<clk_status::CLK_STATUS_SPEC>,
    _reserved12: [u8; 12usize],
    #[doc = "0x60 - Clock Output Control Register"]
    pub clk_clkoctl: crate::Reg<clk_clkoctl::CLK_CLKOCTL_SPEC>,
    _reserved13: [u8; 12usize],
    #[doc = "0x70 - Clock Fail Detector Control Register"]
    pub clk_clkdctl: crate::Reg<clk_clkdctl::CLK_CLKDCTL_SPEC>,
    #[doc = "0x74 - Clock Fail Detector Status Register"]
    pub clk_clkdsts: crate::Reg<clk_clkdsts::CLK_CLKDSTS_SPEC>,
    #[doc = "0x78 - Clock Frequency Detector Upper Boundary Register"]
    pub clk_cdupb: crate::Reg<clk_cdupb::CLK_CDUPB_SPEC>,
    #[doc = "0x7c - Clock Frequency Detector Lower Boundary Register"]
    pub clk_cdlowb: crate::Reg<clk_cdlowb::CLK_CDLOWB_SPEC>,
}
#[doc = "CLK_PWRCTL register accessor: an alias for `Reg<CLK_PWRCTL_SPEC>`"]
pub type CLK_PWRCTL = crate::Reg<clk_pwrctl::CLK_PWRCTL_SPEC>;
#[doc = "System Power-down Control Register"]
pub mod clk_pwrctl;
#[doc = "CLK_AHBCLK register accessor: an alias for `Reg<CLK_AHBCLK_SPEC>`"]
pub type CLK_AHBCLK = crate::Reg<clk_ahbclk::CLK_AHBCLK_SPEC>;
#[doc = "AHB Devices Clock Enable Control Register"]
pub mod clk_ahbclk;
#[doc = "CLK_APBCLK0 register accessor: an alias for `Reg<CLK_APBCLK0_SPEC>`"]
pub type CLK_APBCLK0 = crate::Reg<clk_apbclk0::CLK_APBCLK0_SPEC>;
#[doc = "APB Devices Clock Enable Control Register 0"]
pub mod clk_apbclk0;
#[doc = "CLK_APBCLK1 register accessor: an alias for `Reg<CLK_APBCLK1_SPEC>`"]
pub type CLK_APBCLK1 = crate::Reg<clk_apbclk1::CLK_APBCLK1_SPEC>;
#[doc = "APB Devices Clock Enable Control Register 1"]
pub mod clk_apbclk1;
#[doc = "CLK_CLKSEL0 register accessor: an alias for `Reg<CLK_CLKSEL0_SPEC>`"]
pub type CLK_CLKSEL0 = crate::Reg<clk_clksel0::CLK_CLKSEL0_SPEC>;
#[doc = "Clock Source Select Control Register 0"]
pub mod clk_clksel0;
#[doc = "CLK_CLKSEL1 register accessor: an alias for `Reg<CLK_CLKSEL1_SPEC>`"]
pub type CLK_CLKSEL1 = crate::Reg<clk_clksel1::CLK_CLKSEL1_SPEC>;
#[doc = "Clock Source Select Control Register 1"]
pub mod clk_clksel1;
#[doc = "CLK_CLKSEL2 register accessor: an alias for `Reg<CLK_CLKSEL2_SPEC>`"]
pub type CLK_CLKSEL2 = crate::Reg<clk_clksel2::CLK_CLKSEL2_SPEC>;
#[doc = "Clock Source Select Control Register 2"]
pub mod clk_clksel2;
#[doc = "CLK_CLKSEL3 register accessor: an alias for `Reg<CLK_CLKSEL3_SPEC>`"]
pub type CLK_CLKSEL3 = crate::Reg<clk_clksel3::CLK_CLKSEL3_SPEC>;
#[doc = "Clock Source Select Control Register 3"]
pub mod clk_clksel3;
#[doc = "CLK_CLKDIV0 register accessor: an alias for `Reg<CLK_CLKDIV0_SPEC>`"]
pub type CLK_CLKDIV0 = crate::Reg<clk_clkdiv0::CLK_CLKDIV0_SPEC>;
#[doc = "Clock Divider Number Register 0"]
pub mod clk_clkdiv0;
#[doc = "CLK_CLKDIV1 register accessor: an alias for `Reg<CLK_CLKDIV1_SPEC>`"]
pub type CLK_CLKDIV1 = crate::Reg<clk_clkdiv1::CLK_CLKDIV1_SPEC>;
#[doc = "Clock Divider Number Register 1"]
pub mod clk_clkdiv1;
#[doc = "CLK_PLLCTL register accessor: an alias for `Reg<CLK_PLLCTL_SPEC>`"]
pub type CLK_PLLCTL = crate::Reg<clk_pllctl::CLK_PLLCTL_SPEC>;
#[doc = "PLL Control Register"]
pub mod clk_pllctl;
#[doc = "CLK_STATUS register accessor: an alias for `Reg<CLK_STATUS_SPEC>`"]
pub type CLK_STATUS = crate::Reg<clk_status::CLK_STATUS_SPEC>;
#[doc = "Clock Status Monitor Register"]
pub mod clk_status;
#[doc = "CLK_CLKOCTL register accessor: an alias for `Reg<CLK_CLKOCTL_SPEC>`"]
pub type CLK_CLKOCTL = crate::Reg<clk_clkoctl::CLK_CLKOCTL_SPEC>;
#[doc = "Clock Output Control Register"]
pub mod clk_clkoctl;
#[doc = "CLK_CLKDCTL register accessor: an alias for `Reg<CLK_CLKDCTL_SPEC>`"]
pub type CLK_CLKDCTL = crate::Reg<clk_clkdctl::CLK_CLKDCTL_SPEC>;
#[doc = "Clock Fail Detector Control Register"]
pub mod clk_clkdctl;
#[doc = "CLK_CLKDSTS register accessor: an alias for `Reg<CLK_CLKDSTS_SPEC>`"]
pub type CLK_CLKDSTS = crate::Reg<clk_clkdsts::CLK_CLKDSTS_SPEC>;
#[doc = "Clock Fail Detector Status Register"]
pub mod clk_clkdsts;
#[doc = "CLK_CDUPB register accessor: an alias for `Reg<CLK_CDUPB_SPEC>`"]
pub type CLK_CDUPB = crate::Reg<clk_cdupb::CLK_CDUPB_SPEC>;
#[doc = "Clock Frequency Detector Upper Boundary Register"]
pub mod clk_cdupb;
#[doc = "CLK_CDLOWB register accessor: an alias for `Reg<CLK_CDLOWB_SPEC>`"]
pub type CLK_CDLOWB = crate::Reg<clk_cdlowb::CLK_CDLOWB_SPEC>;
#[doc = "Clock Frequency Detector Lower Boundary Register"]
pub mod clk_cdlowb;
