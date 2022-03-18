#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Lower than Low Frequency Reference Threshold Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLIE_A {
    #[doc = "0: FLL Interrupt is Disabled"]
    _0 = 0,
    #[doc = "1: FLL Interrupt is Enabled"]
    _1 = 1,
}
impl From<FLLIE_A> for bool {
    #[inline(always)]
    fn from(variant: FLLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLLIE` reader - Frequency Lower than Low Frequency Reference Threshold Interrupt Enable"]
pub struct FLLIE_R(crate::FieldReader<bool, FLLIE_A>);
impl FLLIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLIE_A {
        match self.bits {
            false => FLLIE_A::_0,
            true => FLLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLLIE_A::_1
    }
}
impl core::ops::Deref for FLLIE_R {
    type Target = crate::FieldReader<bool, FLLIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLIE` writer - Frequency Lower than Low Frequency Reference Threshold Interrupt Enable"]
pub struct FLLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLL Interrupt is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLLIE_A::_0)
    }
    #[doc = "FLL Interrupt is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLIE_A::_1)
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
#[doc = "Frequency Higher than High Frequency Reference Threshold Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FHHIE_A {
    #[doc = "0: FHH Interrupt is Disabled"]
    _0 = 0,
    #[doc = "1: FHH Interrupt is Enabled"]
    _1 = 1,
}
impl From<FHHIE_A> for bool {
    #[inline(always)]
    fn from(variant: FHHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FHHIE` reader - Frequency Higher than High Frequency Reference Threshold Interrupt Enable"]
pub struct FHHIE_R(crate::FieldReader<bool, FHHIE_A>);
impl FHHIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FHHIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FHHIE_A {
        match self.bits {
            false => FHHIE_A::_0,
            true => FHHIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FHHIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FHHIE_A::_1
    }
}
impl core::ops::Deref for FHHIE_R {
    type Target = crate::FieldReader<bool, FHHIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FHHIE` writer - Frequency Higher than High Frequency Reference Threshold Interrupt Enable"]
pub struct FHHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FHHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FHHIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FHH Interrupt is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FHHIE_A::_0)
    }
    #[doc = "FHH Interrupt is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FHHIE_A::_1)
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
#[doc = "Frequency Lower than Low Frequency Reference Threshold Asynchronous Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLAEE_A {
    #[doc = "0: Asynchronous FLL Event is Disabled"]
    _0 = 0,
    #[doc = "1: Asynchronous FLL Event is Enabled"]
    _1 = 1,
}
impl From<FLLAEE_A> for bool {
    #[inline(always)]
    fn from(variant: FLLAEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLLAEE` reader - Frequency Lower than Low Frequency Reference Threshold Asynchronous Event Enable"]
pub struct FLLAEE_R(crate::FieldReader<bool, FLLAEE_A>);
impl FLLAEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLAEE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLAEE_A {
        match self.bits {
            false => FLLAEE_A::_0,
            true => FLLAEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLLAEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLLAEE_A::_1
    }
}
impl core::ops::Deref for FLLAEE_R {
    type Target = crate::FieldReader<bool, FLLAEE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLAEE` writer - Frequency Lower than Low Frequency Reference Threshold Asynchronous Event Enable"]
pub struct FLLAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLAEE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Asynchronous FLL Event is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLLAEE_A::_0)
    }
    #[doc = "Asynchronous FLL Event is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLAEE_A::_1)
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
#[doc = "Frequency Higher than High Frequency Reference Threshold Asynchronous Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FHHAEE_A {
    #[doc = "0: Asynchronous FHH Event is Disabled"]
    _0 = 0,
    #[doc = "1: Asynchronous FHH Event is Enabled"]
    _1 = 1,
}
impl From<FHHAEE_A> for bool {
    #[inline(always)]
    fn from(variant: FHHAEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FHHAEE` reader - Frequency Higher than High Frequency Reference Threshold Asynchronous Event Enable"]
pub struct FHHAEE_R(crate::FieldReader<bool, FHHAEE_A>);
impl FHHAEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FHHAEE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FHHAEE_A {
        match self.bits {
            false => FHHAEE_A::_0,
            true => FHHAEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FHHAEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FHHAEE_A::_1
    }
}
impl core::ops::Deref for FHHAEE_R {
    type Target = crate::FieldReader<bool, FHHAEE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FHHAEE` writer - Frequency Higher than High Frequency Reference Threshold Asynchronous Event Enable"]
pub struct FHHAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> FHHAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FHHAEE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Asynchronous FHH Event is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FHHAEE_A::_0)
    }
    #[doc = "Asynchronous FHH Event is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FHHAEE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Frequency Lower than Low Frequency Reference Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn fllie(&self) -> FLLIE_R {
        FLLIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frequency Higher than High Frequency Reference Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn fhhie(&self) -> FHHIE_R {
        FHHIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Frequency Lower than Low Frequency Reference Threshold Asynchronous Event Enable"]
    #[inline(always)]
    pub fn fllaee(&self) -> FLLAEE_R {
        FLLAEE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frequency Higher than High Frequency Reference Threshold Asynchronous Event Enable"]
    #[inline(always)]
    pub fn fhhaee(&self) -> FHHAEE_R {
        FHHAEE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Lower than Low Frequency Reference Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn fllie(&mut self) -> FLLIE_W {
        FLLIE_W { w: self }
    }
    #[doc = "Bit 1 - Frequency Higher than High Frequency Reference Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn fhhie(&mut self) -> FHHIE_W {
        FHHIE_W { w: self }
    }
    #[doc = "Bit 2 - Frequency Lower than Low Frequency Reference Threshold Asynchronous Event Enable"]
    #[inline(always)]
    pub fn fllaee(&mut self) -> FLLAEE_W {
        FLLAEE_W { w: self }
    }
    #[doc = "Bit 3 - Frequency Higher than High Frequency Reference Threshold Asynchronous Event Enable"]
    #[inline(always)]
    pub fn fhhaee(&mut self) -> FHHAEE_W {
        FHHAEE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Frequency Check Interrupt/Event Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
