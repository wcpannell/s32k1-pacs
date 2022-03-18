#[doc = "Register `RSFL` reader"]
pub struct R(crate::R<RSFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSFL` writer"]
pub struct W(crate::W<RSFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSFL_SPEC>;
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
impl From<crate::W<RSFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SECTION_FULL` reader - Value Of Receive FIFO Section Full Threshold"]
pub struct RX_SECTION_FULL_R(crate::FieldReader<u8, u8>);
impl RX_SECTION_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_SECTION_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SECTION_FULL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SECTION_FULL` writer - Value Of Receive FIFO Section Full Threshold"]
pub struct RX_SECTION_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SECTION_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub fn rx_section_full(&self) -> RX_SECTION_FULL_R {
        RX_SECTION_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub fn rx_section_full(&mut self) -> RX_SECTION_FULL_W {
        RX_SECTION_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Section Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsfl](index.html) module"]
pub struct RSFL_SPEC;
impl crate::RegisterSpec for RSFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsfl::R](R) reader structure"]
impl crate::Readable for RSFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsfl::W](W) writer structure"]
impl crate::Writable for RSFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSFL to value 0"]
impl crate::Resettable for RSFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
