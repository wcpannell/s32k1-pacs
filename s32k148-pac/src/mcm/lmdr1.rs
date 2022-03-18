#[doc = "Register `LMDR1` reader"]
pub struct R(crate::R<LMDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMDR1` writer"]
pub struct W(crate::W<LMDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LMDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0` reader - Control Field 0 LMDR0\\[CF0\\]
bit field is Reserved and Read-Only 0 for S32K11x variants."]
pub struct CF0_R(crate::FieldReader<u8, u8>);
impl CF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF0` writer - Control Field 0 LMDR0\\[CF0\\]
bit field is Reserved and Read-Only 0 for S32K11x variants."]
pub struct CF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Memory Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MT_A {
    #[doc = "0: SRAM_L"]
    _000 = 0,
    #[doc = "1: SRAM_U"]
    _001 = 1,
}
impl From<MT_A> for u8 {
    #[inline(always)]
    fn from(variant: MT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MT` reader - Memory Type"]
pub struct MT_R(crate::FieldReader<u8, MT_A>);
impl MT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MT_A> {
        match self.bits {
            0 => Some(MT_A::_000),
            1 => Some(MT_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == MT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == MT_A::_001
    }
}
impl core::ops::Deref for MT_R {
    type Target = crate::FieldReader<u8, MT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Writes to the LMDRn\\[7:0\\]
are allowed."]
    _0 = 0,
    #[doc = "1: Writes to the LMDRn\\[7:0\\]
are ignored."]
    _1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::_0,
            true => LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCK_A::_1
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - LOCK"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the LMDRn\\[7:0\\]
are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCK_A::_0)
    }
    #[doc = "Writes to the LMDRn\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "LMEM Data Path Width. This field defines the width of the local memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DPW_A {
    #[doc = "2: LMEMn 32-bits wide"]
    _010 = 2,
    #[doc = "3: LMEMn 64-bits wide"]
    _011 = 3,
}
impl From<DPW_A> for u8 {
    #[inline(always)]
    fn from(variant: DPW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DPW` reader - LMEM Data Path Width. This field defines the width of the local memory."]
pub struct DPW_R(crate::FieldReader<u8, DPW_A>);
impl DPW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DPW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DPW_A> {
        match self.bits {
            2 => Some(DPW_A::_010),
            3 => Some(DPW_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == DPW_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == DPW_A::_011
    }
}
impl core::ops::Deref for DPW_R {
    type Target = crate::FieldReader<u8, DPW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Level 1 Cache Ways\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WY_A {
    #[doc = "0: No Cache"]
    _0000 = 0,
    #[doc = "2: 2-Way Set Associative"]
    _0010 = 2,
    #[doc = "4: 4-Way Set Associative"]
    _0100 = 4,
}
impl From<WY_A> for u8 {
    #[inline(always)]
    fn from(variant: WY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WY` reader - Level 1 Cache Ways"]
pub struct WY_R(crate::FieldReader<u8, WY_A>);
impl WY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WY_A> {
        match self.bits {
            0 => Some(WY_A::_0000),
            2 => Some(WY_A::_0010),
            4 => Some(WY_A::_0100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == WY_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == WY_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == WY_A::_0100
    }
}
impl core::ops::Deref for WY_R {
    type Target = crate::FieldReader<u8, WY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LMEM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LMSZ_A {
    #[doc = "0: no LMEMn (0 KB)"]
    _0000 = 0,
    #[doc = "1: 1 KB LMEMn"]
    _0001 = 1,
    #[doc = "2: 2 KB LMEMn"]
    _0010 = 2,
    #[doc = "3: 4 KB LMEMn"]
    _0011 = 3,
    #[doc = "4: 8 KB LMEMn"]
    _0100 = 4,
    #[doc = "5: 16 KB LMEMn"]
    _0101 = 5,
    #[doc = "6: 32 KB LMEMn"]
    _0110 = 6,
    #[doc = "7: 64 KB LMEMn"]
    _0111 = 7,
    #[doc = "8: 128 KB LMEMn"]
    _1000 = 8,
    #[doc = "9: 256 KB LMEMn"]
    _1001 = 9,
    #[doc = "10: 512 KB LMEMn"]
    _1010 = 10,
    #[doc = "11: 1024 KB LMEMn"]
    _1011 = 11,
    #[doc = "12: 2048 KB LMEMn"]
    _1100 = 12,
    #[doc = "13: 4096 KB LMEMn"]
    _1101 = 13,
    #[doc = "14: 8192 KB LMEMn"]
    _1110 = 14,
    #[doc = "15: 16384 KB LMEMn"]
    _1111 = 15,
}
impl From<LMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: LMSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LMSZ` reader - LMEM Size"]
pub struct LMSZ_R(crate::FieldReader<u8, LMSZ_A>);
impl LMSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LMSZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMSZ_A {
        match self.bits {
            0 => LMSZ_A::_0000,
            1 => LMSZ_A::_0001,
            2 => LMSZ_A::_0010,
            3 => LMSZ_A::_0011,
            4 => LMSZ_A::_0100,
            5 => LMSZ_A::_0101,
            6 => LMSZ_A::_0110,
            7 => LMSZ_A::_0111,
            8 => LMSZ_A::_1000,
            9 => LMSZ_A::_1001,
            10 => LMSZ_A::_1010,
            11 => LMSZ_A::_1011,
            12 => LMSZ_A::_1100,
            13 => LMSZ_A::_1101,
            14 => LMSZ_A::_1110,
            15 => LMSZ_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == LMSZ_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == LMSZ_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == LMSZ_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == LMSZ_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == LMSZ_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == LMSZ_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == LMSZ_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == LMSZ_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == LMSZ_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == LMSZ_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == LMSZ_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == LMSZ_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == LMSZ_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == LMSZ_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == LMSZ_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == LMSZ_A::_1111
    }
}
impl core::ops::Deref for LMSZ_R {
    type Target = crate::FieldReader<u8, LMSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LMEM Size Hole\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSZH_A {
    #[doc = "0: LMEMn is a power-of-2 capacity."]
    _0 = 0,
    #[doc = "1: LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ."]
    _1 = 1,
}
impl From<LMSZH_A> for bool {
    #[inline(always)]
    fn from(variant: LMSZH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMSZH` reader - LMEM Size Hole"]
pub struct LMSZH_R(crate::FieldReader<bool, LMSZH_A>);
impl LMSZH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMSZH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMSZH_A {
        match self.bits {
            false => LMSZH_A::_0,
            true => LMSZH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LMSZH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LMSZH_A::_1
    }
}
impl core::ops::Deref for LMSZH_R {
    type Target = crate::FieldReader<bool, LMSZH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Local Memory Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: LMEMn is not present."]
    _0 = 0,
    #[doc = "1: LMEMn is present."]
    _1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Local Memory Valid"]
pub struct V_R(crate::FieldReader<bool, V_A>);
impl V_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::_0,
            true => V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == V_A::_1
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Control Field 0 LMDR0\\[CF0\\]
bit field is Reserved and Read-Only 0 for S32K11x variants."]
    #[inline(always)]
    pub fn cf0(&self) -> CF0_R {
        CF0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Memory Type"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - LMEM Data Path Width. This field defines the width of the local memory."]
    #[inline(always)]
    pub fn dpw(&self) -> DPW_R {
        DPW_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Level 1 Cache Ways"]
    #[inline(always)]
    pub fn wy(&self) -> WY_R {
        WY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LMEM Size"]
    #[inline(always)]
    pub fn lmsz(&self) -> LMSZ_R {
        LMSZ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - LMEM Size Hole"]
    #[inline(always)]
    pub fn lmszh(&self) -> LMSZH_R {
        LMSZH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Local Memory Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control Field 0 LMDR0\\[CF0\\]
bit field is Reserved and Read-Only 0 for S32K11x variants."]
    #[inline(always)]
    pub fn cf0(&mut self) -> CF0_W {
        CF0_W { w: self }
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr1](index.html) module"]
pub struct LMDR1_SPEC;
impl crate::RegisterSpec for LMDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmdr1::R](R) reader structure"]
impl crate::Readable for LMDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmdr1::W](W) writer structure"]
impl crate::Writable for LMDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMDR1 to value 0"]
impl crate::Resettable for LMDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
