#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSR` writer"]
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF0_A {
    #[doc = "0: Timer has not timed out"]
    TIF0_0 = 0,
    #[doc = "1: Timeout has occurred (timer has timed out)"]
    TIF0_1 = 1,
}
impl From<TIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF0` reader - Channel 0 Timer Interrupt Flag"]
pub struct TIF0_R(crate::FieldReader<bool, TIF0_A>);
impl TIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF0_A {
        match self.bits {
            false => TIF0_A::TIF0_0,
            true => TIF0_A::TIF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIF0_0`"]
    #[inline(always)]
    pub fn is_tif0_0(&self) -> bool {
        **self == TIF0_A::TIF0_0
    }
    #[doc = "Checks if the value of the field is `TIF0_1`"]
    #[inline(always)]
    pub fn is_tif0_1(&self) -> bool {
        **self == TIF0_A::TIF0_1
    }
}
impl core::ops::Deref for TIF0_R {
    type Target = crate::FieldReader<bool, TIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF0` writer - Channel 0 Timer Interrupt Flag"]
pub struct TIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn tif0_0(self) -> &'a mut W {
        self.variant(TIF0_A::TIF0_0)
    }
    #[doc = "Timeout has occurred (timer has timed out)"]
    #[inline(always)]
    pub fn tif0_1(self) -> &'a mut W {
        self.variant(TIF0_A::TIF0_1)
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
#[doc = "Channel 1 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF1_A {
    #[doc = "0: Timer has not timed out"]
    TIF1_0 = 0,
    #[doc = "1: Timeout has occurred (timer has timed out)"]
    TIF1_1 = 1,
}
impl From<TIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF1` reader - Channel 1 Timer Interrupt Flag"]
pub struct TIF1_R(crate::FieldReader<bool, TIF1_A>);
impl TIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF1_A {
        match self.bits {
            false => TIF1_A::TIF1_0,
            true => TIF1_A::TIF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIF1_0`"]
    #[inline(always)]
    pub fn is_tif1_0(&self) -> bool {
        **self == TIF1_A::TIF1_0
    }
    #[doc = "Checks if the value of the field is `TIF1_1`"]
    #[inline(always)]
    pub fn is_tif1_1(&self) -> bool {
        **self == TIF1_A::TIF1_1
    }
}
impl core::ops::Deref for TIF1_R {
    type Target = crate::FieldReader<bool, TIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF1` writer - Channel 1 Timer Interrupt Flag"]
pub struct TIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn tif1_0(self) -> &'a mut W {
        self.variant(TIF1_A::TIF1_0)
    }
    #[doc = "Timeout has occurred (timer has timed out)"]
    #[inline(always)]
    pub fn tif1_1(self) -> &'a mut W {
        self.variant(TIF1_A::TIF1_1)
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
#[doc = "Channel 2 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF2_A {
    #[doc = "0: Timer has not timed out"]
    TIF2_0 = 0,
    #[doc = "1: Timeout has occurred (timer has timed out)"]
    TIF2_1 = 1,
}
impl From<TIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF2` reader - Channel 2 Timer Interrupt Flag"]
pub struct TIF2_R(crate::FieldReader<bool, TIF2_A>);
impl TIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF2_A {
        match self.bits {
            false => TIF2_A::TIF2_0,
            true => TIF2_A::TIF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIF2_0`"]
    #[inline(always)]
    pub fn is_tif2_0(&self) -> bool {
        **self == TIF2_A::TIF2_0
    }
    #[doc = "Checks if the value of the field is `TIF2_1`"]
    #[inline(always)]
    pub fn is_tif2_1(&self) -> bool {
        **self == TIF2_A::TIF2_1
    }
}
impl core::ops::Deref for TIF2_R {
    type Target = crate::FieldReader<bool, TIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF2` writer - Channel 2 Timer Interrupt Flag"]
pub struct TIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn tif2_0(self) -> &'a mut W {
        self.variant(TIF2_A::TIF2_0)
    }
    #[doc = "Timeout has occurred (timer has timed out)"]
    #[inline(always)]
    pub fn tif2_1(self) -> &'a mut W {
        self.variant(TIF2_A::TIF2_1)
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
#[doc = "Channel 3 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF3_A {
    #[doc = "0: Timer has not timed out"]
    TIF3_0 = 0,
    #[doc = "1: Timeout has occurred (timer has timed out)"]
    TIF3_1 = 1,
}
impl From<TIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF3` reader - Channel 3 Timer Interrupt Flag"]
pub struct TIF3_R(crate::FieldReader<bool, TIF3_A>);
impl TIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF3_A {
        match self.bits {
            false => TIF3_A::TIF3_0,
            true => TIF3_A::TIF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIF3_0`"]
    #[inline(always)]
    pub fn is_tif3_0(&self) -> bool {
        **self == TIF3_A::TIF3_0
    }
    #[doc = "Checks if the value of the field is `TIF3_1`"]
    #[inline(always)]
    pub fn is_tif3_1(&self) -> bool {
        **self == TIF3_A::TIF3_1
    }
}
impl core::ops::Deref for TIF3_R {
    type Target = crate::FieldReader<bool, TIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF3` writer - Channel 3 Timer Interrupt Flag"]
pub struct TIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn tif3_0(self) -> &'a mut W {
        self.variant(TIF3_A::TIF3_0)
    }
    #[doc = "Timeout has occurred (timer has timed out)"]
    #[inline(always)]
    pub fn tif3_1(self) -> &'a mut W {
        self.variant(TIF3_A::TIF3_1)
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif0(&self) -> TIF0_R {
        TIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif1(&self) -> TIF1_R {
        TIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif2(&self) -> TIF2_R {
        TIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif3(&self) -> TIF3_R {
        TIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif0(&mut self) -> TIF0_W {
        TIF0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif1(&mut self) -> TIF1_W {
        TIF1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif2(&mut self) -> TIF2_W {
        TIF2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif3(&mut self) -> TIF3_W {
        TIF3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msr::W](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
