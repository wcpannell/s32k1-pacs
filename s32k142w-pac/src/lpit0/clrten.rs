#[doc = "Register `CLRTEN` reader"]
pub struct R(crate::R<CLRTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRTEN` writer"]
pub struct W(crate::W<CLRTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRTEN_SPEC>;
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
impl From<crate::W<CLRTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Timer 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_0_A {
    #[doc = "0: No action"]
    CLR_T_EN_0_0 = 0,
    #[doc = "1: Clear the Timer Enable bit (TCTRL0\\[T_EN\\]) for Timer Channel 0"]
    CLR_T_EN_0_1 = 1,
}
impl From<CLR_T_EN_0_A> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_T_EN_0` reader - Clear Timer 0 Enable"]
pub struct CLR_T_EN_0_R(crate::FieldReader<bool, CLR_T_EN_0_A>);
impl CLR_T_EN_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_T_EN_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR_T_EN_0_A {
        match self.bits {
            false => CLR_T_EN_0_A::CLR_T_EN_0_0,
            true => CLR_T_EN_0_A::CLR_T_EN_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_0_0`"]
    #[inline(always)]
    pub fn is_clr_t_en_0_0(&self) -> bool {
        **self == CLR_T_EN_0_A::CLR_T_EN_0_0
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_0_1`"]
    #[inline(always)]
    pub fn is_clr_t_en_0_1(&self) -> bool {
        **self == CLR_T_EN_0_A::CLR_T_EN_0_1
    }
}
impl core::ops::Deref for CLR_T_EN_0_R {
    type Target = crate::FieldReader<bool, CLR_T_EN_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_T_EN_0` writer - Clear Timer 0 Enable"]
pub struct CLR_T_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn clr_t_en_0_0(self) -> &'a mut W {
        self.variant(CLR_T_EN_0_A::CLR_T_EN_0_0)
    }
    #[doc = "Clear the Timer Enable bit (TCTRL0\\[T_EN\\]) for Timer Channel 0"]
    #[inline(always)]
    pub fn clr_t_en_0_1(self) -> &'a mut W {
        self.variant(CLR_T_EN_0_A::CLR_T_EN_0_1)
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
#[doc = "Clear Timer 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_1_A {
    #[doc = "0: No Action"]
    CLR_T_EN_1_0 = 0,
    #[doc = "1: Clear the Timer Enable bit (TCTRL1\\[T_EN\\]) for Timer Channel 1"]
    CLR_T_EN_1_1 = 1,
}
impl From<CLR_T_EN_1_A> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_T_EN_1` reader - Clear Timer 1 Enable"]
pub struct CLR_T_EN_1_R(crate::FieldReader<bool, CLR_T_EN_1_A>);
impl CLR_T_EN_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_T_EN_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR_T_EN_1_A {
        match self.bits {
            false => CLR_T_EN_1_A::CLR_T_EN_1_0,
            true => CLR_T_EN_1_A::CLR_T_EN_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_1_0`"]
    #[inline(always)]
    pub fn is_clr_t_en_1_0(&self) -> bool {
        **self == CLR_T_EN_1_A::CLR_T_EN_1_0
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_1_1`"]
    #[inline(always)]
    pub fn is_clr_t_en_1_1(&self) -> bool {
        **self == CLR_T_EN_1_A::CLR_T_EN_1_1
    }
}
impl core::ops::Deref for CLR_T_EN_1_R {
    type Target = crate::FieldReader<bool, CLR_T_EN_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_T_EN_1` writer - Clear Timer 1 Enable"]
pub struct CLR_T_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn clr_t_en_1_0(self) -> &'a mut W {
        self.variant(CLR_T_EN_1_A::CLR_T_EN_1_0)
    }
    #[doc = "Clear the Timer Enable bit (TCTRL1\\[T_EN\\]) for Timer Channel 1"]
    #[inline(always)]
    pub fn clr_t_en_1_1(self) -> &'a mut W {
        self.variant(CLR_T_EN_1_A::CLR_T_EN_1_1)
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
#[doc = "Clear Timer 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_2_A {
    #[doc = "0: No Action"]
    CLR_T_EN_2_0 = 0,
    #[doc = "1: Clear the Timer Enable bit (TCTRL2\\[T_EN\\]) for Timer Channel 2"]
    CLR_T_EN_2_1 = 1,
}
impl From<CLR_T_EN_2_A> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_T_EN_2` reader - Clear Timer 2 Enable"]
pub struct CLR_T_EN_2_R(crate::FieldReader<bool, CLR_T_EN_2_A>);
impl CLR_T_EN_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_T_EN_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR_T_EN_2_A {
        match self.bits {
            false => CLR_T_EN_2_A::CLR_T_EN_2_0,
            true => CLR_T_EN_2_A::CLR_T_EN_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_2_0`"]
    #[inline(always)]
    pub fn is_clr_t_en_2_0(&self) -> bool {
        **self == CLR_T_EN_2_A::CLR_T_EN_2_0
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_2_1`"]
    #[inline(always)]
    pub fn is_clr_t_en_2_1(&self) -> bool {
        **self == CLR_T_EN_2_A::CLR_T_EN_2_1
    }
}
impl core::ops::Deref for CLR_T_EN_2_R {
    type Target = crate::FieldReader<bool, CLR_T_EN_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_T_EN_2` writer - Clear Timer 2 Enable"]
pub struct CLR_T_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn clr_t_en_2_0(self) -> &'a mut W {
        self.variant(CLR_T_EN_2_A::CLR_T_EN_2_0)
    }
    #[doc = "Clear the Timer Enable bit (TCTRL2\\[T_EN\\]) for Timer Channel 2"]
    #[inline(always)]
    pub fn clr_t_en_2_1(self) -> &'a mut W {
        self.variant(CLR_T_EN_2_A::CLR_T_EN_2_1)
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
#[doc = "Clear Timer 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_3_A {
    #[doc = "0: No Action"]
    CLR_T_EN_3_0 = 0,
    #[doc = "1: Clear the Timer Enable bit (TCTRL3\\[T_EN\\]) for Timer Channel 3"]
    CLR_T_EN_3_1 = 1,
}
impl From<CLR_T_EN_3_A> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_T_EN_3` reader - Clear Timer 3 Enable"]
pub struct CLR_T_EN_3_R(crate::FieldReader<bool, CLR_T_EN_3_A>);
impl CLR_T_EN_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_T_EN_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR_T_EN_3_A {
        match self.bits {
            false => CLR_T_EN_3_A::CLR_T_EN_3_0,
            true => CLR_T_EN_3_A::CLR_T_EN_3_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_3_0`"]
    #[inline(always)]
    pub fn is_clr_t_en_3_0(&self) -> bool {
        **self == CLR_T_EN_3_A::CLR_T_EN_3_0
    }
    #[doc = "Checks if the value of the field is `CLR_T_EN_3_1`"]
    #[inline(always)]
    pub fn is_clr_t_en_3_1(&self) -> bool {
        **self == CLR_T_EN_3_A::CLR_T_EN_3_1
    }
}
impl core::ops::Deref for CLR_T_EN_3_R {
    type Target = crate::FieldReader<bool, CLR_T_EN_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_T_EN_3` writer - Clear Timer 3 Enable"]
pub struct CLR_T_EN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn clr_t_en_3_0(self) -> &'a mut W {
        self.variant(CLR_T_EN_3_A::CLR_T_EN_3_0)
    }
    #[doc = "Clear the Timer Enable bit (TCTRL3\\[T_EN\\]) for Timer Channel 3"]
    #[inline(always)]
    pub fn clr_t_en_3_1(self) -> &'a mut W {
        self.variant(CLR_T_EN_3_A::CLR_T_EN_3_1)
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
    #[doc = "Bit 0 - Clear Timer 0 Enable"]
    #[inline(always)]
    pub fn clr_t_en_0(&self) -> CLR_T_EN_0_R {
        CLR_T_EN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear Timer 1 Enable"]
    #[inline(always)]
    pub fn clr_t_en_1(&self) -> CLR_T_EN_1_R {
        CLR_T_EN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear Timer 2 Enable"]
    #[inline(always)]
    pub fn clr_t_en_2(&self) -> CLR_T_EN_2_R {
        CLR_T_EN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Timer 3 Enable"]
    #[inline(always)]
    pub fn clr_t_en_3(&self) -> CLR_T_EN_3_R {
        CLR_T_EN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Timer 0 Enable"]
    #[inline(always)]
    pub fn clr_t_en_0(&mut self) -> CLR_T_EN_0_W {
        CLR_T_EN_0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Timer 1 Enable"]
    #[inline(always)]
    pub fn clr_t_en_1(&mut self) -> CLR_T_EN_1_W {
        CLR_T_EN_1_W { w: self }
    }
    #[doc = "Bit 2 - Clear Timer 2 Enable"]
    #[inline(always)]
    pub fn clr_t_en_2(&mut self) -> CLR_T_EN_2_W {
        CLR_T_EN_2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Timer 3 Enable"]
    #[inline(always)]
    pub fn clr_t_en_3(&mut self) -> CLR_T_EN_3_W {
        CLR_T_EN_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Timer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrten](index.html) module"]
pub struct CLRTEN_SPEC;
impl crate::RegisterSpec for CLRTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrten::R](R) reader structure"]
impl crate::Readable for CLRTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrten::W](W) writer structure"]
impl crate::Writable for CLRTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRTEN to value 0"]
impl crate::Resettable for CLRTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
