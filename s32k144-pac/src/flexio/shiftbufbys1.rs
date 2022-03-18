#[doc = "Register `SHIFTBUFBYS1` reader"]
pub struct R(crate::R<SHIFTBUFBYS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFBYS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFBYS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFBYS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFBYS1` writer"]
pub struct W(crate::W<SHIFTBUFBYS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFBYS1_SPEC>;
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
impl From<crate::W<SHIFTBUFBYS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFBYS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFBYS` reader - Shift Buffer"]
pub struct SHIFTBUFBYS_R(crate::FieldReader<u32, u32>);
impl SHIFTBUFBYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SHIFTBUFBYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTBUFBYS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTBUFBYS` writer - Shift Buffer"]
pub struct SHIFTBUFBYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBYS_W<'a> {
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
    pub fn shiftbufbys(&self) -> SHIFTBUFBYS_R {
        SHIFTBUFBYS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbys(&mut self) -> SHIFTBUFBYS_W {
        SHIFTBUFBYS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys1](index.html) module"]
pub struct SHIFTBUFBYS1_SPEC;
impl crate::RegisterSpec for SHIFTBUFBYS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufbys1::R](R) reader structure"]
impl crate::Readable for SHIFTBUFBYS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufbys1::W](W) writer structure"]
impl crate::Writable for SHIFTBUFBYS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTBUFBYS1 to value 0"]
impl crate::Resettable for SHIFTBUFBYS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
