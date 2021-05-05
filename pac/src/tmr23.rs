#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer2 Control and Status Register"]
    pub timer2_ctl: crate::Reg<timer2_ctl::TIMER2_CTL_SPEC>,
    #[doc = "0x04 - Timer2 Compare Register"]
    pub timer2_cmp: crate::Reg<timer2_cmp::TIMER2_CMP_SPEC>,
    #[doc = "0x08 - Timer2 Interrupt Status Register"]
    pub timer2_intsts: crate::Reg<timer2_intsts::TIMER2_INTSTS_SPEC>,
    #[doc = "0x0c - Timer2 Data Register"]
    pub timer2_cnt: crate::Reg<timer2_cnt::TIMER2_CNT_SPEC>,
    #[doc = "0x10 - Timer2 Capture Data Register"]
    pub timer2_cap: crate::Reg<timer2_cap::TIMER2_CAP_SPEC>,
    #[doc = "0x14 - Timer2 External Control Register"]
    pub timer2_extctl: crate::Reg<timer2_extctl::TIMER2_EXTCTL_SPEC>,
    #[doc = "0x18 - Timer2 External Interrupt Status Register"]
    pub timer2_eintsts: crate::Reg<timer2_eintsts::TIMER2_EINTSTS_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Timer3 Control and Status Register"]
    pub timer3_ctl: crate::Reg<timer3_ctl::TIMER3_CTL_SPEC>,
    #[doc = "0x24 - Timer3 Compare Register"]
    pub timer3_cmp: crate::Reg<timer3_cmp::TIMER3_CMP_SPEC>,
    #[doc = "0x28 - Timer3 Interrupt Status Register"]
    pub timer3_intsts: crate::Reg<timer3_intsts::TIMER3_INTSTS_SPEC>,
    #[doc = "0x2c - Timer3 Data Register"]
    pub timer3_cnt: crate::Reg<timer3_cnt::TIMER3_CNT_SPEC>,
    #[doc = "0x30 - Timer3 Capture Data Register"]
    pub timer3_cap: crate::Reg<timer3_cap::TIMER3_CAP_SPEC>,
    #[doc = "0x34 - Timer3 External Control Register"]
    pub timer3_extctl: crate::Reg<timer3_extctl::TIMER3_EXTCTL_SPEC>,
    #[doc = "0x38 - Timer3 External Interrupt Status Register"]
    pub timer3_eintsts: crate::Reg<timer3_eintsts::TIMER3_EINTSTS_SPEC>,
}
#[doc = "TIMER2_CTL register accessor: an alias for `Reg<TIMER2_CTL_SPEC>`"]
pub type TIMER2_CTL = crate::Reg<timer2_ctl::TIMER2_CTL_SPEC>;
#[doc = "Timer2 Control and Status Register"]
pub mod timer2_ctl;
#[doc = "TIMER2_CMP register accessor: an alias for `Reg<TIMER2_CMP_SPEC>`"]
pub type TIMER2_CMP = crate::Reg<timer2_cmp::TIMER2_CMP_SPEC>;
#[doc = "Timer2 Compare Register"]
pub mod timer2_cmp;
#[doc = "TIMER2_INTSTS register accessor: an alias for `Reg<TIMER2_INTSTS_SPEC>`"]
pub type TIMER2_INTSTS = crate::Reg<timer2_intsts::TIMER2_INTSTS_SPEC>;
#[doc = "Timer2 Interrupt Status Register"]
pub mod timer2_intsts;
#[doc = "TIMER2_CNT register accessor: an alias for `Reg<TIMER2_CNT_SPEC>`"]
pub type TIMER2_CNT = crate::Reg<timer2_cnt::TIMER2_CNT_SPEC>;
#[doc = "Timer2 Data Register"]
pub mod timer2_cnt;
#[doc = "TIMER2_CAP register accessor: an alias for `Reg<TIMER2_CAP_SPEC>`"]
pub type TIMER2_CAP = crate::Reg<timer2_cap::TIMER2_CAP_SPEC>;
#[doc = "Timer2 Capture Data Register"]
pub mod timer2_cap;
#[doc = "TIMER2_EXTCTL register accessor: an alias for `Reg<TIMER2_EXTCTL_SPEC>`"]
pub type TIMER2_EXTCTL = crate::Reg<timer2_extctl::TIMER2_EXTCTL_SPEC>;
#[doc = "Timer2 External Control Register"]
pub mod timer2_extctl;
#[doc = "TIMER2_EINTSTS register accessor: an alias for `Reg<TIMER2_EINTSTS_SPEC>`"]
pub type TIMER2_EINTSTS = crate::Reg<timer2_eintsts::TIMER2_EINTSTS_SPEC>;
#[doc = "Timer2 External Interrupt Status Register"]
pub mod timer2_eintsts;
#[doc = "TIMER3_CTL register accessor: an alias for `Reg<TIMER3_CTL_SPEC>`"]
pub type TIMER3_CTL = crate::Reg<timer3_ctl::TIMER3_CTL_SPEC>;
#[doc = "Timer3 Control and Status Register"]
pub mod timer3_ctl;
#[doc = "TIMER3_CMP register accessor: an alias for `Reg<TIMER3_CMP_SPEC>`"]
pub type TIMER3_CMP = crate::Reg<timer3_cmp::TIMER3_CMP_SPEC>;
#[doc = "Timer3 Compare Register"]
pub mod timer3_cmp;
#[doc = "TIMER3_INTSTS register accessor: an alias for `Reg<TIMER3_INTSTS_SPEC>`"]
pub type TIMER3_INTSTS = crate::Reg<timer3_intsts::TIMER3_INTSTS_SPEC>;
#[doc = "Timer3 Interrupt Status Register"]
pub mod timer3_intsts;
#[doc = "TIMER3_CNT register accessor: an alias for `Reg<TIMER3_CNT_SPEC>`"]
pub type TIMER3_CNT = crate::Reg<timer3_cnt::TIMER3_CNT_SPEC>;
#[doc = "Timer3 Data Register"]
pub mod timer3_cnt;
#[doc = "TIMER3_CAP register accessor: an alias for `Reg<TIMER3_CAP_SPEC>`"]
pub type TIMER3_CAP = crate::Reg<timer3_cap::TIMER3_CAP_SPEC>;
#[doc = "Timer3 Capture Data Register"]
pub mod timer3_cap;
#[doc = "TIMER3_EXTCTL register accessor: an alias for `Reg<TIMER3_EXTCTL_SPEC>`"]
pub type TIMER3_EXTCTL = crate::Reg<timer3_extctl::TIMER3_EXTCTL_SPEC>;
#[doc = "Timer3 External Control Register"]
pub mod timer3_extctl;
#[doc = "TIMER3_EINTSTS register accessor: an alias for `Reg<TIMER3_EINTSTS_SPEC>`"]
pub type TIMER3_EINTSTS = crate::Reg<timer3_eintsts::TIMER3_EINTSTS_SPEC>;
#[doc = "Timer3 External Interrupt Status Register"]
pub mod timer3_eintsts;
