#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ERM Configuration Register 0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - ERM Status Register 0"]
    pub sr0: crate::Reg<sr0::SR0_SPEC>,
    _reserved2: [u8; 0xec],
    #[doc = "0x100 - ERM Memory n Error Address Register"]
    pub ear0: crate::Reg<ear0::EAR0_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "ERM Configuration Register 0"]
pub mod cr0;
#[doc = "SR0 register accessor: an alias for `Reg<SR0_SPEC>`"]
pub type SR0 = crate::Reg<sr0::SR0_SPEC>;
#[doc = "ERM Status Register 0"]
pub mod sr0;
#[doc = "EAR0 register accessor: an alias for `Reg<EAR0_SPEC>`"]
pub type EAR0 = crate::Reg<ear0::EAR0_SPEC>;
#[doc = "ERM Memory n Error Address Register"]
pub mod ear0;
