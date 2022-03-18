#[doc = "Register `OCMDR1` reader"]
pub struct R(crate::R<OCMDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMDR1` writer"]
pub struct W(crate::W<OCMDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMDR1_SPEC>;
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
impl From<crate::W<OCMDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCM1` reader - OCMEM Control Field 1"]
pub struct OCM1_R(crate::FieldReader<u8, u8>);
impl OCM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OCM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCM1` writer - OCMEM Control Field 1"]
pub struct OCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OCM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
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
#[doc = "OCMT\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMT_A {
    #[doc = "4: OCMEMn is a Program Flash."]
    _100 = 4,
    #[doc = "5: OCMEMn is a Data Flash."]
    _101 = 5,
    #[doc = "6: OCMEMn is an EEE."]
    _110 = 6,
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
            4 => Some(OCMT_A::_100),
            5 => Some(OCMT_A::_101),
            6 => Some(OCMT_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == OCMT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == OCMT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == OCMT_A::_110
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
    _0 = 0,
    #[doc = "1: Writes to the OCMDRn\\[11:0\\]
are ignored"]
    _1 = 1,
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
            false => RO_A::_0,
            true => RO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RO_A::_1
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
    pub fn _0(self) -> &'a mut W {
        self.variant(RO_A::_0)
    }
    #[doc = "Writes to the OCMDRn\\[11:0\\]
are ignored"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO_A::_1)
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
#[doc = "OCMW\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMW_A {
    #[doc = "2: OCMEMn 32-bits wide"]
    _010 = 2,
    #[doc = "3: OCMEMn 64-bits wide"]
    _011 = 3,
    #[doc = "4: OCMEMn 128-bits wide"]
    _100 = 4,
    #[doc = "5: OCMEMn 256-bits wide"]
    _101 = 5,
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
            2 => Some(OCMW_A::_010),
            3 => Some(OCMW_A::_011),
            4 => Some(OCMW_A::_100),
            5 => Some(OCMW_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == OCMW_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == OCMW_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == OCMW_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == OCMW_A::_101
    }
}
impl core::ops::Deref for OCMW_R {
    type Target = crate::FieldReader<u8, OCMW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OCMSZ\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OCMSZ_A {
    #[doc = "0: no OCMEMn"]
    _0000 = 0,
    #[doc = "1: 1KB OCMEMn"]
    _0001 = 1,
    #[doc = "2: 2KB OCMEMn"]
    _0010 = 2,
    #[doc = "3: 4KB OCMEMn"]
    _0011 = 3,
    #[doc = "4: 8KB OCMEMn"]
    _0100 = 4,
    #[doc = "5: 16KB OCMEMn"]
    _0101 = 5,
    #[doc = "6: 32KB OCMEMn"]
    _0110 = 6,
    #[doc = "7: 64KB OCMEMn"]
    _0111 = 7,
    #[doc = "8: 128KB OCMEMn"]
    _1000 = 8,
    #[doc = "9: 256KB OCMEMn"]
    _1001 = 9,
    #[doc = "10: 512KB OCMEMn"]
    _1010 = 10,
    #[doc = "11: 1MB OCMEMn"]
    _1011 = 11,
    #[doc = "12: 2MB OCMEMn"]
    _1100 = 12,
    #[doc = "13: 4MB OCMEMn"]
    _1101 = 13,
    #[doc = "14: 8MB OCMEMn"]
    _1110 = 14,
    #[doc = "15: 16MB OCMEMn"]
    _1111 = 15,
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
            0 => OCMSZ_A::_0000,
            1 => OCMSZ_A::_0001,
            2 => OCMSZ_A::_0010,
            3 => OCMSZ_A::_0011,
            4 => OCMSZ_A::_0100,
            5 => OCMSZ_A::_0101,
            6 => OCMSZ_A::_0110,
            7 => OCMSZ_A::_0111,
            8 => OCMSZ_A::_1000,
            9 => OCMSZ_A::_1001,
            10 => OCMSZ_A::_1010,
            11 => OCMSZ_A::_1011,
            12 => OCMSZ_A::_1100,
            13 => OCMSZ_A::_1101,
            14 => OCMSZ_A::_1110,
            15 => OCMSZ_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == OCMSZ_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == OCMSZ_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == OCMSZ_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == OCMSZ_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == OCMSZ_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == OCMSZ_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == OCMSZ_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == OCMSZ_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == OCMSZ_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == OCMSZ_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == OCMSZ_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == OCMSZ_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == OCMSZ_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == OCMSZ_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == OCMSZ_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == OCMSZ_A::_1111
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
    _0 = 0,
    #[doc = "1: OCMEMn is not a power-of-2, with a capacity is 0.75 * OCMSZ."]
    _1 = 1,
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
            false => OCMSZH_A::_0,
            true => OCMSZH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCMSZH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCMSZH_A::_1
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
    _0 = 0,
    #[doc = "1: OCMEMn is present."]
    _1 = 1,
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
    #[doc = "Bits 4:5 - OCMEM Control Field 1"]
    #[inline(always)]
    pub fn ocm1(&self) -> OCM1_R {
        OCM1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
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
    #[doc = "Bits 4:5 - OCMEM Control Field 1"]
    #[inline(always)]
    pub fn ocm1(&mut self) -> OCM1_W {
        OCM1_W { w: self }
    }
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
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr1](index.html) module"]
pub struct OCMDR1_SPEC;
impl crate::RegisterSpec for OCMDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocmdr1::R](R) reader structure"]
impl crate::Readable for OCMDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocmdr1::W](W) writer structure"]
impl crate::Writable for OCMDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMDR1 to value 0xca08_b000"]
impl crate::Resettable for OCMDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xca08_b000
    }
}
