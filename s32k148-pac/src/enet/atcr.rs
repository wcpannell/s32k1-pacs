#[doc = "Register `ATCR` reader"]
pub struct R(crate::R<ATCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR` writer"]
pub struct W(crate::W<ATCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR_SPEC>;
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
impl From<crate::W<ATCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: The timer stops at the current value."]
    _0 = 0,
    #[doc = "1: The timer starts incrementing."]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable Timer"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable Timer"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The timer stops at the current value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "The timer starts incrementing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
#[doc = "Enable One-Shot Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFEN_A {
    #[doc = "0: Disable."]
    _0 = 0,
    #[doc = "1: The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    _1 = 1,
}
impl From<OFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFEN` reader - Enable One-Shot Offset Event"]
pub struct OFFEN_R(crate::FieldReader<bool, OFFEN_A>);
impl OFFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFEN_A {
        match self.bits {
            false => OFFEN_A::_0,
            true => OFFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OFFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OFFEN_A::_1
    }
}
impl core::ops::Deref for OFFEN_R {
    type Target = crate::FieldReader<bool, OFFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFEN` writer - Enable One-Shot Offset Event"]
pub struct OFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFFEN_A::_0)
    }
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Reset Timer On Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRST_A {
    #[doc = "0: The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    _0 = 0,
    #[doc = "1: If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    _1 = 1,
}
impl From<OFFRST_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRST` reader - Reset Timer On Offset Event"]
pub struct OFFRST_R(crate::FieldReader<bool, OFFRST_A>);
impl OFFRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRST_A {
        match self.bits {
            false => OFFRST_A::_0,
            true => OFFRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OFFRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OFFRST_A::_1
    }
}
impl core::ops::Deref for OFFRST_R {
    type Target = crate::FieldReader<bool, OFFRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFRST` writer - Reset Timer On Offset Event"]
pub struct OFFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFFRST_A::_0)
    }
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFFRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable Periodical Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEREN_A {
    #[doc = "0: Disable."]
    _0 = 0,
    #[doc = "1: A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    _1 = 1,
}
impl From<PEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PEREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEREN` reader - Enable Periodical Event"]
pub struct PEREN_R(crate::FieldReader<bool, PEREN_A>);
impl PEREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEREN_A {
        match self.bits {
            false => PEREN_A::_0,
            true => PEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PEREN_A::_1
    }
}
impl core::ops::Deref for PEREN_R {
    type Target = crate::FieldReader<bool, PEREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEREN` writer - Enable Periodical Event"]
pub struct PEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEREN_A::_0)
    }
    #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Enables event signal output assertion on period event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPER_A {
    #[doc = "0: Disable."]
    _0 = 0,
    #[doc = "1: Enable."]
    _1 = 1,
}
impl From<PINPER_A> for bool {
    #[inline(always)]
    fn from(variant: PINPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINPER` reader - Enables event signal output assertion on period event"]
pub struct PINPER_R(crate::FieldReader<bool, PINPER_A>);
impl PINPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPER_A {
        match self.bits {
            false => PINPER_A::_0,
            true => PINPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINPER_A::_1
    }
}
impl core::ops::Deref for PINPER_R {
    type Target = crate::FieldReader<bool, PINPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINPER` writer - Enables event signal output assertion on period event"]
pub struct PINPER_W<'a> {
    w: &'a mut W,
}
impl<'a> PINPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINPER_A::_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINPER_A::_1)
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
#[doc = "Field `RESTART` reader - Reset Timer"]
pub struct RESTART_R(crate::FieldReader<bool, bool>);
impl RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESTART` writer - Reset Timer"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Capture Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: The current time is captured and can be read from the ATVR register."]
    _1 = 1,
}
impl From<CAPTURE_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE` reader - Capture Timer Value"]
pub struct CAPTURE_R(crate::FieldReader<bool, CAPTURE_A>);
impl CAPTURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A {
        match self.bits {
            false => CAPTURE_A::_0,
            true => CAPTURE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPTURE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPTURE_A::_1
    }
}
impl core::ops::Deref for CAPTURE_R {
    type Target = crate::FieldReader<bool, CAPTURE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE` writer - Capture Timer Value"]
pub struct CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPTURE_A::_0)
    }
    #[doc = "The current time is captured and can be read from the ATVR register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPTURE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable Timer Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_A {
    #[doc = "0: The timer is active and all configuration fields in this register are relevant."]
    _0 = 0,
    #[doc = "1: The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    _1 = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE` reader - Enable Timer Slave Mode"]
pub struct SLAVE_R(crate::FieldReader<bool, SLAVE_A>);
impl SLAVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::_0,
            true => SLAVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLAVE_A::_1
    }
}
impl core::ops::Deref for SLAVE_R {
    type Target = crate::FieldReader<bool, SLAVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE` writer - Enable Timer Slave Mode"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLAVE_A::_0)
    }
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLAVE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    pub fn offen(&self) -> OFFEN_R {
        OFFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    pub fn offrst(&self) -> OFFRST_R {
        OFFRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline(always)]
    pub fn pinper(&self) -> PINPER_R {
        PINPER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    pub fn offen(&mut self) -> OFFEN_W {
        OFFEN_W { w: self }
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    pub fn offrst(&mut self) -> OFFRST_W {
        OFFRST_W { w: self }
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    pub fn peren(&mut self) -> PEREN_W {
        PEREN_W { w: self }
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline(always)]
    pub fn pinper(&mut self) -> PINPER_W {
        PINPER_W { w: self }
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W {
        CAPTURE_W { w: self }
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Adjustable Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr](index.html) module"]
pub struct ATCR_SPEC;
impl crate::RegisterSpec for ATCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr::R](R) reader structure"]
impl crate::Readable for ATCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr::W](W) writer structure"]
impl crate::Writable for ATCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCR to value 0"]
impl crate::Resettable for ATCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
