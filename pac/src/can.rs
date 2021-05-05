#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub can_con: crate::Reg<can_con::CAN_CON_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub can_status: crate::Reg<can_status::CAN_STATUS_SPEC>,
    #[doc = "0x08 - Error Counter Register"]
    pub can_err: crate::Reg<can_err::CAN_ERR_SPEC>,
    #[doc = "0x0c - Bit Timing Register"]
    pub can_btime: crate::Reg<can_btime::CAN_BTIME_SPEC>,
    #[doc = "0x10 - Interrupt Identifier Register"]
    pub can_iidr: crate::Reg<can_iidr::CAN_IIDR_SPEC>,
    #[doc = "0x14 - Test Register (Register Map Note 1)"]
    pub can_test: crate::Reg<can_test::CAN_TEST_SPEC>,
    #[doc = "0x18 - Baud Rate Prescaler Extension Register"]
    pub can_brpe: crate::Reg<can_brpe::CAN_BRPE_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - IFn (Register Map Note 2) Command Request Registers"]
    pub can_if1_creq: crate::Reg<can_if1_creq::CAN_IF1_CREQ_SPEC>,
    #[doc = "0x24 - IFn Command Mask Registers"]
    pub can_if1_cmask: crate::Reg<can_if1_cmask::CAN_IF1_CMASK_SPEC>,
    #[doc = "0x28 - IFn Mask 1 Registers"]
    pub can_if1_mask1: crate::Reg<can_if1_mask1::CAN_IF1_MASK1_SPEC>,
    #[doc = "0x2c - IFn Mask 2 Registers"]
    pub can_if1_mask2: crate::Reg<can_if1_mask2::CAN_IF1_MASK2_SPEC>,
    #[doc = "0x30 - IFn Arbitration 1 Registers"]
    pub can_if1_arb1: crate::Reg<can_if1_arb1::CAN_IF1_ARB1_SPEC>,
    #[doc = "0x34 - IFn Arbitration 2 Registers"]
    pub can_if1_arb2: crate::Reg<can_if1_arb2::CAN_IF1_ARB2_SPEC>,
    #[doc = "0x38 - IFn Message Control Registers"]
    pub can_if1_mcon: crate::Reg<can_if1_mcon::CAN_IF1_MCON_SPEC>,
    #[doc = "0x3c - IFn Data A1 Registers (Register Map Note 3)"]
    pub can_if1_dat_a1: crate::Reg<can_if1_dat_a1::CAN_IF1_DAT_A1_SPEC>,
    #[doc = "0x40 - IFn Data A2 Registers (Register Map Note 3)"]
    pub can_if1_dat_a2: crate::Reg<can_if1_dat_a2::CAN_IF1_DAT_A2_SPEC>,
    #[doc = "0x44 - IFn Data B1 Registers (Register Map Note 3)"]
    pub can_if1_dat_b1: crate::Reg<can_if1_dat_b1::CAN_IF1_DAT_B1_SPEC>,
    #[doc = "0x48 - IFn Data B2 Registers (Register Map Note 3)"]
    pub can_if1_dat_b2: crate::Reg<can_if1_dat_b2::CAN_IF1_DAT_B2_SPEC>,
    _reserved18: [u8; 52usize],
    #[doc = "0x80 - IFn (Register Map Note 2) Command Request Registers"]
    pub can_if2_creq: crate::Reg<can_if2_creq::CAN_IF2_CREQ_SPEC>,
    #[doc = "0x84 - IFn Command Mask Registers"]
    pub can_if2_cmask: crate::Reg<can_if2_cmask::CAN_IF2_CMASK_SPEC>,
    #[doc = "0x88 - IFn Mask 1 Registers"]
    pub can_if2_mask1: crate::Reg<can_if2_mask1::CAN_IF2_MASK1_SPEC>,
    #[doc = "0x8c - IFn Mask 2 Registers"]
    pub can_if2_mask2: crate::Reg<can_if2_mask2::CAN_IF2_MASK2_SPEC>,
    #[doc = "0x90 - IFn Arbitration 1 Registers"]
    pub can_if2_arb1: crate::Reg<can_if2_arb1::CAN_IF2_ARB1_SPEC>,
    #[doc = "0x94 - IFn Arbitration 2 Registers"]
    pub can_if2_arb2: crate::Reg<can_if2_arb2::CAN_IF2_ARB2_SPEC>,
    #[doc = "0x98 - IFn Message Control Registers"]
    pub can_if2_mcon: crate::Reg<can_if2_mcon::CAN_IF2_MCON_SPEC>,
    #[doc = "0x9c - IFn Data A1 Registers (Register Map Note 3)"]
    pub can_if2_dat_a1: crate::Reg<can_if2_dat_a1::CAN_IF2_DAT_A1_SPEC>,
    #[doc = "0xa0 - IFn Data A2 Registers (Register Map Note 3)"]
    pub can_if2_dat_a2: crate::Reg<can_if2_dat_a2::CAN_IF2_DAT_A2_SPEC>,
    #[doc = "0xa4 - IFn Data B1 Registers (Register Map Note 3)"]
    pub can_if2_dat_b1: crate::Reg<can_if2_dat_b1::CAN_IF2_DAT_B1_SPEC>,
    #[doc = "0xa8 - IFn Data B2 Registers (Register Map Note 3)"]
    pub can_if2_dat_b2: crate::Reg<can_if2_dat_b2::CAN_IF2_DAT_B2_SPEC>,
    _reserved29: [u8; 84usize],
    #[doc = "0x100 - Transmission Request Register 1"]
    pub can_txreq1: crate::Reg<can_txreq1::CAN_TXREQ1_SPEC>,
    #[doc = "0x104 - Transmission Request Register 2"]
    pub can_txreq2: crate::Reg<can_txreq2::CAN_TXREQ2_SPEC>,
    _reserved31: [u8; 24usize],
    #[doc = "0x120 - New Data Register 1"]
    pub can_ndat1: crate::Reg<can_ndat1::CAN_NDAT1_SPEC>,
    #[doc = "0x124 - New Data Register 2"]
    pub can_ndat2: crate::Reg<can_ndat2::CAN_NDAT2_SPEC>,
    _reserved33: [u8; 24usize],
    #[doc = "0x140 - Interrupt Pending Register 1"]
    pub can_ipnd1: crate::Reg<can_ipnd1::CAN_IPND1_SPEC>,
    #[doc = "0x144 - Interrupt Pending Register 2"]
    pub can_ipnd2: crate::Reg<can_ipnd2::CAN_IPND2_SPEC>,
    _reserved35: [u8; 24usize],
    #[doc = "0x160 - Message Valid Register 1"]
    pub can_mvld1: crate::Reg<can_mvld1::CAN_MVLD1_SPEC>,
    #[doc = "0x164 - Message Valid Register 2"]
    pub can_mvld2: crate::Reg<can_mvld2::CAN_MVLD2_SPEC>,
    #[doc = "0x168 - Wake-up Enable Control Register"]
    pub can_wu_en: crate::Reg<can_wu_en::CAN_WU_EN_SPEC>,
    #[doc = "0x16c - Wake-up Status Register"]
    pub can_wu_status: crate::Reg<can_wu_status::CAN_WU_STATUS_SPEC>,
}
#[doc = "CAN_CON register accessor: an alias for `Reg<CAN_CON_SPEC>`"]
pub type CAN_CON = crate::Reg<can_con::CAN_CON_SPEC>;
#[doc = "Control Register"]
pub mod can_con;
#[doc = "CAN_STATUS register accessor: an alias for `Reg<CAN_STATUS_SPEC>`"]
pub type CAN_STATUS = crate::Reg<can_status::CAN_STATUS_SPEC>;
#[doc = "Status Register"]
pub mod can_status;
#[doc = "CAN_ERR register accessor: an alias for `Reg<CAN_ERR_SPEC>`"]
pub type CAN_ERR = crate::Reg<can_err::CAN_ERR_SPEC>;
#[doc = "Error Counter Register"]
pub mod can_err;
#[doc = "CAN_BTIME register accessor: an alias for `Reg<CAN_BTIME_SPEC>`"]
pub type CAN_BTIME = crate::Reg<can_btime::CAN_BTIME_SPEC>;
#[doc = "Bit Timing Register"]
pub mod can_btime;
#[doc = "CAN_IIDR register accessor: an alias for `Reg<CAN_IIDR_SPEC>`"]
pub type CAN_IIDR = crate::Reg<can_iidr::CAN_IIDR_SPEC>;
#[doc = "Interrupt Identifier Register"]
pub mod can_iidr;
#[doc = "CAN_TEST register accessor: an alias for `Reg<CAN_TEST_SPEC>`"]
pub type CAN_TEST = crate::Reg<can_test::CAN_TEST_SPEC>;
#[doc = "Test Register (Register Map Note 1)"]
pub mod can_test;
#[doc = "CAN_BRPE register accessor: an alias for `Reg<CAN_BRPE_SPEC>`"]
pub type CAN_BRPE = crate::Reg<can_brpe::CAN_BRPE_SPEC>;
#[doc = "Baud Rate Prescaler Extension Register"]
pub mod can_brpe;
#[doc = "CAN_IF1_CREQ register accessor: an alias for `Reg<CAN_IF1_CREQ_SPEC>`"]
pub type CAN_IF1_CREQ = crate::Reg<can_if1_creq::CAN_IF1_CREQ_SPEC>;
#[doc = "IFn (Register Map Note 2) Command Request Registers"]
pub mod can_if1_creq;
#[doc = "CAN_IF2_CREQ register accessor: an alias for `Reg<CAN_IF2_CREQ_SPEC>`"]
pub type CAN_IF2_CREQ = crate::Reg<can_if2_creq::CAN_IF2_CREQ_SPEC>;
#[doc = "IFn (Register Map Note 2) Command Request Registers"]
pub mod can_if2_creq;
#[doc = "CAN_IF1_CMASK register accessor: an alias for `Reg<CAN_IF1_CMASK_SPEC>`"]
pub type CAN_IF1_CMASK = crate::Reg<can_if1_cmask::CAN_IF1_CMASK_SPEC>;
#[doc = "IFn Command Mask Registers"]
pub mod can_if1_cmask;
#[doc = "CAN_IF2_CMASK register accessor: an alias for `Reg<CAN_IF2_CMASK_SPEC>`"]
pub type CAN_IF2_CMASK = crate::Reg<can_if2_cmask::CAN_IF2_CMASK_SPEC>;
#[doc = "IFn Command Mask Registers"]
pub mod can_if2_cmask;
#[doc = "CAN_IF1_MASK1 register accessor: an alias for `Reg<CAN_IF1_MASK1_SPEC>`"]
pub type CAN_IF1_MASK1 = crate::Reg<can_if1_mask1::CAN_IF1_MASK1_SPEC>;
#[doc = "IFn Mask 1 Registers"]
pub mod can_if1_mask1;
#[doc = "CAN_IF2_MASK1 register accessor: an alias for `Reg<CAN_IF2_MASK1_SPEC>`"]
pub type CAN_IF2_MASK1 = crate::Reg<can_if2_mask1::CAN_IF2_MASK1_SPEC>;
#[doc = "IFn Mask 1 Registers"]
pub mod can_if2_mask1;
#[doc = "CAN_IF1_MASK2 register accessor: an alias for `Reg<CAN_IF1_MASK2_SPEC>`"]
pub type CAN_IF1_MASK2 = crate::Reg<can_if1_mask2::CAN_IF1_MASK2_SPEC>;
#[doc = "IFn Mask 2 Registers"]
pub mod can_if1_mask2;
#[doc = "CAN_IF2_MASK2 register accessor: an alias for `Reg<CAN_IF2_MASK2_SPEC>`"]
pub type CAN_IF2_MASK2 = crate::Reg<can_if2_mask2::CAN_IF2_MASK2_SPEC>;
#[doc = "IFn Mask 2 Registers"]
pub mod can_if2_mask2;
#[doc = "CAN_IF1_ARB1 register accessor: an alias for `Reg<CAN_IF1_ARB1_SPEC>`"]
pub type CAN_IF1_ARB1 = crate::Reg<can_if1_arb1::CAN_IF1_ARB1_SPEC>;
#[doc = "IFn Arbitration 1 Registers"]
pub mod can_if1_arb1;
#[doc = "CAN_IF2_ARB1 register accessor: an alias for `Reg<CAN_IF2_ARB1_SPEC>`"]
pub type CAN_IF2_ARB1 = crate::Reg<can_if2_arb1::CAN_IF2_ARB1_SPEC>;
#[doc = "IFn Arbitration 1 Registers"]
pub mod can_if2_arb1;
#[doc = "CAN_IF1_ARB2 register accessor: an alias for `Reg<CAN_IF1_ARB2_SPEC>`"]
pub type CAN_IF1_ARB2 = crate::Reg<can_if1_arb2::CAN_IF1_ARB2_SPEC>;
#[doc = "IFn Arbitration 2 Registers"]
pub mod can_if1_arb2;
#[doc = "CAN_IF2_ARB2 register accessor: an alias for `Reg<CAN_IF2_ARB2_SPEC>`"]
pub type CAN_IF2_ARB2 = crate::Reg<can_if2_arb2::CAN_IF2_ARB2_SPEC>;
#[doc = "IFn Arbitration 2 Registers"]
pub mod can_if2_arb2;
#[doc = "CAN_IF1_MCON register accessor: an alias for `Reg<CAN_IF1_MCON_SPEC>`"]
pub type CAN_IF1_MCON = crate::Reg<can_if1_mcon::CAN_IF1_MCON_SPEC>;
#[doc = "IFn Message Control Registers"]
pub mod can_if1_mcon;
#[doc = "CAN_IF2_MCON register accessor: an alias for `Reg<CAN_IF2_MCON_SPEC>`"]
pub type CAN_IF2_MCON = crate::Reg<can_if2_mcon::CAN_IF2_MCON_SPEC>;
#[doc = "IFn Message Control Registers"]
pub mod can_if2_mcon;
#[doc = "CAN_IF1_DAT_A1 register accessor: an alias for `Reg<CAN_IF1_DAT_A1_SPEC>`"]
pub type CAN_IF1_DAT_A1 = crate::Reg<can_if1_dat_a1::CAN_IF1_DAT_A1_SPEC>;
#[doc = "IFn Data A1 Registers (Register Map Note 3)"]
pub mod can_if1_dat_a1;
#[doc = "CAN_IF2_DAT_A1 register accessor: an alias for `Reg<CAN_IF2_DAT_A1_SPEC>`"]
pub type CAN_IF2_DAT_A1 = crate::Reg<can_if2_dat_a1::CAN_IF2_DAT_A1_SPEC>;
#[doc = "IFn Data A1 Registers (Register Map Note 3)"]
pub mod can_if2_dat_a1;
#[doc = "CAN_IF1_DAT_A2 register accessor: an alias for `Reg<CAN_IF1_DAT_A2_SPEC>`"]
pub type CAN_IF1_DAT_A2 = crate::Reg<can_if1_dat_a2::CAN_IF1_DAT_A2_SPEC>;
#[doc = "IFn Data A2 Registers (Register Map Note 3)"]
pub mod can_if1_dat_a2;
#[doc = "CAN_IF2_DAT_A2 register accessor: an alias for `Reg<CAN_IF2_DAT_A2_SPEC>`"]
pub type CAN_IF2_DAT_A2 = crate::Reg<can_if2_dat_a2::CAN_IF2_DAT_A2_SPEC>;
#[doc = "IFn Data A2 Registers (Register Map Note 3)"]
pub mod can_if2_dat_a2;
#[doc = "CAN_IF1_DAT_B1 register accessor: an alias for `Reg<CAN_IF1_DAT_B1_SPEC>`"]
pub type CAN_IF1_DAT_B1 = crate::Reg<can_if1_dat_b1::CAN_IF1_DAT_B1_SPEC>;
#[doc = "IFn Data B1 Registers (Register Map Note 3)"]
pub mod can_if1_dat_b1;
#[doc = "CAN_IF2_DAT_B1 register accessor: an alias for `Reg<CAN_IF2_DAT_B1_SPEC>`"]
pub type CAN_IF2_DAT_B1 = crate::Reg<can_if2_dat_b1::CAN_IF2_DAT_B1_SPEC>;
#[doc = "IFn Data B1 Registers (Register Map Note 3)"]
pub mod can_if2_dat_b1;
#[doc = "CAN_IF1_DAT_B2 register accessor: an alias for `Reg<CAN_IF1_DAT_B2_SPEC>`"]
pub type CAN_IF1_DAT_B2 = crate::Reg<can_if1_dat_b2::CAN_IF1_DAT_B2_SPEC>;
#[doc = "IFn Data B2 Registers (Register Map Note 3)"]
pub mod can_if1_dat_b2;
#[doc = "CAN_IF2_DAT_B2 register accessor: an alias for `Reg<CAN_IF2_DAT_B2_SPEC>`"]
pub type CAN_IF2_DAT_B2 = crate::Reg<can_if2_dat_b2::CAN_IF2_DAT_B2_SPEC>;
#[doc = "IFn Data B2 Registers (Register Map Note 3)"]
pub mod can_if2_dat_b2;
#[doc = "CAN_TXREQ1 register accessor: an alias for `Reg<CAN_TXREQ1_SPEC>`"]
pub type CAN_TXREQ1 = crate::Reg<can_txreq1::CAN_TXREQ1_SPEC>;
#[doc = "Transmission Request Register 1"]
pub mod can_txreq1;
#[doc = "CAN_TXREQ2 register accessor: an alias for `Reg<CAN_TXREQ2_SPEC>`"]
pub type CAN_TXREQ2 = crate::Reg<can_txreq2::CAN_TXREQ2_SPEC>;
#[doc = "Transmission Request Register 2"]
pub mod can_txreq2;
#[doc = "CAN_NDAT1 register accessor: an alias for `Reg<CAN_NDAT1_SPEC>`"]
pub type CAN_NDAT1 = crate::Reg<can_ndat1::CAN_NDAT1_SPEC>;
#[doc = "New Data Register 1"]
pub mod can_ndat1;
#[doc = "CAN_NDAT2 register accessor: an alias for `Reg<CAN_NDAT2_SPEC>`"]
pub type CAN_NDAT2 = crate::Reg<can_ndat2::CAN_NDAT2_SPEC>;
#[doc = "New Data Register 2"]
pub mod can_ndat2;
#[doc = "CAN_IPND1 register accessor: an alias for `Reg<CAN_IPND1_SPEC>`"]
pub type CAN_IPND1 = crate::Reg<can_ipnd1::CAN_IPND1_SPEC>;
#[doc = "Interrupt Pending Register 1"]
pub mod can_ipnd1;
#[doc = "CAN_IPND2 register accessor: an alias for `Reg<CAN_IPND2_SPEC>`"]
pub type CAN_IPND2 = crate::Reg<can_ipnd2::CAN_IPND2_SPEC>;
#[doc = "Interrupt Pending Register 2"]
pub mod can_ipnd2;
#[doc = "CAN_MVLD1 register accessor: an alias for `Reg<CAN_MVLD1_SPEC>`"]
pub type CAN_MVLD1 = crate::Reg<can_mvld1::CAN_MVLD1_SPEC>;
#[doc = "Message Valid Register 1"]
pub mod can_mvld1;
#[doc = "CAN_MVLD2 register accessor: an alias for `Reg<CAN_MVLD2_SPEC>`"]
pub type CAN_MVLD2 = crate::Reg<can_mvld2::CAN_MVLD2_SPEC>;
#[doc = "Message Valid Register 2"]
pub mod can_mvld2;
#[doc = "CAN_WU_EN register accessor: an alias for `Reg<CAN_WU_EN_SPEC>`"]
pub type CAN_WU_EN = crate::Reg<can_wu_en::CAN_WU_EN_SPEC>;
#[doc = "Wake-up Enable Control Register"]
pub mod can_wu_en;
#[doc = "CAN_WU_STATUS register accessor: an alias for `Reg<CAN_WU_STATUS_SPEC>`"]
pub type CAN_WU_STATUS = crate::Reg<can_wu_status::CAN_WU_STATUS_SPEC>;
#[doc = "Wake-up Status Register"]
pub mod can_wu_status;
