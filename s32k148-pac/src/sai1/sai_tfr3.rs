#[doc = "Register `SAI_TFR3` reader"]
pub struct R(crate::R<SAI_TFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_TFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_TFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_TFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFP` reader - Read FIFO Pointer"]
pub struct RFP_R(crate::FieldReader<u8, u8>);
impl RFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFP` reader - Write FIFO Pointer"]
pub struct WFP_R(crate::FieldReader<u8, u8>);
impl WFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Channel Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCP_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
    _1 = 1,
}
impl From<WCP_A> for bool {
    #[inline(always)]
    fn from(variant: WCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCP` reader - Write Channel Pointer"]
pub struct WCP_R(crate::FieldReader<bool, WCP_A>);
impl WCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCP_A {
        match self.bits {
            false => WCP_A::_0,
            true => WCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WCP_A::_1
    }
}
impl core::ops::Deref for WCP_R {
    type Target = crate::FieldReader<bool, WCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Write Channel Pointer"]
    #[inline(always)]
    pub fn wcp(&self) -> WCP_R {
        WCP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "SAI Transmit FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_tfr3](index.html) module"]
pub struct SAI_TFR3_SPEC;
impl crate::RegisterSpec for SAI_TFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_tfr3::R](R) reader structure"]
impl crate::Readable for SAI_TFR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_TFR3 to value 0"]
impl crate::Resettable for SAI_TFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
