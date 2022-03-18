#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMESZ` reader - Frame Size"]
pub struct FRAMESZ_R(crate::FieldReader<u16, u16>);
impl FRAMESZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMESZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESZ` writer - Frame Size"]
pub struct FRAMESZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: 1 bit transfer"]
    ONEBIT = 0,
    #[doc = "1: 2 bit transfer"]
    TWOBIT = 1,
    #[doc = "2: 4 bit transfer"]
    FOURBIT = 2,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WIDTH` reader - Transfer Width"]
pub struct WIDTH_R(crate::FieldReader<u8, WIDTH_A>);
impl WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WIDTH_A> {
        match self.bits {
            0 => Some(WIDTH_A::ONEBIT),
            1 => Some(WIDTH_A::TWOBIT),
            2 => Some(WIDTH_A::FOURBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEBIT`"]
    #[inline(always)]
    pub fn is_onebit(&self) -> bool {
        **self == WIDTH_A::ONEBIT
    }
    #[doc = "Checks if the value of the field is `TWOBIT`"]
    #[inline(always)]
    pub fn is_twobit(&self) -> bool {
        **self == WIDTH_A::TWOBIT
    }
    #[doc = "Checks if the value of the field is `FOURBIT`"]
    #[inline(always)]
    pub fn is_fourbit(&self) -> bool {
        **self == WIDTH_A::FOURBIT
    }
}
impl core::ops::Deref for WIDTH_R {
    type Target = crate::FieldReader<u8, WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDTH` writer - Transfer Width"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 bit transfer"]
    #[inline(always)]
    pub fn onebit(self) -> &'a mut W {
        self.variant(WIDTH_A::ONEBIT)
    }
    #[doc = "2 bit transfer"]
    #[inline(always)]
    pub fn twobit(self) -> &'a mut W {
        self.variant(WIDTH_A::TWOBIT)
    }
    #[doc = "4 bit transfer"]
    #[inline(always)]
    pub fn fourbit(self) -> &'a mut W {
        self.variant(WIDTH_A::FOURBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Transmit Data Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSK_A {
    #[doc = "0: Normal transfer"]
    NORMAL = 0,
    #[doc = "1: Mask transmit data"]
    MASK = 1,
}
impl From<TXMSK_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXMSK` reader - Transmit Data Mask"]
pub struct TXMSK_R(crate::FieldReader<bool, TXMSK_A>);
impl TXMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMSK_A {
        match self.bits {
            false => TXMSK_A::NORMAL,
            true => TXMSK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TXMSK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == TXMSK_A::MASK
    }
}
impl core::ops::Deref for TXMSK_R {
    type Target = crate::FieldReader<bool, TXMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSK` writer - Transmit Data Mask"]
pub struct TXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal transfer"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXMSK_A::NORMAL)
    }
    #[doc = "Mask transmit data"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TXMSK_A::MASK)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Receive Data Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSK_A {
    #[doc = "0: Normal transfer"]
    NORMAL = 0,
    #[doc = "1: Receive data is masked"]
    MASK = 1,
}
impl From<RXMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RXMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXMSK` reader - Receive Data Mask"]
pub struct RXMSK_R(crate::FieldReader<bool, RXMSK_A>);
impl RXMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMSK_A {
        match self.bits {
            false => RXMSK_A::NORMAL,
            true => RXMSK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == RXMSK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == RXMSK_A::MASK
    }
}
impl core::ops::Deref for RXMSK_R {
    type Target = crate::FieldReader<bool, RXMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMSK` writer - Receive Data Mask"]
