#[doc = "Register `OCMDR2` reader"]
pub struct R(crate::R<OCMDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMDR2` writer"]
pub struct W(crate::W<OCMDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMDR2_SPEC>;
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
impl From<crate::W<OCMDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCMPU` reader - OCMPU"]
pub struct OCMPU_R(crate::FieldReader<bool, bool>);
impl OCMPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCMPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCMPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OCMT\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMT_A {
    #[doc = "4: OCMEMn is a Program Flash."]
    OCMT_4 = 4,
    #[doc = "5: OCMEMn is a Data Flash."]
    OCMT_5 = 5,
    #[doc = "6: OCMEMn is an EEE."]
    OCMT_6 = 6,
}
impl From<OCMT_A> for u8 {
    #[inline(always)]
    fn from(variant: OCMT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OCMT` reader - OCMT"]
pub struct OCMT_R(crate::FieldReader<u8, OCMT_A>);
impl OCMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OCMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OCMT_A> {
        match self.bits {
            4 => Some(OCMT_A::OCMT_4),
            5 => Some(OCMT_A::OCMT_5),
            6 => Some(OCMT_A::OCMT_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OCMT_4`"]
    #[inline(always)]
    pub fn is_ocmt_4(&self) -> bool {
        **self == OCMT_A::OCMT_4
    }
    #[doc = "Checks if the value of the field is `OCMT_5`"]
    #[inline(always)]
    pub fn is_ocmt_5(&self) -> bool {
        **self == OCMT_A::OCMT_5
    }
    #[doc = "Checks if the value of the field is `OCMT_6`"]
    #[inline(always)]
    pub fn is_ocmt_6(&self) -> bool {
        **self == OCMT_A::OCMT_6
    }
}
impl core::ops::Deref for OCMT_R {
    type Target = crate::FieldReader<u8, OCMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_A {
    #[doc = "0: Writes to the OCMDRn\\[11:0\\]
are allowed"]
    RO_0 = 0,
    #[doc = "1: Writes to the OCMDRn\\[11:0\\]
are ignored"]
    RO_1 = 1,
}
impl From<RO_A> for bool {
    #[inline(always)]
    fn from(variant: RO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO` reader - RO"]
pub struct RO_R(crate::FieldReader<bool, RO_A>);
impl RO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO_A {
        match self.bits {
            false => RO_A::RO_0,
            true => RO_A::RO_1,
        }
    }
    #[doc = "Checks if the value of the field is `RO_0`"]
    #[inline(always)]
    pub fn is_ro_0(&self) -> bool {
        **self == RO_A::RO_0
    }
    #[doc = "Checks if the value of the field is `RO_1`"]
    #[inline(always)]
    pub fn is_ro_1(&self) -> bool {
        **self == RO_A::RO_1
    }
}
impl core::ops::Deref for RO_R {
    type Target = crate::FieldReader<bool, RO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RO` writer - RO"]
pub struct RO_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the OCMDRn\\[11:0\\]
are allowed"]
    #[inline(always)]
    pub fn ro_0(self) -> &'a mut W {
        self.variant(RO_A::RO_0)
    }
    #[doc = "Writes to the OCMDRn\\[11:0\\]
are ignored"]
    #[inline(always)]
    pub fn ro_1(self) -> &'a mut W {
        self.variant(RO_A::RO_1)
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
#[doc = "OCMW\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMW_A {
    #[doc = "2: OCMEMn 32-bits wide"]
    OCMW_2 = 2,
    #[doc = "3: OCMEMn 64-bits wide"]
    OCMW_3 = 3,
    #[doc = "4: OCMEMn 128-bits wide"]
    OCMW_4 = 4,
    #[doc = "5: OCMEMn 256-bits wide"]
    OCMW_5 = 5,
}
impl From<OCMW_A> for u8 {
    #[inline(always)]
    fn from(variant: OCMW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OCMW` reader - OCMW"]
pub struct OCMW_R(crate::FieldReader<u8, OCMW_A>);
impl OCMW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OCMW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OCMW_A> {
        match self.bits {
            2 => Some(OCMW_A::OCMW_2),
            3 => Some(OCMW_A::OCMW_3),
            4 => Some(OCMW_A::OCMW_4),
            5 => Some(OCMW_A::OCMW_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OCMW_2`"]
    #[inline(always)]
    pub fn is_ocmw_2(&self) -> bool {
        **self == OCMW_A::OCMW_2
    }
    #[doc = "Checks if the value of the field is `OCMW_3`"]
    #[inline(always)]
    pub fn is_ocmw_3(&self) -> bool {
        **self == OCMW_A::OCMW_3
    }
    #[doc = "Checks if the value of the field is `OCMW_4`"]
    #[inline(always)]
    pub fn is_ocmw_4(&self) -> bool {
        **self == OCMW_A::OCMW_4
    }
    #[doc = "Checks if the value of the field is `OCMW_5`"]
    #[inline(always)]
    pub fn is_ocmw_5(&self) -> bool {
        **self == OCMW_A::OCMW_5
    }
}
impl core::ops::Deref for OCMW_R {
    type Target = crate::FieldReader<u8, OCMW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OCMSZ\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMSZ_A {
    #[doc = "0: no OCMEMn"]
    OCMSZ_0 = 0,
    #[doc = "1: 1KB OCMEMn"]
    OCMSZ_1 = 1,
    #[doc = "2: 2KB OCMEMn"]
    OCMSZ_2 = 2,
    #[doc = "3: 4KB OCMEMn"]
    OCMSZ_3 = 3,
    #[doc = "4: 8KB OCMEMn"]
    OCMSZ_4 = 4,
    #[doc = "5: 16KB OCMEMn"]
    OCMSZ_5 = 5,
    #[doc = "6: 32KB OCMEMn"]
    OCMSZ_6 = 6,
    #[doc = "7: 64KB OCMEMn"]
    OCMSZ_7 = 7,
    #[doc = "8: 128KB OCMEMn"]
    OCMSZ_8 = 8,
    #[doc = "9: 256KB OCMEMn"]
    OCMSZ_9 = 9,
    #[doc = "10: 512KB OCMEMn"]
    OCMSZ_10 = 10,
    #[doc = "11: 1MB OCMEMn"]
    OCMSZ_11 = 11,
    #[doc = "12: 2MB OCMEMn"]
    OCMSZ_12 = 12,
    #[doc = "13: 4MB OCMEMn"]
    OCMSZ_13 = 13,
    #[doc = "14: 8MB OCMEMn"]
    OCMSZ_14 = 14,
    #[doc = "15: 16MB OCMEMn"]
    OCMSZ_15 = 15,
}
impl From<OCMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: OCMSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OCMSZ` reader - OCMSZ"]
pub struct OCMSZ_R(crate::FieldReader<u8, OCMSZ_A>);
impl OCMSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OCMSZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCMSZ_A {
        match self.bits {
            0 => OCMSZ_A::OCMSZ_0,
            1 => OCMSZ_A::OCMSZ_1,
            2 => OCMSZ_A::OCMSZ_2,
            3 => OCMSZ_A::OCMSZ_3,
            4 => OCMSZ_A::OCMSZ_4,
            5 => OCMSZ_A::OCMSZ_5,
            6 => OCMSZ_A::OCMSZ_6,
            7 => OCMSZ_A::OCMSZ_7,
            8 => OCMSZ_A::OCMSZ_8,
            9 => OCMSZ_A::OCMSZ_9,
            10 => OCMSZ_A::OCMSZ_10,
            11 => OCMSZ_A::OCMSZ_11,
            12 => OCMSZ_A::OCMSZ_12,
            13 => OCMSZ_A::OCMSZ_13,
            14 => OCMSZ_A::OCMSZ_14,
            15 => OCMSZ_A::OCMSZ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OCMSZ_0`"]
    #[inline(always)]
    pub fn is_ocmsz_0(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_0
    }
    #[doc = "Checks if the value of the field is `OCMSZ_1`"]
    #[inline(always)]
    pub fn is_ocmsz_1(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_1
    }
    #[doc = "Checks if the value of the field is `OCMSZ_2`"]
    #[inline(always)]
    pub fn is_ocmsz_2(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_2
    }
    #[doc = "Checks if the value of the field is `OCMSZ_3`"]
    #[inline(always)]
    pub fn is_ocmsz_3(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_3
    }
    #[doc = "Checks if the value of the field is `OCMSZ_4`"]
    #[inline(always)]
    pub fn is_ocmsz_4(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_4
    }
    #[doc = "Checks if the value of the field is `OCMSZ_5`"]
    #[inline(always)]
    pub fn is_ocmsz_5(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_5
    }
    #[doc = "Checks if the value of the field is `OCMSZ_6`"]
    #[inline(always)]
    pub fn is_ocmsz_6(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_6
    }
    #[doc = "Checks if the value of the field is `OCMSZ_7`"]
    #[inline(always)]
    pub fn is_ocmsz_7(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_7
    }
    #[doc = "Checks if the value of the field is `OCMSZ_8`"]
    #[inline(always)]
    pub fn is_ocmsz_8(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_8
    }
    #[doc = "Checks if the value of the field is `OCMSZ_9`"]
    #[inline(always)]
    pub fn is_ocmsz_9(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_9
    }
    #[doc = "Checks if the value of the field is `OCMSZ_10`"]
    #[inline(always)]
    pub fn is_ocmsz_10(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_10
    }
    #[doc = "Checks if the value of the field is `OCMSZ_11`"]
    #[inline(always)]
    pub fn is_ocmsz_11(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_11
    }
    #[doc = "Checks if the value of the field is `OCMSZ_12`"]
    #[inline(always)]
    pub fn is_ocmsz_12(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_12
    }
    #[doc = "Checks if the value of the field is `OCMSZ_13`"]
    #[inline(always)]
    pub fn is_ocmsz_13(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_13
    }
    #[doc = "Checks if the value of the field is `OCMSZ_14`"]
    #[inline(always)]
    pub fn is_ocmsz_14(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_14
    }
    #[doc = "Checks if the value of the field is `OCMSZ_15`"]
    #[inline(always)]
    pub fn is_ocmsz_15(&self) -> bool {
        **self == OCMSZ_A::OCMSZ_15
    }
}
impl core::ops::Deref for OCMSZ_R {
    type Target = crate::FieldReader<u8, OCMSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OCMSZH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMSZH_A {
    #[doc = "0: OCMEMn is a power-of-2 capacity."]
    OCMSZH_0 = 0,
    #[doc = "1: OCMEMn is not a power-of-2, with a capacity is 0.75 * OCMSZ."]
    OCMSZH_1 = 1,
}
impl From<OCMSZH_A> for bool {
    #[inline(always)]
    fn from(variant: OCMSZH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCMSZH` reader - OCMSZH"]
pub struct OCMSZH_R(crate::FieldReader<bool, OCMSZH_A>);
impl OCMSZH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCMSZH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCMSZH_A {
        match self.bits {
            false => OCMSZH_A::OCMSZH_0,
            true => OCMSZH_A::OCMSZH_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCMSZH_0`"]
    #[inline(always)]
    pub fn is_ocmszh_0(&self) -> bool {
        **self == OCMSZH_A::OCMSZH_0
    }
    #[doc = "Checks if the value of the field is `OCMSZH_1`"]
    #[inline(always)]
    pub fn is_ocmszh_1(&self) -> bool {
        **self == OCMSZH_A::OCMSZH_1
    }
}
impl core::ops::Deref for OCMSZH_R {
    type Target = crate::FieldReader<bool, OCMSZH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: OCMEMn is not present."]
    V_0 = 0,
    #[doc = "1: OCMEMn is present."]
    V_1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - V"]
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
    #[doc = "Bit 12 - OCMPU"]
    #[inline(always)]
    pub fn ocmpu(&self) -> OCMPU_R {
        OCMPU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - OCMT"]
    #[inline(always)]
    pub fn ocmt(&self) -> OCMT_R {
        OCMT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - RO"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - OCMW"]
    #[inline(always)]
    pub fn ocmw(&self) -> OCMW_R {
        OCMW_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - OCMSZ"]
    #[inline(always)]
    pub fn ocmsz(&self) -> OCMSZ_R {
        OCMSZ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - OCMSZH"]
    #[inline(always)]
    pub fn ocmszh(&self) -> OCMSZH_R {
        OCMSZH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - V"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - RO"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr2](index.html) module"]
pub struct OCMDR2_SPEC;
impl crate::RegisterSpec for OCMDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocmdr2::R](R) reader structure"]
impl crate::Readable for OCMDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocmdr2::W](W) writer structure"]
impl crate::Writable for OCMDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMDR2 to value 0xc304_d000"]
impl crate::Resettable for OCMDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc304_d000
    }
}
