#[doc = "Register `IPCR` reader"]
pub struct R(crate::R<IPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCR` writer"]
pub struct W(crate::W<IPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCR_SPEC>;
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
impl From<crate::W<IPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDATSZ` reader - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
pub struct IDATSZ_R(crate::FieldReader<u16, u16>);
impl IDATSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDATSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDATSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDATSZ` writer - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
pub struct IDATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SEQID` reader - Points to a sequence in the Look-up table"]
pub struct SEQID_R(crate::FieldReader<u8, u8>);
impl SEQID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEQID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQID` writer - Points to a sequence in the Look-up table"]
pub struct SEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
    #[inline(always)]
    pub fn idatsz(&self) -> IDATSZ_R {
        IDATSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Points to a sequence in the Look-up table"]
    #[inline(always)]
    pub fn seqid(&self) -> SEQID_R {
        SEQID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
    #[inline(always)]
    pub fn idatsz(&mut self) -> IDATSZ_W {
        IDATSZ_W { w: self }
    }
    #[doc = "Bits 24:27 - Points to a sequence in the Look-up table"]
    #[inline(always)]
    pub fn seqid(&mut self) -> SEQID_W {
        SEQID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr](index.html) module"]
pub struct IPCR_SPEC;
impl crate::RegisterSpec for IPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcr::R](R) reader structure"]
impl crate::Readable for IPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcr::W](W) writer structure"]
impl crate::Writable for IPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPCR to value 0"]
impl crate::Resettable for IPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
