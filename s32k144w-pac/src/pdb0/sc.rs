#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOK` reader - Load OK"]
pub struct LDOK_R(crate::FieldReader<bool, bool>);
impl LDOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDOK` writer - Load OK"]
pub struct LDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Continuous Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: PDB operation in One-Shot mode"]
    CONT_0 = 0,
    #[doc = "1: PDB operation in Continuous mode"]
    CONT_1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous Mode Enable"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::CONT_0,
            true => CONT_A::CONT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONT_0`"]
    #[inline(always)]
    pub fn is_cont_0(&self) -> bool {
        **self == CONT_A::CONT_0
    }
    #[doc = "Checks if the value of the field is `CONT_1`"]
    #[inline(always)]
    pub fn is_cont_1(&self) -> bool {
        **self == CONT_A::CONT_1
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous Mode Enable"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB operation in One-Shot mode"]
    #[inline(always)]
    pub fn cont_0(self) -> &'a mut W {
        self.variant(CONT_A::CONT_0)
    }
    #[doc = "PDB operation in Continuous mode"]
    #[inline(always)]
    pub fn cont_1(self) -> &'a mut W {
        self.variant(CONT_A::CONT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Multiplication Factor Select for Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULT_A {
    #[doc = "0: Multiplication factor is 1."]
    MULT_0 = 0,
    #[doc = "1: Multiplication factor is 10."]
    MULT_1 = 1,
    #[doc = "2: Multiplication factor is 20."]
    MULT_2 = 2,
    #[doc = "3: Multiplication factor is 40."]
    MULT_3 = 3,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MULT` reader - Multiplication Factor Select for Prescaler"]
pub struct MULT_R(crate::FieldReader<u8, MULT_A>);
impl MULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULT_A {
        match self.bits {
            0 => MULT_A::MULT_0,
            1 => MULT_A::MULT_1,
            2 => MULT_A::MULT_2,
            3 => MULT_A::MULT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MULT_0`"]
    #[inline(always)]
    pub fn is_mult_0(&self) -> bool {
        **self == MULT_A::MULT_0
    }
    #[doc = "Checks if the value of the field is `MULT_1`"]
    #[inline(always)]
    pub fn is_mult_1(&self) -> bool {
        **self == MULT_A::MULT_1
    }
    #[doc = "Checks if the value of the field is `MULT_2`"]
    #[inline(always)]
    pub fn is_mult_2(&self) -> bool {
        **self == MULT_A::MULT_2
    }
    #[doc = "Checks if the value of the field is `MULT_3`"]
    #[inline(always)]
    pub fn is_mult_3(&self) -> bool {
        **self == MULT_A::MULT_3
    }
}
impl core::ops::Deref for MULT_R {
    type Target = crate::FieldReader<u8, MULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULT` writer - Multiplication Factor Select for Prescaler"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Multiplication factor is 1."]
    #[inline(always)]
    pub fn mult_0(self) -> &'a mut W {
        self.variant(MULT_A::MULT_0)
    }
    #[doc = "Multiplication factor is 10."]
    #[inline(always)]
    pub fn mult_1(self) -> &'a mut W {
        self.variant(MULT_A::MULT_1)
    }
    #[doc = "Multiplication factor is 20."]
    #[inline(always)]
    pub fn mult_2(self) -> &'a mut W {
        self.variant(MULT_A::MULT_2)
    }
    #[doc = "Multiplication factor is 40."]
    #[inline(always)]
    pub fn mult_3(self) -> &'a mut W {
        self.variant(MULT_A::MULT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "PDB Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBIE_A {
    #[doc = "0: PDB interrupt disabled."]
    PDBIE_0 = 0,
    #[doc = "1: PDB interrupt enabled."]
    PDBIE_1 = 1,
}
impl From<PDBIE_A> for bool {
    #[inline(always)]
    fn from(variant: PDBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBIE` reader - PDB Interrupt Enable"]
pub struct PDBIE_R(crate::FieldReader<bool, PDBIE_A>);
impl PDBIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDBIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBIE_A {
        match self.bits {
            false => PDBIE_A::PDBIE_0,
            true => PDBIE_A::PDBIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDBIE_0`"]
    #[inline(always)]
    pub fn is_pdbie_0(&self) -> bool {
        **self == PDBIE_A::PDBIE_0
    }
    #[doc = "Checks if the value of the field is `PDBIE_1`"]
    #[inline(always)]
    pub fn is_pdbie_1(&self) -> bool {
        **self == PDBIE_A::PDBIE_1
    }
}
impl core::ops::Deref for PDBIE_R {
    type Target = crate::FieldReader<bool, PDBIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDBIE` writer - PDB Interrupt Enable"]
pub struct PDBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDBIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB interrupt disabled."]
    #[inline(always)]
    pub fn pdbie_0(self) -> &'a mut W {
        self.variant(PDBIE_A::PDBIE_0)
    }
    #[doc = "PDB interrupt enabled."]
    #[inline(always)]
    pub fn pdbie_1(self) -> &'a mut W {
        self.variant(PDBIE_A::PDBIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PDBIF` reader - PDB Interrupt Flag"]
pub struct PDBIF_R(crate::FieldReader<bool, bool>);
impl PDBIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDBIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDBIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDBIF` writer - PDB Interrupt Flag"]
pub struct PDBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDBIF_W<'a> {
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
#[doc = "PDB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBEN_A {
    #[doc = "0: PDB disabled. Counter is off."]
    PDBEN_0 = 0,
    #[doc = "1: PDB enabled."]
    PDBEN_1 = 1,
}
impl From<PDBEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBEN` reader - PDB Enable"]
pub struct PDBEN_R(crate::FieldReader<bool, PDBEN_A>);
impl PDBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBEN_A {
        match self.bits {
            false => PDBEN_A::PDBEN_0,
            true => PDBEN_A::PDBEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDBEN_0`"]
    #[inline(always)]
    pub fn is_pdben_0(&self) -> bool {
        **self == PDBEN_A::PDBEN_0
    }
    #[doc = "Checks if the value of the field is `PDBEN_1`"]
    #[inline(always)]
    pub fn is_pdben_1(&self) -> bool {
        **self == PDBEN_A::PDBEN_1
    }
}
impl core::ops::Deref for PDBEN_R {
    type Target = crate::FieldReader<bool, PDBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDBEN` writer - PDB Enable"]
pub struct PDBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB disabled. Counter is off."]
    #[inline(always)]
    pub fn pdben_0(self) -> &'a mut W {
        self.variant(PDBEN_A::PDBEN_0)
    }
    #[doc = "PDB enabled."]
    #[inline(always)]
    pub fn pdben_1(self) -> &'a mut W {
        self.variant(PDBEN_A::PDBEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Trigger Input Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Trigger-In 0 is selected."]
    TRGSEL_0 = 0,
    #[doc = "1: Trigger-In 1 is selected."]
    TRGSEL_1 = 1,
    #[doc = "2: Trigger-In 2 is selected."]
    TRGSEL_2 = 2,
    #[doc = "3: Trigger-In 3 is selected."]
    TRGSEL_3 = 3,
    #[doc = "4: Trigger-In 4 is selected."]
    TRGSEL_4 = 4,
    #[doc = "5: Trigger-In 5 is selected."]
    TRGSEL_5 = 5,
    #[doc = "6: Trigger-In 6 is selected."]
    TRGSEL_6 = 6,
    #[doc = "7: Trigger-In 7 is selected."]
    TRGSEL_7 = 7,
    #[doc = "8: Trigger-In 8 is selected."]
    TRGSEL_8 = 8,
    #[doc = "9: Trigger-In 9 is selected."]
    TRGSEL_9 = 9,
    #[doc = "10: Trigger-In 10 is selected."]
    TRGSEL_10 = 10,
    #[doc = "11: Trigger-In 11 is selected."]
    TRGSEL_11 = 11,
    #[doc = "12: Trigger-In 12 is selected."]
    TRGSEL_12 = 12,
    #[doc = "13: Trigger-In 13 is selected."]
    TRGSEL_13 = 13,
    #[doc = "14: Trigger-In 14 is selected."]
    TRGSEL_14 = 14,
    #[doc = "15: Software trigger is selected."]
    TRGSEL_15 = 15,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Input Source Select"]
pub struct TRGSEL_R(crate::FieldReader<u8, TRGSEL_A>);
impl TRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::TRGSEL_0,
            1 => TRGSEL_A::TRGSEL_1,
            2 => TRGSEL_A::TRGSEL_2,
            3 => TRGSEL_A::TRGSEL_3,
            4 => TRGSEL_A::TRGSEL_4,
            5 => TRGSEL_A::TRGSEL_5,
            6 => TRGSEL_A::TRGSEL_6,
            7 => TRGSEL_A::TRGSEL_7,
            8 => TRGSEL_A::TRGSEL_8,
            9 => TRGSEL_A::TRGSEL_9,
            10 => TRGSEL_A::TRGSEL_10,
            11 => TRGSEL_A::TRGSEL_11,
            12 => TRGSEL_A::TRGSEL_12,
            13 => TRGSEL_A::TRGSEL_13,
            14 => TRGSEL_A::TRGSEL_14,
            15 => TRGSEL_A::TRGSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL_0`"]
    #[inline(always)]
    pub fn is_trgsel_0(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `TRGSEL_1`"]
    #[inline(always)]
    pub fn is_trgsel_1(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_1
    }
    #[doc = "Checks if the value of the field is `TRGSEL_2`"]
    #[inline(always)]
    pub fn is_trgsel_2(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_2
    }
    #[doc = "Checks if the value of the field is `TRGSEL_3`"]
    #[inline(always)]
    pub fn is_trgsel_3(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_3
    }
    #[doc = "Checks if the value of the field is `TRGSEL_4`"]
    #[inline(always)]
    pub fn is_trgsel_4(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_4
    }
    #[doc = "Checks if the value of the field is `TRGSEL_5`"]
    #[inline(always)]
    pub fn is_trgsel_5(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_5
    }
    #[doc = "Checks if the value of the field is `TRGSEL_6`"]
    #[inline(always)]
    pub fn is_trgsel_6(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_6
    }
    #[doc = "Checks if the value of the field is `TRGSEL_7`"]
    #[inline(always)]
    pub fn is_trgsel_7(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_7
    }
    #[doc = "Checks if the value of the field is `TRGSEL_8`"]
    #[inline(always)]
    pub fn is_trgsel_8(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_8
    }
    #[doc = "Checks if the value of the field is `TRGSEL_9`"]
    #[inline(always)]
    pub fn is_trgsel_9(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_9
    }
    #[doc = "Checks if the value of the field is `TRGSEL_10`"]
    #[inline(always)]
    pub fn is_trgsel_10(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_10
    }
    #[doc = "Checks if the value of the field is `TRGSEL_11`"]
    #[inline(always)]
    pub fn is_trgsel_11(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_11
    }
    #[doc = "Checks if the value of the field is `TRGSEL_12`"]
    #[inline(always)]
    pub fn is_trgsel_12(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_12
    }
    #[doc = "Checks if the value of the field is `TRGSEL_13`"]
    #[inline(always)]
    pub fn is_trgsel_13(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_13
    }
    #[doc = "Checks if the value of the field is `TRGSEL_14`"]
    #[inline(always)]
    pub fn is_trgsel_14(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_14
    }
    #[doc = "Checks if the value of the field is `TRGSEL_15`"]
    #[inline(always)]
    pub fn is_trgsel_15(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_15
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Input Source Select"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger-In 0 is selected."]
    #[inline(always)]
    pub fn trgsel_0(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_0)
    }
    #[doc = "Trigger-In 1 is selected."]
    #[inline(always)]
    pub fn trgsel_1(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_1)
    }
    #[doc = "Trigger-In 2 is selected."]
    #[inline(always)]
    pub fn trgsel_2(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_2)
    }
    #[doc = "Trigger-In 3 is selected."]
    #[inline(always)]
    pub fn trgsel_3(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_3)
    }
    #[doc = "Trigger-In 4 is selected."]
    #[inline(always)]
    pub fn trgsel_4(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_4)
    }
    #[doc = "Trigger-In 5 is selected."]
    #[inline(always)]
    pub fn trgsel_5(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_5)
    }
    #[doc = "Trigger-In 6 is selected."]
    #[inline(always)]
    pub fn trgsel_6(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_6)
    }
    #[doc = "Trigger-In 7 is selected."]
    #[inline(always)]
    pub fn trgsel_7(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_7)
    }
    #[doc = "Trigger-In 8 is selected."]
    #[inline(always)]
    pub fn trgsel_8(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_8)
    }
    #[doc = "Trigger-In 9 is selected."]
    #[inline(always)]
    pub fn trgsel_9(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_9)
    }
    #[doc = "Trigger-In 10 is selected."]
    #[inline(always)]
    pub fn trgsel_10(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_10)
    }
    #[doc = "Trigger-In 11 is selected."]
    #[inline(always)]
    pub fn trgsel_11(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_11)
    }
    #[doc = "Trigger-In 12 is selected."]
    #[inline(always)]
    pub fn trgsel_12(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_12)
    }
    #[doc = "Trigger-In 13 is selected."]
    #[inline(always)]
    pub fn trgsel_13(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_13)
    }
    #[doc = "Trigger-In 14 is selected."]
    #[inline(always)]
    pub fn trgsel_14(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_14)
    }
    #[doc = "Software trigger is selected."]
    #[inline(always)]
    pub fn trgsel_15(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Prescaler Divider Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Counting uses the peripheral clock divided by MULT (the multiplication factor)."]
    PRESCALER_0 = 0,
    #[doc = "1: Counting uses the peripheral clock divided by 2 x MULT (the multiplication factor)."]
    PRESCALER_1 = 1,
    #[doc = "2: Counting uses the peripheral clock divided by 4 x MULT (the multiplication factor)."]
    PRESCALER_2 = 2,
    #[doc = "3: Counting uses the peripheral clock divided by 8 x MULT (the multiplication factor)."]
    PRESCALER_3 = 3,
    #[doc = "4: Counting uses the peripheral clock divided by 16 x MULT (the multiplication factor)."]
    PRESCALER_4 = 4,
    #[doc = "5: Counting uses the peripheral clock divided by 32 x MULT (the multiplication factor)."]
    PRESCALER_5 = 5,
    #[doc = "6: Counting uses the peripheral clock divided by 64 x MULT (the multiplication factor)."]
    PRESCALER_6 = 6,
    #[doc = "7: Counting uses the peripheral clock divided by 128 x MULT (the multiplication factor)."]
    PRESCALER_7 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler Divider Select"]
pub struct PRESCALER_R(crate::FieldReader<u8, PRESCALER_A>);
impl PRESCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::PRESCALER_0,
            1 => PRESCALER_A::PRESCALER_1,
            2 => PRESCALER_A::PRESCALER_2,
            3 => PRESCALER_A::PRESCALER_3,
            4 => PRESCALER_A::PRESCALER_4,
            5 => PRESCALER_A::PRESCALER_5,
            6 => PRESCALER_A::PRESCALER_6,
            7 => PRESCALER_A::PRESCALER_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_0`"]
    #[inline(always)]
    pub fn is_prescaler_0(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_2`"]
    #[inline(always)]
    pub fn is_prescaler_2(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_2
    }
    #[doc = "Checks if the value of the field is `PRESCALER_3`"]
    #[inline(always)]
    pub fn is_prescaler_3(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_3
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4`"]
    #[inline(always)]
    pub fn is_prescaler_4(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_4
    }
    #[doc = "Checks if the value of the field is `PRESCALER_5`"]
    #[inline(always)]
    pub fn is_prescaler_5(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_5
    }
    #[doc = "Checks if the value of the field is `PRESCALER_6`"]
    #[inline(always)]
    pub fn is_prescaler_6(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_6
    }
    #[doc = "Checks if the value of the field is `PRESCALER_7`"]
    #[inline(always)]
    pub fn is_prescaler_7(&self) -> bool {
        **self == PRESCALER_A::PRESCALER_7
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u8, PRESCALER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Divider Select"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Counting uses the peripheral clock divided by MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_0(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_0)
    }
    #[doc = "Counting uses the peripheral clock divided by 2 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_1)
    }
    #[doc = "Counting uses the peripheral clock divided by 4 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_2(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_2)
    }
    #[doc = "Counting uses the peripheral clock divided by 8 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_3(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_3)
    }
    #[doc = "Counting uses the peripheral clock divided by 16 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_4(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_4)
    }
    #[doc = "Counting uses the peripheral clock divided by 32 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_5(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_5)
    }
    #[doc = "Counting uses the peripheral clock divided by 64 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_6(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_6)
    }
    #[doc = "Counting uses the peripheral clock divided by 128 x MULT (the multiplication factor)."]
    #[inline(always)]
    pub fn prescaler_7(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled."]
    DMAEN_0 = 0,
    #[doc = "1: DMA enabled."]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        **self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        **self == DMAEN_A::DMAEN_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SWTRIG` reader - Software Trigger"]
pub struct SWTRIG_R(crate::FieldReader<bool, bool>);
impl SWTRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
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
#[doc = "PDB Sequence Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBEIE_A {
    #[doc = "0: PDB sequence error interrupt disabled."]
    PDBEIE_0 = 0,
    #[doc = "1: PDB sequence error interrupt enabled."]
    PDBEIE_1 = 1,
}
impl From<PDBEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PDBEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBEIE` reader - PDB Sequence Error Interrupt Enable"]
pub struct PDBEIE_R(crate::FieldReader<bool, PDBEIE_A>);
impl PDBEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDBEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBEIE_A {
        match self.bits {
            false => PDBEIE_A::PDBEIE_0,
            true => PDBEIE_A::PDBEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDBEIE_0`"]
    #[inline(always)]
    pub fn is_pdbeie_0(&self) -> bool {
        **self == PDBEIE_A::PDBEIE_0
    }
    #[doc = "Checks if the value of the field is `PDBEIE_1`"]
    #[inline(always)]
    pub fn is_pdbeie_1(&self) -> bool {
        **self == PDBEIE_A::PDBEIE_1
    }
}
impl core::ops::Deref for PDBEIE_R {
    type Target = crate::FieldReader<bool, PDBEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDBEIE` writer - PDB Sequence Error Interrupt Enable"]
pub struct PDBEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDBEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDBEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB sequence error interrupt disabled."]
    #[inline(always)]
    pub fn pdbeie_0(self) -> &'a mut W {
        self.variant(PDBEIE_A::PDBEIE_0)
    }
    #[doc = "PDB sequence error interrupt enabled."]
    #[inline(always)]
    pub fn pdbeie_1(self) -> &'a mut W {
        self.variant(PDBEIE_A::PDBEIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Load Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDMOD_A {
    #[doc = "0: The internal registers are loaded with the values from their buffers, immediately after 1 is written to LDOK."]
    LDMOD_0 = 0,
    #[doc = "1: The internal registers are loaded with the values from their buffers when the PDB counter (CNT) = MOD + 1 CNT delay elapsed, after 1 is written to LDOK."]
    LDMOD_1 = 1,
    #[doc = "2: The internal registers are loaded with the values from their buffers when a trigger input event is detected, after 1 is written to LDOK."]
    LDMOD_2 = 2,
    #[doc = "3: The internal registers are loaded with the values from their buffers when either the PDB counter (CNT) = MOD + 1 CNT delay elapsed, or a trigger input event is detected, after 1 is written to LDOK."]
    LDMOD_3 = 3,
}
impl From<LDMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: LDMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LDMOD` reader - Load Mode Select"]
pub struct LDMOD_R(crate::FieldReader<u8, LDMOD_A>);
impl LDMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMOD_A {
        match self.bits {
            0 => LDMOD_A::LDMOD_0,
            1 => LDMOD_A::LDMOD_1,
            2 => LDMOD_A::LDMOD_2,
            3 => LDMOD_A::LDMOD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LDMOD_0`"]
    #[inline(always)]
    pub fn is_ldmod_0(&self) -> bool {
        **self == LDMOD_A::LDMOD_0
    }
    #[doc = "Checks if the value of the field is `LDMOD_1`"]
    #[inline(always)]
    pub fn is_ldmod_1(&self) -> bool {
        **self == LDMOD_A::LDMOD_1
    }
    #[doc = "Checks if the value of the field is `LDMOD_2`"]
    #[inline(always)]
    pub fn is_ldmod_2(&self) -> bool {
        **self == LDMOD_A::LDMOD_2
    }
    #[doc = "Checks if the value of the field is `LDMOD_3`"]
    #[inline(always)]
    pub fn is_ldmod_3(&self) -> bool {
        **self == LDMOD_A::LDMOD_3
    }
}
impl core::ops::Deref for LDMOD_R {
    type Target = crate::FieldReader<u8, LDMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDMOD` writer - Load Mode Select"]
pub struct LDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The internal registers are loaded with the values from their buffers, immediately after 1 is written to LDOK."]
    #[inline(always)]
    pub fn ldmod_0(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_0)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter (CNT) = MOD + 1 CNT delay elapsed, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn ldmod_1(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_1)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn ldmod_2(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_2)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter (CNT) = MOD + 1 CNT delay elapsed, or a trigger input event is detected, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn ldmod_3(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline(always)]
    pub fn pdbie(&self) -> PDBIE_R {
        PDBIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    pub fn pdbif(&self) -> PDBIF_R {
        PDBIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    pub fn pdben(&self) -> PDBEN_R {
        PDBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    pub fn pdbeie(&self) -> PDBEIE_R {
        PDBEIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&self) -> LDMOD_R {
        LDMOD_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    pub fn ldok(&mut self) -> LDOK_W {
        LDOK_W { w: self }
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline(always)]
    pub fn pdbie(&mut self) -> PDBIE_W {
        PDBIE_W { w: self }
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    pub fn pdbif(&mut self) -> PDBIF_W {
        PDBIF_W { w: self }
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    pub fn pdben(&mut self) -> PDBEN_W {
        PDBEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    pub fn pdbeie(&mut self) -> PDBEIE_W {
        PDBEIE_W { w: self }
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&mut self) -> LDMOD_W {
        LDMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
