#[doc = "Register `RX15MASK` reader"]
pub struct R(crate::R<RX15MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX15MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX15MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX15MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX15MASK` writer"]
pub struct W(crate::W<RX15MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX15MASK_SPEC>;
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
impl From<crate::W<RX15MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX15MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX15M` reader - Rx Buffer 15 Mask Bits"]
pub struct RX15M_R(crate::FieldReader<u32, u32>);
impl RX15M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX15M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX15M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX15M` writer - Rx Buffer 15 Mask Bits"]
pub struct RX15M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m(&self) -> RX15M_R {
        RX15M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m(&mut self) -> RX15M_W {
        RX15M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx 15 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx15mask](index.html) module"]
pub struct RX15MASK_SPEC;
impl crate::RegisterSpec for RX15MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx15mask::R](R) reader structure"]
impl crate::Readable for RX15MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](W) writer structure"]
impl crate::Writable for RX15MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX15MASK to value 0"]
impl crate::Resettable for RX15MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
