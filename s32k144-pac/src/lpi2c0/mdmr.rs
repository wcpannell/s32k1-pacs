#[doc = "Register `MDMR` reader"]
pub struct R(crate::R<MDMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMR` writer"]
pub struct W(crate::W<MDMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMR_SPEC>;
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
impl From<crate::W<MDMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH0` reader - Match 0 Value"]
pub struct MATCH0_R(crate::FieldReader<u8, u8>);
impl MATCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MATCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH0` writer - Match 0 Value"]
pub struct MATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MATCH1` reader - Match 1 Value"]
pub struct MATCH1_R(crate::FieldReader<u8, u8>);
impl MATCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MATCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH1` writer - Match 1 Value"]
pub struct MATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Match 0 Value"]
    #[inline(always)]
    pub fn match0(&self) -> MATCH0_R {
        MATCH0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&self) -> MATCH1_R {
        MATCH1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match 0 Value"]
    #[inline(always)]
    pub fn match0(&mut self) -> MATCH0_W {
        MATCH0_W { w: self }
    }
    #[doc = "Bits 16:23 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&mut self) -> MATCH1_W {
        MATCH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Data Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmr](index.html) module"]
pub struct MDMR_SPEC;
impl crate::RegisterSpec for MDMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdmr::R](R) reader structure"]
impl crate::Readable for MDMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdmr::W](W) writer structure"]
impl crate::Writable for MDMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMR to value 0"]
impl crate::Resettable for MDMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
