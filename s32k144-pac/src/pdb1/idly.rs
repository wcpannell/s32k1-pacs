#[doc = "Register `IDLY` reader"]
pub struct R(crate::R<IDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLY` writer"]
pub struct W(crate::W<IDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLY_SPEC>;
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
impl From<crate::W<IDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLY` reader - PDB Interrupt Delay"]
pub struct IDLY_R(crate::FieldReader<u16, u16>);
impl IDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLY` writer - PDB Interrupt Delay"]
pub struct IDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&self) -> IDLY_R {
        IDLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&mut self) -> IDLY_W {
        IDLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idly](index.html) module"]
pub struct IDLY_SPEC;
impl crate::RegisterSpec for IDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idly::R](R) reader structure"]
impl crate::Readable for IDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idly::W](W) writer structure"]
impl crate::Writable for IDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDLY to value 0xffff"]
impl crate::Resettable for IDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
