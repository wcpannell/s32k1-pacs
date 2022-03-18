#[doc = "Register `SAI_TCR3` reader"]
pub struct R(crate::R<SAI_TCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_TCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_TCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_TCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_TCR3` writer"]
pub struct W(crate::W<SAI_TCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_TCR3_SPEC>;
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
impl From<crate::W<SAI_TCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_TCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub struct WDFL_R(crate::FieldReader<u8, u8>);
impl WDFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub struct WDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TCE` reader - Transmit Channel Enable"]
pub struct TCE_R(crate::FieldReader<u8, u8>);
impl TCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCE` writer - Transmit Channel Enable"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CFR` writer - Channel FIFO Reset"]
pub struct CFR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&mut self) -> WDFL_W {
        WDFL_W { w: self }
    }
    #[doc = "Bits 16:19 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Bits 24:27 - Channel FIFO Reset"]
    #[inline(always)]
    pub fn cfr(&mut self) -> CFR_W {
        CFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Configuration 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_tcr3](index.html) module"]
pub struct SAI_TCR3_SPEC;
impl crate::RegisterSpec for SAI_TCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_tcr3::R](R) reader structure"]
impl crate::Readable for SAI_TCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_tcr3::W](W) writer structure"]
impl crate::Writable for SAI_TCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_TCR3 to value 0"]
impl crate::Resettable for SAI_TCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
