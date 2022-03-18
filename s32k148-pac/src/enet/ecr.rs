#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - Ethernet MAC Reset"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - Ethernet MAC Reset"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Ethernet Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHEREN_A {
    #[doc = "0: Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    _0 = 0,
    #[doc = "1: MAC is enabled, and reception and transmission are possible."]
    _1 = 1,
}
impl From<ETHEREN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHEREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETHEREN` reader - Ethernet Enable"]
pub struct ETHEREN_R(crate::FieldReader<bool, ETHEREN_A>);
impl ETHEREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHEREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHEREN_A {
        match self.bits {
            false => ETHEREN_A::_0,
            true => ETHEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ETHEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ETHEREN_A::_1
    }
}
impl core::ops::Deref for ETHEREN_R {
    type Target = crate::FieldReader<bool, ETHEREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHEREN` writer - Ethernet Enable"]
pub struct ETHEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHEREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETHEREN_A::_0)
    }
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETHEREN_A::_1)
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
#[doc = "Magic Packet Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAGICEN_A {
    #[doc = "0: Magic detection logic disabled."]
    _0 = 0,
    #[doc = "1: The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    _1 = 1,
}
impl From<MAGICEN_A> for bool {
    #[inline(always)]
    fn from(variant: MAGICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAGICEN` reader - Magic Packet Detection Enable"]
pub struct MAGICEN_R(crate::FieldReader<bool, MAGICEN_A>);
impl MAGICEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAGICEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAGICEN_A {
        match self.bits {
            false => MAGICEN_A::_0,
            true => MAGICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MAGICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MAGICEN_A::_1
    }
}
impl core::ops::Deref for MAGICEN_R {
    type Target = crate::FieldReader<bool, MAGICEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAGICEN` writer - Magic Packet Detection Enable"]
pub struct MAGICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAGICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAGICEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Magic detection logic disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAGICEN_A::_0)
    }
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAGICEN_A::_1)
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
#[doc = "Sleep Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal operating mode."]
    _0 = 0,
    #[doc = "1: Sleep mode."]
    _1 = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode Enable"]
pub struct SLEEP_R(crate::FieldReader<bool, SLEEP_A>);
impl SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::_0,
            true => SLEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLEEP_A::_1
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, SLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode Enable"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operating mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEP_A::_0)
    }
    #[doc = "Sleep mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEP_A::_1)
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
#[doc = "EN1588 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1588_A {
    #[doc = "0: Legacy FEC buffer descriptors and functions enabled."]
    _0 = 0,
    #[doc = "1: Enhanced frame time-stamping functions enabled."]
    _1 = 1,
}
impl From<EN1588_A> for bool {
    #[inline(always)]
    fn from(variant: EN1588_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1588` reader - EN1588 Enable"]
pub struct EN1588_R(crate::FieldReader<bool, EN1588_A>);
impl EN1588_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN1588_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1588_A {
        match self.bits {
            false => EN1588_A::_0,
            true => EN1588_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN1588_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN1588_A::_1
    }
}
impl core::ops::Deref for EN1588_R {
    type Target = crate::FieldReader<bool, EN1588_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1588` writer - EN1588 Enable"]
pub struct EN1588_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1588_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1588_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1588_A::_0)
    }
    #[doc = "Enhanced frame time-stamping functions enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1588_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: MAC continues operation in debug mode."]
    _0 = 0,
    #[doc = "1: MAC enters hardware freeze mode when the processor is in debug mode."]
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Debug Enable"]
pub struct DBGEN_R(crate::FieldReader<bool, DBGEN_A>);
impl DBGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGEN_A::_1
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, DBGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGEN` writer - Debug Enable"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC continues operation in debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEN_A::_1)
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
#[doc = "Descriptor Byte Swapping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBSWP_A {
    #[doc = "0: The buffer descriptor bytes are not swapped to support big-endian devices."]
    _0 = 0,
    #[doc = "1: The buffer descriptor bytes are swapped to support little-endian devices."]
    _1 = 1,
}
impl From<DBSWP_A> for bool {
    #[inline(always)]
    fn from(variant: DBSWP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBSWP` reader - Descriptor Byte Swapping Enable"]
pub struct DBSWP_R(crate::FieldReader<bool, DBSWP_A>);
impl DBSWP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBSWP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBSWP_A {
        match self.bits {
            false => DBSWP_A::_0,
            true => DBSWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBSWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBSWP_A::_1
    }
}
impl core::ops::Deref for DBSWP_R {
    type Target = crate::FieldReader<bool, DBSWP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBSWP` writer - Descriptor Byte Swapping Enable"]
pub struct DBSWP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBSWP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBSWP_A::_0)
    }
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBSWP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&self) -> ETHEREN_R {
        ETHEREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&self) -> MAGICEN_R {
        MAGICEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&self) -> EN1588_R {
        EN1588_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&self) -> DBSWP_R {
        DBSWP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&mut self) -> ETHEREN_W {
        ETHEREN_W { w: self }
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&mut self) -> MAGICEN_W {
        MAGICEN_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&mut self) -> EN1588_W {
        EN1588_W { w: self }
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&mut self) -> DBSWP_W {
        DBSWP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECR to value 0xf000_0000"]
impl crate::Resettable for ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
