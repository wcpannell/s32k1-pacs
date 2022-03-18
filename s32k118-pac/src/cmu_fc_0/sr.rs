#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Lower than Low Frequency Reference Threshold Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLL_A {
    #[doc = "0: No FLL Event"]
    _0 = 0,
    #[doc = "1: FLL Event Occured"]
    _1 = 1,
}
impl From<FLL_A> for bool {
    #[inline(always)]
    fn from(variant: FLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLL` reader - Frequency Lower than Low Frequency Reference Threshold Event Status"]
pub struct FLL_R(crate::FieldReader<bool, FLL_A>);
impl FLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLL_A {
        match self.bits {
            false => FLL_A::_0,
            true => FLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLL_A::_1
    }
}
impl core::ops::Deref for FLL_R {
    type Target = crate::FieldReader<bool, FLL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLL` writer - Frequency Lower than Low Frequency Reference Threshold Event Status"]
pub struct FLL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No FLL Event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLL_A::_0)
    }
    #[doc = "FLL Event Occured"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLL_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Frequency Higher than High Frequency Reference Threshold Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FHH_A {
    #[doc = "0: No FHH Event"]
    _0 = 0,
    #[doc = "1: FHH Event Occured"]
    _1 = 1,
}
impl From<FHH_A> for bool {
    #[inline(always)]
    fn from(variant: FHH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FHH` reader - Frequency Higher than High Frequency Reference Threshold Event Status"]
pub struct FHH_R(crate::FieldReader<bool, FHH_A>);
impl FHH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FHH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FHH_A {
        match self.bits {
            false => FHH_A::_0,
            true => FHH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FHH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FHH_A::_1
    }
}
impl core::ops::Deref for FHH_R {
    type Target = crate::FieldReader<bool, FHH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FHH` writer - Frequency Higher than High Frequency Reference Threshold Event Status"]
pub struct FHH_W<'a> {
    w: &'a mut W,
}
impl<'a> FHH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FHH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No FHH Event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FHH_A::_0)
    }
    #[doc = "FHH Event Occured"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FHH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Module State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: Configure State- Configuration registers and CMU_FC_IER programming is getting done"]
    _00 = 0,
    #[doc = "1: Initialization State- Register configurations are getting loaded internally."]
    _01 = 1,
    #[doc = "2: Initialization Wait State- The module stays in this state for 1 bus clock cycle."]
    _10 = 2,
    #[doc = "3: Frequency Check State- The module is ready to start frequency check operation in this state."]
    _11 = 3,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Module State"]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            0 => STATE_A::_00,
            1 => STATE_A::_01,
            2 => STATE_A::_10,
            3 => STATE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == STATE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == STATE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == STATE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == STATE_A::_11
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Run Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS_A {
    #[doc = "0: Frequency Check Stopped"]
    _0 = 0,
    #[doc = "1: Frequency Check Running"]
    _1 = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Run Status"]
pub struct RS_R(crate::FieldReader<bool, RS_A>);
impl RS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::_0,
            true => RS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RS_A::_1
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<bool, RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Lower than Low Frequency Reference Threshold Event Status"]
    #[inline(always)]
    pub fn fll(&self) -> FLL_R {
        FLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frequency Higher than High Frequency Reference Threshold Event Status"]
    #[inline(always)]
    pub fn fhh(&self) -> FHH_R {
        FHH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Module State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Run Status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Lower than Low Frequency Reference Threshold Event Status"]
    #[inline(always)]
    pub fn fll(&mut self) -> FLL_W {
        FLL_W { w: self }
    }
    #[doc = "Bit 1 - Frequency Higher than High Frequency Reference Threshold Event Status"]
    #[inline(always)]
    pub fn fhh(&mut self) -> FHH_W {
        FHH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Frequency Check Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
