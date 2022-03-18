#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register"]
    pub cs: crate::Reg<cs::CS_SPEC>,
    #[doc = "0x04 - Watchdog Counter Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Watchdog Timeout Value Register"]
    pub toval: crate::Reg<toval::TOVAL_SPEC>,
    #[doc = "0x0c - Watchdog Window Register"]
    pub win: crate::Reg<win::WIN_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Watchdog Control and Status Register"]
pub mod cs;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Watchdog Counter Register"]
pub mod cnt;
#[doc = "TOVAL register accessor: an alias for `Reg<TOVAL_SPEC>`"]
pub type TOVAL = crate::Reg<toval::TOVAL_SPEC>;
#[doc = "Watchdog Timeout Value Register"]
pub mod toval;
#[doc = "WIN register accessor: an alias for `Reg<WIN_SPEC>`"]
pub type WIN = crate::Reg<win::WIN_SPEC>;
#[doc = "Watchdog Window Register"]
pub mod win;
