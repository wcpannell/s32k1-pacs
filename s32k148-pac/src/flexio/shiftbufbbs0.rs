#[doc = "Register `SHIFTBUFBBS0` reader"]
pub struct R(crate::R<SHIFTBUFBBS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFBBS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFBBS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFBBS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFBBS0` writer"]
pub struct W(crate::W<SHIFTBUFBBS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFBBS0_SPEC>;
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
impl From<crate::W<SHIFTBUFBBS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFBBS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFBBS` reader - Shift Buffer"]
pub struct SHIFTBUFBBS_R(crate::FieldReader<u32, u32>);
impl SHIFTBUFBBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SHIFTBUFBBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTBUFBBS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTBUFBBS` writer - Shift Buffer"]
pub struct SHIFTBUFBBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBBS_W<'a> {
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
    pub fn shiftbufbbs(&self) -> SHIFTBUFBBS_R {
        SHIFTBUFBBS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbbs(&mut self) -> SHIFTBUFBBS_W {
        SHIFTBUFBBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs0](index.html) module"]
pub struct SHIFTBUFBBS0_SPEC;
impl crate::RegisterSpec for SHIFTBUFBBS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufbbs0::R](R) reader structure"]
impl crate::Readable for SHIFTBUFBBS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs0::W](W) writer structure"]
impl crate::Writable for SHIFTBUFBBS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTBUFBBS0 to value 0"]
impl crate::Resettable for SHIFTBUFBBS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
