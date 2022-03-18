#[doc = "Register `RDAR` reader"]
pub struct R(crate::R<RDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDAR` writer"]
pub struct W(crate::W<RDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDAR_SPEC>;
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
impl From<crate::W<RDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDAR` reader - Receive Descriptor Active"]
pub struct RDAR_R(crate::FieldReader<bool, bool>);
impl RDAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDAR` writer - Receive Descriptor Active"]
pub struct RDAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAR_W<'a> {
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
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    pub fn rdar(&self) -> RDAR_R {
        RDAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    pub fn rdar(&mut self) -> RDAR_W {
        RDAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdar](index.html) module"]
pub struct RDAR_SPEC;
impl crate::RegisterSpec for RDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdar::R](R) reader structure"]
impl crate::Readable for RDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdar::W](W) writer structure"]
impl crate::Writable for RDAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDAR to value 0"]
impl crate::Resettable for RDAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
