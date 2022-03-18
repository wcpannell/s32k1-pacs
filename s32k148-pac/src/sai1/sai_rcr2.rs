#[doc = "Register `SAI_RCR2` reader"]
pub struct R(crate::R<SAI_RCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_RCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_RCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_RCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_RCR2` writer"]
pub struct W(crate::W<SAI_RCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_RCR2_SPEC>;
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
impl From<crate::W<SAI_RCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_RCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Bit Clock Divide"]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Bit Clock Divide"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Bit Clock Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCD_A {
    #[doc = "0: Bit clock is generated externally in Slave mode."]
    _0 = 0,
    #[doc = "1: Bit clock is generated internally in Master mode."]
    _1 = 1,
}
impl From<BCD_A> for bool {
    #[inline(always)]
    fn from(variant: BCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCD` reader - Bit Clock Direction"]
pub struct BCD_R(crate::FieldReader<bool, BCD_A>);
impl BCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCD_A {
        match self.bits {
            false => BCD_A::_0,
            true => BCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCD_A::_1
    }
}
impl core::ops::Deref for BCD_R {
    type Target = crate::FieldReader<bool, BCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCD` writer - Bit Clock Direction"]
pub struct BCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCD_A::_0)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCP_A {
    #[doc = "0: Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    _0 = 0,
    #[doc = "1: Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    _1 = 1,
}
impl From<BCP_A> for bool {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCP` reader - Bit Clock Polarity"]
pub struct BCP_R(crate::FieldReader<bool, BCP_A>);
impl BCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCP_A {
        match self.bits {
            false => BCP_A::_0,
            true => BCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCP_A::_1
    }
}
impl core::ops::Deref for BCP_R {
    type Target = crate::FieldReader<bool, BCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCP` writer - Bit Clock Polarity"]
pub struct BCP_W<'a> {
    w: &'a mut W,
}
impl<'a> BCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCP_A::_0)
    }
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCP_A::_1)
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
#[doc = "MCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Bus Clock selected."]
    _00 = 0,
    #[doc = "1: Master Clock (MCLK) 1 option selected."]
    _01 = 1,
    #[doc = "2: Master Clock (MCLK) 2 option selected."]
    _10 = 2,
    #[doc = "3: Master Clock (MCLK) 3 option selected."]
    _11 = 3,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSEL` reader - MCLK Select"]
pub struct MSEL_R(crate::FieldReader<u8, MSEL_A>);
impl MSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::_00,
            1 => MSEL_A::_01,
            2 => MSEL_A::_10,
            3 => MSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == MSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == MSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == MSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == MSEL_A::_11
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, MSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEL` writer - MCLK Select"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bus Clock selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MSEL_A::_00)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MSEL_A::_01)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MSEL_A::_10)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Bit Clock Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCI_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Internal logic is clocked as if bit clock was externally generated."]
    _1 = 1,
}
impl From<BCI_A> for bool {
    #[inline(always)]
    fn from(variant: BCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCI` reader - Bit Clock Input"]
pub struct BCI_R(crate::FieldReader<bool, BCI_A>);
impl BCI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCI_A {
        match self.bits {
            false => BCI_A::_0,
            true => BCI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCI_A::_1
    }
}
impl core::ops::Deref for BCI_R {
    type Target = crate::FieldReader<bool, BCI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCI` writer - Bit Clock Input"]
pub struct BCI_W<'a> {
    w: &'a mut W,
}
impl<'a> BCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCI_A::_0)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCI_A::_1)
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
#[doc = "Bit Clock Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCS_A {
    #[doc = "0: Use the normal bit clock source."]
    _0 = 0,
    #[doc = "1: Swap the bit clock source."]
    _1 = 1,
}
impl From<BCS_A> for bool {
    #[inline(always)]
    fn from(variant: BCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCS` reader - Bit Clock Swap"]
pub struct BCS_R(crate::FieldReader<bool, BCS_A>);
impl BCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCS_A {
        match self.bits {
            false => BCS_A::_0,
            true => BCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCS_A::_1
    }
}
impl core::ops::Deref for BCS_R {
    type Target = crate::FieldReader<bool, BCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCS` writer - Bit Clock Swap"]
pub struct BCS_W<'a> {
    w: &'a mut W,
}
impl<'a> BCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use the normal bit clock source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCS_A::_0)
    }
    #[doc = "Swap the bit clock source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_A {
    #[doc = "0: Asynchronous mode."]
    _00 = 0,
    #[doc = "1: Synchronous with transmitter."]
    _01 = 1,
}
impl From<SYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode"]
pub struct SYNC_R(crate::FieldReader<u8, SYNC_A>);
impl SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_A> {
        match self.bits {
            0 => Some(SYNC_A::_00),
            1 => Some(SYNC_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == SYNC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == SYNC_A::_01
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<u8, SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SYNC_A::_00)
    }
    #[doc = "Synchronous with transmitter."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SYNC_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&self) -> BCI_R {
        BCI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&self) -> BCS_R {
        BCS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&mut self) -> BCD_W {
        BCD_W { w: self }
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&mut self) -> BCP_W {
        BCP_W { w: self }
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&mut self) -> BCI_W {
        BCI_W { w: self }
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&mut self) -> BCS_W {
        BCS_W { w: self }
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_rcr2](index.html) module"]
pub struct SAI_RCR2_SPEC;
impl crate::RegisterSpec for SAI_RCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_rcr2::R](R) reader structure"]
impl crate::Readable for SAI_RCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_rcr2::W](W) writer structure"]
impl crate::Writable for SAI_RCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_RCR2 to value 0"]
impl crate::Resettable for SAI_RCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
