#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACOn` reader - The result of the input comparison for channel n"]
pub struct ACON_R(crate::FieldReader<u8, u8>);
impl ACON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACOn` writer - The result of the input comparison for channel n"]
pub struct ACON_W<'a> {
    w: &'a mut W,
}
impl<'a> ACON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Comparator and DAC initialization delay modulus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INITMOD_A {
    #[doc = "0: The modulus is set to 64 (same with 111111)."]
    INITMOD_0 = 0,
}
impl From<INITMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: INITMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INITMOD` reader - Comparator and DAC initialization delay modulus."]
pub struct INITMOD_R(crate::FieldReader<u8, INITMOD_A>);
impl INITMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INITMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INITMOD_A> {
        match self.bits {
            0 => Some(INITMOD_A::INITMOD_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INITMOD_0`"]
    #[inline(always)]
    pub fn is_initmod_0(&self) -> bool {
        **self == INITMOD_A::INITMOD_0
    }
}
impl core::ops::Deref for INITMOD_R {
    type Target = crate::FieldReader<u8, INITMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITMOD` writer - Comparator and DAC initialization delay modulus."]
pub struct INITMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INITMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The modulus is set to 64 (same with 111111)."]
    #[inline(always)]
    pub fn initmod_0(self) -> &'a mut W {
        self.variant(INITMOD_A::INITMOD_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Number of sample clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSAM_A {
    #[doc = "0: The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    NSAM_0 = 0,
    #[doc = "1: The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    NSAM_1 = 1,
    #[doc = "2: The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_2 = 2,
    #[doc = "3: The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_3 = 3,
}
impl From<NSAM_A> for u8 {
    #[inline(always)]
    fn from(variant: NSAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NSAM` reader - Number of sample clocks"]
pub struct NSAM_R(crate::FieldReader<u8, NSAM_A>);
impl NSAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NSAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSAM_A {
        match self.bits {
            0 => NSAM_A::NSAM_0,
            1 => NSAM_A::NSAM_1,
            2 => NSAM_A::NSAM_2,
            3 => NSAM_A::NSAM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NSAM_0`"]
    #[inline(always)]
    pub fn is_nsam_0(&self) -> bool {
        **self == NSAM_A::NSAM_0
    }
    #[doc = "Checks if the value of the field is `NSAM_1`"]
    #[inline(always)]
    pub fn is_nsam_1(&self) -> bool {
        **self == NSAM_A::NSAM_1
    }
    #[doc = "Checks if the value of the field is `NSAM_2`"]
    #[inline(always)]
    pub fn is_nsam_2(&self) -> bool {
        **self == NSAM_A::NSAM_2
    }
    #[doc = "Checks if the value of the field is `NSAM_3`"]
    #[inline(always)]
    pub fn is_nsam_3(&self) -> bool {
        **self == NSAM_A::NSAM_3
    }
}
impl core::ops::Deref for NSAM_R {
    type Target = crate::FieldReader<u8, NSAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSAM` writer - Number of sample clocks"]
pub struct NSAM_W<'a> {
    w: &'a mut W,
}
impl<'a> NSAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSAM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline(always)]
    pub fn nsam_0(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_0)
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_1(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_1)
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_2(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_2)
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_3(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CH0F` reader - Channel 0 input changed flag"]
pub struct CH0F_R(crate::FieldReader<bool, bool>);
impl CH0F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0F` writer - Channel 0 input changed flag"]
pub struct CH0F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0F_W<'a> {
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
#[doc = "Field `CH1F` reader - Channel 1 input changed flag"]
pub struct CH1F_R(crate::FieldReader<bool, bool>);
impl CH1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1F` writer - Channel 1 input changed flag"]
pub struct CH1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1F_W<'a> {
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
#[doc = "Field `CH2F` reader - Channel 2 input changed flag"]
pub struct CH2F_R(crate::FieldReader<bool, bool>);
impl CH2F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2F` writer - Channel 2 input changed flag"]
pub struct CH2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH3F` reader - Channel 3 input changed flag"]
pub struct CH3F_R(crate::FieldReader<bool, bool>);
impl CH3F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3F` writer - Channel 3 input changed flag"]
pub struct CH3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CH4F` reader - Channel 4 input changed flag"]
pub struct CH4F_R(crate::FieldReader<bool, bool>);
impl CH4F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4F` writer - Channel 4 input changed flag"]
pub struct CH4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CH5F` reader - Channel 5 input changed flag"]
pub struct CH5F_R(crate::FieldReader<bool, bool>);
impl CH5F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5F` writer - Channel 5 input changed flag"]
pub struct CH5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CH6F` reader - Channel 6 input changed flag"]
pub struct CH6F_R(crate::FieldReader<bool, bool>);
impl CH6F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6F` writer - Channel 6 input changed flag"]
pub struct CH6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CH7F` reader - Channel 7 input changed flag"]
pub struct CH7F_R(crate::FieldReader<bool, bool>);
impl CH7F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7F` writer - Channel 7 input changed flag"]
pub struct CH7F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Fixed channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FXMXCH_A {
    #[doc = "0: Channel 0 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_0 = 0,
    #[doc = "1: Channel 1 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_1 = 1,
    #[doc = "2: Channel 2 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_2 = 2,
    #[doc = "3: Channel 3 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_3 = 3,
    #[doc = "4: Channel 4 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_4 = 4,
    #[doc = "5: Channel 5 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_5 = 5,
    #[doc = "6: Channel 6 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_6 = 6,
    #[doc = "7: Channel 7 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_7 = 7,
}
impl From<FXMXCH_A> for u8 {
    #[inline(always)]
    fn from(variant: FXMXCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FXMXCH` reader - Fixed channel selection"]
pub struct FXMXCH_R(crate::FieldReader<u8, FXMXCH_A>);
impl FXMXCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FXMXCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXMXCH_A {
        match self.bits {
            0 => FXMXCH_A::FXMXCH_0,
            1 => FXMXCH_A::FXMXCH_1,
            2 => FXMXCH_A::FXMXCH_2,
            3 => FXMXCH_A::FXMXCH_3,
            4 => FXMXCH_A::FXMXCH_4,
            5 => FXMXCH_A::FXMXCH_5,
            6 => FXMXCH_A::FXMXCH_6,
            7 => FXMXCH_A::FXMXCH_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FXMXCH_0`"]
    #[inline(always)]
    pub fn is_fxmxch_0(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_0
    }
    #[doc = "Checks if the value of the field is `FXMXCH_1`"]
    #[inline(always)]
    pub fn is_fxmxch_1(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_1
    }
    #[doc = "Checks if the value of the field is `FXMXCH_2`"]
    #[inline(always)]
    pub fn is_fxmxch_2(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_2
    }
    #[doc = "Checks if the value of the field is `FXMXCH_3`"]
    #[inline(always)]
    pub fn is_fxmxch_3(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_3
    }
    #[doc = "Checks if the value of the field is `FXMXCH_4`"]
    #[inline(always)]
    pub fn is_fxmxch_4(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_4
    }
    #[doc = "Checks if the value of the field is `FXMXCH_5`"]
    #[inline(always)]
    pub fn is_fxmxch_5(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_5
    }
    #[doc = "Checks if the value of the field is `FXMXCH_6`"]
    #[inline(always)]
    pub fn is_fxmxch_6(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_6
    }
    #[doc = "Checks if the value of the field is `FXMXCH_7`"]
    #[inline(always)]
    pub fn is_fxmxch_7(&self) -> bool {
        **self == FXMXCH_A::FXMXCH_7
    }
}
impl core::ops::Deref for FXMXCH_R {
    type Target = crate::FieldReader<u8, FXMXCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FXMXCH` writer - Fixed channel selection"]
pub struct FXMXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> FXMXCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXMXCH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_0(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_0)
    }
    #[doc = "Channel 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_1(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_1)
    }
    #[doc = "Channel 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_2(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_2)
    }
    #[doc = "Channel 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_3(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_3)
    }
    #[doc = "Channel 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_4(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_4)
    }
    #[doc = "Channel 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_5(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_5)
    }
    #[doc = "Channel 6 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_6(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_6)
    }
    #[doc = "Channel 7 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_7(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Fixed MUX Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXMP_A {
    #[doc = "0: The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    FXMP_0 = 0,
    #[doc = "1: The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    FXMP_1 = 1,
}
impl From<FXMP_A> for bool {
    #[inline(always)]
    fn from(variant: FXMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FXMP` reader - Fixed MUX Port"]
pub struct FXMP_R(crate::FieldReader<bool, FXMP_A>);
impl FXMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FXMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXMP_A {
        match self.bits {
            false => FXMP_A::FXMP_0,
            true => FXMP_A::FXMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FXMP_0`"]
    #[inline(always)]
    pub fn is_fxmp_0(&self) -> bool {
        **self == FXMP_A::FXMP_0
    }
    #[doc = "Checks if the value of the field is `FXMP_1`"]
    #[inline(always)]
    pub fn is_fxmp_1(&self) -> bool {
        **self == FXMP_A::FXMP_1
    }
}
impl core::ops::Deref for FXMP_R {
    type Target = crate::FieldReader<bool, FXMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FXMP` writer - Fixed MUX Port"]
pub struct FXMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FXMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_0(self) -> &'a mut W {
        self.variant(FXMP_A::FXMP_0)
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_1(self) -> &'a mut W {
        self.variant(FXMP_A::FXMP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Round-Robin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIE_A {
    #[doc = "0: The round-robin interrupt is disabled."]
    RRIE_0 = 0,
    #[doc = "1: The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    RRIE_1 = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIE` reader - Round-Robin interrupt enable"]
pub struct RRIE_R(crate::FieldReader<bool, RRIE_A>);
impl RRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::RRIE_0,
            true => RRIE_A::RRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRIE_0`"]
    #[inline(always)]
    pub fn is_rrie_0(&self) -> bool {
        **self == RRIE_A::RRIE_0
    }
    #[doc = "Checks if the value of the field is `RRIE_1`"]
    #[inline(always)]
    pub fn is_rrie_1(&self) -> bool {
        **self == RRIE_A::RRIE_1
    }
}
impl core::ops::Deref for RRIE_R {
    type Target = crate::FieldReader<bool, RRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIE` writer - Round-Robin interrupt enable"]
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The round-robin interrupt is disabled."]
    #[inline(always)]
    pub fn rrie_0(self) -> &'a mut W {
        self.variant(RRIE_A::RRIE_0)
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline(always)]
    pub fn rrie_1(self) -> &'a mut W {
        self.variant(RRIE_A::RRIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Round-Robin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRE_A {
    #[doc = "0: Round-robin operation is disabled."]
    RRE_0 = 0,
    #[doc = "1: Round-robin operation is enabled."]
    RRE_1 = 1,
}
impl From<RRE_A> for bool {
    #[inline(always)]
    fn from(variant: RRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRE` reader - Round-Robin Enable"]
pub struct RRE_R(crate::FieldReader<bool, RRE_A>);
impl RRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRE_A {
        match self.bits {
            false => RRE_A::RRE_0,
            true => RRE_A::RRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRE_0`"]
    #[inline(always)]
    pub fn is_rre_0(&self) -> bool {
        **self == RRE_A::RRE_0
    }
    #[doc = "Checks if the value of the field is `RRE_1`"]
    #[inline(always)]
    pub fn is_rre_1(&self) -> bool {
        **self == RRE_A::RRE_1
    }
}
impl core::ops::Deref for RRE_R {
    type Target = crate::FieldReader<bool, RRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRE` writer - Round-Robin Enable"]
pub struct RRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Round-robin operation is disabled."]
    #[inline(always)]
    pub fn rre_0(self) -> &'a mut W {
        self.variant(RRE_A::RRE_0)
    }
    #[doc = "Round-robin operation is enabled."]
    #[inline(always)]
    pub fn rre_1(self) -> &'a mut W {
        self.variant(RRE_A::RRE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline(always)]
    pub fn acon(&self) -> ACON_R {
        ACON_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&self) -> INITMOD_R {
        INITMOD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&self) -> NSAM_R {
        NSAM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline(always)]
    pub fn ch7f(&self) -> CH7F_R {
        CH7F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&self) -> FXMXCH_R {
        FXMXCH_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&self) -> FXMP_R {
        FXMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline(always)]
    pub fn rre(&self) -> RRE_R {
        RRE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline(always)]
    pub fn acon(&mut self) -> ACON_W {
        ACON_W { w: self }
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&mut self) -> INITMOD_W {
        INITMOD_W { w: self }
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&mut self) -> NSAM_W {
        NSAM_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline(always)]
    pub fn ch0f(&mut self) -> CH0F_W {
        CH0F_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline(always)]
    pub fn ch1f(&mut self) -> CH1F_W {
        CH1F_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline(always)]
    pub fn ch2f(&mut self) -> CH2F_W {
        CH2F_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline(always)]
    pub fn ch3f(&mut self) -> CH3F_W {
        CH3F_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline(always)]
    pub fn ch4f(&mut self) -> CH4F_W {
        CH4F_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline(always)]
    pub fn ch5f(&mut self) -> CH5F_W {
        CH5F_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline(always)]
    pub fn ch6f(&mut self) -> CH6F_W {
        CH6F_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline(always)]
    pub fn ch7f(&mut self) -> CH7F_W {
        CH7F_W { w: self }
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&mut self) -> FXMXCH_W {
        FXMXCH_W { w: self }
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&mut self) -> FXMP_W {
        FXMP_W { w: self }
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline(always)]
    pub fn rre(&mut self) -> RRE_W {
        RRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
