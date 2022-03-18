#[doc = "Register `LMDR2` reader"]
pub struct R(crate::R<LMDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMDR2` writer"]
pub struct W(crate::W<LMDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMDR2_SPEC>;
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
impl From<crate::W<LMDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1` reader - Control Field 1"]
pub struct CF1_R(crate::FieldReader<u8, u8>);
impl CF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF1` writer - Control Field 1"]
pub struct CF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Memory Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MT_A {
    #[doc = "2: PC Cache"]
    MT_2 = 2,
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
            2 => Some(MT_A::MT_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MT_2`"]
    #[inline(always)]
    pub fn is_mt_2(&self) -> bool {
        **self == MT_A::MT_2
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
#[doc = "LMEM Data Path Width. This field defines the width of the local memory.\n\nValue on reset: 2"]
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
#[doc = "Level 1 Cache Ways\n\nValue on reset: 2"]
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
#[doc = "LMEM Size\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LMSZ_A {
    #[doc = "4: 4 KB LMEMn"]
    LMSZ_4 = 4,
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
    pub fn variant(&self) -> Option<LMSZ_A> {
        match self.bits {
            4 => Some(LMSZ_A::LMSZ_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LMSZ_4`"]
    #[inline(always)]
    pub fn is_lmsz_4(&self) -> bool {
        **self == LMSZ_A::LMSZ_4
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
#[doc = "Local Memory Valid\n\nValue on reset: 1"]
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
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 4) & 0x0f) as u8)
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
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline(always)]
    pub fn cf1(&mut self) -> CF1_W {
        CF1_W { w: self }
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
#[doc = "Local Memory Descriptor Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr2](index.html) module"]
pub struct LMDR2_SPEC;
impl crate::RegisterSpec for LMDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmdr2::R](R) reader structure"]
impl crate::Readable for LMDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmdr2::W](W) writer structure"]
impl crate::Writable for LMDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMDR2 to value 0x8424_40a0"]
impl crate::Resettable for LMDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8424_40a0
    }
}
