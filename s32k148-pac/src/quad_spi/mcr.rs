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
#[doc = "Software reset for serial flash domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTSD_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    _1 = 1,
}
impl From<SWRSTSD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTSD` reader - Software reset for serial flash domain"]
pub struct SWRSTSD_R(crate::FieldReader<bool, SWRSTSD_A>);
impl SWRSTSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTSD_A {
        match self.bits {
            false => SWRSTSD_A::_0,
            true => SWRSTSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWRSTSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWRSTSD_A::_1
    }
}
impl core::ops::Deref for SWRSTSD_R {
    type Target = crate::FieldReader<bool, SWRSTSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTSD` writer - Software reset for serial flash domain"]
pub struct SWRSTSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTSD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTSD_A::_0)
    }
    #[doc = "Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTSD_A::_1)
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
#[doc = "Software reset for AHB domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTHD_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    _1 = 1,
}
impl From<SWRSTHD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTHD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTHD` reader - Software reset for AHB domain"]
pub struct SWRSTHD_R(crate::FieldReader<bool, SWRSTHD_A>);
impl SWRSTHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRSTHD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTHD_A {
        match self.bits {
            false => SWRSTHD_A::_0,
            true => SWRSTHD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWRSTHD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWRSTHD_A::_1
    }
}
impl core::ops::Deref for SWRSTHD_R {
    type Target = crate::FieldReader<bool, SWRSTHD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRSTHD` writer - Software reset for AHB domain"]
pub struct SWRSTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTHD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTHD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTHD_A::_0)
    }
    #[doc = "AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTHD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `END_CFG` reader - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
