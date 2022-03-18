#[doc = "Register `CVAL3` reader"]
pub struct R(crate::R<CVAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TMR_CUR_VAL` reader - Current Timer Value"]
pub struct TMR_CUR_VAL_R(crate::FieldReader<u32, u32>);
impl TMR_CUR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TMR_CUR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_CUR_VAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current Timer Value"]
    #[inline(always)]
    pub fn tmr_cur_val(&self) -> TMR_CUR_VAL_R {
        TMR_CUR_VAL_R::new(self.bits)
    }
}
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval3](index.html) module"]
pub struct CVAL3_SPEC;
impl crate::RegisterSpec for CVAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cval3::R](R) reader structure"]
impl crate::Readable for CVAL3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CVAL3 to value 0xffff_ffff"]
impl crate::Resettable for CVAL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
