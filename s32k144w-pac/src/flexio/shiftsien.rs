#[doc = "Register `SHIFTSIEN` reader"]
pub struct R(crate::R<SHIFTSIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSIEN` writer"]
pub struct W(crate::W<SHIFTSIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSIEN_SPEC>;
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
impl From<crate::W<SHIFTSIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSIE` reader - Shifter Status Interrupt Enable"]
pub struct SSIE_R(crate::FieldReader<u8, u8>);
impl SSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSIE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIE` writer - Shifter Status Interrupt Enable"]
pub struct SSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W {
        SSIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsien](index.html) module"]
pub struct SHIFTSIEN_SPEC;
impl crate::RegisterSpec for SHIFTSIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftsien::R](R) reader structure"]
impl crate::Readable for SHIFTSIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftsien::W](W) writer structure"]
impl crate::Writable for SHIFTSIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTSIEN to value 0"]
impl crate::Resettable for SHIFTSIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}