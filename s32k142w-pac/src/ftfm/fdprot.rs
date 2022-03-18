#[doc = "Register `FDPROT` reader"]
pub struct R(crate::R<FDPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDPROT` writer"]
pub struct W(crate::W<FDPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDPROT_SPEC>;
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
impl From<crate::W<FDPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPROT` reader - Data Flash Region Protect"]
pub struct DPROT_R(crate::FieldReader<u8, u8>);
impl DPROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPROT` writer - Data Flash Region Protect"]
pub struct DPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&mut self) -> DPROT_W {
        DPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdprot](index.html) module"]
pub struct FDPROT_SPEC;
impl crate::RegisterSpec for FDPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fdprot::R](R) reader structure"]
impl crate::Readable for FDPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdprot::W](W) writer structure"]
impl crate::Writable for FDPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDPROT to value 0"]
impl crate::Resettable for FDPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