pub struct RXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal transfer"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXMSK_A::NORMAL)
    }
    #[doc = "Receive data is masked"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RXMSK_A::MASK)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Continuing Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTC_A {
    #[doc = "0: Command word for start of new transfer"]
    START = 0,
    #[doc = "1: Command word for continuing transfer"]
    CONTINUE = 1,
}
impl From<CONTC_A> for bool {
    #[inline(always)]
    fn from(variant: CONTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTC` reader - Continuing Command"]
pub struct CONTC_R(crate::FieldReader<bool, CONTC_A>);
impl CONTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTC_A {
        match self.bits {
            false => CONTC_A::START,
            true => CONTC_A::CONTINUE,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CONTC_A::START
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == CONTC_A::CONTINUE
    }
}
impl core::ops::Deref for CONTC_R {
    type Target = crate::FieldReader<bool, CONTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTC` writer - Continuing Command"]
pub struct CONTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command word for start of new transfer"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CONTC_A::START)
    }
    #[doc = "Command word for continuing transfer"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(CONTC_A::CONTINUE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Continuous Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Continuous transfer is disabled"]
    DISABLED = 0,
    #[doc = "1: Continuous transfer is enabled"]
    ENABLED = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous Transfer"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::DISABLED,
            true => CONT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CONT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CONT_A::ENABLED
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous Transfer"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Continuous transfer is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONT_A::DISABLED)
    }
    #[doc = "Continuous transfer is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYSW_A {
    #[doc = "0: Byte swap is disabled"]
    DISABLED = 0,
    #[doc = "1: Byte swap is enabled"]
    ENABLED = 1,
}
impl From<BYSW_A> for bool {
    #[inline(always)]
    fn from(variant: BYSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYSW` reader - Byte Swap"]
pub struct BYSW_R(crate::FieldReader<bool, BYSW_A>);
impl BYSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYSW_A {
        match self.bits {
            false => BYSW_A::DISABLED,
            true => BYSW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BYSW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BYSW_A::ENABLED
    }
}
impl core::ops::Deref for BYSW_R {
    type Target = crate::FieldReader<bool, BYSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYSW` writer - Byte Swap"]
pub struct BYSW_W<'a> {
    w: &'a mut W,
}
impl<'a> BYSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Byte swap is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYSW_A::DISABLED)
    }
    #[doc = "Byte swap is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYSW_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBF_A {
    #[doc = "0: Data is transferred MSB first"]
    MSB_FIRST = 0,
    #[doc = "1: Data is transferred LSB first"]
    LSB_FIRST = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - LSB First"]
