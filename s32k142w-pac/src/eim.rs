#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error Injection Module Configuration Register"]
    pub eimcr: crate::Reg<eimcr::EIMCR_SPEC>,
    #[doc = "0x04 - Error Injection Channel Enable register"]
    pub eichen: crate::Reg<eichen::EICHEN_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100..0x108 - no description available"]
    pub eichdn0: EICHDN,
    _reserved3: [u8; 0xf8],
    #[doc = "0x200..0x208 - no description available"]
    pub eichdn1: EICHDN,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EICHDN {
    #[doc = "0x00 - Error Injection Channel Descriptor n, Word0"]
    pub eichd_word0: crate::Reg<self::eichdn::eichd_word0::EICHD_WORD0_SPEC>,
    #[doc = "0x04 - Error Injection Channel Descriptor n, Word1"]
    pub eichd_word1: crate::Reg<self::eichdn::eichd_word1::EICHD_WORD1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod eichdn;
#[doc = "EIMCR register accessor: an alias for `Reg<EIMCR_SPEC>`"]
pub type EIMCR = crate::Reg<eimcr::EIMCR_SPEC>;
#[doc = "Error Injection Module Configuration Register"]
pub mod eimcr;
#[doc = "EICHEN register accessor: an alias for `Reg<EICHEN_SPEC>`"]
pub type EICHEN = crate::Reg<eichen::EICHEN_SPEC>;
#[doc = "Error Injection Channel Enable register"]
pub mod eichen;
