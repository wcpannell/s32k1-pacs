#[doc = "Register `STOPCTRL` reader"]
pub struct R(crate::R<STOPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STOPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STOPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STOPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STOPCTRL` writer"]
pub struct W(crate::W<STOPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STOPCTRL_SPEC>;
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
impl From<crate::W<STOPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STOPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOPO_A {
    #[doc = "1: STOP1 - Stop with both system and bus clocks disabled"]
    _01 = 1,
    #[doc = "2: STOP2 - Stop with system clock disabled and bus clock enabled"]
    _10 = 2,
}
impl From<STOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STOPO` reader - Stop Option"]
pub struct STOPO_R(crate::FieldReader<u8, STOPO_A>);
impl STOPO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOPO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STOPO_A> {
        match self.bits {
            1 => Some(STOPO_A::_01),
            2 => Some(STOPO_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == STOPO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == STOPO_A::_10
    }
}
impl core::ops::Deref for STOPO_R {
    type Target = crate::FieldReader<u8, STOPO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPO` writer - Stop Option"]
pub struct STOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STOP1 - Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(STOPO_A::_01)
    }
    #[doc = "STOP2 - Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(STOPO_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline(always)]
    pub fn stopo(&self) -> STOPO_R {
        STOPO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline(always)]
    pub fn stopo(&mut self) -> STOPO_W {
        STOPO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stopctrl](index.html) module"]
pub struct STOPCTRL_SPEC;
impl crate::RegisterSpec for STOPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stopctrl::R](R) reader structure"]
impl crate::Readable for STOPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stopctrl::W](W) writer structure"]
impl crate::Writable for STOPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STOPCTRL to value 0x03"]
impl crate::Resettable for STOPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
