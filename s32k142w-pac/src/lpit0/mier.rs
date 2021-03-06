#[doc = "Register `MIER` reader"]
pub struct R(crate::R<MIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIER` writer"]
pub struct W(crate::W<MIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIER_SPEC>;
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
impl From<crate::W<MIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE0_A {
    #[doc = "0: Disabled"]
    TIE0_0 = 0,
    #[doc = "1: Enabled"]
    TIE0_1 = 1,
}
impl From<TIE0_A> for bool {
    #[inline(always)]
    fn from(variant: TIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE0` reader - Channel 0 Timer Interrupt Enable"]
pub struct TIE0_R(crate::FieldReader<bool, TIE0_A>);
impl TIE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE0_A {
        match self.bits {
            false => TIE0_A::TIE0_0,
            true => TIE0_A::TIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE0_0`"]
    #[inline(always)]
    pub fn is_tie0_0(&self) -> bool {
        **self == TIE0_A::TIE0_0
    }
    #[doc = "Checks if the value of the field is `TIE0_1`"]
    #[inline(always)]
    pub fn is_tie0_1(&self) -> bool {
        **self == TIE0_A::TIE0_1
    }
}
impl core::ops::Deref for TIE0_R {
    type Target = crate::FieldReader<bool, TIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE0` writer - Channel 0 Timer Interrupt Enable"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tie0_0(self) -> &'a mut W {
        self.variant(TIE0_A::TIE0_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tie0_1(self) -> &'a mut W {
        self.variant(TIE0_A::TIE0_1)
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
#[doc = "Channel 1 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE1_A {
    #[doc = "0: Disabled"]
    TIE1_0 = 0,
    #[doc = "1: Enabled"]
    TIE1_1 = 1,
}
impl From<TIE1_A> for bool {
    #[inline(always)]
    fn from(variant: TIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE1` reader - Channel 1 Timer Interrupt Enable"]
pub struct TIE1_R(crate::FieldReader<bool, TIE1_A>);
impl TIE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE1_A {
        match self.bits {
            false => TIE1_A::TIE1_0,
            true => TIE1_A::TIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE1_0`"]
    #[inline(always)]
    pub fn is_tie1_0(&self) -> bool {
        **self == TIE1_A::TIE1_0
    }
    #[doc = "Checks if the value of the field is `TIE1_1`"]
    #[inline(always)]
    pub fn is_tie1_1(&self) -> bool {
        **self == TIE1_A::TIE1_1
    }
}
impl core::ops::Deref for TIE1_R {
    type Target = crate::FieldReader<bool, TIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE1` writer - Channel 1 Timer Interrupt Enable"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tie1_0(self) -> &'a mut W {
        self.variant(TIE1_A::TIE1_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tie1_1(self) -> &'a mut W {
        self.variant(TIE1_A::TIE1_1)
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
#[doc = "Channel 2 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE2_A {
    #[doc = "0: Disabled"]
    TIE2_0 = 0,
    #[doc = "1: Enabled"]
    TIE2_1 = 1,
}
impl From<TIE2_A> for bool {
    #[inline(always)]
    fn from(variant: TIE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE2` reader - Channel 2 Timer Interrupt Enable"]
pub struct TIE2_R(crate::FieldReader<bool, TIE2_A>);
impl TIE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE2_A {
        match self.bits {
            false => TIE2_A::TIE2_0,
            true => TIE2_A::TIE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE2_0`"]
    #[inline(always)]
    pub fn is_tie2_0(&self) -> bool {
        **self == TIE2_A::TIE2_0
    }
    #[doc = "Checks if the value of the field is `TIE2_1`"]
    #[inline(always)]
    pub fn is_tie2_1(&self) -> bool {
        **self == TIE2_A::TIE2_1
    }
}
impl core::ops::Deref for TIE2_R {
    type Target = crate::FieldReader<bool, TIE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE2` writer - Channel 2 Timer Interrupt Enable"]
pub struct TIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tie2_0(self) -> &'a mut W {
        self.variant(TIE2_A::TIE2_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tie2_1(self) -> &'a mut W {
        self.variant(TIE2_A::TIE2_1)
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
#[doc = "Channel 3 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE3_A {
    #[doc = "0: Disabled"]
    TIE3_0 = 0,
    #[doc = "1: Enabled"]
    TIE3_1 = 1,
}
impl From<TIE3_A> for bool {
    #[inline(always)]
    fn from(variant: TIE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE3` reader - Channel 3 Timer Interrupt Enable"]
pub struct TIE3_R(crate::FieldReader<bool, TIE3_A>);
impl TIE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE3_A {
        match self.bits {
            false => TIE3_A::TIE3_0,
            true => TIE3_A::TIE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE3_0`"]
    #[inline(always)]
    pub fn is_tie3_0(&self) -> bool {
        **self == TIE3_A::TIE3_0
    }
    #[doc = "Checks if the value of the field is `TIE3_1`"]
    #[inline(always)]
    pub fn is_tie3_1(&self) -> bool {
        **self == TIE3_A::TIE3_1
    }
}
impl core::ops::Deref for TIE3_R {
    type Target = crate::FieldReader<bool, TIE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE3` writer - Channel 3 Timer Interrupt Enable"]
pub struct TIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tie3_0(self) -> &'a mut W {
        self.variant(TIE3_A::TIE3_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tie3_1(self) -> &'a mut W {
        self.variant(TIE3_A::TIE3_1)
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie2(&mut self) -> TIE2_W {
        TIE2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie3(&mut self) -> TIE3_W {
        TIE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mier](index.html) module"]
pub struct MIER_SPEC;
impl crate::RegisterSpec for MIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mier::R](R) reader structure"]
impl crate::Readable for MIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mier::W](W) writer structure"]
impl crate::Writable for MIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIER to value 0"]
impl crate::Resettable for MIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
