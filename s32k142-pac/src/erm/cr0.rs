#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ENCIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCIE1_A {
    #[doc = "0: Interrupt notification of Memory 1 non-correctable error events is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt notification of Memory 1 non-correctable error events is enabled."]
    _1 = 1,
}
impl From<ENCIE1_A> for bool {
    #[inline(always)]
    fn from(variant: ENCIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENCIE1` reader - ENCIE1"]
pub struct ENCIE1_R(crate::FieldReader<bool, ENCIE1_A>);
impl ENCIE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCIE1_A {
        match self.bits {
            false => ENCIE1_A::_0,
            true => ENCIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENCIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENCIE1_A::_1
    }
}
impl core::ops::Deref for ENCIE1_R {
    type Target = crate::FieldReader<bool, ENCIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCIE1` writer - ENCIE1"]
pub struct ENCIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCIE1_A::_0)
    }
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCIE1_A::_1)
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
#[doc = "ESCIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCIE1_A {
    #[doc = "0: Interrupt notification of Memory 1 single-bit correction events is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt notification of Memory 1 single-bit correction events is enabled."]
    _1 = 1,
}
impl From<ESCIE1_A> for bool {
    #[inline(always)]
    fn from(variant: ESCIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESCIE1` reader - ESCIE1"]
pub struct ESCIE1_R(crate::FieldReader<bool, ESCIE1_A>);
impl ESCIE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESCIE1_A {
        match self.bits {
            false => ESCIE1_A::_0,
            true => ESCIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ESCIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ESCIE1_A::_1
    }
}
impl core::ops::Deref for ESCIE1_R {
    type Target = crate::FieldReader<bool, ESCIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCIE1` writer - ESCIE1"]
pub struct ESCIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESCIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESCIE1_A::_0)
    }
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESCIE1_A::_1)
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
#[doc = "ENCIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCIE0_A {
    #[doc = "0: Interrupt notification of Memory 0 non-correctable error events is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt notification of Memory 0 non-correctable error events is enabled."]
    _1 = 1,
}
impl From<ENCIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ENCIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENCIE0` reader - ENCIE0"]
pub struct ENCIE0_R(crate::FieldReader<bool, ENCIE0_A>);
impl ENCIE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCIE0_A {
        match self.bits {
            false => ENCIE0_A::_0,
            true => ENCIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENCIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENCIE0_A::_1
    }
}
impl core::ops::Deref for ENCIE0_R {
    type Target = crate::FieldReader<bool, ENCIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCIE0` writer - ENCIE0"]
pub struct ENCIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCIE0_A::_0)
    }
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCIE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "ESCIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCIE0_A {
    #[doc = "0: Interrupt notification of Memory 0 single-bit correction events is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt notification of Memory 0 single-bit correction events is enabled."]
    _1 = 1,
}
impl From<ESCIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ESCIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESCIE0` reader - ESCIE0"]
pub struct ESCIE0_R(crate::FieldReader<bool, ESCIE0_A>);
impl ESCIE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESCIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESCIE0_A {
        match self.bits {
            false => ESCIE0_A::_0,
            true => ESCIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ESCIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ESCIE0_A::_1
    }
}
impl core::ops::Deref for ESCIE0_R {
    type Target = crate::FieldReader<bool, ESCIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCIE0` writer - ESCIE0"]
pub struct ESCIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESCIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESCIE0_A::_0)
    }
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESCIE0_A::_1)
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
    #[doc = "Bit 26 - ENCIE1"]
    #[inline(always)]
    pub fn encie1(&self) -> ENCIE1_R {
        ENCIE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ESCIE1"]
    #[inline(always)]
    pub fn escie1(&self) -> ESCIE1_R {
        ESCIE1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ENCIE0"]
    #[inline(always)]
    pub fn encie0(&self) -> ENCIE0_R {
        ENCIE0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ESCIE0"]
    #[inline(always)]
    pub fn escie0(&self) -> ESCIE0_R {
        ESCIE0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - ENCIE1"]
    #[inline(always)]
    pub fn encie1(&mut self) -> ENCIE1_W {
        ENCIE1_W { w: self }
    }
    #[doc = "Bit 27 - ESCIE1"]
    #[inline(always)]
    pub fn escie1(&mut self) -> ESCIE1_W {
        ESCIE1_W { w: self }
    }
    #[doc = "Bit 30 - ENCIE0"]
    #[inline(always)]
    pub fn encie0(&mut self) -> ENCIE0_W {
        ENCIE0_W { w: self }
    }
    #[doc = "Bit 31 - ESCIE0"]
    #[inline(always)]
    pub fn escie0(&mut self) -> ESCIE0_W {
        ESCIE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERM Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
