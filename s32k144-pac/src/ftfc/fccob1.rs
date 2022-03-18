#[doc = "Register `FCCOB1` reader"]
pub struct R(crate::R<FCCOB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCOB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCOB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCOB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCOB1` writer"]
pub struct W(crate::W<FCCOB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCOB1_SPEC>;
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
impl From<crate::W<FCCOB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCOB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCOBn` reader - CCOBn"]
pub struct CCOBN_R(crate::FieldReader<u8, u8>);
impl CCOBN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCOBN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCOBN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCOBn` writer - CCOBn"]
pub struct CCOBN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOBN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CCOBn"]
    #[inline(always)]
    pub fn ccobn(&self) -> CCOBN_R {
        CCOBN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CCOBn"]
    #[inline(always)]
    pub fn ccobn(&mut self) -> CCOBN_W {
        CCOBN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob1](index.html) module"]
pub struct FCCOB1_SPEC;
impl crate::RegisterSpec for FCCOB1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fccob1::R](R) reader structure"]
impl crate::Readable for FCCOB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccob1::W](W) writer structure"]
impl crate::Writable for FCCOB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCOB1 to value 0"]
impl crate::Resettable for FCCOB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
