#[doc = "Register `EICHEN` reader"]
pub struct R(crate::R<EICHEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICHEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICHEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICHEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICHEN` writer"]
pub struct W(crate::W<EICHEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICHEN_SPEC>;
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
impl From<crate::W<EICHEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICHEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error Injection Channel 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH1EN_A {
    #[doc = "0: Error injection is disabled on Error Injection Channel 1"]
    EICH1EN_0 = 0,
    #[doc = "1: Error injection is enabled on Error Injection Channel 1"]
    EICH1EN_1 = 1,
}
impl From<EICH1EN_A> for bool {
    #[inline(always)]
    fn from(variant: EICH1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EICH1EN` reader - Error Injection Channel 1 Enable"]
pub struct EICH1EN_R(crate::FieldReader<bool, EICH1EN_A>);
impl EICH1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EICH1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EICH1EN_A {
        match self.bits {
            false => EICH1EN_A::EICH1EN_0,
            true => EICH1EN_A::EICH1EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EICH1EN_0`"]
    #[inline(always)]
    pub fn is_eich1en_0(&self) -> bool {
        **self == EICH1EN_A::EICH1EN_0
    }
    #[doc = "Checks if the value of the field is `EICH1EN_1`"]
    #[inline(always)]
    pub fn is_eich1en_1(&self) -> bool {
        **self == EICH1EN_A::EICH1EN_1
    }
}
impl core::ops::Deref for EICH1EN_R {
    type Target = crate::FieldReader<bool, EICH1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EICH1EN` writer - Error Injection Channel 1 Enable"]
pub struct EICH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EICH1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EICH1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error injection is disabled on Error Injection Channel 1"]
    #[inline(always)]
    pub fn eich1en_0(self) -> &'a mut W {
        self.variant(EICH1EN_A::EICH1EN_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 1"]
    #[inline(always)]
    pub fn eich1en_1(self) -> &'a mut W {
        self.variant(EICH1EN_A::EICH1EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Error Injection Channel 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH0EN_A {
    #[doc = "0: Error injection is disabled on Error Injection Channel 0"]
    EICH0EN_0 = 0,
    #[doc = "1: Error injection is enabled on Error Injection Channel 0"]
    EICH0EN_1 = 1,
}
impl From<EICH0EN_A> for bool {
    #[inline(always)]
    fn from(variant: EICH0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EICH0EN` reader - Error Injection Channel 0 Enable"]
pub struct EICH0EN_R(crate::FieldReader<bool, EICH0EN_A>);
impl EICH0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EICH0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EICH0EN_A {
        match self.bits {
            false => EICH0EN_A::EICH0EN_0,
            true => EICH0EN_A::EICH0EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EICH0EN_0`"]
    #[inline(always)]
    pub fn is_eich0en_0(&self) -> bool {
        **self == EICH0EN_A::EICH0EN_0
    }
    #[doc = "Checks if the value of the field is `EICH0EN_1`"]
    #[inline(always)]
    pub fn is_eich0en_1(&self) -> bool {
        **self == EICH0EN_A::EICH0EN_1
    }
}
impl core::ops::Deref for EICH0EN_R {
    type Target = crate::FieldReader<bool, EICH0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EICH0EN` writer - Error Injection Channel 0 Enable"]
pub struct EICH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EICH0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EICH0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error injection is disabled on Error Injection Channel 0"]
    #[inline(always)]
    pub fn eich0en_0(self) -> &'a mut W {
        self.variant(EICH0EN_A::EICH0EN_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 0"]
    #[inline(always)]
    pub fn eich0en_1(self) -> &'a mut W {
        self.variant(EICH0EN_A::EICH0EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub fn eich1en(&self) -> EICH1EN_R {
        EICH1EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub fn eich0en(&self) -> EICH0EN_R {
        EICH0EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub fn eich1en(&mut self) -> EICH1EN_W {
        EICH1EN_W { w: self }
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub fn eich0en(&mut self) -> EICH0EN_W {
        EICH0EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Injection Channel Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichen](index.html) module"]
pub struct EICHEN_SPEC;
impl crate::RegisterSpec for EICHEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eichen::R](R) reader structure"]
impl crate::Readable for EICHEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eichen::W](W) writer structure"]
impl crate::Writable for EICHEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EICHEN to value 0"]
impl crate::Resettable for EICHEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
