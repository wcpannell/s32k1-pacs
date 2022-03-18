#[doc = "Register `LMFATR` reader"]
pub struct R(crate::R<LMFATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMFATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMFATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMFATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEFPRT` reader - Parity/ECC Fault Protection"]
pub struct PEFPRT_R(crate::FieldReader<u8, u8>);
impl PEFPRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEFPRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEFPRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity/ECC Fault Master Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEFSIZE_A {
    #[doc = "0: 8-bit access"]
    _000 = 0,
    #[doc = "1: 16-bit access"]
    _001 = 1,
    #[doc = "2: 32-bit access"]
    _010 = 2,
    #[doc = "3: 64-bit access"]
    _011 = 3,
}
impl From<PEFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEFSIZE` reader - Parity/ECC Fault Master Size"]
pub struct PEFSIZE_R(crate::FieldReader<u8, PEFSIZE_A>);
impl PEFSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEFSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEFSIZE_A> {
        match self.bits {
            0 => Some(PEFSIZE_A::_000),
            1 => Some(PEFSIZE_A::_001),
            2 => Some(PEFSIZE_A::_010),
            3 => Some(PEFSIZE_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == PEFSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == PEFSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == PEFSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == PEFSIZE_A::_011
    }
}
impl core::ops::Deref for PEFSIZE_R {
    type Target = crate::FieldReader<u8, PEFSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFW` reader - Parity/ECC Fault Write"]
pub struct PEFW_R(crate::FieldReader<bool, bool>);
impl PEFW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEFW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFMST` reader - Parity/ECC Fault Master Number"]
pub struct PEFMST_R(crate::FieldReader<u8, u8>);
impl PEFMST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEFMST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEFMST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` reader - Overrun"]
pub struct OVR_R(crate::FieldReader<bool, bool>);
impl OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Parity/ECC Fault Protection"]
    #[inline(always)]
    pub fn pefprt(&self) -> PEFPRT_R {
        PEFPRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Parity/ECC Fault Master Size"]
    #[inline(always)]
    pub fn pefsize(&self) -> PEFSIZE_R {
        PEFSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Parity/ECC Fault Write"]
    #[inline(always)]
    pub fn pefw(&self) -> PEFW_R {
        PEFW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Parity/ECC Fault Master Number"]
    #[inline(always)]
    pub fn pefmst(&self) -> PEFMST_R {
        PEFMST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "LMEM Fault Attribute Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfatr](index.html) module"]
pub struct LMFATR_SPEC;
impl crate::RegisterSpec for LMFATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmfatr::R](R) reader structure"]
impl crate::Readable for LMFATR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LMFATR to value 0"]
impl crate::Resettable for LMFATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
