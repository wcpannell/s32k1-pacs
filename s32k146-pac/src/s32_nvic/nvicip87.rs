#[doc = "Register `NVICIP87` reader"]
pub struct R(crate::R<NVICIP87_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP87_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP87_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP87_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP87` writer"]
pub struct W(crate::W<NVICIP87_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP87_SPEC>;
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
impl From<crate::W<NVICIP87_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP87_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI87` reader - Priority of interrupt 87"]
pub struct PRI87_R(crate::FieldReader<u8, u8>);
impl PRI87_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI87_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI87_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI87` writer - Priority of interrupt 87"]
pub struct PRI87_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI87_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 87"]
    #[inline(always)]
    pub fn pri87(&self) -> PRI87_R {
        PRI87_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 87"]
    #[inline(always)]
    pub fn pri87(&mut self) -> PRI87_W {
        PRI87_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip87](index.html) module"]
pub struct NVICIP87_SPEC;
impl crate::RegisterSpec for NVICIP87_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip87::R](R) reader structure"]
impl crate::Readable for NVICIP87_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip87::W](W) writer structure"]
impl crate::Writable for NVICIP87_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP87 to value 0"]
impl crate::Resettable for NVICIP87_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
