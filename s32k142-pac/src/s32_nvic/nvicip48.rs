#[doc = "Register `NVICIP48` reader"]
pub struct R(crate::R<NVICIP48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP48` writer"]
pub struct W(crate::W<NVICIP48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP48_SPEC>;
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
impl From<crate::W<NVICIP48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI48` reader - Priority of interrupt 48"]
pub struct PRI48_R(crate::FieldReader<u8, u8>);
impl PRI48_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI48_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI48` writer - Priority of interrupt 48"]
pub struct PRI48_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 48"]
    #[inline(always)]
    pub fn pri48(&self) -> PRI48_R {
        PRI48_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 48"]
    #[inline(always)]
    pub fn pri48(&mut self) -> PRI48_W {
        PRI48_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip48](index.html) module"]
pub struct NVICIP48_SPEC;
impl crate::RegisterSpec for NVICIP48_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip48::R](R) reader structure"]
impl crate::Readable for NVICIP48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip48::W](W) writer structure"]
impl crate::Writable for NVICIP48_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP48 to value 0"]
impl crate::Resettable for NVICIP48_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
