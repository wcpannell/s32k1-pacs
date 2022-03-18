#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor X Type Register"]
    pub cpx_type: crate::Reg<cpx_type::CPXTYPE_SPEC>,
    #[doc = "0x04 - Processor X Number Register"]
    pub cpx_num: crate::Reg<cpx_num::CPXNUM_SPEC>,
    #[doc = "0x08 - Processor X Master Register"]
    pub cpx_master: crate::Reg<cpx_master::CPXMASTER_SPEC>,
    #[doc = "0x0c - Processor X Count Register"]
    pub cpx_count: crate::Reg<cpx_count::CPXCOUNT_SPEC>,
    #[doc = "0x10 - Processor X Configuration Register 0"]
    pub cpx_cfg0: crate::Reg<cpx_cfg0::CPXCFG0_SPEC>,
    #[doc = "0x14 - Processor X Configuration Register 1"]
    pub cpx_cfg1: crate::Reg<cpx_cfg1::CPXCFG1_SPEC>,
    #[doc = "0x18 - Processor X Configuration Register 2"]
    pub cpx_cfg2: crate::Reg<cpx_cfg2::CPXCFG2_SPEC>,
    #[doc = "0x1c - Processor X Configuration Register 3"]
    pub cpx_cfg3: crate::Reg<cpx_cfg3::CPXCFG3_SPEC>,
    #[doc = "0x20 - Processor 0 Type Register"]
    pub cp0type: crate::Reg<cp0type::CP0TYPE_SPEC>,
    #[doc = "0x24 - Processor 0 Number Register"]
    pub cp0num: crate::Reg<cp0num::CP0NUM_SPEC>,
    #[doc = "0x28 - Processor 0 Master Register"]
    pub cp0master: crate::Reg<cp0master::CP0MASTER_SPEC>,
    #[doc = "0x2c - Processor 0 Count Register"]
    pub cp0count: crate::Reg<cp0count::CP0COUNT_SPEC>,
    #[doc = "0x30 - Processor 0 Configuration Register 0"]
    pub cp0cfg0: crate::Reg<cp0cfg0::CP0CFG0_SPEC>,
    #[doc = "0x34 - Processor 0 Configuration Register 1"]
    pub cp0cfg1: crate::Reg<cp0cfg1::CP0CFG1_SPEC>,
    #[doc = "0x38 - Processor 0 Configuration Register 2"]
    pub cp0cfg2: crate::Reg<cp0cfg2::CP0CFG2_SPEC>,
    #[doc = "0x3c - Processor 0 Configuration Register 3"]
    pub cp0cfg3: crate::Reg<cp0cfg3::CP0CFG3_SPEC>,
    _reserved16: [u8; 0x03c0],
    #[doc = "0x400 - On-Chip Memory Descriptor Register"]
    pub ocmdr0: crate::Reg<ocmdr0::OCMDR0_SPEC>,
    #[doc = "0x404 - On-Chip Memory Descriptor Register"]
    pub ocmdr1: crate::Reg<ocmdr1::OCMDR1_SPEC>,
    #[doc = "0x408 - On-Chip Memory Descriptor Register"]
    pub ocmdr2: crate::Reg<ocmdr2::OCMDR2_SPEC>,
}
#[doc = "CPxTYPE register accessor: an alias for `Reg<CPXTYPE_SPEC>`"]
pub type CPXTYPE = crate::Reg<cpx_type::CPXTYPE_SPEC>;
#[doc = "Processor X Type Register"]
pub mod cpx_type;
#[doc = "CPxNUM register accessor: an alias for `Reg<CPXNUM_SPEC>`"]
pub type CPXNUM = crate::Reg<cpx_num::CPXNUM_SPEC>;
#[doc = "Processor X Number Register"]
pub mod cpx_num;
#[doc = "CPxMASTER register accessor: an alias for `Reg<CPXMASTER_SPEC>`"]
pub type CPXMASTER = crate::Reg<cpx_master::CPXMASTER_SPEC>;
#[doc = "Processor X Master Register"]
pub mod cpx_master;
#[doc = "CPxCOUNT register accessor: an alias for `Reg<CPXCOUNT_SPEC>`"]
pub type CPXCOUNT = crate::Reg<cpx_count::CPXCOUNT_SPEC>;
#[doc = "Processor X Count Register"]
pub mod cpx_count;
#[doc = "CPxCFG0 register accessor: an alias for `Reg<CPXCFG0_SPEC>`"]
pub type CPXCFG0 = crate::Reg<cpx_cfg0::CPXCFG0_SPEC>;
#[doc = "Processor X Configuration Register 0"]
pub mod cpx_cfg0;
#[doc = "CPxCFG1 register accessor: an alias for `Reg<CPXCFG1_SPEC>`"]
pub type CPXCFG1 = crate::Reg<cpx_cfg1::CPXCFG1_SPEC>;
#[doc = "Processor X Configuration Register 1"]
pub mod cpx_cfg1;
#[doc = "CPxCFG2 register accessor: an alias for `Reg<CPXCFG2_SPEC>`"]
pub type CPXCFG2 = crate::Reg<cpx_cfg2::CPXCFG2_SPEC>;
#[doc = "Processor X Configuration Register 2"]
pub mod cpx_cfg2;
#[doc = "CPxCFG3 register accessor: an alias for `Reg<CPXCFG3_SPEC>`"]
pub type CPXCFG3 = crate::Reg<cpx_cfg3::CPXCFG3_SPEC>;
#[doc = "Processor X Configuration Register 3"]
pub mod cpx_cfg3;
#[doc = "CP0TYPE register accessor: an alias for `Reg<CP0TYPE_SPEC>`"]
pub type CP0TYPE = crate::Reg<cp0type::CP0TYPE_SPEC>;
#[doc = "Processor 0 Type Register"]
pub mod cp0type;
#[doc = "CP0NUM register accessor: an alias for `Reg<CP0NUM_SPEC>`"]
pub type CP0NUM = crate::Reg<cp0num::CP0NUM_SPEC>;
#[doc = "Processor 0 Number Register"]
pub mod cp0num;
#[doc = "CP0MASTER register accessor: an alias for `Reg<CP0MASTER_SPEC>`"]
pub type CP0MASTER = crate::Reg<cp0master::CP0MASTER_SPEC>;
#[doc = "Processor 0 Master Register"]
pub mod cp0master;
#[doc = "CP0COUNT register accessor: an alias for `Reg<CP0COUNT_SPEC>`"]
pub type CP0COUNT = crate::Reg<cp0count::CP0COUNT_SPEC>;
#[doc = "Processor 0 Count Register"]
pub mod cp0count;
#[doc = "CP0CFG0 register accessor: an alias for `Reg<CP0CFG0_SPEC>`"]
pub type CP0CFG0 = crate::Reg<cp0cfg0::CP0CFG0_SPEC>;
#[doc = "Processor 0 Configuration Register 0"]
pub mod cp0cfg0;
#[doc = "CP0CFG1 register accessor: an alias for `Reg<CP0CFG1_SPEC>`"]
pub type CP0CFG1 = crate::Reg<cp0cfg1::CP0CFG1_SPEC>;
#[doc = "Processor 0 Configuration Register 1"]
pub mod cp0cfg1;
#[doc = "CP0CFG2 register accessor: an alias for `Reg<CP0CFG2_SPEC>`"]
pub type CP0CFG2 = crate::Reg<cp0cfg2::CP0CFG2_SPEC>;
#[doc = "Processor 0 Configuration Register 2"]
pub mod cp0cfg2;
#[doc = "CP0CFG3 register accessor: an alias for `Reg<CP0CFG3_SPEC>`"]
pub type CP0CFG3 = crate::Reg<cp0cfg3::CP0CFG3_SPEC>;
#[doc = "Processor 0 Configuration Register 3"]
pub mod cp0cfg3;
#[doc = "OCMDR0 register accessor: an alias for `Reg<OCMDR0_SPEC>`"]
pub type OCMDR0 = crate::Reg<ocmdr0::OCMDR0_SPEC>;
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr0;
#[doc = "OCMDR1 register accessor: an alias for `Reg<OCMDR1_SPEC>`"]
pub type OCMDR1 = crate::Reg<ocmdr1::OCMDR1_SPEC>;
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr1;
#[doc = "OCMDR2 register accessor: an alias for `Reg<OCMDR2_SPEC>`"]
pub type OCMDR2 = crate::Reg<ocmdr2::OCMDR2_SPEC>;
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr2;
