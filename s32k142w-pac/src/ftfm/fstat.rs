#[doc = "Register `FSTAT` reader"]
pub struct R(crate::R<FSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSTAT` writer"]
pub struct W(crate::W<FSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSTAT_SPEC>;
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
impl From<crate::W<FSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MGSTAT0` reader - Memory Controller Status Flag 0"]
pub struct MGSTAT0_R(crate::FieldReader<bool, bool>);
impl MGSTAT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGSTAT1` reader - Memory Controller Status Flag 1"]
pub struct MGSTAT1_R(crate::FieldReader<bool, bool>);
impl MGSTAT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGSTAT2` reader - Memory Controller Status Flag 2"]
pub struct MGSTAT2_R(crate::FieldReader<bool, bool>);
impl MGSTAT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGSTAT3` reader - Memory Controller Status Flag 3"]
pub struct MGSTAT3_R(crate::FieldReader<bool, bool>);
impl MGSTAT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Protection Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPVIOL_A {
    #[doc = "0: No protection violation detected"]
    FPVIOL_0 = 0,
    #[doc = "1: Protection violation detected"]
    FPVIOL_1 = 1,
}
impl From<FPVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: FPVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPVIOL` reader - Flash Protection Violation Flag"]
pub struct FPVIOL_R(crate::FieldReader<bool, FPVIOL_A>);
impl FPVIOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVIOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPVIOL_A {
        match self.bits {
            false => FPVIOL_A::FPVIOL_0,
            true => FPVIOL_A::FPVIOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPVIOL_0`"]
    #[inline(always)]
    pub fn is_fpviol_0(&self) -> bool {
        **self == FPVIOL_A::FPVIOL_0
    }
    #[doc = "Checks if the value of the field is `FPVIOL_1`"]
    #[inline(always)]
    pub fn is_fpviol_1(&self) -> bool {
        **self == FPVIOL_A::FPVIOL_1
    }
}
impl core::ops::Deref for FPVIOL_R {
    type Target = crate::FieldReader<bool, FPVIOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVIOL` writer - Flash Protection Violation Flag"]
pub struct FPVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPVIOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn fpviol_0(self) -> &'a mut W {
        self.variant(FPVIOL_A::FPVIOL_0)
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn fpviol_1(self) -> &'a mut W {
        self.variant(FPVIOL_A::FPVIOL_1)
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
#[doc = "Flash Access Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCERR_A {
    #[doc = "0: No access error detected"]
    ACCERR_0 = 0,
    #[doc = "1: Access error detected"]
    ACCERR_1 = 1,
}
impl From<ACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCERR` reader - Flash Access Error Flag"]
pub struct ACCERR_R(crate::FieldReader<bool, ACCERR_A>);
impl ACCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCERR_A {
        match self.bits {
            false => ACCERR_A::ACCERR_0,
            true => ACCERR_A::ACCERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACCERR_0`"]
    #[inline(always)]
    pub fn is_accerr_0(&self) -> bool {
        **self == ACCERR_A::ACCERR_0
    }
    #[doc = "Checks if the value of the field is `ACCERR_1`"]
    #[inline(always)]
    pub fn is_accerr_1(&self) -> bool {
        **self == ACCERR_A::ACCERR_1
    }
}
impl core::ops::Deref for ACCERR_R {
    type Target = crate::FieldReader<bool, ACCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCERR` writer - Flash Access Error Flag"]
pub struct ACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn accerr_0(self) -> &'a mut W {
        self.variant(ACCERR_A::ACCERR_0)
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn accerr_1(self) -> &'a mut W {
        self.variant(ACCERR_A::ACCERR_1)
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
#[doc = "FTFM Read Collision Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLERR_A {
    #[doc = "0: No collision error detected"]
    RDCOLERR_0 = 0,
    #[doc = "1: Collision error detected"]
    RDCOLERR_1 = 1,
}
impl From<RDCOLERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCOLERR` reader - FTFM Read Collision Error Flag"]
pub struct RDCOLERR_R(crate::FieldReader<bool, RDCOLERR_A>);
impl RDCOLERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDCOLERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLERR_A {
        match self.bits {
            false => RDCOLERR_A::RDCOLERR_0,
            true => RDCOLERR_A::RDCOLERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDCOLERR_0`"]
    #[inline(always)]
    pub fn is_rdcolerr_0(&self) -> bool {
        **self == RDCOLERR_A::RDCOLERR_0
    }
    #[doc = "Checks if the value of the field is `RDCOLERR_1`"]
    #[inline(always)]
    pub fn is_rdcolerr_1(&self) -> bool {
        **self == RDCOLERR_A::RDCOLERR_1
    }
}
impl core::ops::Deref for RDCOLERR_R {
    type Target = crate::FieldReader<bool, RDCOLERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDCOLERR` writer - FTFM Read Collision Error Flag"]
pub struct RDCOLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCOLERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDCOLERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No collision error detected"]
    #[inline(always)]
    pub fn rdcolerr_0(self) -> &'a mut W {
        self.variant(RDCOLERR_A::RDCOLERR_0)
    }
    #[doc = "Collision error detected"]
    #[inline(always)]
    pub fn rdcolerr_1(self) -> &'a mut W {
        self.variant(RDCOLERR_A::RDCOLERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Command Complete Interrupt Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIF_A {
    #[doc = "0: FTFM command or emulated EEPROM file system operation in progress"]
    CCIF_0 = 0,
    #[doc = "1: FTFM command or emulated EEPROM file system operation has completed"]
    CCIF_1 = 1,
}
impl From<CCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIF` reader - Command Complete Interrupt Flag"]
pub struct CCIF_R(crate::FieldReader<bool, CCIF_A>);
impl CCIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIF_A {
        match self.bits {
            false => CCIF_A::CCIF_0,
            true => CCIF_A::CCIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIF_0`"]
    #[inline(always)]
    pub fn is_ccif_0(&self) -> bool {
        **self == CCIF_A::CCIF_0
    }
    #[doc = "Checks if the value of the field is `CCIF_1`"]
    #[inline(always)]
    pub fn is_ccif_1(&self) -> bool {
        **self == CCIF_A::CCIF_1
    }
}
impl core::ops::Deref for CCIF_R {
    type Target = crate::FieldReader<bool, CCIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCIF` writer - Command Complete Interrupt Flag"]
pub struct CCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTFM command or emulated EEPROM file system operation in progress"]
    #[inline(always)]
    pub fn ccif_0(self) -> &'a mut W {
        self.variant(CCIF_A::CCIF_0)
    }
    #[doc = "FTFM command or emulated EEPROM file system operation has completed"]
    #[inline(always)]
    pub fn ccif_1(self) -> &'a mut W {
        self.variant(CCIF_A::CCIF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Memory Controller Status Flag 0"]
    #[inline(always)]
    pub fn mgstat0(&self) -> MGSTAT0_R {
        MGSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Memory Controller Status Flag 1"]
    #[inline(always)]
    pub fn mgstat1(&self) -> MGSTAT1_R {
        MGSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory Controller Status Flag 2"]
    #[inline(always)]
    pub fn mgstat2(&self) -> MGSTAT2_R {
        MGSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory Controller Status Flag 3"]
    #[inline(always)]
    pub fn mgstat3(&self) -> MGSTAT3_R {
        MGSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    pub fn fpviol(&self) -> FPVIOL_R {
        FPVIOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&self) -> ACCERR_R {
        ACCERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FTFM Read Collision Error Flag"]
    #[inline(always)]
    pub fn rdcolerr(&self) -> RDCOLERR_R {
        RDCOLERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    pub fn fpviol(&mut self) -> FPVIOL_W {
        FPVIOL_W { w: self }
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&mut self) -> ACCERR_W {
        ACCERR_W { w: self }
    }
    #[doc = "Bit 6 - FTFM Read Collision Error Flag"]
    #[inline(always)]
    pub fn rdcolerr(&mut self) -> RDCOLERR_W {
        RDCOLERR_W { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&mut self) -> CCIF_W {
        CCIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](index.html) module"]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fstat::R](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fstat::W](W) writer structure"]
impl crate::Writable for FSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSTAT to value 0x80"]
impl crate::Resettable for FSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
