#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Chip Control register"]
    pub chipctl: crate::Reg<chipctl::CHIPCTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - FTM Option Register 0"]
    pub ftmopt0: crate::Reg<ftmopt0::FTMOPT0_SPEC>,
    #[doc = "0x10 - LPO Clock Select Register"]
    pub lpoclks: crate::Reg<lpoclks::LPOCLKS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - ADC Options Register"]
    pub adcopt: crate::Reg<adcopt::ADCOPT_SPEC>,
    #[doc = "0x1c - FTM Option Register 1"]
    pub ftmopt1: crate::Reg<ftmopt1::FTMOPT1_SPEC>,
    #[doc = "0x20 - Miscellaneous control register 0"]
    pub misctrl0: crate::Reg<misctrl0::MISCTRL0_SPEC>,
    #[doc = "0x24 - System Device Identification Register"]
    pub sdid: crate::Reg<sdid::SDID_SPEC>,
    _reserved7: [u8; 0x18],
    #[doc = "0x40 - Platform Clock Gating Control Register"]
    pub platcgc: crate::Reg<platcgc::PLATCGC_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x4c - Flash Configuration Register 1"]
    pub fcfg1: crate::Reg<fcfg1::FCFG1_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x54 - Unique Identification Register High"]
    pub uidh: crate::Reg<uidh::UIDH_SPEC>,
    #[doc = "0x58 - Unique Identification Register Mid-High"]
    pub uidmh: crate::Reg<uidmh::UIDMH_SPEC>,
    #[doc = "0x5c - Unique Identification Register Mid Low"]
    pub uidml: crate::Reg<uidml::UIDML_SPEC>,
    #[doc = "0x60 - Unique Identification Register Low"]
    pub uidl: crate::Reg<uidl::UIDL_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x68 - System Clock Divider Register 4"]
    pub clkdiv4: crate::Reg<clkdiv4::CLKDIV4_SPEC>,
    #[doc = "0x6c - Miscellaneous Control register 1"]
    pub misctrl1: crate::Reg<misctrl1::MISCTRL1_SPEC>,
}
#[doc = "CHIPCTL register accessor: an alias for `Reg<CHIPCTL_SPEC>`"]
pub type CHIPCTL = crate::Reg<chipctl::CHIPCTL_SPEC>;
#[doc = "Chip Control register"]
pub mod chipctl;
#[doc = "FTMOPT0 register accessor: an alias for `Reg<FTMOPT0_SPEC>`"]
pub type FTMOPT0 = crate::Reg<ftmopt0::FTMOPT0_SPEC>;
#[doc = "FTM Option Register 0"]
pub mod ftmopt0;
#[doc = "LPOCLKS register accessor: an alias for `Reg<LPOCLKS_SPEC>`"]
pub type LPOCLKS = crate::Reg<lpoclks::LPOCLKS_SPEC>;
#[doc = "LPO Clock Select Register"]
pub mod lpoclks;
#[doc = "ADCOPT register accessor: an alias for `Reg<ADCOPT_SPEC>`"]
pub type ADCOPT = crate::Reg<adcopt::ADCOPT_SPEC>;
#[doc = "ADC Options Register"]
pub mod adcopt;
#[doc = "FTMOPT1 register accessor: an alias for `Reg<FTMOPT1_SPEC>`"]
pub type FTMOPT1 = crate::Reg<ftmopt1::FTMOPT1_SPEC>;
#[doc = "FTM Option Register 1"]
pub mod ftmopt1;
#[doc = "MISCTRL0 register accessor: an alias for `Reg<MISCTRL0_SPEC>`"]
pub type MISCTRL0 = crate::Reg<misctrl0::MISCTRL0_SPEC>;
#[doc = "Miscellaneous control register 0"]
pub mod misctrl0;
#[doc = "SDID register accessor: an alias for `Reg<SDID_SPEC>`"]
pub type SDID = crate::Reg<sdid::SDID_SPEC>;
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "PLATCGC register accessor: an alias for `Reg<PLATCGC_SPEC>`"]
pub type PLATCGC = crate::Reg<platcgc::PLATCGC_SPEC>;
#[doc = "Platform Clock Gating Control Register"]
pub mod platcgc;
#[doc = "FCFG1 register accessor: an alias for `Reg<FCFG1_SPEC>`"]
pub type FCFG1 = crate::Reg<fcfg1::FCFG1_SPEC>;
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "UIDH register accessor: an alias for `Reg<UIDH_SPEC>`"]
pub type UIDH = crate::Reg<uidh::UIDH_SPEC>;
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "UIDMH register accessor: an alias for `Reg<UIDMH_SPEC>`"]
pub type UIDMH = crate::Reg<uidmh::UIDMH_SPEC>;
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "UIDML register accessor: an alias for `Reg<UIDML_SPEC>`"]
pub type UIDML = crate::Reg<uidml::UIDML_SPEC>;
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "UIDL register accessor: an alias for `Reg<UIDL_SPEC>`"]
pub type UIDL = crate::Reg<uidl::UIDL_SPEC>;
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "CLKDIV4 register accessor: an alias for `Reg<CLKDIV4_SPEC>`"]
pub type CLKDIV4 = crate::Reg<clkdiv4::CLKDIV4_SPEC>;
#[doc = "System Clock Divider Register 4"]
pub mod clkdiv4;
#[doc = "MISCTRL1 register accessor: an alias for `Reg<MISCTRL1_SPEC>`"]
pub type MISCTRL1 = crate::Reg<misctrl1::MISCTRL1_SPEC>;
#[doc = "Miscellaneous Control register 1"]
pub mod misctrl1;
