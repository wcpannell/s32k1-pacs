#[doc = "Register `CPxCFG3` reader"]
pub struct R(crate::R<CPXCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPXCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPXCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPXCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Floating Point Unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_A {
    #[doc = "0: FPU support is not included."]
    _0 = 0,
    #[doc = "1: FPU support is included."]
    _1 = 1,
}
impl From<FPU_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU` reader - Floating Point Unit"]
pub struct FPU_R(crate::FieldReader<bool, FPU_A>);
impl FPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_A {
        match self.bits {
            false => FPU_A::_0,
            true => FPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPU_A::_1
    }
}
impl core::ops::Deref for FPU_R {
    type Target = crate::FieldReader<bool, FPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SIMD/NEON instruction support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMD_A {
    #[doc = "0: SIMD/NEON support is not included."]
    _0 = 0,
    #[doc = "1: SIMD/NEON support is included."]
    _1 = 1,
}
impl From<SIMD_A> for bool {
    #[inline(always)]
    fn from(variant: SIMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIMD` reader - SIMD/NEON instruction support"]
pub struct SIMD_R(crate::FieldReader<bool, SIMD_A>);
impl SIMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIMD_A {
        match self.bits {
            false => SIMD_A::_0,
            true => SIMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIMD_A::_1
    }
}
impl core::ops::Deref for SIMD_R {
    type Target = crate::FieldReader<bool, SIMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Jazelle support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAZ_A {
    #[doc = "0: Jazelle support is not included."]
    _0 = 0,
    #[doc = "1: Jazelle support is included."]
    _1 = 1,
}
impl From<JAZ_A> for bool {
    #[inline(always)]
    fn from(variant: JAZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAZ` reader - Jazelle support"]
pub struct JAZ_R(crate::FieldReader<bool, JAZ_A>);
impl JAZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JAZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAZ_A {
        match self.bits {
            false => JAZ_A::_0,
            true => JAZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == JAZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == JAZ_A::_1
    }
}
impl core::ops::Deref for JAZ_R {
    type Target = crate::FieldReader<bool, JAZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Memory Management Unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMU_A {
    #[doc = "0: MMU support is not included."]
    _0 = 0,
    #[doc = "1: MMU support is included."]
    _1 = 1,
}
impl From<MMU_A> for bool {
    #[inline(always)]
    fn from(variant: MMU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMU` reader - Memory Management Unit"]
pub struct MMU_R(crate::FieldReader<bool, MMU_A>);
impl MMU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMU_A {
        match self.bits {
            false => MMU_A::_0,
            true => MMU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MMU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MMU_A::_1
    }
}
impl core::ops::Deref for MMU_R {
    type Target = crate::FieldReader<bool, MMU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trust Zone\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZ_A {
    #[doc = "0: Trust Zone support is not included."]
    _0 = 0,
    #[doc = "1: Trust Zone support is included."]
    _1 = 1,
}
impl From<TZ_A> for bool {
    #[inline(always)]
    fn from(variant: TZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TZ` reader - Trust Zone"]
pub struct TZ_R(crate::FieldReader<bool, TZ_A>);
impl TZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZ_A {
        match self.bits {
            false => TZ_A::_0,
            true => TZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TZ_A::_1
    }
}
impl core::ops::Deref for TZ_R {
    type Target = crate::FieldReader<bool, TZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Core Memory Protection unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_A {
    #[doc = "0: Core Memory Protection is not included."]
    _0 = 0,
    #[doc = "1: Core Memory Protection is included."]
    _1 = 1,
}
impl From<CMP_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP` reader - Core Memory Protection unit"]
pub struct CMP_R(crate::FieldReader<bool, CMP_A>);
impl CMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_A {
        match self.bits {
            false => CMP_A::_0,
            true => CMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMP_A::_1
    }
}
impl core::ops::Deref for CMP_R {
    type Target = crate::FieldReader<bool, CMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bit Banding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_A {
    #[doc = "0: Bit Banding is not supported."]
    _0 = 0,
    #[doc = "1: Bit Banding is supported."]
    _1 = 1,
}
impl From<BB_A> for bool {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` reader - Bit Banding"]
pub struct BB_R(crate::FieldReader<bool, BB_A>);
impl BB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_A {
        match self.bits {
            false => BB_A::_0,
            true => BB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BB_A::_1
    }
}
impl core::ops::Deref for BB_R {
    type Target = crate::FieldReader<bool, BB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBP` reader - System Bus Ports"]
pub struct SBP_R(crate::FieldReader<u8, u8>);
impl SBP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Floating Point Unit"]
    #[inline(always)]
    pub fn fpu(&self) -> FPU_R {
        FPU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SIMD/NEON instruction support"]
    #[inline(always)]
    pub fn simd(&self) -> SIMD_R {
        SIMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Jazelle support"]
    #[inline(always)]
    pub fn jaz(&self) -> JAZ_R {
        JAZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory Management Unit"]
    #[inline(always)]
    pub fn mmu(&self) -> MMU_R {
        MMU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trust Zone"]
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Core Memory Protection unit"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bit Banding"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - System Bus Ports"]
    #[inline(always)]
    pub fn sbp(&self) -> SBP_R {
        SBP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
#[doc = "Processor X Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg3](index.html) module"]
pub struct CPXCFG3_SPEC;
impl crate::RegisterSpec for CPXCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpx_cfg3::R](R) reader structure"]
impl crate::Readable for CPXCFG3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPxCFG3 to value 0"]
impl crate::Resettable for CPXCFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
