#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spi_ctl: crate::Reg<spi_ctl::SPI_CTL_SPEC>,
    #[doc = "0x04 - SPI Clock Divider Register"]
    pub spi_clkdiv: crate::Reg<spi_clkdiv::SPI_CLKDIV_SPEC>,
    #[doc = "0x08 - SPI Slave Select Control Register"]
    pub spi_ssctl: crate::Reg<spi_ssctl::SPI_SSCTL_SPEC>,
    #[doc = "0x0c - SPI PDMA Control Register"]
    pub spi_pdmactl: crate::Reg<spi_pdmactl::SPI_PDMACTL_SPEC>,
    #[doc = "0x10 - SPI FIFO Control Register"]
    pub spi_fifoctl: crate::Reg<spi_fifoctl::SPI_FIFOCTL_SPEC>,
    #[doc = "0x14 - SPI Status Register"]
    pub spi_status: crate::Reg<spi_status::SPI_STATUS_SPEC>,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Data Transmit Register"]
    pub spi_tx: crate::Reg<spi_tx::SPI_TX_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x30 - Data Receive Register"]
    pub spi_rx: crate::Reg<spi_rx::SPI_RX_SPEC>,
    _reserved8: [u8; 44usize],
    #[doc = "0x60 - I2S Control Register"]
    pub spi_i2sctl: crate::Reg<spi_i2sctl::SPI_I2SCTL_SPEC>,
    #[doc = "0x64 - I2S Clock Divider Control Register"]
    pub spi_i2sclk: crate::Reg<spi_i2sclk::SPI_I2SCLK_SPEC>,
    #[doc = "0x68 - I2S Status Register"]
    pub spi_i2ssts: crate::Reg<spi_i2ssts::SPI_I2SSTS_SPEC>,
}
#[doc = "SPI_CTL register accessor: an alias for `Reg<SPI_CTL_SPEC>`"]
pub type SPI_CTL = crate::Reg<spi_ctl::SPI_CTL_SPEC>;
#[doc = "SPI Control Register"]
pub mod spi_ctl;
#[doc = "SPI_CLKDIV register accessor: an alias for `Reg<SPI_CLKDIV_SPEC>`"]
pub type SPI_CLKDIV = crate::Reg<spi_clkdiv::SPI_CLKDIV_SPEC>;
#[doc = "SPI Clock Divider Register"]
pub mod spi_clkdiv;
#[doc = "SPI_SSCTL register accessor: an alias for `Reg<SPI_SSCTL_SPEC>`"]
pub type SPI_SSCTL = crate::Reg<spi_ssctl::SPI_SSCTL_SPEC>;
#[doc = "SPI Slave Select Control Register"]
pub mod spi_ssctl;
#[doc = "SPI_PDMACTL register accessor: an alias for `Reg<SPI_PDMACTL_SPEC>`"]
pub type SPI_PDMACTL = crate::Reg<spi_pdmactl::SPI_PDMACTL_SPEC>;
#[doc = "SPI PDMA Control Register"]
pub mod spi_pdmactl;
#[doc = "SPI_FIFOCTL register accessor: an alias for `Reg<SPI_FIFOCTL_SPEC>`"]
pub type SPI_FIFOCTL = crate::Reg<spi_fifoctl::SPI_FIFOCTL_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spi_fifoctl;
#[doc = "SPI_STATUS register accessor: an alias for `Reg<SPI_STATUS_SPEC>`"]
pub type SPI_STATUS = crate::Reg<spi_status::SPI_STATUS_SPEC>;
#[doc = "SPI Status Register"]
pub mod spi_status;
#[doc = "SPI_TX register accessor: an alias for `Reg<SPI_TX_SPEC>`"]
pub type SPI_TX = crate::Reg<spi_tx::SPI_TX_SPEC>;
#[doc = "Data Transmit Register"]
pub mod spi_tx;
#[doc = "SPI_RX register accessor: an alias for `Reg<SPI_RX_SPEC>`"]
pub type SPI_RX = crate::Reg<spi_rx::SPI_RX_SPEC>;
#[doc = "Data Receive Register"]
pub mod spi_rx;
#[doc = "SPI_I2SCTL register accessor: an alias for `Reg<SPI_I2SCTL_SPEC>`"]
pub type SPI_I2SCTL = crate::Reg<spi_i2sctl::SPI_I2SCTL_SPEC>;
#[doc = "I2S Control Register"]
pub mod spi_i2sctl;
#[doc = "SPI_I2SCLK register accessor: an alias for `Reg<SPI_I2SCLK_SPEC>`"]
pub type SPI_I2SCLK = crate::Reg<spi_i2sclk::SPI_I2SCLK_SPEC>;
#[doc = "I2S Clock Divider Control Register"]
pub mod spi_i2sclk;
#[doc = "SPI_I2SSTS register accessor: an alias for `Reg<SPI_I2SSTS_SPEC>`"]
pub type SPI_I2SSTS = crate::Reg<spi_i2ssts::SPI_I2SSTS_SPEC>;
#[doc = "I2S Status Register"]
pub mod spi_i2ssts;
