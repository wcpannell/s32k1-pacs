#[doc = "Register `FERCNFG` reader"]
pub struct R(crate::R<FERCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FERCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FERCNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FERCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FERCNFG` writer"]
pub struct W(crate::W<FERCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERCNFG_SPEC>;
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
impl From<crate::W<FERCNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FERCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Platform FlexRAM Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDFDIE_A {
    #[doc = "0: Fault detect interrupt disabled"]
    PDFDIE_0 = 0,
    #[doc = "1: Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[PDFDIF\\]
flag is set."]
    PDFDIE_1 = 1,
}
impl From<PDFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: PDFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDFDIE` reader - Platform FlexRAM Double Bit Fault Detect Interrupt Enable"]
pub struct PDFDIE_R(crate::FieldReader<bool, PDFDIE_A>);
impl PDFDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDFDIE_A {
        match self.bits {
            false => PDFDIE_A::PDFDIE_0,
            true => PDFDIE_A::PDFDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDFDIE_0`"]
    #[inline(always)]
    pub fn is_pdfdie_0(&self) -> bool {
        **self == PDFDIE_A::PDFDIE_0
    }
    #[doc = "Checks if the value of the field is `PDFDIE_1`"]
    #[inline(always)]
    pub fn is_pdfdie_1(&self) -> bool {
        **self == PDFDIE_A::PDFDIE_1
    }
}
impl core::ops::Deref for PDFDIE_R {
    type Target = crate::FieldReader<bool, PDFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDFDIE` writer - Platform FlexRAM Double Bit Fault Detect Interrupt Enable"]
pub struct PDFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault detect interrupt disabled"]
    #[inline(always)]
    pub fn pdfdie_0(self) -> &'a mut W {
        self.variant(PDFDIE_A::PDFDIE_0)
    }
    #[doc = "Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[PDFDIF\\]
flag is set."]
    #[inline(always)]
    pub fn pdfdie_1(self) -> &'a mut W {
        self.variant(PDFDIE_A::PDFDIE_1)
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
#[doc = "Platform Flash Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIE_A {
    #[doc = "0: Fault detect interrupt disabled"]
    DFDIE_0 = 0,
    #[doc = "1: Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[DFDIF\\]
flag is set."]
    DFDIE_1 = 1,
}
impl From<DFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIE` reader - Platform Flash Double Bit Fault Detect Interrupt Enable"]
pub struct DFDIE_R(crate::FieldReader<bool, DFDIE_A>);
impl DFDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIE_A {
        match self.bits {
            false => DFDIE_A::DFDIE_0,
            true => DFDIE_A::DFDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DFDIE_0`"]
    #[inline(always)]
    pub fn is_dfdie_0(&self) -> bool {
        **self == DFDIE_A::DFDIE_0
    }
    #[doc = "Checks if the value of the field is `DFDIE_1`"]
    #[inline(always)]
    pub fn is_dfdie_1(&self) -> bool {
        **self == DFDIE_A::DFDIE_1
    }
}
impl core::ops::Deref for DFDIE_R {
    type Target = crate::FieldReader<bool, DFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFDIE` writer - Platform Flash Double Bit Fault Detect Interrupt Enable"]
pub struct DFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault detect interrupt disabled"]
    #[inline(always)]
    pub fn dfdie_0(self) -> &'a mut W {
        self.variant(DFDIE_A::DFDIE_0)
    }
    #[doc = "Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[DFDIF\\]
flag is set."]
    #[inline(always)]
    pub fn dfdie_1(self) -> &'a mut W {
        self.variant(DFDIE_A::DFDIE_1)
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
#[doc = "Controller FlexRAM Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFDIE_A {
    #[doc = "0: Fault detect interrupt disabled"]
    EDFDIE_0 = 0,
    #[doc = "1: Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[EDFDIF\\]
flag is set."]
    EDFDIE_1 = 1,
}
impl From<EDFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: EDFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDFDIE` reader - Controller FlexRAM Double Bit Fault Detect Interrupt Enable"]
pub struct EDFDIE_R(crate::FieldReader<bool, EDFDIE_A>);
impl EDFDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDFDIE_A {
        match self.bits {
            false => EDFDIE_A::EDFDIE_0,
            true => EDFDIE_A::EDFDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDFDIE_0`"]
    #[inline(always)]
    pub fn is_edfdie_0(&self) -> bool {
        **self == EDFDIE_A::EDFDIE_0
    }
    #[doc = "Checks if the value of the field is `EDFDIE_1`"]
    #[inline(always)]
    pub fn is_edfdie_1(&self) -> bool {
        **self == EDFDIE_A::EDFDIE_1
    }
}
impl core::ops::Deref for EDFDIE_R {
    type Target = crate::FieldReader<bool, EDFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDFDIE` writer - Controller FlexRAM Double Bit Fault Detect Interrupt Enable"]
pub struct EDFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault detect interrupt disabled"]
    #[inline(always)]
    pub fn edfdie_0(self) -> &'a mut W {
        self.variant(EDFDIE_A::EDFDIE_0)
    }
    #[doc = "Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[EDFDIF\\]
flag is set."]
    #[inline(always)]
    pub fn edfdie_1(self) -> &'a mut W {
        self.variant(EDFDIE_A::EDFDIE_1)
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
#[doc = "Controller Flash Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDFDIE_A {
    #[doc = "0: Fault detect interrupt disabled"]
    CDFDIE_0 = 0,
    #[doc = "1: Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[CDFDIF\\]
flag is set."]
    CDFDIE_1 = 1,
}
impl From<CDFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CDFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDFDIE` reader - Controller Flash Double Bit Fault Detect Interrupt Enable"]
pub struct CDFDIE_R(crate::FieldReader<bool, CDFDIE_A>);
impl CDFDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDFDIE_A {
        match self.bits {
            false => CDFDIE_A::CDFDIE_0,
            true => CDFDIE_A::CDFDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDFDIE_0`"]
    #[inline(always)]
    pub fn is_cdfdie_0(&self) -> bool {
        **self == CDFDIE_A::CDFDIE_0
    }
    #[doc = "Checks if the value of the field is `CDFDIE_1`"]
    #[inline(always)]
    pub fn is_cdfdie_1(&self) -> bool {
        **self == CDFDIE_A::CDFDIE_1
    }
}
impl core::ops::Deref for CDFDIE_R {
    type Target = crate::FieldReader<bool, CDFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDFDIE` writer - Controller Flash Double Bit Fault Detect Interrupt Enable"]
pub struct CDFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault detect interrupt disabled"]
    #[inline(always)]
    pub fn cdfdie_0(self) -> &'a mut W {
        self.variant(CDFDIE_A::CDFDIE_0)
    }
    #[doc = "Fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[CDFDIF\\]
flag is set."]
    #[inline(always)]
    pub fn cdfdie_1(self) -> &'a mut W {
        self.variant(CDFDIE_A::CDFDIE_1)
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
#[doc = "Platform FlexRAM Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFDFD_A {
    #[doc = "0: FERSTAT\\[PDFDIF\\]
sets only if a fault is detected during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    PFDFD_0 = 0,
    #[doc = "1: FERSTAT\\[PDFDIF\\]
sets during any valid FlexRAM or CSE_PRAM read access from the platform flash controller. An interrupt request is generated if the PDFDIE bit is set."]
    PFDFD_1 = 1,
}
impl From<PFDFD_A> for bool {
    #[inline(always)]
    fn from(variant: PFDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFDFD` reader - Platform FlexRAM Force Double Bit Fault Detect"]
pub struct PFDFD_R(crate::FieldReader<bool, PFDFD_A>);
impl PFDFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFDFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFDFD_A {
        match self.bits {
            false => PFDFD_A::PFDFD_0,
            true => PFDFD_A::PFDFD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFDFD_0`"]
    #[inline(always)]
    pub fn is_pfdfd_0(&self) -> bool {
        **self == PFDFD_A::PFDFD_0
    }
    #[doc = "Checks if the value of the field is `PFDFD_1`"]
    #[inline(always)]
    pub fn is_pfdfd_1(&self) -> bool {
        **self == PFDFD_A::PFDFD_1
    }
}
impl core::ops::Deref for PFDFD_R {
    type Target = crate::FieldReader<bool, PFDFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFDFD` writer - Platform FlexRAM Force Double Bit Fault Detect"]
pub struct PFDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PFDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFDFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FERSTAT\\[PDFDIF\\]
sets only if a fault is detected during a valid FlexRAM or CSE_PRAM read access from the platform flash controller"]
    #[inline(always)]
    pub fn pfdfd_0(self) -> &'a mut W {
        self.variant(PFDFD_A::PFDFD_0)
    }
    #[doc = "FERSTAT\\[PDFDIF\\]
sets during any valid FlexRAM or CSE_PRAM read access from the platform flash controller. An interrupt request is generated if the PDFDIE bit is set."]
    #[inline(always)]
    pub fn pfdfd_1(self) -> &'a mut W {
        self.variant(PFDFD_A::PFDFD_1)
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
#[doc = "Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDFD_A {
    #[doc = "0: FERSTAT\\[DFDIF\\]
sets only if a fault is detected during a valid flash read access from the platform flash controller"]
    FDFD_0 = 0,
    #[doc = "1: FERSTAT\\[DFDIF\\]
sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    FDFD_1 = 1,
}
impl From<FDFD_A> for bool {
    #[inline(always)]
    fn from(variant: FDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDFD` reader - Force Double Bit Fault Detect"]
pub struct FDFD_R(crate::FieldReader<bool, FDFD_A>);
impl FDFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDFD_A {
        match self.bits {
            false => FDFD_A::FDFD_0,
            true => FDFD_A::FDFD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FDFD_0`"]
    #[inline(always)]
    pub fn is_fdfd_0(&self) -> bool {
        **self == FDFD_A::FDFD_0
    }
    #[doc = "Checks if the value of the field is `FDFD_1`"]
    #[inline(always)]
    pub fn is_fdfd_1(&self) -> bool {
        **self == FDFD_A::FDFD_1
    }
}
impl core::ops::Deref for FDFD_R {
    type Target = crate::FieldReader<bool, FDFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDFD` writer - Force Double Bit Fault Detect"]
pub struct FDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FERSTAT\\[DFDIF\\]
sets only if a fault is detected during a valid flash read access from the platform flash controller"]
    #[inline(always)]
    pub fn fdfd_0(self) -> &'a mut W {
        self.variant(FDFD_A::FDFD_0)
    }
    #[doc = "FERSTAT\\[DFDIF\\]
sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    #[inline(always)]
    pub fn fdfd_1(self) -> &'a mut W {
        self.variant(FDFD_A::FDFD_1)
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
#[doc = "Controller FlexRAM Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFDFD_A {
    #[doc = "0: FERSTAT\\[EDFDIF\\]
sets only if a fault is detected during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    EFDFD_0 = 0,
    #[doc = "1: FERSTAT\\[EDFDIF\\]
sets during the next FlexRAM or CSE_PRAM read access from the flash memory controller. An interrupt request is generated if the EDFDIE bit is set."]
    EFDFD_1 = 1,
}
impl From<EFDFD_A> for bool {
    #[inline(always)]
    fn from(variant: EFDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFDFD` reader - Controller FlexRAM Force Double Bit Fault Detect"]
pub struct EFDFD_R(crate::FieldReader<bool, EFDFD_A>);
impl EFDFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFDFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFDFD_A {
        match self.bits {
            false => EFDFD_A::EFDFD_0,
            true => EFDFD_A::EFDFD_1,
        }
    }
    #[doc = "Checks if the value of the field is `EFDFD_0`"]
    #[inline(always)]
    pub fn is_efdfd_0(&self) -> bool {
        **self == EFDFD_A::EFDFD_0
    }
    #[doc = "Checks if the value of the field is `EFDFD_1`"]
    #[inline(always)]
    pub fn is_efdfd_1(&self) -> bool {
        **self == EFDFD_A::EFDFD_1
    }
}
impl core::ops::Deref for EFDFD_R {
    type Target = crate::FieldReader<bool, EFDFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFDFD` writer - Controller FlexRAM Force Double Bit Fault Detect"]
pub struct EFDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFDFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FERSTAT\\[EDFDIF\\]
sets only if a fault is detected during a valid FlexRAM or CSE_PRAM read access from the flash memory controller"]
    #[inline(always)]
    pub fn efdfd_0(self) -> &'a mut W {
        self.variant(EFDFD_A::EFDFD_0)
    }
    #[doc = "FERSTAT\\[EDFDIF\\]
sets during the next FlexRAM or CSE_PRAM read access from the flash memory controller. An interrupt request is generated if the EDFDIE bit is set."]
    #[inline(always)]
    pub fn efdfd_1(self) -> &'a mut W {
        self.variant(EFDFD_A::EFDFD_1)
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
#[doc = "Controller Flash Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFDFD_A {
    #[doc = "0: FERSTAT\\[CDFDIF\\]
sets only if a fault is detected during a valid internal RAM or flash read access from the flash memory controller"]
    CFDFD_0 = 0,
    #[doc = "1: FERSTAT\\[CDFDIF\\]
sets during the next internal RAM or flash read access from the flash memory controller. An interrupt request is generated if the CDFDIE bit is set."]
    CFDFD_1 = 1,
}
impl From<CFDFD_A> for bool {
    #[inline(always)]
    fn from(variant: CFDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFDFD` reader - Controller Flash Force Double Bit Fault Detect"]
pub struct CFDFD_R(crate::FieldReader<bool, CFDFD_A>);
impl CFDFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDFD_A {
        match self.bits {
            false => CFDFD_A::CFDFD_0,
            true => CFDFD_A::CFDFD_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFDFD_0`"]
    #[inline(always)]
    pub fn is_cfdfd_0(&self) -> bool {
        **self == CFDFD_A::CFDFD_0
    }
    #[doc = "Checks if the value of the field is `CFDFD_1`"]
    #[inline(always)]
    pub fn is_cfdfd_1(&self) -> bool {
        **self == CFDFD_A::CFDFD_1
    }
}
impl core::ops::Deref for CFDFD_R {
    type Target = crate::FieldReader<bool, CFDFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDFD` writer - Controller Flash Force Double Bit Fault Detect"]
pub struct CFDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFDFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FERSTAT\\[CDFDIF\\]
sets only if a fault is detected during a valid internal RAM or flash read access from the flash memory controller"]
    #[inline(always)]
    pub fn cfdfd_0(self) -> &'a mut W {
        self.variant(CFDFD_A::CFDFD_0)
    }
    #[doc = "FERSTAT\\[CDFDIF\\]
sets during the next internal RAM or flash read access from the flash memory controller. An interrupt request is generated if the CDFDIE bit is set."]
    #[inline(always)]
    pub fn cfdfd_1(self) -> &'a mut W {
        self.variant(CFDFD_A::CFDFD_1)
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
    #[doc = "Bit 0 - Platform FlexRAM Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn pdfdie(&self) -> PDFDIE_R {
        PDFDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Platform Flash Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&self) -> DFDIE_R {
        DFDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controller FlexRAM Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn edfdie(&self) -> EDFDIE_R {
        EDFDIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controller Flash Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn cdfdie(&self) -> CDFDIE_R {
        CDFDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Platform FlexRAM Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn pfdfd(&self) -> PFDFD_R {
        PFDFD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&self) -> FDFD_R {
        FDFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controller FlexRAM Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn efdfd(&self) -> EFDFD_R {
        EFDFD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controller Flash Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn cfdfd(&self) -> CFDFD_R {
        CFDFD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Platform FlexRAM Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn pdfdie(&mut self) -> PDFDIE_W {
        PDFDIE_W { w: self }
    }
    #[doc = "Bit 1 - Platform Flash Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&mut self) -> DFDIE_W {
        DFDIE_W { w: self }
    }
    #[doc = "Bit 2 - Controller FlexRAM Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn edfdie(&mut self) -> EDFDIE_W {
        EDFDIE_W { w: self }
    }
    #[doc = "Bit 3 - Controller Flash Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn cdfdie(&mut self) -> CDFDIE_W {
        CDFDIE_W { w: self }
    }
    #[doc = "Bit 4 - Platform FlexRAM Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn pfdfd(&mut self) -> PFDFD_W {
        PFDFD_W { w: self }
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&mut self) -> FDFD_W {
        FDFD_W { w: self }
    }
    #[doc = "Bit 6 - Controller FlexRAM Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn efdfd(&mut self) -> EFDFD_W {
        EFDFD_W { w: self }
    }
    #[doc = "Bit 7 - Controller Flash Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn cfdfd(&mut self) -> CFDFD_W {
        CFDFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fercnfg](index.html) module"]
pub struct FERCNFG_SPEC;
impl crate::RegisterSpec for FERCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fercnfg::R](R) reader structure"]
impl crate::Readable for FERCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fercnfg::W](W) writer structure"]
impl crate::Writable for FERCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERCNFG to value 0x0d"]
impl crate::Resettable for FERCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
