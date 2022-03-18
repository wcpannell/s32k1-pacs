#[doc = "Register `NVICIP172` reader"]
pub struct R(crate::R<NVICIP172_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP172_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP172_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP172_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP172` writer"]
pub struct W(crate::W<NVICIP172_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP172_SPEC>;
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
impl From<crate::W<NVICIP172_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP172_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI172` reader - Priority of interrupt 172"]
pub struct PRI172_R(crate::FieldReader<u8, u8>);
impl PRI172_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI172_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI172_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI172` writer - Priority of interrupt 172"]
pub struct PRI172_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI172_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 172"]
    #[inline(always)]
    pub fn pri172(&self) -> PRI172_R {
        PRI172_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 172"]
    #[inline(always)]
    pub fn pri172(&mut self) -> PRI172_W {
        PRI172_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip172](index.html) module"]
pub struct NVICIP172_SPEC;
impl crate::RegisterSpec for NVICIP172_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip172::R](R) reader structure"]
impl crate::Readable for NVICIP172_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip172::W](W) writer structure"]
impl crate::Writable for NVICIP172_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP172 to value 0"]
impl crate::Resettable for NVICIP172_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
