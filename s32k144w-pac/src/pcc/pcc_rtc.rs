#[doc = "Register `PCC_RTC` reader"]
pub struct R(crate::R<PCC_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCC_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCC_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCC_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCC_RTC` writer"]
pub struct W(crate::W<PCC_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCC_RTC_SPEC>;
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
impl From<crate::W<PCC_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCC_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGC_A {
    #[doc = "0: Clock disabled. The current clock selection and divider options are not locked and can be modified."]
    DISABLED = 0,
    #[doc = "1: Clock enabled. The current clock selection and divider options are locked and cannot be modified."]
    ENABLED = 1,
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
            false => CGC_A::DISABLED,
            true => CGC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CGC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CGC_A::ENABLED
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
    #[doc = "Clock disabled. The current clock selection and divider options are not locked and can be modified."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CGC_A::DISABLED)
    }
    #[doc = "Clock enabled. The current clock selection and divider options are locked and cannot be modified."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CGC_A::ENABLED)
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
    NOT_PRESENT = 0,
    #[doc = "1: Peripheral is present."]
    PRESENT = 1,
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
            false => PR_A::NOT_PRESENT,
            true => PR_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == PR_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == PR_A::PRESENT
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
#[doc = "PCC RTC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_rtc](index.html) module"]
pub struct PCC_RTC_SPEC;
impl crate::RegisterSpec for PCC_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcc_rtc::R](R) reader structure"]
impl crate::Readable for PCC_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcc_rtc::W](W) writer structure"]
impl crate::Writable for PCC_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCC_RTC to value 0x8000_0000"]
impl crate::Resettable for PCC_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
