#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error Injection Module Configuration Register"]
    pub eimcr: crate::Reg<eimcr::EIMCR_SPEC>,
    #[doc = "0x04 - Error Injection Channel Enable register"]
    pub eichen: crate::Reg<eichen::EICHEN_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Error Injection Channel Descriptor n, Word0"]
    pub eichd0_word0: crate::Reg<eichd0_word0::EICHD0_WORD0_SPEC>,
    #[doc = "0x104 - Error Injection Channel Descriptor n, Word1"]
    pub eichd0_word1: crate::Reg<eichd0_word1::EICHD0_WORD1_SPEC>,
    _reserved4: [u8; 0xf8],
    #[doc = "0x200 - Error Injection Channel Descriptor n, Word0"]
    pub eichd1_word0: crate::Reg<eichd1_word0::EICHD1_WORD0_SPEC>,
    #[doc = "0x204 - Error Injection Channel Descriptor n, Word1"]
    pub eichd1_word1: crate::Reg<eichd1_word1::EICHD1_WORD1_SPEC>,
}
#[doc = "EIMCR register accessor: an alias for `Reg<EIMCR_SPEC>`"]
pub type EIMCR = crate::Reg<eimcr::EIMCR_SPEC>;
#[doc = "Error Injection Module Configuration Register"]
pub mod eimcr;
#[doc = "EICHEN register accessor: an alias for `Reg<EICHEN_SPEC>`"]
pub type EICHEN = crate::Reg<eichen::EICHEN_SPEC>;
#[doc = "Error Injection Channel Enable register"]
pub mod eichen;
#[doc = "EICHD0_WORD0 register accessor: an alias for `Reg<EICHD0_WORD0_SPEC>`"]
pub type EICHD0_WORD0 = crate::Reg<eichd0_word0::EICHD0_WORD0_SPEC>;
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd0_word0;
#[doc = "EICHD0_WORD1 register accessor: an alias for `Reg<EICHD0_WORD1_SPEC>`"]
pub type EICHD0_WORD1 = crate::Reg<eichd0_word1::EICHD0_WORD1_SPEC>;
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd0_word1;
#[doc = "EICHD1_WORD0 register accessor: an alias for `Reg<EICHD1_WORD0_SPEC>`"]
pub type EICHD1_WORD0 = crate::Reg<eichd1_word0::EICHD1_WORD0_SPEC>;
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd1_word0;
#[doc = "EICHD1_WORD1 register accessor: an alias for `Reg<EICHD1_WORD1_SPEC>`"]
pub type EICHD1_WORD1 = crate::Reg<eichd1_word1::EICHD1_WORD1_SPEC>;
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd1_word1;
