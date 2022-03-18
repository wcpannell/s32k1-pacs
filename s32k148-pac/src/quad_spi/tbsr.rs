#[doc = "Register `TBSR` reader"]
pub struct R(crate::R<TBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRBFL` reader - TX Buffer Fill Level"]
pub struct TRBFL_R(crate::FieldReader<u8, u8>);
impl TRBFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRBFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRBFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCTR` reader - Transmit Counter"]
pub struct TRCTR_R(crate::FieldReader<u16, u16>);
impl TRCTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TRCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:13 - TX Buffer Fill Level"]
    #[inline(always)]
    pub fn trbfl(&self) -> TRBFL_R {
        TRBFL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Transmit Counter"]
    #[inline(always)]
    pub fn trctr(&self) -> TRCTR_R {
        TRCTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TX Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbsr](index.html) module"]
pub struct TBSR_SPEC;
impl crate::RegisterSpec for TBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbsr::R](R) reader structure"]
impl crate::Readable for TBSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBSR to value 0"]
impl crate::Resettable for TBSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
