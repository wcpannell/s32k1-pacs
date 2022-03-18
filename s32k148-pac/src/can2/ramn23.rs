#[doc = "Register `RAMn23` reader"]
pub struct R(crate::R<RAMN23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMN23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMN23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMN23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMn23` writer"]
pub struct W(crate::W<RAMN23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMN23_SPEC>;
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
impl From<crate::W<RAMN23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMN23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BYTE_3` reader - Data byte 3 of Rx/Tx frame."]
pub struct DATA_BYTE_3_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BYTE_3` writer - Data byte 3 of Rx/Tx frame."]
pub struct DATA_BYTE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DATA_BYTE_2` reader - Data byte 2 of Rx/Tx frame."]
pub struct DATA_BYTE_2_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BYTE_2` writer - Data byte 2 of Rx/Tx frame."]
pub struct DATA_BYTE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DATA_BYTE_1` reader - Data byte 1 of Rx/Tx frame."]
pub struct DATA_BYTE_1_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BYTE_1` writer - Data byte 1 of Rx/Tx frame."]
pub struct DATA_BYTE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DATA_BYTE_0` reader - Data byte 0 of Rx/Tx frame."]
pub struct DATA_BYTE_0_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BYTE_0` writer - Data byte 0 of Rx/Tx frame."]
pub struct DATA_BYTE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_3(&self) -> DATA_BYTE_3_R {
        DATA_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_2(&self) -> DATA_BYTE_2_R {
        DATA_BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_1(&self) -> DATA_BYTE_1_R {
        DATA_BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_0(&self) -> DATA_BYTE_0_R {
        DATA_BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_3(&mut self) -> DATA_BYTE_3_W {
        DATA_BYTE_3_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_2(&mut self) -> DATA_BYTE_2_W {
        DATA_BYTE_2_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_1(&mut self) -> DATA_BYTE_1_W {
        DATA_BYTE_1_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_0(&mut self) -> DATA_BYTE_0_W {
        DATA_BYTE_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Embedded RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramn23](index.html) module"]
pub struct RAMN23_SPEC;
impl crate::RegisterSpec for RAMN23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramn23::R](R) reader structure"]
impl crate::Readable for RAMN23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramn23::W](W) writer structure"]
impl crate::Writable for RAMN23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMn23 to value 0"]
impl crate::Resettable for RAMN23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
