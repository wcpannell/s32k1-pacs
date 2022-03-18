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
    pub fccob3: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: crate::Reg<fprot::FPROT_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x16 - EEPROM Protection Register"]
    pub feprot: crate::Reg<feprot::FEPROT_SPEC>,
    #[doc = "0x17 - Data Flash Protection Register"]
    pub fdprot: crate::Reg<fdprot::FDPROT_SPEC>,
    _reserved22: [u8; 0x14],
    #[doc = "0x2c - Flash CSEc Status Register 1"]
    pub fcsestat1: crate::Reg<fcsestat1::FCSESTAT1_SPEC>,
    #[doc = "0x2d - Flash CSEc Status Register 0"]
    pub fcsestat0: crate::Reg<fcsestat0::FCSESTAT0_SPEC>,
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
#[doc = "FCCOB register accessor: an alias for `Reg<FCCOB_SPEC>`"]
pub type FCCOB = crate::Reg<fccob::FCCOB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
#[doc = "FEPROT register accessor: an alias for `Reg<FEPROT_SPEC>`"]
pub type FEPROT = crate::Reg<feprot::FEPROT_SPEC>;
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "FDPROT register accessor: an alias for `Reg<FDPROT_SPEC>`"]
pub type FDPROT = crate::Reg<fdprot::FDPROT_SPEC>;
#[doc = "Data Flash Protection Register"]
pub mod fdprot;
#[doc = "FCSESTAT1 register accessor: an alias for `Reg<FCSESTAT1_SPEC>`"]
pub type FCSESTAT1 = crate::Reg<fcsestat1::FCSESTAT1_SPEC>;
#[doc = "Flash CSEc Status Register 1"]
pub mod fcsestat1;
#[doc = "FCSESTAT0 register accessor: an alias for `Reg<FCSESTAT0_SPEC>`"]
pub type FCSESTAT0 = crate::Reg<fcsestat0::FCSESTAT0_SPEC>;
#[doc = "Flash CSEc Status Register 0"]
pub mod fcsestat0;
#[doc = "FERSTAT register accessor: an alias for `Reg<FERSTAT_SPEC>`"]
pub type FERSTAT = crate::Reg<ferstat::FERSTAT_SPEC>;
#[doc = "Flash Error Status Register"]
pub mod ferstat;
#[doc = "FERCNFG register accessor: an alias for `Reg<FERCNFG_SPEC>`"]
pub type FERCNFG = crate::Reg<fercnfg::FERCNFG_SPEC>;
#[doc = "Flash Error Configuration Register"]
pub mod fercnfg;
