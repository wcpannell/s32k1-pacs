#[doc = "Register `IMASK1` reader"]
pub struct R(crate::R<IMASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK1` writer"]
pub struct W(crate::W<IMASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK1_SPEC>;
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
impl From<crate::W<IMASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF31TO0M` reader - Buffer MB i Mask"]
pub struct BUF31TO0M_R(crate::FieldReader<u32, u32>);
impl BUF31TO0M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF31TO0M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF31TO0M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF31TO0M` writer - Buffer MB i Mask"]
pub struct BUF31TO0M_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO0M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf31to0m(&self) -> BUF31TO0M_R {
        BUF31TO0M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf31to0m(&mut self) -> BUF31TO0M_W {
        BUF31TO0M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Masks 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask1](index.html) module"]
pub struct IMASK1_SPEC;
impl crate::RegisterSpec for IMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask1::R](R) reader structure"]
impl crate::Readable for IMASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask1::W](W) writer structure"]
impl crate::Writable for IMASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMASK1 to value 0"]
impl crate::Resettable for IMASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
