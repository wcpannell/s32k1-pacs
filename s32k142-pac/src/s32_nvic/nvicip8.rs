#[doc = "Register `NVICIP8` reader"]
pub struct R(crate::R<NVICIP8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP8` writer"]
pub struct W(crate::W<NVICIP8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP8_SPEC>;
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
impl From<crate::W<NVICIP8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI8` reader - Priority of interrupt 8"]
pub struct PRI8_R(crate::FieldReader<u8, u8>);
impl PRI8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI8` writer - Priority of interrupt 8"]
pub struct PRI8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn pri8(&self) -> PRI8_R {
        PRI8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn pri8(&mut self) -> PRI8_W {
        PRI8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip8](index.html) module"]
pub struct NVICIP8_SPEC;
impl crate::RegisterSpec for NVICIP8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip8::R](R) reader structure"]
impl crate::Readable for NVICIP8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip8::W](W) writer structure"]
impl crate::Writable for NVICIP8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP8 to value 0"]
impl crate::Resettable for NVICIP8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
