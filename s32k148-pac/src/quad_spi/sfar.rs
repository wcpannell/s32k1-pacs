#[doc = "Register `SFAR` reader"]
pub struct R(crate::R<SFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFAR` writer"]
pub struct W(crate::W<SFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFAR_SPEC>;
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
impl From<crate::W<SFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFADR` reader - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
pub struct SFADR_R(crate::FieldReader<u32, u32>);
impl SFADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SFADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFADR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFADR` writer - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
pub struct SFADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SFADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
    #[inline(always)]
    pub fn sfadr(&self) -> SFADR_R {
        SFADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
    #[inline(always)]
    pub fn sfadr(&mut self) -> SFADR_W {
        SFADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Flash Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfar](index.html) module"]
pub struct SFAR_SPEC;
impl crate::RegisterSpec for SFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfar::R](R) reader structure"]
impl crate::Readable for SFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfar::W](W) writer structure"]
impl crate::Writable for SFAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFAR to value 0"]
impl crate::Resettable for SFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
