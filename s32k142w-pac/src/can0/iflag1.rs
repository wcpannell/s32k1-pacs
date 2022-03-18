#[doc = "Register `IFLAG1` reader"]
pub struct R(crate::R<IFLAG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLAG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLAG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLAG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLAG1` writer"]
pub struct W(crate::W<IFLAG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLAG1_SPEC>;
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
impl From<crate::W<IFLAG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLAG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Buffer MB0 Interrupt Or Clear FIFO bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0I_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    BUFFER_TX_RX_NOT_COMPLETE = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    BUFFER_TX_RX_COMPLETE = 1,
}
impl From<BUF0I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUF0I` reader - Buffer MB0 Interrupt Or Clear FIFO bit"]
pub struct BUF0I_R(crate::FieldReader<bool, BUF0I_A>);
impl BUF0I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF0I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF0I_A {
        match self.bits {
            false => BUF0I_A::BUFFER_TX_RX_NOT_COMPLETE,
            true => BUF0I_A::BUFFER_TX_RX_COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFER_TX_RX_NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_buffer_tx_rx_not_complete(&self) -> bool {
        **self == BUF0I_A::BUFFER_TX_RX_NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `BUFFER_TX_RX_COMPLETE`"]
    #[inline(always)]
    pub fn is_buffer_tx_rx_complete(&self) -> bool {
        **self == BUF0I_A::BUFFER_TX_RX_COMPLETE
    }
}
impl core::ops::Deref for BUF0I_R {
    type Target = crate::FieldReader<bool, BUF0I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF0I` writer - Buffer MB0 Interrupt Or Clear FIFO bit"]
pub struct BUF0I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF0I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn buffer_tx_rx_not_complete(self) -> &'a mut W {
        self.variant(BUF0I_A::BUFFER_TX_RX_NOT_COMPLETE)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn buffer_tx_rx_complete(self) -> &'a mut W {
        self.variant(BUF0I_A::BUFFER_TX_RX_COMPLETE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BUF4TO1I` reader - Buffer MB i Interrupt Or Reserved"]
pub struct BUF4TO1I_R(crate::FieldReader<u8, u8>);
impl BUF4TO1I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUF4TO1I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF4TO1I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF4TO1I` writer - Buffer MB i Interrupt Or Reserved"]
pub struct BUF4TO1I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Buffer MB5 Interrupt Or Frames available in Rx FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5I_A {
    #[doc = "0: No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    ID2 = 0,
    #[doc = "1: MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1. It generates a DMA request in case of MCR\\[RFEN\\]
and MCR\\[DMA\\]
are enabled."]
    ID4 = 1,
}
impl From<BUF5I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF5I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUF5I` reader - Buffer MB5 Interrupt Or Frames available in Rx FIFO"]
pub struct BUF5I_R(crate::FieldReader<bool, BUF5I_A>);
impl BUF5I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF5I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF5I_A {
        match self.bits {
            false => BUF5I_A::ID2,
            true => BUF5I_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == BUF5I_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == BUF5I_A::ID4
    }
}
impl core::ops::Deref for BUF5I_R {
    type Target = crate::FieldReader<bool, BUF5I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF5I` writer - Buffer MB5 Interrupt Or Frames available in Rx FIFO"]
pub struct BUF5I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF5I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF5I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(BUF5I_A::ID2)
    }
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1. It generates a DMA request in case of MCR\\[RFEN\\]
and MCR\\[DMA\\]
are enabled."]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(BUF5I_A::ID4)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Buffer MB6 Interrupt Or Rx FIFO Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6I_A {
    #[doc = "0: No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    ID2 = 0,
    #[doc = "1: MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    ID4 = 1,
}
impl From<BUF6I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF6I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUF6I` reader - Buffer MB6 Interrupt Or Rx FIFO Warning"]
pub struct BUF6I_R(crate::FieldReader<bool, BUF6I_A>);
impl BUF6I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF6I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF6I_A {
        match self.bits {
            false => BUF6I_A::ID2,
            true => BUF6I_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == BUF6I_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == BUF6I_A::ID4
    }
}
impl core::ops::Deref for BUF6I_R {
    type Target = crate::FieldReader<bool, BUF6I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF6I` writer - Buffer MB6 Interrupt Or Rx FIFO Warning"]
pub struct BUF6I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF6I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF6I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(BUF6I_A::ID2)
    }
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(BUF6I_A::ID4)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Buffer MB7 Interrupt Or Rx FIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7I_A {
    #[doc = "0: No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    ID2 = 0,
    #[doc = "1: MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    ID4 = 1,
}
impl From<BUF7I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF7I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUF7I` reader - Buffer MB7 Interrupt Or Rx FIFO Overflow"]
pub struct BUF7I_R(crate::FieldReader<bool, BUF7I_A>);
impl BUF7I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF7I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF7I_A {
        match self.bits {
            false => BUF7I_A::ID2,
            true => BUF7I_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == BUF7I_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == BUF7I_A::ID4
    }
}
impl core::ops::Deref for BUF7I_R {
    type Target = crate::FieldReader<bool, BUF7I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF7I` writer - Buffer MB7 Interrupt Or Rx FIFO Overflow"]
pub struct BUF7I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF7I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF7I_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(BUF7I_A::ID2)
    }
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(BUF7I_A::ID4)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BUF31TO8I` reader - Buffer MBi Interrupt"]
pub struct BUF31TO8I_R(crate::FieldReader<u32, u32>);
impl BUF31TO8I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF31TO8I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF31TO8I_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF31TO8I` writer - Buffer MBi Interrupt"]
pub struct BUF31TO8I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear FIFO bit"]
    #[inline(always)]
    pub fn buf0i(&self) -> BUF0I_R {
        BUF0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or Reserved"]
    #[inline(always)]
    pub fn buf4to1i(&self) -> BUF4TO1I_R {
        BUF4TO1I_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or Frames available in Rx FIFO"]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or Rx FIFO Warning"]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or Rx FIFO Overflow"]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i(&self) -> BUF31TO8I_R {
        BUF31TO8I_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear FIFO bit"]
    #[inline(always)]
    pub fn buf0i(&mut self) -> BUF0I_W {
        BUF0I_W { w: self }
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or Reserved"]
    #[inline(always)]
    pub fn buf4to1i(&mut self) -> BUF4TO1I_W {
        BUF4TO1I_W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or Frames available in Rx FIFO"]
    #[inline(always)]
    pub fn buf5i(&mut self) -> BUF5I_W {
        BUF5I_W { w: self }
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or Rx FIFO Warning"]
    #[inline(always)]
    pub fn buf6i(&mut self) -> BUF6I_W {
        BUF6I_W { w: self }
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or Rx FIFO Overflow"]
    #[inline(always)]
    pub fn buf7i(&mut self) -> BUF7I_W {
        BUF7I_W { w: self }
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i(&mut self) -> BUF31TO8I_W {
        BUF31TO8I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag1](index.html) module"]
pub struct IFLAG1_SPEC;
impl crate::RegisterSpec for IFLAG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iflag1::R](R) reader structure"]
impl crate::Readable for IFLAG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iflag1::W](W) writer structure"]
impl crate::Writable for IFLAG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFLAG1 to value 0"]
impl crate::Resettable for IFLAG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
