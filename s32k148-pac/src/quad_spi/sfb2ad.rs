#[doc = "Register `SFB2AD` reader"]
pub struct R(crate::R<SFB2AD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFB2AD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFB2AD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFB2AD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFB2AD` writer"]
pub struct W(crate::W<SFB2AD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFB2AD_SPEC>;
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
impl From<crate::W<SFB2AD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFB2AD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPADB2` reader - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
pub struct TPADB2_R(crate::FieldReader<u32, u32>);
impl TPADB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TPADB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPADB2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPADB2` writer - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
pub struct TPADB2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb2(&self) -> TPADB2_R {
        TPADB2_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb2(&mut self) -> TPADB2_W {
        TPADB2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Flash B2 Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfb2ad](index.html) module"]
pub struct SFB2AD_SPEC;
impl crate::RegisterSpec for SFB2AD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfb2ad::R](R) reader structure"]
impl crate::Readable for SFB2AD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfb2ad::W](W) writer structure"]
impl crate::Writable for SFB2AD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFB2AD to value 0"]
impl crate::Resettable for SFB2AD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
