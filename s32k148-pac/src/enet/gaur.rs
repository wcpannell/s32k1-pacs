#[doc = "Register `GAUR` reader"]
pub struct R(crate::R<GAUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAUR` writer"]
pub struct W(crate::W<GAUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAUR_SPEC>;
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
impl From<crate::W<GAUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GADDR1` reader - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub struct GADDR1_R(crate::FieldReader<u32, u32>);
impl GADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GADDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GADDR1` writer - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub struct GADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> GADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr1(&self) -> GADDR1_R {
        GADDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr1(&mut self) -> GADDR1_W {
        GADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Group Upper Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaur](index.html) module"]
pub struct GAUR_SPEC;
impl crate::RegisterSpec for GAUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gaur::R](R) reader structure"]
impl crate::Readable for GAUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gaur::W](W) writer structure"]
impl crate::Writable for GAUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAUR to value 0"]
impl crate::Resettable for GAUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
