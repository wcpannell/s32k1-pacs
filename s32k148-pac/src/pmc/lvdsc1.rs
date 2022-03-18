#[doc = "Register `LVDSC1` reader"]
pub struct R(crate::R<LVDSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDSC1` writer"]
pub struct W(crate::W<LVDSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDSC1_SPEC>;
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
impl From<crate::W<LVDSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low Voltage Detect Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRE_A {
    #[doc = "0: No system resets on low voltage detect events."]
    _0 = 0,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDRE` reader - Low Voltage Detect Reset Enable"]
pub struct LVDRE_R(crate::FieldReader<bool, LVDRE_A>);
impl LVDRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVDRE_A> {
        match self.bits {
            false => Some(LVDRE_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDRE_A::_0
    }
}
impl core::ops::Deref for LVDRE_R {
    type Target = crate::FieldReader<bool, LVDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDRE` writer - Low Voltage Detect Reset Enable"]
pub struct LVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No system resets on low voltage detect events."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Low Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVDF = 1"]
    _1 = 1,
}
impl From<LVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDIE` reader - Low Voltage Detect Interrupt Enable"]
pub struct LVDIE_R(crate::FieldReader<bool, LVDIE_A>);
impl LVDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDIE_A {
        match self.bits {
            false => LVDIE_A::_0,
            true => LVDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDIE_A::_1
    }
}
impl core::ops::Deref for LVDIE_R {
    type Target = crate::FieldReader<bool, LVDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDIE` writer - Low Voltage Detect Interrupt Enable"]
pub struct LVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LVDACK` writer - Low Voltage Detect Acknowledge"]
pub struct LVDACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Low Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDF_A {
    #[doc = "0: Low-voltage event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage event detected"]
    _1 = 1,
}
impl From<LVDF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDF` reader - Low Voltage Detect Flag"]
pub struct LVDF_R(crate::FieldReader<bool, LVDF_A>);
impl LVDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDF_A {
        match self.bits {
            false => LVDF_A::_0,
            true => LVDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDF_A::_1
    }
}
impl core::ops::Deref for LVDF_R {
    type Target = crate::FieldReader<bool, LVDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&self) -> LVDIE_R {
        LVDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low Voltage Detect Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&mut self) -> LVDRE_W {
        LVDRE_W { w: self }
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&mut self) -> LVDIE_W {
        LVDIE_W { w: self }
    }
    #[doc = "Bit 6 - Low Voltage Detect Acknowledge"]
    #[inline(always)]
    pub fn lvdack(&mut self) -> LVDACK_W {
        LVDACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detect Status and Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdsc1](index.html) module"]
pub struct LVDSC1_SPEC;
impl crate::RegisterSpec for LVDSC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvdsc1::R](R) reader structure"]
impl crate::Readable for LVDSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdsc1::W](W) writer structure"]
impl crate::Writable for LVDSC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVDSC1 to value 0"]
impl crate::Resettable for LVDSC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
