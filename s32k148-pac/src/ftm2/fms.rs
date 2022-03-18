#[doc = "Register `FMS` reader"]
pub struct R(crate::R<FMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMS` writer"]
pub struct W(crate::W<FMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMS_SPEC>;
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
impl From<crate::W<FMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fault Detection Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF0_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTF0` reader - Fault Detection Flag 0"]
pub struct FAULTF0_R(crate::FieldReader<bool, FAULTF0_A>);
impl FAULTF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF0_A {
        match self.bits {
            false => FAULTF0_A::_0,
            true => FAULTF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTF0_A::_1
    }
}
impl core::ops::Deref for FAULTF0_R {
    type Target = crate::FieldReader<bool, FAULTF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault Detection Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF1_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTF1` reader - Fault Detection Flag 1"]
pub struct FAULTF1_R(crate::FieldReader<bool, FAULTF1_A>);
impl FAULTF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF1_A {
        match self.bits {
            false => FAULTF1_A::_0,
            true => FAULTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTF1_A::_1
    }
}
impl core::ops::Deref for FAULTF1_R {
    type Target = crate::FieldReader<bool, FAULTF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault Detection Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF2_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTF2` reader - Fault Detection Flag 2"]
pub struct FAULTF2_R(crate::FieldReader<bool, FAULTF2_A>);
impl FAULTF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF2_A {
        match self.bits {
            false => FAULTF2_A::_0,
            true => FAULTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTF2_A::_1
    }
}
impl core::ops::Deref for FAULTF2_R {
    type Target = crate::FieldReader<bool, FAULTF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault Detection Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF3_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTF3` reader - Fault Detection Flag 3"]
pub struct FAULTF3_R(crate::FieldReader<bool, FAULTF3_A>);
impl FAULTF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF3_A {
        match self.bits {
            false => FAULTF3_A::_0,
            true => FAULTF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTF3_A::_1
    }
}
impl core::ops::Deref for FAULTF3_R {
    type Target = crate::FieldReader<bool, FAULTF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTIN_A {
    #[doc = "0: The logic OR of the enabled fault inputs is 0."]
    _0 = 0,
    #[doc = "1: The logic OR of the enabled fault inputs is 1."]
    _1 = 1,
}
impl From<FAULTIN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTIN` reader - Fault Inputs"]
pub struct FAULTIN_R(crate::FieldReader<bool, FAULTIN_A>);
impl FAULTIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTIN_A {
        match self.bits {
            false => FAULTIN_A::_0,
            true => FAULTIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTIN_A::_1
    }
}
impl core::ops::Deref for FAULTIN_R {
    type Target = crate::FieldReader<bool, FAULTIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPEN_A {
    #[doc = "0: Write protection is disabled. Write protected bits can be written."]
    _0 = 0,
    #[doc = "1: Write protection is enabled. Write protected bits cannot be written."]
    _1 = 1,
}
impl From<WPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub struct WPEN_R(crate::FieldReader<bool, WPEN_A>);
impl WPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPEN_A {
        match self.bits {
            false => WPEN_A::_0,
            true => WPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WPEN_A::_1
    }
}
impl core::ops::Deref for WPEN_R {
    type Target = crate::FieldReader<bool, WPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub struct WPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPEN_A::_0)
    }
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Fault Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF_A {
    #[doc = "0: No fault condition was detected."]
    _0 = 0,
    #[doc = "1: A fault condition was detected."]
    _1 = 1,
}
impl From<FAULTF_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTF` reader - Fault Detection Flag"]
pub struct FAULTF_R(crate::FieldReader<bool, FAULTF_A>);
impl FAULTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF_A {
        match self.bits {
            false => FAULTF_A::_0,
            true => FAULTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTF_A::_1
    }
}
impl core::ops::Deref for FAULTF_R {
    type Target = crate::FieldReader<bool, FAULTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline(always)]
    pub fn faultf0(&self) -> FAULTF0_R {
        FAULTF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline(always)]
    pub fn faultf1(&self) -> FAULTF1_R {
        FAULTF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline(always)]
    pub fn faultf2(&self) -> FAULTF2_R {
        FAULTF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline(always)]
    pub fn faultf3(&self) -> FAULTF3_R {
        FAULTF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Inputs"]
    #[inline(always)]
    pub fn faultin(&self) -> FAULTIN_R {
        FAULTIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline(always)]
    pub fn faultf(&self) -> FAULTF_R {
        FAULTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W {
        WPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Mode Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fms](index.html) module"]
pub struct FMS_SPEC;
impl crate::RegisterSpec for FMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fms::R](R) reader structure"]
impl crate::Readable for FMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fms::W](W) writer structure"]
impl crate::Writable for FMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMS to value 0"]
impl crate::Resettable for FMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
