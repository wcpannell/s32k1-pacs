#[doc = "Register `PID` reader"]
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PID` writer"]
pub struct W(crate::W<PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_SPEC>;
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
impl From<crate::W<PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - M0_PID and M1_PID for MPU"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - M0_PID and M1_PID for MPU"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - M0_PID and M1_PID for MPU"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M0_PID and M1_PID for MPU"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](index.html) module"]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid::R](R) reader structure"]
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pid::W](W) writer structure"]
impl crate::Writable for PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
