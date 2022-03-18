#[doc = "Register `PDOR` reader"]
pub struct R(crate::R<PDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDOR` writer"]
pub struct W(crate::W<PDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDOR_SPEC>;
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
impl From<crate::W<PDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDO` reader - Port Data Output"]
pub struct PDO_R(crate::FieldReader<u32, u32>);
impl PDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDO` writer - Port Data Output"]
pub struct PDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&self) -> PDO_R {
        PDO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&mut self) -> PDO_W {
        PDO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdor](index.html) module"]
pub struct PDOR_SPEC;
impl crate::RegisterSpec for PDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdor::R](R) reader structure"]
impl crate::Readable for PDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdor::W](W) writer structure"]
impl crate::Writable for PDOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDOR to value 0"]
impl crate::Resettable for PDOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
