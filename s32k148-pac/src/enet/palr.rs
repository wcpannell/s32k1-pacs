#[doc = "Register `PALR` reader"]
pub struct R(crate::R<PALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PALR` writer"]
pub struct W(crate::W<PALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PALR_SPEC>;
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
impl From<crate::W<PALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADDR1` reader - Pause Address"]
pub struct PADDR1_R(crate::FieldReader<u32, u32>);
impl PADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADDR1` writer - Pause Address"]
pub struct PADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pause Address"]
    #[inline(always)]
    pub fn paddr1(&self) -> PADDR1_R {
        PADDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pause Address"]
    #[inline(always)]
    pub fn paddr1(&mut self) -> PADDR1_W {
        PADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Physical Address Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palr](index.html) module"]
pub struct PALR_SPEC;
impl crate::RegisterSpec for PALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [palr::R](R) reader structure"]
impl crate::Readable for PALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [palr::W](W) writer structure"]
impl crate::Writable for PALR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PALR to value 0"]
impl crate::Resettable for PALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
