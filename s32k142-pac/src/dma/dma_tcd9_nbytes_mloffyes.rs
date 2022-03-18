#[doc = "Register `TCD9_NBYTES_MLOFFYES` reader"]
pub struct R(crate::R<DMA_TCD9_NBYTES_MLOFFYES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TCD9_NBYTES_MLOFFYES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TCD9_NBYTES_MLOFFYES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TCD9_NBYTES_MLOFFYES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD9_NBYTES_MLOFFYES` writer"]
pub struct W(crate::W<DMA_TCD9_NBYTES_MLOFFYES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TCD9_NBYTES_MLOFFYES_SPEC>;
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
impl From<crate::W<DMA_TCD9_NBYTES_MLOFFYES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TCD9_NBYTES_MLOFFYES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub struct NBYTES_R(crate::FieldReader<u16, u16>);
impl NBYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `MLOFF` reader - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
pub struct MLOFF_R(crate::FieldReader<u32, u32>);
impl MLOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MLOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MLOFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MLOFF` writer - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
pub struct MLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 10)) | ((value as u32 & 0x000f_ffff) << 10);
        self.w
    }
}
#[doc = "Destination Minor Loop Offset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the DADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the DADDR"]
    _1 = 1,
}
impl From<DMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: DMLOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMLOE` reader - Destination Minor Loop Offset enable"]
pub struct DMLOE_R(crate::FieldReader<bool, DMLOE_A>);
impl DMLOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMLOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMLOE_A {
        match self.bits {
            false => DMLOE_A::_0,
            true => DMLOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMLOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMLOE_A::_1
    }
}
impl core::ops::Deref for DMLOE_R {
    type Target = crate::FieldReader<bool, DMLOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMLOE` writer - Destination Minor Loop Offset enable"]
pub struct DMLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMLOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMLOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOE_A::_0)
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOE_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Source Minor Loop Offset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the SADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the SADDR"]
    _1 = 1,
}
impl From<SMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: SMLOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMLOE` reader - Source Minor Loop Offset Enable"]
pub struct SMLOE_R(crate::FieldReader<bool, SMLOE_A>);
impl SMLOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMLOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMLOE_A {
        match self.bits {
            false => SMLOE_A::_0,
            true => SMLOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMLOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMLOE_A::_1
    }
}
impl core::ops::Deref for SMLOE_R {
    type Target = crate::FieldReader<bool, SMLOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMLOE` writer - Source Minor Loop Offset Enable"]
pub struct SMLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMLOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMLOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMLOE_A::_0)
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMLOE_A::_1)
    }
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
    #[doc = "Bits 0:9 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:29 - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub fn mloff(&self) -> MLOFF_R {
        MLOFF_R::new(((self.bits >> 10) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&self) -> DMLOE_R {
        DMLOE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&self) -> SMLOE_R {
        SMLOE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bits 10:29 - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub fn mloff(&mut self) -> MLOFF_W {
        MLOFF_W { w: self }
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&mut self) -> DMLOE_W {
        DMLOE_W { w: self }
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&mut self) -> SMLOE_W {
        SMLOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tcd9_nbytes_mloffyes](index.html) module"]
pub struct DMA_TCD9_NBYTES_MLOFFYES_SPEC;
impl crate::RegisterSpec for DMA_TCD9_NBYTES_MLOFFYES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tcd9_nbytes_mloffyes::R](R) reader structure"]
impl crate::Readable for DMA_TCD9_NBYTES_MLOFFYES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tcd9_nbytes_mloffyes::W](W) writer structure"]
impl crate::Writable for DMA_TCD9_NBYTES_MLOFFYES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD9_NBYTES_MLOFFYES to value 0"]
impl crate::Resettable for DMA_TCD9_NBYTES_MLOFFYES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
