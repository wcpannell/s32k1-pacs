#[doc = "Register `IMASK2` reader"]
pub struct R(crate::R<IMASK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK2` writer"]
pub struct W(crate::W<IMASK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK2_SPEC>;
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
impl From<crate::W<IMASK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF63TO32M` reader - Buffer MB i Mask"]
pub struct BUF63TO32M_R(crate::FieldReader<u32, u32>);
impl BUF63TO32M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF63TO32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF63TO32M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF63TO32M` writer - Buffer MB i Mask"]
pub struct BUF63TO32M_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF63TO32M_W<'a> {
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
    pub fn buf63to32m(&self) -> BUF63TO32M_R {
        BUF63TO32M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf63to32m(&mut self) -> BUF63TO32M_W {
        BUF63TO32M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Masks 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask2](index.html) module"]
pub struct IMASK2_SPEC;
impl crate::RegisterSpec for IMASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask2::R](R) reader structure"]
impl crate::Readable for IMASK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask2::W](W) writer structure"]
impl crate::Writable for IMASK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMASK2 to value 0"]
impl crate::Resettable for IMASK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
