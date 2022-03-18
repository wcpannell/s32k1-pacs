#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DWTCFGCTRL` reader - DWT configuration controls"]
pub struct DWTCFGCTRL_R(crate::FieldReader<u32, u32>);
impl DWTCFGCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DWTCFGCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWTCFGCTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMCMP` reader - Number of comparators"]
pub struct NUMCMP_R(crate::FieldReader<u8, u8>);
impl NUMCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMCMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:27 - DWT configuration controls"]
    #[inline(always)]
    pub fn dwtcfgctrl(&self) -> DWTCFGCTRL_R {
        DWTCFGCTRL_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 28:31 - Number of comparators"]
    #[inline(always)]
    pub fn numcmp(&self) -> NUMCMP_R {
        NUMCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "MTB DWT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL to value 0x2f00_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2f00_0000
    }
}
