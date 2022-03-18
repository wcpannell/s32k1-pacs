#[doc = "Register `YOFS` reader"]
pub struct R(crate::R<YOFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YOFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YOFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YOFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `YOFS` writer"]
pub struct W(crate::W<YOFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YOFS_SPEC>;
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
impl From<crate::W<YOFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YOFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YOFS` reader - Y offset error correction value"]
pub struct YOFS_R(crate::FieldReader<u8, u8>);
impl YOFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YOFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YOFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YOFS` writer - Y offset error correction value"]
pub struct YOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> YOFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Y offset error correction value"]
    #[inline(always)]
    pub fn yofs(&self) -> YOFS_R {
        YOFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y offset error correction value"]
    #[inline(always)]
    pub fn yofs(&mut self) -> YOFS_W {
        YOFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Y Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yofs](index.html) module"]
pub struct YOFS_SPEC;
impl crate::RegisterSpec for YOFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [yofs::R](R) reader structure"]
impl crate::Readable for YOFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [yofs::W](W) writer structure"]
impl crate::Writable for YOFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets YOFS to value 0x37"]
impl crate::Resettable for YOFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x37
    }
}
