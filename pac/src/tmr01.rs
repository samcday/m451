#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0 Control and Status Register"]
    pub timer0_ctl: crate::Reg<timer0_ctl::TIMER0_CTL_SPEC>,
    #[doc = "0x04 - Timer0 Compare Register"]
    pub timer0_cmp: crate::Reg<timer0_cmp::TIMER0_CMP_SPEC>,
    #[doc = "0x08 - Timer0 Interrupt Status Register"]
    pub timer0_intsts: crate::Reg<timer0_intsts::TIMER0_INTSTS_SPEC>,
    #[doc = "0x0c - Timer0 Data Register"]
    pub timer0_cnt: crate::Reg<timer0_cnt::TIMER0_CNT_SPEC>,
    #[doc = "0x10 - Timer0 Capture Data Register"]
    pub timer0_cap: crate::Reg<timer0_cap::TIMER0_CAP_SPEC>,
    #[doc = "0x14 - Timer0 External Control Register"]
    pub timer0_extctl: crate::Reg<timer0_extctl::TIMER0_EXTCTL_SPEC>,
    #[doc = "0x18 - Timer0 External Interrupt Status Register"]
    pub timer0_eintsts: crate::Reg<timer0_eintsts::TIMER0_EINTSTS_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Timer1 Control and Status Register"]
    pub timer1_ctl: crate::Reg<timer1_ctl::TIMER1_CTL_SPEC>,
    #[doc = "0x24 - Timer1 Compare Register"]
    pub timer1_cmp: crate::Reg<timer1_cmp::TIMER1_CMP_SPEC>,
    #[doc = "0x28 - Timer1 Interrupt Status Register"]
    pub timer1_intsts: crate::Reg<timer1_intsts::TIMER1_INTSTS_SPEC>,
    #[doc = "0x2c - Timer1 Data Register"]
    pub timer1_cnt: crate::Reg<timer1_cnt::TIMER1_CNT_SPEC>,
    #[doc = "0x30 - Timer1 Capture Data Register"]
    pub timer1_cap: crate::Reg<timer1_cap::TIMER1_CAP_SPEC>,
    #[doc = "0x34 - Timer1 External Control Register"]
    pub timer1_extctl: crate::Reg<timer1_extctl::TIMER1_EXTCTL_SPEC>,
    #[doc = "0x38 - Timer1 External Interrupt Status Register"]
    pub timer1_eintsts: crate::Reg<timer1_eintsts::TIMER1_EINTSTS_SPEC>,
}
#[doc = "TIMER0_CTL register accessor: an alias for `Reg<TIMER0_CTL_SPEC>`"]
pub type TIMER0_CTL = crate::Reg<timer0_ctl::TIMER0_CTL_SPEC>;
#[doc = "Timer0 Control and Status Register"]
pub mod timer0_ctl;
#[doc = "TIMER0_CMP register accessor: an alias for `Reg<TIMER0_CMP_SPEC>`"]
pub type TIMER0_CMP = crate::Reg<timer0_cmp::TIMER0_CMP_SPEC>;
#[doc = "Timer0 Compare Register"]
pub mod timer0_cmp;
#[doc = "TIMER0_INTSTS register accessor: an alias for `Reg<TIMER0_INTSTS_SPEC>`"]
pub type TIMER0_INTSTS = crate::Reg<timer0_intsts::TIMER0_INTSTS_SPEC>;
#[doc = "Timer0 Interrupt Status Register"]
pub mod timer0_intsts;
#[doc = "TIMER0_CNT register accessor: an alias for `Reg<TIMER0_CNT_SPEC>`"]
pub type TIMER0_CNT = crate::Reg<timer0_cnt::TIMER0_CNT_SPEC>;
#[doc = "Timer0 Data Register"]
pub mod timer0_cnt;
#[doc = "TIMER0_CAP register accessor: an alias for `Reg<TIMER0_CAP_SPEC>`"]
pub type TIMER0_CAP = crate::Reg<timer0_cap::TIMER0_CAP_SPEC>;
#[doc = "Timer0 Capture Data Register"]
pub mod timer0_cap;
#[doc = "TIMER0_EXTCTL register accessor: an alias for `Reg<TIMER0_EXTCTL_SPEC>`"]
pub type TIMER0_EXTCTL = crate::Reg<timer0_extctl::TIMER0_EXTCTL_SPEC>;
#[doc = "Timer0 External Control Register"]
pub mod timer0_extctl;
#[doc = "TIMER0_EINTSTS register accessor: an alias for `Reg<TIMER0_EINTSTS_SPEC>`"]
pub type TIMER0_EINTSTS = crate::Reg<timer0_eintsts::TIMER0_EINTSTS_SPEC>;
#[doc = "Timer0 External Interrupt Status Register"]
pub mod timer0_eintsts;
#[doc = "TIMER1_CTL register accessor: an alias for `Reg<TIMER1_CTL_SPEC>`"]
pub type TIMER1_CTL = crate::Reg<timer1_ctl::TIMER1_CTL_SPEC>;
#[doc = "Timer1 Control and Status Register"]
pub mod timer1_ctl;
#[doc = "TIMER1_CMP register accessor: an alias for `Reg<TIMER1_CMP_SPEC>`"]
pub type TIMER1_CMP = crate::Reg<timer1_cmp::TIMER1_CMP_SPEC>;
#[doc = "Timer1 Compare Register"]
pub mod timer1_cmp;
#[doc = "TIMER1_INTSTS register accessor: an alias for `Reg<TIMER1_INTSTS_SPEC>`"]
pub type TIMER1_INTSTS = crate::Reg<timer1_intsts::TIMER1_INTSTS_SPEC>;
#[doc = "Timer1 Interrupt Status Register"]
pub mod timer1_intsts;
#[doc = "TIMER1_CNT register accessor: an alias for `Reg<TIMER1_CNT_SPEC>`"]
pub type TIMER1_CNT = crate::Reg<timer1_cnt::TIMER1_CNT_SPEC>;
#[doc = "Timer1 Data Register"]
pub mod timer1_cnt;
#[doc = "TIMER1_CAP register accessor: an alias for `Reg<TIMER1_CAP_SPEC>`"]
pub type TIMER1_CAP = crate::Reg<timer1_cap::TIMER1_CAP_SPEC>;
#[doc = "Timer1 Capture Data Register"]
pub mod timer1_cap;
#[doc = "TIMER1_EXTCTL register accessor: an alias for `Reg<TIMER1_EXTCTL_SPEC>`"]
pub type TIMER1_EXTCTL = crate::Reg<timer1_extctl::TIMER1_EXTCTL_SPEC>;
#[doc = "Timer1 External Control Register"]
pub mod timer1_extctl;
#[doc = "TIMER1_EINTSTS register accessor: an alias for `Reg<TIMER1_EINTSTS_SPEC>`"]
pub type TIMER1_EINTSTS = crate::Reg<timer1_eintsts::TIMER1_EINTSTS_SPEC>;
#[doc = "Timer1 External Interrupt Status Register"]
pub mod timer1_eintsts;
