#[doc = "Register `BUF0CR` reader"]
pub struct R(crate::R<BUF0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF0CR` writer"]
pub struct W(crate::W<BUF0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF0CR_SPEC>;
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
impl From<crate::W<BUF0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTRID` reader - Master ID"]
pub struct MSTRID_R(crate::FieldReader<u8, u8>);
impl MSTRID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSTRID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTRID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTRID` writer - Master ID"]
pub struct MSTRID_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ADATSZ` reader - AHB data transfer size"]
pub struct ADATSZ_R(crate::FieldReader<u8, u8>);
impl ADATSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADATSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADATSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADATSZ` writer - AHB data transfer size"]
pub struct ADATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADATSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HP_EN` reader - High Priority Enable"]
pub struct HP_EN_R(crate::FieldReader<bool, bool>);
impl HP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HP_EN` writer - High Priority Enable"]
pub struct HP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Master ID"]
    #[inline(always)]
    pub fn mstrid(&self) -> MSTRID_R {
        MSTRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - AHB data transfer size"]
    #[inline(always)]
    pub fn adatsz(&self) -> ADATSZ_R {
        ADATSZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - High Priority Enable"]
    #[inline(always)]
    pub fn hp_en(&self) -> HP_EN_R {
        HP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master ID"]
    #[inline(always)]
    pub fn mstrid(&mut self) -> MSTRID_W {
        MSTRID_W { w: self }
    }
    #[doc = "Bits 8:15 - AHB data transfer size"]
    #[inline(always)]
    pub fn adatsz(&mut self) -> ADATSZ_W {
        ADATSZ_W { w: self }
    }
    #[doc = "Bit 31 - High Priority Enable"]
    #[inline(always)]
    pub fn hp_en(&mut self) -> HP_EN_W {
        HP_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf0cr](index.html) module"]
pub struct BUF0CR_SPEC;
impl crate::RegisterSpec for BUF0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf0cr::R](R) reader structure"]
impl crate::Readable for BUF0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf0cr::W](W) writer structure"]
impl crate::Writable for BUF0CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF0CR to value 0"]
impl crate::Resettable for BUF0CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
