#[doc = "Register `NVICIP50` reader"]
pub struct R(crate::R<NVICIP50_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP50_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP50_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP50_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP50` writer"]
pub struct W(crate::W<NVICIP50_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP50_SPEC>;
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
impl From<crate::W<NVICIP50_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP50_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI50` reader - Priority of interrupt 50"]
pub struct PRI50_R(crate::FieldReader<u8, u8>);
impl PRI50_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI50_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI50` writer - Priority of interrupt 50"]
pub struct PRI50_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 50"]
    #[inline(always)]
    pub fn pri50(&self) -> PRI50_R {
        PRI50_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 50"]
    #[inline(always)]
    pub fn pri50(&mut self) -> PRI50_W {
        PRI50_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip50](index.html) module"]
pub struct NVICIP50_SPEC;
impl crate::RegisterSpec for NVICIP50_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip50::R](R) reader structure"]
impl crate::Readable for NVICIP50_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip50::W](W) writer structure"]
impl crate::Writable for NVICIP50_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP50 to value 0"]
impl crate::Resettable for NVICIP50_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
