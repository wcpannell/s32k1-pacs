#[doc = "Register `FDCBT` reader"]
pub struct R(crate::R<FDCBT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCBT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCBT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCBT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCBT` writer"]
pub struct W(crate::W<FDCBT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCBT_SPEC>;
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
impl From<crate::W<FDCBT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCBT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPSEG2` reader - Fast Phase Segment 2"]
pub struct FPSEG2_R(crate::FieldReader<u8, u8>);
impl FPSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPSEG2` writer - Fast Phase Segment 2"]
pub struct FPSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FPSEG1` reader - Fast Phase Segment 1"]
pub struct FPSEG1_R(crate::FieldReader<u8, u8>);
impl FPSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPSEG1` writer - Fast Phase Segment 1"]
pub struct FPSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `FPROPSEG` reader - Fast Propagation Segment"]
pub struct FPROPSEG_R(crate::FieldReader<u8, u8>);
impl FPROPSEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPROPSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPROPSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPROPSEG` writer - Fast Propagation Segment"]
pub struct FPROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> FPROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `FRJW` reader - Fast Resync Jump Width"]
pub struct FRJW_R(crate::FieldReader<u8, u8>);
impl FRJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRJW` writer - Fast Resync Jump Width"]
pub struct FRJW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `FPRESDIV` reader - Fast Prescaler Division Factor"]
pub struct FPRESDIV_R(crate::FieldReader<u16, u16>);
impl FPRESDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FPRESDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPRESDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRESDIV` writer - Fast Prescaler Division Factor"]
pub struct FPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Fast Phase Segment 2"]
    #[inline(always)]
    pub fn fpseg2(&self) -> FPSEG2_R {
        FPSEG2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - Fast Phase Segment 1"]
    #[inline(always)]
    pub fn fpseg1(&self) -> FPSEG1_R {
        FPSEG1_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 10:14 - Fast Propagation Segment"]
    #[inline(always)]
    pub fn fpropseg(&self) -> FPROPSEG_R {
        FPROPSEG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Fast Resync Jump Width"]
    #[inline(always)]
    pub fn frjw(&self) -> FRJW_R {
        FRJW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:29 - Fast Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpresdiv(&self) -> FPRESDIV_R {
        FPRESDIV_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fast Phase Segment 2"]
    #[inline(always)]
    pub fn fpseg2(&mut self) -> FPSEG2_W {
        FPSEG2_W { w: self }
    }
    #[doc = "Bits 5:7 - Fast Phase Segment 1"]
    #[inline(always)]
    pub fn fpseg1(&mut self) -> FPSEG1_W {
        FPSEG1_W { w: self }
    }
    #[doc = "Bits 10:14 - Fast Propagation Segment"]
    #[inline(always)]
    pub fn fpropseg(&mut self) -> FPROPSEG_W {
        FPROPSEG_W { w: self }
    }
    #[doc = "Bits 16:18 - Fast Resync Jump Width"]
    #[inline(always)]
    pub fn frjw(&mut self) -> FRJW_W {
        FRJW_W { w: self }
    }
    #[doc = "Bits 20:29 - Fast Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpresdiv(&mut self) -> FPRESDIV_W {
        FPRESDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN FD Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcbt](index.html) module"]
pub struct FDCBT_SPEC;
impl crate::RegisterSpec for FDCBT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcbt::R](R) reader structure"]
impl crate::Readable for FDCBT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcbt::W](W) writer structure"]
impl crate::Writable for FDCBT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCBT to value 0"]
impl crate::Resettable for FDCBT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
