#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control Register"]
    pub crc_ctl: crate::Reg<crc_ctl::CRC_CTL_SPEC>,
    #[doc = "0x04 - CRC Write Data Register"]
    pub crc_dat: crate::Reg<crc_dat::CRC_DAT_SPEC>,
    #[doc = "0x08 - CRC Seed Register"]
    pub crc_seed: crate::Reg<crc_seed::CRC_SEED_SPEC>,
    #[doc = "0x0c - CRC Checksum Register"]
    pub crc_checksum: crate::Reg<crc_checksum::CRC_CHECKSUM_SPEC>,
}
#[doc = "CRC_CTL register accessor: an alias for `Reg<CRC_CTL_SPEC>`"]
pub type CRC_CTL = crate::Reg<crc_ctl::CRC_CTL_SPEC>;
#[doc = "CRC Control Register"]
pub mod crc_ctl;
#[doc = "CRC_DAT register accessor: an alias for `Reg<CRC_DAT_SPEC>`"]
pub type CRC_DAT = crate::Reg<crc_dat::CRC_DAT_SPEC>;
#[doc = "CRC Write Data Register"]
pub mod crc_dat;
#[doc = "CRC_SEED register accessor: an alias for `Reg<CRC_SEED_SPEC>`"]
pub type CRC_SEED = crate::Reg<crc_seed::CRC_SEED_SPEC>;
#[doc = "CRC Seed Register"]
pub mod crc_seed;
#[doc = "CRC_CHECKSUM register accessor: an alias for `Reg<CRC_CHECKSUM_SPEC>`"]
pub type CRC_CHECKSUM = crate::Reg<crc_checksum::CRC_CHECKSUM_SPEC>;
#[doc = "CRC Checksum Register"]
pub mod crc_checksum;
