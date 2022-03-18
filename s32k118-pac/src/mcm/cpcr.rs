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
    _00 = 0,
    #[doc = "1: Waiting for platform idle"]
    _01 = 1,
    #[doc = "3: Platform stalled"]
    _11 = 3,
    #[doc = "2: Unused state"]
    _10 = 2,
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
            0 => HLT_FSM_ST_A::_00,
            1 => HLT_FSM_ST_A::_01,
            3 => HLT_FSM_ST_A::_11,
            2 => HLT_FSM_ST_A::_10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == HLT_FSM_ST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == HLT_FSM_ST_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == HLT_FSM_ST_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == HLT_FSM_ST_A::_10
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
    _0 = 0,
    #[doc = "1: AXBS is receiving halt request"]
    _1 = 1,
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
            false => AXBS_HLT_REQ_A::_0,
            true => AXBS_HLT_REQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AXBS_HLT_REQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AXBS_HLT_REQ_A::_1
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
    _0 = 0,
    #[doc = "1: AXBS is currently halted"]
    _1 = 1,
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
            false => AXBS_HLTD_A::_0,
            true => AXBS_HLTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AXBS_HLTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AXBS_HLTD_A::_1
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
    _0 = 0,
    #[doc = "1: FMC program flash is currently idle"]
    _1 = 1,
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
            false => FMC_PF_IDLE_A::_0,
            true => FMC_PF_IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FMC_PF_IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FMC_PF_IDLE_A::_1
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
    _0 = 0,
    #[doc = "1: PBRIDGE is currently idle"]
    _1 = 1,
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
            false => PBRIDGE_IDLE_A::_0,
            true => PBRIDGE_IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PBRIDGE_IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PBRIDGE_IDLE_A::_1
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
    _0 = 0,
    #[doc = "1: Round-robin arbitration"]
    _1 = 1,
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
            false => CBRR_A::_0,
            true => CBRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CBRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CBRR_A::_1
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
    pub fn _0(self) -> &'a mut W {
        self.variant(CBRR_A::_0)
    }
    #[doc = "Round-robin arbitration"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CBRR_A::_1)
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
}
impl W {
    #[doc = "Bit 9 - Crossbar Round-robin Arbitration Enable"]
    #[inline(always)]
    pub fn cbrr(&mut self) -> CBRR_W {
        CBRR_W { w: self }
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
