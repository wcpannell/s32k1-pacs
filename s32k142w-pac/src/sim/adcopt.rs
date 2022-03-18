#[doc = "Register `ADCOPT` reader"]
pub struct R(crate::R<ADCOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCOPT` writer"]
pub struct W(crate::W<ADCOPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCOPT_SPEC>;
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
impl From<crate::W<ADCOPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCOPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC0 trigger source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0TRGSEL_A {
    #[doc = "0: PDB output"]
    ADC0TRGSEL_0 = 0,
    #[doc = "1: TRGMUX output"]
    ADC0TRGSEL_1 = 1,
}
impl From<ADC0TRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0TRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0TRGSEL` reader - ADC0 trigger source select"]
pub struct ADC0TRGSEL_R(crate::FieldReader<bool, ADC0TRGSEL_A>);
impl ADC0TRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0TRGSEL_A {
        match self.bits {
            false => ADC0TRGSEL_A::ADC0TRGSEL_0,
            true => ADC0TRGSEL_A::ADC0TRGSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0TRGSEL_0`"]
    #[inline(always)]
    pub fn is_adc0trgsel_0(&self) -> bool {
        **self == ADC0TRGSEL_A::ADC0TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC0TRGSEL_1`"]
    #[inline(always)]
    pub fn is_adc0trgsel_1(&self) -> bool {
        **self == ADC0TRGSEL_A::ADC0TRGSEL_1
    }
}
impl core::ops::Deref for ADC0TRGSEL_R {
    type Target = crate::FieldReader<bool, ADC0TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0TRGSEL` writer - ADC0 trigger source select"]
pub struct ADC0TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0TRGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB output"]
    #[inline(always)]
    pub fn adc0trgsel_0(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::ADC0TRGSEL_0)
    }
    #[doc = "TRGMUX output"]
    #[inline(always)]
    pub fn adc0trgsel_1(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::ADC0TRGSEL_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "ADC0 software pretrigger sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC0SWPRETRG_A {
    #[doc = "0: Software pretrigger disabled"]
    ADC0SWPRETRG_0 = 0,
    #[doc = "4: Software pretrigger 0"]
    ADC0SWPRETRG_4 = 4,
    #[doc = "5: Software pretrigger 1"]
    ADC0SWPRETRG_5 = 5,
    #[doc = "6: Software pretrigger 2"]
    ADC0SWPRETRG_6 = 6,
    #[doc = "7: Software pretrigger 3"]
    ADC0SWPRETRG_7 = 7,
}
impl From<ADC0SWPRETRG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0SWPRETRG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC0SWPRETRG` reader - ADC0 software pretrigger sources"]
pub struct ADC0SWPRETRG_R(crate::FieldReader<u8, ADC0SWPRETRG_A>);
impl ADC0SWPRETRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC0SWPRETRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0SWPRETRG_A> {
        match self.bits {
            0 => Some(ADC0SWPRETRG_A::ADC0SWPRETRG_0),
            4 => Some(ADC0SWPRETRG_A::ADC0SWPRETRG_4),
            5 => Some(ADC0SWPRETRG_A::ADC0SWPRETRG_5),
            6 => Some(ADC0SWPRETRG_A::ADC0SWPRETRG_6),
            7 => Some(ADC0SWPRETRG_A::ADC0SWPRETRG_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0SWPRETRG_0`"]
    #[inline(always)]
    pub fn is_adc0swpretrg_0(&self) -> bool {
        **self == ADC0SWPRETRG_A::ADC0SWPRETRG_0
    }
    #[doc = "Checks if the value of the field is `ADC0SWPRETRG_4`"]
    #[inline(always)]
    pub fn is_adc0swpretrg_4(&self) -> bool {
        **self == ADC0SWPRETRG_A::ADC0SWPRETRG_4
    }
    #[doc = "Checks if the value of the field is `ADC0SWPRETRG_5`"]
    #[inline(always)]
    pub fn is_adc0swpretrg_5(&self) -> bool {
        **self == ADC0SWPRETRG_A::ADC0SWPRETRG_5
    }
    #[doc = "Checks if the value of the field is `ADC0SWPRETRG_6`"]
    #[inline(always)]
    pub fn is_adc0swpretrg_6(&self) -> bool {
        **self == ADC0SWPRETRG_A::ADC0SWPRETRG_6
    }
    #[doc = "Checks if the value of the field is `ADC0SWPRETRG_7`"]
    #[inline(always)]
    pub fn is_adc0swpretrg_7(&self) -> bool {
        **self == ADC0SWPRETRG_A::ADC0SWPRETRG_7
    }
}
impl core::ops::Deref for ADC0SWPRETRG_R {
    type Target = crate::FieldReader<u8, ADC0SWPRETRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0SWPRETRG` writer - ADC0 software pretrigger sources"]
pub struct ADC0SWPRETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0SWPRETRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0SWPRETRG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software pretrigger disabled"]
    #[inline(always)]
    pub fn adc0swpretrg_0(self) -> &'a mut W {
        self.variant(ADC0SWPRETRG_A::ADC0SWPRETRG_0)
    }
    #[doc = "Software pretrigger 0"]
    #[inline(always)]
    pub fn adc0swpretrg_4(self) -> &'a mut W {
        self.variant(ADC0SWPRETRG_A::ADC0SWPRETRG_4)
    }
    #[doc = "Software pretrigger 1"]
    #[inline(always)]
    pub fn adc0swpretrg_5(self) -> &'a mut W {
        self.variant(ADC0SWPRETRG_A::ADC0SWPRETRG_5)
    }
    #[doc = "Software pretrigger 2"]
    #[inline(always)]
    pub fn adc0swpretrg_6(self) -> &'a mut W {
        self.variant(ADC0SWPRETRG_A::ADC0SWPRETRG_6)
    }
    #[doc = "Software pretrigger 3"]
    #[inline(always)]
    pub fn adc0swpretrg_7(self) -> &'a mut W {
        self.variant(ADC0SWPRETRG_A::ADC0SWPRETRG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "ADC0 pretrigger source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC0PRETRGSEL_A {
    #[doc = "0: PDB pretrigger (default)"]
    ADC0PRETRGSEL_0 = 0,
    #[doc = "1: TRGMUX pretrigger"]
    ADC0PRETRGSEL_1 = 1,
    #[doc = "2: Software pretrigger"]
    ADC0PRETRGSEL_2 = 2,
}
impl From<ADC0PRETRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0PRETRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC0PRETRGSEL` reader - ADC0 pretrigger source select"]
pub struct ADC0PRETRGSEL_R(crate::FieldReader<u8, ADC0PRETRGSEL_A>);
impl ADC0PRETRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC0PRETRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0PRETRGSEL_A> {
        match self.bits {
            0 => Some(ADC0PRETRGSEL_A::ADC0PRETRGSEL_0),
            1 => Some(ADC0PRETRGSEL_A::ADC0PRETRGSEL_1),
            2 => Some(ADC0PRETRGSEL_A::ADC0PRETRGSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0PRETRGSEL_0`"]
    #[inline(always)]
    pub fn is_adc0pretrgsel_0(&self) -> bool {
        **self == ADC0PRETRGSEL_A::ADC0PRETRGSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC0PRETRGSEL_1`"]
    #[inline(always)]
    pub fn is_adc0pretrgsel_1(&self) -> bool {
        **self == ADC0PRETRGSEL_A::ADC0PRETRGSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC0PRETRGSEL_2`"]
    #[inline(always)]
    pub fn is_adc0pretrgsel_2(&self) -> bool {
        **self == ADC0PRETRGSEL_A::ADC0PRETRGSEL_2
    }
}
impl core::ops::Deref for ADC0PRETRGSEL_R {
    type Target = crate::FieldReader<u8, ADC0PRETRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0PRETRGSEL` writer - ADC0 pretrigger source select"]
pub struct ADC0PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0PRETRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB pretrigger (default)"]
    #[inline(always)]
    pub fn adc0pretrgsel_0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::ADC0PRETRGSEL_0)
    }
    #[doc = "TRGMUX pretrigger"]
    #[inline(always)]
    pub fn adc0pretrgsel_1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::ADC0PRETRGSEL_1)
    }
    #[doc = "Software pretrigger"]
    #[inline(always)]
    pub fn adc0pretrgsel_2(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::ADC0PRETRGSEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "ADC1 trigger source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1TRGSEL_A {
    #[doc = "0: PDB output"]
    ADC1TRGSEL_0 = 0,
    #[doc = "1: TRGMUX output"]
    ADC1TRGSEL_1 = 1,
}
impl From<ADC1TRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1TRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1TRGSEL` reader - ADC1 trigger source select"]
pub struct ADC1TRGSEL_R(crate::FieldReader<bool, ADC1TRGSEL_A>);
impl ADC1TRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1TRGSEL_A {
        match self.bits {
            false => ADC1TRGSEL_A::ADC1TRGSEL_0,
            true => ADC1TRGSEL_A::ADC1TRGSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC1TRGSEL_0`"]
    #[inline(always)]
    pub fn is_adc1trgsel_0(&self) -> bool {
        **self == ADC1TRGSEL_A::ADC1TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC1TRGSEL_1`"]
    #[inline(always)]
    pub fn is_adc1trgsel_1(&self) -> bool {
        **self == ADC1TRGSEL_A::ADC1TRGSEL_1
    }
}
impl core::ops::Deref for ADC1TRGSEL_R {
    type Target = crate::FieldReader<bool, ADC1TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1TRGSEL` writer - ADC1 trigger source select"]
pub struct ADC1TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1TRGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB output"]
    #[inline(always)]
    pub fn adc1trgsel_0(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::ADC1TRGSEL_0)
    }
    #[doc = "TRGMUX output"]
    #[inline(always)]
    pub fn adc1trgsel_1(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::ADC1TRGSEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "ADC1 software pretrigger sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC1SWPRETRG_A {
    #[doc = "0: Software pretrigger disabled"]
    ADC1SWPRETRG_0 = 0,
    #[doc = "4: Software pretrigger 0"]
    ADC1SWPRETRG_4 = 4,
    #[doc = "5: Software pretrigger 1"]
    ADC1SWPRETRG_5 = 5,
    #[doc = "6: Software pretrigger 2"]
    ADC1SWPRETRG_6 = 6,
    #[doc = "7: Software pretrigger 3"]
    ADC1SWPRETRG_7 = 7,
}
impl From<ADC1SWPRETRG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1SWPRETRG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC1SWPRETRG` reader - ADC1 software pretrigger sources"]
pub struct ADC1SWPRETRG_R(crate::FieldReader<u8, ADC1SWPRETRG_A>);
impl ADC1SWPRETRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC1SWPRETRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC1SWPRETRG_A> {
        match self.bits {
            0 => Some(ADC1SWPRETRG_A::ADC1SWPRETRG_0),
            4 => Some(ADC1SWPRETRG_A::ADC1SWPRETRG_4),
            5 => Some(ADC1SWPRETRG_A::ADC1SWPRETRG_5),
            6 => Some(ADC1SWPRETRG_A::ADC1SWPRETRG_6),
            7 => Some(ADC1SWPRETRG_A::ADC1SWPRETRG_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC1SWPRETRG_0`"]
    #[inline(always)]
    pub fn is_adc1swpretrg_0(&self) -> bool {
        **self == ADC1SWPRETRG_A::ADC1SWPRETRG_0
    }
    #[doc = "Checks if the value of the field is `ADC1SWPRETRG_4`"]
    #[inline(always)]
    pub fn is_adc1swpretrg_4(&self) -> bool {
        **self == ADC1SWPRETRG_A::ADC1SWPRETRG_4
    }
    #[doc = "Checks if the value of the field is `ADC1SWPRETRG_5`"]
    #[inline(always)]
    pub fn is_adc1swpretrg_5(&self) -> bool {
        **self == ADC1SWPRETRG_A::ADC1SWPRETRG_5
    }
    #[doc = "Checks if the value of the field is `ADC1SWPRETRG_6`"]
    #[inline(always)]
    pub fn is_adc1swpretrg_6(&self) -> bool {
        **self == ADC1SWPRETRG_A::ADC1SWPRETRG_6
    }
    #[doc = "Checks if the value of the field is `ADC1SWPRETRG_7`"]
    #[inline(always)]
    pub fn is_adc1swpretrg_7(&self) -> bool {
        **self == ADC1SWPRETRG_A::ADC1SWPRETRG_7
    }
}
impl core::ops::Deref for ADC1SWPRETRG_R {
    type Target = crate::FieldReader<u8, ADC1SWPRETRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1SWPRETRG` writer - ADC1 software pretrigger sources"]
pub struct ADC1SWPRETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1SWPRETRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1SWPRETRG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software pretrigger disabled"]
    #[inline(always)]
    pub fn adc1swpretrg_0(self) -> &'a mut W {
        self.variant(ADC1SWPRETRG_A::ADC1SWPRETRG_0)
    }
    #[doc = "Software pretrigger 0"]
    #[inline(always)]
    pub fn adc1swpretrg_4(self) -> &'a mut W {
        self.variant(ADC1SWPRETRG_A::ADC1SWPRETRG_4)
    }
    #[doc = "Software pretrigger 1"]
    #[inline(always)]
    pub fn adc1swpretrg_5(self) -> &'a mut W {
        self.variant(ADC1SWPRETRG_A::ADC1SWPRETRG_5)
    }
    #[doc = "Software pretrigger 2"]
    #[inline(always)]
    pub fn adc1swpretrg_6(self) -> &'a mut W {
        self.variant(ADC1SWPRETRG_A::ADC1SWPRETRG_6)
    }
    #[doc = "Software pretrigger 3"]
    #[inline(always)]
    pub fn adc1swpretrg_7(self) -> &'a mut W {
        self.variant(ADC1SWPRETRG_A::ADC1SWPRETRG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "ADC1 pretrigger source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC1PRETRGSEL_A {
    #[doc = "0: PDB pretrigger (default)"]
    ADC1PRETRGSEL_0 = 0,
    #[doc = "1: TRGMUX pretrigger"]
    ADC1PRETRGSEL_1 = 1,
    #[doc = "2: Software pretrigger"]
    ADC1PRETRGSEL_2 = 2,
}
impl From<ADC1PRETRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1PRETRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC1PRETRGSEL` reader - ADC1 pretrigger source select"]
pub struct ADC1PRETRGSEL_R(crate::FieldReader<u8, ADC1PRETRGSEL_A>);
impl ADC1PRETRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC1PRETRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC1PRETRGSEL_A> {
        match self.bits {
            0 => Some(ADC1PRETRGSEL_A::ADC1PRETRGSEL_0),
            1 => Some(ADC1PRETRGSEL_A::ADC1PRETRGSEL_1),
            2 => Some(ADC1PRETRGSEL_A::ADC1PRETRGSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC1PRETRGSEL_0`"]
    #[inline(always)]
    pub fn is_adc1pretrgsel_0(&self) -> bool {
        **self == ADC1PRETRGSEL_A::ADC1PRETRGSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC1PRETRGSEL_1`"]
    #[inline(always)]
    pub fn is_adc1pretrgsel_1(&self) -> bool {
        **self == ADC1PRETRGSEL_A::ADC1PRETRGSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC1PRETRGSEL_2`"]
    #[inline(always)]
    pub fn is_adc1pretrgsel_2(&self) -> bool {
        **self == ADC1PRETRGSEL_A::ADC1PRETRGSEL_2
    }
}
impl core::ops::Deref for ADC1PRETRGSEL_R {
    type Target = crate::FieldReader<u8, ADC1PRETRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1PRETRGSEL` writer - ADC1 pretrigger source select"]
pub struct ADC1PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1PRETRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB pretrigger (default)"]
    #[inline(always)]
    pub fn adc1pretrgsel_0(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::ADC1PRETRGSEL_0)
    }
    #[doc = "TRGMUX pretrigger"]
    #[inline(always)]
    pub fn adc1pretrgsel_1(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::ADC1PRETRGSEL_1)
    }
    #[doc = "Software pretrigger"]
    #[inline(always)]
    pub fn adc1pretrgsel_2(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::ADC1PRETRGSEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 trigger source select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> ADC0TRGSEL_R {
        ADC0TRGSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - ADC0 software pretrigger sources"]
    #[inline(always)]
    pub fn adc0swpretrg(&self) -> ADC0SWPRETRG_R {
        ADC0SWPRETRG_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - ADC0 pretrigger source select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSEL_R {
        ADC0PRETRGSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - ADC1 trigger source select"]
    #[inline(always)]
    pub fn adc1trgsel(&self) -> ADC1TRGSEL_R {
        ADC1TRGSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - ADC1 software pretrigger sources"]
    #[inline(always)]
    pub fn adc1swpretrg(&self) -> ADC1SWPRETRG_R {
        ADC1SWPRETRG_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - ADC1 pretrigger source select"]
    #[inline(always)]
    pub fn adc1pretrgsel(&self) -> ADC1PRETRGSEL_R {
        ADC1PRETRGSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 trigger source select"]
    #[inline(always)]
    pub fn adc0trgsel(&mut self) -> ADC0TRGSEL_W {
        ADC0TRGSEL_W { w: self }
    }
    #[doc = "Bits 1:3 - ADC0 software pretrigger sources"]
    #[inline(always)]
    pub fn adc0swpretrg(&mut self) -> ADC0SWPRETRG_W {
        ADC0SWPRETRG_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC0 pretrigger source select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&mut self) -> ADC0PRETRGSEL_W {
        ADC0PRETRGSEL_W { w: self }
    }
    #[doc = "Bit 8 - ADC1 trigger source select"]
    #[inline(always)]
    pub fn adc1trgsel(&mut self) -> ADC1TRGSEL_W {
        ADC1TRGSEL_W { w: self }
    }
    #[doc = "Bits 9:11 - ADC1 software pretrigger sources"]
    #[inline(always)]
    pub fn adc1swpretrg(&mut self) -> ADC1SWPRETRG_W {
        ADC1SWPRETRG_W { w: self }
    }
    #[doc = "Bits 12:13 - ADC1 pretrigger source select"]
    #[inline(always)]
    pub fn adc1pretrgsel(&mut self) -> ADC1PRETRGSEL_W {
        ADC1PRETRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Options Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcopt](index.html) module"]
pub struct ADCOPT_SPEC;
impl crate::RegisterSpec for ADCOPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcopt::R](R) reader structure"]
impl crate::Readable for ADCOPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcopt::W](W) writer structure"]
impl crate::Writable for ADCOPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCOPT to value 0"]
impl crate::Resettable for ADCOPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
