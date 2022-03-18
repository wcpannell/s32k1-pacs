#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Existence of HSRUN feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHSRUN_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EHSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: EHSRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHSRUN` reader - Existence of HSRUN feature"]
pub struct EHSRUN_R(crate::FieldReader<bool, EHSRUN_A>);
impl EHSRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EHSRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHSRUN_A {
        match self.bits {
            false => EHSRUN_A::_0,
            true => EHSRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EHSRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EHSRUN_A::_1
    }
}
impl core::ops::Deref for EHSRUN_R {
    type Target = crate::FieldReader<bool, EHSRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of LLS feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLS_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELLS_A> for bool {
    #[inline(always)]
    fn from(variant: ELLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELLS` reader - Existence of LLS feature"]
pub struct ELLS_R(crate::FieldReader<bool, ELLS_A>);
impl ELLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELLS_A {
        match self.bits {
            false => ELLS_A::_0,
            true => ELLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ELLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ELLS_A::_1
    }
}
impl core::ops::Deref for ELLS_R {
    type Target = crate::FieldReader<bool, ELLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of LLS2 feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLS2_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELLS2_A> for bool {
    #[inline(always)]
    fn from(variant: ELLS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELLS2` reader - Existence of LLS2 feature"]
pub struct ELLS2_R(crate::FieldReader<bool, ELLS2_A>);
impl ELLS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELLS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELLS2_A {
        match self.bits {
            false => ELLS2_A::_0,
            true => ELLS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ELLS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ELLS2_A::_1
    }
}
impl core::ops::Deref for ELLS2_R {
    type Target = crate::FieldReader<bool, ELLS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of VLLS0 feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVLLS0_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EVLLS0_A> for bool {
    #[inline(always)]
    fn from(variant: EVLLS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVLLS0` reader - Existence of VLLS0 feature"]
pub struct EVLLS0_R(crate::FieldReader<bool, EVLLS0_A>);
impl EVLLS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVLLS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVLLS0_A {
        match self.bits {
            false => EVLLS0_A::_0,
            true => EVLLS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EVLLS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EVLLS0_A::_1
    }
}
impl core::ops::Deref for EVLLS0_R {
    type Target = crate::FieldReader<bool, EVLLS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Existence of HSRUN feature"]
    #[inline(always)]
    pub fn ehsrun(&self) -> EHSRUN_R {
        EHSRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Existence of LLS feature"]
    #[inline(always)]
    pub fn ells(&self) -> ELLS_R {
        ELLS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Existence of LLS2 feature"]
    #[inline(always)]
    pub fn ells2(&self) -> ELLS2_R {
        ELLS2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Existence of VLLS0 feature"]
    #[inline(always)]
    pub fn evlls0(&self) -> EVLLS0_R {
        EVLLS0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "SMC Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x01"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
