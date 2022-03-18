#[doc = "Register `DATALL` reader"]
pub struct R(crate::R<CRC_DATALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DATALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DATALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DATALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATALL` writer"]
pub struct W(crate::W<CRC_DATALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DATALL_SPEC>;
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
impl From<crate::W<CRC_DATALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DATALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALL` reader - CRCLL stores the first 8 bits of the 32 bit DATA"]
pub struct DATALL_R(crate::FieldReader<u8, u8>);
impl DATALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALL` writer - CRCLL stores the first 8 bits of the 32 bit DATA"]
pub struct DATALL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline(always)]
    pub fn datall(&self) -> DATALL_R {
        DATALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline(always)]
    pub fn datall(&mut self) -> DATALL_W {
        DATALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_DATALL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_datall](index.html) module"]
pub struct CRC_DATALL_SPEC;
impl crate::RegisterSpec for CRC_DATALL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crc_datall::R](R) reader structure"]
impl crate::Readable for CRC_DATALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_datall::W](W) writer structure"]
impl crate::Writable for CRC_DATALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATALL to value 0xff"]
impl crate::Resettable for CRC_DATALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
