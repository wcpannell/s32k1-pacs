#[doc = "Register `NVICIABR7` reader"]
pub struct R(crate::R<NVICIABR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIABR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIABR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIABR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIABR7` writer"]
pub struct W(crate::W<NVICIABR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIABR7_SPEC>;
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
impl From<crate::W<NVICIABR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIABR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE` reader - Interrupt active flags"]
pub struct ACTIVE_R(crate::FieldReader<u32, u32>);
impl ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE` writer - Interrupt active flags"]
pub struct ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W {
        ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Active bit Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nviciabr7](index.html) module"]
pub struct NVICIABR7_SPEC;
impl crate::RegisterSpec for NVICIABR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nviciabr7::R](R) reader structure"]
impl crate::Readable for NVICIABR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nviciabr7::W](W) writer structure"]
impl crate::Writable for NVICIABR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIABR7 to value 0"]
impl crate::Resettable for NVICIABR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
