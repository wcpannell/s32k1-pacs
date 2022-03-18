#[doc = "Register `TBDR` reader"]
pub struct R(crate::R<TBDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBDR` writer"]
pub struct W(crate::W<TBDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBDR_SPEC>;
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
impl From<crate::W<TBDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
pub struct TXDATA_R(crate::FieldReader<u32, u32>);
impl TXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATA` writer - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbdr](index.html) module"]
pub struct TBDR_SPEC;
impl crate::RegisterSpec for TBDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbdr::R](R) reader structure"]
impl crate::Readable for TBDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbdr::W](W) writer structure"]
impl crate::Writable for TBDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBDR to value 0"]
impl crate::Resettable for TBDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
