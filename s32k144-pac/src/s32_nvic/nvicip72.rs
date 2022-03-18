#[doc = "Register `NVICIP72` reader"]
pub struct R(crate::R<NVICIP72_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP72_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP72_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP72_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP72` writer"]
pub struct W(crate::W<NVICIP72_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP72_SPEC>;
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
impl From<crate::W<NVICIP72_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP72_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI72` reader - Priority of interrupt 72"]
pub struct PRI72_R(crate::FieldReader<u8, u8>);
impl PRI72_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI72_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI72_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI72` writer - Priority of interrupt 72"]
pub struct PRI72_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI72_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 72"]
    #[inline(always)]
    pub fn pri72(&self) -> PRI72_R {
        PRI72_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 72"]
    #[inline(always)]
    pub fn pri72(&mut self) -> PRI72_W {
        PRI72_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip72](index.html) module"]
pub struct NVICIP72_SPEC;
impl crate::RegisterSpec for NVICIP72_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip72::R](R) reader structure"]
impl crate::Readable for NVICIP72_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip72::W](W) writer structure"]
impl crate::Writable for NVICIP72_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP72 to value 0"]
impl crate::Resettable for NVICIP72_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
