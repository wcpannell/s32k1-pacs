#[doc = "Register `PCC_FlexCAN2` reader"]
pub struct R(crate::R<PCC_FLEXCAN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCC_FLEXCAN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCC_FLEXCAN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCC_FLEXCAN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCC_FlexCAN2` writer"]
pub struct W(crate::W<PCC_FLEXCAN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCC_FLEXCAN2_SPEC>;
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
impl From<crate::W<PCC_FLEXCAN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCC_FLEXCAN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled. The current clock selection and divider options are locked."]
    _1 = 1,
}
impl From<CGC_A> for bool {
    #[inline(always)]
    fn from(variant: CGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGC` reader - Clock Gate Control"]
pub struct CGC_R(crate::FieldReader<bool, CGC_A>);
impl CGC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGC_A {
        match self.bits {
            false => CGC_A::_0,
            true => CGC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CGC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CGC_A::_1
    }
}
impl core::ops::Deref for CGC_R {
    type Target = crate::FieldReader<bool, CGC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGC` writer - Clock Gate Control"]
pub struct CGC_W<'a> {
    w: &'a mut W,
}
impl<'a> CGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGC_A::_0)
    }
    #[doc = "Clock enabled. The current clock selection and divider options are locked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Present\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
    #[doc = "0: Peripheral is not present."]
    _0 = 0,
    #[doc = "1: Peripheral is present."]
    _1 = 1,
}
impl From<PR_A> for bool {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR` reader - Present"]
pub struct PR_R(crate::FieldReader<bool, PR_A>);
impl PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            false => PR_A::_0,
            true => PR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PR_A::_1
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<bool, PR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline(always)]
    pub fn cgc(&self) -> CGC_R {
        CGC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Present"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline(always)]
    pub fn cgc(&mut self) -> CGC_W {
        CGC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCC FlexCAN2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_flex_can2](index.html) module"]
pub struct PCC_FLEXCAN2_SPEC;
impl crate::RegisterSpec for PCC_FLEXCAN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcc_flex_can2::R](R) reader structure"]
impl crate::Readable for PCC_FLEXCAN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcc_flex_can2::W](W) writer structure"]
impl crate::Writable for PCC_FLEXCAN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCC_FlexCAN2 to value 0x8000_0000"]
impl crate::Resettable for PCC_FLEXCAN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
