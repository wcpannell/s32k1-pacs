#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB DWT Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - MTB_DWT Comparator Register"]
    pub comp0: crate::Reg<comp::COMP_SPEC>,
    #[doc = "0x24 - MTB_DWT Comparator Mask Register"]
    pub mask0: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x28 - MTB_DWT Comparator Function Register 0"]
    pub fct0: crate::Reg<fct0::FCT0_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x30 - MTB_DWT Comparator Register"]
    pub comp1: crate::Reg<comp::COMP_SPEC>,
    #[doc = "0x34 - MTB_DWT Comparator Mask Register"]
    pub mask1: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x38 - MTB_DWT Comparator Function Register 1"]
    pub fct1: crate::Reg<fct1::FCT1_SPEC>,
    _reserved7: [u8; 0x01c4],
    #[doc = "0x200 - MTB_DWT Trace Buffer Control Register"]
    pub tbctrl: crate::Reg<tbctrl::TBCTRL_SPEC>,
    _reserved8: [u8; 0x0dc4],
    #[doc = "0xfc8 - Device Configuration Register"]
    pub devicecfg: crate::Reg<devicecfg::DEVICECFG_SPEC>,
    #[doc = "0xfcc - Device Type Identifier Register"]
    pub devicetypid: crate::Reg<devicetypid::DEVICETYPID_SPEC>,
    #[doc = "0xfd0 - Peripheral ID Register"]
    pub periphid4: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfd4 - Peripheral ID Register"]
    pub periphid5: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfd8 - Peripheral ID Register"]
    pub periphid6: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfdc - Peripheral ID Register"]
    pub periphid7: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe0 - Peripheral ID Register"]
    pub periphid0: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe4 - Peripheral ID Register"]
    pub periphid1: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfe8 - Peripheral ID Register"]
    pub periphid2: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xfec - Peripheral ID Register"]
    pub periphid3: crate::Reg<periphid::PERIPHID_SPEC>,
    #[doc = "0xff0..0x1000 - Component ID Register"]
    pub compid: [crate::Reg<compid::COMPID_SPEC>; 4],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MTB DWT Control Register"]
pub mod ctrl;
#[doc = "COMP register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "MTB_DWT Comparator Register"]
pub mod comp;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "MTB_DWT Comparator Mask Register"]
pub mod mask;
#[doc = "FCT0 register accessor: an alias for `Reg<FCT0_SPEC>`"]
pub type FCT0 = crate::Reg<fct0::FCT0_SPEC>;
#[doc = "MTB_DWT Comparator Function Register 0"]
pub mod fct0;
#[doc = "FCT1 register accessor: an alias for `Reg<FCT1_SPEC>`"]
pub type FCT1 = crate::Reg<fct1::FCT1_SPEC>;
#[doc = "MTB_DWT Comparator Function Register 1"]
pub mod fct1;
#[doc = "TBCTRL register accessor: an alias for `Reg<TBCTRL_SPEC>`"]
pub type TBCTRL = crate::Reg<tbctrl::TBCTRL_SPEC>;
#[doc = "MTB_DWT Trace Buffer Control Register"]
pub mod tbctrl;
#[doc = "DEVICECFG register accessor: an alias for `Reg<DEVICECFG_SPEC>`"]
pub type DEVICECFG = crate::Reg<devicecfg::DEVICECFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod devicecfg;
#[doc = "DEVICETYPID register accessor: an alias for `Reg<DEVICETYPID_SPEC>`"]
pub type DEVICETYPID = crate::Reg<devicetypid::DEVICETYPID_SPEC>;
#[doc = "Device Type Identifier Register"]
pub mod devicetypid;
#[doc = "PERIPHID register accessor: an alias for `Reg<PERIPHID_SPEC>`"]
pub type PERIPHID = crate::Reg<periphid::PERIPHID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "COMPID register accessor: an alias for `Reg<COMPID_SPEC>`"]
pub type COMPID = crate::Reg<compid::COMPID_SPEC>;
#[doc = "Component ID Register"]
pub mod compid;