pub struct END_CFG_R(crate::FieldReader<u8, u8>);
impl END_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        END_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_CFG` writer - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
pub struct END_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> END_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "This field is valid when Data Strobe is also used as an output from controller during Write data phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_OUT_EN_A {
    #[doc = "0: DQS as an output from controller is disabled"]
    _0 = 0,
    #[doc = "1: DQS as an output from controller is enabled"]
    _1 = 1,
}
impl From<DQS_OUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DQS_OUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQS_OUT_EN` reader - This field is valid when Data Strobe is also used as an output from controller during Write data phase"]
pub struct DQS_OUT_EN_R(crate::FieldReader<bool, DQS_OUT_EN_A>);
impl DQS_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DQS_OUT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQS_OUT_EN_A {
        match self.bits {
            false => DQS_OUT_EN_A::_0,
            true => DQS_OUT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQS_OUT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQS_OUT_EN_A::_1
    }
}
impl core::ops::Deref for DQS_OUT_EN_R {
    type Target = crate::FieldReader<bool, DQS_OUT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQS_OUT_EN` writer - This field is valid when Data Strobe is also used as an output from controller during Write data phase"]
pub struct DQS_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_OUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQS_OUT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DQS as an output from controller is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_OUT_EN_A::_0)
    }
    #[doc = "DQS as an output from controller is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_OUT_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "DQS Latency Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_LAT_EN_A {
    #[doc = "0: DQS Latency disabled"]
    _0 = 0,
    #[doc = "1: DQS feature with latency included enabled"]
    _1 = 1,
}
impl From<DQS_LAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DQS_LAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQS_LAT_EN` reader - DQS Latency Enable"]
pub struct DQS_LAT_EN_R(crate::FieldReader<bool, DQS_LAT_EN_A>);
impl DQS_LAT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DQS_LAT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQS_LAT_EN_A {
        match self.bits {
            false => DQS_LAT_EN_A::_0,
            true => DQS_LAT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQS_LAT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQS_LAT_EN_A::_1
    }
}
impl core::ops::Deref for DQS_LAT_EN_R {
    type Target = crate::FieldReader<bool, DQS_LAT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQS_LAT_EN` writer - DQS Latency Enable"]
pub struct DQS_LAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_LAT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQS_LAT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DQS Latency disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_LAT_EN_A::_0)
    }
    #[doc = "DQS feature with latency included enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_LAT_EN_A::_1)
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
#[doc = "DQS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_EN_A {
    #[doc = "0: DQS disabled."]
    _0 = 0,
    #[doc = "1: DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\]
is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\]
is 0. The QSPI_SMPR\\[DDR_SMP\\]
values are ignored."]
    _1 = 1,
}
impl From<DQS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DQS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQS_EN` reader - DQS enable"]
pub struct DQS_EN_R(crate::FieldReader<bool, DQS_EN_A>);
impl DQS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DQS_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQS_EN_A {
        match self.bits {
            false => DQS_EN_A::_0,
            true => DQS_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQS_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQS_EN_A::_1
    }
}
impl core::ops::Deref for DQS_EN_R {
    type Target = crate::FieldReader<bool, DQS_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQS_EN` writer - DQS enable"]
pub struct DQS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQS_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DQS disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_EN_A::_0)
    }
    #[doc = "DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\]
is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\]
is 0. The QSPI_SMPR\\[DDR_SMP\\]
values are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_EN_A::_1)
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
#[doc = "DDR mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR_EN_A {
    #[doc = "0: 2x clock are disabled for SDR instructions only"]
    _0 = 0,
    #[doc = "1: 2x clock are enabled supports both SDR and DDR instruction."]
    _1 = 1,
}
impl From<DDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR_EN` reader - DDR mode enable"]
pub struct DDR_EN_R(crate::FieldReader<bool, DDR_EN_A>);
impl DDR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR_EN_A {
        match self.bits {
            false => DDR_EN_A::_0,
            true => DDR_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DDR_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DDR_EN_A::_1
    }
}
impl core::ops::Deref for DDR_EN_R {
    type Target = crate::FieldReader<bool, DDR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_EN` writer - DDR mode enable"]
pub struct DDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "2x clock are disabled for SDR instructions only"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DDR_EN_A::_0)
    }
    #[doc = "2x clock are enabled supports both SDR and DDR instruction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DDR_EN_A::_1)
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
#[doc = "This field is used to enable variable latency feature in the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VAR_LAT_EN_A {
    #[doc = "0: Fixed latency: Twice + 1 latency enable"]
    _0 = 0,
    #[doc = "1: Variable latency: 'once' or 'twice + 1' the initial latency based on Data strobe during CA phase. If enabled also need to ensure QSPI_FLSHCR\\[TCSS\\]
must be >= 2."]
    _1 = 1,
}
impl From<VAR_LAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VAR_LAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAR_LAT_EN` reader - This field is used to enable variable latency feature in the controller"]
pub struct VAR_LAT_EN_R(crate::FieldReader<bool, VAR_LAT_EN_A>);
impl VAR_LAT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VAR_LAT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAR_LAT_EN_A {
        match self.bits {
            false => VAR_LAT_EN_A::_0,
            true => VAR_LAT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VAR_LAT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VAR_LAT_EN_A::_1
    }
}
impl core::ops::Deref for VAR_LAT_EN_R {
    type Target = crate::FieldReader<bool, VAR_LAT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAR_LAT_EN` writer - This field is used to enable variable latency feature in the controller"]
pub struct VAR_LAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VAR_LAT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VAR_LAT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed latency: Twice + 1 latency enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VAR_LAT_EN_A::_0)
    }
    #[doc = "Variable latency: 'once' or 'twice + 1' the initial latency based on Data strobe during CA phase. If enabled also need to ensure QSPI_FLSHCR\\[TCSS\\]
must be >= 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VAR_LAT_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_RXF_AW {
    #[doc = "0: No action."]
    _0 = 0,
    #[doc = "1: Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\]
is reset to 0."]
    _1 = 1,
}
impl From<CLR_RXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_RXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_RXF` writer - Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field."]
pub struct CLR_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_RXF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_0)
    }
    #[doc = "Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\]
is reset to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_TXF_AW {
    #[doc = "0: No action."]
    _0 = 0,
    #[doc = "1: Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\]
is reset to 0."]
    _1 = 1,
}
impl From<CLR_TXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_TXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_TXF` writer - Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field."]
pub struct CLR_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_TXF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_0)
    }
    #[doc = "Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\]
is reset to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_1)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable QuadSPI clocks."]
    _0 = 0,
    #[doc = "1: Allow external logic to disable QuadSPI clocks."]
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
    #[doc = "Enable QuadSPI clocks."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Allow external logic to disable QuadSPI clocks."]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_A {
    #[doc = "0: A doze request will be ignored by the QuadSPI module"]
    _0 = 0,
    #[doc = "1: A doze request will be processed by the QuadSPI module"]
    _1 = 1,
}
impl From<DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZE` reader - Doze Enable"]
pub struct DOZE_R(crate::FieldReader<bool, DOZE_A>);
impl DOZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_A {
        match self.bits {
            false => DOZE_A::_0,
            true => DOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOZE_A::_1
    }
}
impl core::ops::Deref for DOZE_R {
    type Target = crate::FieldReader<bool, DOZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZE` writer - Doze Enable"]
pub struct DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A doze request will be ignored by the QuadSPI module"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_A::_0)
    }
    #[doc = "A doze request will be processed by the QuadSPI module"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_A::_1)
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
#[doc = "Idle Signal Drive IOFA\\[2\\]
Flash A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISD2FA_A {
    #[doc = "0: IOFA\\[2\\]
is driven to logic L"]
    _0 = 0,
    #[doc = "1: IOFA\\[2\\]
is driven to logic H"]
    _1 = 1,
}
impl From<ISD2FA_A> for bool {
    #[inline(always)]
    fn from(variant: ISD2FA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISD2FA` reader - Idle Signal Drive IOFA\\[2\\]
Flash A"]
pub struct ISD2FA_R(crate::FieldReader<bool, ISD2FA_A>);
impl ISD2FA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISD2FA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISD2FA_A {
        match self.bits {
            false => ISD2FA_A::_0,
            true => ISD2FA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISD2FA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISD2FA_A::_1
    }
}
impl core::ops::Deref for ISD2FA_R {
    type Target = crate::FieldReader<bool, ISD2FA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISD2FA` writer - Idle Signal Drive IOFA\\[2\\]
Flash A"]
pub struct ISD2FA_W<'a> {
    w: &'a mut W,
}
impl<'a> ISD2FA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISD2FA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IOFA\\[2\\]
is driven to logic L"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISD2FA_A::_0)
    }
    #[doc = "IOFA\\[2\\]
is driven to logic H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISD2FA_A::_1)
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
#[doc = "Idle Signal Drive IOFA\\[3\\]
Flash A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISD3FA_A {
    #[doc = "0: IOFA\\[3\\]
is driven to logic L"]
    _0 = 0,
    #[doc = "1: IOFA\\[3\\]
is driven to logic H"]
    _1 = 1,
}
impl From<ISD3FA_A> for bool {
    #[inline(always)]
    fn from(variant: ISD3FA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISD3FA` reader - Idle Signal Drive IOFA\\[3\\]
Flash A"]
pub struct ISD3FA_R(crate::FieldReader<bool, ISD3FA_A>);
impl ISD3FA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISD3FA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISD3FA_A {
        match self.bits {
            false => ISD3FA_A::_0,
            true => ISD3FA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISD3FA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISD3FA_A::_1
    }
}
impl core::ops::Deref for ISD3FA_R {
    type Target = crate::FieldReader<bool, ISD3FA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISD3FA` writer - Idle Signal Drive IOFA\\[3\\]
Flash A"]
pub struct ISD3FA_W<'a> {
    w: &'a mut W,
}
impl<'a> ISD3FA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISD3FA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IOFA\\[3\\]
is driven to logic L"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISD3FA_A::_0)
    }
    #[doc = "IOFA\\[3\\]
is driven to logic H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISD3FA_A::_1)
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
#[doc = "Idle Signal Drive IOFB\\[2\\]
Flash B\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISD2FB_A {
    #[doc = "0: IOFB\\[2\\]
is driven to logic L"]
    _0 = 0,
    #[doc = "1: IOFB\\[2\\]
is driven to logic H"]
    _1 = 1,
}
impl From<ISD2FB_A> for bool {
    #[inline(always)]
    fn from(variant: ISD2FB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISD2FB` reader - Idle Signal Drive IOFB\\[2\\]
Flash B"]
pub struct ISD2FB_R(crate::FieldReader<bool, ISD2FB_A>);
impl ISD2FB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISD2FB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISD2FB_A {
        match self.bits {
            false => ISD2FB_A::_0,
            true => ISD2FB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISD2FB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISD2FB_A::_1
    }
}
impl core::ops::Deref for ISD2FB_R {
    type Target = crate::FieldReader<bool, ISD2FB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISD2FB` writer - Idle Signal Drive IOFB\\[2\\]
Flash B"]
pub struct ISD2FB_W<'a> {
    w: &'a mut W,
}
impl<'a> ISD2FB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISD2FB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IOFB\\[2\\]
is driven to logic L"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISD2FB_A::_0)
    }
    #[doc = "IOFB\\[2\\]
is driven to logic H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISD2FB_A::_1)
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
#[doc = "Idle Signal Drive IOFB\\[3\\]
Flash B\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISD3FB_A {
    #[doc = "0: IOFB\\[3\\]
is driven to logic L"]
    _0 = 0,
    #[doc = "1: IOFB\\[3\\]
is driven to logic H"]
    _1 = 1,
}
impl From<ISD3FB_A> for bool {
    #[inline(always)]
    fn from(variant: ISD3FB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISD3FB` reader - Idle Signal Drive IOFB\\[3\\]
Flash B"]
pub struct ISD3FB_R(crate::FieldReader<bool, ISD3FB_A>);
impl ISD3FB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISD3FB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISD3FB_A {
        match self.bits {
            false => ISD3FB_A::_0,
            true => ISD3FB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISD3FB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISD3FB_A::_1
    }
}
impl core::ops::Deref for ISD3FB_R {
    type Target = crate::FieldReader<bool, ISD3FB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISD3FB` writer - Idle Signal Drive IOFB\\[3\\]
Flash B"]
pub struct ISD3FB_W<'a> {
    w: &'a mut W,
}
impl<'a> ISD3FB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISD3FB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IOFB\\[3\\]
is driven to logic L"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISD3FB_A::_0)
    }
    #[doc = "IOFB\\[3\\]
is driven to logic H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISD3FB_A::_1)
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
#[doc = "Field `SCLKCFG` reader - Serial Clock Configuration"]
pub struct SCLKCFG_R(crate::FieldReader<u8, u8>);
impl SCLKCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLKCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLKCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLKCFG` writer - Serial Clock Configuration"]
pub struct SCLKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline(always)]
    pub fn swrstsd(&self) -> SWRSTSD_R {
        SWRSTSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline(always)]
    pub fn swrsthd(&self) -> SWRSTHD_R {
        SWRSTHD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline(always)]
    pub fn end_cfg(&self) -> END_CFG_R {
        END_CFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - This field is valid when Data Strobe is also used as an output from controller during Write data phase"]
    #[inline(always)]
    pub fn dqs_out_en(&self) -> DQS_OUT_EN_R {
        DQS_OUT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline(always)]
    pub fn dqs_lat_en(&self) -> DQS_LAT_EN_R {
        DQS_LAT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline(always)]
    pub fn dqs_en(&self) -> DQS_EN_R {
        DQS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DDR_EN_R {
        DDR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This field is used to enable variable latency feature in the controller"]
    #[inline(always)]
    pub fn var_lat_en(&self) -> VAR_LAT_EN_R {
        VAR_LAT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&self) -> DOZE_R {
        DOZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Idle Signal Drive IOFA\\[2\\]
Flash A"]
    #[inline(always)]
    pub fn isd2fa(&self) -> ISD2FA_R {
        ISD2FA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Idle Signal Drive IOFA\\[3\\]
Flash A"]
    #[inline(always)]
    pub fn isd3fa(&self) -> ISD3FA_R {
        ISD3FA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Idle Signal Drive IOFB\\[2\\]
Flash B"]
    #[inline(always)]
    pub fn isd2fb(&self) -> ISD2FB_R {
        ISD2FB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Idle Signal Drive IOFB\\[3\\]
Flash B"]
    #[inline(always)]
    pub fn isd3fb(&self) -> ISD3FB_R {
        ISD3FB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&self) -> SCLKCFG_R {
        SCLKCFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline(always)]
    pub fn swrstsd(&mut self) -> SWRSTSD_W {
        SWRSTSD_W { w: self }
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline(always)]
    pub fn swrsthd(&mut self) -> SWRSTHD_W {
        SWRSTHD_W { w: self }
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline(always)]
    pub fn end_cfg(&mut self) -> END_CFG_W {
        END_CFG_W { w: self }
    }
    #[doc = "Bit 4 - This field is valid when Data Strobe is also used as an output from controller during Write data phase"]
    #[inline(always)]
    pub fn dqs_out_en(&mut self) -> DQS_OUT_EN_W {
        DQS_OUT_EN_W { w: self }
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline(always)]
    pub fn dqs_lat_en(&mut self) -> DQS_LAT_EN_W {
        DQS_LAT_EN_W { w: self }
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline(always)]
    pub fn dqs_en(&mut self) -> DQS_EN_W {
        DQS_EN_W { w: self }
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DDR_EN_W {
        DDR_EN_W { w: self }
    }
    #[doc = "Bit 8 - This field is used to enable variable latency feature in the controller"]
    #[inline(always)]
    pub fn var_lat_en(&mut self) -> VAR_LAT_EN_W {
        VAR_LAT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field."]
    #[inline(always)]
    pub fn clr_rxf(&mut self) -> CLR_RXF_W {
        CLR_RXF_W { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field."]
    #[inline(always)]
    pub fn clr_txf(&mut self) -> CLR_TXF_W {
        CLR_TXF_W { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&mut self) -> DOZE_W {
        DOZE_W { w: self }
    }
    #[doc = "Bit 16 - Idle Signal Drive IOFA\\[2\\]
Flash A"]
    #[inline(always)]
    pub fn isd2fa(&mut self) -> ISD2FA_W {
        ISD2FA_W { w: self }
    }
    #[doc = "Bit 17 - Idle Signal Drive IOFA\\[3\\]
Flash A"]
    #[inline(always)]
    pub fn isd3fa(&mut self) -> ISD3FA_W {
        ISD3FA_W { w: self }
    }
    #[doc = "Bit 18 - Idle Signal Drive IOFB\\[2\\]
Flash B"]
    #[inline(always)]
    pub fn isd2fb(&mut self) -> ISD2FB_W {
        ISD2FB_W { w: self }
    }
    #[doc = "Bit 19 - Idle Signal Drive IOFB\\[3\\]
Flash B"]
    #[inline(always)]
    pub fn isd3fb(&mut self) -> ISD3FB_W {
        ISD3FB_W { w: self }
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&mut self) -> SCLKCFG_W {
        SCLKCFG_W { w: self }
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
#[doc = "`reset()` method sets MCR to value 0x000f_4000"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_4000
    }
}
