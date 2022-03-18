#[doc = "Register `SPLLCSR` reader"]
pub struct R(crate::R<SPLLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLLCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPLLCSR` writer"]
pub struct W(crate::W<SPLLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLLCSR_SPEC>;
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
impl From<crate::W<SPLLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLLCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System PLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLEN_A {
    #[doc = "0: System PLL is disabled"]
    _0 = 0,
    #[doc = "1: System PLL is enabled"]
    _1 = 1,
}
impl From<SPLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLEN` reader - System PLL Enable"]
pub struct SPLLEN_R(crate::FieldReader<bool, SPLLEN_A>);
impl SPLLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLEN_A {
        match self.bits {
            false => SPLLEN_A::_0,
            true => SPLLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLEN_A::_1
    }
}
impl core::ops::Deref for SPLLEN_R {
    type Target = crate::FieldReader<bool, SPLLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLEN` writer - System PLL Enable"]
pub struct SPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System PLL is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLEN_A::_0)
    }
    #[doc = "System PLL is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLEN_A::_1)
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
#[doc = "System PLL Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLCM_A {
    #[doc = "0: System PLL Clock Monitor is disabled"]
    _0 = 0,
    #[doc = "1: System PLL Clock Monitor is enabled"]
    _1 = 1,
}
impl From<SPLLCM_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLCM` reader - System PLL Clock Monitor"]
pub struct SPLLCM_R(crate::FieldReader<bool, SPLLCM_A>);
impl SPLLCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLCM_A {
        match self.bits {
            false => SPLLCM_A::_0,
            true => SPLLCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLCM_A::_1
    }
}
impl core::ops::Deref for SPLLCM_R {
    type Target = crate::FieldReader<bool, SPLLCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLCM` writer - System PLL Clock Monitor"]
pub struct SPLLCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System PLL Clock Monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLCM_A::_0)
    }
    #[doc = "System PLL Clock Monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLCM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "System PLL Clock Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLCMRE_A {
    #[doc = "0: Clock Monitor generates interrupt when error detected"]
    _0 = 0,
    #[doc = "1: Clock Monitor generates reset when error detected"]
    _1 = 1,
}
impl From<SPLLCMRE_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLCMRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLCMRE` reader - System PLL Clock Monitor Reset Enable"]
pub struct SPLLCMRE_R(crate::FieldReader<bool, SPLLCMRE_A>);
impl SPLLCMRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLCMRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLCMRE_A {
        match self.bits {
            false => SPLLCMRE_A::_0,
            true => SPLLCMRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLCMRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLCMRE_A::_1
    }
}
impl core::ops::Deref for SPLLCMRE_R {
    type Target = crate::FieldReader<bool, SPLLCMRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLCMRE` writer - System PLL Clock Monitor Reset Enable"]
pub struct SPLLCMRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLCMRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLCMRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Monitor generates interrupt when error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLCMRE_A::_0)
    }
    #[doc = "Clock Monitor generates reset when error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLCMRE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: Control Status Register can be written."]
    _0 = 0,
    #[doc = "1: Control Status Register cannot be written."]
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
    #[doc = "Control Status Register can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "Control Status Register cannot be written."]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "System PLL Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLVLD_A {
    #[doc = "0: System PLL is not enabled or clock is not valid"]
    _0 = 0,
    #[doc = "1: System PLL is enabled and output clock is valid"]
    _1 = 1,
}
impl From<SPLLVLD_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLVLD` reader - System PLL Valid"]
pub struct SPLLVLD_R(crate::FieldReader<bool, SPLLVLD_A>);
impl SPLLVLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLVLD_A {
        match self.bits {
            false => SPLLVLD_A::_0,
            true => SPLLVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLVLD_A::_1
    }
}
impl core::ops::Deref for SPLLVLD_R {
    type Target = crate::FieldReader<bool, SPLLVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System PLL Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLSEL_A {
    #[doc = "0: System PLL is not the system clock source"]
    _0 = 0,
    #[doc = "1: System PLL is the system clock source"]
    _1 = 1,
}
impl From<SPLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLSEL` reader - System PLL Selected"]
pub struct SPLLSEL_R(crate::FieldReader<bool, SPLLSEL_A>);
impl SPLLSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLSEL_A {
        match self.bits {
            false => SPLLSEL_A::_0,
            true => SPLLSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLSEL_A::_1
    }
}
impl core::ops::Deref for SPLLSEL_R {
    type Target = crate::FieldReader<bool, SPLLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System PLL Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLERR_A {
    #[doc = "0: System PLL Clock Monitor is disabled or has not detected an error"]
    _0 = 0,
    #[doc = "1: System PLL Clock Monitor is enabled and detected an error. System PLL Clock Error flag will not set when System OSC is selected as its source and SOSCERR has set."]
    _1 = 1,
}
impl From<SPLLERR_A> for bool {
    #[inline(always)]
    fn from(variant: SPLLERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLLERR` reader - System PLL Clock Error"]
pub struct SPLLERR_R(crate::FieldReader<bool, SPLLERR_A>);
impl SPLLERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPLLERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLERR_A {
        match self.bits {
            false => SPLLERR_A::_0,
            true => SPLLERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLLERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLLERR_A::_1
    }
}
impl core::ops::Deref for SPLLERR_R {
    type Target = crate::FieldReader<bool, SPLLERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLERR` writer - System PLL Clock Error"]
pub struct SPLLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System PLL Clock Monitor is disabled or has not detected an error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLERR_A::_0)
    }
    #[doc = "System PLL Clock Monitor is enabled and detected an error. System PLL Clock Error flag will not set when System OSC is selected as its source and SOSCERR has set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System PLL Enable"]
    #[inline(always)]
    pub fn spllen(&self) -> SPLLEN_R {
        SPLLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - System PLL Clock Monitor"]
    #[inline(always)]
    pub fn spllcm(&self) -> SPLLCM_R {
        SPLLCM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - System PLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn spllcmre(&self) -> SPLLCMRE_R {
        SPLLCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - System PLL Valid"]
    #[inline(always)]
    pub fn spllvld(&self) -> SPLLVLD_R {
        SPLLVLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - System PLL Selected"]
    #[inline(always)]
    pub fn spllsel(&self) -> SPLLSEL_R {
        SPLLSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - System PLL Clock Error"]
    #[inline(always)]
    pub fn spllerr(&self) -> SPLLERR_R {
        SPLLERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System PLL Enable"]
    #[inline(always)]
    pub fn spllen(&mut self) -> SPLLEN_W {
        SPLLEN_W { w: self }
    }
    #[doc = "Bit 16 - System PLL Clock Monitor"]
    #[inline(always)]
    pub fn spllcm(&mut self) -> SPLLCM_W {
        SPLLCM_W { w: self }
    }
    #[doc = "Bit 17 - System PLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn spllcmre(&mut self) -> SPLLCMRE_W {
        SPLLCMRE_W { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Bit 26 - System PLL Clock Error"]
    #[inline(always)]
    pub fn spllerr(&mut self) -> SPLLERR_W {
        SPLLERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllcsr](index.html) module"]
pub struct SPLLCSR_SPEC;
impl crate::RegisterSpec for SPLLCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spllcsr::R](R) reader structure"]
impl crate::Readable for SPLLCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spllcsr::W](W) writer structure"]
impl crate::Writable for SPLLCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPLLCSR to value 0"]
impl crate::Resettable for SPLLCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
