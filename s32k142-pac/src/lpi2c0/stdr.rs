#[doc = "Register `STDR` reader"]
pub struct R(crate::R<STDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDR` writer"]
pub struct W(crate::W<STDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDR_SPEC>;
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
impl From<crate::W<STDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Transmit Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdr](index.html) module"]
pub struct STDR_SPEC;
impl crate::RegisterSpec for STDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stdr::R](R) reader structure"]
impl crate::Readable for STDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdr::W](W) writer structure"]
impl crate::Writable for STDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STDR to value 0"]
impl crate::Resettable for STDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
