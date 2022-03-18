#[doc = "Register `_EmbeddedRAM4LL` reader"]
pub struct R(crate::R<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_EmbeddedRAM4LL` writer"]
pub struct W(crate::W<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>;
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
impl From<crate::W<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_LL` reader - RAM_LL stores the first 8 bits of the 32 bit CRC"]
pub struct RAM_LL_R(crate::FieldReader<u8, u8>);
impl RAM_LL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_LL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_LL` writer - RAM_LL stores the first 8 bits of the 32 bit CRC"]
pub struct RAM_LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_LL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM_LL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn ram_ll(&self) -> RAM_LL_R {
        RAM_LL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM_LL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn ram_ll(&mut self) -> RAM_LL_W {
        RAM_LL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSE PRAM4LL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse_pram__embedded_ram4ll](index.html) module"]
pub struct CSE_PRAM__EMBEDDEDRAM4LL_SPEC;
impl crate::RegisterSpec for CSE_PRAM__EMBEDDEDRAM4LL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cse_pram__embedded_ram4ll::R](R) reader structure"]
impl crate::Readable for CSE_PRAM__EMBEDDEDRAM4LL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cse_pram__embedded_ram4ll::W](W) writer structure"]
impl crate::Writable for CSE_PRAM__EMBEDDEDRAM4LL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _EmbeddedRAM4LL to value 0"]
impl crate::Resettable for CSE_PRAM__EMBEDDEDRAM4LL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
