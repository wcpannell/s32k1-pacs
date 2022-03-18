#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: crate::Reg<fcnfg::FCNFG_SPEC>,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: crate::Reg<fsec::FSEC_SPEC>,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: crate::Reg<fopt::FOPT_SPEC>,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: crate::Reg<fccob3::FCCOB3_SPEC>,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: crate::Reg<fccob2::FCCOB2_SPEC>,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: crate::Reg<fccob1::FCCOB1_SPEC>,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: crate::Reg<fccob0::FCCOB0_SPEC>,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: crate::Reg<fccob7::FCCOB7_SPEC>,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: crate::Reg<fccob6::FCCOB6_SPEC>,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: crate::Reg<fccob5::FCCOB5_SPEC>,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: crate::Reg<fccob4::FCCOB4_SPEC>,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: crate::Reg<fccobb::FCCOBB_SPEC>,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: crate::Reg<fccoba::FCCOBA_SPEC>,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: crate::Reg<fccob9::FCCOB9_SPEC>,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: crate::Reg<fccob8::FCCOB8_SPEC>,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: crate::Reg<fprot3::FPROT3_SPEC>,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: crate::Reg<fprot2::FPROT2_SPEC>,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: crate::Reg<fprot1::FPROT1_SPEC>,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: crate::Reg<fprot0::FPROT0_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x16 - EEPROM Protection Register"]
    pub feprot: crate::Reg<feprot::FEPROT_SPEC>,
    #[doc = "0x17 - Data Flash Protection Register"]
    pub fdprot: crate::Reg<fdprot::FDPROT_SPEC>,
    _reserved22: [u8; 0x14],
    #[doc = "0x2c - Flash CSEc Status Register"]
    pub fcsestat: crate::Reg<fcsestat::FCSESTAT_SPEC>,
    _reserved23: [u8; 0x01],
    #[doc = "0x2e - Flash Error Status Register"]
    pub ferstat: crate::Reg<ferstat::FERSTAT_SPEC>,
    #[doc = "0x2f - Flash Error Configuration Register"]
    pub fercnfg: crate::Reg<fercnfg::FERCNFG_SPEC>,
}
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB3 register accessor: an alias for `Reg<FCCOB3_SPEC>`"]
pub type FCCOB3 = crate::Reg<fccob3::FCCOB3_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob3;
#[doc = "FCCOB2 register accessor: an alias for `Reg<FCCOB2_SPEC>`"]
pub type FCCOB2 = crate::Reg<fccob2::FCCOB2_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob2;
#[doc = "FCCOB1 register accessor: an alias for `Reg<FCCOB1_SPEC>`"]
pub type FCCOB1 = crate::Reg<fccob1::FCCOB1_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob1;
#[doc = "FCCOB0 register accessor: an alias for `Reg<FCCOB0_SPEC>`"]
pub type FCCOB0 = crate::Reg<fccob0::FCCOB0_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob0;
#[doc = "FCCOB7 register accessor: an alias for `Reg<FCCOB7_SPEC>`"]
pub type FCCOB7 = crate::Reg<fccob7::FCCOB7_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob7;
#[doc = "FCCOB6 register accessor: an alias for `Reg<FCCOB6_SPEC>`"]
pub type FCCOB6 = crate::Reg<fccob6::FCCOB6_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob6;
#[doc = "FCCOB5 register accessor: an alias for `Reg<FCCOB5_SPEC>`"]
pub type FCCOB5 = crate::Reg<fccob5::FCCOB5_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob5;
#[doc = "FCCOB4 register accessor: an alias for `Reg<FCCOB4_SPEC>`"]
pub type FCCOB4 = crate::Reg<fccob4::FCCOB4_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob4;
#[doc = "FCCOBB register accessor: an alias for `Reg<FCCOBB_SPEC>`"]
pub type FCCOBB = crate::Reg<fccobb::FCCOBB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccobb;
#[doc = "FCCOBA register accessor: an alias for `Reg<FCCOBA_SPEC>`"]
pub type FCCOBA = crate::Reg<fccoba::FCCOBA_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccoba;
#[doc = "FCCOB9 register accessor: an alias for `Reg<FCCOB9_SPEC>`"]
pub type FCCOB9 = crate::Reg<fccob9::FCCOB9_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob9;
#[doc = "FCCOB8 register accessor: an alias for `Reg<FCCOB8_SPEC>`"]
pub type FCCOB8 = crate::Reg<fccob8::FCCOB8_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob8;
#[doc = "FPROT3 register accessor: an alias for `Reg<FPROT3_SPEC>`"]
pub type FPROT3 = crate::Reg<fprot3::FPROT3_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot3;
#[doc = "FPROT2 register accessor: an alias for `Reg<FPROT2_SPEC>`"]
pub type FPROT2 = crate::Reg<fprot2::FPROT2_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot2;
#[doc = "FPROT1 register accessor: an alias for `Reg<FPROT1_SPEC>`"]
pub type FPROT1 = crate::Reg<fprot1::FPROT1_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot1;
#[doc = "FPROT0 register accessor: an alias for `Reg<FPROT0_SPEC>`"]
pub type FPROT0 = crate::Reg<fprot0::FPROT0_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot0;
#[doc = "FEPROT register accessor: an alias for `Reg<FEPROT_SPEC>`"]
pub type FEPROT = crate::Reg<feprot::FEPROT_SPEC>;
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "FDPROT register accessor: an alias for `Reg<FDPROT_SPEC>`"]
pub type FDPROT = crate::Reg<fdprot::FDPROT_SPEC>;
#[doc = "Data Flash Protection Register"]
pub mod fdprot;
#[doc = "FCSESTAT register accessor: an alias for `Reg<FCSESTAT_SPEC>`"]
pub type FCSESTAT = crate::Reg<fcsestat::FCSESTAT_SPEC>;
#[doc = "Flash CSEc Status Register"]
pub mod fcsestat;
#[doc = "FERSTAT register accessor: an alias for `Reg<FERSTAT_SPEC>`"]
pub type FERSTAT = crate::Reg<ferstat::FERSTAT_SPEC>;
#[doc = "Flash Error Status Register"]
pub mod ferstat;
#[doc = "FERCNFG register accessor: an alias for `Reg<FERCNFG_SPEC>`"]
pub type FERCNFG = crate::Reg<fercnfg::FERCNFG_SPEC>;
#[doc = "Flash Error Configuration Register"]
pub mod fercnfg;
