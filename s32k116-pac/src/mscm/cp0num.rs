#[doc = "Register `CP0NUM` reader"]
pub struct R(crate::R<CP0NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP0NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP0NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP0NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPN` reader - Processor 0 Number"]
pub struct CPN_R(crate::FieldReader<bool, bool>);
impl CPN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Processor 0 Number"]
    #[inline(always)]
    pub fn cpn(&self) -> CPN_R {
        CPN_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Processor 0 Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0num](index.html) module"]
pub struct CP0NUM_SPEC;
impl crate::RegisterSpec for CP0NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cp0num::R](R) reader structure"]
impl crate::Readable for CP0NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CP0NUM to value 0"]
impl crate::Resettable for CP0NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
