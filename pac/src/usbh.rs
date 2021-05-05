#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host Controller Revision Register"]
    pub hcrevision: crate::Reg<hcrevision::HCREVISION_SPEC>,
    #[doc = "0x04 - Host Controller Control Register"]
    pub hccontrol: crate::Reg<hccontrol::HCCONTROL_SPEC>,
    #[doc = "0x08 - Host Controller CMD Status Register"]
    pub hccommandstatus: crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>,
    #[doc = "0x0c - Host Controller Interrupt Status Register"]
    pub hcinterruptstatus: crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>,
    #[doc = "0x10 - Host Controller Interrupt Enable Register"]
    pub hcinterruptenable: crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>,
    #[doc = "0x14 - Host Controller Interrupt Disable Register"]
    pub hcinterruptdisable: crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>,
    #[doc = "0x18 - Host Controller Communication Area Register"]
    pub hchcca: crate::Reg<hchcca::HCHCCA_SPEC>,
    #[doc = "0x1c - Host Controller Period Current ED Register"]
    pub hcperiodcurrented: crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>,
    #[doc = "0x20 - Host Controller Control Head ED Register"]
    pub hccontrolheaded: crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>,
    #[doc = "0x24 - Host Controller Control Current ED Register"]
    pub hccontrolcurrented: crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>,
    #[doc = "0x28 - Host Controller Bulk Head ED Register"]
    pub hcbulkheaded: crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>,
    #[doc = "0x2c - Host Controller Bulk Current ED Register"]
    pub hcbulkcurrented: crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>,
    #[doc = "0x30 - Host Controller Done Head Register"]
    pub hcdonehead: crate::Reg<hcdonehead::HCDONEHEAD_SPEC>,
    #[doc = "0x34 - Host Controller Frame Interval Register"]
    pub hcfminterval: crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>,
    #[doc = "0x38 - Host Controller Frame Remaining Register"]
    pub hcfmremaining: crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>,
    #[doc = "0x3c - Host Controller Frame Number Register"]
    pub hcfmnumber: crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>,
    #[doc = "0x40 - Host Controller Periodic Start Register"]
    pub hcperiodicstart: crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>,
    #[doc = "0x44 - Host Controller Low-speed Threshold Register"]
    pub hclsthreshold: crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>,
    #[doc = "0x48 - Host Controller Root Hub Descriptor A Register"]
    pub hcrhdescriptora: crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>,
    #[doc = "0x4c - Host Controller Root Hub Descriptor B Register"]
    pub hcrhdescriptorb: crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>,
    #[doc = "0x50 - Host Controller Root Hub Status Register"]
    pub hcrhstatus: crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>,
    #[doc = "0x54 - Host Controller Root Hub Port Status \\[1\\]"]
    pub hcrhportstatus1: crate::Reg<hcrhportstatus1::HCRHPORTSTATUS1_SPEC>,
    _reserved22: [u8; 424usize],
    #[doc = "0x200 - Host Controller PHY Control Regsiter"]
    pub hcphycontrol: crate::Reg<hcphycontrol::HCPHYCONTROL_SPEC>,
    #[doc = "0x204 - Host Controller Miscellaneous Control Register"]
    pub hcmisccontrol: crate::Reg<hcmisccontrol::HCMISCCONTROL_SPEC>,
}
#[doc = "HCREVISION register accessor: an alias for `Reg<HCREVISION_SPEC>`"]
pub type HCREVISION = crate::Reg<hcrevision::HCREVISION_SPEC>;
#[doc = "Host Controller Revision Register"]
pub mod hcrevision;
#[doc = "HCCONTROL register accessor: an alias for `Reg<HCCONTROL_SPEC>`"]
pub type HCCONTROL = crate::Reg<hccontrol::HCCONTROL_SPEC>;
#[doc = "Host Controller Control Register"]
pub mod hccontrol;
#[doc = "HCCOMMANDSTATUS register accessor: an alias for `Reg<HCCOMMANDSTATUS_SPEC>`"]
pub type HCCOMMANDSTATUS = crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>;
#[doc = "Host Controller CMD Status Register"]
pub mod hccommandstatus;
#[doc = "HCINTERRUPTSTATUS register accessor: an alias for `Reg<HCINTERRUPTSTATUS_SPEC>`"]
pub type HCINTERRUPTSTATUS = crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>;
#[doc = "Host Controller Interrupt Status Register"]
pub mod hcinterruptstatus;
#[doc = "HCINTERRUPTENABLE register accessor: an alias for `Reg<HCINTERRUPTENABLE_SPEC>`"]
pub type HCINTERRUPTENABLE = crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>;
#[doc = "Host Controller Interrupt Enable Register"]
pub mod hcinterruptenable;
#[doc = "HCINTERRUPTDISABLE register accessor: an alias for `Reg<HCINTERRUPTDISABLE_SPEC>`"]
pub type HCINTERRUPTDISABLE = crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>;
#[doc = "Host Controller Interrupt Disable Register"]
pub mod hcinterruptdisable;
#[doc = "HCHCCA register accessor: an alias for `Reg<HCHCCA_SPEC>`"]
pub type HCHCCA = crate::Reg<hchcca::HCHCCA_SPEC>;
#[doc = "Host Controller Communication Area Register"]
pub mod hchcca;
#[doc = "HCPERIODCURRENTED register accessor: an alias for `Reg<HCPERIODCURRENTED_SPEC>`"]
pub type HCPERIODCURRENTED = crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>;
#[doc = "Host Controller Period Current ED Register"]
pub mod hcperiodcurrented;
#[doc = "HCCONTROLHEADED register accessor: an alias for `Reg<HCCONTROLHEADED_SPEC>`"]
pub type HCCONTROLHEADED = crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>;
#[doc = "Host Controller Control Head ED Register"]
pub mod hccontrolheaded;
#[doc = "HCCONTROLCURRENTED register accessor: an alias for `Reg<HCCONTROLCURRENTED_SPEC>`"]
pub type HCCONTROLCURRENTED = crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>;
#[doc = "Host Controller Control Current ED Register"]
pub mod hccontrolcurrented;
#[doc = "HCBULKHEADED register accessor: an alias for `Reg<HCBULKHEADED_SPEC>`"]
pub type HCBULKHEADED = crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>;
#[doc = "Host Controller Bulk Head ED Register"]
pub mod hcbulkheaded;
#[doc = "HCBULKCURRENTED register accessor: an alias for `Reg<HCBULKCURRENTED_SPEC>`"]
pub type HCBULKCURRENTED = crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>;
#[doc = "Host Controller Bulk Current ED Register"]
pub mod hcbulkcurrented;
#[doc = "HCDONEHEAD register accessor: an alias for `Reg<HCDONEHEAD_SPEC>`"]
pub type HCDONEHEAD = crate::Reg<hcdonehead::HCDONEHEAD_SPEC>;
#[doc = "Host Controller Done Head Register"]
pub mod hcdonehead;
#[doc = "HCFMINTERVAL register accessor: an alias for `Reg<HCFMINTERVAL_SPEC>`"]
pub type HCFMINTERVAL = crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>;
#[doc = "Host Controller Frame Interval Register"]
pub mod hcfminterval;
#[doc = "HCFMREMAINING register accessor: an alias for `Reg<HCFMREMAINING_SPEC>`"]
pub type HCFMREMAINING = crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>;
#[doc = "Host Controller Frame Remaining Register"]
pub mod hcfmremaining;
#[doc = "HCFMNUMBER register accessor: an alias for `Reg<HCFMNUMBER_SPEC>`"]
pub type HCFMNUMBER = crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>;
#[doc = "Host Controller Frame Number Register"]
pub mod hcfmnumber;
#[doc = "HCPERIODICSTART register accessor: an alias for `Reg<HCPERIODICSTART_SPEC>`"]
pub type HCPERIODICSTART = crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>;
#[doc = "Host Controller Periodic Start Register"]
pub mod hcperiodicstart;
#[doc = "HCLSTHRESHOLD register accessor: an alias for `Reg<HCLSTHRESHOLD_SPEC>`"]
pub type HCLSTHRESHOLD = crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>;
#[doc = "Host Controller Low-speed Threshold Register"]
pub mod hclsthreshold;
#[doc = "HCRHDESCRIPTORA register accessor: an alias for `Reg<HCRHDESCRIPTORA_SPEC>`"]
pub type HCRHDESCRIPTORA = crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>;
#[doc = "Host Controller Root Hub Descriptor A Register"]
pub mod hcrhdescriptora;
#[doc = "HCRHDESCRIPTORB register accessor: an alias for `Reg<HCRHDESCRIPTORB_SPEC>`"]
pub type HCRHDESCRIPTORB = crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>;
#[doc = "Host Controller Root Hub Descriptor B Register"]
pub mod hcrhdescriptorb;
#[doc = "HCRHSTATUS register accessor: an alias for `Reg<HCRHSTATUS_SPEC>`"]
pub type HCRHSTATUS = crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>;
#[doc = "Host Controller Root Hub Status Register"]
pub mod hcrhstatus;
#[doc = "HCRHPORTSTATUS1 register accessor: an alias for `Reg<HCRHPORTSTATUS1_SPEC>`"]
pub type HCRHPORTSTATUS1 = crate::Reg<hcrhportstatus1::HCRHPORTSTATUS1_SPEC>;
#[doc = "Host Controller Root Hub Port Status \\[1\\]"]
pub mod hcrhportstatus1;
#[doc = "HCPHYCONTROL register accessor: an alias for `Reg<HCPHYCONTROL_SPEC>`"]
pub type HCPHYCONTROL = crate::Reg<hcphycontrol::HCPHYCONTROL_SPEC>;
#[doc = "Host Controller PHY Control Regsiter"]
pub mod hcphycontrol;
#[doc = "HCMISCCONTROL register accessor: an alias for `Reg<HCMISCCONTROL_SPEC>`"]
pub type HCMISCCONTROL = crate::Reg<hcmisccontrol::HCMISCCONTROL_SPEC>;
#[doc = "Host Controller Miscellaneous Control Register"]
pub mod hcmisccontrol;
