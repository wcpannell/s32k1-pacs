#[doc = "Register `FCNFG` reader"]
pub struct R(crate::R<FCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCNFG` writer"]
pub struct W(crate::W<FCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCNFG_SPEC>;
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
impl From<crate::W<FCNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEERDY` reader - EEERDY"]
pub struct EEERDY_R(crate::FieldReader<bool, bool>);
impl EEERDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMRDY` reader - RAM Ready"]
pub struct RAMRDY_R(crate::FieldReader<bool, bool>);
impl RAMRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSP_A {
    #[doc = "0: No suspend requested"]
    _0 = 0,
    #[doc = "1: Suspend the current Erase Flash Sector command execution"]
    _1 = 1,
}
impl From<ERSSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: ERSSUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSSUSP` reader - Erase Suspend"]
pub struct ERSSUSP_R(crate::FieldReader<bool, ERSSUSP_A>);
impl ERSSUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERSSUSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSSUSP_A {
        match self.bits {
            false => ERSSUSP_A::_0,
            true => ERSSUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERSSUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERSSUSP_A::_1
    }
}
impl core::ops::Deref for ERSSUSP_R {
    type Target = crate::FieldReader<bool, ERSSUSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERSSUSP` writer - Erase Suspend"]
pub struct ERSSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERSSUSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_1)
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
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQ_A {
    #[doc = "0: No request or request complete"]
    _0 = 0,
}
impl From<ERSAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERSAREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSAREQ` reader - Erase All Request"]
pub struct ERSAREQ_R(crate::FieldReader<bool, ERSAREQ_A>);
impl ERSAREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERSAREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERSAREQ_A> {
        match self.bits {
            false => Some(ERSAREQ_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERSAREQ_A::_0
    }
}
impl core::ops::Deref for ERSAREQ_R {
    type Target = crate::FieldReader<bool, ERSAREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIE_A {
    #[doc = "0: Read collision error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1 = 1,
}
impl From<RDCOLLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCOLLIE` reader - Read Collision Error Interrupt Enable"]
pub struct RDCOLLIE_R(crate::FieldReader<bool, RDCOLLIE_A>);
impl RDCOLLIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDCOLLIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLLIE_A {
        match self.bits {
            false => RDCOLLIE_A::_0,
            true => RDCOLLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDCOLLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDCOLLIE_A::_1
    }
}
impl core::ops::Deref for RDCOLLIE_R {
    type Target = crate::FieldReader<bool, RDCOLLIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDCOLLIE` writer - Read Collision Error Interrupt Enable"]
pub struct RDCOLLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCOLLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDCOLLIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_1)
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
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt disabled"]
    _0 = 0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    _1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub struct CCIE_R(crate::FieldReader<bool, CCIE_A>);
impl CCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CCIE_A::_1
    }
}
impl core::ops::Deref for CCIE_R {
    type Target = crate::FieldReader<bool, CCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCIE` writer - Command Complete Interrupt Enable"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
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
    #[doc = "Bit 0 - EEERDY"]
    #[inline(always)]
    pub fn eeerdy(&self) -> EEERDY_R {
        EEERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline(always)]
    pub fn ramrdy(&self) -> RAMRDY_R {
        RAMRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&self) -> ERSSUSP_R {
        ERSSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ERSAREQ_R {
        ERSAREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&self) -> RDCOLLIE_R {
        RDCOLLIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&mut self) -> ERSSUSP_W {
        ERSSUSP_W { w: self }
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&mut self) -> RDCOLLIE_W {
        RDCOLLIE_W { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](index.html) module"]
pub struct FCNFG_SPEC;
impl crate::RegisterSpec for FCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcnfg::R](R) reader structure"]
impl crate::Readable for FCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](W) writer structure"]
impl crate::Writable for FCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCNFG to value 0x02"]
impl crate::Resettable for FCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
