#[doc = "Register `MCFGR2` reader"]
pub struct R(crate::R<MCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR2` writer"]
pub struct W(crate::W<MCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR2_SPEC>;
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
impl From<crate::W<MCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSIDLE` reader - Bus Idle Timeout"]
pub struct BUSIDLE_R(crate::FieldReader<u16, u16>);
impl BUSIDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUSIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSIDLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSIDLE` writer - Bus Idle Timeout"]
pub struct BUSIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSIDLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
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
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn busidle(&self) -> BUSIDLE_R {
        BUSIDLE_R::new((self.bits & 0x0fff) as u16)
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
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn busidle(&mut self) -> BUSIDLE_W {
        BUSIDLE_W { w: self }
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
#[doc = "Master Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr2](index.html) module"]
pub struct MCFGR2_SPEC;
impl crate::RegisterSpec for MCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr2::R](R) reader structure"]
impl crate::Readable for MCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr2::W](W) writer structure"]
impl crate::Writable for MCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCFGR2 to value 0"]
impl crate::Resettable for MCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
