#[doc = "Register `FCSESTAT0` reader"]
pub struct R(crate::R<FCSESTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSESTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSESTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSESTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPE_A {
    #[doc = "0: Flash command or no command"]
    CMDTYPE_0 = 0,
    #[doc = "1: CSEc command"]
    CMDTYPE_1 = 1,
}
impl From<CMDTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTYPE` reader - Command Type"]
pub struct CMDTYPE_R(crate::FieldReader<bool, CMDTYPE_A>);
impl CMDTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYPE_A {
        match self.bits {
            false => CMDTYPE_A::CMDTYPE_0,
            true => CMDTYPE_A::CMDTYPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMDTYPE_0`"]
    #[inline(always)]
    pub fn is_cmdtype_0(&self) -> bool {
        **self == CMDTYPE_A::CMDTYPE_0
    }
    #[doc = "Checks if the value of the field is `CMDTYPE_1`"]
    #[inline(always)]
    pub fn is_cmdtype_1(&self) -> bool {
        **self == CMDTYPE_A::CMDTYPE_1
    }
}
impl core::ops::Deref for CMDTYPE_R {
    type Target = crate::FieldReader<bool, CMDTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Memory Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMERR_A {
    #[doc = "0: Uncorrectable ECC fault not detected, CSE_PRAM access not blocked"]
    MEMERR_0 = 0,
    #[doc = "1: Uncorrectable ECC fault detected, CSE_PRAM access blocked"]
    MEMERR_1 = 1,
}
impl From<MEMERR_A> for bool {
    #[inline(always)]
    fn from(variant: MEMERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMERR` reader - Memory Error"]
pub struct MEMERR_R(crate::FieldReader<bool, MEMERR_A>);
impl MEMERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMERR_A {
        match self.bits {
            false => MEMERR_A::MEMERR_0,
            true => MEMERR_A::MEMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMERR_0`"]
    #[inline(always)]
    pub fn is_memerr_0(&self) -> bool {
        **self == MEMERR_A::MEMERR_0
    }
    #[doc = "Checks if the value of the field is `MEMERR_1`"]
    #[inline(always)]
    pub fn is_memerr_1(&self) -> bool {
        **self == MEMERR_A::MEMERR_1
    }
}
impl core::ops::Deref for MEMERR_R {
    type Target = crate::FieldReader<bool, MEMERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory Error"]
    #[inline(always)]
    pub fn memerr(&self) -> MEMERR_R {
        MEMERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Flash CSEc Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcsestat0](index.html) module"]
pub struct FCSESTAT0_SPEC;
impl crate::RegisterSpec for FCSESTAT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcsestat0::R](R) reader structure"]
impl crate::Readable for FCSESTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCSESTAT0 to value 0"]
impl crate::Resettable for FCSESTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
