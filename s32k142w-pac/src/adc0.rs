#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Register 1"]
    pub sc1a: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x04 - ADC Status and Control Register 1"]
    pub sc1b: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x08 - ADC Status and Control Register 1"]
    pub sc1c: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x0c - ADC Status and Control Register 1"]
    pub sc1d: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x10 - ADC Status and Control Register 1"]
    pub sc1e: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x14 - ADC Status and Control Register 1"]
    pub sc1f: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x18 - ADC Status and Control Register 1"]
    pub sc1g: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x1c - ADC Status and Control Register 1"]
    pub sc1h: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x20 - ADC Status and Control Register 1"]
    pub sc1i: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x24 - ADC Status and Control Register 1"]
    pub sc1j: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x28 - ADC Status and Control Register 1"]
    pub sc1k: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x2c - ADC Status and Control Register 1"]
    pub sc1l: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x30 - ADC Status and Control Register 1"]
    pub sc1m: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x34 - ADC Status and Control Register 1"]
    pub sc1n: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x38 - ADC Status and Control Register 1"]
    pub sc1o: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x3c - ADC Status and Control Register 1"]
    pub sc1p: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x40 - ADC Configuration Register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x44 - ADC Configuration Register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    #[doc = "0x48 - ADC Data Result Registers"]
    pub ra: crate::Reg<r::R_SPEC>,
    #[doc = "0x4c - ADC Data Result Registers"]
    pub rb: crate::Reg<r::R_SPEC>,
    #[doc = "0x50 - ADC Data Result Registers"]
    pub rc: crate::Reg<r::R_SPEC>,
    #[doc = "0x54 - ADC Data Result Registers"]
    pub rd: crate::Reg<r::R_SPEC>,
    #[doc = "0x58 - ADC Data Result Registers"]
    pub re: crate::Reg<r::R_SPEC>,
    #[doc = "0x5c - ADC Data Result Registers"]
    pub rf: crate::Reg<r::R_SPEC>,
    #[doc = "0x60 - ADC Data Result Registers"]
    pub rg: crate::Reg<r::R_SPEC>,
    #[doc = "0x64 - ADC Data Result Registers"]
    pub rh: crate::Reg<r::R_SPEC>,
    #[doc = "0x68 - ADC Data Result Registers"]
    pub ri: crate::Reg<r::R_SPEC>,
    #[doc = "0x6c - ADC Data Result Registers"]
    pub rj: crate::Reg<r::R_SPEC>,
    #[doc = "0x70 - ADC Data Result Registers"]
    pub rk: crate::Reg<r::R_SPEC>,
    #[doc = "0x74 - ADC Data Result Registers"]
    pub rl: crate::Reg<r::R_SPEC>,
    #[doc = "0x78 - ADC Data Result Registers"]
    pub rm: crate::Reg<r::R_SPEC>,
    #[doc = "0x7c - ADC Data Result Registers"]
    pub rn: crate::Reg<r::R_SPEC>,
    #[doc = "0x80 - ADC Data Result Registers"]
    pub ro: crate::Reg<r::R_SPEC>,
    #[doc = "0x84 - ADC Data Result Registers"]
    pub rp: crate::Reg<r::R_SPEC>,
    #[doc = "0x88 - Compare Value Registers"]
    pub cv1: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x8c - Compare Value Registers"]
    pub cv2: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x90 - Status and Control Register 2"]
    pub sc2: crate::Reg<sc2::SC2_SPEC>,
    #[doc = "0x94 - Status and Control Register 3"]
    pub sc3: crate::Reg<sc3::SC3_SPEC>,
    #[doc = "0x98 - BASE Offset Register"]
    pub base_ofs: crate::Reg<base_ofs::BASE_OFS_SPEC>,
    #[doc = "0x9c - ADC Offset Correction Register"]
    pub ofs: crate::Reg<ofs::OFS_SPEC>,
    #[doc = "0xa0 - USER Offset Correction Register"]
    pub usr_ofs: crate::Reg<usr_ofs::USR_OFS_SPEC>,
    #[doc = "0xa4 - ADC X Offset Correction Register"]
    pub xofs: crate::Reg<xofs::XOFS_SPEC>,
    #[doc = "0xa8 - ADC Y Offset Correction Register"]
    pub yofs: crate::Reg<yofs::YOFS_SPEC>,
    #[doc = "0xac - ADC Gain Register"]
    pub g: crate::Reg<g::G_SPEC>,
    #[doc = "0xb0 - ADC User Gain Register"]
    pub ug: crate::Reg<ug::UG_SPEC>,
    #[doc = "0xb4 - ADC General Calibration Value Register S"]
    pub clps: crate::Reg<clps::CLPS_SPEC>,
    #[doc = "0xb8 - ADC Plus-Side General Calibration Value Register 3"]
    pub clp3: crate::Reg<clp3::CLP3_SPEC>,
    #[doc = "0xbc - ADC Plus-Side General Calibration Value Register 2"]
    pub clp2: crate::Reg<clp2::CLP2_SPEC>,
    #[doc = "0xc0 - ADC Plus-Side General Calibration Value Register 1"]
    pub clp1: crate::Reg<clp1::CLP1_SPEC>,
    #[doc = "0xc4 - ADC Plus-Side General Calibration Value Register 0"]
    pub clp0: crate::Reg<clp0::CLP0_SPEC>,
    #[doc = "0xc8 - ADC Plus-Side General Calibration Value Register X"]
    pub clpx: crate::Reg<clpx::CLPX_SPEC>,
    #[doc = "0xcc - ADC Plus-Side General Calibration Value Register 9"]
    pub clp9: crate::Reg<clp9::CLP9_SPEC>,
    #[doc = "0xd0 - ADC General Calibration Offset Value Register S"]
    pub clps_ofs: crate::Reg<clps_ofs::CLPS_OFS_SPEC>,
    #[doc = "0xd4 - ADC Plus-Side General Calibration Offset Value Register 3"]
    pub clp3_ofs: crate::Reg<clp3_ofs::CLP3_OFS_SPEC>,
    #[doc = "0xd8 - ADC Plus-Side General Calibration Offset Value Register 2"]
    pub clp2_ofs: crate::Reg<clp2_ofs::CLP2_OFS_SPEC>,
    #[doc = "0xdc - ADC Plus-Side General Calibration Offset Value Register 1"]
    pub clp1_ofs: crate::Reg<clp1_ofs::CLP1_OFS_SPEC>,
    #[doc = "0xe0 - ADC Plus-Side General Calibration Offset Value Register 0"]
    pub clp0_ofs: crate::Reg<clp0_ofs::CLP0_OFS_SPEC>,
    #[doc = "0xe4 - ADC Plus-Side General Calibration Offset Value Register X"]
    pub clpx_ofs: crate::Reg<clpx_ofs::CLPX_OFS_SPEC>,
    #[doc = "0xe8 - ADC Plus-Side General Calibration Offset Value Register 9"]
    pub clp9_ofs: crate::Reg<clp9_ofs::CLP9_OFS_SPEC>,
}
#[doc = "SC1 register accessor: an alias for `Reg<SC1_SPEC>`"]
pub type SC1 = crate::Reg<sc1::SC1_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "R register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod r;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "SC2 register accessor: an alias for `Reg<SC2_SPEC>`"]
pub type SC2 = crate::Reg<sc2::SC2_SPEC>;
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "SC3 register accessor: an alias for `Reg<SC3_SPEC>`"]
pub type SC3 = crate::Reg<sc3::SC3_SPEC>;
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "BASE_OFS register accessor: an alias for `Reg<BASE_OFS_SPEC>`"]
pub type BASE_OFS = crate::Reg<base_ofs::BASE_OFS_SPEC>;
#[doc = "BASE Offset Register"]
pub mod base_ofs;
#[doc = "OFS register accessor: an alias for `Reg<OFS_SPEC>`"]
pub type OFS = crate::Reg<ofs::OFS_SPEC>;
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "USR_OFS register accessor: an alias for `Reg<USR_OFS_SPEC>`"]
pub type USR_OFS = crate::Reg<usr_ofs::USR_OFS_SPEC>;
#[doc = "USER Offset Correction Register"]
pub mod usr_ofs;
#[doc = "XOFS register accessor: an alias for `Reg<XOFS_SPEC>`"]
pub type XOFS = crate::Reg<xofs::XOFS_SPEC>;
#[doc = "ADC X Offset Correction Register"]
pub mod xofs;
#[doc = "YOFS register accessor: an alias for `Reg<YOFS_SPEC>`"]
pub type YOFS = crate::Reg<yofs::YOFS_SPEC>;
#[doc = "ADC Y Offset Correction Register"]
pub mod yofs;
#[doc = "G register accessor: an alias for `Reg<G_SPEC>`"]
pub type G = crate::Reg<g::G_SPEC>;
#[doc = "ADC Gain Register"]
pub mod g;
#[doc = "UG register accessor: an alias for `Reg<UG_SPEC>`"]
pub type UG = crate::Reg<ug::UG_SPEC>;
#[doc = "ADC User Gain Register"]
pub mod ug;
#[doc = "CLPS register accessor: an alias for `Reg<CLPS_SPEC>`"]
pub type CLPS = crate::Reg<clps::CLPS_SPEC>;
#[doc = "ADC General Calibration Value Register S"]
pub mod clps;
#[doc = "CLP3 register accessor: an alias for `Reg<CLP3_SPEC>`"]
pub type CLP3 = crate::Reg<clp3::CLP3_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register 3"]
pub mod clp3;
#[doc = "CLP2 register accessor: an alias for `Reg<CLP2_SPEC>`"]
pub type CLP2 = crate::Reg<clp2::CLP2_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register 2"]
pub mod clp2;
#[doc = "CLP1 register accessor: an alias for `Reg<CLP1_SPEC>`"]
pub type CLP1 = crate::Reg<clp1::CLP1_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register 1"]
pub mod clp1;
#[doc = "CLP0 register accessor: an alias for `Reg<CLP0_SPEC>`"]
pub type CLP0 = crate::Reg<clp0::CLP0_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register 0"]
pub mod clp0;
#[doc = "CLPX register accessor: an alias for `Reg<CLPX_SPEC>`"]
pub type CLPX = crate::Reg<clpx::CLPX_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register X"]
pub mod clpx;
#[doc = "CLP9 register accessor: an alias for `Reg<CLP9_SPEC>`"]
pub type CLP9 = crate::Reg<clp9::CLP9_SPEC>;
#[doc = "ADC Plus-Side General Calibration Value Register 9"]
pub mod clp9;
#[doc = "CLPS_OFS register accessor: an alias for `Reg<CLPS_OFS_SPEC>`"]
pub type CLPS_OFS = crate::Reg<clps_ofs::CLPS_OFS_SPEC>;
#[doc = "ADC General Calibration Offset Value Register S"]
pub mod clps_ofs;
#[doc = "CLP3_OFS register accessor: an alias for `Reg<CLP3_OFS_SPEC>`"]
pub type CLP3_OFS = crate::Reg<clp3_ofs::CLP3_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 3"]
pub mod clp3_ofs;
#[doc = "CLP2_OFS register accessor: an alias for `Reg<CLP2_OFS_SPEC>`"]
pub type CLP2_OFS = crate::Reg<clp2_ofs::CLP2_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2"]
pub mod clp2_ofs;
#[doc = "CLP1_OFS register accessor: an alias for `Reg<CLP1_OFS_SPEC>`"]
pub type CLP1_OFS = crate::Reg<clp1_ofs::CLP1_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1"]
pub mod clp1_ofs;
#[doc = "CLP0_OFS register accessor: an alias for `Reg<CLP0_OFS_SPEC>`"]
pub type CLP0_OFS = crate::Reg<clp0_ofs::CLP0_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 0"]
pub mod clp0_ofs;
#[doc = "CLPX_OFS register accessor: an alias for `Reg<CLPX_OFS_SPEC>`"]
pub type CLPX_OFS = crate::Reg<clpx_ofs::CLPX_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register X"]
pub mod clpx_ofs;
#[doc = "CLP9_OFS register accessor: an alias for `Reg<CLP9_OFS_SPEC>`"]
pub type CLP9_OFS = crate::Reg<clp9_ofs::CLP9_OFS_SPEC>;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 9"]
pub mod clp9_ofs;
