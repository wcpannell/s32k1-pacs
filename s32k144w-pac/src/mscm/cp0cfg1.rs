#[doc = "Register `CP0CFG1` reader"]
pub struct R(crate::R<CP0CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP0CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP0CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP0CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2WY` reader - Level 2 Instruction Cache Ways"]
pub struct L2WY_R(crate::FieldReader<u8, u8>);
impl L2WY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        L2WY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L2WY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L2SZ` reader - Level 2 Instruction Cache Size"]
pub struct L2SZ_R(crate::FieldReader<u8, u8>);
impl L2SZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        L2SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L2SZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:23 - Level 2 Instruction Cache Ways"]
    #[inline(always)]
    pub fn l2wy(&self) -> L2WY_R {
        L2WY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Level 2 Instruction Cache Size"]
    #[inline(always)]
    pub fn l2sz(&self) -> L2SZ_R {
        L2SZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Processor 0 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg1](index.html) module"]
pub struct CP0CFG1_SPEC;
impl crate::RegisterSpec for CP0CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cp0cfg1::R](R) reader structure"]
impl crate::Readable for CP0CFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CP0CFG1 to value 0"]
impl crate::Resettable for CP0CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
