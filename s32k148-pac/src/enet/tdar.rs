#[doc = "Register `TDAR` reader"]
pub struct R(crate::R<TDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDAR` writer"]
pub struct W(crate::W<TDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDAR_SPEC>;
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
impl From<crate::W<TDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAR` reader - Transmit Descriptor Active"]
pub struct TDAR_R(crate::FieldReader<bool, bool>);
impl TDAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDAR` writer - Transmit Descriptor Active"]
pub struct TDAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    pub fn tdar(&self) -> TDAR_R {
        TDAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    pub fn tdar(&mut self) -> TDAR_W {
        TDAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdar](index.html) module"]
pub struct TDAR_SPEC;
impl crate::RegisterSpec for TDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdar::R](R) reader structure"]
impl crate::Readable for TDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdar::W](W) writer structure"]
impl crate::Writable for TDAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDAR to value 0"]
impl crate::Resettable for TDAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
