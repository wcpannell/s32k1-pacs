#[doc = "Register `HCCR` reader"]
pub struct R(crate::R<HCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCR` writer"]
pub struct W(crate::W<HCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCR_SPEC>;
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
impl From<crate::W<HCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slow Clock Divide Ratio\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVSLOW_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
}
impl From<DIVSLOW_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSLOW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVSLOW` reader - Slow Clock Divide Ratio"]
pub struct DIVSLOW_R(crate::FieldReader<u8, DIVSLOW_A>);
impl DIVSLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVSLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVSLOW_A> {
        match self.bits {
            0 => Some(DIVSLOW_A::_0000),
            1 => Some(DIVSLOW_A::_0001),
            2 => Some(DIVSLOW_A::_0010),
            3 => Some(DIVSLOW_A::_0011),
            4 => Some(DIVSLOW_A::_0100),
            5 => Some(DIVSLOW_A::_0101),
            6 => Some(DIVSLOW_A::_0110),
            7 => Some(DIVSLOW_A::_0111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == DIVSLOW_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == DIVSLOW_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == DIVSLOW_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == DIVSLOW_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == DIVSLOW_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == DIVSLOW_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == DIVSLOW_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == DIVSLOW_A::_0111
    }
}
impl core::ops::Deref for DIVSLOW_R {
    type Target = crate::FieldReader<u8, DIVSLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVSLOW` writer - Slow Clock Divide Ratio"]
pub struct DIVSLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVSLOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVSLOW_A::_0111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Bus Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVBUS_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
    #[doc = "8: Divide-by-9"]
    _1000 = 8,
    #[doc = "9: Divide-by-10"]
    _1001 = 9,
    #[doc = "10: Divide-by-11"]
    _1010 = 10,
    #[doc = "11: Divide-by-12"]
    _1011 = 11,
    #[doc = "12: Divide-by-13"]
    _1100 = 12,
    #[doc = "13: Divide-by-14"]
    _1101 = 13,
    #[doc = "14: Divide-by-15"]
    _1110 = 14,
    #[doc = "15: Divide-by-16"]
    _1111 = 15,
}
impl From<DIVBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVBUS` reader - Bus Clock Divide Ratio"]
pub struct DIVBUS_R(crate::FieldReader<u8, DIVBUS_A>);
impl DIVBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVBUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVBUS_A {
        match self.bits {
            0 => DIVBUS_A::_0000,
            1 => DIVBUS_A::_0001,
            2 => DIVBUS_A::_0010,
            3 => DIVBUS_A::_0011,
            4 => DIVBUS_A::_0100,
            5 => DIVBUS_A::_0101,
            6 => DIVBUS_A::_0110,
            7 => DIVBUS_A::_0111,
            8 => DIVBUS_A::_1000,
            9 => DIVBUS_A::_1001,
            10 => DIVBUS_A::_1010,
            11 => DIVBUS_A::_1011,
            12 => DIVBUS_A::_1100,
            13 => DIVBUS_A::_1101,
            14 => DIVBUS_A::_1110,
            15 => DIVBUS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == DIVBUS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == DIVBUS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == DIVBUS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == DIVBUS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == DIVBUS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == DIVBUS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == DIVBUS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == DIVBUS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == DIVBUS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == DIVBUS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == DIVBUS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == DIVBUS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == DIVBUS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == DIVBUS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == DIVBUS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == DIVBUS_A::_1111
    }
}
impl core::ops::Deref for DIVBUS_R {
    type Target = crate::FieldReader<u8, DIVBUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVBUS` writer - Bus Clock Divide Ratio"]
pub struct DIVBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBUS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVBUS_A::_0111)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1000)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1001)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1010)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1011)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1100)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1101)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1110)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DIVBUS_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Core Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVCORE_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
    #[doc = "8: Divide-by-9"]
    _1000 = 8,
    #[doc = "9: Divide-by-10"]
    _1001 = 9,
    #[doc = "10: Divide-by-11"]
    _1010 = 10,
    #[doc = "11: Divide-by-12"]
    _1011 = 11,
    #[doc = "12: Divide-by-13"]
    _1100 = 12,
    #[doc = "13: Divide-by-14"]
    _1101 = 13,
    #[doc = "14: Divide-by-15"]
    _1110 = 14,
    #[doc = "15: Divide-by-16"]
    _1111 = 15,
}
impl From<DIVCORE_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVCORE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVCORE` reader - Core Clock Divide Ratio"]
pub struct DIVCORE_R(crate::FieldReader<u8, DIVCORE_A>);
impl DIVCORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVCORE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVCORE_A {
        match self.bits {
            0 => DIVCORE_A::_0000,
            1 => DIVCORE_A::_0001,
            2 => DIVCORE_A::_0010,
            3 => DIVCORE_A::_0011,
            4 => DIVCORE_A::_0100,
            5 => DIVCORE_A::_0101,
            6 => DIVCORE_A::_0110,
            7 => DIVCORE_A::_0111,
            8 => DIVCORE_A::_1000,
            9 => DIVCORE_A::_1001,
            10 => DIVCORE_A::_1010,
            11 => DIVCORE_A::_1011,
            12 => DIVCORE_A::_1100,
            13 => DIVCORE_A::_1101,
            14 => DIVCORE_A::_1110,
            15 => DIVCORE_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == DIVCORE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == DIVCORE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == DIVCORE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == DIVCORE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == DIVCORE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == DIVCORE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == DIVCORE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == DIVCORE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == DIVCORE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == DIVCORE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == DIVCORE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == DIVCORE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == DIVCORE_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == DIVCORE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == DIVCORE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == DIVCORE_A::_1111
    }
}
impl core::ops::Deref for DIVCORE_R {
    type Target = crate::FieldReader<u8, DIVCORE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVCORE` writer - Core Clock Divide Ratio"]
pub struct DIVCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCORE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVCORE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVCORE_A::_0111)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1000)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1001)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1010)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1011)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1100)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1101)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1110)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DIVCORE_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "System Clock Source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "3: Fast IRC (FIRC_CLK)"]
    _0011 = 3,
    #[doc = "6: System PLL (SPLL_CLK)"]
    _0110 = 6,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCS` reader - System Clock Source"]
pub struct SCS_R(crate::FieldReader<u8, SCS_A>);
impl SCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCS_A> {
        match self.bits {
            3 => Some(SCS_A::_0011),
            6 => Some(SCS_A::_0110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == SCS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == SCS_A::_0110
    }
}
impl core::ops::Deref for SCS_R {
    type Target = crate::FieldReader<u8, SCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCS` writer - System Clock Source"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fast IRC (FIRC_CLK)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SCS_A::_0011)
    }
    #[doc = "System PLL (SPLL_CLK)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SCS_A::_0110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
    #[inline(always)]
    pub fn divslow(&self) -> DIVSLOW_R {
        DIVSLOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
    #[inline(always)]
    pub fn divbus(&self) -> DIVBUS_R {
        DIVBUS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
    #[inline(always)]
    pub fn divcore(&self) -> DIVCORE_R {
        DIVCORE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - System Clock Source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
    #[inline(always)]
    pub fn divslow(&mut self) -> DIVSLOW_W {
        DIVSLOW_W { w: self }
    }
    #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
    #[inline(always)]
    pub fn divbus(&mut self) -> DIVBUS_W {
        DIVBUS_W { w: self }
    }
    #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
    #[inline(always)]
    pub fn divcore(&mut self) -> DIVCORE_W {
        DIVCORE_W { w: self }
    }
    #[doc = "Bits 24:27 - System Clock Source"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSRUN Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccr](index.html) module"]
pub struct HCCR_SPEC;
impl crate::RegisterSpec for HCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccr::R](R) reader structure"]
impl crate::Readable for HCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccr::W](W) writer structure"]
impl crate::Writable for HCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCR to value 0x0300_0001"]
impl crate::Resettable for HCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0001
    }
}
