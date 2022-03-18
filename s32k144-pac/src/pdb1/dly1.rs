#[doc = "Register `DLY1` reader"]
pub struct R(crate::R<DLY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLY1` writer"]
pub struct W(crate::W<DLY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLY1_SPEC>;
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
impl From<crate::W<DLY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY1` reader - DLY1"]
pub struct DLY1_R(crate::FieldReader<u16, u16>);
impl DLY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DLY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY1` writer - DLY1"]
pub struct DLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DLY1"]
    #[inline(always)]
    pub fn dly1(&self) -> DLY1_R {
        DLY1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DLY1"]
    #[inline(always)]
    pub fn dly1(&mut self) -> DLY1_W {
        DLY1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDB1_DLY1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly1](index.html) module"]
pub struct DLY1_SPEC;
impl crate::RegisterSpec for DLY1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dly1::R](R) reader structure"]
impl crate::Readable for DLY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dly1::W](W) writer structure"]
impl crate::Writable for DLY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLY1 to value 0xffff"]
impl crate::Resettable for DLY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
