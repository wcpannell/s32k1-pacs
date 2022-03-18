#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_datahu: [u8; 0x04],
    #[doc = "0x04 - CRC Polynomial register"]
    pub gpoly: crate::Reg<gpoly::GPOLY_SPEC>,
    #[doc = "0x08 - CRC Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn crc_datall(&self) -> &crate::Reg<crc_datall::CRC_DATALL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_datall::CRC_DATALL_SPEC>)
        }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn crc_datal(&self) -> &crate::Reg<crc_datal::CRC_DATAL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_datal::CRC_DATAL_SPEC>)
        }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn crc_data(&self) -> &crate::Reg<crc_data::CRC_DATA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_data::CRC_DATA_SPEC>)
        }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu(&self) -> &crate::Reg<datalu::DATALU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1usize)
                as *const crate::Reg<datalu::DATALU_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn crc_datahl(&self) -> &crate::Reg<crc_datahl::CRC_DATAHL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_datahl::CRC_DATAHL_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn crc_datah(&self) -> &crate::Reg<crc_datah::CRC_DATAH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_datah::CRC_DATAH_SPEC>)
        }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu(&self) -> &crate::Reg<datahu::DATAHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3usize)
                as *const crate::Reg<datahu::DATAHU_SPEC>)
        }
    }
}
#[doc = "CRC_DATA register accessor: an alias for `Reg<CRC_DATA_SPEC>`"]
pub type CRC_DATA = crate::Reg<crc_data::CRC_DATA_SPEC>;
#[doc = "CRC Data register"]
pub mod crc_data;
#[doc = "CRC_DATAL register accessor: an alias for `Reg<CRC_DATAL_SPEC>`"]
pub type CRC_DATAL = crate::Reg<crc_datal::CRC_DATAL_SPEC>;
#[doc = "CRC_DATAL register."]
pub mod crc_datal;
#[doc = "CRC_DATALL register accessor: an alias for `Reg<CRC_DATALL_SPEC>`"]
pub type CRC_DATALL = crate::Reg<crc_datall::CRC_DATALL_SPEC>;
#[doc = "CRC_DATALL register."]
pub mod crc_datall;
#[doc = "DATALU register accessor: an alias for `Reg<DATALU_SPEC>`"]
pub type DATALU = crate::Reg<datalu::DATALU_SPEC>;
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAH register accessor: an alias for `Reg<CRC_DATAH_SPEC>`"]
pub type CRC_DATAH = crate::Reg<crc_datah::CRC_DATAH_SPEC>;
#[doc = "CRC_DATAH register."]
pub mod crc_datah;
#[doc = "CRC_DATAHL register accessor: an alias for `Reg<CRC_DATAHL_SPEC>`"]
pub type CRC_DATAHL = crate::Reg<crc_datahl::CRC_DATAHL_SPEC>;
#[doc = "CRC_DATAHL register."]
pub mod crc_datahl;
#[doc = "DATAHU register accessor: an alias for `Reg<DATAHU_SPEC>`"]
pub type DATAHU = crate::Reg<datahu::DATAHU_SPEC>;
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "GPOLY register accessor: an alias for `Reg<GPOLY_SPEC>`"]
pub type GPOLY = crate::Reg<gpoly::GPOLY_SPEC>;
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control register"]
pub mod ctrl;
