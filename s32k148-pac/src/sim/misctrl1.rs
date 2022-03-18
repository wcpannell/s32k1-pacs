#[doc = "Register `MISCTRL1` reader"]
pub struct R(crate::R<MISCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCTRL1` writer"]
pub struct W(crate::W<MISCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCTRL1_SPEC>;
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
impl From<crate::W<MISCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_TRG` reader - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
pub struct SW_TRG_R(crate::FieldReader<bool, bool>);
impl SW_TRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_TRG` writer - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
pub struct SW_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TRG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
    #[inline(always)]
    pub fn sw_trg(&self) -> SW_TRG_R {
        SW_TRG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
    #[inline(always)]
    pub fn sw_trg(&mut self) -> SW_TRG_W {
        SW_TRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misctrl1](index.html) module"]
pub struct MISCTRL1_SPEC;
impl crate::RegisterSpec for MISCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misctrl1::R](R) reader structure"]
impl crate::Readable for MISCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misctrl1::W](W) writer structure"]
impl crate::Writable for MISCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISCTRL1 to value 0"]
impl crate::Resettable for MISCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
