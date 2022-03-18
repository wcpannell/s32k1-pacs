#[doc = "Register `ISFR` reader"]
pub struct R(crate::R<ISFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISFR` writer"]
pub struct W(crate::W<ISFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISFR_SPEC>;
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
impl From<crate::W<ISFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ISF_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF_A> for u32 {
    #[inline(always)]
    fn from(variant: ISF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub struct ISF_R(crate::FieldReader<u32, ISF_A>);
impl ISF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ISF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ISF_A> {
        match self.bits {
            0 => Some(ISF_A::_0),
            1 => Some(ISF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISF_A::_1
    }
}
impl core::ops::Deref for ISF_R {
    type Target = crate::FieldReader<u32, ISF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub struct ISF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&mut self) -> ISF_W {
        ISF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isfr](index.html) module"]
pub struct ISFR_SPEC;
impl crate::RegisterSpec for ISFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isfr::R](R) reader structure"]
impl crate::Readable for ISFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isfr::W](W) writer structure"]
impl crate::Writable for ISFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISFR to value 0"]
impl crate::Resettable for ISFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
