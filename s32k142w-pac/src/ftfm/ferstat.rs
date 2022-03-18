#[doc = "Register `FERSTAT` reader"]
pub struct R(crate::R<FERSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FERSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FERSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FERSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FERSTAT` writer"]
pub struct W(crate::W<FERSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERSTAT_SPEC>;
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
impl From<crate::W<FERSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FERSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Platform FlexRAM Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDFDIF_A {
    #[doc = "0: Fault not detected during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    PDFDIF_0 = 0,
    #[doc = "1: Fault detected (or FERCNFG\\[PFDFD\\]
is set) during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    PDFDIF_1 = 1,
}
impl From<PDFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: PDFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDFDIF` reader - Platform FlexRAM Double Bit Fault Detect Interrupt Flag"]
pub struct PDFDIF_R(crate::FieldReader<bool, PDFDIF_A>);
impl PDFDIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDFDIF_A {
        match self.bits {
            false => PDFDIF_A::PDFDIF_0,
            true => PDFDIF_A::PDFDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDFDIF_0`"]
    #[inline(always)]
    pub fn is_pdfdif_0(&self) -> bool {
        **self == PDFDIF_A::PDFDIF_0
    }
    #[doc = "Checks if the value of the field is `PDFDIF_1`"]
    #[inline(always)]
    pub fn is_pdfdif_1(&self) -> bool {
        **self == PDFDIF_A::PDFDIF_1
    }
}
impl core::ops::Deref for PDFDIF_R {
    type Target = crate::FieldReader<bool, PDFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDFDIF` writer - Platform FlexRAM Double Bit Fault Detect Interrupt Flag"]
pub struct PDFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault not detected during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    #[inline(always)]
    pub fn pdfdif_0(self) -> &'a mut W {
        self.variant(PDFDIF_A::PDFDIF_0)
    }
    #[doc = "Fault detected (or FERCNFG\\[PFDFD\\]
is set) during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    #[inline(always)]
    pub fn pdfdif_1(self) -> &'a mut W {
        self.variant(PDFDIF_A::PDFDIF_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Platform Flash Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIF_A {
    #[doc = "0: Fault not detected during a valid flash read access from the platform flash controller"]
    DFDIF_0 = 0,
    #[doc = "1: Fault detected (or FERCNFG\\[FDFD\\]
is set) during a valid flash read access from the platform flash controller"]
    DFDIF_1 = 1,
}
impl From<DFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIF` reader - Platform Flash Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_R(crate::FieldReader<bool, DFDIF_A>);
impl DFDIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIF_A {
        match self.bits {
            false => DFDIF_A::DFDIF_0,
            true => DFDIF_A::DFDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DFDIF_0`"]
    #[inline(always)]
    pub fn is_dfdif_0(&self) -> bool {
        **self == DFDIF_A::DFDIF_0
    }
    #[doc = "Checks if the value of the field is `DFDIF_1`"]
    #[inline(always)]
    pub fn is_dfdif_1(&self) -> bool {
        **self == DFDIF_A::DFDIF_1
    }
}
impl core::ops::Deref for DFDIF_R {
    type Target = crate::FieldReader<bool, DFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFDIF` writer - Platform Flash Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault not detected during a valid flash read access from the platform flash controller"]
    #[inline(always)]
    pub fn dfdif_0(self) -> &'a mut W {
        self.variant(DFDIF_A::DFDIF_0)
    }
    #[doc = "Fault detected (or FERCNFG\\[FDFD\\]
is set) during a valid flash read access from the platform flash controller"]
    #[inline(always)]
    pub fn dfdif_1(self) -> &'a mut W {
        self.variant(DFDIF_A::DFDIF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Controller FlexRAM Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFDIF_A {
    #[doc = "0: Fault not detected during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    EDFDIF_0 = 0,
    #[doc = "1: Fault detected (or FERCNFG\\[EFDFD\\]
is set) during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    EDFDIF_1 = 1,
}
impl From<EDFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: EDFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDFDIF` reader - Controller FlexRAM Double Bit Fault Detect Interrupt Flag"]
pub struct EDFDIF_R(crate::FieldReader<bool, EDFDIF_A>);
impl EDFDIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDFDIF_A {
        match self.bits {
            false => EDFDIF_A::EDFDIF_0,
            true => EDFDIF_A::EDFDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDFDIF_0`"]
    #[inline(always)]
    pub fn is_edfdif_0(&self) -> bool {
        **self == EDFDIF_A::EDFDIF_0
    }
    #[doc = "Checks if the value of the field is `EDFDIF_1`"]
    #[inline(always)]
    pub fn is_edfdif_1(&self) -> bool {
        **self == EDFDIF_A::EDFDIF_1
    }
}
impl core::ops::Deref for EDFDIF_R {
    type Target = crate::FieldReader<bool, EDFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDFDIF` writer - Controller FlexRAM Double Bit Fault Detect Interrupt Flag"]
pub struct EDFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault not detected during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    #[inline(always)]
    pub fn edfdif_0(self) -> &'a mut W {
        self.variant(EDFDIF_A::EDFDIF_0)
    }
    #[doc = "Fault detected (or FERCNFG\\[EFDFD\\]
is set) during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    #[inline(always)]
    pub fn edfdif_1(self) -> &'a mut W {
        self.variant(EDFDIF_A::EDFDIF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Controller Flash Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDFDIF_A {
    #[doc = "0: Fault not detected during a valid internal RAM or flash read access from the flash memory controller"]
    CDFDIF_0 = 0,
    #[doc = "1: Fault detected (or FERCNFG\\[CFDFD\\]
is set) during a valid internal RAM or flash read access from the flash memory controller"]
    CDFDIF_1 = 1,
}
impl From<CDFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: CDFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDFDIF` reader - Controller Flash Double Bit Fault Detect Interrupt Flag"]
pub struct CDFDIF_R(crate::FieldReader<bool, CDFDIF_A>);
impl CDFDIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDFDIF_A {
        match self.bits {
            false => CDFDIF_A::CDFDIF_0,
            true => CDFDIF_A::CDFDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDFDIF_0`"]
    #[inline(always)]
    pub fn is_cdfdif_0(&self) -> bool {
        **self == CDFDIF_A::CDFDIF_0
    }
    #[doc = "Checks if the value of the field is `CDFDIF_1`"]
    #[inline(always)]
    pub fn is_cdfdif_1(&self) -> bool {
        **self == CDFDIF_A::CDFDIF_1
    }
}
impl core::ops::Deref for CDFDIF_R {
    type Target = crate::FieldReader<bool, CDFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDFDIF` writer - Controller Flash Double Bit Fault Detect Interrupt Flag"]
pub struct CDFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault not detected during a valid internal RAM or flash read access from the flash memory controller"]
    #[inline(always)]
    pub fn cdfdif_0(self) -> &'a mut W {
        self.variant(CDFDIF_A::CDFDIF_0)
    }
    #[doc = "Fault detected (or FERCNFG\\[CFDFD\\]
is set) during a valid internal RAM or flash read access from the flash memory controller"]
    #[inline(always)]
    pub fn cdfdif_1(self) -> &'a mut W {
        self.variant(CDFDIF_A::CDFDIF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Platform FlexRAM Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn pdfdif(&self) -> PDFDIF_R {
        PDFDIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Platform Flash Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&self) -> DFDIF_R {
        DFDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controller FlexRAM Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn edfdif(&self) -> EDFDIF_R {
        EDFDIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controller Flash Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn cdfdif(&self) -> CDFDIF_R {
        CDFDIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Platform FlexRAM Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn pdfdif(&mut self) -> PDFDIF_W {
        PDFDIF_W { w: self }
    }
    #[doc = "Bit 1 - Platform Flash Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&mut self) -> DFDIF_W {
        DFDIF_W { w: self }
    }
    #[doc = "Bit 2 - Controller FlexRAM Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn edfdif(&mut self) -> EDFDIF_W {
        EDFDIF_W { w: self }
    }
    #[doc = "Bit 3 - Controller Flash Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn cdfdif(&mut self) -> CDFDIF_W {
        CDFDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ferstat](index.html) module"]
pub struct FERSTAT_SPEC;
impl crate::RegisterSpec for FERSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ferstat::R](R) reader structure"]
impl crate::Readable for FERSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ferstat::W](W) writer structure"]
impl crate::Writable for FERSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERSTAT to value 0"]
impl crate::Resettable for FERSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
