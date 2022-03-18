#[doc = "Register `SHIFTSTAT` reader"]
pub struct R(crate::R<SHIFTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSTAT` writer"]
pub struct W(crate::W<SHIFTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSTAT_SPEC>;
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
impl From<crate::W<SHIFTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSF` reader - Shifter Status Flag"]
pub struct SSF_R(crate::FieldReader<u8, u8>);
impl SSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSF` writer - Shifter Status Flag"]
pub struct SSF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SSF_W {
        SSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstat](index.html) module"]
pub struct SHIFTSTAT_SPEC;
impl crate::RegisterSpec for SHIFTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftstat::R](R) reader structure"]
impl crate::Readable for SHIFTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftstat::W](W) writer structure"]
impl crate::Writable for SHIFTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTSTAT to value 0"]
impl crate::Resettable for SHIFTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
