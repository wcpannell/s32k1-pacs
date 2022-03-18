#[doc = "Register `ATINC` reader"]
pub struct R(crate::R<ATINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATINC` writer"]
pub struct W(crate::W<ATINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATINC_SPEC>;
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
impl From<crate::W<ATINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INC` reader - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
pub struct INC_R(crate::FieldReader<u8, u8>);
impl INC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC` writer - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
pub struct INC_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `INC_CORR` reader - Correction Increment Value"]
pub struct INC_CORR_R(crate::FieldReader<u8, u8>);
impl INC_CORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INC_CORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_CORR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC_CORR` writer - Correction Increment Value"]
pub struct INC_CORR_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_CORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub fn inc(&self) -> INC_R {
        INC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Correction Increment Value"]
    #[inline(always)]
    pub fn inc_corr(&self) -> INC_CORR_R {
        INC_CORR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub fn inc(&mut self) -> INC_W {
        INC_W { w: self }
    }
    #[doc = "Bits 8:14 - Correction Increment Value"]
    #[inline(always)]
    pub fn inc_corr(&mut self) -> INC_CORR_W {
        INC_CORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-Stamping Clock Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atinc](index.html) module"]
pub struct ATINC_SPEC;
impl crate::RegisterSpec for ATINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atinc::R](R) reader structure"]
impl crate::Readable for ATINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atinc::W](W) writer structure"]
impl crate::Writable for ATINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATINC to value 0"]
impl crate::Resettable for ATINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
