#[doc = "Register `_EmbeddedRAM17` reader"]
pub struct R(crate::R<CSE_PRAM__EMBEDDEDRAM17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSE_PRAM__EMBEDDEDRAM17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSE_PRAM__EMBEDDEDRAM17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSE_PRAM__EMBEDDEDRAM17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_EmbeddedRAM17` writer"]
pub struct W(crate::W<CSE_PRAM__EMBEDDEDRAM17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSE_PRAM__EMBEDDEDRAM17_SPEC>;
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
impl From<crate::W<CSE_PRAM__EMBEDDEDRAM17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSE_PRAM__EMBEDDEDRAM17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE_3` reader - Data byte 3 of Rx/Tx frame."]
pub struct BYTE_3_R(crate::FieldReader<u8, u8>);
impl BYTE_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_3` writer - Data byte 3 of Rx/Tx frame."]
pub struct BYTE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BYTE_2` reader - Data byte 2 of Rx/Tx frame."]
pub struct BYTE_2_R(crate::FieldReader<u8, u8>);
impl BYTE_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_2` writer - Data byte 2 of Rx/Tx frame."]
pub struct BYTE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE_1` reader - Data byte 1 of Rx/Tx frame."]
pub struct BYTE_1_R(crate::FieldReader<u8, u8>);
impl BYTE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_1` writer - Data byte 1 of Rx/Tx frame."]
pub struct BYTE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `BYTE_0` reader - Data byte 0 of Rx/Tx frame."]
pub struct BYTE_0_R(crate::FieldReader<u8, u8>);
impl BYTE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_0` writer - Data byte 0 of Rx/Tx frame."]
pub struct BYTE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_3(&self) -> BYTE_3_R {
        BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_2(&self) -> BYTE_2_R {
        BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_1(&self) -> BYTE_1_R {
        BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_0(&self) -> BYTE_0_R {
        BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_3(&mut self) -> BYTE_3_W {
        BYTE_3_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_2(&mut self) -> BYTE_2_W {
        BYTE_2_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_1(&mut self) -> BYTE_1_W {
        BYTE_1_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn byte_0(&mut self) -> BYTE_0_W {
        BYTE_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSE PRAM 17 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse_pram__embedded_ram17](index.html) module"]
pub struct CSE_PRAM__EMBEDDEDRAM17_SPEC;
impl crate::RegisterSpec for CSE_PRAM__EMBEDDEDRAM17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cse_pram__embedded_ram17::R](R) reader structure"]
impl crate::Readable for CSE_PRAM__EMBEDDEDRAM17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cse_pram__embedded_ram17::W](W) writer structure"]
impl crate::Writable for CSE_PRAM__EMBEDDEDRAM17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _EmbeddedRAM17 to value 0"]
impl crate::Resettable for CSE_PRAM__EMBEDDEDRAM17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
