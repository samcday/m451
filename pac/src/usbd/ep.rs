#[doc = "BUFSEG register accessor: an alias for `Reg<BUFSEG_SPEC>`"]
pub type BUFSEG = crate::Reg<bufseg::BUFSEG_SPEC>;
#[doc = "USB Endpoint 0 Buffer Segmentation Register"]
pub mod bufseg;
#[doc = "MXPLD register accessor: an alias for `Reg<MXPLD_SPEC>`"]
pub type MXPLD = crate::Reg<mxpld::MXPLD_SPEC>;
#[doc = "USB Endpoint 0 Maximal Payload Register"]
pub mod mxpld;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "USB Endpoint 0 Configuration Register"]
pub mod cfg;
#[doc = "CFGP register accessor: an alias for `Reg<CFGP_SPEC>`"]
pub type CFGP = crate::Reg<cfgp::CFGP_SPEC>;
#[doc = "USB Endpoint 0 Set Stall and Clear In/Out Ready Control Register"]
pub mod cfgp;
