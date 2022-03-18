#[doc = "Register `FLTPOL` reader"]
pub struct R(crate::R<FLTPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTPOL` writer"]
pub struct W(crate::W<FLTPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTPOL_SPEC>;
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
impl From<crate::W<FLTPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fault Input 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT0POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    FLT0POL_0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    FLT0POL_1 = 1,
}
impl From<FLT0POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT0POL` reader - Fault Input 0 Polarity"]
pub struct FLT0POL_R(crate::FieldReader<bool, FLT0POL_A>);
impl FLT0POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT0POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT0POL_A {
        match self.bits {
            false => FLT0POL_A::FLT0POL_0,
            true => FLT0POL_A::FLT0POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLT0POL_0`"]
    #[inline(always)]
    pub fn is_flt0pol_0(&self) -> bool {
        **self == FLT0POL_A::FLT0POL_0
    }
    #[doc = "Checks if the value of the field is `FLT0POL_1`"]
    #[inline(always)]
    pub fn is_flt0pol_1(&self) -> bool {
        **self == FLT0POL_A::FLT0POL_1
    }
}
impl core::ops::Deref for FLT0POL_R {
    type Target = crate::FieldReader<bool, FLT0POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT0POL` writer - Fault Input 0 Polarity"]
pub struct FLT0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT0POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt0pol_0(self) -> &'a mut W {
        self.variant(FLT0POL_A::FLT0POL_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt0pol_1(self) -> &'a mut W {
        self.variant(FLT0POL_A::FLT0POL_1)
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
#[doc = "Fault Input 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    FLT1POL_0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    FLT1POL_1 = 1,
}
impl From<FLT1POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1POL` reader - Fault Input 1 Polarity"]
pub struct FLT1POL_R(crate::FieldReader<bool, FLT1POL_A>);
impl FLT1POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT1POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1POL_A {
        match self.bits {
            false => FLT1POL_A::FLT1POL_0,
            true => FLT1POL_A::FLT1POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLT1POL_0`"]
    #[inline(always)]
    pub fn is_flt1pol_0(&self) -> bool {
        **self == FLT1POL_A::FLT1POL_0
    }
    #[doc = "Checks if the value of the field is `FLT1POL_1`"]
    #[inline(always)]
    pub fn is_flt1pol_1(&self) -> bool {
        **self == FLT1POL_A::FLT1POL_1
    }
}
impl core::ops::Deref for FLT1POL_R {
    type Target = crate::FieldReader<bool, FLT1POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1POL` writer - Fault Input 1 Polarity"]
pub struct FLT1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt1pol_0(self) -> &'a mut W {
        self.variant(FLT1POL_A::FLT1POL_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt1pol_1(self) -> &'a mut W {
        self.variant(FLT1POL_A::FLT1POL_1)
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
#[doc = "Fault Input 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT2POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    FLT2POL_0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    FLT2POL_1 = 1,
}
impl From<FLT2POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT2POL` reader - Fault Input 2 Polarity"]
pub struct FLT2POL_R(crate::FieldReader<bool, FLT2POL_A>);
impl FLT2POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT2POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT2POL_A {
        match self.bits {
            false => FLT2POL_A::FLT2POL_0,
            true => FLT2POL_A::FLT2POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLT2POL_0`"]
    #[inline(always)]
    pub fn is_flt2pol_0(&self) -> bool {
        **self == FLT2POL_A::FLT2POL_0
    }
    #[doc = "Checks if the value of the field is `FLT2POL_1`"]
    #[inline(always)]
    pub fn is_flt2pol_1(&self) -> bool {
        **self == FLT2POL_A::FLT2POL_1
    }
}
impl core::ops::Deref for FLT2POL_R {
    type Target = crate::FieldReader<bool, FLT2POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2POL` writer - Fault Input 2 Polarity"]
pub struct FLT2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt2pol_0(self) -> &'a mut W {
        self.variant(FLT2POL_A::FLT2POL_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt2pol_1(self) -> &'a mut W {
        self.variant(FLT2POL_A::FLT2POL_1)
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
#[doc = "Fault Input 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT3POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    FLT3POL_0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    FLT3POL_1 = 1,
}
impl From<FLT3POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT3POL` reader - Fault Input 3 Polarity"]
pub struct FLT3POL_R(crate::FieldReader<bool, FLT3POL_A>);
impl FLT3POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT3POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT3POL_A {
        match self.bits {
            false => FLT3POL_A::FLT3POL_0,
            true => FLT3POL_A::FLT3POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLT3POL_0`"]
    #[inline(always)]
    pub fn is_flt3pol_0(&self) -> bool {
        **self == FLT3POL_A::FLT3POL_0
    }
    #[doc = "Checks if the value of the field is `FLT3POL_1`"]
    #[inline(always)]
    pub fn is_flt3pol_1(&self) -> bool {
        **self == FLT3POL_A::FLT3POL_1
    }
}
impl core::ops::Deref for FLT3POL_R {
    type Target = crate::FieldReader<bool, FLT3POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3POL` writer - Fault Input 3 Polarity"]
pub struct FLT3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt3pol_0(self) -> &'a mut W {
        self.variant(FLT3POL_A::FLT3POL_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn flt3pol_1(self) -> &'a mut W {
        self.variant(FLT3POL_A::FLT3POL_1)
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
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&self) -> FLT0POL_R {
        FLT0POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&self) -> FLT1POL_R {
        FLT1POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&self) -> FLT2POL_R {
        FLT2POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&self) -> FLT3POL_R {
        FLT3POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&mut self) -> FLT0POL_W {
        FLT0POL_W { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&mut self) -> FLT1POL_W {
        FLT1POL_W { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&mut self) -> FLT2POL_W {
        FLT2POL_W { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&mut self) -> FLT3POL_W {
        FLT3POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Fault Input Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltpol](index.html) module"]
pub struct FLTPOL_SPEC;
impl crate::RegisterSpec for FLTPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltpol::R](R) reader structure"]
impl crate::Readable for FLTPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltpol::W](W) writer structure"]
impl crate::Writable for FLTPOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTPOL to value 0"]
impl crate::Resettable for FLTPOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
