#[doc = "Register `SC1%s` reader"]
pub struct R(crate::R<SC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC1%s` writer"]
pub struct W(crate::W<SC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC1_SPEC>;
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
impl From<crate::W<SC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input channel select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: Exernal channel 0 is selected as input."]
    _00000 = 0,
    #[doc = "1: Exernal channel 1 is selected as input."]
    _00001 = 1,
    #[doc = "2: Exernal channel 2 is selected as input."]
    _00010 = 2,
    #[doc = "3: Exernal channel 3 is selected as input."]
    _00011 = 3,
    #[doc = "4: Exernal channel 4 is selected as input."]
    _00100 = 4,
    #[doc = "5: Exernal channel 5 is selected as input."]
    _00101 = 5,
    #[doc = "6: Exernal channel 6 is selected as input."]
    _00110 = 6,
    #[doc = "7: Exernal channel 7 is selected as input."]
    _00111 = 7,
    #[doc = "8: Exernal channel 8 is selected as input."]
    _01000 = 8,
    #[doc = "9: Exernal channel 9 is selected as input."]
    _01001 = 9,
    #[doc = "10: Exernal channel 10 is selected as input."]
    _01010 = 10,
    #[doc = "11: Exernal channel 11 is selected as input."]
    _01011 = 11,
    #[doc = "12: Exernal channel 12 is selected as input."]
    _01100 = 12,
    #[doc = "13: Exernal channel 13 is selected as input."]
    _01101 = 13,
    #[doc = "14: Exernal channel 14 is selected as input."]
    _01110 = 14,
    #[doc = "15: Exernal channel 15 is selected as input."]
    _01111 = 15,
    #[doc = "18: Exernal channel 18 is selected as input."]
    _10010 = 18,
    #[doc = "19: Exernal channel 19 is selected as input."]
    _10011 = 19,
    #[doc = "21: Internal channel 0 is selected as input."]
    _10101 = 21,
    #[doc = "22: Internal channel 1 is selected as input."]
    _10110 = 22,
    #[doc = "23: Internal channel 2 is selected as input."]
    _10111 = 23,
    #[doc = "26: Temp Sensor"]
    _11010 = 26,
    #[doc = "27: Band Gap"]
    _11011 = 27,
    #[doc = "28: Internal channel 3 is selected as input."]
    _11100 = 28,
    #[doc = "29: VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11101 = 29,
    #[doc = "30: VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11110 = 30,
    #[doc = "31: Module is disabled"]
    _11111 = 31,
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
            0 => Some(ADCH_A::_00000),
            1 => Some(ADCH_A::_00001),
            2 => Some(ADCH_A::_00010),
            3 => Some(ADCH_A::_00011),
            4 => Some(ADCH_A::_00100),
            5 => Some(ADCH_A::_00101),
            6 => Some(ADCH_A::_00110),
            7 => Some(ADCH_A::_00111),
            8 => Some(ADCH_A::_01000),
            9 => Some(ADCH_A::_01001),
            10 => Some(ADCH_A::_01010),
            11 => Some(ADCH_A::_01011),
            12 => Some(ADCH_A::_01100),
            13 => Some(ADCH_A::_01101),
            14 => Some(ADCH_A::_01110),
            15 => Some(ADCH_A::_01111),
            18 => Some(ADCH_A::_10010),
            19 => Some(ADCH_A::_10011),
            21 => Some(ADCH_A::_10101),
            22 => Some(ADCH_A::_10110),
            23 => Some(ADCH_A::_10111),
            26 => Some(ADCH_A::_11010),
            27 => Some(ADCH_A::_11011),
            28 => Some(ADCH_A::_11100),
            29 => Some(ADCH_A::_11101),
            30 => Some(ADCH_A::_11110),
            31 => Some(ADCH_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        **self == ADCH_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        **self == ADCH_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        **self == ADCH_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        **self == ADCH_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        **self == ADCH_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        **self == ADCH_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        **self == ADCH_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        **self == ADCH_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        **self == ADCH_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        **self == ADCH_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        **self == ADCH_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        **self == ADCH_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        **self == ADCH_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        **self == ADCH_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        **self == ADCH_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        **self == ADCH_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        **self == ADCH_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        **self == ADCH_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        **self == ADCH_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        **self == ADCH_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        **self == ADCH_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        **self == ADCH_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        **self == ADCH_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        **self == ADCH_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        **self == ADCH_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        **self == ADCH_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        **self == ADCH_A::_11111
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
    #[doc = "Exernal channel 0 is selected as input."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(ADCH_A::_00000)
    }
    #[doc = "Exernal channel 1 is selected as input."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(ADCH_A::_00001)
    }
    #[doc = "Exernal channel 2 is selected as input."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(ADCH_A::_00010)
    }
    #[doc = "Exernal channel 3 is selected as input."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(ADCH_A::_00011)
    }
    #[doc = "Exernal channel 4 is selected as input."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(ADCH_A::_00100)
    }
    #[doc = "Exernal channel 5 is selected as input."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(ADCH_A::_00101)
    }
    #[doc = "Exernal channel 6 is selected as input."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(ADCH_A::_00110)
    }
    #[doc = "Exernal channel 7 is selected as input."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(ADCH_A::_00111)
    }
    #[doc = "Exernal channel 8 is selected as input."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(ADCH_A::_01000)
    }
    #[doc = "Exernal channel 9 is selected as input."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(ADCH_A::_01001)
    }
    #[doc = "Exernal channel 10 is selected as input."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(ADCH_A::_01010)
    }
    #[doc = "Exernal channel 11 is selected as input."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(ADCH_A::_01011)
    }
    #[doc = "Exernal channel 12 is selected as input."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(ADCH_A::_01100)
    }
    #[doc = "Exernal channel 13 is selected as input."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(ADCH_A::_01101)
    }
    #[doc = "Exernal channel 14 is selected as input."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(ADCH_A::_01110)
    }
    #[doc = "Exernal channel 15 is selected as input."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(ADCH_A::_01111)
    }
    #[doc = "Exernal channel 18 is selected as input."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(ADCH_A::_10010)
    }
    #[doc = "Exernal channel 19 is selected as input."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(ADCH_A::_10011)
    }
    #[doc = "Internal channel 0 is selected as input."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(ADCH_A::_10101)
    }
    #[doc = "Internal channel 1 is selected as input."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(ADCH_A::_10110)
    }
    #[doc = "Internal channel 2 is selected as input."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(ADCH_A::_10111)
    }
    #[doc = "Temp Sensor"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(ADCH_A::_11010)
    }
    #[doc = "Band Gap"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(ADCH_A::_11011)
    }
    #[doc = "Internal channel 3 is selected as input."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(ADCH_A::_11100)
    }
    #[doc = "VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(ADCH_A::_11101)
    }
    #[doc = "VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(ADCH_A::_11110)
    }
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(ADCH_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
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
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
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
    #[doc = "Bits 0:4 - Input channel select"]
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
#[doc = "ADC Status and Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](index.html) module"]
pub struct SC1_SPEC;
impl crate::RegisterSpec for SC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc1::R](R) reader structure"]
impl crate::Readable for SC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc1::W](W) writer structure"]
impl crate::Writable for SC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC1%s to value 0x1f"]
impl crate::Resettable for SC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
