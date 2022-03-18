#[doc = "Register `FCCOBA` reader"]
pub struct R(crate::R<FCCOBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCOBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCOBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCOBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCOBA` writer"]
pub struct W(crate::W<FCCOBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCOBA_SPEC>;
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
impl From<crate::W<FCCOBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCOBA_SPEC>) -> Self {
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
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccoba](index.html) module"]
pub struct FCCOBA_SPEC;
impl crate::RegisterSpec for FCCOBA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fccoba::R](R) reader structure"]
impl crate::Readable for FCCOBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccoba::W](W) writer structure"]
impl crate::Writable for FCCOBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCOBA to value 0"]
impl crate::Resettable for FCCOBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
