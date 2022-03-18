#[doc = "Register `DFCR` reader"]
pub struct R(crate::R<DFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFCR` writer"]
pub struct W(crate::W<DFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFCR_SPEC>;
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
impl From<crate::W<DFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: Digital filters are clocked by the bus clock."]
    _0 = 0,
    #[doc = "1: Digital filters are clocked by the LPO clock."]
    _1 = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Clock Source"]
pub struct CS_R(crate::FieldReader<bool, CS_A>);
impl CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::_0,
            true => CS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CS_A::_1
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<bool, CS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - Clock Source"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Digital filters are clocked by the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CS_A::_0)
    }
    #[doc = "Digital filters are clocked by the LPO clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CS_A::_1)
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
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Filter Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfcr](index.html) module"]
pub struct DFCR_SPEC;
impl crate::RegisterSpec for DFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfcr::R](R) reader structure"]
impl crate::Readable for DFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfcr::W](W) writer structure"]
impl crate::Writable for DFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFCR to value 0"]
impl crate::Resettable for DFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
