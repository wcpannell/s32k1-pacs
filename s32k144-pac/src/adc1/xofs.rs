#[doc = "Register `XOFS` reader"]
pub struct R(crate::R<XOFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOFS` writer"]
pub struct W(crate::W<XOFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOFS_SPEC>;
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
impl From<crate::W<XOFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOFS` reader - X offset error correction value"]
pub struct XOFS_R(crate::FieldReader<u8, u8>);
impl XOFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XOFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFS` writer - X offset error correction value"]
pub struct XOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - X offset error correction value"]
    #[inline(always)]
    pub fn xofs(&self) -> XOFS_R {
        XOFS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - X offset error correction value"]
    #[inline(always)]
    pub fn xofs(&mut self) -> XOFS_W {
        XOFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC X Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xofs](index.html) module"]
pub struct XOFS_SPEC;
impl crate::RegisterSpec for XOFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xofs::R](R) reader structure"]
impl crate::Readable for XOFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xofs::W](W) writer structure"]
impl crate::Writable for XOFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOFS to value 0x30"]
impl crate::Resettable for XOFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
