#[doc = "Register `aSC1%s` reader"]
pub struct R(crate::R<ASC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aSC1%s` writer"]
pub struct W(crate::W<ASC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASC1_SPEC>;
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
impl From<crate::W<ASC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input channel select\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: Exernal intput channel 0 is selected."]
    _000000 = 0,
    #[doc = "1: Exernal channel 1 is selected as input."]
    _000001 = 1,
    #[doc = "2: Exernal channel 2 is selected as input."]
    _000010 = 2,
    #[doc = "3: Exernal channel 3 is selected as input."]
    _000011 = 3,
    #[doc = "4: Exernal channel 4 is selected as input."]
    _000100 = 4,
    #[doc = "5: Exernal channel 5 is selected as input."]
    _000101 = 5,
    #[doc = "6: Exernal channel 6 is selected as input."]
    _000110 = 6,
    #[doc = "7: Exernal channel 7 is selected as input."]
    _000111 = 7,
    #[doc = "8: Exernal channel 8 is selected as input."]
    _001000 = 8,
    #[doc = "9: Exernal channel 9 is selected as input."]
    _01001 = 9,
    #[doc = "10: Exernal channel 10 is selected as input."]
    _001010 = 10,
    #[doc = "11: Exernal channel 11 is selected as input."]
    _001011 = 11,
    #[doc = "12: Exernal channel 12 is selected as input."]
    _001100 = 12,
    #[doc = "13: Exernal channel 13 is selected as input."]
    _001101 = 13,
    #[doc = "14: Exernal channel 14 is selected as input."]
    _001110 = 14,
    #[doc = "15: Exernal channel 15 is selected as input."]
    _001111 = 15,
    #[doc = "21: Internal channel 0 is selected as input."]
    _010101 = 21,
    #[doc = "22: Internal channel 1 is selected as input."]
    _010110 = 22,
    #[doc = "23: Internal channel 2 is selected as input."]
    _010111 = 23,
    #[doc = "27: Band Gap"]
    _011011 = 27,
    #[doc = "28: Internal channel 3 is selected as input."]
    _011100 = 28,
    #[doc = "29: VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _011101 = 29,
    #[doc = "30: VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _011110 = 30,
    #[doc = "31: Module is disabled"]
    _011111 = 31,
    #[doc = "32: Internal channel 16 is selected as input."]
    _100000 = 32,
    #[doc = "33: Exernal channel 17 is selected as input."]
    _100001 = 33,
    #[doc = "34: Exernal channel 18 is selected as input."]
    _100010 = 34,
    #[doc = "35: Exernal channel 19 is selected as input."]
    _100011 = 35,
    #[doc = "36: Exernal channel 20 is selected as input."]
    _100100 = 36,
    #[doc = "37: Exernal channel 21 is selected as input."]
    _100101 = 37,
    #[doc = "38: Exernal channel 22 is selected as input."]
    _100110 = 38,
    #[doc = "39: Exernal channel 23 is selected as input."]
    _100111 = 39,
    #[doc = "40: Exernal channel 24 is selected as input."]
    _101000 = 40,
    #[doc = "41: Exernal channel 25 is selected as input."]
    _101001 = 41,
    #[doc = "42: Exernal channel 26 is selected as input."]
    _101010 = 42,
    #[doc = "43: Exernal channel 27 is selected as input."]
    _101011 = 43,
    #[doc = "44: Exernal channel 28 is selected as input."]
    _101100 = 44,
    #[doc = "45: Exernal channel 29 is selected as input."]
    _101101 = 45,
    #[doc = "46: Exernal channel 30 is selected as input."]
    _101110 = 46,
    #[doc = "47: Exernal channel 31 is selected as input."]
    _101111 = 47,
    #[doc = "48: Module is disabled"]
    _11XXXX = 48,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCH` reader - Input channel select"]
pub struct ADCH_R(crate::FieldReader<u8, ADCH_A>);
impl ADCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::_000000),
            1 => Some(ADCH_A::_000001),
            2 => Some(ADCH_A::_000010),
            3 => Some(ADCH_A::_000011),
            4 => Some(ADCH_A::_000100),
            5 => Some(ADCH_A::_000101),
            6 => Some(ADCH_A::_000110),
            7 => Some(ADCH_A::_000111),
            8 => Some(ADCH_A::_001000),
            9 => Some(ADCH_A::_01001),
            10 => Some(ADCH_A::_001010),
            11 => Some(ADCH_A::_001011),
            12 => Some(ADCH_A::_001100),
            13 => Some(ADCH_A::_001101),
            14 => Some(ADCH_A::_001110),
            15 => Some(ADCH_A::_001111),
            21 => Some(ADCH_A::_010101),
            22 => Some(ADCH_A::_010110),
            23 => Some(ADCH_A::_010111),
            27 => Some(ADCH_A::_011011),
            28 => Some(ADCH_A::_011100),
            29 => Some(ADCH_A::_011101),
            30 => Some(ADCH_A::_011110),
            31 => Some(ADCH_A::_011111),
            32 => Some(ADCH_A::_100000),
            33 => Some(ADCH_A::_100001),
            34 => Some(ADCH_A::_100010),
            35 => Some(ADCH_A::_100011),
            36 => Some(ADCH_A::_100100),
            37 => Some(ADCH_A::_100101),
            38 => Some(ADCH_A::_100110),
            39 => Some(ADCH_A::_100111),
            40 => Some(ADCH_A::_101000),
            41 => Some(ADCH_A::_101001),
            42 => Some(ADCH_A::_101010),
            43 => Some(ADCH_A::_101011),
            44 => Some(ADCH_A::_101100),
            45 => Some(ADCH_A::_101101),
            46 => Some(ADCH_A::_101110),
            47 => Some(ADCH_A::_101111),
            48 => Some(ADCH_A::_11XXXX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        **self == ADCH_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        **self == ADCH_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        **self == ADCH_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        **self == ADCH_A::_000011
    }
    #[doc = "Checks if the value of the field is `_000100`"]
    #[inline(always)]
    pub fn is_000100(&self) -> bool {
        **self == ADCH_A::_000100
    }
    #[doc = "Checks if the value of the field is `_000101`"]
    #[inline(always)]
    pub fn is_000101(&self) -> bool {
        **self == ADCH_A::_000101
    }
    #[doc = "Checks if the value of the field is `_000110`"]
    #[inline(always)]
    pub fn is_000110(&self) -> bool {
        **self == ADCH_A::_000110
    }
    #[doc = "Checks if the value of the field is `_000111`"]
    #[inline(always)]
    pub fn is_000111(&self) -> bool {
        **self == ADCH_A::_000111
    }
    #[doc = "Checks if the value of the field is `_001000`"]
    #[inline(always)]
    pub fn is_001000(&self) -> bool {
        **self == ADCH_A::_001000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        **self == ADCH_A::_01001
    }
    #[doc = "Checks if the value of the field is `_001010`"]
    #[inline(always)]
    pub fn is_001010(&self) -> bool {
        **self == ADCH_A::_001010
    }
    #[doc = "Checks if the value of the field is `_001011`"]
    #[inline(always)]
    pub fn is_001011(&self) -> bool {
        **self == ADCH_A::_001011
    }
    #[doc = "Checks if the value of the field is `_001100`"]
    #[inline(always)]
    pub fn is_001100(&self) -> bool {
        **self == ADCH_A::_001100
    }
    #[doc = "Checks if the value of the field is `_001101`"]
    #[inline(always)]
    pub fn is_001101(&self) -> bool {
        **self == ADCH_A::_001101
    }
    #[doc = "Checks if the value of the field is `_001110`"]
    #[inline(always)]
    pub fn is_001110(&self) -> bool {
        **self == ADCH_A::_001110
    }
    #[doc = "Checks if the value of the field is `_001111`"]
    #[inline(always)]
    pub fn is_001111(&self) -> bool {
        **self == ADCH_A::_001111
    }
    #[doc = "Checks if the value of the field is `_010101`"]
    #[inline(always)]
    pub fn is_010101(&self) -> bool {
        **self == ADCH_A::_010101
    }
    #[doc = "Checks if the value of the field is `_010110`"]
    #[inline(always)]
    pub fn is_010110(&self) -> bool {
        **self == ADCH_A::_010110
    }
    #[doc = "Checks if the value of the field is `_010111`"]
    #[inline(always)]
    pub fn is_010111(&self) -> bool {
        **self == ADCH_A::_010111
    }
    #[doc = "Checks if the value of the field is `_011011`"]
    #[inline(always)]
    pub fn is_011011(&self) -> bool {
        **self == ADCH_A::_011011
    }
    #[doc = "Checks if the value of the field is `_011100`"]
    #[inline(always)]
    pub fn is_011100(&self) -> bool {
        **self == ADCH_A::_011100
    }
    #[doc = "Checks if the value of the field is `_011101`"]
    #[inline(always)]
    pub fn is_011101(&self) -> bool {
        **self == ADCH_A::_011101
    }
    #[doc = "Checks if the value of the field is `_011110`"]
    #[inline(always)]
    pub fn is_011110(&self) -> bool {
        **self == ADCH_A::_011110
    }
    #[doc = "Checks if the value of the field is `_011111`"]
    #[inline(always)]
    pub fn is_011111(&self) -> bool {
        **self == ADCH_A::_011111
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        **self == ADCH_A::_100000
    }
    #[doc = "Checks if the value of the field is `_100001`"]
    #[inline(always)]
    pub fn is_100001(&self) -> bool {
        **self == ADCH_A::_100001
    }
    #[doc = "Checks if the value of the field is `_100010`"]
    #[inline(always)]
    pub fn is_100010(&self) -> bool {
        **self == ADCH_A::_100010
    }
    #[doc = "Checks if the value of the field is `_100011`"]
    #[inline(always)]
    pub fn is_100011(&self) -> bool {
        **self == ADCH_A::_100011
    }
    #[doc = "Checks if the value of the field is `_100100`"]
    #[inline(always)]
    pub fn is_100100(&self) -> bool {
        **self == ADCH_A::_100100
    }
    #[doc = "Checks if the value of the field is `_100101`"]
    #[inline(always)]
    pub fn is_100101(&self) -> bool {
        **self == ADCH_A::_100101
    }
    #[doc = "Checks if the value of the field is `_100110`"]
    #[inline(always)]
    pub fn is_100110(&self) -> bool {
        **self == ADCH_A::_100110
    }
    #[doc = "Checks if the value of the field is `_100111`"]
    #[inline(always)]
    pub fn is_100111(&self) -> bool {
        **self == ADCH_A::_100111
    }
    #[doc = "Checks if the value of the field is `_101000`"]
    #[inline(always)]
    pub fn is_101000(&self) -> bool {
        **self == ADCH_A::_101000
    }
    #[doc = "Checks if the value of the field is `_101001`"]
    #[inline(always)]
    pub fn is_101001(&self) -> bool {
        **self == ADCH_A::_101001
    }
    #[doc = "Checks if the value of the field is `_101010`"]
    #[inline(always)]
    pub fn is_101010(&self) -> bool {
        **self == ADCH_A::_101010
    }
    #[doc = "Checks if the value of the field is `_101011`"]
    #[inline(always)]
    pub fn is_101011(&self) -> bool {
        **self == ADCH_A::_101011
    }
    #[doc = "Checks if the value of the field is `_101100`"]
    #[inline(always)]
    pub fn is_101100(&self) -> bool {
        **self == ADCH_A::_101100
    }
    #[doc = "Checks if the value of the field is `_101101`"]
    #[inline(always)]
    pub fn is_101101(&self) -> bool {
        **self == ADCH_A::_101101
    }
    #[doc = "Checks if the value of the field is `_101110`"]
    #[inline(always)]
    pub fn is_101110(&self) -> bool {
        **self == ADCH_A::_101110
    }
    #[doc = "Checks if the value of the field is `_101111`"]
    #[inline(always)]
    pub fn is_101111(&self) -> bool {
        **self == ADCH_A::_101111
    }
    #[doc = "Checks if the value of the field is `_11XXXX`"]
    #[inline(always)]
    pub fn is_11xxxx(&self) -> bool {
        **self == ADCH_A::_11XXXX
    }
}
impl core::ops::Deref for ADCH_R {
    type Target = crate::FieldReader<u8, ADCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Exernal intput channel 0 is selected."]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(ADCH_A::_000000)
    }
    #[doc = "Exernal channel 1 is selected as input."]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(ADCH_A::_000001)
    }
    #[doc = "Exernal channel 2 is selected as input."]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(ADCH_A::_000010)
    }
    #[doc = "Exernal channel 3 is selected as input."]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut W {
        self.variant(ADCH_A::_000011)
    }
    #[doc = "Exernal channel 4 is selected as input."]
    #[inline(always)]
    pub fn _000100(self) -> &'a mut W {
        self.variant(ADCH_A::_000100)
    }
    #[doc = "Exernal channel 5 is selected as input."]
    #[inline(always)]
    pub fn _000101(self) -> &'a mut W {
        self.variant(ADCH_A::_000101)
    }
    #[doc = "Exernal channel 6 is selected as input."]
    #[inline(always)]
    pub fn _000110(self) -> &'a mut W {
        self.variant(ADCH_A::_000110)
    }
    #[doc = "Exernal channel 7 is selected as input."]
    #[inline(always)]
    pub fn _000111(self) -> &'a mut W {
        self.variant(ADCH_A::_000111)
    }
    #[doc = "Exernal channel 8 is selected as input."]
    #[inline(always)]
    pub fn _001000(self) -> &'a mut W {
        self.variant(ADCH_A::_001000)
    }
    #[doc = "Exernal channel 9 is selected as input."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(ADCH_A::_01001)
    }
    #[doc = "Exernal channel 10 is selected as input."]
    #[inline(always)]
    pub fn _001010(self) -> &'a mut W {
        self.variant(ADCH_A::_001010)
    }
    #[doc = "Exernal channel 11 is selected as input."]
    #[inline(always)]
    pub fn _001011(self) -> &'a mut W {
        self.variant(ADCH_A::_001011)
    }
    #[doc = "Exernal channel 12 is selected as input."]
    #[inline(always)]
    pub fn _001100(self) -> &'a mut W {
        self.variant(ADCH_A::_001100)
    }
    #[doc = "Exernal channel 13 is selected as input."]
    #[inline(always)]
    pub fn _001101(self) -> &'a mut W {
        self.variant(ADCH_A::_001101)
    }
    #[doc = "Exernal channel 14 is selected as input."]
    #[inline(always)]
    pub fn _001110(self) -> &'a mut W {
        self.variant(ADCH_A::_001110)
    }
    #[doc = "Exernal channel 15 is selected as input."]
    #[inline(always)]
    pub fn _001111(self) -> &'a mut W {
        self.variant(ADCH_A::_001111)
    }
    #[doc = "Internal channel 0 is selected as input."]
    #[inline(always)]
    pub fn _010101(self) -> &'a mut W {
        self.variant(ADCH_A::_010101)
    }
    #[doc = "Internal channel 1 is selected as input."]
    #[inline(always)]
    pub fn _010110(self) -> &'a mut W {
        self.variant(ADCH_A::_010110)
    }
    #[doc = "Internal channel 2 is selected as input."]
    #[inline(always)]
    pub fn _010111(self) -> &'a mut W {
        self.variant(ADCH_A::_010111)
    }
    #[doc = "Band Gap"]
    #[inline(always)]
    pub fn _011011(self) -> &'a mut W {
        self.variant(ADCH_A::_011011)
    }
    #[doc = "Internal channel 3 is selected as input."]
    #[inline(always)]
    pub fn _011100(self) -> &'a mut W {
        self.variant(ADCH_A::_011100)
    }
    #[doc = "VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _011101(self) -> &'a mut W {
        self.variant(ADCH_A::_011101)
    }
    #[doc = "VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _011110(self) -> &'a mut W {
        self.variant(ADCH_A::_011110)
    }
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn _011111(self) -> &'a mut W {
        self.variant(ADCH_A::_011111)
    }
    #[doc = "Internal channel 16 is selected as input."]
    #[inline(always)]
    pub fn _100000(self) -> &'a mut W {
        self.variant(ADCH_A::_100000)
    }
    #[doc = "Exernal channel 17 is selected as input."]
    #[inline(always)]
    pub fn _100001(self) -> &'a mut W {
        self.variant(ADCH_A::_100001)
    }
    #[doc = "Exernal channel 18 is selected as input."]
    #[inline(always)]
    pub fn _100010(self) -> &'a mut W {
        self.variant(ADCH_A::_100010)
    }
    #[doc = "Exernal channel 19 is selected as input."]
    #[inline(always)]
    pub fn _100011(self) -> &'a mut W {
        self.variant(ADCH_A::_100011)
    }
    #[doc = "Exernal channel 20 is selected as input."]
    #[inline(always)]
    pub fn _100100(self) -> &'a mut W {
        self.variant(ADCH_A::_100100)
    }
    #[doc = "Exernal channel 21 is selected as input."]
    #[inline(always)]
    pub fn _100101(self) -> &'a mut W {
        self.variant(ADCH_A::_100101)
    }
    #[doc = "Exernal channel 22 is selected as input."]
    #[inline(always)]
    pub fn _100110(self) -> &'a mut W {
        self.variant(ADCH_A::_100110)
    }
    #[doc = "Exernal channel 23 is selected as input."]
    #[inline(always)]
    pub fn _100111(self) -> &'a mut W {
        self.variant(ADCH_A::_100111)
    }
    #[doc = "Exernal channel 24 is selected as input."]
    #[inline(always)]
    pub fn _101000(self) -> &'a mut W {
        self.variant(ADCH_A::_101000)
    }
    #[doc = "Exernal channel 25 is selected as input."]
    #[inline(always)]
    pub fn _101001(self) -> &'a mut W {
        self.variant(ADCH_A::_101001)
    }
    #[doc = "Exernal channel 26 is selected as input."]
    #[inline(always)]
    pub fn _101010(self) -> &'a mut W {
        self.variant(ADCH_A::_101010)
    }
    #[doc = "Exernal channel 27 is selected as input."]
    #[inline(always)]
    pub fn _101011(self) -> &'a mut W {
        self.variant(ADCH_A::_101011)
    }
    #[doc = "Exernal channel 28 is selected as input."]
    #[inline(always)]
    pub fn _101100(self) -> &'a mut W {
        self.variant(ADCH_A::_101100)
    }
    #[doc = "Exernal channel 29 is selected as input."]
    #[inline(always)]
    pub fn _101101(self) -> &'a mut W {
        self.variant(ADCH_A::_101101)
    }
    #[doc = "Exernal channel 30 is selected as input."]
    #[inline(always)]
    pub fn _101110(self) -> &'a mut W {
        self.variant(ADCH_A::_101110)
    }
    #[doc = "Exernal channel 31 is selected as input."]
    #[inline(always)]
    pub fn _101111(self) -> &'a mut W {
        self.variant(ADCH_A::_101111)
    }
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn _11xxxx(self) -> &'a mut W {
        self.variant(ADCH_A::_11XXXX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Conversion complete interrupt is enabled."]
    _1 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIEN` reader - Interrupt Enable"]
pub struct AIEN_R(crate::FieldReader<bool, AIEN_A>);
impl AIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::_0,
            true => AIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AIEN_A::_1
    }
}
impl core::ops::Deref for AIEN_R {
    type Target = crate::FieldReader<bool, AIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIEN` writer - Interrupt Enable"]
pub struct AIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIEN_A::_0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Conversion Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COCO_A {
    #[doc = "0: Conversion is not completed."]
    _0 = 0,
    #[doc = "1: Conversion is completed."]
    _1 = 1,
}
impl From<COCO_A> for bool {
    #[inline(always)]
    fn from(variant: COCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COCO` reader - Conversion Complete Flag"]
pub struct COCO_R(crate::FieldReader<bool, COCO_A>);
impl COCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COCO_A {
        match self.bits {
            false => COCO_A::_0,
            true => COCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COCO_A::_1
    }
}
impl core::ops::Deref for COCO_R {
    type Target = crate::FieldReader<bool, COCO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco(&self) -> COCO_R {
        COCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&mut self) -> AIEN_W {
        AIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Status and Control Register 1 (alias)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a_sc1](index.html) module"]
pub struct ASC1_SPEC;
impl crate::RegisterSpec for ASC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a_sc1::R](R) reader structure"]
impl crate::Readable for ASC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a_sc1::W](W) writer structure"]
impl crate::Writable for ASC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets aSC1%s to value 0x3f"]
impl crate::Resettable for ASC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
