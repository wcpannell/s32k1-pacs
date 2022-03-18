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
    #[doc = "0: External channel 0 is selected as input."]
    ADCH_0 = 0,
    #[doc = "1: External channel 1 is selected as input."]
    ADCH_1 = 1,
    #[doc = "2: External channel 2 is selected as input."]
    ADCH_2 = 2,
    #[doc = "3: External channel 3 is selected as input."]
    ADCH_3 = 3,
    #[doc = "4: External channel 4 is selected as input."]
    ADCH_4 = 4,
    #[doc = "5: External channel 5 is selected as input."]
    ADCH_5 = 5,
    #[doc = "6: External channel 6 is selected as input."]
    ADCH_6 = 6,
    #[doc = "7: External channel 7 is selected as input."]
    ADCH_7 = 7,
    #[doc = "8: External channel 8 is selected as input."]
    ADCH_8 = 8,
    #[doc = "9: External channel 9 is selected as input."]
    ADCH_9 = 9,
    #[doc = "10: External channel 10 is selected as input."]
    ADCH_10 = 10,
    #[doc = "11: External channel 11 is selected as input."]
    ADCH_11 = 11,
    #[doc = "12: External channel 12 is selected as input."]
    ADCH_12 = 12,
    #[doc = "13: External channel 13 is selected as input."]
    ADCH_13 = 13,
    #[doc = "14: External channel 14 is selected as input."]
    ADCH_14 = 14,
    #[doc = "15: External channel 15 is selected as input."]
    ADCH_15 = 15,
    #[doc = "18: External channel 18 is selected as input."]
    ADCH_18 = 18,
    #[doc = "19: External channel 19 is selected as input."]
    ADCH_19 = 19,
    #[doc = "21: Internal channel 0 is selected as input."]
    ADCH_21 = 21,
    #[doc = "22: Internal channel 1 is selected as input."]
    ADCH_22 = 22,
    #[doc = "23: Internal channel 2 is selected as input."]
    ADCH_23 = 23,
    #[doc = "27: Band Gap"]
    ADCH_27 = 27,
    #[doc = "28: Internal channel 3 is selected as input."]
    ADCH_28 = 28,
    #[doc = "29: VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    ADCH_29 = 29,
    #[doc = "30: VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    ADCH_30 = 30,
    #[doc = "31: Module is disabled"]
    ADCH_31 = 31,
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
            0 => Some(ADCH_A::ADCH_0),
            1 => Some(ADCH_A::ADCH_1),
            2 => Some(ADCH_A::ADCH_2),
            3 => Some(ADCH_A::ADCH_3),
            4 => Some(ADCH_A::ADCH_4),
            5 => Some(ADCH_A::ADCH_5),
            6 => Some(ADCH_A::ADCH_6),
            7 => Some(ADCH_A::ADCH_7),
            8 => Some(ADCH_A::ADCH_8),
            9 => Some(ADCH_A::ADCH_9),
            10 => Some(ADCH_A::ADCH_10),
            11 => Some(ADCH_A::ADCH_11),
            12 => Some(ADCH_A::ADCH_12),
            13 => Some(ADCH_A::ADCH_13),
            14 => Some(ADCH_A::ADCH_14),
            15 => Some(ADCH_A::ADCH_15),
            18 => Some(ADCH_A::ADCH_18),
            19 => Some(ADCH_A::ADCH_19),
            21 => Some(ADCH_A::ADCH_21),
            22 => Some(ADCH_A::ADCH_22),
            23 => Some(ADCH_A::ADCH_23),
            27 => Some(ADCH_A::ADCH_27),
            28 => Some(ADCH_A::ADCH_28),
            29 => Some(ADCH_A::ADCH_29),
            30 => Some(ADCH_A::ADCH_30),
            31 => Some(ADCH_A::ADCH_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_0`"]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        **self == ADCH_A::ADCH_0
    }
    #[doc = "Checks if the value of the field is `ADCH_1`"]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        **self == ADCH_A::ADCH_1
    }
    #[doc = "Checks if the value of the field is `ADCH_2`"]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        **self == ADCH_A::ADCH_2
    }
    #[doc = "Checks if the value of the field is `ADCH_3`"]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        **self == ADCH_A::ADCH_3
    }
    #[doc = "Checks if the value of the field is `ADCH_4`"]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        **self == ADCH_A::ADCH_4
    }
    #[doc = "Checks if the value of the field is `ADCH_5`"]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        **self == ADCH_A::ADCH_5
    }
    #[doc = "Checks if the value of the field is `ADCH_6`"]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        **self == ADCH_A::ADCH_6
    }
    #[doc = "Checks if the value of the field is `ADCH_7`"]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        **self == ADCH_A::ADCH_7
    }
    #[doc = "Checks if the value of the field is `ADCH_8`"]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        **self == ADCH_A::ADCH_8
    }
    #[doc = "Checks if the value of the field is `ADCH_9`"]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        **self == ADCH_A::ADCH_9
    }
    #[doc = "Checks if the value of the field is `ADCH_10`"]
    #[inline(always)]
    pub fn is_adch_10(&self) -> bool {
        **self == ADCH_A::ADCH_10
    }
    #[doc = "Checks if the value of the field is `ADCH_11`"]
    #[inline(always)]
    pub fn is_adch_11(&self) -> bool {
        **self == ADCH_A::ADCH_11
    }
    #[doc = "Checks if the value of the field is `ADCH_12`"]
    #[inline(always)]
    pub fn is_adch_12(&self) -> bool {
        **self == ADCH_A::ADCH_12
    }
    #[doc = "Checks if the value of the field is `ADCH_13`"]
    #[inline(always)]
    pub fn is_adch_13(&self) -> bool {
        **self == ADCH_A::ADCH_13
    }
    #[doc = "Checks if the value of the field is `ADCH_14`"]
    #[inline(always)]
    pub fn is_adch_14(&self) -> bool {
        **self == ADCH_A::ADCH_14
    }
    #[doc = "Checks if the value of the field is `ADCH_15`"]
    #[inline(always)]
    pub fn is_adch_15(&self) -> bool {
        **self == ADCH_A::ADCH_15
    }
    #[doc = "Checks if the value of the field is `ADCH_18`"]
    #[inline(always)]
    pub fn is_adch_18(&self) -> bool {
        **self == ADCH_A::ADCH_18
    }
    #[doc = "Checks if the value of the field is `ADCH_19`"]
    #[inline(always)]
    pub fn is_adch_19(&self) -> bool {
        **self == ADCH_A::ADCH_19
    }
    #[doc = "Checks if the value of the field is `ADCH_21`"]
    #[inline(always)]
    pub fn is_adch_21(&self) -> bool {
        **self == ADCH_A::ADCH_21
    }
    #[doc = "Checks if the value of the field is `ADCH_22`"]
    #[inline(always)]
    pub fn is_adch_22(&self) -> bool {
        **self == ADCH_A::ADCH_22
    }
    #[doc = "Checks if the value of the field is `ADCH_23`"]
    #[inline(always)]
    pub fn is_adch_23(&self) -> bool {
        **self == ADCH_A::ADCH_23
    }
    #[doc = "Checks if the value of the field is `ADCH_27`"]
    #[inline(always)]
    pub fn is_adch_27(&self) -> bool {
        **self == ADCH_A::ADCH_27
    }
    #[doc = "Checks if the value of the field is `ADCH_28`"]
    #[inline(always)]
    pub fn is_adch_28(&self) -> bool {
        **self == ADCH_A::ADCH_28
    }
    #[doc = "Checks if the value of the field is `ADCH_29`"]
    #[inline(always)]
    pub fn is_adch_29(&self) -> bool {
        **self == ADCH_A::ADCH_29
    }
    #[doc = "Checks if the value of the field is `ADCH_30`"]
    #[inline(always)]
    pub fn is_adch_30(&self) -> bool {
        **self == ADCH_A::ADCH_30
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        **self == ADCH_A::ADCH_31
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
    #[doc = "External channel 0 is selected as input."]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_0)
    }
    #[doc = "External channel 1 is selected as input."]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_1)
    }
    #[doc = "External channel 2 is selected as input."]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_2)
    }
    #[doc = "External channel 3 is selected as input."]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_3)
    }
    #[doc = "External channel 4 is selected as input."]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_4)
    }
    #[doc = "External channel 5 is selected as input."]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_5)
    }
    #[doc = "External channel 6 is selected as input."]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_6)
    }
    #[doc = "External channel 7 is selected as input."]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_7)
    }
    #[doc = "External channel 8 is selected as input."]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_8)
    }
    #[doc = "External channel 9 is selected as input."]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_9)
    }
    #[doc = "External channel 10 is selected as input."]
    #[inline(always)]
    pub fn adch_10(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_10)
    }
    #[doc = "External channel 11 is selected as input."]
    #[inline(always)]
    pub fn adch_11(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_11)
    }
    #[doc = "External channel 12 is selected as input."]
    #[inline(always)]
    pub fn adch_12(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_12)
    }
    #[doc = "External channel 13 is selected as input."]
    #[inline(always)]
    pub fn adch_13(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_13)
    }
    #[doc = "External channel 14 is selected as input."]
    #[inline(always)]
    pub fn adch_14(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_14)
    }
    #[doc = "External channel 15 is selected as input."]
    #[inline(always)]
    pub fn adch_15(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_15)
    }
    #[doc = "External channel 18 is selected as input."]
    #[inline(always)]
    pub fn adch_18(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_18)
    }
    #[doc = "External channel 19 is selected as input."]
    #[inline(always)]
    pub fn adch_19(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_19)
    }
    #[doc = "Internal channel 0 is selected as input."]
    #[inline(always)]
    pub fn adch_21(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_21)
    }
    #[doc = "Internal channel 1 is selected as input."]
    #[inline(always)]
    pub fn adch_22(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_22)
    }
    #[doc = "Internal channel 2 is selected as input."]
    #[inline(always)]
    pub fn adch_23(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_23)
    }
    #[doc = "Band Gap"]
    #[inline(always)]
    pub fn adch_27(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_27)
    }
    #[doc = "Internal channel 3 is selected as input."]
    #[inline(always)]
    pub fn adch_28(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_28)
    }
    #[doc = "VREFSH is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn adch_29(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_29)
    }
    #[doc = "VREFSL is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn adch_30(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_30)
    }
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_31)
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
    AIEN_0 = 0,
    #[doc = "1: Conversion complete interrupt is enabled."]
    AIEN_1 = 1,
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
            false => AIEN_A::AIEN_0,
            true => AIEN_A::AIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIEN_0`"]
    #[inline(always)]
    pub fn is_aien_0(&self) -> bool {
        **self == AIEN_A::AIEN_0
    }
    #[doc = "Checks if the value of the field is `AIEN_1`"]
    #[inline(always)]
    pub fn is_aien_1(&self) -> bool {
        **self == AIEN_A::AIEN_1
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
    pub fn aien_0(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn aien_1(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_1)
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
    #[doc = "0: Conversion is not complete."]
    COCO_0 = 0,
    #[doc = "1: Conversion is complete."]
    COCO_1 = 1,
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
            false => COCO_A::COCO_0,
            true => COCO_A::COCO_1,
        }
    }
    #[doc = "Checks if the value of the field is `COCO_0`"]
    #[inline(always)]
    pub fn is_coco_0(&self) -> bool {
        **self == COCO_A::COCO_0
    }
    #[doc = "Checks if the value of the field is `COCO_1`"]
    #[inline(always)]
    pub fn is_coco_1(&self) -> bool {
        **self == COCO_A::COCO_1
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
