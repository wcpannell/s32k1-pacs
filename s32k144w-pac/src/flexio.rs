#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Pin State Register"]
    pub pin: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: crate::Reg<shiftstat::SHIFTSTAT_SPEC>,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: crate::Reg<shifterr::SHIFTERR_SPEC>,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: crate::Reg<timstat::TIMSTAT_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: crate::Reg<shiftsien::SHIFTSIEN_SPEC>,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: crate::Reg<shifteien::SHIFTEIEN_SPEC>,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: crate::Reg<timien::TIMIEN_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: crate::Reg<shiftsden::SHIFTSDEN_SPEC>,
    _reserved11: [u8; 0x4c],
    #[doc = "0x80..0x90 - Shifter Control N Register"]
    pub shiftctl: [crate::Reg<shiftctl::SHIFTCTL_SPEC>; 4],
    _reserved12: [u8; 0x70],
    #[doc = "0x100..0x110 - Shifter Configuration N Register"]
    pub shiftcfg: [crate::Reg<shiftcfg::SHIFTCFG_SPEC>; 4],
    _reserved13: [u8; 0xf0],
    #[doc = "0x200..0x210 - Shifter Buffer N Register"]
    pub shiftbuf: [crate::Reg<shiftbuf::SHIFTBUF_SPEC>; 4],
    _reserved14: [u8; 0x70],
    #[doc = "0x280..0x290 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis: [crate::Reg<shiftbufbis::SHIFTBUFBIS_SPEC>; 4],
    _reserved15: [u8; 0x70],
    #[doc = "0x300..0x310 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys: [crate::Reg<shiftbufbys::SHIFTBUFBYS_SPEC>; 4],
    _reserved16: [u8; 0x70],
    #[doc = "0x380..0x390 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs: [crate::Reg<shiftbufbbs::SHIFTBUFBBS_SPEC>; 4],
    _reserved17: [u8; 0x70],
    #[doc = "0x400..0x410 - Timer Control N Register"]
    pub timctl: [crate::Reg<timctl::TIMCTL_SPEC>; 4],
    _reserved18: [u8; 0x70],
    #[doc = "0x480..0x490 - Timer Configuration N Register"]
    pub timcfg: [crate::Reg<timcfg::TIMCFG_SPEC>; 4],
    _reserved19: [u8; 0x70],
    #[doc = "0x500..0x510 - Timer Compare N Register"]
    pub timcmp: [crate::Reg<timcmp::TIMCMP_SPEC>; 4],
}
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "SHIFTSTAT register accessor: an alias for `Reg<SHIFTSTAT_SPEC>`"]
pub type SHIFTSTAT = crate::Reg<shiftstat::SHIFTSTAT_SPEC>;
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "SHIFTERR register accessor: an alias for `Reg<SHIFTERR_SPEC>`"]
pub type SHIFTERR = crate::Reg<shifterr::SHIFTERR_SPEC>;
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "TIMSTAT register accessor: an alias for `Reg<TIMSTAT_SPEC>`"]
pub type TIMSTAT = crate::Reg<timstat::TIMSTAT_SPEC>;
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "SHIFTSIEN register accessor: an alias for `Reg<SHIFTSIEN_SPEC>`"]
pub type SHIFTSIEN = crate::Reg<shiftsien::SHIFTSIEN_SPEC>;
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "SHIFTEIEN register accessor: an alias for `Reg<SHIFTEIEN_SPEC>`"]
pub type SHIFTEIEN = crate::Reg<shifteien::SHIFTEIEN_SPEC>;
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "TIMIEN register accessor: an alias for `Reg<TIMIEN_SPEC>`"]
pub type TIMIEN = crate::Reg<timien::TIMIEN_SPEC>;
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "SHIFTSDEN register accessor: an alias for `Reg<SHIFTSDEN_SPEC>`"]
pub type SHIFTSDEN = crate::Reg<shiftsden::SHIFTSDEN_SPEC>;
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "SHIFTCTL register accessor: an alias for `Reg<SHIFTCTL_SPEC>`"]
pub type SHIFTCTL = crate::Reg<shiftctl::SHIFTCTL_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "SHIFTCFG register accessor: an alias for `Reg<SHIFTCFG_SPEC>`"]
pub type SHIFTCFG = crate::Reg<shiftcfg::SHIFTCFG_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "SHIFTBUF register accessor: an alias for `Reg<SHIFTBUF_SPEC>`"]
pub type SHIFTBUF = crate::Reg<shiftbuf::SHIFTBUF_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "SHIFTBUFBIS register accessor: an alias for `Reg<SHIFTBUFBIS_SPEC>`"]
pub type SHIFTBUFBIS = crate::Reg<shiftbufbis::SHIFTBUFBIS_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "SHIFTBUFBYS register accessor: an alias for `Reg<SHIFTBUFBYS_SPEC>`"]
pub type SHIFTBUFBYS = crate::Reg<shiftbufbys::SHIFTBUFBYS_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "SHIFTBUFBBS register accessor: an alias for `Reg<SHIFTBUFBBS_SPEC>`"]
pub type SHIFTBUFBBS = crate::Reg<shiftbufbbs::SHIFTBUFBBS_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "TIMCTL register accessor: an alias for `Reg<TIMCTL_SPEC>`"]
pub type TIMCTL = crate::Reg<timctl::TIMCTL_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "TIMCFG register accessor: an alias for `Reg<TIMCFG_SPEC>`"]
pub type TIMCFG = crate::Reg<timcfg::TIMCFG_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "TIMCMP register accessor: an alias for `Reg<TIMCMP_SPEC>`"]
pub type TIMCMP = crate::Reg<timcmp::TIMCMP_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp;
