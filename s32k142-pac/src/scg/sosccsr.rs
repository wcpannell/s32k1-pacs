#[doc = "Register `SOSCCSR` reader"]
pub struct R(crate::R<SOSCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOSCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOSCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOSCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOSCCSR` writer"]
pub struct W(crate::W<SOSCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOSCCSR_SPEC>;
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
impl From<crate::W<SOSCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOSCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System OSC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCEN_A {
    #[doc = "0: System OSC is disabled"]
    _0 = 0,
    #[doc = "1: System OSC is enabled"]
    _1 = 1,
}
impl From<SOSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCEN` reader - System OSC Enable"]
pub struct SOSCEN_R(crate::FieldReader<bool, SOSCEN_A>);
impl SOSCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCEN_A {
        match self.bits {
            false => SOSCEN_A::_0,
            true => SOSCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCEN_A::_1
    }
}
impl core::ops::Deref for SOSCEN_R {
    type Target = crate::FieldReader<bool, SOSCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCEN` writer - System OSC Enable"]
pub struct SOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System OSC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCEN_A::_0)
    }
    #[doc = "System OSC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCEN_A::_1)
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
#[doc = "System OSC Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCM_A {
    #[doc = "0: System OSC Clock Monitor is disabled"]
    _0 = 0,
    #[doc = "1: System OSC Clock Monitor is enabled"]
    _1 = 1,
}
impl From<SOSCCM_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCCM` reader - System OSC Clock Monitor"]
pub struct SOSCCM_R(crate::FieldReader<bool, SOSCCM_A>);
impl SOSCCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCCM_A {
        match self.bits {
            false => SOSCCM_A::_0,
            true => SOSCCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCCM_A::_1
    }
}
impl core::ops::Deref for SOSCCM_R {
    type Target = crate::FieldReader<bool, SOSCCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCCM` writer - System OSC Clock Monitor"]
pub struct SOSCCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System OSC Clock Monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCM_A::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCM_A::_1)
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
#[doc = "System OSC Clock Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCMRE_A {
    #[doc = "0: Clock Monitor generates interrupt when error detected"]
    _0 = 0,
    #[doc = "1: Clock Monitor generates reset when error detected"]
    _1 = 1,
}
impl From<SOSCCMRE_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCCMRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCCMRE` reader - System OSC Clock Monitor Reset Enable"]
pub struct SOSCCMRE_R(crate::FieldReader<bool, SOSCCMRE_A>);
impl SOSCCMRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCCMRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCCMRE_A {
        match self.bits {
            false => SOSCCMRE_A::_0,
            true => SOSCCMRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCCMRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCCMRE_A::_1
    }
}
impl core::ops::Deref for SOSCCMRE_R {
    type Target = crate::FieldReader<bool, SOSCCMRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCCMRE` writer - System OSC Clock Monitor Reset Enable"]
pub struct SOSCCMRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCCMRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCCMRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Monitor generates interrupt when error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCMRE_A::_0)
    }
    #[doc = "Clock Monitor generates reset when error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCMRE_A::_1)
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
    #[doc = "0: This Control Status Register can be written."]
    _0 = 0,
    #[doc = "1: This Control Status Register cannot be written."]
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
    #[doc = "This Control Status Register can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "This Control Status Register cannot be written."]
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
#[doc = "System OSC Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCVLD_A {
    #[doc = "0: System OSC is not enabled or clock is not valid"]
    _0 = 0,
    #[doc = "1: System OSC is enabled and output clock is valid"]
    _1 = 1,
}
impl From<SOSCVLD_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCVLD` reader - System OSC Valid"]
pub struct SOSCVLD_R(crate::FieldReader<bool, SOSCVLD_A>);
impl SOSCVLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCVLD_A {
        match self.bits {
            false => SOSCVLD_A::_0,
            true => SOSCVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCVLD_A::_1
    }
}
impl core::ops::Deref for SOSCVLD_R {
    type Target = crate::FieldReader<bool, SOSCVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System OSC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCSEL_A {
    #[doc = "0: System OSC is not the system clock source"]
    _0 = 0,
    #[doc = "1: System OSC is the system clock source"]
    _1 = 1,
}
impl From<SOSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCSEL` reader - System OSC Selected"]
pub struct SOSCSEL_R(crate::FieldReader<bool, SOSCSEL_A>);
impl SOSCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCSEL_A {
        match self.bits {
            false => SOSCSEL_A::_0,
            true => SOSCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCSEL_A::_1
    }
}
impl core::ops::Deref for SOSCSEL_R {
    type Target = crate::FieldReader<bool, SOSCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System OSC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCERR_A {
    #[doc = "0: System OSC Clock Monitor is disabled or has not detected an error"]
    _0 = 0,
    #[doc = "1: System OSC Clock Monitor is enabled and detected an error"]
    _1 = 1,
}
impl From<SOSCERR_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCERR` reader - System OSC Clock Error"]
pub struct SOSCERR_R(crate::FieldReader<bool, SOSCERR_A>);
impl SOSCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOSCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCERR_A {
        match self.bits {
            false => SOSCERR_A::_0,
            true => SOSCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOSCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOSCERR_A::_1
    }
}
impl core::ops::Deref for SOSCERR_R {
    type Target = crate::FieldReader<bool, SOSCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCERR` writer - System OSC Clock Error"]
pub struct SOSCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System OSC Clock Monitor is disabled or has not detected an error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCERR_A::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled and detected an error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCERR_A::_1)
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
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline(always)]
    pub fn soscen(&self) -> SOSCEN_R {
        SOSCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline(always)]
    pub fn sosccm(&self) -> SOSCCM_R {
        SOSCCM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn sosccmre(&self) -> SOSCCMRE_R {
        SOSCCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - System OSC Valid"]
    #[inline(always)]
    pub fn soscvld(&self) -> SOSCVLD_R {
        SOSCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - System OSC Selected"]
    #[inline(always)]
    pub fn soscsel(&self) -> SOSCSEL_R {
        SOSCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline(always)]
    pub fn soscerr(&self) -> SOSCERR_R {
        SOSCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline(always)]
    pub fn soscen(&mut self) -> SOSCEN_W {
        SOSCEN_W { w: self }
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline(always)]
    pub fn sosccm(&mut self) -> SOSCCM_W {
        SOSCCM_W { w: self }
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn sosccmre(&mut self) -> SOSCCMRE_W {
        SOSCCMRE_W { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline(always)]
    pub fn soscerr(&mut self) -> SOSCERR_W {
        SOSCERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System OSC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sosccsr](index.html) module"]
pub struct SOSCCSR_SPEC;
impl crate::RegisterSpec for SOSCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sosccsr::R](R) reader structure"]
impl crate::Readable for SOSCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sosccsr::W](W) writer structure"]
impl crate::Writable for SOSCCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOSCCSR to value 0"]
impl crate::Resettable for SOSCCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
