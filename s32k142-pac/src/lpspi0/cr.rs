#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEN_A {
    #[doc = "0: Module is disabled."]
    _0 = 0,
    #[doc = "1: Module is enabled."]
    _1 = 1,
}
impl From<MEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEN` reader - Module Enable"]
pub struct MEN_R(crate::FieldReader<bool, MEN_A>);
impl MEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEN_A {
        match self.bits {
            false => MEN_A::_0,
            true => MEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MEN_A::_1
    }
}
impl core::ops::Deref for MEN_R {
    type Target = crate::FieldReader<bool, MEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEN` writer - Module Enable"]
pub struct MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEN_A::_0)
    }
    #[doc = "Module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Master logic is not reset."]
    _0 = 0,
    #[doc = "1: Master logic is reset."]
    _1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::_0,
            true => RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RST_A::_1
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master logic is not reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RST_A::_0)
    }
    #[doc = "Master logic is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RST_A::_1)
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
#[doc = "Doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
    #[doc = "0: Module is enabled in Doze mode."]
    _0 = 0,
    #[doc = "1: Module is disabled in Doze mode."]
    _1 = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze mode enable"]
pub struct DOZEN_R(crate::FieldReader<bool, DOZEN_A>);
impl DOZEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::_0,
            true => DOZEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOZEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOZEN_A::_1
    }
}
impl core::ops::Deref for DOZEN_R {
    type Target = crate::FieldReader<bool, DOZEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZEN` writer - Doze mode enable"]
pub struct DOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Module is enabled in Doze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEN_A::_0)
    }
    #[doc = "Module is disabled in Doze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEN_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: Module is disabled in debug mode."]
    _0 = 0,
    #[doc = "1: Module is enabled in debug mode."]
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
    #[doc = "Module is disabled in debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "Module is enabled in debug mode."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTF_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Transmit FIFO is reset."]
    _1 = 1,
}
impl From<RTF_AW> for bool {
    #[inline(always)]
    fn from(variant: RTF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTF` writer - Reset Transmit FIFO"]
pub struct RTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTF_AW::_0)
    }
    #[doc = "Transmit FIFO is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTF_AW::_1)
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
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRF_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Receive FIFO is reset."]
    _1 = 1,
}
impl From<RRF_AW> for bool {
    #[inline(always)]
    fn from(variant: RRF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRF` writer - Reset Receive FIFO"]
pub struct RRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRF_AW::_0)
    }
    #[doc = "Receive FIFO is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn men(&mut self) -> MEN_W {
        MEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DOZEN_W {
        DOZEN_W { w: self }
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&mut self) -> RTF_W {
        RTF_W { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&mut self) -> RRF_W {
        RRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
