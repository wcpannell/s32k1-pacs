#[doc = "Register `PL1_HI` reader"]
pub struct R(crate::R<PL1_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL1_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL1_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL1_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL1_HI` writer"]
pub struct W(crate::W<PL1_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL1_HI_SPEC>;
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
impl From<crate::W<PL1_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL1_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Data_byte_7` reader - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
pub struct DATA_BYTE_7_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_7` writer - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
pub struct DATA_BYTE_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Data_byte_6` reader - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
pub struct DATA_BYTE_6_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_6` writer - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
pub struct DATA_BYTE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `Data_byte_5` reader - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
pub struct DATA_BYTE_5_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_5` writer - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
pub struct DATA_BYTE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `Data_byte_4` reader - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
pub struct DATA_BYTE_4_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_4` writer - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
pub struct DATA_BYTE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
    #[inline(always)]
    pub fn data_byte_7(&self) -> DATA_BYTE_7_R {
        DATA_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
    #[inline(always)]
    pub fn data_byte_6(&self) -> DATA_BYTE_6_R {
        DATA_BYTE_6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
    #[inline(always)]
    pub fn data_byte_5(&self) -> DATA_BYTE_5_R {
        DATA_BYTE_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
    #[inline(always)]
    pub fn data_byte_4(&self) -> DATA_BYTE_4_R {
        DATA_BYTE_4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
    #[inline(always)]
    pub fn data_byte_7(&mut self) -> DATA_BYTE_7_W {
        DATA_BYTE_7_W { w: self }
    }
    #[doc = "Bits 8:15 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
    #[inline(always)]
    pub fn data_byte_6(&mut self) -> DATA_BYTE_6_W {
        DATA_BYTE_6_W { w: self }
    }
    #[doc = "Bits 16:23 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
    #[inline(always)]
    pub fn data_byte_5(&mut self) -> DATA_BYTE_5_W {
        DATA_BYTE_5_W { w: self }
    }
    #[doc = "Bits 24:31 - Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
    #[inline(always)]
    pub fn data_byte_4(&mut self) -> DATA_BYTE_4_W {
        DATA_BYTE_4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking Payload High Filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl1_hi](index.html) module"]
pub struct PL1_HI_SPEC;
impl crate::RegisterSpec for PL1_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pl1_hi::R](R) reader structure"]
impl crate::Readable for PL1_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl1_hi::W](W) writer structure"]
impl crate::Writable for PL1_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PL1_HI to value 0"]
impl crate::Resettable for PL1_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
