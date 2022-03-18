#[doc = "Register `RXMGMASK` reader"]
pub struct R(crate::R<RXMGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMGMASK` writer"]
pub struct W(crate::W<RXMGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMGMASK_SPEC>;
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
impl From<crate::W<RXMGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MG` reader - Rx Mailboxes Global Mask Bits"]
pub struct MG_R(crate::FieldReader<u32, u32>);
impl MG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MG` writer - Rx Mailboxes Global Mask Bits"]
pub struct MG_W<'a> {
    w: &'a mut W,
}
impl<'a> MG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg(&mut self) -> MG_W {
        MG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Mailboxes Global Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmgmask](index.html) module"]
pub struct RXMGMASK_SPEC;
impl crate::RegisterSpec for RXMGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmgmask::R](R) reader structure"]
impl crate::Readable for RXMGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](W) writer structure"]
impl crate::Writable for RXMGMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXMGMASK to value 0"]
impl crate::Resettable for RXMGMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
