#[doc = "Register `LMDR%s` reader"]
pub struct R(crate::R<LMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMDR%s` writer"]
pub struct W(crate::W<LMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMDR_SPEC>;
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
impl From<crate::W<LMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMDR_SPEC>) -> Self {
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
    MT_0 = 0,
    #[doc = "1: SRAM_U"]
    MT_1 = 1,
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
            0 => Some(MT_A::MT_0),
            1 => Some(MT_A::MT_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MT_0`"]
    #[inline(always)]
    pub fn is_mt_0(&self) -> bool {
        **self == MT_A::MT_0
    }
    #[doc = "Checks if the value of the field is `MT_1`"]
    #[inline(always)]
    pub fn is_mt_1(&self) -> bool {
        **self == MT_A::MT_1
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
    LOCK_0 = 0,
    #[doc = "1: Writes to the LMDRn\\[7:0\\]
are ignored."]
    LOCK_1 = 1,
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
            false => LOCK_A::LOCK_0,
            true => LOCK_A::LOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_0`"]
    #[inline(always)]
    pub fn is_lock_0(&self) -> bool {
        **self == LOCK_A::LOCK_0
    }
    #[doc = "Checks if the value of the field is `LOCK_1`"]
    #[inline(always)]
    pub fn is_lock_1(&self) -> bool {
        **self == LOCK_A::LOCK_1
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
    pub fn lock_0(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK_0)
    }
    #[doc = "Writes to the LMDRn\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn lock_1(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK_1)
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
    DPW_2 = 2,
    #[doc = "3: LMEMn 64-bits wide"]
    DPW_3 = 3,
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
            2 => Some(DPW_A::DPW_2),
            3 => Some(DPW_A::DPW_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DPW_2`"]
    #[inline(always)]
    pub fn is_dpw_2(&self) -> bool {
        **self == DPW_A::DPW_2
    }
    #[doc = "Checks if the value of the field is `DPW_3`"]
    #[inline(always)]
    pub fn is_dpw_3(&self) -> bool {
        **self == DPW_A::DPW_3
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
    WY_0 = 0,
    #[doc = "2: 2-Way Set Associative"]
    WY_2 = 2,
    #[doc = "4: 4-Way Set Associative"]
    WY_4 = 4,
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
            0 => Some(WY_A::WY_0),
            2 => Some(WY_A::WY_2),
            4 => Some(WY_A::WY_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WY_0`"]
    #[inline(always)]
    pub fn is_wy_0(&self) -> bool {
        **self == WY_A::WY_0
    }
    #[doc = "Checks if the value of the field is `WY_2`"]
    #[inline(always)]
    pub fn is_wy_2(&self) -> bool {
        **self == WY_A::WY_2
    }
    #[doc = "Checks if the value of the field is `WY_4`"]
    #[inline(always)]
    pub fn is_wy_4(&self) -> bool {
        **self == WY_A::WY_4
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
    LMSZ_0 = 0,
    #[doc = "1: 1 KB LMEMn"]
    LMSZ_1 = 1,
    #[doc = "2: 2 KB LMEMn"]
    LMSZ_2 = 2,
    #[doc = "3: 4 KB LMEMn"]
    LMSZ_3 = 3,
    #[doc = "4: 8 KB LMEMn"]
    LMSZ_4 = 4,
    #[doc = "5: 16 KB LMEMn"]
    LMSZ_5 = 5,
    #[doc = "6: 32 KB LMEMn"]
    LMSZ_6 = 6,
    #[doc = "7: 64 KB LMEMn"]
    LMSZ_7 = 7,
    #[doc = "8: 128 KB LMEMn"]
    LMSZ_8 = 8,
    #[doc = "9: 256 KB LMEMn"]
    LMSZ_9 = 9,
    #[doc = "10: 512 KB LMEMn"]
    LMSZ_10 = 10,
    #[doc = "11: 1024 KB LMEMn"]
    LMSZ_11 = 11,
    #[doc = "12: 2048 KB LMEMn"]
    LMSZ_12 = 12,
    #[doc = "13: 4096 KB LMEMn"]
    LMSZ_13 = 13,
    #[doc = "14: 8192 KB LMEMn"]
    LMSZ_14 = 14,
    #[doc = "15: 16384 KB LMEMn"]
    LMSZ_15 = 15,
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
            0 => LMSZ_A::LMSZ_0,
            1 => LMSZ_A::LMSZ_1,
            2 => LMSZ_A::LMSZ_2,
            3 => LMSZ_A::LMSZ_3,
            4 => LMSZ_A::LMSZ_4,
            5 => LMSZ_A::LMSZ_5,
            6 => LMSZ_A::LMSZ_6,
            7 => LMSZ_A::LMSZ_7,
            8 => LMSZ_A::LMSZ_8,
            9 => LMSZ_A::LMSZ_9,
            10 => LMSZ_A::LMSZ_10,
            11 => LMSZ_A::LMSZ_11,
            12 => LMSZ_A::LMSZ_12,
            13 => LMSZ_A::LMSZ_13,
            14 => LMSZ_A::LMSZ_14,
            15 => LMSZ_A::LMSZ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LMSZ_0`"]
    #[inline(always)]
    pub fn is_lmsz_0(&self) -> bool {
        **self == LMSZ_A::LMSZ_0
    }
    #[doc = "Checks if the value of the field is `LMSZ_1`"]
    #[inline(always)]
    pub fn is_lmsz_1(&self) -> bool {
        **self == LMSZ_A::LMSZ_1
    }
    #[doc = "Checks if the value of the field is `LMSZ_2`"]
    #[inline(always)]
    pub fn is_lmsz_2(&self) -> bool {
        **self == LMSZ_A::LMSZ_2
    }
    #[doc = "Checks if the value of the field is `LMSZ_3`"]
    #[inline(always)]
    pub fn is_lmsz_3(&self) -> bool {
        **self == LMSZ_A::LMSZ_3
    }
    #[doc = "Checks if the value of the field is `LMSZ_4`"]
    #[inline(always)]
    pub fn is_lmsz_4(&self) -> bool {
        **self == LMSZ_A::LMSZ_4
    }
    #[doc = "Checks if the value of the field is `LMSZ_5`"]
    #[inline(always)]
    pub fn is_lmsz_5(&self) -> bool {
        **self == LMSZ_A::LMSZ_5
    }
    #[doc = "Checks if the value of the field is `LMSZ_6`"]
    #[inline(always)]
    pub fn is_lmsz_6(&self) -> bool {
        **self == LMSZ_A::LMSZ_6
    }
    #[doc = "Checks if the value of the field is `LMSZ_7`"]
    #[inline(always)]
    pub fn is_lmsz_7(&self) -> bool {
        **self == LMSZ_A::LMSZ_7
    }
    #[doc = "Checks if the value of the field is `LMSZ_8`"]
    #[inline(always)]
    pub fn is_lmsz_8(&self) -> bool {
        **self == LMSZ_A::LMSZ_8
    }
    #[doc = "Checks if the value of the field is `LMSZ_9`"]
    #[inline(always)]
    pub fn is_lmsz_9(&self) -> bool {
        **self == LMSZ_A::LMSZ_9
    }
    #[doc = "Checks if the value of the field is `LMSZ_10`"]
    #[inline(always)]
    pub fn is_lmsz_10(&self) -> bool {
        **self == LMSZ_A::LMSZ_10
    }
    #[doc = "Checks if the value of the field is `LMSZ_11`"]
    #[inline(always)]
    pub fn is_lmsz_11(&self) -> bool {
        **self == LMSZ_A::LMSZ_11
    }
    #[doc = "Checks if the value of the field is `LMSZ_12`"]
    #[inline(always)]
    pub fn is_lmsz_12(&self) -> bool {
        **self == LMSZ_A::LMSZ_12
    }
    #[doc = "Checks if the value of the field is `LMSZ_13`"]
    #[inline(always)]
    pub fn is_lmsz_13(&self) -> bool {
        **self == LMSZ_A::LMSZ_13
    }
    #[doc = "Checks if the value of the field is `LMSZ_14`"]
    #[inline(always)]
    pub fn is_lmsz_14(&self) -> bool {
        **self == LMSZ_A::LMSZ_14
    }
    #[doc = "Checks if the value of the field is `LMSZ_15`"]
    #[inline(always)]
    pub fn is_lmsz_15(&self) -> bool {
        **self == LMSZ_A::LMSZ_15
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
    LMSZH_0 = 0,
    #[doc = "1: LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ."]
    LMSZH_1 = 1,
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
            false => LMSZH_A::LMSZH_0,
            true => LMSZH_A::LMSZH_1,
        }
    }
    #[doc = "Checks if the value of the field is `LMSZH_0`"]
    #[inline(always)]
    pub fn is_lmszh_0(&self) -> bool {
        **self == LMSZH_A::LMSZH_0
    }
    #[doc = "Checks if the value of the field is `LMSZH_1`"]
    #[inline(always)]
    pub fn is_lmszh_1(&self) -> bool {
        **self == LMSZH_A::LMSZH_1
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
    V_0 = 0,
    #[doc = "1: LMEMn is present."]
    V_1 = 1,
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
            false => V_A::V_0,
            true => V_A::V_1,
        }
    }
    #[doc = "Checks if the value of the field is `V_0`"]
    #[inline(always)]
    pub fn is_v_0(&self) -> bool {
        **self == V_A::V_0
    }
    #[doc = "Checks if the value of the field is `V_1`"]
    #[inline(always)]
    pub fn is_v_1(&self) -> bool {
        **self == V_A::V_1
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
#[doc = "Local Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr](index.html) module"]
pub struct LMDR_SPEC;
impl crate::RegisterSpec for LMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmdr::R](R) reader structure"]
impl crate::Readable for LMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmdr::W](W) writer structure"]
impl crate::Writable for LMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMDR%s to value 0"]
impl crate::Resettable for LMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
