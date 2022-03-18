#[doc = "Register `SIRCCSR` reader"]
pub struct R(crate::R<SIRCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIRCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIRCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIRCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIRCCSR` writer"]
pub struct W(crate::W<SIRCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIRCCSR_SPEC>;
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
impl From<crate::W<SIRCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIRCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slow IRC Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCEN_A {
    #[doc = "0: Slow IRC is disabled"]
    _0 = 0,
    #[doc = "1: Slow IRC is enabled"]
    _1 = 1,
}
impl From<SIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRCEN` reader - Slow IRC Enable"]
pub struct SIRCEN_R(crate::FieldReader<bool, SIRCEN_A>);
impl SIRCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCEN_A {
        match self.bits {
            false => SIRCEN_A::_0,
            true => SIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIRCEN_A::_1
    }
}
impl core::ops::Deref for SIRCEN_R {
    type Target = crate::FieldReader<bool, SIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRCEN` writer - Slow IRC Enable"]
pub struct SIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow IRC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCEN_A::_0)
    }
    #[doc = "Slow IRC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCEN_A::_1)
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
#[doc = "Slow IRC Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSTEN_A {
    #[doc = "0: Slow IRC is disabled in supported Stop modes"]
    _0 = 0,
    #[doc = "1: Slow IRC is enabled in supported Stop modes"]
    _1 = 1,
}
impl From<SIRCSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIRCSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRCSTEN` reader - Slow IRC Stop Enable"]
pub struct SIRCSTEN_R(crate::FieldReader<bool, SIRCSTEN_A>);
impl SIRCSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRCSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCSTEN_A {
        match self.bits {
            false => SIRCSTEN_A::_0,
            true => SIRCSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIRCSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIRCSTEN_A::_1
    }
}
impl core::ops::Deref for SIRCSTEN_R {
    type Target = crate::FieldReader<bool, SIRCSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRCSTEN` writer - Slow IRC Stop Enable"]
pub struct SIRCSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow IRC is disabled in supported Stop modes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCSTEN_A::_0)
    }
    #[doc = "Slow IRC is enabled in supported Stop modes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCSTEN_A::_1)
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
#[doc = "Slow IRC Low Power Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCLPEN_A {
    #[doc = "0: Slow IRC is disabled in VLP modes"]
    _0 = 0,
    #[doc = "1: Slow IRC is enabled in VLP modes"]
    _1 = 1,
}
impl From<SIRCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIRCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRCLPEN` reader - Slow IRC Low Power Enable"]
pub struct SIRCLPEN_R(crate::FieldReader<bool, SIRCLPEN_A>);
impl SIRCLPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRCLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCLPEN_A {
        match self.bits {
            false => SIRCLPEN_A::_0,
            true => SIRCLPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIRCLPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIRCLPEN_A::_1
    }
}
impl core::ops::Deref for SIRCLPEN_R {
    type Target = crate::FieldReader<bool, SIRCLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRCLPEN` writer - Slow IRC Low Power Enable"]
pub struct SIRCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow IRC is disabled in VLP modes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCLPEN_A::_0)
    }
    #[doc = "Slow IRC is enabled in VLP modes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCLPEN_A::_1)
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
#[doc = "Slow IRC Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCVLD_A {
    #[doc = "0: Slow IRC is not enabled or clock is not valid"]
    _0 = 0,
    #[doc = "1: Slow IRC is enabled and output clock is valid"]
    _1 = 1,
}
impl From<SIRCVLD_A> for bool {
    #[inline(always)]
    fn from(variant: SIRCVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRCVLD` reader - Slow IRC Valid"]
pub struct SIRCVLD_R(crate::FieldReader<bool, SIRCVLD_A>);
impl SIRCVLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRCVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCVLD_A {
        match self.bits {
            false => SIRCVLD_A::_0,
            true => SIRCVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIRCVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIRCVLD_A::_1
    }
}
impl core::ops::Deref for SIRCVLD_R {
    type Target = crate::FieldReader<bool, SIRCVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slow IRC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSEL_A {
    #[doc = "0: Slow IRC is not the system clock source"]
    _0 = 0,
    #[doc = "1: Slow IRC is the system clock source"]
    _1 = 1,
}
impl From<SIRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SIRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRCSEL` reader - Slow IRC Selected"]
pub struct SIRCSEL_R(crate::FieldReader<bool, SIRCSEL_A>);
impl SIRCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCSEL_A {
        match self.bits {
            false => SIRCSEL_A::_0,
            true => SIRCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIRCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIRCSEL_A::_1
    }
}
impl core::ops::Deref for SIRCSEL_R {
    type Target = crate::FieldReader<bool, SIRCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Slow IRC Enable"]
    #[inline(always)]
    pub fn sircen(&self) -> SIRCEN_R {
        SIRCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slow IRC Stop Enable"]
    #[inline(always)]
    pub fn sircsten(&self) -> SIRCSTEN_R {
        SIRCSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slow IRC Low Power Enable"]
    #[inline(always)]
    pub fn sirclpen(&self) -> SIRCLPEN_R {
        SIRCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Slow IRC Valid"]
    #[inline(always)]
    pub fn sircvld(&self) -> SIRCVLD_R {
        SIRCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Slow IRC Selected"]
    #[inline(always)]
    pub fn sircsel(&self) -> SIRCSEL_R {
        SIRCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slow IRC Enable"]
    #[inline(always)]
    pub fn sircen(&mut self) -> SIRCEN_W {
        SIRCEN_W { w: self }
    }
    #[doc = "Bit 1 - Slow IRC Stop Enable"]
    #[inline(always)]
    pub fn sircsten(&mut self) -> SIRCSTEN_W {
        SIRCSTEN_W { w: self }
    }
    #[doc = "Bit 2 - Slow IRC Low Power Enable"]
    #[inline(always)]
    pub fn sirclpen(&mut self) -> SIRCLPEN_W {
        SIRCLPEN_W { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sirccsr](index.html) module"]
pub struct SIRCCSR_SPEC;
impl crate::RegisterSpec for SIRCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sirccsr::R](R) reader structure"]
impl crate::Readable for SIRCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sirccsr::W](W) writer structure"]
impl crate::Writable for SIRCCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIRCCSR to value 0x0100_0005"]
impl crate::Resettable for SIRCCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0005
    }
}
