#[doc = "Register `CPCR` reader"]
pub struct R(crate::R<CPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPCR` writer"]
pub struct W(crate::W<CPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AXBS Halt State Machine Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HLT_FSM_ST_A {
    #[doc = "0: Waiting for request"]
    HLT_FSM_ST_0 = 0,
    #[doc = "1: Waiting for platform idle"]
    HLT_FSM_ST_1 = 1,
    #[doc = "2: Unused state"]
    HLT_FSM_ST_2 = 2,
    #[doc = "3: Platform stalled"]
    HLT_FSM_ST_3 = 3,
}
impl From<HLT_FSM_ST_A> for u8 {
    #[inline(always)]
    fn from(variant: HLT_FSM_ST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HLT_FSM_ST` reader - AXBS Halt State Machine Status"]
pub struct HLT_FSM_ST_R(crate::FieldReader<u8, HLT_FSM_ST_A>);
impl HLT_FSM_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HLT_FSM_ST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLT_FSM_ST_A {
        match self.bits {
            0 => HLT_FSM_ST_A::HLT_FSM_ST_0,
            1 => HLT_FSM_ST_A::HLT_FSM_ST_1,
            2 => HLT_FSM_ST_A::HLT_FSM_ST_2,
            3 => HLT_FSM_ST_A::HLT_FSM_ST_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HLT_FSM_ST_0`"]
    #[inline(always)]
    pub fn is_hlt_fsm_st_0(&self) -> bool {
        **self == HLT_FSM_ST_A::HLT_FSM_ST_0
    }
    #[doc = "Checks if the value of the field is `HLT_FSM_ST_1`"]
    #[inline(always)]
    pub fn is_hlt_fsm_st_1(&self) -> bool {
        **self == HLT_FSM_ST_A::HLT_FSM_ST_1
    }
    #[doc = "Checks if the value of the field is `HLT_FSM_ST_2`"]
    #[inline(always)]
    pub fn is_hlt_fsm_st_2(&self) -> bool {
        **self == HLT_FSM_ST_A::HLT_FSM_ST_2
    }
    #[doc = "Checks if the value of the field is `HLT_FSM_ST_3`"]
    #[inline(always)]
    pub fn is_hlt_fsm_st_3(&self) -> bool {
        **self == HLT_FSM_ST_A::HLT_FSM_ST_3
    }
}
impl core::ops::Deref for HLT_FSM_ST_R {
    type Target = crate::FieldReader<u8, HLT_FSM_ST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AXBS Halt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_HLT_REQ_A {
    #[doc = "0: AXBS is not receiving halt request"]
    AXBS_HLT_REQ_0 = 0,
    #[doc = "1: AXBS is receiving halt request"]
    AXBS_HLT_REQ_1 = 1,
}
impl From<AXBS_HLT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_HLT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXBS_HLT_REQ` reader - AXBS Halt Request"]
pub struct AXBS_HLT_REQ_R(crate::FieldReader<bool, AXBS_HLT_REQ_A>);
impl AXBS_HLT_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AXBS_HLT_REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_HLT_REQ_A {
        match self.bits {
            false => AXBS_HLT_REQ_A::AXBS_HLT_REQ_0,
            true => AXBS_HLT_REQ_A::AXBS_HLT_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_HLT_REQ_0`"]
    #[inline(always)]
    pub fn is_axbs_hlt_req_0(&self) -> bool {
        **self == AXBS_HLT_REQ_A::AXBS_HLT_REQ_0
    }
    #[doc = "Checks if the value of the field is `AXBS_HLT_REQ_1`"]
    #[inline(always)]
    pub fn is_axbs_hlt_req_1(&self) -> bool {
        **self == AXBS_HLT_REQ_A::AXBS_HLT_REQ_1
    }
}
impl core::ops::Deref for AXBS_HLT_REQ_R {
    type Target = crate::FieldReader<bool, AXBS_HLT_REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AXBS Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_HLTD_A {
    #[doc = "0: AXBS is not currently halted"]
    AXBS_HLTD_0 = 0,
    #[doc = "1: AXBS is currently halted"]
    AXBS_HLTD_1 = 1,
}
impl From<AXBS_HLTD_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_HLTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXBS_HLTD` reader - AXBS Halted"]
pub struct AXBS_HLTD_R(crate::FieldReader<bool, AXBS_HLTD_A>);
impl AXBS_HLTD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AXBS_HLTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_HLTD_A {
        match self.bits {
            false => AXBS_HLTD_A::AXBS_HLTD_0,
            true => AXBS_HLTD_A::AXBS_HLTD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_HLTD_0`"]
    #[inline(always)]
    pub fn is_axbs_hltd_0(&self) -> bool {
        **self == AXBS_HLTD_A::AXBS_HLTD_0
    }
    #[doc = "Checks if the value of the field is `AXBS_HLTD_1`"]
    #[inline(always)]
    pub fn is_axbs_hltd_1(&self) -> bool {
        **self == AXBS_HLTD_A::AXBS_HLTD_1
    }
}
impl core::ops::Deref for AXBS_HLTD_R {
    type Target = crate::FieldReader<bool, AXBS_HLTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Memory Controller Program Flash Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_PF_IDLE_A {
    #[doc = "0: FMC program flash is not idle"]
    FMC_PF_IDLE_0 = 0,
    #[doc = "1: FMC program flash is currently idle"]
    FMC_PF_IDLE_1 = 1,
}
impl From<FMC_PF_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_PF_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMC_PF_IDLE` reader - Flash Memory Controller Program Flash Idle"]
pub struct FMC_PF_IDLE_R(crate::FieldReader<bool, FMC_PF_IDLE_A>);
impl FMC_PF_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMC_PF_IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_PF_IDLE_A {
        match self.bits {
            false => FMC_PF_IDLE_A::FMC_PF_IDLE_0,
            true => FMC_PF_IDLE_A::FMC_PF_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FMC_PF_IDLE_0`"]
    #[inline(always)]
    pub fn is_fmc_pf_idle_0(&self) -> bool {
        **self == FMC_PF_IDLE_A::FMC_PF_IDLE_0
    }
    #[doc = "Checks if the value of the field is `FMC_PF_IDLE_1`"]
    #[inline(always)]
    pub fn is_fmc_pf_idle_1(&self) -> bool {
        **self == FMC_PF_IDLE_A::FMC_PF_IDLE_1
    }
}
impl core::ops::Deref for FMC_PF_IDLE_R {
    type Target = crate::FieldReader<bool, FMC_PF_IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Peripheral Bridge Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBRIDGE_IDLE_A {
    #[doc = "0: PBRIDGE is not idle"]
    PBRIDGE_IDLE_0 = 0,
    #[doc = "1: PBRIDGE is currently idle"]
    PBRIDGE_IDLE_1 = 1,
}
impl From<PBRIDGE_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: PBRIDGE_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBRIDGE_IDLE` reader - Peripheral Bridge Idle"]
pub struct PBRIDGE_IDLE_R(crate::FieldReader<bool, PBRIDGE_IDLE_A>);
impl PBRIDGE_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBRIDGE_IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBRIDGE_IDLE_A {
        match self.bits {
            false => PBRIDGE_IDLE_A::PBRIDGE_IDLE_0,
            true => PBRIDGE_IDLE_A::PBRIDGE_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PBRIDGE_IDLE_0`"]
    #[inline(always)]
    pub fn is_pbridge_idle_0(&self) -> bool {
        **self == PBRIDGE_IDLE_A::PBRIDGE_IDLE_0
    }
    #[doc = "Checks if the value of the field is `PBRIDGE_IDLE_1`"]
    #[inline(always)]
    pub fn is_pbridge_idle_1(&self) -> bool {
        **self == PBRIDGE_IDLE_A::PBRIDGE_IDLE_1
    }
}
impl core::ops::Deref for PBRIDGE_IDLE_R {
    type Target = crate::FieldReader<bool, PBRIDGE_IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Crossbar Round-robin Arbitration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBRR_A {
    #[doc = "0: Fixed-priority arbitration"]
    CBRR_0 = 0,
    #[doc = "1: Round-robin arbitration"]
    CBRR_1 = 1,
}
impl From<CBRR_A> for bool {
    #[inline(always)]
    fn from(variant: CBRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBRR` reader - Crossbar Round-robin Arbitration Enable"]
pub struct CBRR_R(crate::FieldReader<bool, CBRR_A>);
impl CBRR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBRR_A {
        match self.bits {
            false => CBRR_A::CBRR_0,
            true => CBRR_A::CBRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CBRR_0`"]
    #[inline(always)]
    pub fn is_cbrr_0(&self) -> bool {
        **self == CBRR_A::CBRR_0
    }
    #[doc = "Checks if the value of the field is `CBRR_1`"]
    #[inline(always)]
    pub fn is_cbrr_1(&self) -> bool {
        **self == CBRR_A::CBRR_1
    }
}
impl core::ops::Deref for CBRR_R {
    type Target = crate::FieldReader<bool, CBRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBRR` writer - Crossbar Round-robin Arbitration Enable"]
pub struct CBRR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed-priority arbitration"]
    #[inline(always)]
    pub fn cbrr_0(self) -> &'a mut W {
        self.variant(CBRR_A::CBRR_0)
    }
    #[doc = "Round-robin arbitration"]
    #[inline(always)]
    pub fn cbrr_1(self) -> &'a mut W {
        self.variant(CBRR_A::CBRR_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "SRAM_U Arbitration Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMUAP_A {
    #[doc = "0: Round robin"]
    SRAMUAP_0 = 0,
    #[doc = "1: Special round robin (favors SRAM backdoor accesses over the processor)"]
    SRAMUAP_1 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    SRAMUAP_2 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    SRAMUAP_3 = 3,
}
impl From<SRAMUAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMUAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMUAP` reader - SRAM_U Arbitration Priority"]
pub struct SRAMUAP_R(crate::FieldReader<u8, SRAMUAP_A>);
impl SRAMUAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAMUAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMUAP_A {
        match self.bits {
            0 => SRAMUAP_A::SRAMUAP_0,
            1 => SRAMUAP_A::SRAMUAP_1,
            2 => SRAMUAP_A::SRAMUAP_2,
            3 => SRAMUAP_A::SRAMUAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRAMUAP_0`"]
    #[inline(always)]
    pub fn is_sramuap_0(&self) -> bool {
        **self == SRAMUAP_A::SRAMUAP_0
    }
    #[doc = "Checks if the value of the field is `SRAMUAP_1`"]
    #[inline(always)]
    pub fn is_sramuap_1(&self) -> bool {
        **self == SRAMUAP_A::SRAMUAP_1
    }
    #[doc = "Checks if the value of the field is `SRAMUAP_2`"]
    #[inline(always)]
    pub fn is_sramuap_2(&self) -> bool {
        **self == SRAMUAP_A::SRAMUAP_2
    }
    #[doc = "Checks if the value of the field is `SRAMUAP_3`"]
    #[inline(always)]
    pub fn is_sramuap_3(&self) -> bool {
        **self == SRAMUAP_A::SRAMUAP_3
    }
}
impl core::ops::Deref for SRAMUAP_R {
    type Target = crate::FieldReader<u8, SRAMUAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMUAP` writer - SRAM_U Arbitration Priority"]
pub struct SRAMUAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMUAP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn sramuap_0(self) -> &'a mut W {
        self.variant(SRAMUAP_A::SRAMUAP_0)
    }
    #[doc = "Special round robin (favors SRAM backdoor accesses over the processor)"]
    #[inline(always)]
    pub fn sramuap_1(self) -> &'a mut W {
        self.variant(SRAMUAP_A::SRAMUAP_1)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn sramuap_2(self) -> &'a mut W {
        self.variant(SRAMUAP_A::SRAMUAP_2)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn sramuap_3(self) -> &'a mut W {
        self.variant(SRAMUAP_A::SRAMUAP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `SRAMUWP` reader - SRAM_U Write Protect"]
pub struct SRAMUWP_R(crate::FieldReader<bool, bool>);
impl SRAMUWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAMUWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAMUWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMUWP` writer - SRAM_U Write Protect"]
pub struct SRAMUWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUWP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "SRAM_L Arbitration Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMLAP_A {
    #[doc = "0: Round robin"]
    SRAMLAP_0 = 0,
    #[doc = "1: Special round robin (favors SRAM backdoor accesses over the processor)"]
    SRAMLAP_1 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    SRAMLAP_2 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    SRAMLAP_3 = 3,
}
impl From<SRAMLAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMLAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMLAP` reader - SRAM_L Arbitration Priority"]
pub struct SRAMLAP_R(crate::FieldReader<u8, SRAMLAP_A>);
impl SRAMLAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAMLAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMLAP_A {
        match self.bits {
            0 => SRAMLAP_A::SRAMLAP_0,
            1 => SRAMLAP_A::SRAMLAP_1,
            2 => SRAMLAP_A::SRAMLAP_2,
            3 => SRAMLAP_A::SRAMLAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRAMLAP_0`"]
    #[inline(always)]
    pub fn is_sramlap_0(&self) -> bool {
        **self == SRAMLAP_A::SRAMLAP_0
    }
    #[doc = "Checks if the value of the field is `SRAMLAP_1`"]
    #[inline(always)]
    pub fn is_sramlap_1(&self) -> bool {
        **self == SRAMLAP_A::SRAMLAP_1
    }
    #[doc = "Checks if the value of the field is `SRAMLAP_2`"]
    #[inline(always)]
    pub fn is_sramlap_2(&self) -> bool {
        **self == SRAMLAP_A::SRAMLAP_2
    }
    #[doc = "Checks if the value of the field is `SRAMLAP_3`"]
    #[inline(always)]
    pub fn is_sramlap_3(&self) -> bool {
        **self == SRAMLAP_A::SRAMLAP_3
    }
}
impl core::ops::Deref for SRAMLAP_R {
    type Target = crate::FieldReader<u8, SRAMLAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMLAP` writer - SRAM_L Arbitration Priority"]
pub struct SRAMLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMLAP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn sramlap_0(self) -> &'a mut W {
        self.variant(SRAMLAP_A::SRAMLAP_0)
    }
    #[doc = "Special round robin (favors SRAM backdoor accesses over the processor)"]
    #[inline(always)]
    pub fn sramlap_1(self) -> &'a mut W {
        self.variant(SRAMLAP_A::SRAMLAP_1)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn sramlap_2(self) -> &'a mut W {
        self.variant(SRAMLAP_A::SRAMLAP_2)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn sramlap_3(self) -> &'a mut W {
        self.variant(SRAMLAP_A::SRAMLAP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `SRAMLWP` reader - SRAM_L Write Protect"]
pub struct SRAMLWP_R(crate::FieldReader<bool, bool>);
impl SRAMLWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAMLWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAMLWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMLWP` writer - SRAM_L Write Protect"]
pub struct SRAMLWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLWP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AXBS Halt State Machine Status"]
    #[inline(always)]
    pub fn hlt_fsm_st(&self) -> HLT_FSM_ST_R {
        HLT_FSM_ST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - AXBS Halt Request"]
    #[inline(always)]
    pub fn axbs_hlt_req(&self) -> AXBS_HLT_REQ_R {
        AXBS_HLT_REQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXBS Halted"]
    #[inline(always)]
    pub fn axbs_hltd(&self) -> AXBS_HLTD_R {
        AXBS_HLTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Memory Controller Program Flash Idle"]
    #[inline(always)]
    pub fn fmc_pf_idle(&self) -> FMC_PF_IDLE_R {
        FMC_PF_IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral Bridge Idle"]
    #[inline(always)]
    pub fn pbridge_idle(&self) -> PBRIDGE_IDLE_R {
        PBRIDGE_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Crossbar Round-robin Arbitration Enable"]
    #[inline(always)]
    pub fn cbrr(&self) -> CBRR_R {
        CBRR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - SRAM_U Arbitration Priority"]
    #[inline(always)]
    pub fn sramuap(&self) -> SRAMUAP_R {
        SRAMUAP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - SRAM_U Write Protect"]
    #[inline(always)]
    pub fn sramuwp(&self) -> SRAMUWP_R {
        SRAMUWP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - SRAM_L Arbitration Priority"]
    #[inline(always)]
    pub fn sramlap(&self) -> SRAMLAP_R {
        SRAMLAP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&self) -> SRAMLWP_R {
        SRAMLWP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Crossbar Round-robin Arbitration Enable"]
    #[inline(always)]
    pub fn cbrr(&mut self) -> CBRR_W {
        CBRR_W { w: self }
    }
    #[doc = "Bits 24:25 - SRAM_U Arbitration Priority"]
    #[inline(always)]
    pub fn sramuap(&mut self) -> SRAMUAP_W {
        SRAMUAP_W { w: self }
    }
    #[doc = "Bit 26 - SRAM_U Write Protect"]
    #[inline(always)]
    pub fn sramuwp(&mut self) -> SRAMUWP_W {
        SRAMUWP_W { w: self }
    }
    #[doc = "Bits 28:29 - SRAM_L Arbitration Priority"]
    #[inline(always)]
    pub fn sramlap(&mut self) -> SRAMLAP_W {
        SRAMLAP_W { w: self }
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&mut self) -> SRAMLWP_W {
        SRAMLWP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Platform Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpcr](index.html) module"]
pub struct CPCR_SPEC;
impl crate::RegisterSpec for CPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpcr::R](R) reader structure"]
impl crate::Readable for CPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpcr::W](W) writer structure"]
impl crate::Writable for CPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPCR to value 0"]
impl crate::Resettable for CPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
