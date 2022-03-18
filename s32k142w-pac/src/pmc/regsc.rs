#[doc = "Register `REGSC` reader"]
pub struct R(crate::R<REGSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGSC` writer"]
pub struct W(crate::W<REGSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGSC_SPEC>;
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
impl From<crate::W<REGSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bias Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASEN_A {
    #[doc = "0: Biasing disabled, core logic can run in full performance"]
    BIASEN_0 = 0,
    #[doc = "1: Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    BIASEN_1 = 1,
}
impl From<BIASEN_A> for bool {
    #[inline(always)]
    fn from(variant: BIASEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIASEN` reader - Bias Enable Bit"]
pub struct BIASEN_R(crate::FieldReader<bool, BIASEN_A>);
impl BIASEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIASEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIASEN_A {
        match self.bits {
            false => BIASEN_A::BIASEN_0,
            true => BIASEN_A::BIASEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIASEN_0`"]
    #[inline(always)]
    pub fn is_biasen_0(&self) -> bool {
        **self == BIASEN_A::BIASEN_0
    }
    #[doc = "Checks if the value of the field is `BIASEN_1`"]
    #[inline(always)]
    pub fn is_biasen_1(&self) -> bool {
        **self == BIASEN_A::BIASEN_1
    }
}
impl core::ops::Deref for BIASEN_R {
    type Target = crate::FieldReader<bool, BIASEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASEN` writer - Bias Enable Bit"]
pub struct BIASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIASEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Biasing disabled, core logic can run in full performance"]
    #[inline(always)]
    pub fn biasen_0(self) -> &'a mut W {
        self.variant(BIASEN_A::BIASEN_0)
    }
    #[doc = "Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    #[inline(always)]
    pub fn biasen_1(self) -> &'a mut W {
        self.variant(BIASEN_A::BIASEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Clock Bias Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKBIASDIS_A {
    #[doc = "0: No effect"]
    CLKBIASDIS_0 = 0,
    #[doc = "1: In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    CLKBIASDIS_1 = 1,
}
impl From<CLKBIASDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKBIASDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKBIASDIS` reader - Clock Bias Disable Bit"]
pub struct CLKBIASDIS_R(crate::FieldReader<bool, CLKBIASDIS_A>);
impl CLKBIASDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKBIASDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKBIASDIS_A {
        match self.bits {
            false => CLKBIASDIS_A::CLKBIASDIS_0,
            true => CLKBIASDIS_A::CLKBIASDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKBIASDIS_0`"]
    #[inline(always)]
    pub fn is_clkbiasdis_0(&self) -> bool {
        **self == CLKBIASDIS_A::CLKBIASDIS_0
    }
    #[doc = "Checks if the value of the field is `CLKBIASDIS_1`"]
    #[inline(always)]
    pub fn is_clkbiasdis_1(&self) -> bool {
        **self == CLKBIASDIS_A::CLKBIASDIS_1
    }
}
impl core::ops::Deref for CLKBIASDIS_R {
    type Target = crate::FieldReader<bool, CLKBIASDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKBIASDIS` writer - Clock Bias Disable Bit"]
pub struct CLKBIASDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKBIASDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKBIASDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clkbiasdis_0(self) -> &'a mut W {
        self.variant(CLKBIASDIS_A::CLKBIASDIS_0)
    }
    #[doc = "In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    #[inline(always)]
    pub fn clkbiasdis_1(self) -> &'a mut W {
        self.variant(CLKBIASDIS_A::CLKBIASDIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Regulator in Full Performance Mode Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGFPM_A {
    #[doc = "0: Regulator is in low power mode or transition to/from"]
    REGFPM_0 = 0,
    #[doc = "1: Regulator is in full performance mode"]
    REGFPM_1 = 1,
}
impl From<REGFPM_A> for bool {
    #[inline(always)]
    fn from(variant: REGFPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGFPM` reader - Regulator in Full Performance Mode Status Bit"]
pub struct REGFPM_R(crate::FieldReader<bool, REGFPM_A>);
impl REGFPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGFPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGFPM_A {
        match self.bits {
            false => REGFPM_A::REGFPM_0,
            true => REGFPM_A::REGFPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `REGFPM_0`"]
    #[inline(always)]
    pub fn is_regfpm_0(&self) -> bool {
        **self == REGFPM_A::REGFPM_0
    }
    #[doc = "Checks if the value of the field is `REGFPM_1`"]
    #[inline(always)]
    pub fn is_regfpm_1(&self) -> bool {
        **self == REGFPM_A::REGFPM_1
    }
}
impl core::ops::Deref for REGFPM_R {
    type Target = crate::FieldReader<bool, REGFPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPO Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSTAT_A {
    #[doc = "0: Low power oscillator in low phase"]
    LPOSTAT_0 = 0,
    #[doc = "1: Low power oscillator in high phase"]
    LPOSTAT_1 = 1,
}
impl From<LPOSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOSTAT` reader - LPO Status Bit"]
pub struct LPOSTAT_R(crate::FieldReader<bool, LPOSTAT_A>);
impl LPOSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPOSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSTAT_A {
        match self.bits {
            false => LPOSTAT_A::LPOSTAT_0,
            true => LPOSTAT_A::LPOSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPOSTAT_0`"]
    #[inline(always)]
    pub fn is_lpostat_0(&self) -> bool {
        **self == LPOSTAT_A::LPOSTAT_0
    }
    #[doc = "Checks if the value of the field is `LPOSTAT_1`"]
    #[inline(always)]
    pub fn is_lpostat_1(&self) -> bool {
        **self == LPOSTAT_A::LPOSTAT_1
    }
}
impl core::ops::Deref for LPOSTAT_R {
    type Target = crate::FieldReader<bool, LPOSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPO Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPODIS_A {
    #[doc = "0: Low power oscillator enabled"]
    LPODIS_0 = 0,
    #[doc = "1: Low power oscillator disabled"]
    LPODIS_1 = 1,
}
impl From<LPODIS_A> for bool {
    #[inline(always)]
    fn from(variant: LPODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPODIS` reader - LPO Disable Bit"]
pub struct LPODIS_R(crate::FieldReader<bool, LPODIS_A>);
impl LPODIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPODIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPODIS_A {
        match self.bits {
            false => LPODIS_A::LPODIS_0,
            true => LPODIS_A::LPODIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPODIS_0`"]
    #[inline(always)]
    pub fn is_lpodis_0(&self) -> bool {
        **self == LPODIS_A::LPODIS_0
    }
    #[doc = "Checks if the value of the field is `LPODIS_1`"]
    #[inline(always)]
    pub fn is_lpodis_1(&self) -> bool {
        **self == LPODIS_A::LPODIS_1
    }
}
impl core::ops::Deref for LPODIS_R {
    type Target = crate::FieldReader<bool, LPODIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPODIS` writer - LPO Disable Bit"]
pub struct LPODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low power oscillator enabled"]
    #[inline(always)]
    pub fn lpodis_0(self) -> &'a mut W {
        self.variant(LPODIS_A::LPODIS_0)
    }
    #[doc = "Low power oscillator disabled"]
    #[inline(always)]
    pub fn lpodis_1(self) -> &'a mut W {
        self.variant(LPODIS_A::LPODIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline(always)]
    pub fn biasen(&self) -> BIASEN_R {
        BIASEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline(always)]
    pub fn clkbiasdis(&self) -> CLKBIASDIS_R {
        CLKBIASDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Regulator in Full Performance Mode Status Bit"]
    #[inline(always)]
    pub fn regfpm(&self) -> REGFPM_R {
        REGFPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPO Status Bit"]
    #[inline(always)]
    pub fn lpostat(&self) -> LPOSTAT_R {
        LPOSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline(always)]
    pub fn lpodis(&self) -> LPODIS_R {
        LPODIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline(always)]
    pub fn biasen(&mut self) -> BIASEN_W {
        BIASEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline(always)]
    pub fn clkbiasdis(&mut self) -> CLKBIASDIS_W {
        CLKBIASDIS_W { w: self }
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline(always)]
    pub fn lpodis(&mut self) -> LPODIS_W {
        LPODIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regsc](index.html) module"]
pub struct REGSC_SPEC;
impl crate::RegisterSpec for REGSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [regsc::R](R) reader structure"]
impl crate::Readable for REGSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regsc::W](W) writer structure"]
impl crate::Writable for REGSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGSC to value 0x04"]
impl crate::Resettable for REGSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
