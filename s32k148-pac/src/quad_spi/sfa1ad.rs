#[doc = "Register `SFA1AD` reader"]
pub struct R(crate::R<SFA1AD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFA1AD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFA1AD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFA1AD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFA1AD` writer"]
pub struct W(crate::W<SFA1AD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFA1AD_SPEC>;
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
impl From<crate::W<SFA1AD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFA1AD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPADA1` reader - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
pub struct TPADA1_R(crate::FieldReader<u32, u32>);
impl TPADA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TPADA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPADA1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPADA1` writer - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
pub struct TPADA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada1(&self) -> TPADA1_R {
        TPADA1_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada1(&mut self) -> TPADA1_W {
        TPADA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Flash A1 Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfa1ad](index.html) module"]
pub struct SFA1AD_SPEC;
impl crate::RegisterSpec for SFA1AD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfa1ad::R](R) reader structure"]
impl crate::Readable for SFA1AD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfa1ad::W](W) writer structure"]
impl crate::Writable for SFA1AD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFA1AD to value 0"]
impl crate::Resettable for SFA1AD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
