#[doc = "Register `RX14MASK` reader"]
pub struct R(crate::R<RX14MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX14MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX14MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX14MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX14MASK` writer"]
pub struct W(crate::W<RX14MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX14MASK_SPEC>;
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
impl From<crate::W<RX14MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX14MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX14M` reader - Rx Buffer 14 Mask Bits"]
pub struct RX14M_R(crate::FieldReader<u32, u32>);
impl RX14M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX14M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX14M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX14M` writer - Rx Buffer 14 Mask Bits"]
pub struct RX14M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX14M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&self) -> RX14M_R {
        RX14M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&mut self) -> RX14M_W {
        RX14M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx 14 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx14mask](index.html) module"]
pub struct RX14MASK_SPEC;
impl crate::RegisterSpec for RX14MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx14mask::R](R) reader structure"]
impl crate::Readable for RX14MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx14mask::W](W) writer structure"]
impl crate::Writable for RX14MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX14MASK to value 0"]
impl crate::Resettable for RX14MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
