#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Modulus register"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x08 - Counter register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0c - Interrupt Delay register"]
    pub idly: crate::Reg<idly::IDLY_SPEC>,
    #[doc = "0x10 - Channel n Control register 1"]
    pub ch0c1: crate::Reg<chc1::CHC1_SPEC>,
    #[doc = "0x14 - Channel n Status register"]
    pub ch0s: crate::Reg<chs::CHS_SPEC>,
    #[doc = "0x18 - Channel n Delay 0 register"]
    pub ch0dly0: crate::Reg<chdly0::CHDLY0_SPEC>,
    #[doc = "0x1c - Channel n Delay 1 register"]
    pub ch0dly1: crate::Reg<chdly1::CHDLY1_SPEC>,
    #[doc = "0x20 - Channel n Delay 2 register"]
    pub ch0dly2: crate::Reg<chdly2::CHDLY2_SPEC>,
    #[doc = "0x24 - Channel n Delay 3 register"]
    pub ch0dly3: crate::Reg<chdly3::CHDLY3_SPEC>,
    #[doc = "0x28 - Channel n Delay 4 register"]
    pub ch0dly4: crate::Reg<chdly4::CHDLY4_SPEC>,
    #[doc = "0x2c - Channel n Delay 5 register"]
    pub ch0dly5: crate::Reg<chdly5::CHDLY5_SPEC>,
    #[doc = "0x30 - Channel n Delay 6 register"]
    pub ch0dly6: crate::Reg<chdly6::CHDLY6_SPEC>,
    #[doc = "0x34 - Channel n Delay 7 register"]
    pub ch0dly7: crate::Reg<chdly7::CHDLY7_SPEC>,
    #[doc = "0x38 - Channel n Control register 1"]
    pub ch1c1: crate::Reg<chc1::CHC1_SPEC>,
    #[doc = "0x3c - Channel n Status register"]
    pub ch1s: crate::Reg<chs::CHS_SPEC>,
    #[doc = "0x40 - Channel n Delay 0 register"]
    pub ch1dly0: crate::Reg<chdly0::CHDLY0_SPEC>,
    #[doc = "0x44 - Channel n Delay 1 register"]
    pub ch1dly1: crate::Reg<chdly1::CHDLY1_SPEC>,
    #[doc = "0x48 - Channel n Delay 2 register"]
    pub ch1dly2: crate::Reg<chdly2::CHDLY2_SPEC>,
    #[doc = "0x4c - Channel n Delay 3 register"]
    pub ch1dly3: crate::Reg<chdly3::CHDLY3_SPEC>,
    #[doc = "0x50 - Channel n Delay 4 register"]
    pub ch1dly4: crate::Reg<chdly4::CHDLY4_SPEC>,
    #[doc = "0x54 - Channel n Delay 5 register"]
    pub ch1dly5: crate::Reg<chdly5::CHDLY5_SPEC>,
    #[doc = "0x58 - Channel n Delay 6 register"]
    pub ch1dly6: crate::Reg<chdly6::CHDLY6_SPEC>,
    #[doc = "0x5c - Channel n Delay 7 register"]
    pub ch1dly7: crate::Reg<chdly7::CHDLY7_SPEC>,
    #[doc = "0x60 - Channel n Control register 1"]
    pub ch2c1: crate::Reg<chc1::CHC1_SPEC>,
    #[doc = "0x64 - Channel n Status register"]
    pub ch2s: crate::Reg<chs::CHS_SPEC>,
    #[doc = "0x68 - Channel n Delay 0 register"]
    pub ch2dly0: crate::Reg<chdly0::CHDLY0_SPEC>,
    #[doc = "0x6c - Channel n Delay 1 register"]
    pub ch2dly1: crate::Reg<chdly1::CHDLY1_SPEC>,
    #[doc = "0x70 - Channel n Delay 2 register"]
    pub ch2dly2: crate::Reg<chdly2::CHDLY2_SPEC>,
    #[doc = "0x74 - Channel n Delay 3 register"]
    pub ch2dly3: crate::Reg<chdly3::CHDLY3_SPEC>,
    #[doc = "0x78 - Channel n Delay 4 register"]
    pub ch2dly4: crate::Reg<chdly4::CHDLY4_SPEC>,
    #[doc = "0x7c - Channel n Delay 5 register"]
    pub ch2dly5: crate::Reg<chdly5::CHDLY5_SPEC>,
    #[doc = "0x80 - Channel n Delay 6 register"]
    pub ch2dly6: crate::Reg<chdly6::CHDLY6_SPEC>,
    #[doc = "0x84 - Channel n Delay 7 register"]
    pub ch2dly7: crate::Reg<chdly7::CHDLY7_SPEC>,
    _reserved34: [u8; 0x0108],
    #[doc = "0x190 - Pulse-Out n Enable register"]
    pub poen: crate::Reg<poen::POEN_SPEC>,
    _reserved_35_dly1: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x194 - PDB0_DLY2 register."]
    #[inline(always)]
    pub fn pdb0_dly2(&self) -> &crate::Reg<pdb0_dly2::PDB0_DLY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(404usize)
                as *const crate::Reg<pdb0_dly2::PDB0_DLY2_SPEC>)
        }
    }
    #[doc = "0x194 - Pulse-Out n Delay register"]
    #[inline(always)]
    pub fn pdb0_podly(&self) -> &crate::Reg<pdb0_podly::PDB0_PODLY_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(404usize)
                as *const crate::Reg<pdb0_podly::PDB0_PODLY_SPEC>)
        }
    }
    #[doc = "0x196 - PDB0_DLY1 register."]
    #[inline(always)]
    pub fn dly1(&self) -> &crate::Reg<dly1::DLY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(406usize)
                as *const crate::Reg<dly1::DLY1_SPEC>)
        }
    }
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter register"]
pub mod cnt;
#[doc = "IDLY register accessor: an alias for `Reg<IDLY_SPEC>`"]
pub type IDLY = crate::Reg<idly::IDLY_SPEC>;
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "CHC1 register accessor: an alias for `Reg<CHC1_SPEC>`"]
pub type CHC1 = crate::Reg<chc1::CHC1_SPEC>;
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "CHS register accessor: an alias for `Reg<CHS_SPEC>`"]
pub type CHS = crate::Reg<chs::CHS_SPEC>;
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "CHDLY0 register accessor: an alias for `Reg<CHDLY0_SPEC>`"]
pub type CHDLY0 = crate::Reg<chdly0::CHDLY0_SPEC>;
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "CHDLY1 register accessor: an alias for `Reg<CHDLY1_SPEC>`"]
pub type CHDLY1 = crate::Reg<chdly1::CHDLY1_SPEC>;
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "CHDLY2 register accessor: an alias for `Reg<CHDLY2_SPEC>`"]
pub type CHDLY2 = crate::Reg<chdly2::CHDLY2_SPEC>;
#[doc = "Channel n Delay 2 register"]
pub mod chdly2;
#[doc = "CHDLY3 register accessor: an alias for `Reg<CHDLY3_SPEC>`"]
pub type CHDLY3 = crate::Reg<chdly3::CHDLY3_SPEC>;
#[doc = "Channel n Delay 3 register"]
pub mod chdly3;
#[doc = "CHDLY4 register accessor: an alias for `Reg<CHDLY4_SPEC>`"]
pub type CHDLY4 = crate::Reg<chdly4::CHDLY4_SPEC>;
#[doc = "Channel n Delay 4 register"]
pub mod chdly4;
#[doc = "CHDLY5 register accessor: an alias for `Reg<CHDLY5_SPEC>`"]
pub type CHDLY5 = crate::Reg<chdly5::CHDLY5_SPEC>;
#[doc = "Channel n Delay 5 register"]
pub mod chdly5;
#[doc = "CHDLY6 register accessor: an alias for `Reg<CHDLY6_SPEC>`"]
pub type CHDLY6 = crate::Reg<chdly6::CHDLY6_SPEC>;
#[doc = "Channel n Delay 6 register"]
pub mod chdly6;
#[doc = "CHDLY7 register accessor: an alias for `Reg<CHDLY7_SPEC>`"]
pub type CHDLY7 = crate::Reg<chdly7::CHDLY7_SPEC>;
#[doc = "Channel n Delay 7 register"]
pub mod chdly7;
#[doc = "POEN register accessor: an alias for `Reg<POEN_SPEC>`"]
pub type POEN = crate::Reg<poen::POEN_SPEC>;
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "PDB0_PODLY register accessor: an alias for `Reg<PDB0_PODLY_SPEC>`"]
pub type PDB0_PODLY = crate::Reg<pdb0_podly::PDB0_PODLY_SPEC>;
#[doc = "Pulse-Out n Delay register"]
pub mod pdb0_podly;
#[doc = "PDB0_DLY2 register accessor: an alias for `Reg<PDB0_DLY2_SPEC>`"]
pub type PDB0_DLY2 = crate::Reg<pdb0_dly2::PDB0_DLY2_SPEC>;
#[doc = "PDB0_DLY2 register."]
pub mod pdb0_dly2;
#[doc = "DLY1 register accessor: an alias for `Reg<DLY1_SPEC>`"]
pub type DLY1 = crate::Reg<dly1::DLY1_SPEC>;
#[doc = "PDB0_DLY1 register."]
pub mod dly1;
