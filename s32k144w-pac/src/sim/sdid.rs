#[doc = "Register `SDID` reader"]
pub struct R(crate::R<SDID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEATURES` reader - Features"]
pub struct FEATURES_R(crate::FieldReader<u8, u8>);
impl FEATURES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FEATURES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEATURES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Package\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKAGE_A {
    #[doc = "2: 48 LQFP"]
    PACKAGE_2 = 2,
    #[doc = "3: 64 LQFP"]
    PACKAGE_3 = 3,
    #[doc = "4: 100 LQFP"]
    PACKAGE_4 = 4,
    #[doc = "6: 144 LQFP"]
    PACKAGE_6 = 6,
    #[doc = "7: 176 LQFP"]
    PACKAGE_7 = 7,
    #[doc = "8: 100 MAP BGA"]
    PACKAGE_8 = 8,
}
impl From<PACKAGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PACKAGE` reader - Package"]
pub struct PACKAGE_R(crate::FieldReader<u8, PACKAGE_A>);
impl PACKAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PACKAGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACKAGE_A> {
        match self.bits {
            2 => Some(PACKAGE_A::PACKAGE_2),
            3 => Some(PACKAGE_A::PACKAGE_3),
            4 => Some(PACKAGE_A::PACKAGE_4),
            6 => Some(PACKAGE_A::PACKAGE_6),
            7 => Some(PACKAGE_A::PACKAGE_7),
            8 => Some(PACKAGE_A::PACKAGE_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PACKAGE_2`"]
    #[inline(always)]
    pub fn is_package_2(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_2
    }
    #[doc = "Checks if the value of the field is `PACKAGE_3`"]
    #[inline(always)]
    pub fn is_package_3(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_3
    }
    #[doc = "Checks if the value of the field is `PACKAGE_4`"]
    #[inline(always)]
    pub fn is_package_4(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_4
    }
    #[doc = "Checks if the value of the field is `PACKAGE_6`"]
    #[inline(always)]
    pub fn is_package_6(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_6
    }
    #[doc = "Checks if the value of the field is `PACKAGE_7`"]
    #[inline(always)]
    pub fn is_package_7(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_7
    }
    #[doc = "Checks if the value of the field is `PACKAGE_8`"]
    #[inline(always)]
    pub fn is_package_8(&self) -> bool {
        **self == PACKAGE_A::PACKAGE_8
    }
}
impl core::ops::Deref for PACKAGE_R {
    type Target = crate::FieldReader<u8, PACKAGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVID` reader - Device revision number"]
pub struct REVID_R(crate::FieldReader<u8, u8>);
impl REVID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMSIZE_A {
    #[doc = "11: 192 KB (S32K148), 96 KB (S32K146), Reserved (others)"]
    RAMSIZE_11 = 11,
    #[doc = "13: 48 KB (S32K144), Reserved (others)"]
    RAMSIZE_13 = 13,
    #[doc = "15: 256 KB (S32K148), 128 KB (S32K146), 64 KB (S32K144), 32 KB (S32K142), 25 KB (S32K118), 17 KB (S32K116)"]
    RAMSIZE_15 = 15,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAMSIZE` reader - RAM size"]
pub struct RAMSIZE_R(crate::FieldReader<u8, RAMSIZE_A>);
impl RAMSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAMSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMSIZE_A> {
        match self.bits {
            11 => Some(RAMSIZE_A::RAMSIZE_11),
            13 => Some(RAMSIZE_A::RAMSIZE_13),
            15 => Some(RAMSIZE_A::RAMSIZE_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RAMSIZE_11`"]
    #[inline(always)]
    pub fn is_ramsize_11(&self) -> bool {
        **self == RAMSIZE_A::RAMSIZE_11
    }
    #[doc = "Checks if the value of the field is `RAMSIZE_13`"]
    #[inline(always)]
    pub fn is_ramsize_13(&self) -> bool {
        **self == RAMSIZE_A::RAMSIZE_13
    }
    #[doc = "Checks if the value of the field is `RAMSIZE_15`"]
    #[inline(always)]
    pub fn is_ramsize_15(&self) -> bool {
        **self == RAMSIZE_A::RAMSIZE_15
    }
}
impl core::ops::Deref for RAMSIZE_R {
    type Target = crate::FieldReader<u8, RAMSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERIVATE` reader - Derivate"]
pub struct DERIVATE_R(crate::FieldReader<u8, u8>);
impl DERIVATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DERIVATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERIVATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBSERIES` reader - Subseries"]
pub struct SUBSERIES_R(crate::FieldReader<u8, u8>);
impl SUBSERIES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUBSERIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBSERIES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENERATION` reader - S32K product series generation"]
pub struct GENERATION_R(crate::FieldReader<u8, u8>);
impl GENERATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERATION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Features"]
    #[inline(always)]
    pub fn features(&self) -> FEATURES_R {
        FEATURES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Package"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Derivate"]
    #[inline(always)]
    pub fn derivate(&self) -> DERIVATE_R {
        DERIVATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Subseries"]
    #[inline(always)]
    pub fn subseries(&self) -> SUBSERIES_R {
        SUBSERIES_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - S32K product series generation"]
    #[inline(always)]
    pub fn generation(&self) -> GENERATION_R {
        GENERATION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdid](index.html) module"]
pub struct SDID_SPEC;
impl crate::RegisterSpec for SDID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdid::R](R) reader structure"]
impl crate::Readable for SDID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDID to value 0"]
impl crate::Resettable for SDID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
