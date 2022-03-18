#[doc = "Register `TGSR` reader"]
pub struct R(crate::R<TGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TGSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TGSR` writer"]
pub struct W(crate::W<TGSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TGSR_SPEC>;
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
impl From<crate::W<TGSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TGSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Copy Of Timer Flag For Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF0_A {
    #[doc = "0: Timer Flag for Channel 0 is clear"]
    _0 = 0,
    #[doc = "1: Timer Flag for Channel 0 is set"]
    _1 = 1,
}
impl From<TF0_A> for bool {
    #[inline(always)]
    fn from(variant: TF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF0` reader - Copy Of Timer Flag For Channel 0"]
pub struct TF0_R(crate::FieldReader<bool, TF0_A>);
impl TF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF0_A {
        match self.bits {
            false => TF0_A::_0,
            true => TF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TF0_A::_1
    }
}
impl core::ops::Deref for TF0_R {
    type Target = crate::FieldReader<bool, TF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF0` writer - Copy Of Timer Flag For Channel 0"]
pub struct TF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Flag for Channel 0 is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF0_A::_0)
    }
    #[doc = "Timer Flag for Channel 0 is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF0_A::_1)
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
#[doc = "Copy Of Timer Flag For Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF1_A {
    #[doc = "0: Timer Flag for Channel 1 is clear"]
    _0 = 0,
    #[doc = "1: Timer Flag for Channel 1 is set"]
    _1 = 1,
}
impl From<TF1_A> for bool {
    #[inline(always)]
    fn from(variant: TF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF1` reader - Copy Of Timer Flag For Channel 1"]
pub struct TF1_R(crate::FieldReader<bool, TF1_A>);
impl TF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF1_A {
        match self.bits {
            false => TF1_A::_0,
            true => TF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TF1_A::_1
    }
}
impl core::ops::Deref for TF1_R {
    type Target = crate::FieldReader<bool, TF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1` writer - Copy Of Timer Flag For Channel 1"]
pub struct TF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Flag for Channel 1 is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF1_A::_0)
    }
    #[doc = "Timer Flag for Channel 1 is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF1_A::_1)
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
#[doc = "Copy Of Timer Flag For Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF2_A {
    #[doc = "0: Timer Flag for Channel 2 is clear"]
    _0 = 0,
    #[doc = "1: Timer Flag for Channel 2 is set"]
    _1 = 1,
}
impl From<TF2_A> for bool {
    #[inline(always)]
    fn from(variant: TF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF2` reader - Copy Of Timer Flag For Channel 2"]
pub struct TF2_R(crate::FieldReader<bool, TF2_A>);
impl TF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF2_A {
        match self.bits {
            false => TF2_A::_0,
            true => TF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TF2_A::_1
    }
}
impl core::ops::Deref for TF2_R {
    type Target = crate::FieldReader<bool, TF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF2` writer - Copy Of Timer Flag For Channel 2"]
pub struct TF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Flag for Channel 2 is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF2_A::_0)
    }
    #[doc = "Timer Flag for Channel 2 is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF2_A::_1)
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
#[doc = "Copy Of Timer Flag For Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF3_A {
    #[doc = "0: Timer Flag for Channel 3 is clear"]
    _0 = 0,
    #[doc = "1: Timer Flag for Channel 3 is set"]
    _1 = 1,
}
impl From<TF3_A> for bool {
    #[inline(always)]
    fn from(variant: TF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF3` reader - Copy Of Timer Flag For Channel 3"]
pub struct TF3_R(crate::FieldReader<bool, TF3_A>);
impl TF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF3_A {
        match self.bits {
            false => TF3_A::_0,
            true => TF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TF3_A::_1
    }
}
impl core::ops::Deref for TF3_R {
    type Target = crate::FieldReader<bool, TF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF3` writer - Copy Of Timer Flag For Channel 3"]
pub struct TF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Flag for Channel 3 is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF3_A::_0)
    }
    #[doc = "Timer Flag for Channel 3 is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF3_A::_1)
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
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub fn tf0(&self) -> TF0_R {
        TF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub fn tf1(&self) -> TF1_R {
        TF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub fn tf2(&self) -> TF2_R {
        TF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub fn tf3(&self) -> TF3_R {
        TF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub fn tf0(&mut self) -> TF0_W {
        TF0_W { w: self }
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub fn tf1(&mut self) -> TF1_W {
        TF1_W { w: self }
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub fn tf2(&mut self) -> TF2_W {
        TF2_W { w: self }
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub fn tf3(&mut self) -> TF3_W {
        TF3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tgsr](index.html) module"]
pub struct TGSR_SPEC;
impl crate::RegisterSpec for TGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tgsr::R](R) reader structure"]
impl crate::Readable for TGSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tgsr::W](W) writer structure"]
impl crate::Writable for TGSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TGSR to value 0"]
impl crate::Resettable for TGSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
