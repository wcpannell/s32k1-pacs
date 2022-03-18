#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Low Voltage Detect Status and Control 2 Register"]
    pub lvdsc2: crate::Reg<lvdsc2::LVDSC2_SPEC>,
    #[doc = "0x02 - Regulator Status and Control Register"]
    pub regsc: crate::Reg<regsc::REGSC_SPEC>,
    #[doc = "0x03 - Low Voltage Reset Flags Register"]
    pub lvrflg: crate::Reg<lvrflg::LVRFLG_SPEC>,
    #[doc = "0x04 - Low Power Oscillator Trim Register"]
    pub lpotrim: crate::Reg<lpotrim::LPOTRIM_SPEC>,
}
#[doc = "LVDSC2 register accessor: an alias for `Reg<LVDSC2_SPEC>`"]
pub type LVDSC2 = crate::Reg<lvdsc2::LVDSC2_SPEC>;
#[doc = "Low Voltage Detect Status and Control 2 Register"]
pub mod lvdsc2;
#[doc = "REGSC register accessor: an alias for `Reg<REGSC_SPEC>`"]
pub type REGSC = crate::Reg<regsc::REGSC_SPEC>;
#[doc = "Regulator Status and Control Register"]
pub mod regsc;
#[doc = "LVRFLG register accessor: an alias for `Reg<LVRFLG_SPEC>`"]
pub type LVRFLG = crate::Reg<lvrflg::LVRFLG_SPEC>;
#[doc = "Low Voltage Reset Flags Register"]
pub mod lvrflg;
#[doc = "LPOTRIM register accessor: an alias for `Reg<LPOTRIM_SPEC>`"]
pub type LPOTRIM = crate::Reg<lpotrim::LPOTRIM_SPEC>;
#[doc = "Low Power Oscillator Trim Register"]
pub mod lpotrim;
