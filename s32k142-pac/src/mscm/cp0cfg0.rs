#[doc = "Register `CP0CFG0` reader"]
pub struct R(crate::R<CP0CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP0CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP0CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP0CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCWY` reader - Level 1 Data Cache Ways"]
pub struct DCWY_R(crate::FieldReader<u8, u8>);
impl DCWY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCWY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCWY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCSZ` reader - Level 1 Data Cache Size"]
pub struct DCSZ_R(crate::FieldReader<u8, u8>);
impl DCSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICWY` reader - Level 1 Instruction Cache Ways"]
pub struct ICWY_R(crate::FieldReader<u8, u8>);
impl ICWY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICWY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICWY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICSZ` reader - Level 1 Instruction Cache Size"]
pub struct ICSZ_R(crate::FieldReader<u8, u8>);
impl ICSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Level 1 Data Cache Ways"]
    #[inline(always)]
    pub fn dcwy(&self) -> DCWY_R {
        DCWY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Level 1 Data Cache Size"]
    #[inline(always)]
    pub fn dcsz(&self) -> DCSZ_R {
        DCSZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Level 1 Instruction Cache Ways"]
    #[inline(always)]
    pub fn icwy(&self) -> ICWY_R {
        ICWY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Level 1 Instruction Cache Size"]
    #[inline(always)]
    pub fn icsz(&self) -> ICSZ_R {
        ICSZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Processor 0 Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg0](index.html) module"]
pub struct CP0CFG0_SPEC;
impl crate::RegisterSpec for CP0CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cp0cfg0::R](R) reader structure"]
impl crate::Readable for CP0CFG0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CP0CFG0 to value 0x0400_0000"]
impl crate::Resettable for CP0CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
