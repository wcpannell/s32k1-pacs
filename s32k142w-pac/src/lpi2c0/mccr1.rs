#[doc = "Register `MCCR1` reader"]
pub struct R(crate::R<MCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCCR1` writer"]
pub struct W(crate::W<MCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCCR1_SPEC>;
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
impl From<crate::W<MCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKLO` reader - Clock Low Period"]
pub struct CLKLO_R(crate::FieldReader<u8, u8>);
impl CLKLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKLO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKLO` writer - Clock Low Period"]
pub struct CLKLO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CLKHI` reader - Clock High Period"]
pub struct CLKHI_R(crate::FieldReader<u8, u8>);
impl CLKHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKHI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKHI` writer - Clock High Period"]
pub struct CLKHI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `SETHOLD` reader - Setup Hold Delay"]
pub struct SETHOLD_R(crate::FieldReader<u8, u8>);
impl SETHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETHOLD` writer - Setup Hold Delay"]
pub struct SETHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SETHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
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
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    pub fn clklo(&self) -> CLKLO_R {
        CLKLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    pub fn clkhi(&self) -> CLKHI_R {
        CLKHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    pub fn sethold(&self) -> SETHOLD_R {
        SETHOLD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    pub fn clklo(&mut self) -> CLKLO_W {
        CLKLO_W { w: self }
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    pub fn clkhi(&mut self) -> CLKHI_W {
        CLKHI_W { w: self }
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    pub fn sethold(&mut self) -> SETHOLD_W {
        SETHOLD_W { w: self }
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DATAVD_W {
        DATAVD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccr1](index.html) module"]
pub struct MCCR1_SPEC;
impl crate::RegisterSpec for MCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mccr1::R](R) reader structure"]
impl crate::Readable for MCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mccr1::W](W) writer structure"]
impl crate::Writable for MCCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCCR1 to value 0"]
impl crate::Resettable for MCCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
