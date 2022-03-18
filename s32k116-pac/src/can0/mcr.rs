#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXMB` reader - Number Of The Last Message Buffer"]
pub struct MAXMB_R(crate::FieldReader<u8, u8>);
impl MAXMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXMB` writer - Number Of The Last Message Buffer"]
pub struct MAXMB_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00 = 0,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01 = 1,
    #[doc = "2: Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10 = 2,
    #[doc = "3: Format D: All frames rejected."]
    _11 = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDAM` reader - ID Acceptance Mode"]
pub struct IDAM_R(crate::FieldReader<u8, IDAM_A>);
impl IDAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::_00,
            1 => IDAM_A::_01,
            2 => IDAM_A::_10,
            3 => IDAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == IDAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == IDAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == IDAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == IDAM_A::_11
    }
}
impl core::ops::Deref for IDAM_R {
    type Target = crate::FieldReader<u8, IDAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDAM` writer - ID Acceptance Mode"]
pub struct IDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAM_A::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAM_A::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAM_A::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "CAN FD operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDEN_A {
    #[doc = "1: CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    _1 = 1,
    #[doc = "0: CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    _0 = 0,
}
impl From<FDEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDEN` reader - CAN FD operation enable"]
pub struct FDEN_R(crate::FieldReader<bool, FDEN_A>);
impl FDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDEN_A {
        match self.bits {
            true => FDEN_A::_1,
            false => FDEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FDEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FDEN_A::_0
    }
}
impl core::ops::Deref for FDEN_R {
    type Target = crate::FieldReader<bool, FDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDEN` writer - CAN FD operation enable"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDEN_A::_1)
    }
    #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDEN_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled."]
    _0 = 0,
    #[doc = "1: Abort enabled."]
    _1 = 1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEN` reader - Abort Enable"]
pub struct AEN_R(crate::FieldReader<bool, AEN_A>);
impl AEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::_0,
            true => AEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AEN_A::_1
    }
}
impl core::ops::Deref for AEN_R {
    type Target = crate::FieldReader<bool, AEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AEN` writer - Abort Enable"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Abort disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEN_A::_0)
    }
    #[doc = "Abort enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled."]
    _0 = 0,
    #[doc = "1: Local Priority enabled."]
    _1 = 1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRIOEN` reader - Local Priority Enable"]
pub struct LPRIOEN_R(crate::FieldReader<bool, LPRIOEN_A>);
impl LPRIOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPRIOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::_0,
            true => LPRIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPRIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPRIOEN_A::_1
    }
}
impl core::ops::Deref for LPRIOEN_R {
    type Target = crate::FieldReader<bool, LPRIOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRIOEN` writer - Local Priority Enable"]
pub struct LPRIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPRIOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Local Priority disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_0)
    }
    #[doc = "Local Priority enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Pretended Networking Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PNET_EN_A {
    #[doc = "0: Pretended Networking mode is disabled."]
    _0 = 0,
    #[doc = "1: Pretended Networking mode is enabled."]
    _1 = 1,
}
impl From<PNET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PNET_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PNET_EN` reader - Pretended Networking Enable"]
pub struct PNET_EN_R(crate::FieldReader<bool, PNET_EN_A>);
impl PNET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PNET_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PNET_EN_A {
        match self.bits {
            false => PNET_EN_A::_0,
            true => PNET_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PNET_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PNET_EN_A::_1
    }
}
impl core::ops::Deref for PNET_EN_R {
    type Target = crate::FieldReader<bool, PNET_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PNET_EN` writer - Pretended Networking Enable"]
pub struct PNET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PNET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PNET_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pretended Networking mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PNET_EN_A::_0)
    }
    #[doc = "Pretended Networking mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PNET_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: DMA feature for RX FIFO disabled."]
    _0 = 0,
    #[doc = "1: DMA feature for RX FIFO enabled."]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMA_A::_1
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA feature for RX FIFO disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "DMA feature for RX FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Individual Rx Masking And Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0 = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    _1 = 1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRMQ` reader - Individual Rx Masking And Queue Enable"]
pub struct IRMQ_R(crate::FieldReader<bool, IRMQ_A>);
impl IRMQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRMQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::_0,
            true => IRMQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRMQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRMQ_A::_1
    }
}
impl core::ops::Deref for IRMQ_R {
    type Target = crate::FieldReader<bool, IRMQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRMQ` writer - Individual Rx Masking And Queue Enable"]
pub struct IRMQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRMQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRMQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRMQ_A::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRMQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDIS_A {
    #[doc = "0: Self reception enabled."]
    _0 = 0,
    #[doc = "1: Self reception disabled."]
    _1 = 1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRXDIS` reader - Self Reception Disable"]
pub struct SRXDIS_R(crate::FieldReader<bool, SRXDIS_A>);
impl SRXDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRXDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::_0,
            true => SRXDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRXDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRXDIS_A::_1
    }
}
impl core::ops::Deref for SRXDIS_R {
    type Target = crate::FieldReader<bool, SRXDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRXDIS` writer - Self Reception Disable"]
pub struct SRXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRXDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Self reception enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRXDIS_A::_0)
    }
    #[doc = "Self reception disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRXDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Low-Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    #[doc = "0: FlexCAN is not in a low-power mode."]
    _0 = 0,
    #[doc = "1: FlexCAN is in a low-power mode."]
    _1 = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - Low-Power Mode Acknowledge"]
pub struct LPMACK_R(crate::FieldReader<bool, LPMACK_A>);
impl LPMACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPMACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::_0,
            true => LPMACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPMACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPMACK_A::_1
    }
}
impl core::ops::Deref for LPMACK_R {
    type Target = crate::FieldReader<bool, LPMACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNEN_A {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0 = 0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRNEN` reader - Warning Interrupt Enable"]
pub struct WRNEN_R(crate::FieldReader<bool, WRNEN_A>);
impl WRNEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRNEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::_0,
            true => WRNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WRNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WRNEN_A::_1
    }
}
impl core::ops::Deref for WRNEN_R {
    type Target = crate::FieldReader<bool, WRNEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRNEN` writer - Warning Interrupt Enable"]
pub struct WRNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRNEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRNEN_A::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRNEN_A::_1)
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
#[doc = "Field `SUPV` reader - Supervisor Mode"]
pub struct SUPV_R(crate::FieldReader<bool, bool>);
impl SUPV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUPV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPV` writer - Supervisor Mode"]
pub struct SUPV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPV_W<'a> {
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
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACK_A {
    #[doc = "0: FlexCAN not in Freeze mode, prescaler running."]
    _0 = 0,
    #[doc = "1: FlexCAN in Freeze mode, prescaler stopped."]
    _1 = 1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZACK` reader - Freeze Mode Acknowledge"]
pub struct FRZACK_R(crate::FieldReader<bool, FRZACK_A>);
impl FRZACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::_0,
            true => FRZACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRZACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRZACK_A::_1
    }
}
impl core::ops::Deref for FRZACK_R {
    type Target = crate::FieldReader<bool, FRZACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request."]
    _0 = 0,
    #[doc = "1: Resets the registers affected by soft reset."]
    _1 = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRST` reader - Soft Reset"]
pub struct SOFTRST_R(crate::FieldReader<bool, SOFTRST_A>);
impl SOFTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFTRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::_0,
            true => SOFTRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOFTRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOFTRST_A::_1
    }
}
impl core::ops::Deref for SOFTRST_R {
    type Target = crate::FieldReader<bool, SOFTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTRST` writer - Soft Reset"]
pub struct SOFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTRST_A::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDY_A {
    #[doc = "0: FlexCAN module is either in Normal mode, Listen-Only mode or Loop-Back mode."]
    _0 = 0,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRDY` reader - FlexCAN Not Ready"]
pub struct NOTRDY_R(crate::FieldReader<bool, NOTRDY_A>);
impl NOTRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOTRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NOTRDY_A> {
        match self.bits {
            false => Some(NOTRDY_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NOTRDY_A::_0
    }
}
impl core::ops::Deref for NOTRDY_R {
    type Target = crate::FieldReader<bool, NOTRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: No Freeze mode request."]
    _0 = 0,
    #[doc = "1: Enters Freeze mode if the FRZ bit is asserted."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt FlexCAN"]
pub struct HALT_R(crate::FieldReader<bool, HALT_A>);
impl HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HALT_A::_1
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<bool, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Halt FlexCAN"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Freeze mode request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: Rx FIFO not enabled."]
    _0 = 0,
    #[doc = "1: Rx FIFO enabled."]
    _1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rx FIFO Enable"]
pub struct RFEN_R(crate::FieldReader<bool, RFEN_A>);
impl RFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFEN_A::_1
    }
}
impl core::ops::Deref for RFEN_R {
    type Target = crate::FieldReader<bool, RFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEN` writer - Rx FIFO Enable"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rx FIFO not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFEN_A::_0)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze mode."]
    _0 = 0,
    #[doc = "1: Enabled to enter Freeze mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze Enable"]
pub struct FRZ_R(crate::FieldReader<bool, FRZ_A>);
impl FRZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRZ_A::_1
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, FRZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - Freeze Enable"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FlexCAN module."]
    _0 = 0,
    #[doc = "1: Disable the FlexCAN module."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub struct MDIS_R(crate::FieldReader<bool, MDIS_A>);
impl MDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MDIS_A::_1
    }
}
impl core::ops::Deref for MDIS_R {
    type Target = crate::FieldReader<bool, MDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pretended Networking Enable"]
    #[inline(always)]
    pub fn pnet_en(&self) -> PNET_EN_R {
        PNET_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&mut self) -> MAXMB_W {
        MAXMB_W { w: self }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&mut self) -> IDAM_W {
        IDAM_W { w: self }
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&mut self) -> LPRIOEN_W {
        LPRIOEN_W { w: self }
    }
    #[doc = "Bit 14 - Pretended Networking Enable"]
    #[inline(always)]
    pub fn pnet_en(&mut self) -> PNET_EN_W {
        PNET_EN_W { w: self }
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&mut self) -> IRMQ_W {
        IRMQ_W { w: self }
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&mut self) -> SRXDIS_W {
        SRXDIS_W { w: self }
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&mut self) -> WRNEN_W {
        WRNEN_W { w: self }
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&mut self) -> SUPV_W {
        SUPV_W { w: self }
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W {
        SOFTRST_W { w: self }
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0xd890_000f"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd890_000f
    }
}
