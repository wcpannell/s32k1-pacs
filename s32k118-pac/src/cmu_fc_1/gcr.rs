#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCE_A {
    #[doc = "0: Frequency Check Disabled"]
    _0 = 0,
    #[doc = "1: Frequency Check Enabled"]
    _1 = 1,
}
impl From<FCE_A> for bool {
    #[inline(always)]
    fn from(variant: FCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` reader - Frequency Check Enable"]
pub struct FCE_R(crate::FieldReader<bool, FCE_A>);
impl FCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCE_A {
        match self.bits {
            false => FCE_A::_0,
            true => FCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCE_A::_1
    }
}
impl core::ops::Deref for FCE_R {
    type Target = crate::FieldReader<bool, FCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCE` writer - Frequency Check Enable"]
pub struct FCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frequency Check Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCE_A::_0)
    }
    #[doc = "Frequency Check Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Frequency Check Enable"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Check Enable"]
    #[inline(always)]
    pub fn fce(&mut self) -> FCE_W {
        FCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Frequency Check Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
