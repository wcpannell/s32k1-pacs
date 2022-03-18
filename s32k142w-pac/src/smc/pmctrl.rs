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
    STOPM_0 = 0,
    #[doc = "2: Very-Low-Power Stop (VLPS)"]
    STOPM_2 = 2,
    #[doc = "6: Reseved"]
    STOPM_6 = 6,
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
            0 => Some(STOPM_A::STOPM_0),
            2 => Some(STOPM_A::STOPM_2),
            6 => Some(STOPM_A::STOPM_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOPM_0`"]
    #[inline(always)]
    pub fn is_stopm_0(&self) -> bool {
        **self == STOPM_A::STOPM_0
    }
    #[doc = "Checks if the value of the field is `STOPM_2`"]
    #[inline(always)]
    pub fn is_stopm_2(&self) -> bool {
        **self == STOPM_A::STOPM_2
    }
    #[doc = "Checks if the value of the field is `STOPM_6`"]
    #[inline(always)]
    pub fn is_stopm_6(&self) -> bool {
        **self == STOPM_A::STOPM_6
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
    pub fn stopm_0(self) -> &'a mut W {
        self.variant(STOPM_A::STOPM_0)
    }
    #[doc = "Very-Low-Power Stop (VLPS)"]
    #[inline(always)]
    pub fn stopm_2(self) -> &'a mut W {
        self.variant(STOPM_A::STOPM_2)
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn stopm_6(self) -> &'a mut W {
        self.variant(STOPM_A::STOPM_6)
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
    VLPSA_0 = 0,
    #[doc = "1: The previous stop mode entry was aborted."]
    VLPSA_1 = 1,
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
            false => VLPSA_A::VLPSA_0,
            true => VLPSA_A::VLPSA_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLPSA_0`"]
    #[inline(always)]
    pub fn is_vlpsa_0(&self) -> bool {
        **self == VLPSA_A::VLPSA_0
    }
    #[doc = "Checks if the value of the field is `VLPSA_1`"]
    #[inline(always)]
    pub fn is_vlpsa_1(&self) -> bool {
        **self == VLPSA_A::VLPSA_1
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
    RUNM_0 = 0,
    #[doc = "2: Very-Low-Power Run mode (VLPR)"]
    RUNM_2 = 2,
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
            0 => Some(RUNM_A::RUNM_0),
            2 => Some(RUNM_A::RUNM_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RUNM_0`"]
    #[inline(always)]
    pub fn is_runm_0(&self) -> bool {
        **self == RUNM_A::RUNM_0
    }
    #[doc = "Checks if the value of the field is `RUNM_2`"]
    #[inline(always)]
    pub fn is_runm_2(&self) -> bool {
        **self == RUNM_A::RUNM_2
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
    pub fn runm_0(self) -> &'a mut W {
        self.variant(RUNM_A::RUNM_0)
    }
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    #[inline(always)]
    pub fn runm_2(self) -> &'a mut W {
        self.variant(RUNM_A::RUNM_2)
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
