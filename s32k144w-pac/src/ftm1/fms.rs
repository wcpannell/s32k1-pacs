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
    FAULTF0_0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    FAULTF0_1 = 1,
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
            false => FAULTF0_A::FAULTF0_0,
            true => FAULTF0_A::FAULTF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTF0_0`"]
    #[inline(always)]
    pub fn is_faultf0_0(&self) -> bool {
        **self == FAULTF0_A::FAULTF0_0
    }
    #[doc = "Checks if the value of the field is `FAULTF0_1`"]
    #[inline(always)]
    pub fn is_faultf0_1(&self) -> bool {
        **self == FAULTF0_A::FAULTF0_1
    }
}
impl core::ops::Deref for FAULTF0_R {
    type Target = crate::FieldReader<bool, FAULTF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTF0` writer - Fault Detection Flag 0"]
pub struct FAULTF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf0_0(self) -> &'a mut W {
        self.variant(FAULTF0_A::FAULTF0_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf0_1(self) -> &'a mut W {
        self.variant(FAULTF0_A::FAULTF0_1)
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
#[doc = "Fault Detection Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF1_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    FAULTF1_0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    FAULTF1_1 = 1,
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
            false => FAULTF1_A::FAULTF1_0,
            true => FAULTF1_A::FAULTF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTF1_0`"]
    #[inline(always)]
    pub fn is_faultf1_0(&self) -> bool {
        **self == FAULTF1_A::FAULTF1_0
    }
    #[doc = "Checks if the value of the field is `FAULTF1_1`"]
    #[inline(always)]
    pub fn is_faultf1_1(&self) -> bool {
        **self == FAULTF1_A::FAULTF1_1
    }
}
impl core::ops::Deref for FAULTF1_R {
    type Target = crate::FieldReader<bool, FAULTF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTF1` writer - Fault Detection Flag 1"]
pub struct FAULTF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf1_0(self) -> &'a mut W {
        self.variant(FAULTF1_A::FAULTF1_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf1_1(self) -> &'a mut W {
        self.variant(FAULTF1_A::FAULTF1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Fault Detection Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF2_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    FAULTF2_0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    FAULTF2_1 = 1,
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
            false => FAULTF2_A::FAULTF2_0,
            true => FAULTF2_A::FAULTF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTF2_0`"]
    #[inline(always)]
    pub fn is_faultf2_0(&self) -> bool {
        **self == FAULTF2_A::FAULTF2_0
    }
    #[doc = "Checks if the value of the field is `FAULTF2_1`"]
    #[inline(always)]
    pub fn is_faultf2_1(&self) -> bool {
        **self == FAULTF2_A::FAULTF2_1
    }
}
impl core::ops::Deref for FAULTF2_R {
    type Target = crate::FieldReader<bool, FAULTF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTF2` writer - Fault Detection Flag 2"]
pub struct FAULTF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf2_0(self) -> &'a mut W {
        self.variant(FAULTF2_A::FAULTF2_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf2_1(self) -> &'a mut W {
        self.variant(FAULTF2_A::FAULTF2_1)
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
#[doc = "Fault Detection Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF3_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    FAULTF3_0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    FAULTF3_1 = 1,
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
            false => FAULTF3_A::FAULTF3_0,
            true => FAULTF3_A::FAULTF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTF3_0`"]
    #[inline(always)]
    pub fn is_faultf3_0(&self) -> bool {
        **self == FAULTF3_A::FAULTF3_0
    }
    #[doc = "Checks if the value of the field is `FAULTF3_1`"]
    #[inline(always)]
    pub fn is_faultf3_1(&self) -> bool {
        **self == FAULTF3_A::FAULTF3_1
    }
}
impl core::ops::Deref for FAULTF3_R {
    type Target = crate::FieldReader<bool, FAULTF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTF3` writer - Fault Detection Flag 3"]
pub struct FAULTF3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf3_0(self) -> &'a mut W {
        self.variant(FAULTF3_A::FAULTF3_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn faultf3_1(self) -> &'a mut W {
        self.variant(FAULTF3_A::FAULTF3_1)
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
#[doc = "Fault Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTIN_A {
    #[doc = "0: The logic OR of the enabled fault inputs is 0."]
    FAULTIN_0 = 0,
    #[doc = "1: The logic OR of the enabled fault inputs is 1."]
    FAULTIN_1 = 1,
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
            false => FAULTIN_A::FAULTIN_0,
            true => FAULTIN_A::FAULTIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTIN_0`"]
    #[inline(always)]
    pub fn is_faultin_0(&self) -> bool {
        **self == FAULTIN_A::FAULTIN_0
    }
    #[doc = "Checks if the value of the field is `FAULTIN_1`"]
    #[inline(always)]
    pub fn is_faultin_1(&self) -> bool {
        **self == FAULTIN_A::FAULTIN_1
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
    WPEN_0 = 0,
    #[doc = "1: Write protection is enabled. Write protected bits cannot be written."]
    WPEN_1 = 1,
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
            false => WPEN_A::WPEN_0,
            true => WPEN_A::WPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPEN_0`"]
    #[inline(always)]
    pub fn is_wpen_0(&self) -> bool {
        **self == WPEN_A::WPEN_0
    }
    #[doc = "Checks if the value of the field is `WPEN_1`"]
    #[inline(always)]
    pub fn is_wpen_1(&self) -> bool {
        **self == WPEN_A::WPEN_1
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
    pub fn wpen_0(self) -> &'a mut W {
        self.variant(WPEN_A::WPEN_0)
    }
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    #[inline(always)]
    pub fn wpen_1(self) -> &'a mut W {
        self.variant(WPEN_A::WPEN_1)
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
    FAULTF_0 = 0,
    #[doc = "1: A fault condition was detected."]
    FAULTF_1 = 1,
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
            false => FAULTF_A::FAULTF_0,
            true => FAULTF_A::FAULTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULTF_0`"]
    #[inline(always)]
    pub fn is_faultf_0(&self) -> bool {
        **self == FAULTF_A::FAULTF_0
    }
    #[doc = "Checks if the value of the field is `FAULTF_1`"]
    #[inline(always)]
    pub fn is_faultf_1(&self) -> bool {
        **self == FAULTF_A::FAULTF_1
    }
}
impl core::ops::Deref for FAULTF_R {
    type Target = crate::FieldReader<bool, FAULTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTF` writer - Fault Detection Flag"]
pub struct FAULTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No fault condition was detected."]
    #[inline(always)]
    pub fn faultf_0(self) -> &'a mut W {
        self.variant(FAULTF_A::FAULTF_0)
    }
    #[doc = "A fault condition was detected."]
    #[inline(always)]
    pub fn faultf_1(self) -> &'a mut W {
        self.variant(FAULTF_A::FAULTF_1)
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
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline(always)]
    pub fn faultf0(&mut self) -> FAULTF0_W {
        FAULTF0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline(always)]
    pub fn faultf1(&mut self) -> FAULTF1_W {
        FAULTF1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline(always)]
    pub fn faultf2(&mut self) -> FAULTF2_W {
        FAULTF2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline(always)]
    pub fn faultf3(&mut self) -> FAULTF3_W {
        FAULTF3_W { w: self }
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W {
        WPEN_W { w: self }
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline(always)]
    pub fn faultf(&mut self) -> FAULTF_W {
        FAULTF_W { w: self }
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
