#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Module Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_ACC` reader - IP Access. Asserted when transaction currently executed was initiated by IP bus."]
pub struct IP_ACC_R(crate::FieldReader<bool, bool>);
impl IP_ACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IP_ACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_ACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_ACC` reader - AHB Access. Asserted when the transaction currently executed was initiated by AHB bus."]
pub struct AHB_ACC_R(crate::FieldReader<bool, bool>);
impl AHB_ACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_ACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_ACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBTRN` reader - AHB Access Transaction pending"]
pub struct AHBTRN_R(crate::FieldReader<bool, bool>);
impl AHBTRN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBTRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBTRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB0NE` reader - AHB 0 Buffer Not Empty. Asserted when AHB 0 buffer contains data."]
pub struct AHB0NE_R(crate::FieldReader<bool, bool>);
impl AHB0NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB0NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB0NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB1NE` reader - AHB 1 Buffer Not Empty. Asserted when AHB 1 buffer contains data."]
pub struct AHB1NE_R(crate::FieldReader<bool, bool>);
impl AHB1NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB1NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB1NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB2NE` reader - AHB 2 Buffer Not Empty. Asserted when AHB 2 buffer contains data."]
pub struct AHB2NE_R(crate::FieldReader<bool, bool>);
impl AHB2NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB2NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB2NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB3NE` reader - AHB 3 Buffer Not Empty. Asserted when AHB 3 buffer contains data."]
pub struct AHB3NE_R(crate::FieldReader<bool, bool>);
impl AHB3NE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB3NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB3NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB0FUL` reader - AHB 0 Buffer Full. Asserted when AHB 0 buffer is full."]
pub struct AHB0FUL_R(crate::FieldReader<bool, bool>);
impl AHB0FUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB0FUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB0FUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB1FUL` reader - AHB 1 Buffer Full. Asserted when AHB 1 buffer is full."]
pub struct AHB1FUL_R(crate::FieldReader<bool, bool>);
impl AHB1FUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB1FUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB1FUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB2FUL` reader - AHB 2 Buffer Full. Asserted when AHB 2 buffer is full."]
pub struct AHB2FUL_R(crate::FieldReader<bool, bool>);
impl AHB2FUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB2FUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB2FUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB3FUL` reader - AHB 3 Buffer Full. Asserted when AHB 3 buffer is full."]
pub struct AHB3FUL_R(crate::FieldReader<bool, bool>);
impl AHB3FUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB3FUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB3FUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWE` reader - RX Buffer Watermark Exceeded"]
pub struct RXWE_R(crate::FieldReader<bool, bool>);
impl RXWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULL` reader - RX Buffer Full"]
pub struct RXFULL_R(crate::FieldReader<bool, bool>);
impl RXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA` reader - RX Buffer DMA. Asserted when RX Buffer read out via DMA is active i.e DMA is requested or running."]
pub struct RXDMA_R(crate::FieldReader<bool, bool>);
impl RXDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEDA` reader - Tx Buffer Enough Data Available"]
pub struct TXEDA_R(crate::FieldReader<bool, bool>);
impl TXEDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXWA` reader - TX Buffer watermark Available"]
pub struct TXWA_R(crate::FieldReader<bool, bool>);
impl TXWA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXWA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXWA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA` reader - TXDMA"]
pub struct TXDMA_R(crate::FieldReader<bool, bool>);
impl TXDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFULL` reader - TX Buffer Full. Asserted when no more data can be stored."]
pub struct TXFULL_R(crate::FieldReader<bool, bool>);
impl TXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Module Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP Access. Asserted when transaction currently executed was initiated by IP bus."]
    #[inline(always)]
    pub fn ip_acc(&self) -> IP_ACC_R {
        IP_ACC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Access. Asserted when the transaction currently executed was initiated by AHB bus."]
    #[inline(always)]
    pub fn ahb_acc(&self) -> AHB_ACC_R {
        AHB_ACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AHB Access Transaction pending"]
    #[inline(always)]
    pub fn ahbtrn(&self) -> AHBTRN_R {
        AHBTRN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AHB 0 Buffer Not Empty. Asserted when AHB 0 buffer contains data."]
    #[inline(always)]
    pub fn ahb0ne(&self) -> AHB0NE_R {
        AHB0NE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AHB 1 Buffer Not Empty. Asserted when AHB 1 buffer contains data."]
    #[inline(always)]
    pub fn ahb1ne(&self) -> AHB1NE_R {
        AHB1NE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AHB 2 Buffer Not Empty. Asserted when AHB 2 buffer contains data."]
    #[inline(always)]
    pub fn ahb2ne(&self) -> AHB2NE_R {
        AHB2NE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AHB 3 Buffer Not Empty. Asserted when AHB 3 buffer contains data."]
    #[inline(always)]
    pub fn ahb3ne(&self) -> AHB3NE_R {
        AHB3NE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AHB 0 Buffer Full. Asserted when AHB 0 buffer is full."]
    #[inline(always)]
    pub fn ahb0ful(&self) -> AHB0FUL_R {
        AHB0FUL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB 1 Buffer Full. Asserted when AHB 1 buffer is full."]
    #[inline(always)]
    pub fn ahb1ful(&self) -> AHB1FUL_R {
        AHB1FUL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB 2 Buffer Full. Asserted when AHB 2 buffer is full."]
    #[inline(always)]
    pub fn ahb2ful(&self) -> AHB2FUL_R {
        AHB2FUL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB 3 Buffer Full. Asserted when AHB 3 buffer is full."]
    #[inline(always)]
    pub fn ahb3ful(&self) -> AHB3FUL_R {
        AHB3FUL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Watermark Exceeded"]
    #[inline(always)]
    pub fn rxwe(&self) -> RXWE_R {
        RXWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RX Buffer DMA. Asserted when RX Buffer read out via DMA is active i.e DMA is requested or running."]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tx Buffer Enough Data Available"]
    #[inline(always)]
    pub fn txeda(&self) -> TXEDA_R {
        TXEDA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TX Buffer watermark Available"]
    #[inline(always)]
    pub fn txwa(&self) -> TXWA_R {
        TXWA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Full. Asserted when no more data can be stored."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x0200_3800"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_3800
    }
}
