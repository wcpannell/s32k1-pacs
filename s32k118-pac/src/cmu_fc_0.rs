#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Frequency Check Global Configuration Register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
    #[doc = "0x04 - CMU Frequency Check Reference Count Configuration Register"]
    pub rccr: crate::Reg<rccr::RCCR_SPEC>,
    #[doc = "0x08 - CMU Frequency Check High Threshold Configuration Register"]
    pub htcr: crate::Reg<htcr::HTCR_SPEC>,
    #[doc = "0x0c - CMU Frequency Check Low Threshold Configuration Register"]
    pub ltcr: crate::Reg<ltcr::LTCR_SPEC>,
    #[doc = "0x10 - CMU Frequency Check Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - CMU Frequency Check Interrupt/Event Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
}
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "CMU Frequency Check Global Configuration Register"]
pub mod gcr;
#[doc = "RCCR register accessor: an alias for `Reg<RCCR_SPEC>`"]
pub type RCCR = crate::Reg<rccr::RCCR_SPEC>;
#[doc = "CMU Frequency Check Reference Count Configuration Register"]
pub mod rccr;
#[doc = "HTCR register accessor: an alias for `Reg<HTCR_SPEC>`"]
pub type HTCR = crate::Reg<htcr::HTCR_SPEC>;
#[doc = "CMU Frequency Check High Threshold Configuration Register"]
pub mod htcr;
#[doc = "LTCR register accessor: an alias for `Reg<LTCR_SPEC>`"]
pub type LTCR = crate::Reg<ltcr::LTCR_SPEC>;
#[doc = "CMU Frequency Check Low Threshold Configuration Register"]
pub mod ltcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "CMU Frequency Check Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "CMU Frequency Check Interrupt/Event Enable Register"]
pub mod ier;
