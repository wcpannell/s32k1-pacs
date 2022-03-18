#[doc = "Register `INVCTRL` reader"]
pub struct R(crate::R<INVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INVCTRL` writer"]
pub struct W(crate::W<INVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INVCTRL_SPEC>;
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
impl From<crate::W<INVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pair Channels 0 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV0EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV0EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV0EN` reader - Pair Channels 0 Inverting Enable"]
pub struct INV0EN_R(crate::FieldReader<bool, INV0EN_A>);
impl INV0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV0EN_A {
        match self.bits {
            false => INV0EN_A::_0,
            true => INV0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV0EN_A::_1
    }
}
impl core::ops::Deref for INV0EN_R {
    type Target = crate::FieldReader<bool, INV0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV0EN` writer - Pair Channels 0 Inverting Enable"]
pub struct INV0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV0EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV0EN_A::_1)
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
#[doc = "Pair Channels 1 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV1EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV1EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV1EN` reader - Pair Channels 1 Inverting Enable"]
pub struct INV1EN_R(crate::FieldReader<bool, INV1EN_A>);
impl INV1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV1EN_A {
        match self.bits {
            false => INV1EN_A::_0,
            true => INV1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV1EN_A::_1
    }
}
impl core::ops::Deref for INV1EN_R {
    type Target = crate::FieldReader<bool, INV1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV1EN` writer - Pair Channels 1 Inverting Enable"]
pub struct INV1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV1EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV1EN_A::_1)
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
#[doc = "Pair Channels 2 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV2EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV2EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV2EN` reader - Pair Channels 2 Inverting Enable"]
pub struct INV2EN_R(crate::FieldReader<bool, INV2EN_A>);
impl INV2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV2EN_A {
        match self.bits {
            false => INV2EN_A::_0,
            true => INV2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV2EN_A::_1
    }
}
impl core::ops::Deref for INV2EN_R {
    type Target = crate::FieldReader<bool, INV2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV2EN` writer - Pair Channels 2 Inverting Enable"]
pub struct INV2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV2EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV2EN_A::_1)
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
#[doc = "Pair Channels 3 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV3EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV3EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV3EN` reader - Pair Channels 3 Inverting Enable"]
pub struct INV3EN_R(crate::FieldReader<bool, INV3EN_A>);
impl INV3EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV3EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV3EN_A {
        match self.bits {
            false => INV3EN_A::_0,
            true => INV3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV3EN_A::_1
    }
}
impl core::ops::Deref for INV3EN_R {
    type Target = crate::FieldReader<bool, INV3EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV3EN` writer - Pair Channels 3 Inverting Enable"]
pub struct INV3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV3EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV3EN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&self) -> INV0EN_R {
        INV0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&self) -> INV1EN_R {
        INV1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&self) -> INV2EN_R {
        INV2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&self) -> INV3EN_R {
        INV3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&mut self) -> INV0EN_W {
        INV0EN_W { w: self }
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&mut self) -> INV1EN_W {
        INV1EN_W { w: self }
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&mut self) -> INV2EN_W {
        INV2EN_W { w: self }
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&mut self) -> INV3EN_W {
        INV3EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Inverting Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invctrl](index.html) module"]
pub struct INVCTRL_SPEC;
impl crate::RegisterSpec for INVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [invctrl::R](R) reader structure"]
impl crate::Readable for INVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [invctrl::W](W) writer structure"]
impl crate::Writable for INVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INVCTRL to value 0"]
impl crate::Resettable for INVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
