#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSR` reader - Time Seconds Register"]
pub struct TSR_R(crate::FieldReader<u32, u32>);
impl TSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSR` writer - Time Seconds Register"]
pub struct TSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time Seconds Register"]
    #[inline(always)]
    pub fn tsr(&self) -> TSR_R {
        TSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time Seconds Register"]
    #[inline(always)]
    pub fn tsr(&mut self) -> TSR_W {
        TSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
