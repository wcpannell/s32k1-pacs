#[doc = "Register `UG` reader"]
pub struct R(crate::R<UG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UG` writer"]
pub struct W(crate::W<UG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UG_SPEC>;
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
impl From<crate::W<UG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` reader - UG"]
pub struct UG_R(crate::FieldReader<u16, u16>);
impl UG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UG` writer - UG"]
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - UG"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC User Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ug](index.html) module"]
pub struct UG_SPEC;
impl crate::RegisterSpec for UG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ug::R](R) reader structure"]
impl crate::Readable for UG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ug::W](W) writer structure"]
impl crate::Writable for UG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UG to value 0x04"]
impl crate::Resettable for UG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
