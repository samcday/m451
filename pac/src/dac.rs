#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Control Register"]
    pub dac_ctl: crate::Reg<dac_ctl::DAC_CTL_SPEC>,
    #[doc = "0x04 - DAC Software Trigger Control Register"]
    pub dac_swtrg: crate::Reg<dac_swtrg::DAC_SWTRG_SPEC>,
    #[doc = "0x08 - DAC Data Holding Register"]
    pub dac_dat: crate::Reg<dac_dat::DAC_DAT_SPEC>,
    #[doc = "0x0c - DAC Data Output Register"]
    pub dac_datout: crate::Reg<dac_datout::DAC_DATOUT_SPEC>,
    #[doc = "0x10 - DAC Status Register"]
    pub dac_status: crate::Reg<dac_status::DAC_STATUS_SPEC>,
    #[doc = "0x14 - DAC Timing Control Register"]
    pub dac_tctl: crate::Reg<dac_tctl::DAC_TCTL_SPEC>,
}
#[doc = "DAC_CTL register accessor: an alias for `Reg<DAC_CTL_SPEC>`"]
pub type DAC_CTL = crate::Reg<dac_ctl::DAC_CTL_SPEC>;
#[doc = "DAC Control Register"]
pub mod dac_ctl;
#[doc = "DAC_SWTRG register accessor: an alias for `Reg<DAC_SWTRG_SPEC>`"]
pub type DAC_SWTRG = crate::Reg<dac_swtrg::DAC_SWTRG_SPEC>;
#[doc = "DAC Software Trigger Control Register"]
pub mod dac_swtrg;
#[doc = "DAC_DAT register accessor: an alias for `Reg<DAC_DAT_SPEC>`"]
pub type DAC_DAT = crate::Reg<dac_dat::DAC_DAT_SPEC>;
#[doc = "DAC Data Holding Register"]
pub mod dac_dat;
#[doc = "DAC_DATOUT register accessor: an alias for `Reg<DAC_DATOUT_SPEC>`"]
pub type DAC_DATOUT = crate::Reg<dac_datout::DAC_DATOUT_SPEC>;
#[doc = "DAC Data Output Register"]
pub mod dac_datout;
#[doc = "DAC_STATUS register accessor: an alias for `Reg<DAC_STATUS_SPEC>`"]
pub type DAC_STATUS = crate::Reg<dac_status::DAC_STATUS_SPEC>;
#[doc = "DAC Status Register"]
pub mod dac_status;
#[doc = "DAC_TCTL register accessor: an alias for `Reg<DAC_TCTL_SPEC>`"]
pub type DAC_TCTL = crate::Reg<dac_tctl::DAC_TCTL_SPEC>;
#[doc = "DAC Timing Control Register"]
pub mod dac_tctl;
