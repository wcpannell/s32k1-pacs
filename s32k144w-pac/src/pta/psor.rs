#[doc = "Register `PSOR` reader"]
pub struct R(crate::R<PSOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSOR` writer"]
pub struct W(crate::W<PSOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSOR_SPEC>;
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
impl From<crate::W<PSOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTSO` reader - Port Set Output"]
pub struct PTSO_R(crate::FieldReader<u32, u32>);
impl PTSO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PTSO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTSO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTSO` writer - Port Set Output"]
pub struct PTSO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&self) -> PTSO_R {
        PTSO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&mut self) -> PTSO_W {
        PTSO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Set Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psor](index.html) module"]
pub struct PSOR_SPEC;
impl crate::RegisterSpec for PSOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psor::R](R) reader structure"]
impl crate::Readable for PSOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psor::W](W) writer structure"]
impl crate::Writable for PSOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PSOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
