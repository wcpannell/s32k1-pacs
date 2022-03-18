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
    #[doc = "0x80 - Shifter Control N Register"]
    pub shiftctl0: crate::Reg<shiftctl0::SHIFTCTL0_SPEC>,
    #[doc = "0x84 - Shifter Control N Register"]
    pub shiftctl1: crate::Reg<shiftctl1::SHIFTCTL1_SPEC>,
    #[doc = "0x88 - Shifter Control N Register"]
    pub shiftctl2: crate::Reg<shiftctl2::SHIFTCTL2_SPEC>,
    #[doc = "0x8c - Shifter Control N Register"]
    pub shiftctl3: crate::Reg<shiftctl3::SHIFTCTL3_SPEC>,
    _reserved15: [u8; 0x70],
    #[doc = "0x100 - Shifter Configuration N Register"]
    pub shiftcfg0: crate::Reg<shiftcfg0::SHIFTCFG0_SPEC>,
    #[doc = "0x104 - Shifter Configuration N Register"]
    pub shiftcfg1: crate::Reg<shiftcfg1::SHIFTCFG1_SPEC>,
    #[doc = "0x108 - Shifter Configuration N Register"]
    pub shiftcfg2: crate::Reg<shiftcfg2::SHIFTCFG2_SPEC>,
    #[doc = "0x10c - Shifter Configuration N Register"]
    pub shiftcfg3: crate::Reg<shiftcfg3::SHIFTCFG3_SPEC>,
    _reserved19: [u8; 0xf0],
    #[doc = "0x200 - Shifter Buffer N Register"]
    pub shiftbuf0: crate::Reg<shiftbuf0::SHIFTBUF0_SPEC>,
    #[doc = "0x204 - Shifter Buffer N Register"]
    pub shiftbuf1: crate::Reg<shiftbuf1::SHIFTBUF1_SPEC>,
    #[doc = "0x208 - Shifter Buffer N Register"]
    pub shiftbuf2: crate::Reg<shiftbuf2::SHIFTBUF2_SPEC>,
    #[doc = "0x20c - Shifter Buffer N Register"]
    pub shiftbuf3: crate::Reg<shiftbuf3::SHIFTBUF3_SPEC>,
    _reserved23: [u8; 0x70],
    #[doc = "0x280 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis0: crate::Reg<shiftbufbis0::SHIFTBUFBIS0_SPEC>,
    #[doc = "0x284 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis1: crate::Reg<shiftbufbis1::SHIFTBUFBIS1_SPEC>,
    #[doc = "0x288 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis2: crate::Reg<shiftbufbis2::SHIFTBUFBIS2_SPEC>,
    #[doc = "0x28c - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis3: crate::Reg<shiftbufbis3::SHIFTBUFBIS3_SPEC>,
    _reserved27: [u8; 0x70],
    #[doc = "0x300 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys0: crate::Reg<shiftbufbys0::SHIFTBUFBYS0_SPEC>,
    #[doc = "0x304 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys1: crate::Reg<shiftbufbys1::SHIFTBUFBYS1_SPEC>,
    #[doc = "0x308 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys2: crate::Reg<shiftbufbys2::SHIFTBUFBYS2_SPEC>,
    #[doc = "0x30c - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys3: crate::Reg<shiftbufbys3::SHIFTBUFBYS3_SPEC>,
    _reserved31: [u8; 0x70],
    #[doc = "0x380 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs0: crate::Reg<shiftbufbbs0::SHIFTBUFBBS0_SPEC>,
    #[doc = "0x384 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs1: crate::Reg<shiftbufbbs1::SHIFTBUFBBS1_SPEC>,
    #[doc = "0x388 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs2: crate::Reg<shiftbufbbs2::SHIFTBUFBBS2_SPEC>,
    #[doc = "0x38c - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs3: crate::Reg<shiftbufbbs3::SHIFTBUFBBS3_SPEC>,
    _reserved35: [u8; 0x70],
    #[doc = "0x400 - Timer Control N Register"]
    pub timctl0: crate::Reg<timctl0::TIMCTL0_SPEC>,
    #[doc = "0x404 - Timer Control N Register"]
    pub timctl1: crate::Reg<timctl1::TIMCTL1_SPEC>,
    #[doc = "0x408 - Timer Control N Register"]
    pub timctl2: crate::Reg<timctl2::TIMCTL2_SPEC>,
    #[doc = "0x40c - Timer Control N Register"]
    pub timctl3: crate::Reg<timctl3::TIMCTL3_SPEC>,
    _reserved39: [u8; 0x70],
    #[doc = "0x480 - Timer Configuration N Register"]
    pub timcfg0: crate::Reg<timcfg0::TIMCFG0_SPEC>,
    #[doc = "0x484 - Timer Configuration N Register"]
    pub timcfg1: crate::Reg<timcfg1::TIMCFG1_SPEC>,
    #[doc = "0x488 - Timer Configuration N Register"]
    pub timcfg2: crate::Reg<timcfg2::TIMCFG2_SPEC>,
    #[doc = "0x48c - Timer Configuration N Register"]
    pub timcfg3: crate::Reg<timcfg3::TIMCFG3_SPEC>,
    _reserved43: [u8; 0x70],
    #[doc = "0x500 - Timer Compare N Register"]
    pub timcmp0: crate::Reg<timcmp0::TIMCMP0_SPEC>,
    #[doc = "0x504 - Timer Compare N Register"]
    pub timcmp1: crate::Reg<timcmp1::TIMCMP1_SPEC>,
    #[doc = "0x508 - Timer Compare N Register"]
    pub timcmp2: crate::Reg<timcmp2::TIMCMP2_SPEC>,
    #[doc = "0x50c - Timer Compare N Register"]
    pub timcmp3: crate::Reg<timcmp3::TIMCMP3_SPEC>,
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
#[doc = "SHIFTCTL0 register accessor: an alias for `Reg<SHIFTCTL0_SPEC>`"]
pub type SHIFTCTL0 = crate::Reg<shiftctl0::SHIFTCTL0_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl0;
#[doc = "SHIFTCTL1 register accessor: an alias for `Reg<SHIFTCTL1_SPEC>`"]
pub type SHIFTCTL1 = crate::Reg<shiftctl1::SHIFTCTL1_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl1;
#[doc = "SHIFTCTL2 register accessor: an alias for `Reg<SHIFTCTL2_SPEC>`"]
pub type SHIFTCTL2 = crate::Reg<shiftctl2::SHIFTCTL2_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl2;
#[doc = "SHIFTCTL3 register accessor: an alias for `Reg<SHIFTCTL3_SPEC>`"]
pub type SHIFTCTL3 = crate::Reg<shiftctl3::SHIFTCTL3_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl3;
#[doc = "SHIFTCFG0 register accessor: an alias for `Reg<SHIFTCFG0_SPEC>`"]
pub type SHIFTCFG0 = crate::Reg<shiftcfg0::SHIFTCFG0_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg0;
#[doc = "SHIFTCFG1 register accessor: an alias for `Reg<SHIFTCFG1_SPEC>`"]
pub type SHIFTCFG1 = crate::Reg<shiftcfg1::SHIFTCFG1_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg1;
#[doc = "SHIFTCFG2 register accessor: an alias for `Reg<SHIFTCFG2_SPEC>`"]
pub type SHIFTCFG2 = crate::Reg<shiftcfg2::SHIFTCFG2_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg2;
#[doc = "SHIFTCFG3 register accessor: an alias for `Reg<SHIFTCFG3_SPEC>`"]
pub type SHIFTCFG3 = crate::Reg<shiftcfg3::SHIFTCFG3_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg3;
#[doc = "SHIFTBUF0 register accessor: an alias for `Reg<SHIFTBUF0_SPEC>`"]
pub type SHIFTBUF0 = crate::Reg<shiftbuf0::SHIFTBUF0_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf0;
#[doc = "SHIFTBUF1 register accessor: an alias for `Reg<SHIFTBUF1_SPEC>`"]
pub type SHIFTBUF1 = crate::Reg<shiftbuf1::SHIFTBUF1_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf1;
#[doc = "SHIFTBUF2 register accessor: an alias for `Reg<SHIFTBUF2_SPEC>`"]
pub type SHIFTBUF2 = crate::Reg<shiftbuf2::SHIFTBUF2_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf2;
#[doc = "SHIFTBUF3 register accessor: an alias for `Reg<SHIFTBUF3_SPEC>`"]
pub type SHIFTBUF3 = crate::Reg<shiftbuf3::SHIFTBUF3_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf3;
#[doc = "SHIFTBUFBIS0 register accessor: an alias for `Reg<SHIFTBUFBIS0_SPEC>`"]
pub type SHIFTBUFBIS0 = crate::Reg<shiftbufbis0::SHIFTBUFBIS0_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis0;
#[doc = "SHIFTBUFBIS1 register accessor: an alias for `Reg<SHIFTBUFBIS1_SPEC>`"]
pub type SHIFTBUFBIS1 = crate::Reg<shiftbufbis1::SHIFTBUFBIS1_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis1;
#[doc = "SHIFTBUFBIS2 register accessor: an alias for `Reg<SHIFTBUFBIS2_SPEC>`"]
pub type SHIFTBUFBIS2 = crate::Reg<shiftbufbis2::SHIFTBUFBIS2_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis2;
#[doc = "SHIFTBUFBIS3 register accessor: an alias for `Reg<SHIFTBUFBIS3_SPEC>`"]
pub type SHIFTBUFBIS3 = crate::Reg<shiftbufbis3::SHIFTBUFBIS3_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis3;
#[doc = "SHIFTBUFBYS0 register accessor: an alias for `Reg<SHIFTBUFBYS0_SPEC>`"]
pub type SHIFTBUFBYS0 = crate::Reg<shiftbufbys0::SHIFTBUFBYS0_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys0;
#[doc = "SHIFTBUFBYS1 register accessor: an alias for `Reg<SHIFTBUFBYS1_SPEC>`"]
pub type SHIFTBUFBYS1 = crate::Reg<shiftbufbys1::SHIFTBUFBYS1_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys1;
#[doc = "SHIFTBUFBYS2 register accessor: an alias for `Reg<SHIFTBUFBYS2_SPEC>`"]
pub type SHIFTBUFBYS2 = crate::Reg<shiftbufbys2::SHIFTBUFBYS2_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys2;
#[doc = "SHIFTBUFBYS3 register accessor: an alias for `Reg<SHIFTBUFBYS3_SPEC>`"]
pub type SHIFTBUFBYS3 = crate::Reg<shiftbufbys3::SHIFTBUFBYS3_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys3;
#[doc = "SHIFTBUFBBS0 register accessor: an alias for `Reg<SHIFTBUFBBS0_SPEC>`"]
pub type SHIFTBUFBBS0 = crate::Reg<shiftbufbbs0::SHIFTBUFBBS0_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs0;
#[doc = "SHIFTBUFBBS1 register accessor: an alias for `Reg<SHIFTBUFBBS1_SPEC>`"]
pub type SHIFTBUFBBS1 = crate::Reg<shiftbufbbs1::SHIFTBUFBBS1_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs1;
#[doc = "SHIFTBUFBBS2 register accessor: an alias for `Reg<SHIFTBUFBBS2_SPEC>`"]
pub type SHIFTBUFBBS2 = crate::Reg<shiftbufbbs2::SHIFTBUFBBS2_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs2;
#[doc = "SHIFTBUFBBS3 register accessor: an alias for `Reg<SHIFTBUFBBS3_SPEC>`"]
pub type SHIFTBUFBBS3 = crate::Reg<shiftbufbbs3::SHIFTBUFBBS3_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs3;
#[doc = "TIMCTL0 register accessor: an alias for `Reg<TIMCTL0_SPEC>`"]
pub type TIMCTL0 = crate::Reg<timctl0::TIMCTL0_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl0;
#[doc = "TIMCTL1 register accessor: an alias for `Reg<TIMCTL1_SPEC>`"]
pub type TIMCTL1 = crate::Reg<timctl1::TIMCTL1_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl1;
#[doc = "TIMCTL2 register accessor: an alias for `Reg<TIMCTL2_SPEC>`"]
pub type TIMCTL2 = crate::Reg<timctl2::TIMCTL2_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl2;
#[doc = "TIMCTL3 register accessor: an alias for `Reg<TIMCTL3_SPEC>`"]
pub type TIMCTL3 = crate::Reg<timctl3::TIMCTL3_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl3;
#[doc = "TIMCFG0 register accessor: an alias for `Reg<TIMCFG0_SPEC>`"]
pub type TIMCFG0 = crate::Reg<timcfg0::TIMCFG0_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg0;
#[doc = "TIMCFG1 register accessor: an alias for `Reg<TIMCFG1_SPEC>`"]
pub type TIMCFG1 = crate::Reg<timcfg1::TIMCFG1_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg1;
#[doc = "TIMCFG2 register accessor: an alias for `Reg<TIMCFG2_SPEC>`"]
pub type TIMCFG2 = crate::Reg<timcfg2::TIMCFG2_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg2;
#[doc = "TIMCFG3 register accessor: an alias for `Reg<TIMCFG3_SPEC>`"]
pub type TIMCFG3 = crate::Reg<timcfg3::TIMCFG3_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg3;
#[doc = "TIMCMP0 register accessor: an alias for `Reg<TIMCMP0_SPEC>`"]
pub type TIMCMP0 = crate::Reg<timcmp0::TIMCMP0_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp0;
#[doc = "TIMCMP1 register accessor: an alias for `Reg<TIMCMP1_SPEC>`"]
pub type TIMCMP1 = crate::Reg<timcmp1::TIMCMP1_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp1;
#[doc = "TIMCMP2 register accessor: an alias for `Reg<TIMCMP2_SPEC>`"]
pub type TIMCMP2 = crate::Reg<timcmp2::TIMCMP2_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp2;
#[doc = "TIMCMP3 register accessor: an alias for `Reg<TIMCMP3_SPEC>`"]
pub type TIMCMP3 = crate::Reg<timcmp3::TIMCMP3_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp3;
