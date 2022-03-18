#[doc = "Register `SOCCR` reader"]
pub struct R(crate::R<SOCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOCCR` writer"]
pub struct W(crate::W<SOCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOCCR_SPEC>;
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
impl From<crate::W<SOCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOCCFG` reader - SOC Configuration For details, refer to chip-specific QuadSPI information."]
pub struct SOCCFG_R(crate::FieldReader<u32, u32>);
impl SOCCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SOCCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOCCFG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOCCFG` writer - SOC Configuration For details, refer to chip-specific QuadSPI information."]
pub struct SOCCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SOC Configuration For details, refer to chip-specific QuadSPI information."]
    #[inline(always)]
    pub fn soccfg(&self) -> SOCCFG_R {
        SOCCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SOC Configuration For details, refer to chip-specific QuadSPI information."]
    #[inline(always)]
    pub fn soccfg(&mut self) -> SOCCFG_W {
        SOCCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soccr](index.html) module"]
pub struct SOCCR_SPEC;
impl crate::RegisterSpec for SOCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soccr::R](R) reader structure"]
impl crate::Readable for SOCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soccr::W](W) writer structure"]
impl crate::Writable for SOCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOCCR to value 0"]
impl crate::Resettable for SOCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
