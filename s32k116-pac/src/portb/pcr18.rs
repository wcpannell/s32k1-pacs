#[doc = "Register `PCR18` reader"]
pub struct R(crate::R<PCR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR18` writer"]
pub struct W(crate::W<PCR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR18_SPEC>;
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
impl From<crate::W<PCR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _0 = 0,
    #[doc = "1: Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Pull Select"]
pub struct PS_R(crate::FieldReader<bool, PS_A>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::_0,
            true => PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PS_A::_1
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Pull Select"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS_A::_0)
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS_A::_1)
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
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Pull Enable"]
pub struct PE_R(crate::FieldReader<bool, PE_A>);
impl PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PE_A::_1
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE` writer - Pull Enable"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
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
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Pin disabled (Alternative 0) (analog)."]
    _000 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    _001 = 1,
    #[doc = "2: Alternative 2 (chip-specific)."]
    _010 = 2,
    #[doc = "3: Alternative 3 (chip-specific)."]
    _011 = 3,
    #[doc = "4: Alternative 4 (chip-specific)."]
    _100 = 4,
    #[doc = "5: Alternative 5 (chip-specific)."]
    _101 = 5,
    #[doc = "6: Alternative 6 (chip-specific)."]
    _110 = 6,
    #[doc = "7: Alternative 7 (chip-specific)."]
    _111 = 7,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUX` reader - Pin Mux Control"]
pub struct MUX_R(crate::FieldReader<u8, MUX_A>);
impl MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::_000,
            1 => MUX_A::_001,
            2 => MUX_A::_010,
            3 => MUX_A::_011,
            4 => MUX_A::_100,
            5 => MUX_A::_101,
            6 => MUX_A::_110,
            7 => MUX_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == MUX_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == MUX_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == MUX_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == MUX_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == MUX_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == MUX_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == MUX_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == MUX_A::_111
    }
}
impl core::ops::Deref for MUX_R {
    type Target = crate::FieldReader<u8, MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX` writer - Pin Mux Control"]
pub struct MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pin disabled (Alternative 0) (analog)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MUX_A::_000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MUX_A::_001)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MUX_A::_010)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MUX_A::_011)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MUX_A::_100)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MUX_A::_101)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MUX_A::_110)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MUX_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: Pin Control Register fields \\[15:0\\]
are not locked."]
    _0 = 0,
    #[doc = "1: Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    _1 = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - Lock Register"]
pub struct LK_R(crate::FieldReader<bool, LK_A>);
impl LK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::_0,
            true => LK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LK_A::_1
    }
}
impl core::ops::Deref for LK_R {
    type Target = crate::FieldReader<bool, LK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LK` writer - Lock Register"]
pub struct LK_W<'a> {
    w: &'a mut W,
}
impl<'a> LK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin Control Register fields \\[15:0\\]
are not locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LK_A::_1)
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
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQC_A {
    #[doc = "0: Interrupt Status Flag (ISF) is disabled."]
    _0000 = 0,
    #[doc = "1: ISF flag and DMA request on rising edge."]
    _0001 = 1,
    #[doc = "2: ISF flag and DMA request on falling edge."]
    _0010 = 2,
    #[doc = "3: ISF flag and DMA request on either edge."]
    _0011 = 3,
    #[doc = "8: ISF flag and Interrupt when logic 0."]
    _1000 = 8,
    #[doc = "9: ISF flag and Interrupt on rising-edge."]
    _1001 = 9,
    #[doc = "10: ISF flag and Interrupt on falling-edge."]
    _1010 = 10,
    #[doc = "11: ISF flag and Interrupt on either edge."]
    _1011 = 11,
    #[doc = "12: ISF flag and Interrupt when logic 1."]
    _1100 = 12,
}
impl From<IRQC_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IRQC` reader - Interrupt Configuration"]
pub struct IRQC_R(crate::FieldReader<u8, IRQC_A>);
impl IRQC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IRQC_A> {
        match self.bits {
            0 => Some(IRQC_A::_0000),
            1 => Some(IRQC_A::_0001),
            2 => Some(IRQC_A::_0010),
            3 => Some(IRQC_A::_0011),
            8 => Some(IRQC_A::_1000),
            9 => Some(IRQC_A::_1001),
            10 => Some(IRQC_A::_1010),
            11 => Some(IRQC_A::_1011),
            12 => Some(IRQC_A::_1100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == IRQC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == IRQC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == IRQC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == IRQC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == IRQC_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == IRQC_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == IRQC_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == IRQC_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == IRQC_A::_1100
    }
}
impl core::ops::Deref for IRQC_R {
    type Target = crate::FieldReader<u8, IRQC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQC` writer - Interrupt Configuration"]
pub struct IRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt Status Flag (ISF) is disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(IRQC_A::_0000)
    }
    #[doc = "ISF flag and DMA request on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(IRQC_A::_0001)
    }
    #[doc = "ISF flag and DMA request on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(IRQC_A::_0010)
    }
    #[doc = "ISF flag and DMA request on either edge."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(IRQC_A::_0011)
    }
    #[doc = "ISF flag and Interrupt when logic 0."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(IRQC_A::_1000)
    }
    #[doc = "ISF flag and Interrupt on rising-edge."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(IRQC_A::_1001)
    }
    #[doc = "ISF flag and Interrupt on falling-edge."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(IRQC_A::_1010)
    }
    #[doc = "ISF flag and Interrupt on either edge."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(IRQC_A::_1011)
    }
    #[doc = "ISF flag and Interrupt when logic 1."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(IRQC_A::_1100)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF_A> for bool {
    #[inline(always)]
    fn from(variant: ISF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub struct ISF_R(crate::FieldReader<bool, ISF_A>);
impl ISF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF_A {
        match self.bits {
            false => ISF_A::_0,
            true => ISF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISF_A::_1
    }
}
impl core::ops::Deref for ISF_R {
    type Target = crate::FieldReader<bool, ISF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub struct ISF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IRQC_R {
        IRQC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W { w: self }
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&mut self) -> IRQC_W {
        IRQC_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&mut self) -> ISF_W {
        ISF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr18](index.html) module"]
pub struct PCR18_SPEC;
impl crate::RegisterSpec for PCR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr18::R](R) reader structure"]
impl crate::Readable for PCR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr18::W](W) writer structure"]
impl crate::Writable for PCR18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR18 to value 0"]
impl crate::Resettable for PCR18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
