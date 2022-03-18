#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Modulo"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x0c..0x4c - no description available"]
    pub controls: [CONTROLS; 8],
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: crate::Reg<cntin::CNTIN_SPEC>,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x58 - Synchronization"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: crate::Reg<outinit::OUTINIT_SPEC>,
    #[doc = "0x60 - Output Mask"]
    pub outmask: crate::Reg<outmask::OUTMASK_SPEC>,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: crate::Reg<combine::COMBINE_SPEC>,
    #[doc = "0x68 - Deadtime Configuration"]
    pub deadtime: crate::Reg<deadtime::DEADTIME_SPEC>,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: crate::Reg<exttrig::EXTTRIG_SPEC>,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: crate::Reg<pol::POL_SPEC>,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: crate::Reg<fms::FMS_SPEC>,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: crate::Reg<filter::FILTER_SPEC>,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: crate::Reg<fltctrl::FLTCTRL_SPEC>,
    #[doc = "0x80 - Quadrature Decoder Control And Status"]
    pub qdctrl: crate::Reg<qdctrl::QDCTRL_SPEC>,
    #[doc = "0x84 - Configuration"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: crate::Reg<fltpol::FLTPOL_SPEC>,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: crate::Reg<synconf::SYNCONF_SPEC>,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: crate::Reg<invctrl::INVCTRL_SPEC>,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: crate::Reg<swoctrl::SWOCTRL_SPEC>,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: crate::Reg<pwmload::PWMLOAD_SPEC>,
    #[doc = "0x9c - Half Cycle Register"]
    pub hcr: crate::Reg<hcr::HCR_SPEC>,
    #[doc = "0xa0 - Pair 0 Deadtime Configuration"]
    pub pair0deadtime: crate::Reg<pair0deadtime::PAIR0DEADTIME_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xa8 - Pair 1 Deadtime Configuration"]
    pub pair1deadtime: crate::Reg<pair1deadtime::PAIR1DEADTIME_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0xb0 - Pair 2 Deadtime Configuration"]
    pub pair2deadtime: crate::Reg<pair2deadtime::PAIR2DEADTIME_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0xb8 - Pair 3 Deadtime Configuration"]
    pub pair3deadtime: crate::Reg<pair3deadtime::PAIR3DEADTIME_SPEC>,
    _reserved29: [u8; 0x0144],
    #[doc = "0x200 - Mirror of Modulo Value"]
    pub mod_mirror: crate::Reg<mod_mirror::MOD_MIRROR_SPEC>,
    #[doc = "0x204..0x224 - Mirror of Channel (n) Match Value"]
    pub cv_mirror: [crate::Reg<cv_mirror::CV_MIRROR_SPEC>; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CONTROLS {
    #[doc = "0x00 - Channel (n) Status And Control"]
    pub csc: crate::Reg<self::controls::csc::CSC_SPEC>,
    #[doc = "0x04 - Channel (n) Value"]
    pub cv: crate::Reg<self::controls::cv::CV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod controls;
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status And Control"]
pub mod sc;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CNTIN register accessor: an alias for `Reg<CNTIN_SPEC>`"]
pub type CNTIN = crate::Reg<cntin::CNTIN_SPEC>;
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "Synchronization"]
pub mod sync;
#[doc = "OUTINIT register accessor: an alias for `Reg<OUTINIT_SPEC>`"]
pub type OUTINIT = crate::Reg<outinit::OUTINIT_SPEC>;
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "OUTMASK register accessor: an alias for `Reg<OUTMASK_SPEC>`"]
pub type OUTMASK = crate::Reg<outmask::OUTMASK_SPEC>;
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "COMBINE register accessor: an alias for `Reg<COMBINE_SPEC>`"]
pub type COMBINE = crate::Reg<combine::COMBINE_SPEC>;
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "DEADTIME register accessor: an alias for `Reg<DEADTIME_SPEC>`"]
pub type DEADTIME = crate::Reg<deadtime::DEADTIME_SPEC>;
#[doc = "Deadtime Configuration"]
pub mod deadtime;
#[doc = "EXTTRIG register accessor: an alias for `Reg<EXTTRIG_SPEC>`"]
pub type EXTTRIG = crate::Reg<exttrig::EXTTRIG_SPEC>;
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "POL register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "FMS register accessor: an alias for `Reg<FMS_SPEC>`"]
pub type FMS = crate::Reg<fms::FMS_SPEC>;
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "FILTER register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "FLTCTRL register accessor: an alias for `Reg<FLTCTRL_SPEC>`"]
pub type FLTCTRL = crate::Reg<fltctrl::FLTCTRL_SPEC>;
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "QDCTRL register accessor: an alias for `Reg<QDCTRL_SPEC>`"]
pub type QDCTRL = crate::Reg<qdctrl::QDCTRL_SPEC>;
#[doc = "Quadrature Decoder Control And Status"]
pub mod qdctrl;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration"]
pub mod conf;
#[doc = "FLTPOL register accessor: an alias for `Reg<FLTPOL_SPEC>`"]
pub type FLTPOL = crate::Reg<fltpol::FLTPOL_SPEC>;
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "SYNCONF register accessor: an alias for `Reg<SYNCONF_SPEC>`"]
pub type SYNCONF = crate::Reg<synconf::SYNCONF_SPEC>;
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "INVCTRL register accessor: an alias for `Reg<INVCTRL_SPEC>`"]
pub type INVCTRL = crate::Reg<invctrl::INVCTRL_SPEC>;
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "SWOCTRL register accessor: an alias for `Reg<SWOCTRL_SPEC>`"]
pub type SWOCTRL = crate::Reg<swoctrl::SWOCTRL_SPEC>;
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "PWMLOAD register accessor: an alias for `Reg<PWMLOAD_SPEC>`"]
pub type PWMLOAD = crate::Reg<pwmload::PWMLOAD_SPEC>;
#[doc = "FTM PWM Load"]
pub mod pwmload;
#[doc = "HCR register accessor: an alias for `Reg<HCR_SPEC>`"]
pub type HCR = crate::Reg<hcr::HCR_SPEC>;
#[doc = "Half Cycle Register"]
pub mod hcr;
#[doc = "PAIR0DEADTIME register accessor: an alias for `Reg<PAIR0DEADTIME_SPEC>`"]
pub type PAIR0DEADTIME = crate::Reg<pair0deadtime::PAIR0DEADTIME_SPEC>;
#[doc = "Pair 0 Deadtime Configuration"]
pub mod pair0deadtime;
#[doc = "PAIR1DEADTIME register accessor: an alias for `Reg<PAIR1DEADTIME_SPEC>`"]
pub type PAIR1DEADTIME = crate::Reg<pair1deadtime::PAIR1DEADTIME_SPEC>;
#[doc = "Pair 1 Deadtime Configuration"]
pub mod pair1deadtime;
#[doc = "PAIR2DEADTIME register accessor: an alias for `Reg<PAIR2DEADTIME_SPEC>`"]
pub type PAIR2DEADTIME = crate::Reg<pair2deadtime::PAIR2DEADTIME_SPEC>;
#[doc = "Pair 2 Deadtime Configuration"]
pub mod pair2deadtime;
#[doc = "PAIR3DEADTIME register accessor: an alias for `Reg<PAIR3DEADTIME_SPEC>`"]
pub type PAIR3DEADTIME = crate::Reg<pair3deadtime::PAIR3DEADTIME_SPEC>;
#[doc = "Pair 3 Deadtime Configuration"]
pub mod pair3deadtime;
#[doc = "MOD_MIRROR register accessor: an alias for `Reg<MOD_MIRROR_SPEC>`"]
pub type MOD_MIRROR = crate::Reg<mod_mirror::MOD_MIRROR_SPEC>;
#[doc = "Mirror of Modulo Value"]
pub mod mod_mirror;
#[doc = "CV_MIRROR register accessor: an alias for `Reg<CV_MIRROR_SPEC>`"]
pub type CV_MIRROR = crate::Reg<cv_mirror::CV_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod cv_mirror;
