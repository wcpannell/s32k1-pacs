#[doc = "Register `IFLAG2` reader"]
pub struct R(crate::R<IFLAG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLAG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLAG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLAG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLAG2` writer"]
pub struct W(crate::W<IFLAG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLAG2_SPEC>;
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
impl From<crate::W<IFLAG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLAG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF63TO32I` reader - Buffer MB i Interrupt"]
pub struct BUF63TO32I_R(crate::FieldReader<u32, u32>);
impl BUF63TO32I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF63TO32I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF63TO32I_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF63TO32I` writer - Buffer MB i Interrupt"]
pub struct BUF63TO32I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF63TO32I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MB i Interrupt"]
    #[inline(always)]
    pub fn buf63to32i(&self) -> BUF63TO32I_R {
        BUF63TO32I_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MB i Interrupt"]
    #[inline(always)]
    pub fn buf63to32i(&mut self) -> BUF63TO32I_W {
        BUF63TO32I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag2](index.html) module"]
pub struct IFLAG2_SPEC;
impl crate::RegisterSpec for IFLAG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iflag2::R](R) reader structure"]
impl crate::Readable for IFLAG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iflag2::W](W) writer structure"]
impl crate::Writable for IFLAG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFLAG2 to value 0"]
impl crate::Resettable for IFLAG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
