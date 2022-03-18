#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub c0: crate::Reg<c0::C0_SPEC>,
    #[doc = "0x04 - CMP Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x08 - CMP Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
}
#[doc = "C0 register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "CMP Control Register 0"]
pub mod c0;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "CMP Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "CMP Control Register 2"]
pub mod c2;
