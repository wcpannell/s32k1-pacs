#[doc = "Register `GICHR` writer"]
pub struct W(crate::W<GICHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICHR_SPEC>;
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
impl From<crate::W<GICHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global Interrupt Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GIWE_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GIWE_AW> for u16 {
    #[inline(always)]
    fn from(variant: GIWE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `GIWE` writer - Global Interrupt Write Enable"]
pub struct GIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIWE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GIWE_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GIWE_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `GIWD` writer - Global Interrupt Write Data"]
pub struct GIWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe(&mut self) -> GIWE_W {
        GIWE_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&mut self) -> GIWD_W {
        GIWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Interrupt Control High Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gichr](index.html) module"]
pub struct GICHR_SPEC;
impl crate::RegisterSpec for GICHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gichr::W](W) writer structure"]
impl crate::Writable for GICHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICHR to value 0"]
impl crate::Resettable for GICHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
