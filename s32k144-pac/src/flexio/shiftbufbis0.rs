#[doc = "Register `SHIFTBUFBIS0` reader"]
pub struct R(crate::R<SHIFTBUFBIS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFBIS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFBIS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFBIS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFBIS0` writer"]
pub struct W(crate::W<SHIFTBUFBIS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFBIS0_SPEC>;
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
impl From<crate::W<SHIFTBUFBIS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFBIS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFBIS` reader - Shift Buffer"]
pub struct SHIFTBUFBIS_R(crate::FieldReader<u32, u32>);
impl SHIFTBUFBIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SHIFTBUFBIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTBUFBIS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTBUFBIS` writer - Shift Buffer"]
pub struct SHIFTBUFBIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbis(&self) -> SHIFTBUFBIS_R {
        SHIFTBUFBIS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbis(&mut self) -> SHIFTBUFBIS_W {
        SHIFTBUFBIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis0](index.html) module"]
pub struct SHIFTBUFBIS0_SPEC;
impl crate::RegisterSpec for SHIFTBUFBIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufbis0::R](R) reader structure"]
impl crate::Readable for SHIFTBUFBIS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufbis0::W](W) writer structure"]
impl crate::Writable for SHIFTBUFBIS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTBUFBIS0 to value 0"]
impl crate::Resettable for SHIFTBUFBIS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