pub struct LSBF_R(crate::FieldReader<bool, LSBF_A>);
impl LSBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::MSB_FIRST,
            true => LSBF_A::LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        **self == LSBF_A::MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        **self == LSBF_A::LSB_FIRST
    }
}
impl core::ops::Deref for LSBF_R {
    type Target = crate::FieldReader<bool, LSBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSBF` writer - LSB First"]
pub struct LSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is transferred MSB first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(LSBF_A::MSB_FIRST)
    }
    #[doc = "Data is transferred LSB first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(LSBF_A::LSB_FIRST)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Transfer using LPSPI_PCS\\[0\\]"]
    TX_PCS0 = 0,
    #[doc = "1: Transfer using LPSPI_PCS\\[1\\]"]
    TX_PCS1 = 1,
    #[doc = "2: Transfer using LPSPI_PCS\\[2\\]"]
    TX_PCS2 = 2,
    #[doc = "3: Transfer using LPSPI_PCS\\[3\\]"]
    TX_PCS3 = 3,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub struct PCS_R(crate::FieldReader<u8, PCS_A>);
impl PCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::TX_PCS0,
            1 => PCS_A::TX_PCS1,
            2 => PCS_A::TX_PCS2,
            3 => PCS_A::TX_PCS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PCS0`"]
    #[inline(always)]
    pub fn is_tx_pcs0(&self) -> bool {
        **self == PCS_A::TX_PCS0
    }
    #[doc = "Checks if the value of the field is `TX_PCS1`"]
    #[inline(always)]
    pub fn is_tx_pcs1(&self) -> bool {
        **self == PCS_A::TX_PCS1
    }
    #[doc = "Checks if the value of the field is `TX_PCS2`"]
    #[inline(always)]
    pub fn is_tx_pcs2(&self) -> bool {
        **self == PCS_A::TX_PCS2
    }
    #[doc = "Checks if the value of the field is `TX_PCS3`"]
    #[inline(always)]
    pub fn is_tx_pcs3(&self) -> bool {
        **self == PCS_A::TX_PCS3
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, PCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Transfer using LPSPI_PCS\\[0\\]"]
    #[inline(always)]
    pub fn tx_pcs0(self) -> &'a mut W {
        self.variant(PCS_A::TX_PCS0)
    }
    #[doc = "Transfer using LPSPI_PCS\\[1\\]"]
    #[inline(always)]
    pub fn tx_pcs1(self) -> &'a mut W {
        self.variant(PCS_A::TX_PCS1)
    }
    #[doc = "Transfer using LPSPI_PCS\\[2\\]"]
    #[inline(always)]
    pub fn tx_pcs2(self) -> &'a mut W {
        self.variant(PCS_A::TX_PCS2)
    }
    #[doc = "Transfer using LPSPI_PCS\\[3\\]"]
    #[inline(always)]
    pub fn tx_pcs3(self) -> &'a mut W {
        self.variant(PCS_A::TX_PCS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide by 1"]
    DIVIDEBY1 = 0,
    #[doc = "1: Divide by 2"]
    DIVIDEBY2 = 1,
    #[doc = "2: Divide by 4"]
    DIVIDEBY4 = 2,
    #[doc = "3: Divide by 8"]
    DIVIDEBY8 = 3,
    #[doc = "4: Divide by 16"]
    DIVIDEBY16 = 4,
    #[doc = "5: Divide by 32"]
    DIVIDEBY32 = 5,
    #[doc = "6: Divide by 64"]
    DIVIDEBY64 = 6,
    #[doc = "7: Divide by 128"]
    DIVIDEBY128 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALE` reader - Prescaler Value"]
pub struct PRESCALE_R(crate::FieldReader<u8, PRESCALE_A>);
impl PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::DIVIDEBY1,
            1 => PRESCALE_A::DIVIDEBY2,
            2 => PRESCALE_A::DIVIDEBY4,
            3 => PRESCALE_A::DIVIDEBY8,
            4 => PRESCALE_A::DIVIDEBY16,
            5 => PRESCALE_A::DIVIDEBY32,
            6 => PRESCALE_A::DIVIDEBY64,
            7 => PRESCALE_A::DIVIDEBY128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY1`"]
    #[inline(always)]
    pub fn is_divideby1(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY1
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY2
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline(always)]
    pub fn is_divideby4(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline(always)]
    pub fn is_divideby8(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY16`"]
    #[inline(always)]
    pub fn is_divideby16(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY16
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY32`"]
    #[inline(always)]
    pub fn is_divideby32(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY32
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY64`"]
    #[inline(always)]
    pub fn is_divideby64(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY64
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY128`"]
    #[inline(always)]
    pub fn is_divideby128(&self) -> bool {
        **self == PRESCALE_A::DIVIDEBY128
    }
}
impl core::ops::Deref for PRESCALE_R {
    type Target = crate::FieldReader<u8, PRESCALE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALE` writer - Prescaler Value"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divideby1(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divideby4(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divideby8(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divideby16(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divideby32(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divideby64(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn divideby128(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDEBY128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
    CAPTURED = 0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
    CHANGED = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::CAPTURED,
            true => CPHA_A::CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURED`"]
    #[inline(always)]
    pub fn is_captured(&self) -> bool {
        **self == CPHA_A::CAPTURED
    }
    #[doc = "Checks if the value of the field is `CHANGED`"]
    #[inline(always)]
    pub fn is_changed(&self) -> bool {
        **self == CPHA_A::CHANGED
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
    #[inline(always)]
    pub fn captured(self) -> &'a mut W {
        self.variant(CPHA_A::CAPTURED)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
    #[inline(always)]
    pub fn changed(self) -> &'a mut W {
        self.variant(CPHA_A::CHANGED)
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
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low"]
    INACTIVE_LOW = 0,
    #[doc = "1: The inactive state value of SCK is high"]
    INACTIVE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::INACTIVE_LOW,
            true => CPOL_A::INACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_inactive_low(&self) -> bool {
        **self == CPOL_A::INACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `INACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_inactive_high(&self) -> bool {
        **self == CPOL_A::INACTIVE_HIGH
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The inactive state value of SCK is low"]
    #[inline(always)]
    pub fn inactive_low(self) -> &'a mut W {
        self.variant(CPOL_A::INACTIVE_LOW)
    }
    #[doc = "The inactive state value of SCK is high"]
    #[inline(always)]
    pub fn inactive_high(self) -> &'a mut W {
        self.variant(CPOL_A::INACTIVE_HIGH)
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
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline(always)]
    pub fn framesz(&self) -> FRAMESZ_R {
        FRAMESZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline(always)]
    pub fn txmsk(&self) -> TXMSK_R {
        TXMSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline(always)]
    pub fn rxmsk(&self) -> RXMSK_R {
        RXMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline(always)]
    pub fn contc(&self) -> CONTC_R {
        CONTC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline(always)]
    pub fn bysw(&self) -> BYSW_R {
        BYSW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline(always)]
    pub fn framesz(&mut self) -> FRAMESZ_W {
        FRAMESZ_W { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline(always)]
    pub fn txmsk(&mut self) -> TXMSK_W {
        TXMSK_W { w: self }
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline(always)]
    pub fn rxmsk(&mut self) -> RXMSK_W {
        RXMSK_W { w: self }
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline(always)]
    pub fn contc(&mut self) -> CONTC_W {
        CONTC_W { w: self }
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline(always)]
    pub fn bysw(&mut self) -> BYSW_W {
        BYSW_W { w: self }
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W {
        LSBF_W { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR to value 0x1f"]
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
