#[doc = "Register `SOSCCFG` reader"]
pub struct R(crate::R<SOSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOSCCFG` writer"]
pub struct W(crate::W<SOSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOSCCFG_SPEC>;
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
impl From<crate::W<SOSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFS_A {
    #[doc = "0: External reference clock selected"]
    _0 = 0,
    #[doc = "1: Internal crystal oscillator of OSC selected."]
    _1 = 1,
}
impl From<EREFS_A> for bool {
    #[inline(always)]
    fn from(variant: EREFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFS` reader - External Reference Select"]
pub struct EREFS_R(crate::FieldReader<bool, EREFS_A>);
impl EREFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EREFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFS_A {
        match self.bits {
            false => EREFS_A::_0,
            true => EREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EREFS_A::_1
    }
}
impl core::ops::Deref for EREFS_R {
    type Target = crate::FieldReader<bool, EREFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EREFS` writer - External Reference Select"]
pub struct EREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EREFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External reference clock selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS_A::_0)
    }
    #[doc = "Internal crystal oscillator of OSC selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS_A::_1)
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
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO_A {
    #[doc = "0: Configure crystal oscillator for low-gain operation"]
    _0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation"]
    _1 = 1,
}
impl From<HGO_A> for bool {
    #[inline(always)]
    fn from(variant: HGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO` reader - High Gain Oscillator Select"]
pub struct HGO_R(crate::FieldReader<bool, HGO_A>);
impl HGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO_A {
        match self.bits {
            false => HGO_A::_0,
            true => HGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HGO_A::_1
    }
}
impl core::ops::Deref for HGO_R {
    type Target = crate::FieldReader<bool, HGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HGO` writer - High Gain Oscillator Select"]
pub struct HGO_W<'a> {
    w: &'a mut W,
}
impl<'a> HGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HGO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configure crystal oscillator for low-gain operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO_A::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO_A::_1)
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
#[doc = "System OSC Range Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "1: Low frequency range selected for the crystal oscillator"]
    _01 = 1,
    #[doc = "2: Medium frequency range selected for the crytstal oscillator"]
    _10 = 2,
    #[doc = "3: High frequency range selected for the crystal oscillator"]
    _11 = 3,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGE` reader - System OSC Range Select"]
pub struct RANGE_R(crate::FieldReader<u8, RANGE_A>);
impl RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            1 => Some(RANGE_A::_01),
            2 => Some(RANGE_A::_10),
            3 => Some(RANGE_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == RANGE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == RANGE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == RANGE_A::_11
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<u8, RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - System OSC Range Select"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low frequency range selected for the crystal oscillator"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE_A::_01)
    }
    #[doc = "Medium frequency range selected for the crytstal oscillator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RANGE_A::_10)
    }
    #[doc = "High frequency range selected for the crystal oscillator"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RANGE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&self) -> EREFS_R {
        EREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&self) -> HGO_R {
        HGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&mut self) -> EREFS_W {
        EREFS_W { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&mut self) -> HGO_W {
        HGO_W { w: self }
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Oscillator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sosccfg](index.html) module"]
pub struct SOSCCFG_SPEC;
impl crate::RegisterSpec for SOSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sosccfg::R](R) reader structure"]
impl crate::Readable for SOSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sosccfg::W](W) writer structure"]
impl crate::Writable for SOSCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOSCCFG to value 0x10"]
impl crate::Resettable for SOSCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
