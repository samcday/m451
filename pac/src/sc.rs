#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SC Receiving/Transmit Holding Buffer Register."]
    pub sc_dat: crate::Reg<sc_dat::SC_DAT_SPEC>,
    #[doc = "0x04 - SC Control Register."]
    pub sc_ctl: crate::Reg<sc_ctl::SC_CTL_SPEC>,
    #[doc = "0x08 - SC Alternate Control Register."]
    pub sc_altctl: crate::Reg<sc_altctl::SC_ALTCTL_SPEC>,
    #[doc = "0x0c - SC Extend Guard Time Register."]
    pub sc_egt: crate::Reg<sc_egt::SC_EGT_SPEC>,
    #[doc = "0x10 - SC Receive Buffer Time-out Register."]
    pub sc_rxtout: crate::Reg<sc_rxtout::SC_RXTOUT_SPEC>,
    #[doc = "0x14 - SC ETU Control Register."]
    pub sc_etuctl: crate::Reg<sc_etuctl::SC_ETUCTL_SPEC>,
    #[doc = "0x18 - SC Interrupt Enable Control Register."]
    pub sc_inten: crate::Reg<sc_inten::SC_INTEN_SPEC>,
    #[doc = "0x1c - SC Interrupt Status Register."]
    pub sc_intsts: crate::Reg<sc_intsts::SC_INTSTS_SPEC>,
    #[doc = "0x20 - SC Status Register."]
    pub sc_status: crate::Reg<sc_status::SC_STATUS_SPEC>,
    #[doc = "0x24 - SC Pin Control State Register."]
    pub sc_pinctl: crate::Reg<sc_pinctl::SC_PINCTL_SPEC>,
    #[doc = "0x28 - SC Internal Timer Control Register 0."]
    pub sc_tmrctl0: crate::Reg<sc_tmrctl0::SC_TMRCTL0_SPEC>,
    #[doc = "0x2c - SC Internal Timer Control Register 1."]
    pub sc_tmrctl1: crate::Reg<sc_tmrctl1::SC_TMRCTL1_SPEC>,
    #[doc = "0x30 - SC Internal Timer Control Register 2."]
    pub sc_tmrctl2: crate::Reg<sc_tmrctl2::SC_TMRCTL2_SPEC>,
    #[doc = "0x34 - SC UART Mode Control Register."]
    pub sc_uartctl: crate::Reg<sc_uartctl::SC_UARTCTL_SPEC>,
    #[doc = "0x38 - SC Timer Current Data Register A."]
    pub sc_tmrdat0: crate::Reg<sc_tmrdat0::SC_TMRDAT0_SPEC>,
    #[doc = "0x3c - SC Timer Current Data Register B."]
    pub sc_tmrdat1_2: crate::Reg<sc_tmrdat1_2::SC_TMRDAT1_2_SPEC>,
}
#[doc = "SC_DAT register accessor: an alias for `Reg<SC_DAT_SPEC>`"]
pub type SC_DAT = crate::Reg<sc_dat::SC_DAT_SPEC>;
#[doc = "SC Receiving/Transmit Holding Buffer Register."]
pub mod sc_dat;
#[doc = "SC_CTL register accessor: an alias for `Reg<SC_CTL_SPEC>`"]
pub type SC_CTL = crate::Reg<sc_ctl::SC_CTL_SPEC>;
#[doc = "SC Control Register."]
pub mod sc_ctl;
#[doc = "SC_ALTCTL register accessor: an alias for `Reg<SC_ALTCTL_SPEC>`"]
pub type SC_ALTCTL = crate::Reg<sc_altctl::SC_ALTCTL_SPEC>;
#[doc = "SC Alternate Control Register."]
pub mod sc_altctl;
#[doc = "SC_EGT register accessor: an alias for `Reg<SC_EGT_SPEC>`"]
pub type SC_EGT = crate::Reg<sc_egt::SC_EGT_SPEC>;
#[doc = "SC Extend Guard Time Register."]
pub mod sc_egt;
#[doc = "SC_RXTOUT register accessor: an alias for `Reg<SC_RXTOUT_SPEC>`"]
pub type SC_RXTOUT = crate::Reg<sc_rxtout::SC_RXTOUT_SPEC>;
#[doc = "SC Receive Buffer Time-out Register."]
pub mod sc_rxtout;
#[doc = "SC_ETUCTL register accessor: an alias for `Reg<SC_ETUCTL_SPEC>`"]
pub type SC_ETUCTL = crate::Reg<sc_etuctl::SC_ETUCTL_SPEC>;
#[doc = "SC ETU Control Register."]
pub mod sc_etuctl;
#[doc = "SC_INTEN register accessor: an alias for `Reg<SC_INTEN_SPEC>`"]
pub type SC_INTEN = crate::Reg<sc_inten::SC_INTEN_SPEC>;
#[doc = "SC Interrupt Enable Control Register."]
pub mod sc_inten;
#[doc = "SC_INTSTS register accessor: an alias for `Reg<SC_INTSTS_SPEC>`"]
pub type SC_INTSTS = crate::Reg<sc_intsts::SC_INTSTS_SPEC>;
#[doc = "SC Interrupt Status Register."]
pub mod sc_intsts;
#[doc = "SC_STATUS register accessor: an alias for `Reg<SC_STATUS_SPEC>`"]
pub type SC_STATUS = crate::Reg<sc_status::SC_STATUS_SPEC>;
#[doc = "SC Status Register."]
pub mod sc_status;
#[doc = "SC_PINCTL register accessor: an alias for `Reg<SC_PINCTL_SPEC>`"]
pub type SC_PINCTL = crate::Reg<sc_pinctl::SC_PINCTL_SPEC>;
#[doc = "SC Pin Control State Register."]
pub mod sc_pinctl;
#[doc = "SC_TMRCTL0 register accessor: an alias for `Reg<SC_TMRCTL0_SPEC>`"]
pub type SC_TMRCTL0 = crate::Reg<sc_tmrctl0::SC_TMRCTL0_SPEC>;
#[doc = "SC Internal Timer Control Register 0."]
pub mod sc_tmrctl0;
#[doc = "SC_TMRCTL1 register accessor: an alias for `Reg<SC_TMRCTL1_SPEC>`"]
pub type SC_TMRCTL1 = crate::Reg<sc_tmrctl1::SC_TMRCTL1_SPEC>;
#[doc = "SC Internal Timer Control Register 1."]
pub mod sc_tmrctl1;
#[doc = "SC_TMRCTL2 register accessor: an alias for `Reg<SC_TMRCTL2_SPEC>`"]
pub type SC_TMRCTL2 = crate::Reg<sc_tmrctl2::SC_TMRCTL2_SPEC>;
#[doc = "SC Internal Timer Control Register 2."]
pub mod sc_tmrctl2;
#[doc = "SC_UARTCTL register accessor: an alias for `Reg<SC_UARTCTL_SPEC>`"]
pub type SC_UARTCTL = crate::Reg<sc_uartctl::SC_UARTCTL_SPEC>;
#[doc = "SC UART Mode Control Register."]
pub mod sc_uartctl;
#[doc = "SC_TMRDAT0 register accessor: an alias for `Reg<SC_TMRDAT0_SPEC>`"]
pub type SC_TMRDAT0 = crate::Reg<sc_tmrdat0::SC_TMRDAT0_SPEC>;
#[doc = "SC Timer Current Data Register A."]
pub mod sc_tmrdat0;
#[doc = "SC_TMRDAT1_2 register accessor: an alias for `Reg<SC_TMRDAT1_2_SPEC>`"]
pub type SC_TMRDAT1_2 = crate::Reg<sc_tmrdat1_2::SC_TMRDAT1_2_SPEC>;
#[doc = "SC Timer Current Data Register B."]
pub mod sc_tmrdat1_2;
