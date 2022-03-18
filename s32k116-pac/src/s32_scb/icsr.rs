#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub struct VECTACTIVE_R(crate::FieldReader<u8, u8>);
impl VECTACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VECTACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTACTIVE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub struct VECTPENDING_R(crate::FieldReader<u8, u8>);
impl VECTPENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VECTPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTPENDING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt pending flag, excluding NMI and Faults\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDING_A {
    #[doc = "0: interrupt not pending"]
    _0 = 0,
    #[doc = "1: interrupt pending"]
    _1 = 1,
}
impl From<ISRPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag, excluding NMI and Faults"]
pub struct ISRPENDING_R(crate::FieldReader<bool, ISRPENDING_A>);
impl ISRPENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISRPENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPENDING_A {
        match self.bits {
            false => ISRPENDING_A::_0,
            true => ISRPENDING_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISRPENDING_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISRPENDING_A::_1
    }
}
impl core::ops::Deref for ISRPENDING_R {
    type Target = crate::FieldReader<bool, ISRPENDING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SysTick exception clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: removes the pending state from the SysTick exception"]
    _1 = 1,
}
impl From<PENDSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick exception clear-pending bit"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "SysTick exception set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: write: no effect; read: SysTick exception is not pending"]
    _0 = 0,
    #[doc = "1: write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    _1 = 1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit"]
pub struct PENDSTSET_R(crate::FieldReader<bool, PENDSTSET_A>);
impl PENDSTSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::_0,
            true => PENDSTSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSTSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSTSET_A::_1
    }
}
impl core::ops::Deref for PENDSTSET_R {
    type Target = crate::FieldReader<bool, PENDSTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: removes the pending state from the PendSV exception"]
    _1 = 1,
}
impl From<PENDSVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "PendSV set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: write: no effect; read: PendSV exception is not pending"]
    _0 = 0,
    #[doc = "1: write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    _1 = 1,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit"]
pub struct PENDSVSET_R(crate::FieldReader<bool, PENDSVSET_A>);
impl PENDSVSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::_0,
            true => PENDSVSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSVSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSVSET_A::_1
    }
}
impl core::ops::Deref for PENDSVSET_R {
    type Target = crate::FieldReader<bool, PENDSVSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "NMI set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "0: write: no effect; read: NMI exception is not pending"]
    _0 = 0,
    #[doc = "1: write: changes NMI exception state to pending; read: NMI exception is pending"]
    _1 = 1,
}
impl From<NMIPENDSET_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit"]
pub struct NMIPENDSET_R(crate::FieldReader<bool, NMIPENDSET_A>);
impl NMIPENDSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIPENDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::_0,
            true => NMIPENDSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NMIPENDSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NMIPENDSET_A::_1
    }
}
impl core::ops::Deref for NMIPENDSET_R {
    type Target = crate::FieldReader<bool, NMIPENDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag, excluding NMI and Faults"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
