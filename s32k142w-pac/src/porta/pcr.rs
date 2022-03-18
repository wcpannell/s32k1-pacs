#[doc = "Register `PCR%s` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR%s` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    PS_0 = 0,
    #[doc = "1: Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    PS_1 = 1,
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
            false => PS_A::PS_0,
            true => PS_A::PS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline(always)]
    pub fn is_ps_0(&self) -> bool {
        **self == PS_A::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline(always)]
    pub fn is_ps_1(&self) -> bool {
        **self == PS_A::PS_1
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
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PS_A::PS_0)
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PS_A::PS_1)
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
    PE_0 = 0,
    #[doc = "1: Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    PE_1 = 1,
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
            false => PE_A::PE_0,
            true => PE_A::PE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PE_0`"]
    #[inline(always)]
    pub fn is_pe_0(&self) -> bool {
        **self == PE_A::PE_0
    }
    #[doc = "Checks if the value of the field is `PE_1`"]
    #[inline(always)]
    pub fn is_pe_1(&self) -> bool {
        **self == PE_A::PE_1
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
    pub fn pe_0(self) -> &'a mut W {
        self.variant(PE_A::PE_0)
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn pe_1(self) -> &'a mut W {
        self.variant(PE_A::PE_1)
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
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFE_A {
    #[doc = "0: Passive input filter is disabled on the corresponding pin."]
    PFE_0 = 0,
    #[doc = "1: Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    PFE_1 = 1,
}
impl From<PFE_A> for bool {
    #[inline(always)]
    fn from(variant: PFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFE` reader - Passive Filter Enable"]
