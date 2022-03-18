#[doc = "Register `SAI_RMR` reader"]
pub struct R(crate::R<SAI_RMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_RMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_RMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_RMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_RMR` writer"]
pub struct W(crate::W<SAI_RMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_RMR_SPEC>;
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
impl From<crate::W<SAI_RMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_RMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RWM_A {
    #[doc = "0: Word N is enabled."]
    _0000000000000000 = 0,
    #[doc = "1: Word N is masked."]
    _0000000000000001 = 1,
}
impl From<RWM_A> for u16 {
    #[inline(always)]
    fn from(variant: RWM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RWM` reader - Receive Word Mask"]
pub struct RWM_R(crate::FieldReader<u16, RWM_A>);
impl RWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RWM_A> {
        match self.bits {
            0 => Some(RWM_A::_0000000000000000),
            1 => Some(RWM_A::_0000000000000001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000000000000000`"]
    #[inline(always)]
    pub fn is_0000000000000000(&self) -> bool {
        **self == RWM_A::_0000000000000000
    }
    #[doc = "Checks if the value of the field is `_0000000000000001`"]
    #[inline(always)]
    pub fn is_0000000000000001(&self) -> bool {
        **self == RWM_A::_0000000000000001
    }
}
impl core::ops::Deref for RWM_R {
    type Target = crate::FieldReader<u16, RWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWM` writer - Receive Word Mask"]
pub struct RWM_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0000000000000000(self) -> &'a mut W {
        self.variant(RWM_A::_0000000000000000)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _0000000000000001(self) -> &'a mut W {
        self.variant(RWM_A::_0000000000000001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm(&self) -> RWM_R {
        RWM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm(&mut self) -> RWM_W {
        RWM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_rmr](index.html) module"]
pub struct SAI_RMR_SPEC;
impl crate::RegisterSpec for SAI_RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_rmr::R](R) reader structure"]
impl crate::Readable for SAI_RMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_rmr::W](W) writer structure"]
impl crate::Writable for SAI_RMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_RMR to value 0"]
impl crate::Resettable for SAI_RMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
