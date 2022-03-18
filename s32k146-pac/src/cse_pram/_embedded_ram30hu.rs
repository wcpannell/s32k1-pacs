#[doc = "Register `_EmbeddedRAM30HU` reader"]
pub struct R(crate::R<_EMBEDDEDRAM30HU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_EMBEDDEDRAM30HU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_EMBEDDEDRAM30HU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_EMBEDDEDRAM30HU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_EmbeddedRAM30HU` writer"]
pub struct W(crate::W<_EMBEDDEDRAM30HU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_EMBEDDEDRAM30HU_SPEC>;
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
impl From<crate::W<_EMBEDDEDRAM30HU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_EMBEDDEDRAM30HU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_HU` reader - RAM_HU stores the fourth 8 bits of the 32 bit CRC"]
pub struct RAM_HU_R(crate::FieldReader<u8, u8>);
impl RAM_HU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_HU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_HU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_HU` writer - RAM_HU stores the fourth 8 bits of the 32 bit CRC"]
pub struct RAM_HU_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_HU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM_HU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn ram_hu(&self) -> RAM_HU_R {
        RAM_HU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM_HU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn ram_hu(&mut self) -> RAM_HU_W {
        RAM_HU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSE PRAM30HU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_embedded_ram30hu](index.html) module"]
pub struct _EMBEDDEDRAM30HU_SPEC;
impl crate::RegisterSpec for _EMBEDDEDRAM30HU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [_embedded_ram30hu::R](R) reader structure"]
impl crate::Readable for _EMBEDDEDRAM30HU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_embedded_ram30hu::W](W) writer structure"]
impl crate::Writable for _EMBEDDEDRAM30HU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _EmbeddedRAM30HU to value 0"]
impl crate::Resettable for _EMBEDDEDRAM30HU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
