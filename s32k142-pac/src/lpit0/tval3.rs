#[doc = "Register `TVAL3` reader"]
pub struct R(crate::R<TVAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TVAL3` writer"]
pub struct W(crate::W<TVAL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVAL3_SPEC>;
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
impl From<crate::W<TVAL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVAL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TMR_VAL_A {
    #[doc = "0: Invalid load value in compare modes."]
    _0 = 0,
    #[doc = "1: Invalid load value in compare modes."]
    _1 = 1,
}
impl From<TMR_VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: TMR_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR_VAL` reader - Timer Value"]
pub struct TMR_VAL_R(crate::FieldReader<u32, TMR_VAL_A>);
impl TMR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TMR_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR_VAL_A> {
        match self.bits {
            0 => Some(TMR_VAL_A::_0),
            1 => Some(TMR_VAL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR_VAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR_VAL_A::_1
    }
}
impl core::ops::Deref for TMR_VAL_R {
    type Target = crate::FieldReader<u32, TMR_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_VAL` writer - Timer Value"]
pub struct TMR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Invalid load value in compare modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR_VAL_A::_0)
    }
    #[doc = "Invalid load value in compare modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR_VAL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&self) -> TMR_VAL_R {
        TMR_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&mut self) -> TMR_VAL_W {
        TMR_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval3](index.html) module"]
pub struct TVAL3_SPEC;
impl crate::RegisterSpec for TVAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tval3::R](R) reader structure"]
impl crate::Readable for TVAL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tval3::W](W) writer structure"]
impl crate::Writable for TVAL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TVAL3 to value 0"]
impl crate::Resettable for TVAL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
