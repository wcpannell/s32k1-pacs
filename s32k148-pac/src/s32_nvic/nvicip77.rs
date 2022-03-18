#[doc = "Register `NVICIP77` reader"]
pub struct R(crate::R<NVICIP77_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP77_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP77_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP77_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP77` writer"]
pub struct W(crate::W<NVICIP77_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP77_SPEC>;
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
impl From<crate::W<NVICIP77_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP77_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI77` reader - Priority of interrupt 77"]
pub struct PRI77_R(crate::FieldReader<u8, u8>);
impl PRI77_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI77_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI77_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI77` writer - Priority of interrupt 77"]
pub struct PRI77_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI77_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 77"]
    #[inline(always)]
    pub fn pri77(&self) -> PRI77_R {
        PRI77_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 77"]
    #[inline(always)]
    pub fn pri77(&mut self) -> PRI77_W {
        PRI77_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip77](index.html) module"]
pub struct NVICIP77_SPEC;
impl crate::RegisterSpec for NVICIP77_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip77::R](R) reader structure"]
impl crate::Readable for NVICIP77_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip77::W](W) writer structure"]
impl crate::Writable for NVICIP77_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP77 to value 0"]
impl crate::Resettable for NVICIP77_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
