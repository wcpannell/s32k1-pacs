#[doc = "Register `MFSR` reader"]
pub struct R(crate::R<MFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOUNT` reader - Transmit FIFO Count"]
pub struct TXCOUNT_R(crate::FieldReader<u8, u8>);
impl TXCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCOUNT` reader - Receive FIFO Count"]
pub struct RXCOUNT_R(crate::FieldReader<u8, u8>);
impl RXCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Count"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Receive FIFO Count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "Master FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfsr](index.html) module"]
pub struct MFSR_SPEC;
impl crate::RegisterSpec for MFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfsr::R](R) reader structure"]
impl crate::Readable for MFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFSR to value 0"]
impl crate::Resettable for MFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
