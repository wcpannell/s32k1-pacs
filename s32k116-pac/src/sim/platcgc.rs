#[doc = "Register `PLATCGC` reader"]
pub struct R(crate::R<PLATCGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLATCGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLATCGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLATCGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLATCGC` writer"]
pub struct W(crate::W<PLATCGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLATCGC_SPEC>;
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
impl From<crate::W<PLATCGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLATCGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MSCM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMSCM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCMSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCMSCM` reader - MSCM Clock Gating Control"]
pub struct CGCMSCM_R(crate::FieldReader<bool, CGCMSCM_A>);
impl CGCMSCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCMSCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCMSCM_A {
        match self.bits {
            false => CGCMSCM_A::_0,
            true => CGCMSCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCMSCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCMSCM_A::_1
    }
}
impl core::ops::Deref for CGCMSCM_R {
    type Target = crate::FieldReader<bool, CGCMSCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCMSCM` writer - MSCM Clock Gating Control"]
pub struct CGCMSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCMSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCMSCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMSCM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMSCM_A::_1)
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
#[doc = "MPU Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMPU_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCMPU_A> for bool {
    #[inline(always)]
    fn from(variant: CGCMPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCMPU` reader - MPU Clock Gating Control"]
pub struct CGCMPU_R(crate::FieldReader<bool, CGCMPU_A>);
impl CGCMPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCMPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCMPU_A {
        match self.bits {
            false => CGCMPU_A::_0,
            true => CGCMPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCMPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCMPU_A::_1
    }
}
impl core::ops::Deref for CGCMPU_R {
    type Target = crate::FieldReader<bool, CGCMPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCMPU` writer - MPU Clock Gating Control"]
pub struct CGCMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCMPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCMPU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMPU_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMPU_A::_1)
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
#[doc = "DMA Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCDMA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCDMA_A> for bool {
    #[inline(always)]
    fn from(variant: CGCDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCDMA` reader - DMA Clock Gating Control"]
pub struct CGCDMA_R(crate::FieldReader<bool, CGCDMA_A>);
impl CGCDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCDMA_A {
        match self.bits {
            false => CGCDMA_A::_0,
            true => CGCDMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCDMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCDMA_A::_1
    }
}
impl core::ops::Deref for CGCDMA_R {
    type Target = crate::FieldReader<bool, CGCDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCDMA` writer - DMA Clock Gating Control"]
pub struct CGCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCDMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCDMA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCDMA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "ERM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCERM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCERM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCERM` reader - ERM Clock Gating Control"]
pub struct CGCERM_R(crate::FieldReader<bool, CGCERM_A>);
impl CGCERM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCERM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCERM_A {
        match self.bits {
            false => CGCERM_A::_0,
            true => CGCERM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCERM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCERM_A::_1
    }
}
impl core::ops::Deref for CGCERM_R {
    type Target = crate::FieldReader<bool, CGCERM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCERM` writer - ERM Clock Gating Control"]
pub struct CGCERM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCERM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCERM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCERM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "EIM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCEIM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCEIM` reader - EIM Clock Gating Control"]
pub struct CGCEIM_R(crate::FieldReader<bool, CGCEIM_A>);
impl CGCEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCEIM_A {
        match self.bits {
            false => CGCEIM_A::_0,
            true => CGCEIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCEIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCEIM_A::_1
    }
}
impl core::ops::Deref for CGCEIM_R {
    type Target = crate::FieldReader<bool, CGCEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCEIM` writer - EIM Clock Gating Control"]
pub struct CGCEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCEIM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCEIM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIO Clock Gating Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCGPIO_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCGPIO_A> for bool {
    #[inline(always)]
    fn from(variant: CGCGPIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGCGPIO` reader - GPIO Clock Gating Control"]
pub struct CGCGPIO_R(crate::FieldReader<bool, CGCGPIO_A>);
impl CGCGPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGCGPIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCGPIO_A {
        match self.bits {
            false => CGCGPIO_A::_0,
            true => CGCGPIO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGCGPIO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGCGPIO_A::_1
    }
}
impl core::ops::Deref for CGCGPIO_R {
    type Target = crate::FieldReader<bool, CGCGPIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGCGPIO` writer - GPIO Clock Gating Control"]
pub struct CGCGPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCGPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCGPIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCGPIO_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCGPIO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmscm(&self) -> CGCMSCM_R {
        CGCMSCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmpu(&self) -> CGCMPU_R {
        CGCMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline(always)]
    pub fn cgcdma(&self) -> CGCDMA_R {
        CGCDMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcerm(&self) -> CGCERM_R {
        CGCERM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline(always)]
    pub fn cgceim(&self) -> CGCEIM_R {
        CGCEIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Clock Gating Control"]
    #[inline(always)]
    pub fn cgcgpio(&self) -> CGCGPIO_R {
        CGCGPIO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmscm(&mut self) -> CGCMSCM_W {
        CGCMSCM_W { w: self }
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmpu(&mut self) -> CGCMPU_W {
        CGCMPU_W { w: self }
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline(always)]
    pub fn cgcdma(&mut self) -> CGCDMA_W {
        CGCDMA_W { w: self }
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcerm(&mut self) -> CGCERM_W {
        CGCERM_W { w: self }
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline(always)]
    pub fn cgceim(&mut self) -> CGCEIM_W {
        CGCEIM_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Clock Gating Control"]
    #[inline(always)]
    pub fn cgcgpio(&mut self) -> CGCGPIO_W {
        CGCGPIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Platform Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [platcgc](index.html) module"]
pub struct PLATCGC_SPEC;
impl crate::RegisterSpec for PLATCGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [platcgc::R](R) reader structure"]
impl crate::Readable for PLATCGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [platcgc::W](W) writer structure"]
impl crate::Writable for PLATCGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLATCGC to value 0x1f"]
impl crate::Resettable for PLATCGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
