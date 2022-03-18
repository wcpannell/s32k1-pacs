#[doc = "Register `NVICIP188` reader"]
pub struct R(crate::R<NVICIP188_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP188_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP188_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP188_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP188` writer"]
pub struct W(crate::W<NVICIP188_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP188_SPEC>;
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
impl From<crate::W<NVICIP188_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP188_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI188` reader - Priority of interrupt 188"]
pub struct PRI188_R(crate::FieldReader<u8, u8>);
impl PRI188_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI188_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI188_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI188` writer - Priority of interrupt 188"]
pub struct PRI188_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI188_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 188"]
    #[inline(always)]
    pub fn pri188(&self) -> PRI188_R {
        PRI188_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 188"]
    #[inline(always)]
    pub fn pri188(&mut self) -> PRI188_W {
        PRI188_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip188](index.html) module"]
pub struct NVICIP188_SPEC;
impl crate::RegisterSpec for NVICIP188_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip188::R](R) reader structure"]
impl crate::Readable for NVICIP188_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip188::W](W) writer structure"]
impl crate::Writable for NVICIP188_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP188 to value 0"]
impl crate::Resettable for NVICIP188_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
