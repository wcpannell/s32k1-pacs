#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Modulo"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: crate::Reg<c0sc::C0SC_SPEC>,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: crate::Reg<c0v::C0V_SPEC>,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: crate::Reg<c1sc::C1SC_SPEC>,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: crate::Reg<c1v::C1V_SPEC>,
    #[doc = "0x1c - Channel (n) Status And Control"]
    pub c2sc: crate::Reg<c2sc::C2SC_SPEC>,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: crate::Reg<c2v::C2V_SPEC>,
    #[doc = "0x24 - Channel (n) Status And Control"]
    pub c3sc: crate::Reg<c3sc::C3SC_SPEC>,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: crate::Reg<c3v::C3V_SPEC>,
    #[doc = "0x2c - Channel (n) Status And Control"]
    pub c4sc: crate::Reg<c4sc::C4SC_SPEC>,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: crate::Reg<c4v::C4V_SPEC>,
    #[doc = "0x34 - Channel (n) Status And Control"]
    pub c5sc: crate::Reg<c5sc::C5SC_SPEC>,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: crate::Reg<c5v::C5V_SPEC>,
    #[doc = "0x3c - Channel (n) Status And Control"]
    pub c6sc: crate::Reg<c6sc::C6SC_SPEC>,
    #[doc = "0x40 - Channel (n) Value"]
    pub c6v: crate::Reg<c6v::C6V_SPEC>,
    #[doc = "0x44 - Channel (n) Status And Control"]
    pub c7sc: crate::Reg<c7sc::C7SC_SPEC>,
    #[doc = "0x48 - Channel (n) Value"]
    pub c7v: crate::Reg<c7v::C7V_SPEC>,
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
    _reserved41: [u8; 0x04],
    #[doc = "0xa8 - Pair 1 Deadtime Configuration"]
    pub pair1deadtime: crate::Reg<pair1deadtime::PAIR1DEADTIME_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0xb0 - Pair 2 Deadtime Configuration"]
    pub pair2deadtime: crate::Reg<pair2deadtime::PAIR2DEADTIME_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0xb8 - Pair 3 Deadtime Configuration"]
    pub pair3deadtime: crate::Reg<pair3deadtime::PAIR3DEADTIME_SPEC>,
    _reserved44: [u8; 0x0144],
    #[doc = "0x200 - Mirror of Modulo Value"]
    pub mod_mirror: crate::Reg<mod_mirror::MOD_MIRROR_SPEC>,
    #[doc = "0x204 - Mirror of Channel (n) Match Value"]
    pub c0v_mirror: crate::Reg<c0v_mirror::C0V_MIRROR_SPEC>,
    #[doc = "0x208 - Mirror of Channel (n) Match Value"]
    pub c1v_mirror: crate::Reg<c1v_mirror::C1V_MIRROR_SPEC>,
    #[doc = "0x20c - Mirror of Channel (n) Match Value"]
    pub c2v_mirror: crate::Reg<c2v_mirror::C2V_MIRROR_SPEC>,
    #[doc = "0x210 - Mirror of Channel (n) Match Value"]
    pub c3v_mirror: crate::Reg<c3v_mirror::C3V_MIRROR_SPEC>,
    #[doc = "0x214 - Mirror of Channel (n) Match Value"]
    pub c4v_mirror: crate::Reg<c4v_mirror::C4V_MIRROR_SPEC>,
    #[doc = "0x218 - Mirror of Channel (n) Match Value"]
    pub c5v_mirror: crate::Reg<c5v_mirror::C5V_MIRROR_SPEC>,
    #[doc = "0x21c - Mirror of Channel (n) Match Value"]
    pub c6v_mirror: crate::Reg<c6v_mirror::C6V_MIRROR_SPEC>,
    #[doc = "0x220 - Mirror of Channel (n) Match Value"]
    pub c7v_mirror: crate::Reg<c7v_mirror::C7V_MIRROR_SPEC>,
}
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
#[doc = "C0SC register accessor: an alias for `Reg<C0SC_SPEC>`"]
pub type C0SC = crate::Reg<c0sc::C0SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c0sc;
#[doc = "C0V register accessor: an alias for `Reg<C0V_SPEC>`"]
pub type C0V = crate::Reg<c0v::C0V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c0v;
#[doc = "C1SC register accessor: an alias for `Reg<C1SC_SPEC>`"]
pub type C1SC = crate::Reg<c1sc::C1SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c1sc;
#[doc = "C1V register accessor: an alias for `Reg<C1V_SPEC>`"]
pub type C1V = crate::Reg<c1v::C1V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c1v;
#[doc = "C2SC register accessor: an alias for `Reg<C2SC_SPEC>`"]
pub type C2SC = crate::Reg<c2sc::C2SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c2sc;
#[doc = "C2V register accessor: an alias for `Reg<C2V_SPEC>`"]
pub type C2V = crate::Reg<c2v::C2V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c2v;
#[doc = "C3SC register accessor: an alias for `Reg<C3SC_SPEC>`"]
pub type C3SC = crate::Reg<c3sc::C3SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c3sc;
#[doc = "C3V register accessor: an alias for `Reg<C3V_SPEC>`"]
pub type C3V = crate::Reg<c3v::C3V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c3v;
#[doc = "C4SC register accessor: an alias for `Reg<C4SC_SPEC>`"]
pub type C4SC = crate::Reg<c4sc::C4SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c4sc;
#[doc = "C4V register accessor: an alias for `Reg<C4V_SPEC>`"]
pub type C4V = crate::Reg<c4v::C4V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c4v;
#[doc = "C5SC register accessor: an alias for `Reg<C5SC_SPEC>`"]
pub type C5SC = crate::Reg<c5sc::C5SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c5sc;
#[doc = "C5V register accessor: an alias for `Reg<C5V_SPEC>`"]
pub type C5V = crate::Reg<c5v::C5V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c5v;
#[doc = "C6SC register accessor: an alias for `Reg<C6SC_SPEC>`"]
pub type C6SC = crate::Reg<c6sc::C6SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c6sc;
#[doc = "C6V register accessor: an alias for `Reg<C6V_SPEC>`"]
pub type C6V = crate::Reg<c6v::C6V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c6v;
#[doc = "C7SC register accessor: an alias for `Reg<C7SC_SPEC>`"]
pub type C7SC = crate::Reg<c7sc::C7SC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod c7sc;
#[doc = "C7V register accessor: an alias for `Reg<C7V_SPEC>`"]
pub type C7V = crate::Reg<c7v::C7V_SPEC>;
#[doc = "Channel (n) Value"]
pub mod c7v;
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
#[doc = "C0V_MIRROR register accessor: an alias for `Reg<C0V_MIRROR_SPEC>`"]
pub type C0V_MIRROR = crate::Reg<c0v_mirror::C0V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c0v_mirror;
#[doc = "C1V_MIRROR register accessor: an alias for `Reg<C1V_MIRROR_SPEC>`"]
pub type C1V_MIRROR = crate::Reg<c1v_mirror::C1V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c1v_mirror;
#[doc = "C2V_MIRROR register accessor: an alias for `Reg<C2V_MIRROR_SPEC>`"]
pub type C2V_MIRROR = crate::Reg<c2v_mirror::C2V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c2v_mirror;
#[doc = "C3V_MIRROR register accessor: an alias for `Reg<C3V_MIRROR_SPEC>`"]
pub type C3V_MIRROR = crate::Reg<c3v_mirror::C3V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c3v_mirror;
#[doc = "C4V_MIRROR register accessor: an alias for `Reg<C4V_MIRROR_SPEC>`"]
pub type C4V_MIRROR = crate::Reg<c4v_mirror::C4V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c4v_mirror;
#[doc = "C5V_MIRROR register accessor: an alias for `Reg<C5V_MIRROR_SPEC>`"]
pub type C5V_MIRROR = crate::Reg<c5v_mirror::C5V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c5v_mirror;
#[doc = "C6V_MIRROR register accessor: an alias for `Reg<C6V_MIRROR_SPEC>`"]
pub type C6V_MIRROR = crate::Reg<c6v_mirror::C6V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c6v_mirror;
#[doc = "C7V_MIRROR register accessor: an alias for `Reg<C7V_MIRROR_SPEC>`"]
pub type C7V_MIRROR = crate::Reg<c7v_mirror::C7V_MIRROR_SPEC>;
#[doc = "Mirror of Channel (n) Match Value"]
pub mod c7v_mirror;
