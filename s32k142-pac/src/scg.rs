#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Clock Status Register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x14 - Run Clock Control Register"]
    pub rccr: crate::Reg<rccr::RCCR_SPEC>,
    #[doc = "0x18 - VLPR Clock Control Register"]
    pub vccr: crate::Reg<vccr::VCCR_SPEC>,
    #[doc = "0x1c - HSRUN Clock Control Register"]
    pub hccr: crate::Reg<hccr::HCCR_SPEC>,
    #[doc = "0x20 - SCG CLKOUT Configuration Register"]
    pub clkoutcnfg: crate::Reg<clkoutcnfg::CLKOUTCNFG_SPEC>,
    _reserved7: [u8; 0xdc],
    #[doc = "0x100 - System OSC Control Status Register"]
    pub sosccsr: crate::Reg<sosccsr::SOSCCSR_SPEC>,
    #[doc = "0x104 - System OSC Divide Register"]
    pub soscdiv: crate::Reg<soscdiv::SOSCDIV_SPEC>,
    #[doc = "0x108 - System Oscillator Configuration Register"]
    pub sosccfg: crate::Reg<sosccfg::SOSCCFG_SPEC>,
    _reserved10: [u8; 0xf4],
    #[doc = "0x200 - Slow IRC Control Status Register"]
    pub sirccsr: crate::Reg<sirccsr::SIRCCSR_SPEC>,
    #[doc = "0x204 - Slow IRC Divide Register"]
    pub sircdiv: crate::Reg<sircdiv::SIRCDIV_SPEC>,
    #[doc = "0x208 - Slow IRC Configuration Register"]
    pub sirccfg: crate::Reg<sirccfg::SIRCCFG_SPEC>,
    _reserved13: [u8; 0xf4],
    #[doc = "0x300 - Fast IRC Control Status Register"]
    pub firccsr: crate::Reg<firccsr::FIRCCSR_SPEC>,
    #[doc = "0x304 - Fast IRC Divide Register"]
    pub fircdiv: crate::Reg<fircdiv::FIRCDIV_SPEC>,
    #[doc = "0x308 - Fast IRC Configuration Register"]
    pub firccfg: crate::Reg<firccfg::FIRCCFG_SPEC>,
    _reserved16: [u8; 0x02f4],
    #[doc = "0x600 - System PLL Control Status Register"]
    pub spllcsr: crate::Reg<spllcsr::SPLLCSR_SPEC>,
    #[doc = "0x604 - System PLL Divide Register"]
    pub splldiv: crate::Reg<splldiv::SPLLDIV_SPEC>,
    #[doc = "0x608 - System PLL Configuration Register"]
    pub spllcfg: crate::Reg<spllcfg::SPLLCFG_SPEC>,
}
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Clock Status Register"]
pub mod csr;
#[doc = "RCCR register accessor: an alias for `Reg<RCCR_SPEC>`"]
pub type RCCR = crate::Reg<rccr::RCCR_SPEC>;
#[doc = "Run Clock Control Register"]
pub mod rccr;
#[doc = "VCCR register accessor: an alias for `Reg<VCCR_SPEC>`"]
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
#[doc = "VLPR Clock Control Register"]
pub mod vccr;
#[doc = "HCCR register accessor: an alias for `Reg<HCCR_SPEC>`"]
pub type HCCR = crate::Reg<hccr::HCCR_SPEC>;
#[doc = "HSRUN Clock Control Register"]
pub mod hccr;
#[doc = "CLKOUTCNFG register accessor: an alias for `Reg<CLKOUTCNFG_SPEC>`"]
pub type CLKOUTCNFG = crate::Reg<clkoutcnfg::CLKOUTCNFG_SPEC>;
#[doc = "SCG CLKOUT Configuration Register"]
pub mod clkoutcnfg;
#[doc = "SOSCCSR register accessor: an alias for `Reg<SOSCCSR_SPEC>`"]
pub type SOSCCSR = crate::Reg<sosccsr::SOSCCSR_SPEC>;
#[doc = "System OSC Control Status Register"]
pub mod sosccsr;
#[doc = "SOSCDIV register accessor: an alias for `Reg<SOSCDIV_SPEC>`"]
pub type SOSCDIV = crate::Reg<soscdiv::SOSCDIV_SPEC>;
#[doc = "System OSC Divide Register"]
pub mod soscdiv;
#[doc = "SOSCCFG register accessor: an alias for `Reg<SOSCCFG_SPEC>`"]
pub type SOSCCFG = crate::Reg<sosccfg::SOSCCFG_SPEC>;
#[doc = "System Oscillator Configuration Register"]
pub mod sosccfg;
#[doc = "SIRCCSR register accessor: an alias for `Reg<SIRCCSR_SPEC>`"]
pub type SIRCCSR = crate::Reg<sirccsr::SIRCCSR_SPEC>;
#[doc = "Slow IRC Control Status Register"]
pub mod sirccsr;
#[doc = "SIRCDIV register accessor: an alias for `Reg<SIRCDIV_SPEC>`"]
pub type SIRCDIV = crate::Reg<sircdiv::SIRCDIV_SPEC>;
#[doc = "Slow IRC Divide Register"]
pub mod sircdiv;
#[doc = "SIRCCFG register accessor: an alias for `Reg<SIRCCFG_SPEC>`"]
pub type SIRCCFG = crate::Reg<sirccfg::SIRCCFG_SPEC>;
#[doc = "Slow IRC Configuration Register"]
pub mod sirccfg;
#[doc = "FIRCCSR register accessor: an alias for `Reg<FIRCCSR_SPEC>`"]
pub type FIRCCSR = crate::Reg<firccsr::FIRCCSR_SPEC>;
#[doc = "Fast IRC Control Status Register"]
pub mod firccsr;
#[doc = "FIRCDIV register accessor: an alias for `Reg<FIRCDIV_SPEC>`"]
pub type FIRCDIV = crate::Reg<fircdiv::FIRCDIV_SPEC>;
#[doc = "Fast IRC Divide Register"]
pub mod fircdiv;
#[doc = "FIRCCFG register accessor: an alias for `Reg<FIRCCFG_SPEC>`"]
pub type FIRCCFG = crate::Reg<firccfg::FIRCCFG_SPEC>;
#[doc = "Fast IRC Configuration Register"]
pub mod firccfg;
#[doc = "SPLLCSR register accessor: an alias for `Reg<SPLLCSR_SPEC>`"]
pub type SPLLCSR = crate::Reg<spllcsr::SPLLCSR_SPEC>;
#[doc = "System PLL Control Status Register"]
pub mod spllcsr;
#[doc = "SPLLDIV register accessor: an alias for `Reg<SPLLDIV_SPEC>`"]
pub type SPLLDIV = crate::Reg<splldiv::SPLLDIV_SPEC>;
#[doc = "System PLL Divide Register"]
pub mod splldiv;
#[doc = "SPLLCFG register accessor: an alias for `Reg<SPLLCFG_SPEC>`"]
pub type SPLLCFG = crate::Reg<spllcfg::SPLLCFG_SPEC>;
#[doc = "System PLL Configuration Register"]
pub mod spllcfg;
