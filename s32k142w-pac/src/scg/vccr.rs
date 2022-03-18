#[doc = "Register `VCCR` reader"]
pub struct R(crate::R<VCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCCR` writer"]
pub struct W(crate::W<VCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCCR_SPEC>;
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
impl From<crate::W<VCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slow Clock Divide Ratio\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVSLOW_A {
    #[doc = "0: Divide-by-1"]
    DIVSLOW_0 = 0,
    #[doc = "1: Divide-by-2"]
    DIVSLOW_1 = 1,
    #[doc = "2: Divide-by-3"]
    DIVSLOW_2 = 2,
    #[doc = "3: Divide-by-4"]
    DIVSLOW_3 = 3,
    #[doc = "4: Divide-by-5"]
    DIVSLOW_4 = 4,
    #[doc = "5: Divide-by-6"]
    DIVSLOW_5 = 5,
    #[doc = "6: Divide-by-7"]
    DIVSLOW_6 = 6,
    #[doc = "7: Divide-by-8"]
    DIVSLOW_7 = 7,
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
            0 => Some(DIVSLOW_A::DIVSLOW_0),
            1 => Some(DIVSLOW_A::DIVSLOW_1),
            2 => Some(DIVSLOW_A::DIVSLOW_2),
            3 => Some(DIVSLOW_A::DIVSLOW_3),
            4 => Some(DIVSLOW_A::DIVSLOW_4),
            5 => Some(DIVSLOW_A::DIVSLOW_5),
            6 => Some(DIVSLOW_A::DIVSLOW_6),
            7 => Some(DIVSLOW_A::DIVSLOW_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_0`"]
    #[inline(always)]
    pub fn is_divslow_0(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_0
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_1`"]
    #[inline(always)]
    pub fn is_divslow_1(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_1
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_2`"]
    #[inline(always)]
    pub fn is_divslow_2(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_2
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_3`"]
    #[inline(always)]
    pub fn is_divslow_3(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_3
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_4`"]
    #[inline(always)]
    pub fn is_divslow_4(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_4
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_5`"]
    #[inline(always)]
    pub fn is_divslow_5(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_5
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_6`"]
    #[inline(always)]
    pub fn is_divslow_6(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_6
    }
    #[doc = "Checks if the value of the field is `DIVSLOW_7`"]
    #[inline(always)]
    pub fn is_divslow_7(&self) -> bool {
        **self == DIVSLOW_A::DIVSLOW_7
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
    pub fn divslow_0(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn divslow_1(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_1)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn divslow_2(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_2)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn divslow_3(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_3)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn divslow_4(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_4)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn divslow_5(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_5)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn divslow_6(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_6)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn divslow_7(self) -> &'a mut W {
        self.variant(DIVSLOW_A::DIVSLOW_7)
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
    DIVBUS_0 = 0,
    #[doc = "1: Divide-by-2"]
    DIVBUS_1 = 1,
    #[doc = "2: Divide-by-3"]
    DIVBUS_2 = 2,
    #[doc = "3: Divide-by-4"]
    DIVBUS_3 = 3,
    #[doc = "4: Divide-by-5"]
    DIVBUS_4 = 4,
    #[doc = "5: Divide-by-6"]
    DIVBUS_5 = 5,
    #[doc = "6: Divide-by-7"]
    DIVBUS_6 = 6,
    #[doc = "7: Divide-by-8"]
    DIVBUS_7 = 7,
    #[doc = "8: Divide-by-9"]
    DIVBUS_8 = 8,
    #[doc = "9: Divide-by-10"]
    DIVBUS_9 = 9,
    #[doc = "10: Divide-by-11"]
    DIVBUS_10 = 10,
    #[doc = "11: Divide-by-12"]
    DIVBUS_11 = 11,
    #[doc = "12: Divide-by-13"]
    DIVBUS_12 = 12,
    #[doc = "13: Divide-by-14"]
    DIVBUS_13 = 13,
    #[doc = "14: Divide-by-15"]
    DIVBUS_14 = 14,
    #[doc = "15: Divide-by-16"]
    DIVBUS_15 = 15,
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
            0 => DIVBUS_A::DIVBUS_0,
            1 => DIVBUS_A::DIVBUS_1,
            2 => DIVBUS_A::DIVBUS_2,
            3 => DIVBUS_A::DIVBUS_3,
            4 => DIVBUS_A::DIVBUS_4,
            5 => DIVBUS_A::DIVBUS_5,
            6 => DIVBUS_A::DIVBUS_6,
            7 => DIVBUS_A::DIVBUS_7,
            8 => DIVBUS_A::DIVBUS_8,
            9 => DIVBUS_A::DIVBUS_9,
            10 => DIVBUS_A::DIVBUS_10,
            11 => DIVBUS_A::DIVBUS_11,
            12 => DIVBUS_A::DIVBUS_12,
            13 => DIVBUS_A::DIVBUS_13,
            14 => DIVBUS_A::DIVBUS_14,
            15 => DIVBUS_A::DIVBUS_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBUS_0`"]
    #[inline(always)]
    pub fn is_divbus_0(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_0
    }
    #[doc = "Checks if the value of the field is `DIVBUS_1`"]
    #[inline(always)]
    pub fn is_divbus_1(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_1
    }
    #[doc = "Checks if the value of the field is `DIVBUS_2`"]
    #[inline(always)]
    pub fn is_divbus_2(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_2
    }
    #[doc = "Checks if the value of the field is `DIVBUS_3`"]
    #[inline(always)]
    pub fn is_divbus_3(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_3
    }
    #[doc = "Checks if the value of the field is `DIVBUS_4`"]
    #[inline(always)]
    pub fn is_divbus_4(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_4
    }
    #[doc = "Checks if the value of the field is `DIVBUS_5`"]
    #[inline(always)]
    pub fn is_divbus_5(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_5
    }
    #[doc = "Checks if the value of the field is `DIVBUS_6`"]
    #[inline(always)]
    pub fn is_divbus_6(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_6
    }
    #[doc = "Checks if the value of the field is `DIVBUS_7`"]
    #[inline(always)]
    pub fn is_divbus_7(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_7
    }
    #[doc = "Checks if the value of the field is `DIVBUS_8`"]
    #[inline(always)]
    pub fn is_divbus_8(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_8
    }
    #[doc = "Checks if the value of the field is `DIVBUS_9`"]
    #[inline(always)]
    pub fn is_divbus_9(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_9
    }
    #[doc = "Checks if the value of the field is `DIVBUS_10`"]
    #[inline(always)]
    pub fn is_divbus_10(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_10
    }
    #[doc = "Checks if the value of the field is `DIVBUS_11`"]
    #[inline(always)]
    pub fn is_divbus_11(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_11
    }
    #[doc = "Checks if the value of the field is `DIVBUS_12`"]
    #[inline(always)]
    pub fn is_divbus_12(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_12
    }
    #[doc = "Checks if the value of the field is `DIVBUS_13`"]
    #[inline(always)]
    pub fn is_divbus_13(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_13
    }
    #[doc = "Checks if the value of the field is `DIVBUS_14`"]
    #[inline(always)]
    pub fn is_divbus_14(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_14
    }
    #[doc = "Checks if the value of the field is `DIVBUS_15`"]
    #[inline(always)]
    pub fn is_divbus_15(&self) -> bool {
        **self == DIVBUS_A::DIVBUS_15
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
    pub fn divbus_0(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn divbus_1(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_1)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn divbus_2(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_2)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn divbus_3(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_3)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn divbus_4(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_4)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn divbus_5(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_5)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn divbus_6(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_6)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn divbus_7(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_7)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn divbus_8(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_8)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn divbus_9(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_9)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn divbus_10(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_10)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn divbus_11(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_11)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn divbus_12(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_12)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn divbus_13(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_13)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn divbus_14(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_14)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn divbus_15(self) -> &'a mut W {
        self.variant(DIVBUS_A::DIVBUS_15)
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
    DIVCORE_0 = 0,
    #[doc = "1: Divide-by-2"]
    DIVCORE_1 = 1,
    #[doc = "2: Divide-by-3"]
    DIVCORE_2 = 2,
    #[doc = "3: Divide-by-4"]
    DIVCORE_3 = 3,
    #[doc = "4: Divide-by-5"]
    DIVCORE_4 = 4,
    #[doc = "5: Divide-by-6"]
    DIVCORE_5 = 5,
    #[doc = "6: Divide-by-7"]
    DIVCORE_6 = 6,
    #[doc = "7: Divide-by-8"]
    DIVCORE_7 = 7,
    #[doc = "8: Divide-by-9"]
    DIVCORE_8 = 8,
    #[doc = "9: Divide-by-10"]
    DIVCORE_9 = 9,
    #[doc = "10: Divide-by-11"]
    DIVCORE_10 = 10,
    #[doc = "11: Divide-by-12"]
    DIVCORE_11 = 11,
    #[doc = "12: Divide-by-13"]
    DIVCORE_12 = 12,
    #[doc = "13: Divide-by-14"]
    DIVCORE_13 = 13,
    #[doc = "14: Divide-by-15"]
    DIVCORE_14 = 14,
    #[doc = "15: Divide-by-16"]
    DIVCORE_15 = 15,
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
            0 => DIVCORE_A::DIVCORE_0,
            1 => DIVCORE_A::DIVCORE_1,
            2 => DIVCORE_A::DIVCORE_2,
            3 => DIVCORE_A::DIVCORE_3,
            4 => DIVCORE_A::DIVCORE_4,
            5 => DIVCORE_A::DIVCORE_5,
            6 => DIVCORE_A::DIVCORE_6,
            7 => DIVCORE_A::DIVCORE_7,
            8 => DIVCORE_A::DIVCORE_8,
            9 => DIVCORE_A::DIVCORE_9,
            10 => DIVCORE_A::DIVCORE_10,
            11 => DIVCORE_A::DIVCORE_11,
            12 => DIVCORE_A::DIVCORE_12,
            13 => DIVCORE_A::DIVCORE_13,
            14 => DIVCORE_A::DIVCORE_14,
            15 => DIVCORE_A::DIVCORE_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVCORE_0`"]
    #[inline(always)]
    pub fn is_divcore_0(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_0
    }
    #[doc = "Checks if the value of the field is `DIVCORE_1`"]
    #[inline(always)]
    pub fn is_divcore_1(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_1
    }
    #[doc = "Checks if the value of the field is `DIVCORE_2`"]
    #[inline(always)]
    pub fn is_divcore_2(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_2
    }
    #[doc = "Checks if the value of the field is `DIVCORE_3`"]
    #[inline(always)]
    pub fn is_divcore_3(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_3
    }
    #[doc = "Checks if the value of the field is `DIVCORE_4`"]
    #[inline(always)]
    pub fn is_divcore_4(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_4
    }
    #[doc = "Checks if the value of the field is `DIVCORE_5`"]
    #[inline(always)]
    pub fn is_divcore_5(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_5
    }
    #[doc = "Checks if the value of the field is `DIVCORE_6`"]
    #[inline(always)]
    pub fn is_divcore_6(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_6
    }
    #[doc = "Checks if the value of the field is `DIVCORE_7`"]
    #[inline(always)]
    pub fn is_divcore_7(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_7
    }
    #[doc = "Checks if the value of the field is `DIVCORE_8`"]
    #[inline(always)]
    pub fn is_divcore_8(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_8
    }
    #[doc = "Checks if the value of the field is `DIVCORE_9`"]
    #[inline(always)]
    pub fn is_divcore_9(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_9
    }
    #[doc = "Checks if the value of the field is `DIVCORE_10`"]
    #[inline(always)]
    pub fn is_divcore_10(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_10
    }
    #[doc = "Checks if the value of the field is `DIVCORE_11`"]
    #[inline(always)]
    pub fn is_divcore_11(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_11
    }
    #[doc = "Checks if the value of the field is `DIVCORE_12`"]
    #[inline(always)]
    pub fn is_divcore_12(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_12
    }
    #[doc = "Checks if the value of the field is `DIVCORE_13`"]
    #[inline(always)]
    pub fn is_divcore_13(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_13
    }
    #[doc = "Checks if the value of the field is `DIVCORE_14`"]
    #[inline(always)]
    pub fn is_divcore_14(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_14
    }
    #[doc = "Checks if the value of the field is `DIVCORE_15`"]
    #[inline(always)]
    pub fn is_divcore_15(&self) -> bool {
        **self == DIVCORE_A::DIVCORE_15
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
    pub fn divcore_0(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn divcore_1(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_1)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn divcore_2(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_2)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn divcore_3(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_3)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn divcore_4(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_4)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn divcore_5(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_5)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn divcore_6(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_6)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn divcore_7(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_7)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn divcore_8(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_8)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn divcore_9(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_9)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn divcore_10(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_10)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn divcore_11(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_11)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn divcore_12(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_12)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn divcore_13(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_13)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn divcore_14(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_14)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn divcore_15(self) -> &'a mut W {
        self.variant(DIVCORE_A::DIVCORE_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "System Clock Source\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "2: Slow IRC (SIRC_CLK)"]
    SCS_2 = 2,
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
            2 => Some(SCS_A::SCS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCS_2`"]
    #[inline(always)]
    pub fn is_scs_2(&self) -> bool {
        **self == SCS_A::SCS_2
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
    #[doc = "Slow IRC (SIRC_CLK)"]
    #[inline(always)]
    pub fn scs_2(self) -> &'a mut W {
        self.variant(SCS_A::SCS_2)
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
#[doc = "VLPR Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vccr](index.html) module"]
pub struct VCCR_SPEC;
impl crate::RegisterSpec for VCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vccr::R](R) reader structure"]
impl crate::Readable for VCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vccr::W](W) writer structure"]
impl crate::Writable for VCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCCR to value 0x0200_0001"]
impl crate::Resettable for VCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0001
    }
}
