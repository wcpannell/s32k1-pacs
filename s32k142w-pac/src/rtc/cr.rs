#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
    #[doc = "0: No effect."]
    SWR_0 = 0,
    #[doc = "1: Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    SWR_1 = 1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub struct SWR_R(crate::FieldReader<bool, SWR_A>);
impl SWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWR_A {
        match self.bits {
            false => SWR_A::SWR_0,
            true => SWR_A::SWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWR_0`"]
    #[inline(always)]
    pub fn is_swr_0(&self) -> bool {
        **self == SWR_A::SWR_0
    }
    #[doc = "Checks if the value of the field is `SWR_1`"]
    #[inline(always)]
    pub fn is_swr_1(&self) -> bool {
        **self == SWR_A::SWR_1
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, SWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - Software Reset"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn swr_0(self) -> &'a mut W {
        self.variant(SWR_A::SWR_0)
    }
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    #[inline(always)]
    pub fn swr_1(self) -> &'a mut W {
        self.variant(SWR_A::SWR_1)
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
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUP_A {
    #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
    SUP_0 = 0,
    #[doc = "1: Non-supervisor mode write accesses are supported."]
    SUP_1 = 1,
}
impl From<SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUP` reader - Supervisor Access"]
pub struct SUP_R(crate::FieldReader<bool, SUP_A>);
impl SUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUP_A {
        match self.bits {
            false => SUP_A::SUP_0,
            true => SUP_A::SUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUP_0`"]
    #[inline(always)]
    pub fn is_sup_0(&self) -> bool {
        **self == SUP_A::SUP_0
    }
    #[doc = "Checks if the value of the field is `SUP_1`"]
    #[inline(always)]
    pub fn is_sup_1(&self) -> bool {
        **self == SUP_A::SUP_1
    }
}
impl core::ops::Deref for SUP_R {
    type Target = crate::FieldReader<bool, SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUP` writer - Supervisor Access"]
pub struct SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn sup_0(self) -> &'a mut W {
        self.variant(SUP_A::SUP_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn sup_1(self) -> &'a mut W {
        self.variant(SUP_A::SUP_1)
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
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UM_A {
    #[doc = "0: Registers cannot be written when locked."]
    UM_0 = 0,
    #[doc = "1: Registers can be written when locked under limited conditions."]
    UM_1 = 1,
}
impl From<UM_A> for bool {
    #[inline(always)]
    fn from(variant: UM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UM` reader - Update Mode"]
pub struct UM_R(crate::FieldReader<bool, UM_A>);
impl UM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UM_A {
        match self.bits {
            false => UM_A::UM_0,
            true => UM_A::UM_1,
        }
    }
    #[doc = "Checks if the value of the field is `UM_0`"]
    #[inline(always)]
    pub fn is_um_0(&self) -> bool {
        **self == UM_A::UM_0
    }
    #[doc = "Checks if the value of the field is `UM_1`"]
    #[inline(always)]
    pub fn is_um_1(&self) -> bool {
        **self == UM_A::UM_1
    }
}
impl core::ops::Deref for UM_R {
    type Target = crate::FieldReader<bool, UM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UM` writer - Update Mode"]
pub struct UM_W<'a> {
    w: &'a mut W,
}
impl<'a> UM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn um_0(self) -> &'a mut W {
        self.variant(UM_A::UM_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn um_1(self) -> &'a mut W {
        self.variant(UM_A::UM_1)
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
#[doc = "Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPS_A {
    #[doc = "0: The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
    CPS_0 = 0,
    #[doc = "1: The RTC 32.768 kHz clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    CPS_1 = 1,
}
impl From<CPS_A> for bool {
    #[inline(always)]
    fn from(variant: CPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPS` reader - Clock Pin Select"]
pub struct CPS_R(crate::FieldReader<bool, CPS_A>);
impl CPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPS_A {
        match self.bits {
            false => CPS_A::CPS_0,
            true => CPS_A::CPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPS_0`"]
    #[inline(always)]
    pub fn is_cps_0(&self) -> bool {
        **self == CPS_A::CPS_0
    }
    #[doc = "Checks if the value of the field is `CPS_1`"]
    #[inline(always)]
    pub fn is_cps_1(&self) -> bool {
        **self == CPS_A::CPS_1
    }
}
impl core::ops::Deref for CPS_R {
    type Target = crate::FieldReader<bool, CPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPS` writer - Clock Pin Select"]
pub struct CPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
    #[inline(always)]
    pub fn cps_0(self) -> &'a mut W {
        self.variant(CPS_A::CPS_0)
    }
    #[doc = "The RTC 32.768 kHz clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    #[inline(always)]
    pub fn cps_1(self) -> &'a mut W {
        self.variant(CPS_A::CPS_1)
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
#[doc = "LPO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOS_A {
    #[doc = "0: RTC prescaler increments using 32.768 kHz clock."]
    LPOS_0 = 0,
    #[doc = "1: RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\]
of the prescaler are ignored."]
    LPOS_1 = 1,
}
impl From<LPOS_A> for bool {
    #[inline(always)]
    fn from(variant: LPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOS` reader - LPO Select"]
pub struct LPOS_R(crate::FieldReader<bool, LPOS_A>);
impl LPOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOS_A {
        match self.bits {
            false => LPOS_A::LPOS_0,
            true => LPOS_A::LPOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPOS_0`"]
    #[inline(always)]
    pub fn is_lpos_0(&self) -> bool {
        **self == LPOS_A::LPOS_0
    }
    #[doc = "Checks if the value of the field is `LPOS_1`"]
    #[inline(always)]
    pub fn is_lpos_1(&self) -> bool {
        **self == LPOS_A::LPOS_1
    }
}
impl core::ops::Deref for LPOS_R {
    type Target = crate::FieldReader<bool, LPOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOS` writer - LPO Select"]
pub struct LPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC prescaler increments using 32.768 kHz clock."]
    #[inline(always)]
    pub fn lpos_0(self) -> &'a mut W {
        self.variant(LPOS_A::LPOS_0)
    }
    #[doc = "RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\]
of the prescaler are ignored."]
    #[inline(always)]
    pub fn lpos_1(self) -> &'a mut W {
        self.variant(LPOS_A::LPOS_1)
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
#[doc = "Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO_A {
    #[doc = "0: The 32 kHz clock is output to other peripherals."]
    CLKO_0 = 0,
    #[doc = "1: The 32 kHz clock is not output to other peripherals."]
    CLKO_1 = 1,
}
impl From<CLKO_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKO` reader - Clock Output"]
pub struct CLKO_R(crate::FieldReader<bool, CLKO_A>);
impl CLKO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO_A {
        match self.bits {
            false => CLKO_A::CLKO_0,
            true => CLKO_A::CLKO_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO_0`"]
    #[inline(always)]
    pub fn is_clko_0(&self) -> bool {
        **self == CLKO_A::CLKO_0
    }
    #[doc = "Checks if the value of the field is `CLKO_1`"]
    #[inline(always)]
    pub fn is_clko_1(&self) -> bool {
        **self == CLKO_A::CLKO_1
    }
}
impl core::ops::Deref for CLKO_R {
    type Target = crate::FieldReader<bool, CLKO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKO` writer - Clock Output"]
pub struct CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn clko_0(self) -> &'a mut W {
        self.variant(CLKO_A::CLKO_0)
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline(always)]
    pub fn clko_1(self) -> &'a mut W {
        self.variant(CLKO_A::CLKO_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Clock Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPE_A {
    #[doc = "0: The RTC_CLKOUT function is disabled."]
    CPE_0 = 0,
    #[doc = "1: Enable RTC_CLKOUT function."]
    CPE_1 = 1,
}
impl From<CPE_A> for bool {
    #[inline(always)]
    fn from(variant: CPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPE` reader - Clock Pin Enable"]
pub struct CPE_R(crate::FieldReader<bool, CPE_A>);
impl CPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPE_A {
        match self.bits {
            false => CPE_A::CPE_0,
            true => CPE_A::CPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPE_0`"]
    #[inline(always)]
    pub fn is_cpe_0(&self) -> bool {
        **self == CPE_A::CPE_0
    }
    #[doc = "Checks if the value of the field is `CPE_1`"]
    #[inline(always)]
    pub fn is_cpe_1(&self) -> bool {
        **self == CPE_A::CPE_1
    }
}
impl core::ops::Deref for CPE_R {
    type Target = crate::FieldReader<bool, CPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPE` writer - Clock Pin Enable"]
pub struct CPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The RTC_CLKOUT function is disabled."]
    #[inline(always)]
    pub fn cpe_0(self) -> &'a mut W {
        self.variant(CPE_A::CPE_0)
    }
    #[doc = "Enable RTC_CLKOUT function."]
    #[inline(always)]
    pub fn cpe_1(self) -> &'a mut W {
        self.variant(CPE_A::CPE_1)
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&self) -> SUP_R {
        SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&self) -> UM_R {
        UM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline(always)]
    pub fn lpos(&self) -> LPOS_R {
        LPOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&mut self) -> SUP_W {
        SUP_W { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&mut self) -> UM_W {
        UM_W { w: self }
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline(always)]
    pub fn cps(&mut self) -> CPS_W {
        CPS_W { w: self }
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline(always)]
    pub fn lpos(&mut self) -> LPOS_W {
        LPOS_W { w: self }
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline(always)]
    pub fn cpe(&mut self) -> CPE_W {
        CPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
