#[doc = "Register `CTRL2_PN` reader"]
pub struct R(crate::R<CTRL2_PN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_PN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_PN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_PN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2_PN` writer"]
pub struct W(crate::W<CTRL2_PN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_PN_SPEC>;
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
impl From<crate::W<CTRL2_PN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_PN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCHTO` reader - Timeout for No Message Matching the Filtering Criteria"]
pub struct MATCHTO_R(crate::FieldReader<u16, u16>);
impl MATCHTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MATCHTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCHTO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCHTO` writer - Timeout for No Message Matching the Filtering Criteria"]
pub struct MATCHTO_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timeout for No Message Matching the Filtering Criteria"]
    #[inline(always)]
    pub fn matchto(&self) -> MATCHTO_R {
        MATCHTO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout for No Message Matching the Filtering Criteria"]
    #[inline(always)]
    pub fn matchto(&mut self) -> MATCHTO_W {
        MATCHTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2_pn](index.html) module"]
pub struct CTRL2_PN_SPEC;
impl crate::RegisterSpec for CTRL2_PN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2_pn::R](R) reader structure"]
impl crate::Readable for CTRL2_PN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2_pn::W](W) writer structure"]
impl crate::Writable for CTRL2_PN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2_PN to value 0"]
impl crate::Resettable for CTRL2_PN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
