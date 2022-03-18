#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Set Enable Register"]
    pub s32_nvic_iser: crate::Reg<s32_nvic_iser::S32_NVIC_ISER_SPEC>,
    _reserved1: [u8; 0x7c],
    #[doc = "0x80 - Interrupt Clear Enable Register"]
    pub s32_nvic_icer: crate::Reg<s32_nvic_icer::S32_NVIC_ICER_SPEC>,
    _reserved2: [u8; 0x7c],
    #[doc = "0x100 - Interrupt Set Pending Register"]
    pub s32_nvic_ispr: crate::Reg<s32_nvic_ispr::S32_NVIC_ISPR_SPEC>,
    _reserved3: [u8; 0x7c],
    #[doc = "0x180 - Interrupt Clear Pending Register"]
    pub s32_nvic_icpr: crate::Reg<s32_nvic_icpr::S32_NVIC_ICPR_SPEC>,
    _reserved4: [u8; 0x017c],
    #[doc = "0x300 - Interrupt Priority Register n"]
    pub s32_nvic_ipr0: crate::Reg<s32_nvic_ipr0::S32_NVIC_IPR0_SPEC>,
    #[doc = "0x304 - Interrupt Priority Register n"]
    pub s32_nvic_ipr1: crate::Reg<s32_nvic_ipr1::S32_NVIC_IPR1_SPEC>,
    #[doc = "0x308 - Interrupt Priority Register n"]
    pub s32_nvic_ipr2: crate::Reg<s32_nvic_ipr2::S32_NVIC_IPR2_SPEC>,
    #[doc = "0x30c - Interrupt Priority Register n"]
    pub s32_nvic_ipr3: crate::Reg<s32_nvic_ipr3::S32_NVIC_IPR3_SPEC>,
    #[doc = "0x310 - Interrupt Priority Register n"]
    pub s32_nvic_ipr4: crate::Reg<s32_nvic_ipr4::S32_NVIC_IPR4_SPEC>,
    #[doc = "0x314 - Interrupt Priority Register n"]
    pub s32_nvic_ipr5: crate::Reg<s32_nvic_ipr5::S32_NVIC_IPR5_SPEC>,
    #[doc = "0x318 - Interrupt Priority Register n"]
    pub s32_nvic_ipr6: crate::Reg<s32_nvic_ipr6::S32_NVIC_IPR6_SPEC>,
    #[doc = "0x31c - Interrupt Priority Register n"]
    pub s32_nvic_ipr7: crate::Reg<s32_nvic_ipr7::S32_NVIC_IPR7_SPEC>,
}
#[doc = "S32_NVIC_ISER register accessor: an alias for `Reg<S32_NVIC_ISER_SPEC>`"]
pub type S32_NVIC_ISER = crate::Reg<s32_nvic_iser::S32_NVIC_ISER_SPEC>;
#[doc = "Interrupt Set Enable Register"]
pub mod s32_nvic_iser;
#[doc = "S32_NVIC_ICER register accessor: an alias for `Reg<S32_NVIC_ICER_SPEC>`"]
pub type S32_NVIC_ICER = crate::Reg<s32_nvic_icer::S32_NVIC_ICER_SPEC>;
#[doc = "Interrupt Clear Enable Register"]
pub mod s32_nvic_icer;
#[doc = "S32_NVIC_ISPR register accessor: an alias for `Reg<S32_NVIC_ISPR_SPEC>`"]
pub type S32_NVIC_ISPR = crate::Reg<s32_nvic_ispr::S32_NVIC_ISPR_SPEC>;
#[doc = "Interrupt Set Pending Register"]
pub mod s32_nvic_ispr;
#[doc = "S32_NVIC_ICPR register accessor: an alias for `Reg<S32_NVIC_ICPR_SPEC>`"]
pub type S32_NVIC_ICPR = crate::Reg<s32_nvic_icpr::S32_NVIC_ICPR_SPEC>;
#[doc = "Interrupt Clear Pending Register"]
pub mod s32_nvic_icpr;
#[doc = "S32_NVIC_IPR0 register accessor: an alias for `Reg<S32_NVIC_IPR0_SPEC>`"]
pub type S32_NVIC_IPR0 = crate::Reg<s32_nvic_ipr0::S32_NVIC_IPR0_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr0;
#[doc = "S32_NVIC_IPR1 register accessor: an alias for `Reg<S32_NVIC_IPR1_SPEC>`"]
pub type S32_NVIC_IPR1 = crate::Reg<s32_nvic_ipr1::S32_NVIC_IPR1_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr1;
#[doc = "S32_NVIC_IPR2 register accessor: an alias for `Reg<S32_NVIC_IPR2_SPEC>`"]
pub type S32_NVIC_IPR2 = crate::Reg<s32_nvic_ipr2::S32_NVIC_IPR2_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr2;
#[doc = "S32_NVIC_IPR3 register accessor: an alias for `Reg<S32_NVIC_IPR3_SPEC>`"]
pub type S32_NVIC_IPR3 = crate::Reg<s32_nvic_ipr3::S32_NVIC_IPR3_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr3;
#[doc = "S32_NVIC_IPR4 register accessor: an alias for `Reg<S32_NVIC_IPR4_SPEC>`"]
pub type S32_NVIC_IPR4 = crate::Reg<s32_nvic_ipr4::S32_NVIC_IPR4_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr4;
#[doc = "S32_NVIC_IPR5 register accessor: an alias for `Reg<S32_NVIC_IPR5_SPEC>`"]
pub type S32_NVIC_IPR5 = crate::Reg<s32_nvic_ipr5::S32_NVIC_IPR5_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr5;
#[doc = "S32_NVIC_IPR6 register accessor: an alias for `Reg<S32_NVIC_IPR6_SPEC>`"]
pub type S32_NVIC_IPR6 = crate::Reg<s32_nvic_ipr6::S32_NVIC_IPR6_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr6;
#[doc = "S32_NVIC_IPR7 register accessor: an alias for `Reg<S32_NVIC_IPR7_SPEC>`"]
pub type S32_NVIC_IPR7 = crate::Reg<s32_nvic_ipr7::S32_NVIC_IPR7_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod s32_nvic_ipr7;
