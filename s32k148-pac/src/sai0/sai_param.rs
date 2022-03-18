#[doc = "Register `SAI_PARAM` reader"]
pub struct R(crate::R<SAI_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATALINE` reader - Number of Datalines"]
pub struct DATALINE_R(crate::FieldReader<u8, u8>);
impl DATALINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO` reader - FIFO Size"]
pub struct FIFO_R(crate::FieldReader<u8, u8>);
impl FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME` reader - Frame Size"]
pub struct FRAME_R(crate::FieldReader<u8, u8>);
impl FRAME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Datalines"]
    #[inline(always)]
    pub fn dataline(&self) -> DATALINE_R {
        DATALINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - FIFO Size"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Frame Size"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_param](index.html) module"]
pub struct SAI_PARAM_SPEC;
impl crate::RegisterSpec for SAI_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_param::R](R) reader structure"]
impl crate::Readable for SAI_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_PARAM to value 0x0004_0304"]
impl crate::Resettable for SAI_PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0304
    }
}
