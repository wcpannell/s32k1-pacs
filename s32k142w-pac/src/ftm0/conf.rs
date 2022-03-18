#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDFQ` reader - Frequency of the Reload Opportunities"]
pub struct LDFQ_R(crate::FieldReader<u8, u8>);
impl LDFQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDFQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDFQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDFQ` writer - Frequency of the Reload Opportunities"]
pub struct LDFQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LDFQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `BDMMODE` reader - Debug Mode"]
pub struct BDMMODE_R(crate::FieldReader<u8, u8>);
impl BDMMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BDMMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMMODE` writer - Debug Mode"]
pub struct BDMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Global Time Base Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
    #[doc = "0: Use of an external global time base is disabled."]
    GTBEEN_0 = 0,
    #[doc = "1: Use of an external global time base is enabled."]
    GTBEEN_1 = 1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEEN` reader - Global Time Base Enable"]
pub struct GTBEEN_R(crate::FieldReader<bool, GTBEEN_A>);
impl GTBEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GTBEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::GTBEEN_0,
            true => GTBEEN_A::GTBEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GTBEEN_0`"]
    #[inline(always)]
    pub fn is_gtbeen_0(&self) -> bool {
        **self == GTBEEN_A::GTBEEN_0
    }
    #[doc = "Checks if the value of the field is `GTBEEN_1`"]
    #[inline(always)]
    pub fn is_gtbeen_1(&self) -> bool {
        **self == GTBEEN_A::GTBEEN_1
    }
}
impl core::ops::Deref for GTBEEN_R {
    type Target = crate::FieldReader<bool, GTBEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBEEN` writer - Global Time Base Enable"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use of an external global time base is disabled."]
    #[inline(always)]
    pub fn gtbeen_0(self) -> &'a mut W {
        self.variant(GTBEEN_A::GTBEEN_0)
    }
    #[doc = "Use of an external global time base is enabled."]
    #[inline(always)]
    pub fn gtbeen_1(self) -> &'a mut W {
        self.variant(GTBEEN_A::GTBEEN_1)
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
#[doc = "Global Time Base Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEOUT_A {
    #[doc = "0: A global time base signal generation is disabled."]
    GTBEOUT_0 = 0,
    #[doc = "1: A global time base signal generation is enabled."]
    GTBEOUT_1 = 1,
}
impl From<GTBEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEOUT` reader - Global Time Base Output"]
pub struct GTBEOUT_R(crate::FieldReader<bool, GTBEOUT_A>);
impl GTBEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GTBEOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEOUT_A {
        match self.bits {
            false => GTBEOUT_A::GTBEOUT_0,
            true => GTBEOUT_A::GTBEOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GTBEOUT_0`"]
    #[inline(always)]
    pub fn is_gtbeout_0(&self) -> bool {
        **self == GTBEOUT_A::GTBEOUT_0
    }
    #[doc = "Checks if the value of the field is `GTBEOUT_1`"]
    #[inline(always)]
    pub fn is_gtbeout_1(&self) -> bool {
        **self == GTBEOUT_A::GTBEOUT_1
    }
}
impl core::ops::Deref for GTBEOUT_R {
    type Target = crate::FieldReader<bool, GTBEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBEOUT` writer - Global Time Base Output"]
pub struct GTBEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A global time base signal generation is disabled."]
    #[inline(always)]
    pub fn gtbeout_0(self) -> &'a mut W {
        self.variant(GTBEOUT_A::GTBEOUT_0)
    }
    #[doc = "A global time base signal generation is enabled."]
    #[inline(always)]
    pub fn gtbeout_1(self) -> &'a mut W {
        self.variant(GTBEOUT_A::GTBEOUT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Initialization trigger on Reload Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITRIGR_A {
    #[doc = "0: Initialization trigger is generated on counter wrap events."]
    ITRIGR_0 = 0,
    #[doc = "1: Initialization trigger is generated when a reload point is reached."]
    ITRIGR_1 = 1,
}
impl From<ITRIGR_A> for bool {
    #[inline(always)]
    fn from(variant: ITRIGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITRIGR` reader - Initialization trigger on Reload Point"]
pub struct ITRIGR_R(crate::FieldReader<bool, ITRIGR_A>);
impl ITRIGR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITRIGR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITRIGR_A {
        match self.bits {
            false => ITRIGR_A::ITRIGR_0,
            true => ITRIGR_A::ITRIGR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITRIGR_0`"]
    #[inline(always)]
    pub fn is_itrigr_0(&self) -> bool {
        **self == ITRIGR_A::ITRIGR_0
    }
    #[doc = "Checks if the value of the field is `ITRIGR_1`"]
    #[inline(always)]
    pub fn is_itrigr_1(&self) -> bool {
        **self == ITRIGR_A::ITRIGR_1
    }
}
impl core::ops::Deref for ITRIGR_R {
    type Target = crate::FieldReader<bool, ITRIGR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIGR` writer - Initialization trigger on Reload Point"]
pub struct ITRIGR_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIGR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITRIGR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Initialization trigger is generated on counter wrap events."]
    #[inline(always)]
    pub fn itrigr_0(self) -> &'a mut W {
        self.variant(ITRIGR_A::ITRIGR_0)
    }
    #[doc = "Initialization trigger is generated when a reload point is reached."]
    #[inline(always)]
    pub fn itrigr_1(self) -> &'a mut W {
        self.variant(ITRIGR_A::ITRIGR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline(always)]
    pub fn ldfq(&self) -> LDFQ_R {
        LDFQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&self) -> BDMMODE_R {
        BDMMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&self) -> GTBEOUT_R {
        GTBEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline(always)]
    pub fn itrigr(&self) -> ITRIGR_R {
        ITRIGR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline(always)]
    pub fn ldfq(&mut self) -> LDFQ_W {
        LDFQ_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&mut self) -> BDMMODE_W {
        BDMMODE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&mut self) -> GTBEOUT_W {
        GTBEOUT_W { w: self }
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline(always)]
    pub fn itrigr(&mut self) -> ITRIGR_W {
        ITRIGR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
