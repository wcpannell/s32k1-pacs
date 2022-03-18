#[doc = "Register `WMB3_CS` reader"]
pub struct R(crate::R<WMB3_CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMB3_CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMB3_CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMB3_CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLC` reader - Length of Data in Bytes"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Remote Transmission Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Frame is data one (not remote)"]
    _0 = 0,
    #[doc = "1: Frame is a remote one"]
    _1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - Remote Transmission Request Bit"]
pub struct RTR_R(crate::FieldReader<bool, RTR_A>);
impl RTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::_0,
            true => RTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTR_A::_1
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, RTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ID Extended Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Frame format is standard"]
    _0 = 0,
    #[doc = "1: Frame format is extended"]
    _1 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - ID Extended Bit"]
pub struct IDE_R(crate::FieldReader<bool, IDE_A>);
impl IDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::_0,
            true => IDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDE_A::_1
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, IDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRR` reader - Substitute Remote Request"]
pub struct SRR_R(crate::FieldReader<bool, bool>);
impl SRR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:19 - Length of Data in Bytes"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Remote Transmission Request Bit"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ID Extended Bit"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Substitute Remote Request"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
#[doc = "Wake Up Message Buffer Register for C/S\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb3_cs](index.html) module"]
pub struct WMB3_CS_SPEC;
impl crate::RegisterSpec for WMB3_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmb3_cs::R](R) reader structure"]
impl crate::Readable for WMB3_CS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WMB3_CS to value 0"]
impl crate::Resettable for WMB3_CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
