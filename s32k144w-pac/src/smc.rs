#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - SMC Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x08 - Power Mode Protection register"]
    pub pmprot: crate::Reg<pmprot::PMPROT_SPEC>,
    #[doc = "0x0c - Power Mode Control register"]
    pub pmctrl: crate::Reg<pmctrl::PMCTRL_SPEC>,
    #[doc = "0x10 - Stop Control Register"]
    pub stopctrl: crate::Reg<stopctrl::STOPCTRL_SPEC>,
    #[doc = "0x14 - Power Mode Status register"]
    pub pmstat: crate::Reg<pmstat::PMSTAT_SPEC>,
}
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "SMC Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "SMC Parameter Register"]
pub mod param;
#[doc = "PMPROT register accessor: an alias for `Reg<PMPROT_SPEC>`"]
pub type PMPROT = crate::Reg<pmprot::PMPROT_SPEC>;
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "PMCTRL register accessor: an alias for `Reg<PMCTRL_SPEC>`"]
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "STOPCTRL register accessor: an alias for `Reg<STOPCTRL_SPEC>`"]
pub type STOPCTRL = crate::Reg<stopctrl::STOPCTRL_SPEC>;
#[doc = "Stop Control Register"]
pub mod stopctrl;
#[doc = "PMSTAT register accessor: an alias for `Reg<PMSTAT_SPEC>`"]
pub type PMSTAT = crate::Reg<pmstat::PMSTAT_SPEC>;
#[doc = "Power Mode Status register"]
pub mod pmstat;
