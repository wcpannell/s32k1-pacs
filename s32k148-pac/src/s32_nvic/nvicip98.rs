#[doc = "Register `NVICIP98` reader"]
pub struct R(crate::R<NVICIP98_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP98_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP98_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP98_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP98` writer"]
pub struct W(crate::W<NVICIP98_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP98_SPEC>;
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
impl From<crate::W<NVICIP98_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP98_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI98` reader - Priority of interrupt 98"]
pub struct PRI98_R(crate::FieldReader<u8, u8>);
impl PRI98_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI98_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI98_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI98` writer - Priority of interrupt 98"]
pub struct PRI98_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI98_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 98"]
    #[inline(always)]
    pub fn pri98(&self) -> PRI98_R {
        PRI98_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 98"]
    #[inline(always)]
    pub fn pri98(&mut self) -> PRI98_W {
        PRI98_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip98](index.html) module"]
pub struct NVICIP98_SPEC;
impl crate::RegisterSpec for NVICIP98_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip98::R](R) reader structure"]
impl crate::Readable for NVICIP98_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip98::W](W) writer structure"]
impl crate::Writable for NVICIP98_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP98 to value 0"]
impl crate::Resettable for NVICIP98_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
