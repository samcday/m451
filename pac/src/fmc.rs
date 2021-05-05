#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISP Control Register"]
    pub fmc_ispctl: crate::Reg<fmc_ispctl::FMC_ISPCTL_SPEC>,
    #[doc = "0x04 - ISP Address Register"]
    pub fmc_ispaddr: crate::Reg<fmc_ispaddr::FMC_ISPADDR_SPEC>,
    #[doc = "0x08 - ISP Data Register"]
    pub fmc_ispdat: crate::Reg<fmc_ispdat::FMC_ISPDAT_SPEC>,
    #[doc = "0x0c - ISP CMD Register"]
    pub fmc_ispcmd: crate::Reg<fmc_ispcmd::FMC_ISPCMD_SPEC>,
    #[doc = "0x10 - ISP Trigger Control Register"]
    pub fmc_isptrg: crate::Reg<fmc_isptrg::FMC_ISPTRG_SPEC>,
    #[doc = "0x14 - Data Flash Base Address"]
    pub fmc_dfba: crate::Reg<fmc_dfba::FMC_DFBA_SPEC>,
    #[doc = "0x18 - Flash Access Time Control Register"]
    pub fmc_ftctl: crate::Reg<fmc_ftctl::FMC_FTCTL_SPEC>,
    _reserved7: [u8; 36usize],
    #[doc = "0x40 - ISP Status Register"]
    pub fmc_ispsts: crate::Reg<fmc_ispsts::FMC_ISPSTS_SPEC>,
    _reserved8: [u8; 60usize],
    #[doc = "0x80 - ISP Data0 Register"]
    pub fmc_mpdat0: crate::Reg<fmc_mpdat0::FMC_MPDAT0_SPEC>,
    #[doc = "0x84 - ISP Data1 Register"]
    pub fmc_mpdat1: crate::Reg<fmc_mpdat1::FMC_MPDAT1_SPEC>,
    #[doc = "0x88 - ISP Data2 Register"]
    pub fmc_mpdat2: crate::Reg<fmc_mpdat2::FMC_MPDAT2_SPEC>,
    #[doc = "0x8c - ISP Data3 Register"]
    pub fmc_mpdat3: crate::Reg<fmc_mpdat3::FMC_MPDAT3_SPEC>,
    _reserved12: [u8; 48usize],
    #[doc = "0xc0 - ISP Multi-program Status Register"]
    pub fmc_mpsts: crate::Reg<fmc_mpsts::FMC_MPSTS_SPEC>,
    #[doc = "0xc4 - ISP Multi-program Address Register"]
    pub fmc_mpaddr: crate::Reg<fmc_mpaddr::FMC_MPADDR_SPEC>,
}
#[doc = "FMC_ISPCTL register accessor: an alias for `Reg<FMC_ISPCTL_SPEC>`"]
pub type FMC_ISPCTL = crate::Reg<fmc_ispctl::FMC_ISPCTL_SPEC>;
#[doc = "ISP Control Register"]
pub mod fmc_ispctl;
#[doc = "FMC_ISPADDR register accessor: an alias for `Reg<FMC_ISPADDR_SPEC>`"]
pub type FMC_ISPADDR = crate::Reg<fmc_ispaddr::FMC_ISPADDR_SPEC>;
#[doc = "ISP Address Register"]
pub mod fmc_ispaddr;
#[doc = "FMC_ISPDAT register accessor: an alias for `Reg<FMC_ISPDAT_SPEC>`"]
pub type FMC_ISPDAT = crate::Reg<fmc_ispdat::FMC_ISPDAT_SPEC>;
#[doc = "ISP Data Register"]
pub mod fmc_ispdat;
#[doc = "FMC_ISPCMD register accessor: an alias for `Reg<FMC_ISPCMD_SPEC>`"]
pub type FMC_ISPCMD = crate::Reg<fmc_ispcmd::FMC_ISPCMD_SPEC>;
#[doc = "ISP CMD Register"]
pub mod fmc_ispcmd;
#[doc = "FMC_ISPTRG register accessor: an alias for `Reg<FMC_ISPTRG_SPEC>`"]
pub type FMC_ISPTRG = crate::Reg<fmc_isptrg::FMC_ISPTRG_SPEC>;
#[doc = "ISP Trigger Control Register"]
pub mod fmc_isptrg;
#[doc = "FMC_DFBA register accessor: an alias for `Reg<FMC_DFBA_SPEC>`"]
pub type FMC_DFBA = crate::Reg<fmc_dfba::FMC_DFBA_SPEC>;
#[doc = "Data Flash Base Address"]
pub mod fmc_dfba;
#[doc = "FMC_FTCTL register accessor: an alias for `Reg<FMC_FTCTL_SPEC>`"]
pub type FMC_FTCTL = crate::Reg<fmc_ftctl::FMC_FTCTL_SPEC>;
#[doc = "Flash Access Time Control Register"]
pub mod fmc_ftctl;
#[doc = "FMC_ISPSTS register accessor: an alias for `Reg<FMC_ISPSTS_SPEC>`"]
pub type FMC_ISPSTS = crate::Reg<fmc_ispsts::FMC_ISPSTS_SPEC>;
#[doc = "ISP Status Register"]
pub mod fmc_ispsts;
#[doc = "FMC_MPDAT0 register accessor: an alias for `Reg<FMC_MPDAT0_SPEC>`"]
pub type FMC_MPDAT0 = crate::Reg<fmc_mpdat0::FMC_MPDAT0_SPEC>;
#[doc = "ISP Data0 Register"]
pub mod fmc_mpdat0;
#[doc = "FMC_MPDAT1 register accessor: an alias for `Reg<FMC_MPDAT1_SPEC>`"]
pub type FMC_MPDAT1 = crate::Reg<fmc_mpdat1::FMC_MPDAT1_SPEC>;
#[doc = "ISP Data1 Register"]
pub mod fmc_mpdat1;
#[doc = "FMC_MPDAT2 register accessor: an alias for `Reg<FMC_MPDAT2_SPEC>`"]
pub type FMC_MPDAT2 = crate::Reg<fmc_mpdat2::FMC_MPDAT2_SPEC>;
#[doc = "ISP Data2 Register"]
pub mod fmc_mpdat2;
#[doc = "FMC_MPDAT3 register accessor: an alias for `Reg<FMC_MPDAT3_SPEC>`"]
pub type FMC_MPDAT3 = crate::Reg<fmc_mpdat3::FMC_MPDAT3_SPEC>;
#[doc = "ISP Data3 Register"]
pub mod fmc_mpdat3;
#[doc = "FMC_MPSTS register accessor: an alias for `Reg<FMC_MPSTS_SPEC>`"]
pub type FMC_MPSTS = crate::Reg<fmc_mpsts::FMC_MPSTS_SPEC>;
#[doc = "ISP Multi-program Status Register"]
pub mod fmc_mpsts;
#[doc = "FMC_MPADDR register accessor: an alias for `Reg<FMC_MPADDR_SPEC>`"]
pub type FMC_MPADDR = crate::Reg<fmc_mpaddr::FMC_MPADDR_SPEC>;
#[doc = "ISP Multi-program Address Register"]
pub mod fmc_mpaddr;
