#[doc = "Register `GPOLY` reader"]
pub struct R(crate::R<GPOLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPOLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPOLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPOLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLY` writer"]
pub struct W(crate::W<GPOLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPOLY_SPEC>;
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
impl From<crate::W<GPOLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPOLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - Low Polynominal Half-word"]
pub struct LOW_R(crate::FieldReader<u16, u16>);
impl LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW` writer - Low Polynominal Half-word"]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `HIGH` reader - High Polynominal Half-word"]
pub struct HIGH_R(crate::FieldReader<u16, u16>);
impl HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH` writer - High Polynominal Half-word"]
pub struct HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low Polynominal Half-word"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Polynominal Half-word"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Polynominal Half-word"]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 16:31 - High Polynominal Half-word"]
    #[inline(always)]
    pub fn high(&mut self) -> HIGH_W {
        HIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpoly](index.html) module"]
pub struct GPOLY_SPEC;
impl crate::RegisterSpec for GPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpoly::R](R) reader structure"]
impl crate::Readable for GPOLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpoly::W](W) writer structure"]
impl crate::Writable for GPOLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLY to value 0x1021"]
impl crate::Resettable for GPOLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1021
    }
}
