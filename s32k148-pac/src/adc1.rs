#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Register 1"]
    pub sc1a: crate::Reg<sc1a::SC1A_SPEC>,
    #[doc = "0x04 - ADC Status and Control Register 1"]
    pub sc1b: crate::Reg<sc1b::SC1B_SPEC>,
    #[doc = "0x08 - ADC Status and Control Register 1"]
    pub sc1c: crate::Reg<sc1c::SC1C_SPEC>,
    #[doc = "0x0c - ADC Status and Control Register 1"]
    pub sc1d: crate::Reg<sc1d::SC1D_SPEC>,
    #[doc = "0x10 - ADC Status and Control Register 1"]
    pub sc1e: crate::Reg<sc1e::SC1E_SPEC>,
    #[doc = "0x14 - ADC Status and Control Register 1"]
    pub sc1f: crate::Reg<sc1f::SC1F_SPEC>,
    #[doc = "0x18 - ADC Status and Control Register 1"]
    pub sc1g: crate::Reg<sc1g::SC1G_SPEC>,
    #[doc = "0x1c - ADC Status and Control Register 1"]
    pub sc1h: crate::Reg<sc1h::SC1H_SPEC>,
    #[doc = "0x20 - ADC Status and Control Register 1"]
    pub sc1i: crate::Reg<sc1i::SC1I_SPEC>,
    #[doc = "0x24 - ADC Status and Control Register 1"]
    pub sc1j: crate::Reg<sc1j::SC1J_SPEC>,
    #[doc = "0x28 - ADC Status and Control Register 1"]
    pub sc1k: crate::Reg<sc1k::SC1K_SPEC>,
    #[doc = "0x2c - ADC Status and Control Register 1"]
    pub sc1l: crate::Reg<sc1l::SC1L_SPEC>,
    #[doc = "0x30 - ADC Status and Control Register 1"]
    pub sc1m: crate::Reg<sc1m::SC1M_SPEC>,
    #[doc = "0x34 - ADC Status and Control Register 1"]
    pub sc1n: crate::Reg<sc1n::SC1N_SPEC>,
    #[doc = "0x38 - ADC Status and Control Register 1"]
    pub sc1o: crate::Reg<sc1o::SC1O_SPEC>,
    #[doc = "0x3c - ADC Status and Control Register 1"]
    pub sc1p: crate::Reg<sc1p::SC1P_SPEC>,
    #[doc = "0x40 - ADC Configuration Register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x44 - ADC Configuration Register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    #[doc = "0x48 - ADC Data Result Registers"]
    pub ra: crate::Reg<ra::RA_SPEC>,
    #[doc = "0x4c - ADC Data Result Registers"]
    pub rb: crate::Reg<rb::RB_SPEC>,
    #[doc = "0x50 - ADC Data Result Registers"]
    pub rc: crate::Reg<rc::RC_SPEC>,
    #[doc = "0x54 - ADC Data Result Registers"]
    pub rd: crate::Reg<rd::RD_SPEC>,
    #[doc = "0x58 - ADC Data Result Registers"]
    pub re: crate::Reg<re::RE_SPEC>,
    #[doc = "0x5c - ADC Data Result Registers"]
    pub rf: crate::Reg<rf::RF_SPEC>,
    #[doc = "0x60 - ADC Data Result Registers"]
    pub rg: crate::Reg<rg::RG_SPEC>,
    #[doc = "0x64 - ADC Data Result Registers"]
    pub rh: crate::Reg<rh::RH_SPEC>,
    #[doc = "0x68 - ADC Data Result Registers"]
    pub ri: crate::Reg<ri::RI_SPEC>,
    #[doc = "0x6c - ADC Data Result Registers"]
    pub rj: crate::Reg<rj::RJ_SPEC>,
    #[doc = "0x70 - ADC Data Result Registers"]
    pub rk: crate::Reg<rk::RK_SPEC>,
    #[doc = "0x74 - ADC Data Result Registers"]
    pub rl: crate::Reg<rl::RL_SPEC>,
    #[doc = "0x78 - ADC Data Result Registers"]
    pub rm: crate::Reg<rm::RM_SPEC>,
    #[doc = "0x7c - ADC Data Result Registers"]
    pub rn: crate::Reg<rn::RN_SPEC>,
    #[doc = "0x80 - ADC Data Result Registers"]
    pub ro: crate::Reg<ro::RO_SPEC>,
    #[doc = "0x84 - ADC Data Result Registers"]
    pub rp: crate::Reg<rp::RP_SPEC>,
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
    _reserved59: [u8; 0x1c],
    #[doc = "0x108 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1a: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x10c - ADC Status and Control Register 1 (alias)"]
    pub a_sc1b: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x110 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1c: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x114 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1d: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x118 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1e: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x11c - ADC Status and Control Register 1 (alias)"]
    pub a_sc1f: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x120 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1g: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x124 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1h: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x128 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1i: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x12c - ADC Status and Control Register 1 (alias)"]
    pub a_sc1j: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x130 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1k: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x134 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1l: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x138 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1m: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x13c - ADC Status and Control Register 1 (alias)"]
    pub a_sc1n: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x140 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1o: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x144 - ADC Status and Control Register 1 (alias)"]
    pub a_sc1p: crate::Reg<a_sc1::ASC1_SPEC>,
    #[doc = "0x148 - ADC Status and Control Register 1"]
    pub sc1q: crate::Reg<sc1q::SC1Q_SPEC>,
    #[doc = "0x14c - ADC Status and Control Register 1"]
    pub sc1r: crate::Reg<sc1r::SC1R_SPEC>,
    #[doc = "0x150 - ADC Status and Control Register 1"]
    pub sc1s: crate::Reg<sc1s::SC1S_SPEC>,
    #[doc = "0x154 - ADC Status and Control Register 1"]
    pub sc1t: crate::Reg<sc1t::SC1T_SPEC>,
    #[doc = "0x158 - ADC Status and Control Register 1"]
    pub sc1u: crate::Reg<sc1u::SC1U_SPEC>,
    #[doc = "0x15c - ADC Status and Control Register 1"]
    pub sc1v: crate::Reg<sc1v::SC1V_SPEC>,
    #[doc = "0x160 - ADC Status and Control Register 1"]
    pub sc1w: crate::Reg<sc1w::SC1W_SPEC>,
    #[doc = "0x164 - ADC Status and Control Register 1"]
    pub sc1x: crate::Reg<sc1x::SC1X_SPEC>,
    #[doc = "0x168 - ADC Status and Control Register 1"]
    pub sc1y: crate::Reg<sc1y::SC1Y_SPEC>,
    #[doc = "0x16c - ADC Status and Control Register 1"]
    pub sc1z: crate::Reg<sc1z::SC1Z_SPEC>,
    #[doc = "0x170 - ADC Status and Control Register 1"]
    pub sc1aa: crate::Reg<sc1aa::SC1AA_SPEC>,
    #[doc = "0x174 - ADC Status and Control Register 1"]
    pub sc1ab: crate::Reg<sc1ab::SC1AB_SPEC>,
    #[doc = "0x178 - ADC Status and Control Register 1"]
    pub sc1ac: crate::Reg<sc1ac::SC1AC_SPEC>,
    #[doc = "0x17c - ADC Status and Control Register 1"]
    pub sc1ad: crate::Reg<sc1ad::SC1AD_SPEC>,
    #[doc = "0x180 - ADC Status and Control Register 1"]
    pub sc1ae: crate::Reg<sc1ae::SC1AE_SPEC>,
    #[doc = "0x184 - ADC Status and Control Register 1"]
    pub sc1af: crate::Reg<sc1af::SC1AF_SPEC>,
    #[doc = "0x188 - ADC Data Result Registers (alias)"]
    pub a_ra: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x18c - ADC Data Result Registers (alias)"]
    pub a_rb: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x190 - ADC Data Result Registers (alias)"]
    pub a_rc: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x194 - ADC Data Result Registers (alias)"]
    pub a_rd: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x198 - ADC Data Result Registers (alias)"]
    pub a_re: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x19c - ADC Data Result Registers (alias)"]
    pub a_rf: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1a0 - ADC Data Result Registers (alias)"]
    pub a_rg: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1a4 - ADC Data Result Registers (alias)"]
    pub a_rh: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1a8 - ADC Data Result Registers (alias)"]
    pub a_ri: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1ac - ADC Data Result Registers (alias)"]
    pub a_rj: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1b0 - ADC Data Result Registers (alias)"]
    pub a_rk: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1b4 - ADC Data Result Registers (alias)"]
    pub a_rl: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1b8 - ADC Data Result Registers (alias)"]
    pub a_rm: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1bc - ADC Data Result Registers (alias)"]
    pub a_rn: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1c0 - ADC Data Result Registers (alias)"]
    pub a_ro: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1c4 - ADC Data Result Registers (alias)"]
    pub a_rp: crate::Reg<a_r::AR_SPEC>,
    #[doc = "0x1c8 - ADC Data Result Registers"]
    pub rq: crate::Reg<rq::RQ_SPEC>,
    #[doc = "0x1cc - ADC Data Result Registers"]
    pub rr: crate::Reg<rr::RR_SPEC>,
    #[doc = "0x1d0 - ADC Data Result Registers"]
    pub rs: crate::Reg<rs::RS_SPEC>,
    #[doc = "0x1d4 - ADC Data Result Registers"]
    pub rt: crate::Reg<rt::RT_SPEC>,
    #[doc = "0x1d8 - ADC Data Result Registers"]
    pub ru: crate::Reg<ru::RU_SPEC>,
    #[doc = "0x1dc - ADC Data Result Registers"]
    pub rv: crate::Reg<rv::RV_SPEC>,
    #[doc = "0x1e0 - ADC Data Result Registers"]
    pub rw: crate::Reg<rw::RW_SPEC>,
    #[doc = "0x1e4 - ADC Data Result Registers"]
    pub rx: crate::Reg<rx::RX_SPEC>,
    #[doc = "0x1e8 - ADC Data Result Registers"]
    pub ry: crate::Reg<ry::RY_SPEC>,
    #[doc = "0x1ec - ADC Data Result Registers"]
    pub rz: crate::Reg<rz::RZ_SPEC>,
    #[doc = "0x1f0 - ADC Data Result Registers"]
    pub raa: crate::Reg<raa::RAA_SPEC>,
    #[doc = "0x1f4 - ADC Data Result Registers"]
    pub rab: crate::Reg<rab::RAB_SPEC>,
    #[doc = "0x1f8 - ADC Data Result Registers"]
    pub rac: crate::Reg<rac::RAC_SPEC>,
    #[doc = "0x1fc - ADC Data Result Registers"]
    pub rad: crate::Reg<rad::RAD_SPEC>,
    #[doc = "0x200 - ADC Data Result Registers"]
    pub rae: crate::Reg<rae::RAE_SPEC>,
    #[doc = "0x204 - ADC Data Result Registers"]
    pub raf: crate::Reg<raf::RAF_SPEC>,
}
#[doc = "SC1A register accessor: an alias for `Reg<SC1A_SPEC>`"]
pub type SC1A = crate::Reg<sc1a::SC1A_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1a;
#[doc = "SC1B register accessor: an alias for `Reg<SC1B_SPEC>`"]
pub type SC1B = crate::Reg<sc1b::SC1B_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1b;
#[doc = "SC1C register accessor: an alias for `Reg<SC1C_SPEC>`"]
pub type SC1C = crate::Reg<sc1c::SC1C_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1c;
#[doc = "SC1D register accessor: an alias for `Reg<SC1D_SPEC>`"]
pub type SC1D = crate::Reg<sc1d::SC1D_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1d;
#[doc = "SC1E register accessor: an alias for `Reg<SC1E_SPEC>`"]
pub type SC1E = crate::Reg<sc1e::SC1E_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1e;
#[doc = "SC1F register accessor: an alias for `Reg<SC1F_SPEC>`"]
pub type SC1F = crate::Reg<sc1f::SC1F_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1f;
#[doc = "SC1G register accessor: an alias for `Reg<SC1G_SPEC>`"]
pub type SC1G = crate::Reg<sc1g::SC1G_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1g;
#[doc = "SC1H register accessor: an alias for `Reg<SC1H_SPEC>`"]
pub type SC1H = crate::Reg<sc1h::SC1H_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1h;
#[doc = "SC1I register accessor: an alias for `Reg<SC1I_SPEC>`"]
pub type SC1I = crate::Reg<sc1i::SC1I_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1i;
#[doc = "SC1J register accessor: an alias for `Reg<SC1J_SPEC>`"]
pub type SC1J = crate::Reg<sc1j::SC1J_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1j;
#[doc = "SC1K register accessor: an alias for `Reg<SC1K_SPEC>`"]
pub type SC1K = crate::Reg<sc1k::SC1K_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1k;
#[doc = "SC1L register accessor: an alias for `Reg<SC1L_SPEC>`"]
pub type SC1L = crate::Reg<sc1l::SC1L_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1l;
#[doc = "SC1M register accessor: an alias for `Reg<SC1M_SPEC>`"]
pub type SC1M = crate::Reg<sc1m::SC1M_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1m;
#[doc = "SC1N register accessor: an alias for `Reg<SC1N_SPEC>`"]
pub type SC1N = crate::Reg<sc1n::SC1N_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1n;
#[doc = "SC1O register accessor: an alias for `Reg<SC1O_SPEC>`"]
pub type SC1O = crate::Reg<sc1o::SC1O_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1o;
#[doc = "SC1P register accessor: an alias for `Reg<SC1P_SPEC>`"]
pub type SC1P = crate::Reg<sc1p::SC1P_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1p;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "RA register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod ra;
#[doc = "RB register accessor: an alias for `Reg<RB_SPEC>`"]
pub type RB = crate::Reg<rb::RB_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rb;
#[doc = "RC register accessor: an alias for `Reg<RC_SPEC>`"]
pub type RC = crate::Reg<rc::RC_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rc;
#[doc = "RD register accessor: an alias for `Reg<RD_SPEC>`"]
pub type RD = crate::Reg<rd::RD_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rd;
#[doc = "RE register accessor: an alias for `Reg<RE_SPEC>`"]
pub type RE = crate::Reg<re::RE_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod re;
#[doc = "RF register accessor: an alias for `Reg<RF_SPEC>`"]
pub type RF = crate::Reg<rf::RF_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rf;
#[doc = "RG register accessor: an alias for `Reg<RG_SPEC>`"]
pub type RG = crate::Reg<rg::RG_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rg;
#[doc = "RH register accessor: an alias for `Reg<RH_SPEC>`"]
pub type RH = crate::Reg<rh::RH_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rh;
#[doc = "RI register accessor: an alias for `Reg<RI_SPEC>`"]
pub type RI = crate::Reg<ri::RI_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod ri;
#[doc = "RJ register accessor: an alias for `Reg<RJ_SPEC>`"]
pub type RJ = crate::Reg<rj::RJ_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rj;
#[doc = "RK register accessor: an alias for `Reg<RK_SPEC>`"]
pub type RK = crate::Reg<rk::RK_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rk;
#[doc = "RL register accessor: an alias for `Reg<RL_SPEC>`"]
pub type RL = crate::Reg<rl::RL_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rl;
#[doc = "RM register accessor: an alias for `Reg<RM_SPEC>`"]
pub type RM = crate::Reg<rm::RM_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rm;
#[doc = "RN register accessor: an alias for `Reg<RN_SPEC>`"]
pub type RN = crate::Reg<rn::RN_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rn;
#[doc = "RO register accessor: an alias for `Reg<RO_SPEC>`"]
pub type RO = crate::Reg<ro::RO_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod ro;
#[doc = "RP register accessor: an alias for `Reg<RP_SPEC>`"]
pub type RP = crate::Reg<rp::RP_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rp;
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
#[doc = "aSC1 register accessor: an alias for `Reg<ASC1_SPEC>`"]
pub type ASC1 = crate::Reg<a_sc1::ASC1_SPEC>;
#[doc = "ADC Status and Control Register 1 (alias)"]
pub mod a_sc1;
#[doc = "SC1Q register accessor: an alias for `Reg<SC1Q_SPEC>`"]
pub type SC1Q = crate::Reg<sc1q::SC1Q_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1q;
#[doc = "SC1R register accessor: an alias for `Reg<SC1R_SPEC>`"]
pub type SC1R = crate::Reg<sc1r::SC1R_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1r;
#[doc = "SC1S register accessor: an alias for `Reg<SC1S_SPEC>`"]
pub type SC1S = crate::Reg<sc1s::SC1S_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1s;
#[doc = "SC1T register accessor: an alias for `Reg<SC1T_SPEC>`"]
pub type SC1T = crate::Reg<sc1t::SC1T_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1t;
#[doc = "SC1U register accessor: an alias for `Reg<SC1U_SPEC>`"]
pub type SC1U = crate::Reg<sc1u::SC1U_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1u;
#[doc = "SC1V register accessor: an alias for `Reg<SC1V_SPEC>`"]
pub type SC1V = crate::Reg<sc1v::SC1V_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1v;
#[doc = "SC1W register accessor: an alias for `Reg<SC1W_SPEC>`"]
pub type SC1W = crate::Reg<sc1w::SC1W_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1w;
#[doc = "SC1X register accessor: an alias for `Reg<SC1X_SPEC>`"]
pub type SC1X = crate::Reg<sc1x::SC1X_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1x;
#[doc = "SC1Y register accessor: an alias for `Reg<SC1Y_SPEC>`"]
pub type SC1Y = crate::Reg<sc1y::SC1Y_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1y;
#[doc = "SC1Z register accessor: an alias for `Reg<SC1Z_SPEC>`"]
pub type SC1Z = crate::Reg<sc1z::SC1Z_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1z;
#[doc = "SC1AA register accessor: an alias for `Reg<SC1AA_SPEC>`"]
pub type SC1AA = crate::Reg<sc1aa::SC1AA_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1aa;
#[doc = "SC1AB register accessor: an alias for `Reg<SC1AB_SPEC>`"]
pub type SC1AB = crate::Reg<sc1ab::SC1AB_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1ab;
#[doc = "SC1AC register accessor: an alias for `Reg<SC1AC_SPEC>`"]
pub type SC1AC = crate::Reg<sc1ac::SC1AC_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1ac;
#[doc = "SC1AD register accessor: an alias for `Reg<SC1AD_SPEC>`"]
pub type SC1AD = crate::Reg<sc1ad::SC1AD_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1ad;
#[doc = "SC1AE register accessor: an alias for `Reg<SC1AE_SPEC>`"]
pub type SC1AE = crate::Reg<sc1ae::SC1AE_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1ae;
#[doc = "SC1AF register accessor: an alias for `Reg<SC1AF_SPEC>`"]
pub type SC1AF = crate::Reg<sc1af::SC1AF_SPEC>;
#[doc = "ADC Status and Control Register 1"]
pub mod sc1af;
#[doc = "aR register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<a_r::AR_SPEC>;
#[doc = "ADC Data Result Registers (alias)"]
pub mod a_r;
#[doc = "RQ register accessor: an alias for `Reg<RQ_SPEC>`"]
pub type RQ = crate::Reg<rq::RQ_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rq;
#[doc = "RR register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rr;
#[doc = "RS register accessor: an alias for `Reg<RS_SPEC>`"]
pub type RS = crate::Reg<rs::RS_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rs;
#[doc = "RT register accessor: an alias for `Reg<RT_SPEC>`"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rt;
#[doc = "RU register accessor: an alias for `Reg<RU_SPEC>`"]
pub type RU = crate::Reg<ru::RU_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod ru;
#[doc = "RV register accessor: an alias for `Reg<RV_SPEC>`"]
pub type RV = crate::Reg<rv::RV_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rv;
#[doc = "RW register accessor: an alias for `Reg<RW_SPEC>`"]
pub type RW = crate::Reg<rw::RW_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rw;
#[doc = "RX register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rx;
#[doc = "RY register accessor: an alias for `Reg<RY_SPEC>`"]
pub type RY = crate::Reg<ry::RY_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod ry;
#[doc = "RZ register accessor: an alias for `Reg<RZ_SPEC>`"]
pub type RZ = crate::Reg<rz::RZ_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rz;
#[doc = "RAA register accessor: an alias for `Reg<RAA_SPEC>`"]
pub type RAA = crate::Reg<raa::RAA_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod raa;
#[doc = "RAB register accessor: an alias for `Reg<RAB_SPEC>`"]
pub type RAB = crate::Reg<rab::RAB_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rab;
#[doc = "RAC register accessor: an alias for `Reg<RAC_SPEC>`"]
pub type RAC = crate::Reg<rac::RAC_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rac;
#[doc = "RAD register accessor: an alias for `Reg<RAD_SPEC>`"]
pub type RAD = crate::Reg<rad::RAD_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rad;
#[doc = "RAE register accessor: an alias for `Reg<RAE_SPEC>`"]
pub type RAE = crate::Reg<rae::RAE_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod rae;
#[doc = "RAF register accessor: an alias for `Reg<RAF_SPEC>`"]
pub type RAF = crate::Reg<raf::RAF_SPEC>;
#[doc = "ADC Data Result Registers"]
pub mod raf;
