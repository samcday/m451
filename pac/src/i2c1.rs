#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Register"]
    pub i2c_ctl: crate::Reg<i2c_ctl::I2C_CTL_SPEC>,
    #[doc = "0x04 - I2C Slave Address Register0"]
    pub i2c_addr0: crate::Reg<i2c_addr0::I2C_ADDR0_SPEC>,
    #[doc = "0x08 - I2C Data Register"]
    pub i2c_dat: crate::Reg<i2c_dat::I2C_DAT_SPEC>,
    #[doc = "0x0c - I2C Status Register"]
    pub i2c_status: crate::Reg<i2c_status::I2C_STATUS_SPEC>,
    #[doc = "0x10 - I2C Clock Divided Register"]
    pub i2c_clkdiv: crate::Reg<i2c_clkdiv::I2C_CLKDIV_SPEC>,
    #[doc = "0x14 - I2C Time-out Control Register"]
    pub i2c_toctl: crate::Reg<i2c_toctl::I2C_TOCTL_SPEC>,
    #[doc = "0x18 - I2C Slave Address Register1"]
    pub i2c_addr1: crate::Reg<i2c_addr1::I2C_ADDR1_SPEC>,
    #[doc = "0x1c - I2C Slave Address Register2"]
    pub i2c_addr2: crate::Reg<i2c_addr2::I2C_ADDR2_SPEC>,
    #[doc = "0x20 - I2C Slave Address Register3"]
    pub i2c_addr3: crate::Reg<i2c_addr3::I2C_ADDR3_SPEC>,
    #[doc = "0x24 - I2C Slave Address Mask Register0"]
    pub i2c_addrmsk0: crate::Reg<i2c_addrmsk0::I2C_ADDRMSK0_SPEC>,
    #[doc = "0x28 - I2C Slave Address Mask Register1"]
    pub i2c_addrmsk1: crate::Reg<i2c_addrmsk1::I2C_ADDRMSK1_SPEC>,
    #[doc = "0x2c - I2C Slave Address Mask Register2"]
    pub i2c_addrmsk2: crate::Reg<i2c_addrmsk2::I2C_ADDRMSK2_SPEC>,
    #[doc = "0x30 - I2C Slave Address Mask Register3"]
    pub i2c_addrmsk3: crate::Reg<i2c_addrmsk3::I2C_ADDRMSK3_SPEC>,
    _reserved13: [u8; 8usize],
    #[doc = "0x3c - I2C Wake-up Control Register"]
    pub i2c_wkctl: crate::Reg<i2c_wkctl::I2C_WKCTL_SPEC>,
    #[doc = "0x40 - I2C Wake-up Status Register"]
    pub i2c_wksts: crate::Reg<i2c_wksts::I2C_WKSTS_SPEC>,
    #[doc = "0x44 - I2C Bus Management Control Register"]
    pub i2c_busctl: crate::Reg<i2c_busctl::I2C_BUSCTL_SPEC>,
    #[doc = "0x48 - I2C Bus Management Timer Control Register"]
    pub i2c_bustctl: crate::Reg<i2c_bustctl::I2C_BUSTCTL_SPEC>,
    #[doc = "0x4c - I2C Bus Management Status Register"]
    pub i2c_bussts: crate::Reg<i2c_bussts::I2C_BUSSTS_SPEC>,
    #[doc = "0x50 - I2C Packet Error Checking Byte Number Register"]
    pub i2c_pktsize: crate::Reg<i2c_pktsize::I2C_PKTSIZE_SPEC>,
    #[doc = "0x54 - I2C Packet Error Checking Byte Value Register"]
    pub i2c_pktcrc: crate::Reg<i2c_pktcrc::I2C_PKTCRC_SPEC>,
    #[doc = "0x58 - I2C Bus Management Timer Register"]
    pub i2c_bustout: crate::Reg<i2c_bustout::I2C_BUSTOUT_SPEC>,
    #[doc = "0x5c - I2C Bus Management Clock Low Timer Register"]
    pub i2c_clktout: crate::Reg<i2c_clktout::I2C_CLKTOUT_SPEC>,
}
#[doc = "I2C_CTL register accessor: an alias for `Reg<I2C_CTL_SPEC>`"]
pub type I2C_CTL = crate::Reg<i2c_ctl::I2C_CTL_SPEC>;
#[doc = "I2C Control Register"]
pub mod i2c_ctl;
#[doc = "I2C_ADDR0 register accessor: an alias for `Reg<I2C_ADDR0_SPEC>`"]
pub type I2C_ADDR0 = crate::Reg<i2c_addr0::I2C_ADDR0_SPEC>;
#[doc = "I2C Slave Address Register0"]
pub mod i2c_addr0;
#[doc = "I2C_DAT register accessor: an alias for `Reg<I2C_DAT_SPEC>`"]
pub type I2C_DAT = crate::Reg<i2c_dat::I2C_DAT_SPEC>;
#[doc = "I2C Data Register"]
pub mod i2c_dat;
#[doc = "I2C_STATUS register accessor: an alias for `Reg<I2C_STATUS_SPEC>`"]
pub type I2C_STATUS = crate::Reg<i2c_status::I2C_STATUS_SPEC>;
#[doc = "I2C Status Register"]
pub mod i2c_status;
#[doc = "I2C_CLKDIV register accessor: an alias for `Reg<I2C_CLKDIV_SPEC>`"]
pub type I2C_CLKDIV = crate::Reg<i2c_clkdiv::I2C_CLKDIV_SPEC>;
#[doc = "I2C Clock Divided Register"]
pub mod i2c_clkdiv;
#[doc = "I2C_TOCTL register accessor: an alias for `Reg<I2C_TOCTL_SPEC>`"]
pub type I2C_TOCTL = crate::Reg<i2c_toctl::I2C_TOCTL_SPEC>;
#[doc = "I2C Time-out Control Register"]
pub mod i2c_toctl;
#[doc = "I2C_ADDR1 register accessor: an alias for `Reg<I2C_ADDR1_SPEC>`"]
pub type I2C_ADDR1 = crate::Reg<i2c_addr1::I2C_ADDR1_SPEC>;
#[doc = "I2C Slave Address Register1"]
pub mod i2c_addr1;
#[doc = "I2C_ADDR2 register accessor: an alias for `Reg<I2C_ADDR2_SPEC>`"]
pub type I2C_ADDR2 = crate::Reg<i2c_addr2::I2C_ADDR2_SPEC>;
#[doc = "I2C Slave Address Register2"]
pub mod i2c_addr2;
#[doc = "I2C_ADDR3 register accessor: an alias for `Reg<I2C_ADDR3_SPEC>`"]
pub type I2C_ADDR3 = crate::Reg<i2c_addr3::I2C_ADDR3_SPEC>;
#[doc = "I2C Slave Address Register3"]
pub mod i2c_addr3;
#[doc = "I2C_ADDRMSK0 register accessor: an alias for `Reg<I2C_ADDRMSK0_SPEC>`"]
pub type I2C_ADDRMSK0 = crate::Reg<i2c_addrmsk0::I2C_ADDRMSK0_SPEC>;
#[doc = "I2C Slave Address Mask Register0"]
pub mod i2c_addrmsk0;
#[doc = "I2C_ADDRMSK1 register accessor: an alias for `Reg<I2C_ADDRMSK1_SPEC>`"]
pub type I2C_ADDRMSK1 = crate::Reg<i2c_addrmsk1::I2C_ADDRMSK1_SPEC>;
#[doc = "I2C Slave Address Mask Register1"]
pub mod i2c_addrmsk1;
#[doc = "I2C_ADDRMSK2 register accessor: an alias for `Reg<I2C_ADDRMSK2_SPEC>`"]
pub type I2C_ADDRMSK2 = crate::Reg<i2c_addrmsk2::I2C_ADDRMSK2_SPEC>;
#[doc = "I2C Slave Address Mask Register2"]
pub mod i2c_addrmsk2;
#[doc = "I2C_ADDRMSK3 register accessor: an alias for `Reg<I2C_ADDRMSK3_SPEC>`"]
pub type I2C_ADDRMSK3 = crate::Reg<i2c_addrmsk3::I2C_ADDRMSK3_SPEC>;
#[doc = "I2C Slave Address Mask Register3"]
pub mod i2c_addrmsk3;
#[doc = "I2C_WKCTL register accessor: an alias for `Reg<I2C_WKCTL_SPEC>`"]
pub type I2C_WKCTL = crate::Reg<i2c_wkctl::I2C_WKCTL_SPEC>;
#[doc = "I2C Wake-up Control Register"]
pub mod i2c_wkctl;
#[doc = "I2C_WKSTS register accessor: an alias for `Reg<I2C_WKSTS_SPEC>`"]
pub type I2C_WKSTS = crate::Reg<i2c_wksts::I2C_WKSTS_SPEC>;
#[doc = "I2C Wake-up Status Register"]
pub mod i2c_wksts;
#[doc = "I2C_BUSCTL register accessor: an alias for `Reg<I2C_BUSCTL_SPEC>`"]
pub type I2C_BUSCTL = crate::Reg<i2c_busctl::I2C_BUSCTL_SPEC>;
#[doc = "I2C Bus Management Control Register"]
pub mod i2c_busctl;
#[doc = "I2C_BUSTCTL register accessor: an alias for `Reg<I2C_BUSTCTL_SPEC>`"]
pub type I2C_BUSTCTL = crate::Reg<i2c_bustctl::I2C_BUSTCTL_SPEC>;
#[doc = "I2C Bus Management Timer Control Register"]
pub mod i2c_bustctl;
#[doc = "I2C_BUSSTS register accessor: an alias for `Reg<I2C_BUSSTS_SPEC>`"]
pub type I2C_BUSSTS = crate::Reg<i2c_bussts::I2C_BUSSTS_SPEC>;
#[doc = "I2C Bus Management Status Register"]
pub mod i2c_bussts;
#[doc = "I2C_PKTSIZE register accessor: an alias for `Reg<I2C_PKTSIZE_SPEC>`"]
pub type I2C_PKTSIZE = crate::Reg<i2c_pktsize::I2C_PKTSIZE_SPEC>;
#[doc = "I2C Packet Error Checking Byte Number Register"]
pub mod i2c_pktsize;
#[doc = "I2C_PKTCRC register accessor: an alias for `Reg<I2C_PKTCRC_SPEC>`"]
pub type I2C_PKTCRC = crate::Reg<i2c_pktcrc::I2C_PKTCRC_SPEC>;
#[doc = "I2C Packet Error Checking Byte Value Register"]
pub mod i2c_pktcrc;
#[doc = "I2C_BUSTOUT register accessor: an alias for `Reg<I2C_BUSTOUT_SPEC>`"]
pub type I2C_BUSTOUT = crate::Reg<i2c_bustout::I2C_BUSTOUT_SPEC>;
#[doc = "I2C Bus Management Timer Register"]
pub mod i2c_bustout;
#[doc = "I2C_CLKTOUT register accessor: an alias for `Reg<I2C_CLKTOUT_SPEC>`"]
pub type I2C_CLKTOUT = crate::Reg<i2c_clktout::I2C_CLKTOUT_SPEC>;
#[doc = "I2C Bus Management Clock Low Timer Register"]
pub mod i2c_clktout;
