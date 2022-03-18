#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSR` writer"]
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Data Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDF_A {
    #[doc = "0: Transmit data not requested."]
    _0 = 0,
    #[doc = "1: Transmit data is requested."]
    _1 = 1,
}
impl From<TDF_A> for bool {
    #[inline(always)]
    fn from(variant: TDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDF` reader - Transmit Data Flag"]
pub struct TDF_R(crate::FieldReader<bool, TDF_A>);
impl TDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDF_A {
        match self.bits {
            false => TDF_A::_0,
            true => TDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDF_A::_1
    }
}
impl core::ops::Deref for TDF_R {
    type Target = crate::FieldReader<bool, TDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDF_A {
    #[doc = "0: Receive Data is not ready."]
    _0 = 0,
    #[doc = "1: Receive data is ready."]
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDF` reader - Receive Data Flag"]
pub struct RDF_R(crate::FieldReader<bool, RDF_A>);
impl RDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDF_A::_1
    }
}
impl core::ops::Deref for RDF_R {
    type Target = crate::FieldReader<bool, RDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End Packet Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPF_A {
    #[doc = "0: Master has not generated a STOP or Repeated START condition."]
    _0 = 0,
    #[doc = "1: Master has generated a STOP or Repeated START condition."]
    _1 = 1,
}
impl From<EPF_A> for bool {
    #[inline(always)]
    fn from(variant: EPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPF` reader - End Packet Flag"]
pub struct EPF_R(crate::FieldReader<bool, EPF_A>);
impl EPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPF_A {
        match self.bits {
            false => EPF_A::_0,
            true => EPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPF_A::_1
    }
}
impl core::ops::Deref for EPF_R {
    type Target = crate::FieldReader<bool, EPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPF` writer - End Packet Flag"]
pub struct EPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master has not generated a STOP or Repeated START condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPF_A::_0)
    }
    #[doc = "Master has generated a STOP or Repeated START condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPF_A::_1)
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
#[doc = "STOP Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDF_A {
    #[doc = "0: Master has not generated a STOP condition."]
    _0 = 0,
    #[doc = "1: Master has generated a STOP condition."]
    _1 = 1,
}
impl From<SDF_A> for bool {
    #[inline(always)]
    fn from(variant: SDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDF` reader - STOP Detect Flag"]
pub struct SDF_R(crate::FieldReader<bool, SDF_A>);
impl SDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDF_A {
        match self.bits {
            false => SDF_A::_0,
            true => SDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SDF_A::_1
    }
}
impl core::ops::Deref for SDF_R {
    type Target = crate::FieldReader<bool, SDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDF` writer - STOP Detect Flag"]
pub struct SDF_W<'a> {
    w: &'a mut W,
}
impl<'a> SDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master has not generated a STOP condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDF_A::_0)
    }
    #[doc = "Master has generated a STOP condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "NACK Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDF_A {
    #[doc = "0: Unexpected NACK not detected."]
    _0 = 0,
    #[doc = "1: Unexpected NACK was detected."]
    _1 = 1,
}
impl From<NDF_A> for bool {
    #[inline(always)]
    fn from(variant: NDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDF` reader - NACK Detect Flag"]
pub struct NDF_R(crate::FieldReader<bool, NDF_A>);
impl NDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDF_A {
        match self.bits {
            false => NDF_A::_0,
            true => NDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NDF_A::_1
    }
}
impl core::ops::Deref for NDF_R {
    type Target = crate::FieldReader<bool, NDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDF` writer - NACK Detect Flag"]
pub struct NDF_W<'a> {
    w: &'a mut W,
}
impl<'a> NDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unexpected NACK not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NDF_A::_0)
    }
    #[doc = "Unexpected NACK was detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NDF_A::_1)
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
#[doc = "Arbitration Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALF_A {
    #[doc = "0: Master has not lost arbitration."]
    _0 = 0,
    #[doc = "1: Master has lost arbitration."]
    _1 = 1,
}
impl From<ALF_A> for bool {
    #[inline(always)]
    fn from(variant: ALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALF` reader - Arbitration Lost Flag"]
pub struct ALF_R(crate::FieldReader<bool, ALF_A>);
impl ALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALF_A {
        match self.bits {
            false => ALF_A::_0,
            true => ALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALF_A::_1
    }
}
impl core::ops::Deref for ALF_R {
    type Target = crate::FieldReader<bool, ALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALF` writer - Arbitration Lost Flag"]
pub struct ALF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master has not lost arbitration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALF_A::_0)
    }
    #[doc = "Master has lost arbitration."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALF_A::_1)
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
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Master sending or receiving data without START condition."]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub struct FEF_R(crate::FieldReader<bool, FEF_A>);
impl FEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEF_A::_1
    }
}
impl core::ops::Deref for FEF_R {
    type Target = crate::FieldReader<bool, FEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEF` writer - FIFO Error Flag"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Master sending or receiving data without START condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
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
#[doc = "Pin Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTF_A {
    #[doc = "0: Pin low timeout has not occurred or is disabled."]
    _0 = 0,
    #[doc = "1: Pin low timeout has occurred."]
    _1 = 1,
}
impl From<PLTF_A> for bool {
    #[inline(always)]
    fn from(variant: PLTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLTF` reader - Pin Low Timeout Flag"]
pub struct PLTF_R(crate::FieldReader<bool, PLTF_A>);
impl PLTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTF_A {
        match self.bits {
            false => PLTF_A::_0,
            true => PLTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLTF_A::_1
    }
}
impl core::ops::Deref for PLTF_R {
    type Target = crate::FieldReader<bool, PLTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLTF` writer - Pin Low Timeout Flag"]
pub struct PLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin low timeout has not occurred or is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLTF_A::_0)
    }
    #[doc = "Pin low timeout has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLTF_A::_1)
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
#[doc = "Data Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMF_A {
    #[doc = "0: Have not received matching data."]
    _0 = 0,
    #[doc = "1: Have received matching data."]
    _1 = 1,
}
impl From<DMF_A> for bool {
    #[inline(always)]
    fn from(variant: DMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMF` reader - Data Match Flag"]
pub struct DMF_R(crate::FieldReader<bool, DMF_A>);
impl DMF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMF_A {
        match self.bits {
            false => DMF_A::_0,
            true => DMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMF_A::_1
    }
}
impl core::ops::Deref for DMF_R {
    type Target = crate::FieldReader<bool, DMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMF` writer - Data Match Flag"]
pub struct DMF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Have not received matching data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMF_A::_0)
    }
    #[doc = "Have received matching data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMF_A::_1)
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
#[doc = "Master Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBF_A {
    #[doc = "0: I2C Master is idle."]
    _0 = 0,
    #[doc = "1: I2C Master is busy."]
    _1 = 1,
}
impl From<MBF_A> for bool {
    #[inline(always)]
    fn from(variant: MBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBF` reader - Master Busy Flag"]
pub struct MBF_R(crate::FieldReader<bool, MBF_A>);
impl MBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBF_A {
        match self.bits {
            false => MBF_A::_0,
            true => MBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MBF_A::_1
    }
}
impl core::ops::Deref for MBF_R {
    type Target = crate::FieldReader<bool, MBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bus Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBF_A {
    #[doc = "0: I2C Bus is idle."]
    _0 = 0,
    #[doc = "1: I2C Bus is busy."]
    _1 = 1,
}
impl From<BBF_A> for bool {
    #[inline(always)]
    fn from(variant: BBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BBF` reader - Bus Busy Flag"]
pub struct BBF_R(crate::FieldReader<bool, BBF_A>);
impl BBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBF_A {
        match self.bits {
            false => BBF_A::_0,
            true => BBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BBF_A::_1
    }
}
impl core::ops::Deref for BBF_R {
    type Target = crate::FieldReader<bool, BBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline(always)]
    pub fn tdf(&self) -> TDF_R {
        TDF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&self) -> ALF_R {
        ALF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    pub fn pltf(&self) -> PLTF_R {
        PLTF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&self) -> DMF_R {
        DMF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Master Busy Flag"]
    #[inline(always)]
    pub fn mbf(&self) -> MBF_R {
        MBF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline(always)]
    pub fn bbf(&self) -> BBF_R {
        BBF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    pub fn epf(&mut self) -> EPF_W {
        EPF_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&mut self) -> SDF_W {
        SDF_W { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    pub fn ndf(&mut self) -> NDF_W {
        NDF_W { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&mut self) -> ALF_W {
        ALF_W { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    pub fn pltf(&mut self) -> PLTF_W {
        PLTF_W { w: self }
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&mut self) -> DMF_W {
        DMF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msr::W](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSR to value 0x01"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
