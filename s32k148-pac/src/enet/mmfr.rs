#[doc = "Register `MMFR` reader"]
pub struct R(crate::R<MMFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMFR` writer"]
pub struct W(crate::W<MMFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMFR_SPEC>;
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
impl From<crate::W<MMFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Management Frame Data"]
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Management Frame Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TA` reader - Turn Around"]
pub struct TA_R(crate::FieldReader<u8, u8>);
impl TA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA` writer - Turn Around"]
pub struct TA_W<'a> {
    w: &'a mut W,
}
impl<'a> TA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RA` reader - Register Address"]
pub struct RA_R(crate::FieldReader<u8, u8>);
impl RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Register Address"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `PA` reader - PHY Address"]
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - PHY Address"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | ((value as u32 & 0x1f) << 23);
        self.w
    }
}
#[doc = "Field `OP` reader - Operation Code"]
pub struct OP_R(crate::FieldReader<u8, u8>);
impl OP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP` writer - Operation Code"]
pub struct OP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `ST` reader - Start Of Frame Delimiter"]
pub struct ST_R(crate::FieldReader<u8, u8>);
impl ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` writer - Start Of Frame Delimiter"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    pub fn ta(&mut self) -> TA_W {
        TA_W { w: self }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W {
        OP_W { w: self }
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Management Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfr](index.html) module"]
pub struct MMFR_SPEC;
impl crate::RegisterSpec for MMFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmfr::R](R) reader structure"]
impl crate::Readable for MMFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmfr::W](W) writer structure"]
impl crate::Writable for MMFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMFR to value 0"]
impl crate::Resettable for MMFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
