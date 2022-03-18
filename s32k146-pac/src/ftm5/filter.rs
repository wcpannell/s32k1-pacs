#[doc = "Register `FILTER` reader"]
pub struct R(crate::R<FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER` writer"]
pub struct W(crate::W<FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_SPEC>;
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
impl From<crate::W<FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0FVAL` reader - Channel 0 Input Filter"]
pub struct CH0FVAL_R(crate::FieldReader<u8, u8>);
impl CH0FVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0FVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0FVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0FVAL` writer - Channel 0 Input Filter"]
pub struct CH0FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CH1FVAL` reader - Channel 1 Input Filter"]
pub struct CH1FVAL_R(crate::FieldReader<u8, u8>);
impl CH1FVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1FVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1FVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1FVAL` writer - Channel 1 Input Filter"]
pub struct CH1FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CH2FVAL` reader - Channel 2 Input Filter"]
pub struct CH2FVAL_R(crate::FieldReader<u8, u8>);
impl CH2FVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH2FVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2FVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2FVAL` writer - Channel 2 Input Filter"]
pub struct CH2FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CH3FVAL` reader - Channel 3 Input Filter"]
pub struct CH3FVAL_R(crate::FieldReader<u8, u8>);
impl CH3FVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH3FVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3FVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3FVAL` writer - Channel 3 Input Filter"]
pub struct CH3FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&self) -> CH0FVAL_R {
        CH0FVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&self) -> CH1FVAL_R {
        CH1FVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&self) -> CH2FVAL_R {
        CH2FVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&self) -> CH3FVAL_R {
        CH3FVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&mut self) -> CH0FVAL_W {
        CH0FVAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&mut self) -> CH1FVAL_W {
        CH1FVAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&mut self) -> CH2FVAL_W {
        CH2FVAL_W { w: self }
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&mut self) -> CH3FVAL_W {
        CH3FVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Capture Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](index.html) module"]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter::R](R) reader structure"]
impl crate::Readable for FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter::W](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
