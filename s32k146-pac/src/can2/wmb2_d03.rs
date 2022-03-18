#[doc = "Register `WMB2_D03` reader"]
pub struct R(crate::R<WMB2_D03_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMB2_D03_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMB2_D03_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMB2_D03_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Data_byte_3` reader - Received payload corresponding to the data byte 3 under Pretended Networking mode"]
pub struct DATA_BYTE_3_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_2` reader - Received payload corresponding to the data byte 2 under Pretended Networking mode"]
pub struct DATA_BYTE_2_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_1` reader - Received payload corresponding to the data byte 1 under Pretended Networking mode"]
pub struct DATA_BYTE_1_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_byte_0` reader - Received payload corresponding to the data byte 0 under Pretended Networking mode"]
pub struct DATA_BYTE_0_R(crate::FieldReader<u8, u8>);
impl DATA_BYTE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BYTE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BYTE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 3 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_3(&self) -> DATA_BYTE_3_R {
        DATA_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 2 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_2(&self) -> DATA_BYTE_2_R {
        DATA_BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 1 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_1(&self) -> DATA_BYTE_1_R {
        DATA_BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 0 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_0(&self) -> DATA_BYTE_0_R {
        DATA_BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Wake Up Message Buffer Register for Data 0-3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb2_d03](index.html) module"]
pub struct WMB2_D03_SPEC;
impl crate::RegisterSpec for WMB2_D03_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmb2_d03::R](R) reader structure"]
impl crate::Readable for WMB2_D03_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WMB2_D03 to value 0"]
impl crate::Resettable for WMB2_D03_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
