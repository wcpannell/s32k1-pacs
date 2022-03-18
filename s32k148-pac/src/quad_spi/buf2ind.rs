#[doc = "Register `BUF2IND` reader"]
pub struct R(crate::R<BUF2IND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF2IND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF2IND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF2IND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF2IND` writer"]
pub struct W(crate::W<BUF2IND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF2IND_SPEC>;
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
impl From<crate::W<BUF2IND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF2IND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPINDX2` reader - Top index of buffer 2."]
pub struct TPINDX2_R(crate::FieldReader<u32, u32>);
impl TPINDX2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TPINDX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPINDX2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPINDX2` writer - Top index of buffer 2."]
pub struct TPINDX2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPINDX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Top index of buffer 2."]
    #[inline(always)]
    pub fn tpindx2(&self) -> TPINDX2_R {
        TPINDX2_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Top index of buffer 2."]
    #[inline(always)]
    pub fn tpindx2(&mut self) -> TPINDX2_W {
        TPINDX2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer2 Top Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf2ind](index.html) module"]
pub struct BUF2IND_SPEC;
impl crate::RegisterSpec for BUF2IND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf2ind::R](R) reader structure"]
impl crate::Readable for BUF2IND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf2ind::W](W) writer structure"]
impl crate::Writable for BUF2IND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF2IND to value 0"]
impl crate::Resettable for BUF2IND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
