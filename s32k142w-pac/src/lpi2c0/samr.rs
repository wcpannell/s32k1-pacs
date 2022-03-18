#[doc = "Register `SAMR` reader"]
pub struct R(crate::R<SAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMR` writer"]
pub struct W(crate::W<SAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMR_SPEC>;
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
impl From<crate::W<SAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - Address 0 Value"]
pub struct ADDR0_R(crate::FieldReader<u16, u16>);
impl ADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR0` writer - Address 0 Value"]
pub struct ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Field `ADDR1` reader - Address 1 Value"]
pub struct ADDR1_R(crate::FieldReader<u16, u16>);
impl ADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR1` writer - Address 1 Value"]
pub struct ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 17)) | ((value as u32 & 0x03ff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W {
        ADDR0_W { w: self }
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W {
        ADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samr](index.html) module"]
pub struct SAMR_SPEC;
impl crate::RegisterSpec for SAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samr::R](R) reader structure"]
impl crate::Readable for SAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samr::W](W) writer structure"]
impl crate::Writable for SAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMR to value 0"]
impl crate::Resettable for SAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
