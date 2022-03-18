#[doc = "Register `QDCTRL` reader"]
pub struct R(crate::R<QDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDCTRL` writer"]
pub struct W(crate::W<QDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDCTRL_SPEC>;
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
impl From<crate::W<QDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Quadrature Decoder Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADEN_A {
    #[doc = "0: Quadrature Decoder mode is disabled."]
    QUADEN_0 = 0,
    #[doc = "1: Quadrature Decoder mode is enabled."]
    QUADEN_1 = 1,
}
impl From<QUADEN_A> for bool {
    #[inline(always)]
    fn from(variant: QUADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUADEN` reader - Quadrature Decoder Mode Enable"]
pub struct QUADEN_R(crate::FieldReader<bool, QUADEN_A>);
impl QUADEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADEN_A {
        match self.bits {
            false => QUADEN_A::QUADEN_0,
            true => QUADEN_A::QUADEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `QUADEN_0`"]
    #[inline(always)]
    pub fn is_quaden_0(&self) -> bool {
        **self == QUADEN_A::QUADEN_0
    }
    #[doc = "Checks if the value of the field is `QUADEN_1`"]
    #[inline(always)]
    pub fn is_quaden_1(&self) -> bool {
        **self == QUADEN_A::QUADEN_1
    }
}
impl core::ops::Deref for QUADEN_R {
    type Target = crate::FieldReader<bool, QUADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUADEN` writer - Quadrature Decoder Mode Enable"]
pub struct QUADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Quadrature Decoder mode is disabled."]
    #[inline(always)]
    pub fn quaden_0(self) -> &'a mut W {
        self.variant(QUADEN_A::QUADEN_0)
    }
    #[doc = "Quadrature Decoder mode is enabled."]
    #[inline(always)]
    pub fn quaden_1(self) -> &'a mut W {
        self.variant(QUADEN_A::QUADEN_1)
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
#[doc = "Timer Overflow Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIR_A {
    #[doc = "0: TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (CNTIN register) to its maximum value (MOD register)."]
    TOFDIR_0 = 0,
    #[doc = "1: TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (CNTIN register)."]
    TOFDIR_1 = 1,
}
impl From<TOFDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TOFDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOFDIR` reader - Timer Overflow Direction In Quadrature Decoder Mode"]
pub struct TOFDIR_R(crate::FieldReader<bool, TOFDIR_A>);
impl TOFDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOFDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOFDIR_A {
        match self.bits {
            false => TOFDIR_A::TOFDIR_0,
            true => TOFDIR_A::TOFDIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOFDIR_0`"]
    #[inline(always)]
    pub fn is_tofdir_0(&self) -> bool {
        **self == TOFDIR_A::TOFDIR_0
    }
    #[doc = "Checks if the value of the field is `TOFDIR_1`"]
    #[inline(always)]
    pub fn is_tofdir_1(&self) -> bool {
        **self == TOFDIR_A::TOFDIR_1
    }
}
impl core::ops::Deref for TOFDIR_R {
    type Target = crate::FieldReader<bool, TOFDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FTM Counter Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIR_A {
    #[doc = "0: Counting direction is decreasing (FTM counter decrement)."]
    QUADIR_0 = 0,
    #[doc = "1: Counting direction is increasing (FTM counter increment)."]
    QUADIR_1 = 1,
}
impl From<QUADIR_A> for bool {
    #[inline(always)]
    fn from(variant: QUADIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUADIR` reader - FTM Counter Direction In Quadrature Decoder Mode"]
pub struct QUADIR_R(crate::FieldReader<bool, QUADIR_A>);
impl QUADIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUADIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADIR_A {
        match self.bits {
            false => QUADIR_A::QUADIR_0,
            true => QUADIR_A::QUADIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `QUADIR_0`"]
    #[inline(always)]
    pub fn is_quadir_0(&self) -> bool {
        **self == QUADIR_A::QUADIR_0
    }
    #[doc = "Checks if the value of the field is `QUADIR_1`"]
    #[inline(always)]
    pub fn is_quadir_1(&self) -> bool {
        **self == QUADIR_A::QUADIR_1
    }
}
impl core::ops::Deref for QUADIR_R {
    type Target = crate::FieldReader<bool, QUADIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODE_A {
    #[doc = "0: Phase A and phase B encoding mode."]
    QUADMODE_0 = 0,
    #[doc = "1: Count and direction encoding mode."]
    QUADMODE_1 = 1,
}
impl From<QUADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: QUADMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUADMODE` reader - Quadrature Decoder Mode"]
pub struct QUADMODE_R(crate::FieldReader<bool, QUADMODE_A>);
impl QUADMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUADMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADMODE_A {
        match self.bits {
            false => QUADMODE_A::QUADMODE_0,
            true => QUADMODE_A::QUADMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QUADMODE_0`"]
    #[inline(always)]
    pub fn is_quadmode_0(&self) -> bool {
        **self == QUADMODE_A::QUADMODE_0
    }
    #[doc = "Checks if the value of the field is `QUADMODE_1`"]
    #[inline(always)]
    pub fn is_quadmode_1(&self) -> bool {
        **self == QUADMODE_A::QUADMODE_1
    }
}
impl core::ops::Deref for QUADMODE_R {
    type Target = crate::FieldReader<bool, QUADMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUADMODE` writer - Quadrature Decoder Mode"]
pub struct QUADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Phase A and phase B encoding mode."]
    #[inline(always)]
    pub fn quadmode_0(self) -> &'a mut W {
        self.variant(QUADMODE_A::QUADMODE_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline(always)]
    pub fn quadmode_1(self) -> &'a mut W {
        self.variant(QUADMODE_A::QUADMODE_1)
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
#[doc = "Phase B Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBPOL_A {
    #[doc = "0: Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    PHBPOL_0 = 0,
    #[doc = "1: Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    PHBPOL_1 = 1,
}
impl From<PHBPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHBPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHBPOL` reader - Phase B Input Polarity"]
pub struct PHBPOL_R(crate::FieldReader<bool, PHBPOL_A>);
impl PHBPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHBPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBPOL_A {
        match self.bits {
            false => PHBPOL_A::PHBPOL_0,
            true => PHBPOL_A::PHBPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHBPOL_0`"]
    #[inline(always)]
    pub fn is_phbpol_0(&self) -> bool {
        **self == PHBPOL_A::PHBPOL_0
    }
    #[doc = "Checks if the value of the field is `PHBPOL_1`"]
    #[inline(always)]
    pub fn is_phbpol_1(&self) -> bool {
        **self == PHBPOL_A::PHBPOL_1
    }
}
impl core::ops::Deref for PHBPOL_R {
    type Target = crate::FieldReader<bool, PHBPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHBPOL` writer - Phase B Input Polarity"]
pub struct PHBPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHBPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHBPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn phbpol_0(self) -> &'a mut W {
        self.variant(PHBPOL_A::PHBPOL_0)
    }
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn phbpol_1(self) -> &'a mut W {
        self.variant(PHBPOL_A::PHBPOL_1)
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
#[doc = "Phase A Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAPOL_A {
    #[doc = "0: Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    PHAPOL_0 = 0,
    #[doc = "1: Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    PHAPOL_1 = 1,
}
impl From<PHAPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHAPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHAPOL` reader - Phase A Input Polarity"]
pub struct PHAPOL_R(crate::FieldReader<bool, PHAPOL_A>);
impl PHAPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHAPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAPOL_A {
        match self.bits {
            false => PHAPOL_A::PHAPOL_0,
            true => PHAPOL_A::PHAPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHAPOL_0`"]
    #[inline(always)]
    pub fn is_phapol_0(&self) -> bool {
        **self == PHAPOL_A::PHAPOL_0
    }
    #[doc = "Checks if the value of the field is `PHAPOL_1`"]
    #[inline(always)]
    pub fn is_phapol_1(&self) -> bool {
        **self == PHAPOL_A::PHAPOL_1
    }
}
impl core::ops::Deref for PHAPOL_R {
    type Target = crate::FieldReader<bool, PHAPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHAPOL` writer - Phase A Input Polarity"]
pub struct PHAPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHAPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHAPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn phapol_0(self) -> &'a mut W {
        self.variant(PHAPOL_A::PHAPOL_0)
    }
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn phapol_1(self) -> &'a mut W {
        self.variant(PHAPOL_A::PHAPOL_1)
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
#[doc = "Phase B Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBFLTREN_A {
    #[doc = "0: Phase B input filter is disabled."]
    PHBFLTREN_0 = 0,
    #[doc = "1: Phase B input filter is enabled."]
    PHBFLTREN_1 = 1,
}
impl From<PHBFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHBFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHBFLTREN` reader - Phase B Input Filter Enable"]
pub struct PHBFLTREN_R(crate::FieldReader<bool, PHBFLTREN_A>);
impl PHBFLTREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHBFLTREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBFLTREN_A {
        match self.bits {
            false => PHBFLTREN_A::PHBFLTREN_0,
            true => PHBFLTREN_A::PHBFLTREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHBFLTREN_0`"]
    #[inline(always)]
    pub fn is_phbfltren_0(&self) -> bool {
        **self == PHBFLTREN_A::PHBFLTREN_0
    }
    #[doc = "Checks if the value of the field is `PHBFLTREN_1`"]
    #[inline(always)]
    pub fn is_phbfltren_1(&self) -> bool {
        **self == PHBFLTREN_A::PHBFLTREN_1
    }
}
impl core::ops::Deref for PHBFLTREN_R {
    type Target = crate::FieldReader<bool, PHBFLTREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHBFLTREN` writer - Phase B Input Filter Enable"]
pub struct PHBFLTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHBFLTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHBFLTREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Phase B input filter is disabled."]
    #[inline(always)]
    pub fn phbfltren_0(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::PHBFLTREN_0)
    }
    #[doc = "Phase B input filter is enabled."]
    #[inline(always)]
    pub fn phbfltren_1(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::PHBFLTREN_1)
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
#[doc = "Phase A Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAFLTREN_A {
    #[doc = "0: Phase A input filter is disabled."]
    PHAFLTREN_0 = 0,
    #[doc = "1: Phase A input filter is enabled."]
    PHAFLTREN_1 = 1,
}
impl From<PHAFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHAFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHAFLTREN` reader - Phase A Input Filter Enable"]
pub struct PHAFLTREN_R(crate::FieldReader<bool, PHAFLTREN_A>);
impl PHAFLTREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHAFLTREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAFLTREN_A {
        match self.bits {
            false => PHAFLTREN_A::PHAFLTREN_0,
            true => PHAFLTREN_A::PHAFLTREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHAFLTREN_0`"]
    #[inline(always)]
    pub fn is_phafltren_0(&self) -> bool {
        **self == PHAFLTREN_A::PHAFLTREN_0
    }
    #[doc = "Checks if the value of the field is `PHAFLTREN_1`"]
    #[inline(always)]
    pub fn is_phafltren_1(&self) -> bool {
        **self == PHAFLTREN_A::PHAFLTREN_1
    }
}
impl core::ops::Deref for PHAFLTREN_R {
    type Target = crate::FieldReader<bool, PHAFLTREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHAFLTREN` writer - Phase A Input Filter Enable"]
pub struct PHAFLTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHAFLTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHAFLTREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Phase A input filter is disabled."]
    #[inline(always)]
    pub fn phafltren_0(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::PHAFLTREN_0)
    }
    #[doc = "Phase A input filter is enabled."]
    #[inline(always)]
    pub fn phafltren_1(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::PHAFLTREN_1)
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
impl R {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    pub fn quaden(&self) -> QUADEN_R {
        QUADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Overflow Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn tofdir(&self) -> TOFDIR_R {
        TOFDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadir(&self) -> QUADIR_R {
        QUADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&self) -> QUADMODE_R {
        QUADMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    pub fn phbpol(&self) -> PHBPOL_R {
        PHBPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    pub fn phapol(&self) -> PHAPOL_R {
        PHAPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    pub fn phbfltren(&self) -> PHBFLTREN_R {
        PHBFLTREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    pub fn phafltren(&self) -> PHAFLTREN_R {
        PHAFLTREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    pub fn quaden(&mut self) -> QUADEN_W {
        QUADEN_W { w: self }
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&mut self) -> QUADMODE_W {
        QUADMODE_W { w: self }
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    pub fn phbpol(&mut self) -> PHBPOL_W {
        PHBPOL_W { w: self }
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    pub fn phapol(&mut self) -> PHAPOL_W {
        PHAPOL_W { w: self }
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    pub fn phbfltren(&mut self) -> PHBFLTREN_W {
        PHBFLTREN_W { w: self }
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    pub fn phafltren(&mut self) -> PHAFLTREN_W {
        PHAFLTREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Decoder Control And Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdctrl](index.html) module"]
pub struct QDCTRL_SPEC;
impl crate::RegisterSpec for QDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdctrl::R](R) reader structure"]
impl crate::Readable for QDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdctrl::W](W) writer structure"]
impl crate::Writable for QDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDCTRL to value 0"]
impl crate::Resettable for QDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
