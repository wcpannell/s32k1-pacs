#[doc = "Register `EICHD1_WORD0` reader"]
pub struct R(crate::R<EICHD1_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICHD1_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICHD1_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICHD1_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICHD1_WORD0` writer"]
pub struct W(crate::W<EICHD1_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICHD1_WORD0_SPEC>;
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
impl From<crate::W<EICHD1_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICHD1_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHKBIT_MASK` reader - Checkbit Mask"]
pub struct CHKBIT_MASK_R(crate::FieldReader<u8, u8>);
impl CHKBIT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHKBIT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHKBIT_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKBIT_MASK` writer - Checkbit Mask"]
pub struct CHKBIT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKBIT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - Checkbit Mask"]
    #[inline(always)]
    pub fn chkbit_mask(&self) -> CHKBIT_MASK_R {
        CHKBIT_MASK_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - Checkbit Mask"]
    #[inline(always)]
    pub fn chkbit_mask(&mut self) -> CHKBIT_MASK_W {
        CHKBIT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Injection Channel Descriptor n, Word0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd1_word0](index.html) module"]
pub struct EICHD1_WORD0_SPEC;
impl crate::RegisterSpec for EICHD1_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eichd1_word0::R](R) reader structure"]
impl crate::Readable for EICHD1_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eichd1_word0::W](W) writer structure"]
impl crate::Writable for EICHD1_WORD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EICHD1_WORD0 to value 0"]
impl crate::Resettable for EICHD1_WORD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
