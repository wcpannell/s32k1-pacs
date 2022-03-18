#[doc = "Register `FIRCCFG` reader"]
pub struct R(crate::R<FIRCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIRCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIRCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIRCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIRCCFG` writer"]
pub struct W(crate::W<FIRCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRCCFG_SPEC>;
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
impl From<crate::W<FIRCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: Fast IRC is trimmed to 48 MHz"]
    RANGE_0 = 0,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGE` reader - Frequency Range"]
pub struct RANGE_R(crate::FieldReader<u8, RANGE_A>);
impl RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            0 => Some(RANGE_A::RANGE_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RANGE_0`"]
    #[inline(always)]
    pub fn is_range_0(&self) -> bool {
        **self == RANGE_A::RANGE_0
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<u8, RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - Frequency Range"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fast IRC is trimmed to 48 MHz"]
    #[inline(always)]
    pub fn range_0(self) -> &'a mut W {
        self.variant(RANGE_A::RANGE_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firccfg](index.html) module"]
pub struct FIRCCFG_SPEC;
impl crate::RegisterSpec for FIRCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [firccfg::R](R) reader structure"]
impl crate::Readable for FIRCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [firccfg::W](W) writer structure"]
impl crate::Writable for FIRCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIRCCFG to value 0"]
impl crate::Resettable for FIRCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
