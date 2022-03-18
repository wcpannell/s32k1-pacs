#[doc = "Register `SCFGR2` reader"]
pub struct R(crate::R<SCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGR2` writer"]
pub struct W(crate::W<SCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGR2_SPEC>;
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
impl From<crate::W<SCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKHOLD` reader - Clock Hold Time"]
pub struct CLKHOLD_R(crate::FieldReader<u8, u8>);
impl CLKHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKHOLD` writer - Clock Hold Time"]
pub struct CLKHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DATAVD` reader - Data Valid Delay"]
pub struct DATAVD_R(crate::FieldReader<u8, u8>);
impl DATAVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATAVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAVD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAVD` writer - Data Valid Delay"]
pub struct DATAVD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `FILTSCL` reader - Glitch Filter SCL"]
pub struct FILTSCL_R(crate::FieldReader<u8, u8>);
impl FILTSCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTSCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTSCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTSCL` writer - Glitch Filter SCL"]
pub struct FILTSCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `FILTSDA` reader - Glitch Filter SDA"]
pub struct FILTSDA_R(crate::FieldReader<u8, u8>);
impl FILTSDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTSDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTSDA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTSDA` writer - Glitch Filter SDA"]
pub struct FILTSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    pub fn filtscl(&self) -> FILTSCL_R {
        FILTSCL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    pub fn filtsda(&self) -> FILTSDA_R {
        FILTSDA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    pub fn clkhold(&mut self) -> CLKHOLD_W {
        CLKHOLD_W { w: self }
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DATAVD_W {
        DATAVD_W { w: self }
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    pub fn filtscl(&mut self) -> FILTSCL_W {
        FILTSCL_W { w: self }
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    pub fn filtsda(&mut self) -> FILTSDA_W {
        FILTSDA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr2](index.html) module"]
pub struct SCFGR2_SPEC;
impl crate::RegisterSpec for SCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgr2::R](R) reader structure"]
impl crate::Readable for SCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgr2::W](W) writer structure"]
impl crate::Writable for SCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCFGR2 to value 0"]
impl crate::Resettable for SCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
