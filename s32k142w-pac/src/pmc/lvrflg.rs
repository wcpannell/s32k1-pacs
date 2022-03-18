#[doc = "Register `LVRFLG` reader"]
pub struct R(crate::R<LVRFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVRFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVRFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVRFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVRFLG` writer"]
pub struct W(crate::W<LVRFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVRFLG_SPEC>;
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
impl From<crate::W<LVRFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVRFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVRF` reader - LVR Core Flag"]
pub struct LVRF_R(crate::FieldReader<bool, bool>);
impl LVRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRF` writer - LVR Core Flag"]
pub struct LVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `LVRLPF` reader - LVR in low power mode core Flag"]
pub struct LVRLPF_R(crate::FieldReader<bool, bool>);
impl LVRLPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVRLPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVRLPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRLPF` writer - LVR in low power mode core Flag"]
pub struct LVRLPF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRLPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LVRXF` reader - LVR External Flag"]
pub struct LVRXF_R(crate::FieldReader<bool, bool>);
impl LVRXF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVRXF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVRXF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRXF` writer - LVR External Flag"]
pub struct LVRXF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRXF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `LVRXLPF` reader - LVR external in low power mode flag"]
pub struct LVRXLPF_R(crate::FieldReader<bool, bool>);
impl LVRXLPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVRXLPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVRXLPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRXLPF` writer - LVR external in low power mode flag"]
pub struct LVRXLPF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRXLPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `LVR3F` reader - LVR 3V Flag"]
pub struct LVR3F_R(crate::FieldReader<bool, bool>);
impl LVR3F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVR3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVR3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVR3F` writer - LVR 3V Flag"]
pub struct LVR3F_W<'a> {
    w: &'a mut W,
}
impl<'a> LVR3F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LVR3FLSF` reader - LVR 3V Flash memory Flag"]
pub struct LVR3FLSF_R(crate::FieldReader<bool, bool>);
impl LVR3FLSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVR3FLSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVR3FLSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVR3FLSF` writer - LVR 3V Flash memory Flag"]
pub struct LVR3FLSF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVR3FLSF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PORF` reader - POR Flag"]
pub struct PORF_R(crate::FieldReader<bool, bool>);
impl PORF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORF` writer - POR Flag"]
pub struct PORF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LVR Core Flag"]
    #[inline(always)]
    pub fn lvrf(&self) -> LVRF_R {
        LVRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LVR in low power mode core Flag"]
    #[inline(always)]
    pub fn lvrlpf(&self) -> LVRLPF_R {
        LVRLPF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LVR External Flag"]
    #[inline(always)]
    pub fn lvrxf(&self) -> LVRXF_R {
        LVRXF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LVR external in low power mode flag"]
    #[inline(always)]
    pub fn lvrxlpf(&self) -> LVRXLPF_R {
        LVRXLPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LVR 3V Flag"]
    #[inline(always)]
    pub fn lvr3f(&self) -> LVR3F_R {
        LVR3F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LVR 3V Flash memory Flag"]
    #[inline(always)]
    pub fn lvr3flsf(&self) -> LVR3FLSF_R {
        LVR3FLSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - POR Flag"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LVR Core Flag"]
    #[inline(always)]
    pub fn lvrf(&mut self) -> LVRF_W {
        LVRF_W { w: self }
    }
    #[doc = "Bit 1 - LVR in low power mode core Flag"]
    #[inline(always)]
    pub fn lvrlpf(&mut self) -> LVRLPF_W {
        LVRLPF_W { w: self }
    }
    #[doc = "Bit 2 - LVR External Flag"]
    #[inline(always)]
    pub fn lvrxf(&mut self) -> LVRXF_W {
        LVRXF_W { w: self }
    }
    #[doc = "Bit 3 - LVR external in low power mode flag"]
    #[inline(always)]
    pub fn lvrxlpf(&mut self) -> LVRXLPF_W {
        LVRXLPF_W { w: self }
    }
    #[doc = "Bit 4 - LVR 3V Flag"]
    #[inline(always)]
    pub fn lvr3f(&mut self) -> LVR3F_W {
        LVR3F_W { w: self }
    }
    #[doc = "Bit 5 - LVR 3V Flash memory Flag"]
    #[inline(always)]
    pub fn lvr3flsf(&mut self) -> LVR3FLSF_W {
        LVR3FLSF_W { w: self }
    }
    #[doc = "Bit 7 - POR Flag"]
    #[inline(always)]
    pub fn porf(&mut self) -> PORF_W {
        PORF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Reset Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvrflg](index.html) module"]
pub struct LVRFLG_SPEC;
impl crate::RegisterSpec for LVRFLG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvrflg::R](R) reader structure"]
impl crate::Readable for LVRFLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvrflg::W](W) writer structure"]
impl crate::Writable for LVRFLG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVRFLG to value 0"]
impl crate::Resettable for LVRFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
