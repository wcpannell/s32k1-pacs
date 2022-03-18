#[doc = "Register `PMCTRL` reader"]
pub struct R(crate::R<PMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCTRL` writer"]
pub struct W(crate::W<PMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCTRL_SPEC>;
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
impl From<crate::W<PMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOPM_A {
    #[doc = "0: Normal Stop (STOP)"]
    _000 = 0,
    #[doc = "2: Very-Low-Power Stop (VLPS)"]
    _010 = 2,
    #[doc = "6: Reseved"]
    _110 = 6,
}
impl From<STOPM_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STOPM` reader - Stop Mode Control"]
pub struct STOPM_R(crate::FieldReader<u8, STOPM_A>);
impl STOPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STOPM_A> {
        match self.bits {
            0 => Some(STOPM_A::_000),
            2 => Some(STOPM_A::_010),
            6 => Some(STOPM_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == STOPM_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == STOPM_A::_010
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == STOPM_A::_110
    }
}
impl core::ops::Deref for STOPM_R {
    type Target = crate::FieldReader<u8, STOPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPM` writer - Stop Mode Control"]
pub struct STOPM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Stop (STOP)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(STOPM_A::_000)
    }
    #[doc = "Very-Low-Power Stop (VLPS)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(STOPM_A::_010)
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(STOPM_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Very Low Power Stop Aborted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLPSA_A {
    #[doc = "0: The previous stop mode entry was successful."]
    _0 = 0,
    #[doc = "1: The previous stop mode entry was aborted."]
    _1 = 1,
}
impl From<VLPSA_A> for bool {
    #[inline(always)]
    fn from(variant: VLPSA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLPSA` reader - Very Low Power Stop Aborted"]
pub struct VLPSA_R(crate::FieldReader<bool, VLPSA_A>);
impl VLPSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VLPSA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLPSA_A {
        match self.bits {
            false => VLPSA_A::_0,
            true => VLPSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VLPSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VLPSA_A::_1
    }
}
impl core::ops::Deref for VLPSA_R {
    type Target = crate::FieldReader<bool, VLPSA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Run Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RUNM_A {
    #[doc = "0: Normal Run mode (RUN)"]
    _00 = 0,
    #[doc = "2: Very-Low-Power Run mode (VLPR)"]
    _10 = 2,
}
impl From<RUNM_A> for u8 {
    #[inline(always)]
    fn from(variant: RUNM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RUNM` reader - Run Mode Control"]
pub struct RUNM_R(crate::FieldReader<u8, RUNM_A>);
impl RUNM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RUNM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUNM_A> {
        match self.bits {
            0 => Some(RUNM_A::_00),
            2 => Some(RUNM_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == RUNM_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == RUNM_A::_10
    }
}
impl core::ops::Deref for RUNM_R {
    type Target = crate::FieldReader<u8, RUNM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNM` writer - Run Mode Control"]
pub struct RUNM_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUNM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Run mode (RUN)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RUNM_A::_00)
    }
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RUNM_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&self) -> STOPM_R {
        STOPM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Very Low Power Stop Aborted"]
    #[inline(always)]
    pub fn vlpsa(&self) -> VLPSA_R {
        VLPSA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&self) -> RUNM_R {
        RUNM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&mut self) -> STOPM_W {
        STOPM_W { w: self }
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&mut self) -> RUNM_W {
        RUNM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctrl](index.html) module"]
pub struct PMCTRL_SPEC;
impl crate::RegisterSpec for PMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmctrl::R](R) reader structure"]
impl crate::Readable for PMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](W) writer structure"]
impl crate::Writable for PMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
