#[doc = "Register `WMB0_D47` reader"]
pub struct R(crate::R<WMB0_D47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMB0_D47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMB0_D47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMB0_D47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Data_byte_7` reader - Received payload corresponding to the data byte 7 under Pretended Networking mode"]
pub struct DATA_BYTE_7_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_6` reader - Received payload corresponding to the data byte 6 under Pretended Networking mode"]
pub struct DATA_BYTE_6_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_5` reader - Received payload corresponding to the data byte 5 under Pretended Networking mode"]
pub struct DATA_BYTE_5_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_4` reader - Received payload corresponding to the data byte 4 under Pretended Networking mode"]
pub struct DATA_BYTE_4_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 7 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_7(&self) -> DATA_BYTE_7_R {
        DATA_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 6 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_6(&self) -> DATA_BYTE_6_R {
        DATA_BYTE_6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 5 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_5(&self) -> DATA_BYTE_5_R {
        DATA_BYTE_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 4 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_4(&self) -> DATA_BYTE_4_R {
        DATA_BYTE_4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Wake Up Message Buffer Register Data 4-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_d47](index.html) module"]
pub struct WMB0_D47_SPEC;
impl crate::RegisterSpec for WMB0_D47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmb0_d47::R](R) reader structure"]
impl crate::Readable for WMB0_D47_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WMB0_D47 to value 0"]
impl crate::Resettable for WMB0_D47_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
