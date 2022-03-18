#[doc = "Register `SPNDST` reader"]
pub struct R(crate::R<SPNDST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPNDST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPNDST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPNDST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPND` reader - When set, it signifies that a sequence is in suspended state"]
pub struct SUSPND_R(crate::FieldReader<bool, bool>);
impl SUSPND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDBUF` reader - Suspended Buffer: Provides the suspended buffer number. Valid only when SUSPND is set to 1'b1"]
pub struct SPDBUF_R(crate::FieldReader<u8, u8>);
impl SPDBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPDBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLFT` reader - Data left: Provides information about the amount of data left to be read in the suspended sequence"]
pub struct DATLFT_R(crate::FieldReader<u8, u8>);
impl DATLFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATLFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When set, it signifies that a sequence is in suspended state"]
    #[inline(always)]
    pub fn suspnd(&self) -> SUSPND_R {
        SUSPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Suspended Buffer: Provides the suspended buffer number. Valid only when SUSPND is set to 1'b1"]
    #[inline(always)]
    pub fn spdbuf(&self) -> SPDBUF_R {
        SPDBUF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 9:15 - Data left: Provides information about the amount of data left to be read in the suspended sequence"]
    #[inline(always)]
    pub fn datlft(&self) -> DATLFT_R {
        DATLFT_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "Sequence Suspend Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spndst](index.html) module"]
pub struct SPNDST_SPEC;
impl crate::RegisterSpec for SPNDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spndst::R](R) reader structure"]
impl crate::Readable for SPNDST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPNDST to value 0"]
impl crate::Resettable for SPNDST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
