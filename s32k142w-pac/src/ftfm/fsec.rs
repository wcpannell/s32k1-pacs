#[doc = "Register `FSEC` reader"]
pub struct R(crate::R<FSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Flash Security\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "0: MCU security status is secure"]
    SEC_0 = 0,
    #[doc = "1: MCU security status is secure"]
    SEC_1 = 1,
    #[doc = "2: MCU security status is unsecure (The standard shipping condition of the FTFM is unsecure.)"]
    SEC_2 = 2,
    #[doc = "3: MCU security status is secure"]
    SEC_3 = 3,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC` reader - Flash Security"]
pub struct SEC_R(crate::FieldReader<u8, SEC_A>);
impl SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            0 => SEC_A::SEC_0,
            1 => SEC_A::SEC_1,
            2 => SEC_A::SEC_2,
            3 => SEC_A::SEC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEC_0`"]
    #[inline(always)]
    pub fn is_sec_0(&self) -> bool {
        **self == SEC_A::SEC_0
    }
    #[doc = "Checks if the value of the field is `SEC_1`"]
    #[inline(always)]
    pub fn is_sec_1(&self) -> bool {
        **self == SEC_A::SEC_1
    }
    #[doc = "Checks if the value of the field is `SEC_2`"]
    #[inline(always)]
    pub fn is_sec_2(&self) -> bool {
        **self == SEC_A::SEC_2
    }
    #[doc = "Checks if the value of the field is `SEC_3`"]
    #[inline(always)]
    pub fn is_sec_3(&self) -> bool {
        **self == SEC_A::SEC_3
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, SEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Factory Failure Analysis Access Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSLACC_A {
    #[doc = "0: Factory access granted"]
    FSLACC_0 = 0,
    #[doc = "1: Factory access denied"]
    FSLACC_1 = 1,
    #[doc = "2: Factory access denied"]
    FSLACC_2 = 2,
    #[doc = "3: Factory access granted"]
    FSLACC_3 = 3,
}
impl From<FSLACC_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLACC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSLACC` reader - Factory Failure Analysis Access Code"]
pub struct FSLACC_R(crate::FieldReader<u8, FSLACC_A>);
impl FSLACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSLACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSLACC_A {
        match self.bits {
            0 => FSLACC_A::FSLACC_0,
            1 => FSLACC_A::FSLACC_1,
            2 => FSLACC_A::FSLACC_2,
            3 => FSLACC_A::FSLACC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FSLACC_0`"]
    #[inline(always)]
    pub fn is_fslacc_0(&self) -> bool {
        **self == FSLACC_A::FSLACC_0
    }
    #[doc = "Checks if the value of the field is `FSLACC_1`"]
    #[inline(always)]
    pub fn is_fslacc_1(&self) -> bool {
        **self == FSLACC_A::FSLACC_1
    }
    #[doc = "Checks if the value of the field is `FSLACC_2`"]
    #[inline(always)]
    pub fn is_fslacc_2(&self) -> bool {
        **self == FSLACC_A::FSLACC_2
    }
    #[doc = "Checks if the value of the field is `FSLACC_3`"]
    #[inline(always)]
    pub fn is_fslacc_3(&self) -> bool {
        **self == FSLACC_A::FSLACC_3
    }
}
impl core::ops::Deref for FSLACC_R {
    type Target = crate::FieldReader<u8, FSLACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mass Erase Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEEN_A {
    #[doc = "0: Mass erase is enabled"]
    MEEN_0 = 0,
    #[doc = "1: Mass erase is enabled"]
    MEEN_1 = 1,
    #[doc = "2: Mass erase is disabled"]
    MEEN_2 = 2,
    #[doc = "3: Mass erase is enabled"]
    MEEN_3 = 3,
}
impl From<MEEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MEEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MEEN` reader - Mass Erase Enable Bits"]
pub struct MEEN_R(crate::FieldReader<u8, MEEN_A>);
impl MEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEEN_A {
        match self.bits {
            0 => MEEN_A::MEEN_0,
            1 => MEEN_A::MEEN_1,
            2 => MEEN_A::MEEN_2,
            3 => MEEN_A::MEEN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEEN_0`"]
    #[inline(always)]
    pub fn is_meen_0(&self) -> bool {
        **self == MEEN_A::MEEN_0
    }
    #[doc = "Checks if the value of the field is `MEEN_1`"]
    #[inline(always)]
    pub fn is_meen_1(&self) -> bool {
        **self == MEEN_A::MEEN_1
    }
    #[doc = "Checks if the value of the field is `MEEN_2`"]
    #[inline(always)]
    pub fn is_meen_2(&self) -> bool {
        **self == MEEN_A::MEEN_2
    }
    #[doc = "Checks if the value of the field is `MEEN_3`"]
    #[inline(always)]
    pub fn is_meen_3(&self) -> bool {
        **self == MEEN_A::MEEN_3
    }
}
impl core::ops::Deref for MEEN_R {
    type Target = crate::FieldReader<u8, MEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Backdoor Key Security Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYEN_A {
    #[doc = "0: Backdoor key access disabled"]
    KEYEN_0 = 0,
    #[doc = "1: Backdoor key access disabled (preferred KEYEN state to disable backdoor key access)"]
    KEYEN_1 = 1,
    #[doc = "2: Backdoor key access enabled"]
    KEYEN_2 = 2,
    #[doc = "3: Backdoor key access disabled"]
    KEYEN_3 = 3,
}
impl From<KEYEN_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable"]
pub struct KEYEN_R(crate::FieldReader<u8, KEYEN_A>);
impl KEYEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYEN_A {
        match self.bits {
            0 => KEYEN_A::KEYEN_0,
            1 => KEYEN_A::KEYEN_1,
            2 => KEYEN_A::KEYEN_2,
            3 => KEYEN_A::KEYEN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `KEYEN_0`"]
    #[inline(always)]
    pub fn is_keyen_0(&self) -> bool {
        **self == KEYEN_A::KEYEN_0
    }
    #[doc = "Checks if the value of the field is `KEYEN_1`"]
    #[inline(always)]
    pub fn is_keyen_1(&self) -> bool {
        **self == KEYEN_A::KEYEN_1
    }
    #[doc = "Checks if the value of the field is `KEYEN_2`"]
    #[inline(always)]
    pub fn is_keyen_2(&self) -> bool {
        **self == KEYEN_A::KEYEN_2
    }
    #[doc = "Checks if the value of the field is `KEYEN_3`"]
    #[inline(always)]
    pub fn is_keyen_3(&self) -> bool {
        **self == KEYEN_A::KEYEN_3
    }
}
impl core::ops::Deref for KEYEN_R {
    type Target = crate::FieldReader<u8, KEYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Factory Failure Analysis Access Code"]
    #[inline(always)]
    pub fn fslacc(&self) -> FSLACC_R {
        FSLACC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Mass Erase Enable Bits"]
    #[inline(always)]
    pub fn meen(&self) -> MEEN_R {
        MEEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](index.html) module"]
pub struct FSEC_SPEC;
impl crate::RegisterSpec for FSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fsec::R](R) reader structure"]
impl crate::Readable for FSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSEC to value 0"]
impl crate::Resettable for FSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
