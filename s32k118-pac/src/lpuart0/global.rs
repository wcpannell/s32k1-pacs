#[doc = "Register `GLOBAL` reader"]
pub struct R(crate::R<GLOBAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBAL` writer"]
pub struct W(crate::W<GLOBAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBAL_SPEC>;
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
impl From<crate::W<GLOBAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Module is not reset."]
    _0 = 0,
    #[doc = "1: Module is reset."]
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
    #[doc = "Module is not reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RST_A::_0)
    }
    #[doc = "Module is reset."]
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
impl R {
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Global Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global](index.html) module"]
pub struct GLOBAL_SPEC;
impl crate::RegisterSpec for GLOBAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [global::R](R) reader structure"]
impl crate::Readable for GLOBAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [global::W](W) writer structure"]
impl crate::Writable for GLOBAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBAL to value 0"]
impl crate::Resettable for GLOBAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
