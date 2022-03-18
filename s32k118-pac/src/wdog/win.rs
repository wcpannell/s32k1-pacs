#[doc = "Register `WIN` reader"]
pub struct R(crate::R<WIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIN` writer"]
pub struct W(crate::W<WIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIN_SPEC>;
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
impl From<crate::W<WIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINLOW` reader - Low byte of Watchdog Window"]
pub struct WINLOW_R(crate::FieldReader<u8, u8>);
impl WINLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLOW` writer - Low byte of Watchdog Window"]
pub struct WINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `WINHIGH` reader - High byte of Watchdog Window"]
pub struct WINHIGH_R(crate::FieldReader<u8, u8>);
impl WINHIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINHIGH` writer - High byte of Watchdog Window"]
pub struct WINHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WINHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&mut self) -> WINLOW_W {
        WINLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&mut self) -> WINHIGH_W {
        WINHIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win](index.html) module"]
pub struct WIN_SPEC;
impl crate::RegisterSpec for WIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [win::R](R) reader structure"]
impl crate::Readable for WIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [win::W](W) writer structure"]
impl crate::Writable for WIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIN to value 0"]
impl crate::Resettable for WIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
