#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Receive/Transmit Buffer Register"]
    pub uart_dat: crate::Reg<uart_dat::UART_DAT_SPEC>,
    #[doc = "0x04 - UART Interrupt Enable Register"]
    pub uart_inten: crate::Reg<uart_inten::UART_INTEN_SPEC>,
    #[doc = "0x08 - UART FIFO Control Register"]
    pub uart_fifo: crate::Reg<uart_fifo::UART_FIFO_SPEC>,
    #[doc = "0x0c - UART Line Control Register"]
    pub uart_line: crate::Reg<uart_line::UART_LINE_SPEC>,
    #[doc = "0x10 - UART Modem Control Register"]
    pub uart_modem: crate::Reg<uart_modem::UART_MODEM_SPEC>,
    #[doc = "0x14 - UART Modem Status Register"]
    pub uart_modemsts: crate::Reg<uart_modemsts::UART_MODEMSTS_SPEC>,
    #[doc = "0x18 - UART FIFO Status Register"]
    pub uart_fifosts: crate::Reg<uart_fifosts::UART_FIFOSTS_SPEC>,
    #[doc = "0x1c - UART Interrupt Status Register"]
    pub uart_intsts: crate::Reg<uart_intsts::UART_INTSTS_SPEC>,
    #[doc = "0x20 - UART Time-out Register"]
    pub uart_tout: crate::Reg<uart_tout::UART_TOUT_SPEC>,
    #[doc = "0x24 - UART Baud Rate Divisor Register"]
    pub uart_baud: crate::Reg<uart_baud::UART_BAUD_SPEC>,
    #[doc = "0x28 - UART IrDA Control Register"]
    pub uart_irda: crate::Reg<uart_irda::UART_IRDA_SPEC>,
    #[doc = "0x2c - UART Alternate Control/Status Register"]
    pub uart_altctl: crate::Reg<uart_altctl::UART_ALTCTL_SPEC>,
    #[doc = "0x30 - UART Function Select Register"]
    pub uart_funcsel: crate::Reg<uart_funcsel::UART_FUNCSEL_SPEC>,
}
#[doc = "UART_DAT register accessor: an alias for `Reg<UART_DAT_SPEC>`"]
pub type UART_DAT = crate::Reg<uart_dat::UART_DAT_SPEC>;
#[doc = "UART Receive/Transmit Buffer Register"]
pub mod uart_dat;
#[doc = "UART_INTEN register accessor: an alias for `Reg<UART_INTEN_SPEC>`"]
pub type UART_INTEN = crate::Reg<uart_inten::UART_INTEN_SPEC>;
#[doc = "UART Interrupt Enable Register"]
pub mod uart_inten;
#[doc = "UART_FIFO register accessor: an alias for `Reg<UART_FIFO_SPEC>`"]
pub type UART_FIFO = crate::Reg<uart_fifo::UART_FIFO_SPEC>;
#[doc = "UART FIFO Control Register"]
pub mod uart_fifo;
#[doc = "UART_LINE register accessor: an alias for `Reg<UART_LINE_SPEC>`"]
pub type UART_LINE = crate::Reg<uart_line::UART_LINE_SPEC>;
#[doc = "UART Line Control Register"]
pub mod uart_line;
#[doc = "UART_MODEM register accessor: an alias for `Reg<UART_MODEM_SPEC>`"]
pub type UART_MODEM = crate::Reg<uart_modem::UART_MODEM_SPEC>;
#[doc = "UART Modem Control Register"]
pub mod uart_modem;
#[doc = "UART_MODEMSTS register accessor: an alias for `Reg<UART_MODEMSTS_SPEC>`"]
pub type UART_MODEMSTS = crate::Reg<uart_modemsts::UART_MODEMSTS_SPEC>;
#[doc = "UART Modem Status Register"]
pub mod uart_modemsts;
#[doc = "UART_FIFOSTS register accessor: an alias for `Reg<UART_FIFOSTS_SPEC>`"]
pub type UART_FIFOSTS = crate::Reg<uart_fifosts::UART_FIFOSTS_SPEC>;
#[doc = "UART FIFO Status Register"]
pub mod uart_fifosts;
#[doc = "UART_INTSTS register accessor: an alias for `Reg<UART_INTSTS_SPEC>`"]
pub type UART_INTSTS = crate::Reg<uart_intsts::UART_INTSTS_SPEC>;
#[doc = "UART Interrupt Status Register"]
pub mod uart_intsts;
#[doc = "UART_TOUT register accessor: an alias for `Reg<UART_TOUT_SPEC>`"]
pub type UART_TOUT = crate::Reg<uart_tout::UART_TOUT_SPEC>;
#[doc = "UART Time-out Register"]
pub mod uart_tout;
#[doc = "UART_BAUD register accessor: an alias for `Reg<UART_BAUD_SPEC>`"]
pub type UART_BAUD = crate::Reg<uart_baud::UART_BAUD_SPEC>;
#[doc = "UART Baud Rate Divisor Register"]
pub mod uart_baud;
#[doc = "UART_IRDA register accessor: an alias for `Reg<UART_IRDA_SPEC>`"]
pub type UART_IRDA = crate::Reg<uart_irda::UART_IRDA_SPEC>;
#[doc = "UART IrDA Control Register"]
pub mod uart_irda;
#[doc = "UART_ALTCTL register accessor: an alias for `Reg<UART_ALTCTL_SPEC>`"]
pub type UART_ALTCTL = crate::Reg<uart_altctl::UART_ALTCTL_SPEC>;
#[doc = "UART Alternate Control/Status Register"]
pub mod uart_altctl;
#[doc = "UART_FUNCSEL register accessor: an alias for `Reg<UART_FUNCSEL_SPEC>`"]
pub type UART_FUNCSEL = crate::Reg<uart_funcsel::UART_FUNCSEL_SPEC>;
#[doc = "UART Function Select Register"]
pub mod uart_funcsel;