pub struct PFE_R(crate::FieldReader<bool, PFE_A>);
impl PFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFE_A {
        match self.bits {
            false => PFE_A::PFE_0,
            true => PFE_A::PFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFE_0`"]
    #[inline(always)]
    pub fn is_pfe_0(&self) -> bool {
        **self == PFE_A::PFE_0
    }
    #[doc = "Checks if the value of the field is `PFE_1`"]
    #[inline(always)]
    pub fn is_pfe_1(&self) -> bool {
        **self == PFE_A::PFE_1
    }
}
impl core::ops::Deref for PFE_R {
    type Target = crate::FieldReader<bool, PFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFE` writer - Passive Filter Enable"]
pub struct PFE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive input filter is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn pfe_0(self) -> &'a mut W {
        self.variant(PFE_A::PFE_0)
    }
    #[doc = "Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    #[inline(always)]
    pub fn pfe_1(self) -> &'a mut W {
        self.variant(PFE_A::PFE_1)
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
#[doc = "Drive Strength Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSE_A {
    #[doc = "0: Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    DSE_0 = 0,
    #[doc = "1: High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    DSE_1 = 1,
}
impl From<DSE_A> for bool {
    #[inline(always)]
    fn from(variant: DSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSE` reader - Drive Strength Enable"]
pub struct DSE_R(crate::FieldReader<bool, DSE_A>);
impl DSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSE_A {
        match self.bits {
            false => DSE_A::DSE_0,
            true => DSE_A::DSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSE_0`"]
    #[inline(always)]
    pub fn is_dse_0(&self) -> bool {
        **self == DSE_A::DSE_0
    }
    #[doc = "Checks if the value of the field is `DSE_1`"]
    #[inline(always)]
    pub fn is_dse_1(&self) -> bool {
        **self == DSE_A::DSE_1
    }
}
impl core::ops::Deref for DSE_R {
    type Target = crate::FieldReader<bool, DSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSE` writer - Drive Strength Enable"]
pub struct DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn dse_0(self) -> &'a mut W {
        self.variant(DSE_A::DSE_0)
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn dse_1(self) -> &'a mut W {
        self.variant(DSE_A::DSE_1)
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
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Pin disabled (Alternative 0) (analog)."]
    MUX_0 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    MUX_1 = 1,
    #[doc = "2: Alternative 2 (chip-specific)."]
    MUX_2 = 2,
    #[doc = "3: Alternative 3 (chip-specific)."]
    MUX_3 = 3,
    #[doc = "4: Alternative 4 (chip-specific)."]
    MUX_4 = 4,
    #[doc = "5: Alternative 5 (chip-specific)."]
    MUX_5 = 5,
    #[doc = "6: Alternative 6 (chip-specific)."]
    MUX_6 = 6,
    #[doc = "7: Alternative 7 (chip-specific)."]
    MUX_7 = 7,
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
            0 => MUX_A::MUX_0,
            1 => MUX_A::MUX_1,
            2 => MUX_A::MUX_2,
            3 => MUX_A::MUX_3,
            4 => MUX_A::MUX_4,
            5 => MUX_A::MUX_5,
            6 => MUX_A::MUX_6,
            7 => MUX_A::MUX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_0`"]
    #[inline(always)]
    pub fn is_mux_0(&self) -> bool {
        **self == MUX_A::MUX_0
    }
    #[doc = "Checks if the value of the field is `MUX_1`"]
    #[inline(always)]
    pub fn is_mux_1(&self) -> bool {
        **self == MUX_A::MUX_1
    }
    #[doc = "Checks if the value of the field is `MUX_2`"]
    #[inline(always)]
    pub fn is_mux_2(&self) -> bool {
        **self == MUX_A::MUX_2
    }
    #[doc = "Checks if the value of the field is `MUX_3`"]
    #[inline(always)]
    pub fn is_mux_3(&self) -> bool {
        **self == MUX_A::MUX_3
    }
    #[doc = "Checks if the value of the field is `MUX_4`"]
    #[inline(always)]
    pub fn is_mux_4(&self) -> bool {
        **self == MUX_A::MUX_4
    }
    #[doc = "Checks if the value of the field is `MUX_5`"]
    #[inline(always)]
    pub fn is_mux_5(&self) -> bool {
        **self == MUX_A::MUX_5
    }
    #[doc = "Checks if the value of the field is `MUX_6`"]
    #[inline(always)]
    pub fn is_mux_6(&self) -> bool {
        **self == MUX_A::MUX_6
    }
    #[doc = "Checks if the value of the field is `MUX_7`"]
    #[inline(always)]
    pub fn is_mux_7(&self) -> bool {
        **self == MUX_A::MUX_7
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
    pub fn mux_0(self) -> &'a mut W {
        self.variant(MUX_A::MUX_0)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn mux_1(self) -> &'a mut W {
        self.variant(MUX_A::MUX_1)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn mux_2(self) -> &'a mut W {
        self.variant(MUX_A::MUX_2)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn mux_3(self) -> &'a mut W {
        self.variant(MUX_A::MUX_3)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn mux_4(self) -> &'a mut W {
        self.variant(MUX_A::MUX_4)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn mux_5(self) -> &'a mut W {
        self.variant(MUX_A::MUX_5)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn mux_6(self) -> &'a mut W {
        self.variant(MUX_A::MUX_6)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn mux_7(self) -> &'a mut W {
        self.variant(MUX_A::MUX_7)
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
    LK_0 = 0,
    #[doc = "1: Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    LK_1 = 1,
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
            false => LK_A::LK_0,
            true => LK_A::LK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LK_0`"]
    #[inline(always)]
    pub fn is_lk_0(&self) -> bool {
        **self == LK_A::LK_0
    }
    #[doc = "Checks if the value of the field is `LK_1`"]
    #[inline(always)]
    pub fn is_lk_1(&self) -> bool {
        **self == LK_A::LK_1
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
    pub fn lk_0(self) -> &'a mut W {
        self.variant(LK_A::LK_0)
    }
    #[doc = "Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    #[inline(always)]
    pub fn lk_1(self) -> &'a mut W {
        self.variant(LK_A::LK_1)
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
    IRQC_0 = 0,
    #[doc = "1: ISF flag and DMA request on rising edge."]
    IRQC_1 = 1,
    #[doc = "2: ISF flag and DMA request on falling edge."]
    IRQC_2 = 2,
    #[doc = "3: ISF flag and DMA request on either edge."]
    IRQC_3 = 3,
    #[doc = "8: ISF flag and Interrupt when logic 0."]
    IRQC_8 = 8,
    #[doc = "9: ISF flag and Interrupt on rising-edge."]
    IRQC_9 = 9,
    #[doc = "10: ISF flag and Interrupt on falling-edge."]
    IRQC_10 = 10,
    #[doc = "11: ISF flag and Interrupt on either edge."]
    IRQC_11 = 11,
    #[doc = "12: ISF flag and Interrupt when logic 1."]
    IRQC_12 = 12,
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
            0 => Some(IRQC_A::IRQC_0),
            1 => Some(IRQC_A::IRQC_1),
            2 => Some(IRQC_A::IRQC_2),
            3 => Some(IRQC_A::IRQC_3),
            8 => Some(IRQC_A::IRQC_8),
            9 => Some(IRQC_A::IRQC_9),
            10 => Some(IRQC_A::IRQC_10),
            11 => Some(IRQC_A::IRQC_11),
            12 => Some(IRQC_A::IRQC_12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IRQC_0`"]
    #[inline(always)]
    pub fn is_irqc_0(&self) -> bool {
        **self == IRQC_A::IRQC_0
    }
    #[doc = "Checks if the value of the field is `IRQC_1`"]
    #[inline(always)]
    pub fn is_irqc_1(&self) -> bool {
        **self == IRQC_A::IRQC_1
    }
    #[doc = "Checks if the value of the field is `IRQC_2`"]
    #[inline(always)]
    pub fn is_irqc_2(&self) -> bool {
        **self == IRQC_A::IRQC_2
    }
    #[doc = "Checks if the value of the field is `IRQC_3`"]
    #[inline(always)]
    pub fn is_irqc_3(&self) -> bool {
        **self == IRQC_A::IRQC_3
    }
    #[doc = "Checks if the value of the field is `IRQC_8`"]
    #[inline(always)]
    pub fn is_irqc_8(&self) -> bool {
        **self == IRQC_A::IRQC_8
    }
    #[doc = "Checks if the value of the field is `IRQC_9`"]
    #[inline(always)]
    pub fn is_irqc_9(&self) -> bool {
        **self == IRQC_A::IRQC_9
    }
    #[doc = "Checks if the value of the field is `IRQC_10`"]
    #[inline(always)]
    pub fn is_irqc_10(&self) -> bool {
        **self == IRQC_A::IRQC_10
    }
    #[doc = "Checks if the value of the field is `IRQC_11`"]
    #[inline(always)]
    pub fn is_irqc_11(&self) -> bool {
        **self == IRQC_A::IRQC_11
    }
    #[doc = "Checks if the value of the field is `IRQC_12`"]
    #[inline(always)]
    pub fn is_irqc_12(&self) -> bool {
        **self == IRQC_A::IRQC_12
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
    pub fn irqc_0(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_0)
    }
    #[doc = "ISF flag and DMA request on rising edge."]
    #[inline(always)]
    pub fn irqc_1(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_1)
    }
    #[doc = "ISF flag and DMA request on falling edge."]
    #[inline(always)]
    pub fn irqc_2(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_2)
    }
    #[doc = "ISF flag and DMA request on either edge."]
    #[inline(always)]
    pub fn irqc_3(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_3)
    }
    #[doc = "ISF flag and Interrupt when logic 0."]
    #[inline(always)]
    pub fn irqc_8(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_8)
    }
    #[doc = "ISF flag and Interrupt on rising-edge."]
    #[inline(always)]
    pub fn irqc_9(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_9)
    }
    #[doc = "ISF flag and Interrupt on falling-edge."]
    #[inline(always)]
    pub fn irqc_10(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_10)
    }
    #[doc = "ISF flag and Interrupt on either edge."]
    #[inline(always)]
    pub fn irqc_11(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_11)
    }
    #[doc = "ISF flag and Interrupt when logic 1."]
    #[inline(always)]
    pub fn irqc_12(self) -> &'a mut W {
        self.variant(IRQC_A::IRQC_12)
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
    ISF_0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    ISF_1 = 1,
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
            false => ISF_A::ISF_0,
            true => ISF_A::ISF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISF_0`"]
    #[inline(always)]
    pub fn is_isf_0(&self) -> bool {
        **self == ISF_A::ISF_0
    }
    #[doc = "Checks if the value of the field is `ISF_1`"]
    #[inline(always)]
    pub fn is_isf_1(&self) -> bool {
        **self == ISF_A::ISF_1
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
    pub fn isf_0(self) -> &'a mut W {
        self.variant(ISF_A::ISF_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn isf_1(self) -> &'a mut W {
        self.variant(ISF_A::ISF_1)
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
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PFE_R {
        PFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DSE_R {
        DSE_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&mut self) -> PFE_W {
        PFE_W { w: self }
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&mut self) -> DSE_W {
        DSE_W { w: self }
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
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR%s to value 0"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
