#[doc = "Register `FPROT0` reader"]
pub struct R(crate::R<FPROT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPROT0` writer"]
pub struct W(crate::W<FPROT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPROT0_SPEC>;
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
impl From<crate::W<FPROT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPROT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT` reader - Program Flash Region Protect"]
pub struct PROT_R(crate::FieldReader<u8, u8>);
impl PROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT` writer - Program Flash Region Protect"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot0](index.html) module"]
pub struct FPROT0_SPEC;
impl crate::RegisterSpec for FPROT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot0::R](R) reader structure"]
impl crate::Readable for FPROT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fprot0::W](W) writer structure"]
impl crate::Writable for FPROT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPROT0 to value 0"]
impl crate::Resettable for FPROT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
