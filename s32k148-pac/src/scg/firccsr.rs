#[doc = "Register `FIRCCSR` reader"]
pub struct R(crate::R<FIRCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIRCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIRCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIRCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIRCCSR` writer"]
pub struct W(crate::W<FIRCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRCCSR_SPEC>;
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
impl From<crate::W<FIRCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fast IRC Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCEN_A {
    #[doc = "0: Fast IRC is disabled"]
    _0 = 0,
    #[doc = "1: Fast IRC is enabled"]
    _1 = 1,
}
impl From<FIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRCEN` reader - Fast IRC Enable"]
pub struct FIRCEN_R(crate::FieldReader<bool, FIRCEN_A>);
impl FIRCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCEN_A {
        match self.bits {
            false => FIRCEN_A::_0,
            true => FIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FIRCEN_A::_1
    }
}
impl core::ops::Deref for FIRCEN_R {
    type Target = crate::FieldReader<bool, FIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIRCEN` writer - Fast IRC Enable"]
pub struct FIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fast IRC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCEN_A::_0)
    }
    #[doc = "Fast IRC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCEN_A::_1)
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
#[doc = "Fast IRC Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCREGOFF_A {
    #[doc = "0: Fast IRC Regulator is enabled."]
    _0 = 0,
    #[doc = "1: Fast IRC Regulator is disabled."]
    _1 = 1,
}
impl From<FIRCREGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FIRCREGOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRCREGOFF` reader - Fast IRC Regulator Enable"]
pub struct FIRCREGOFF_R(crate::FieldReader<bool, FIRCREGOFF_A>);
impl FIRCREGOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRCREGOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCREGOFF_A {
        match self.bits {
            false => FIRCREGOFF_A::_0,
            true => FIRCREGOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FIRCREGOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FIRCREGOFF_A::_1
    }
}
impl core::ops::Deref for FIRCREGOFF_R {
    type Target = crate::FieldReader<bool, FIRCREGOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIRCREGOFF` writer - Fast IRC Regulator Enable"]
pub struct FIRCREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRCREGOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIRCREGOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fast IRC Regulator is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCREGOFF_A::_0)
    }
    #[doc = "Fast IRC Regulator is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCREGOFF_A::_1)
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
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: Control Status Register can be written."]
    _0 = 0,
    #[doc = "1: Control Status Register cannot be written."]
    _1 = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - Lock Register"]
pub struct LK_R(crate::FieldReader<bool, LK_A>);
impl LK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::_0,
            true => LK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LK_A::_1
    }
}
impl core::ops::Deref for LK_R {
    type Target = crate::FieldReader<bool, LK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LK` writer - Lock Register"]
pub struct LK_W<'a> {
    w: &'a mut W,
}
impl<'a> LK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Control Status Register can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "Control Status Register cannot be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Fast IRC Valid status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCVLD_A {
    #[doc = "0: Fast IRC is not enabled or clock is not valid."]
    _0 = 0,
    #[doc = "1: Fast IRC is enabled and output clock is valid. The clock is valid once there is an output clock from the FIRC analog."]
    _1 = 1,
}
impl From<FIRCVLD_A> for bool {
    #[inline(always)]
    fn from(variant: FIRCVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRCVLD` reader - Fast IRC Valid status"]
pub struct FIRCVLD_R(crate::FieldReader<bool, FIRCVLD_A>);
impl FIRCVLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRCVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCVLD_A {
        match self.bits {
            false => FIRCVLD_A::_0,
            true => FIRCVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FIRCVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FIRCVLD_A::_1
    }
}
impl core::ops::Deref for FIRCVLD_R {
    type Target = crate::FieldReader<bool, FIRCVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast IRC Selected status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCSEL_A {
    #[doc = "0: Fast IRC is not the system clock source"]
    _0 = 0,
    #[doc = "1: Fast IRC is the system clock source"]
    _1 = 1,
}
impl From<FIRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FIRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRCSEL` reader - Fast IRC Selected status"]
pub struct FIRCSEL_R(crate::FieldReader<bool, FIRCSEL_A>);
impl FIRCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCSEL_A {
        match self.bits {
            false => FIRCSEL_A::_0,
            true => FIRCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FIRCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FIRCSEL_A::_1
    }
}
impl core::ops::Deref for FIRCSEL_R {
    type Target = crate::FieldReader<bool, FIRCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast IRC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCERR_A {
    #[doc = "0: Error not detected with the Fast IRC trimming."]
    _0 = 0,
    #[doc = "1: Error detected with the Fast IRC trimming."]
    _1 = 1,
}
impl From<FIRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: FIRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRCERR` reader - Fast IRC Clock Error"]
pub struct FIRCERR_R(crate::FieldReader<bool, FIRCERR_A>);
impl FIRCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCERR_A {
        match self.bits {
            false => FIRCERR_A::_0,
            true => FIRCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FIRCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FIRCERR_A::_1
    }
}
impl core::ops::Deref for FIRCERR_R {
    type Target = crate::FieldReader<bool, FIRCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIRCERR` writer - Fast IRC Clock Error"]
pub struct FIRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIRCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error not detected with the Fast IRC trimming."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCERR_A::_0)
    }
    #[doc = "Error detected with the Fast IRC trimming."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fast IRC Enable"]
    #[inline(always)]
    pub fn fircen(&self) -> FIRCEN_R {
        FIRCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast IRC Regulator Enable"]
    #[inline(always)]
    pub fn fircregoff(&self) -> FIRCREGOFF_R {
        FIRCREGOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast IRC Valid status"]
    #[inline(always)]
    pub fn fircvld(&self) -> FIRCVLD_R {
        FIRCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast IRC Selected status"]
    #[inline(always)]
    pub fn fircsel(&self) -> FIRCSEL_R {
        FIRCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast IRC Clock Error"]
    #[inline(always)]
    pub fn fircerr(&self) -> FIRCERR_R {
        FIRCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast IRC Enable"]
    #[inline(always)]
    pub fn fircen(&mut self) -> FIRCEN_W {
        FIRCEN_W { w: self }
    }
    #[doc = "Bit 3 - Fast IRC Regulator Enable"]
    #[inline(always)]
    pub fn fircregoff(&mut self) -> FIRCREGOFF_W {
        FIRCREGOFF_W { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Bit 26 - Fast IRC Clock Error"]
    #[inline(always)]
    pub fn fircerr(&mut self) -> FIRCERR_W {
        FIRCERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firccsr](index.html) module"]
pub struct FIRCCSR_SPEC;
impl crate::RegisterSpec for FIRCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [firccsr::R](R) reader structure"]
impl crate::Readable for FIRCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [firccsr::W](W) writer structure"]
impl crate::Writable for FIRCCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIRCCSR to value 0x0300_0001"]
impl crate::Resettable for FIRCCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0001
    }
}
