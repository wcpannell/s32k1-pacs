#[doc = "Register `GPCLR` writer"]
pub struct W(crate::W<GPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCLR_SPEC>;
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
impl From<crate::W<GPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPWD` writer - Global Pin Write Data"]
pub struct GPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPWE_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE_AW> for u16 {
    #[inline(always)]
    fn from(variant: GPWE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `GPWE` writer - Global Pin Write Enable"]
pub struct GPWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    pub fn gpwd(&mut self) -> GPWD_W {
        GPWD_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe(&mut self) -> GPWE_W {
        GPWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Pin Control Low Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpclr](index.html) module"]
pub struct GPCLR_SPEC;
impl crate::RegisterSpec for GPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpclr::W](W) writer structure"]
impl crate::Writable for GPCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCLR to value 0"]
impl crate::Resettable for GPCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
