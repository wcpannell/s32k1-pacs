#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: crate::Reg<mpra::MPRA_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: crate::Reg<pacra::PACRA_SPEC>,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: crate::Reg<pacrb::PACRB_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: crate::Reg<pacrd::PACRD_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Register"]
    pub opacra: crate::Reg<opacra::OPACRA_SPEC>,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Register"]
    pub opacrb: crate::Reg<opacrb::OPACRB_SPEC>,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Register"]
    pub opacrc: crate::Reg<opacrc::OPACRC_SPEC>,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Register"]
    pub opacrd: crate::Reg<opacrd::OPACRD_SPEC>,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Register"]
    pub opacre: crate::Reg<opacre::OPACRE_SPEC>,
    #[doc = "0x54 - Off-Platform Peripheral Access Control Register"]
    pub opacrf: crate::Reg<opacrf::OPACRF_SPEC>,
    #[doc = "0x58 - Off-Platform Peripheral Access Control Register"]
    pub opacrg: crate::Reg<opacrg::OPACRG_SPEC>,
    #[doc = "0x5c - Off-Platform Peripheral Access Control Register"]
    pub opacrh: crate::Reg<opacrh::OPACRH_SPEC>,
    #[doc = "0x60 - Off-Platform Peripheral Access Control Register"]
    pub opacri: crate::Reg<opacri::OPACRI_SPEC>,
    #[doc = "0x64 - Off-Platform Peripheral Access Control Register"]
    pub opacrj: crate::Reg<opacrj::OPACRJ_SPEC>,
    #[doc = "0x68 - Off-Platform Peripheral Access Control Register"]
    pub opacrk: crate::Reg<opacrk::OPACRK_SPEC>,
    #[doc = "0x6c - Off-Platform Peripheral Access Control Register"]
    pub opacrl: crate::Reg<opacrl::OPACRL_SPEC>,
}
#[doc = "MPRA register accessor: an alias for `Reg<MPRA_SPEC>`"]
pub type MPRA = crate::Reg<mpra::MPRA_SPEC>;
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "PACRA register accessor: an alias for `Reg<PACRA_SPEC>`"]
pub type PACRA = crate::Reg<pacra::PACRA_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "PACRB register accessor: an alias for `Reg<PACRB_SPEC>`"]
pub type PACRB = crate::Reg<pacrb::PACRB_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "PACRD register accessor: an alias for `Reg<PACRD_SPEC>`"]
pub type PACRD = crate::Reg<pacrd::PACRD_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "OPACRA register accessor: an alias for `Reg<OPACRA_SPEC>`"]
pub type OPACRA = crate::Reg<opacra::OPACRA_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacra;
#[doc = "OPACRB register accessor: an alias for `Reg<OPACRB_SPEC>`"]
pub type OPACRB = crate::Reg<opacrb::OPACRB_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrb;
#[doc = "OPACRC register accessor: an alias for `Reg<OPACRC_SPEC>`"]
pub type OPACRC = crate::Reg<opacrc::OPACRC_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrc;
#[doc = "OPACRD register accessor: an alias for `Reg<OPACRD_SPEC>`"]
pub type OPACRD = crate::Reg<opacrd::OPACRD_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrd;
#[doc = "OPACRE register accessor: an alias for `Reg<OPACRE_SPEC>`"]
pub type OPACRE = crate::Reg<opacre::OPACRE_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacre;
#[doc = "OPACRF register accessor: an alias for `Reg<OPACRF_SPEC>`"]
pub type OPACRF = crate::Reg<opacrf::OPACRF_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrf;
#[doc = "OPACRG register accessor: an alias for `Reg<OPACRG_SPEC>`"]
pub type OPACRG = crate::Reg<opacrg::OPACRG_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrg;
#[doc = "OPACRH register accessor: an alias for `Reg<OPACRH_SPEC>`"]
pub type OPACRH = crate::Reg<opacrh::OPACRH_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrh;
#[doc = "OPACRI register accessor: an alias for `Reg<OPACRI_SPEC>`"]
pub type OPACRI = crate::Reg<opacri::OPACRI_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacri;
#[doc = "OPACRJ register accessor: an alias for `Reg<OPACRJ_SPEC>`"]
pub type OPACRJ = crate::Reg<opacrj::OPACRJ_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrj;
#[doc = "OPACRK register accessor: an alias for `Reg<OPACRK_SPEC>`"]
pub type OPACRK = crate::Reg<opacrk::OPACRK_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrk;
#[doc = "OPACRL register accessor: an alias for `Reg<OPACRL_SPEC>`"]
pub type OPACRL = crate::Reg<opacrl::OPACRL_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrl;
