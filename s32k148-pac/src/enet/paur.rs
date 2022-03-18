#[doc = "Register `PAUR` reader"]
pub struct R(crate::R<PAUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAUR` writer"]
pub struct W(crate::W<PAUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUR_SPEC>;
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
impl From<crate::W<PAUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - Type Field In PAUSE Frames"]
pub struct TYPE_R(crate::FieldReader<u16, u16>);
impl TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADDR2` reader - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
pub struct PADDR2_R(crate::FieldReader<u16, u16>);
impl PADDR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PADDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADDR2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADDR2` writer - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
pub struct PADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Type Field In PAUSE Frames"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub fn paddr2(&self) -> PADDR2_R {
        PADDR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub fn paddr2(&mut self) -> PADDR2_W {
        PADDR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Physical Address Upper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paur](index.html) module"]
pub struct PAUR_SPEC;
impl crate::RegisterSpec for PAUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paur::R](R) reader structure"]
impl crate::Readable for PAUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paur::W](W) writer structure"]
impl crate::Writable for PAUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAUR to value 0x8808"]
impl crate::Resettable for PAUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8808
    }
}
