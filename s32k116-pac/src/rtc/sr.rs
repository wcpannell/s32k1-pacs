#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Invalid Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: Time is valid."]
    _0 = 0,
    #[doc = "1: Time is invalid and time counter is read as zero."]
    _1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Time Invalid Flag"]
pub struct TIF_R(crate::FieldReader<bool, TIF_A>);
impl TIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::_0,
            true => TIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIF_A::_1
    }
}
impl core::ops::Deref for TIF_R {
    type Target = crate::FieldReader<bool, TIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Time Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: Time overflow has not occurred."]
    _0 = 0,
    #[doc = "1: Time overflow has occurred and time counter is read as zero."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Time Overflow Flag"]
pub struct TOF_R(crate::FieldReader<bool, TOF_A>);
impl TOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOF_A::_1
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, TOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Time Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAF_A {
    #[doc = "0: Time alarm has not occurred."]
    _0 = 0,
    #[doc = "1: Time alarm has occurred."]
    _1 = 1,
}
impl From<TAF_A> for bool {
    #[inline(always)]
    fn from(variant: TAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAF` reader - Time Alarm Flag"]
pub struct TAF_R(crate::FieldReader<bool, TAF_A>);
impl TAF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAF_A {
        match self.bits {
            false => TAF_A::_0,
            true => TAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TAF_A::_1
    }
}
impl core::ops::Deref for TAF_R {
    type Target = crate::FieldReader<bool, TAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE_A {
    #[doc = "0: Time counter is disabled."]
    _0 = 0,
    #[doc = "1: Time counter is enabled."]
    _1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Time Counter Enable"]
pub struct TCE_R(crate::FieldReader<bool, TCE_A>);
impl TCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::_0,
            true => TCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCE_A::_1
    }
}
impl core::ops::Deref for TCE_R {
    type Target = crate::FieldReader<bool, TCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCE` writer - Time Counter Enable"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time counter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE_A::_0)
    }
    #[doc = "Time counter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
