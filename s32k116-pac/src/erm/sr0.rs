#[doc = "Register `SR0` reader"]
pub struct R(crate::R<SR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR0` writer"]
pub struct W(crate::W<SR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR0_SPEC>;
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
impl From<crate::W<SR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "NCE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE0_A {
    #[doc = "0: No non-correctable error event on Memory 0 detected"]
    _0 = 0,
    #[doc = "1: Non-correctable error event on Memory 0 detected"]
    _1 = 1,
}
impl From<NCE0_A> for bool {
    #[inline(always)]
    fn from(variant: NCE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCE0` reader - NCE0"]
pub struct NCE0_R(crate::FieldReader<bool, NCE0_A>);
impl NCE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NCE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCE0_A {
        match self.bits {
            false => NCE0_A::_0,
            true => NCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NCE0_A::_1
    }
}
impl core::ops::Deref for NCE0_R {
    type Target = crate::FieldReader<bool, NCE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCE0` writer - NCE0"]
pub struct NCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> NCE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No non-correctable error event on Memory 0 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCE0_A::_0)
    }
    #[doc = "Non-correctable error event on Memory 0 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCE0_A::_1)
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
#[doc = "SBC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC0_A {
    #[doc = "0: No single-bit correction event on Memory 0 detected"]
    _0 = 0,
    #[doc = "1: Single-bit correction event on Memory 0 detected"]
    _1 = 1,
}
impl From<SBC0_A> for bool {
    #[inline(always)]
    fn from(variant: SBC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC0` reader - SBC0"]
pub struct SBC0_R(crate::FieldReader<bool, SBC0_A>);
impl SBC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SBC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBC0_A {
        match self.bits {
            false => SBC0_A::_0,
            true => SBC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SBC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SBC0_A::_1
    }
}
impl core::ops::Deref for SBC0_R {
    type Target = crate::FieldReader<bool, SBC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBC0` writer - SBC0"]
pub struct SBC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No single-bit correction event on Memory 0 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBC0_A::_0)
    }
    #[doc = "Single-bit correction event on Memory 0 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBC0_A::_1)
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
    #[doc = "Bit 30 - NCE0"]
    #[inline(always)]
    pub fn nce0(&self) -> NCE0_R {
        NCE0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline(always)]
    pub fn sbc0(&self) -> SBC0_R {
        SBC0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - NCE0"]
    #[inline(always)]
    pub fn nce0(&mut self) -> NCE0_W {
        NCE0_W { w: self }
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline(always)]
    pub fn sbc0(&mut self) -> SBC0_W {
        SBC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERM Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr0](index.html) module"]
pub struct SR0_SPEC;
impl crate::RegisterSpec for SR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr0::R](R) reader structure"]
impl crate::Readable for SR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr0::W](W) writer structure"]
impl crate::Writable for SR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR0 to value 0"]
impl crate::Resettable for SR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
