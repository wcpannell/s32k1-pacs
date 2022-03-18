#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFF` reader - IP Command Transaction Finished Flag"]
pub struct TFF_R(crate::FieldReader<bool, bool>);
impl TFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFF` writer - IP Command Transaction Finished Flag"]
pub struct TFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `IPIEF` reader - IP Command Trigger could not be executed Error Flag"]
pub struct IPIEF_R(crate::FieldReader<bool, bool>);
impl IPIEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPIEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPIEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPIEF` writer - IP Command Trigger could not be executed Error Flag"]
pub struct IPIEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPIEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IPAEF` reader - IP Command Trigger during AHB Access Error Flag"]
pub struct IPAEF_R(crate::FieldReader<bool, bool>);
impl IPAEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPAEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPAEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPAEF` writer - IP Command Trigger during AHB Access Error Flag"]
pub struct IPAEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPAEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ABOF` reader - AHB Buffer Overflow Flag"]
pub struct ABOF_R(crate::FieldReader<bool, bool>);
impl ABOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABOF` writer - AHB Buffer Overflow Flag"]
pub struct ABOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `AIBSEF` reader - AHB Illegal Burst Size Error Flag"]
pub struct AIBSEF_R(crate::FieldReader<bool, bool>);
impl AIBSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIBSEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIBSEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIBSEF` writer - AHB Illegal Burst Size Error Flag"]
pub struct AIBSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> AIBSEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `AITEF` reader - AHB Illegal transaction error flag"]
pub struct AITEF_R(crate::FieldReader<bool, bool>);
impl AITEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AITEF` writer - AHB Illegal transaction error flag"]
pub struct AITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> AITEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ABSEF` reader - AHB Sequence Error Flag"]
pub struct ABSEF_R(crate::FieldReader<bool, bool>);
impl ABSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABSEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABSEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABSEF` writer - AHB Sequence Error Flag"]
pub struct ABSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RBDF` reader - RX Buffer Drain Flag"]
pub struct RBDF_R(crate::FieldReader<bool, bool>);
impl RBDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBDF` writer - RX Buffer Drain Flag"]
pub struct RBDF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RBOF` reader - RX Buffer Overflow Flag"]
pub struct RBOF_R(crate::FieldReader<bool, bool>);
impl RBOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBOF` writer - RX Buffer Overflow Flag"]
pub struct RBOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ILLINE` reader - Illegal Instruction Error Flag"]
pub struct ILLINE_R(crate::FieldReader<bool, bool>);
impl ILLINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ILLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILLINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILLINE` writer - Illegal Instruction Error Flag"]
pub struct ILLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLINE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TBUF` reader - TX Buffer Underrun Flag"]
pub struct TBUF_R(crate::FieldReader<bool, bool>);
impl TBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBUF` writer - TX Buffer Underrun Flag"]
pub struct TBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TBFF` reader - TX Buffer Fill Flag"]
pub struct TBFF_R(crate::FieldReader<bool, bool>);
impl TBFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBFF` writer - TX Buffer Fill Flag"]
pub struct TBFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IP Command Transaction Finished Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP Command Trigger could not be executed Error Flag"]
    #[inline(always)]
    pub fn ipief(&self) -> IPIEF_R {
        IPIEF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Flag"]
    #[inline(always)]
    pub fn ipaef(&self) -> IPAEF_R {
        IPAEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Flag"]
    #[inline(always)]
    pub fn abof(&self) -> ABOF_R {
        ABOF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Error Flag"]
    #[inline(always)]
    pub fn aibsef(&self) -> AIBSEF_R {
        AIBSEF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB Illegal transaction error flag"]
    #[inline(always)]
    pub fn aitef(&self) -> AITEF_R {
        AITEF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB Sequence Error Flag"]
    #[inline(always)]
    pub fn absef(&self) -> ABSEF_R {
        ABSEF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Drain Flag"]
    #[inline(always)]
    pub fn rbdf(&self) -> RBDF_R {
        RBDF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rbof(&self) -> RBOF_R {
        RBOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Illegal Instruction Error Flag"]
    #[inline(always)]
    pub fn illine(&self) -> ILLINE_R {
        ILLINE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TX Buffer Underrun Flag"]
    #[inline(always)]
    pub fn tbuf(&self) -> TBUF_R {
        TBUF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Fill Flag"]
    #[inline(always)]
    pub fn tbff(&self) -> TBFF_R {
        TBFF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP Command Transaction Finished Flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W { w: self }
    }
    #[doc = "Bit 6 - IP Command Trigger could not be executed Error Flag"]
    #[inline(always)]
    pub fn ipief(&mut self) -> IPIEF_W {
        IPIEF_W { w: self }
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Flag"]
    #[inline(always)]
    pub fn ipaef(&mut self) -> IPAEF_W {
        IPAEF_W { w: self }
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Flag"]
    #[inline(always)]
    pub fn abof(&mut self) -> ABOF_W {
        ABOF_W { w: self }
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Error Flag"]
    #[inline(always)]
    pub fn aibsef(&mut self) -> AIBSEF_W {
        AIBSEF_W { w: self }
    }
    #[doc = "Bit 14 - AHB Illegal transaction error flag"]
    #[inline(always)]
    pub fn aitef(&mut self) -> AITEF_W {
        AITEF_W { w: self }
    }
    #[doc = "Bit 15 - AHB Sequence Error Flag"]
    #[inline(always)]
    pub fn absef(&mut self) -> ABSEF_W {
        ABSEF_W { w: self }
    }
    #[doc = "Bit 16 - RX Buffer Drain Flag"]
    #[inline(always)]
    pub fn rbdf(&mut self) -> RBDF_W {
        RBDF_W { w: self }
    }
    #[doc = "Bit 17 - RX Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rbof(&mut self) -> RBOF_W {
        RBOF_W { w: self }
    }
    #[doc = "Bit 23 - Illegal Instruction Error Flag"]
    #[inline(always)]
    pub fn illine(&mut self) -> ILLINE_W {
        ILLINE_W { w: self }
    }
    #[doc = "Bit 26 - TX Buffer Underrun Flag"]
    #[inline(always)]
    pub fn tbuf(&mut self) -> TBUF_W {
        TBUF_W { w: self }
    }
    #[doc = "Bit 27 - TX Buffer Fill Flag"]
    #[inline(always)]
    pub fn tbff(&mut self) -> TBFF_W {
        TBFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR to value 0x0800_0000"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_0000
    }
}
