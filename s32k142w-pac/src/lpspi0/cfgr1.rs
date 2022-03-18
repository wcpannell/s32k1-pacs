#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode"]
    SLAVE_MODE = 0,
    #[doc = "1: Master mode"]
    MASTER_MODE = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Master Mode"]
pub struct MASTER_R(crate::FieldReader<bool, MASTER_A>);
impl MASTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE_MODE,
            true => MASTER_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        **self == MASTER_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        **self == MASTER_A::MASTER_MODE
    }
}
impl core::ops::Deref for MASTER_R {
    type Target = crate::FieldReader<bool, MASTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER` writer - Master Mode"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE_MODE)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_MODE)
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
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_A {
    #[doc = "0: Input data is sampled on SCK edge"]
    ON_SCK_EDGE = 0,
    #[doc = "1: Input data is sampled on delayed SCK edge"]
    ON_DELAYED_SCK_EDGE = 1,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLE` reader - Sample Point"]
pub struct SAMPLE_R(crate::FieldReader<bool, SAMPLE_A>);
impl SAMPLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLE_A {
        match self.bits {
            false => SAMPLE_A::ON_SCK_EDGE,
            true => SAMPLE_A::ON_DELAYED_SCK_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `ON_SCK_EDGE`"]
    #[inline(always)]
    pub fn is_on_sck_edge(&self) -> bool {
        **self == SAMPLE_A::ON_SCK_EDGE
    }
    #[doc = "Checks if the value of the field is `ON_DELAYED_SCK_EDGE`"]
    #[inline(always)]
    pub fn is_on_delayed_sck_edge(&self) -> bool {
        **self == SAMPLE_A::ON_DELAYED_SCK_EDGE
    }
}
impl core::ops::Deref for SAMPLE_R {
    type Target = crate::FieldReader<bool, SAMPLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLE` writer - Sample Point"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input data is sampled on SCK edge"]
    #[inline(always)]
    pub fn on_sck_edge(self) -> &'a mut W {
        self.variant(SAMPLE_A::ON_SCK_EDGE)
    }
    #[doc = "Input data is sampled on delayed SCK edge"]
    #[inline(always)]
    pub fn on_delayed_sck_edge(self) -> &'a mut W {
        self.variant(SAMPLE_A::ON_DELAYED_SCK_EDGE)
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
#[doc = "Automatic PCS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOPCS_A {
    #[doc = "0: Automatic PCS generation is disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic PCS generation is enabled"]
    ENABLED = 1,
}
impl From<AUTOPCS_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOPCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOPCS` reader - Automatic PCS"]
pub struct AUTOPCS_R(crate::FieldReader<bool, AUTOPCS_A>);
impl AUTOPCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOPCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOPCS_A {
        match self.bits {
            false => AUTOPCS_A::DISABLED,
            true => AUTOPCS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOPCS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOPCS_A::ENABLED
    }
}
impl core::ops::Deref for AUTOPCS_R {
    type Target = crate::FieldReader<bool, AUTOPCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOPCS` writer - Automatic PCS"]
pub struct AUTOPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOPCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic PCS generation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOPCS_A::DISABLED)
    }
    #[doc = "Automatic PCS generation is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOPCS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "No Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTALL_A {
    #[doc = "0: Transfers will stall when the transmit FIFO is empty"]
    DISABLED = 0,
    #[doc = "1: Transfers will not stall, allowing transmit FIFO underruns to occur"]
    ENABLED = 1,
}
impl From<NOSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTALL` reader - No Stall"]
pub struct NOSTALL_R(crate::FieldReader<bool, NOSTALL_A>);
impl NOSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOSTALL_A {
        match self.bits {
            false => NOSTALL_A::DISABLED,
            true => NOSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NOSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NOSTALL_A::ENABLED
    }
}
impl core::ops::Deref for NOSTALL_R {
    type Target = crate::FieldReader<bool, NOSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOSTALL` writer - No Stall"]
pub struct NOSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfers will stall when the transmit FIFO is empty"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTALL_A::DISABLED)
    }
    #[doc = "Transfers will not stall, allowing transmit FIFO underruns to occur"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTALL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PCSPOL` reader - Peripheral Chip Select Polarity"]
pub struct PCSPOL_R(crate::FieldReader<u8, u8>);
impl PCSPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCSPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSPOL` writer - Peripheral Chip Select Polarity"]
pub struct PCSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match is disabled"]
    DISABLED = 0,
    #[doc = "2: 010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    ENABLED_FIRSTDATAMATCH = 2,
    #[doc = "3: 011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    ENABLED_ANYDATAMATCH = 3,
    #[doc = "4: 100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \\[(1st data word = MATCH0) * (2nd data word = MATCH1)\\]"]
    ENABLED_DATAMATCH_100 = 4,
    #[doc = "5: 101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \\[(any data word = MATCH0) * (next data word = MATCH1)\\]"]
    ENABLED_DATAMATCH_101 = 5,
    #[doc = "6: 110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(1st data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    ENABLED_DATAMATCH_110 = 6,
    #[doc = "7: 111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(any data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    ENABLED_DATAMATCH_111 = 7,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MATCFG` reader - Match Configuration"]
pub struct MATCFG_R(crate::FieldReader<u8, MATCFG_A>);
impl MATCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MATCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MATCFG_A> {
        match self.bits {
            0 => Some(MATCFG_A::DISABLED),
            2 => Some(MATCFG_A::ENABLED_FIRSTDATAMATCH),
            3 => Some(MATCFG_A::ENABLED_ANYDATAMATCH),
            4 => Some(MATCFG_A::ENABLED_DATAMATCH_100),
            5 => Some(MATCFG_A::ENABLED_DATAMATCH_101),
            6 => Some(MATCFG_A::ENABLED_DATAMATCH_110),
            7 => Some(MATCFG_A::ENABLED_DATAMATCH_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MATCFG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED_FIRSTDATAMATCH`"]
    #[inline(always)]
    pub fn is_enabled_firstdatamatch(&self) -> bool {
        **self == MATCFG_A::ENABLED_FIRSTDATAMATCH
    }
    #[doc = "Checks if the value of the field is `ENABLED_ANYDATAMATCH`"]
    #[inline(always)]
    pub fn is_enabled_anydatamatch(&self) -> bool {
        **self == MATCFG_A::ENABLED_ANYDATAMATCH
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_100`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_100(&self) -> bool {
        **self == MATCFG_A::ENABLED_DATAMATCH_100
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_101`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_101(&self) -> bool {
        **self == MATCFG_A::ENABLED_DATAMATCH_101
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_110`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_110(&self) -> bool {
        **self == MATCFG_A::ENABLED_DATAMATCH_110
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_111`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_111(&self) -> bool {
        **self == MATCFG_A::ENABLED_DATAMATCH_111
    }
}
impl core::ops::Deref for MATCFG_R {
    type Target = crate::FieldReader<u8, MATCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCFG` writer - Match Configuration"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Match is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MATCFG_A::DISABLED)
    }
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    #[inline(always)]
    pub fn enabled_firstdatamatch(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_FIRSTDATAMATCH)
    }
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    #[inline(always)]
    pub fn enabled_anydatamatch(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_ANYDATAMATCH)
    }
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \\[(1st data word = MATCH0) * (2nd data word = MATCH1)\\]"]
    #[inline(always)]
    pub fn enabled_datamatch_100(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_100)
    }
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \\[(any data word = MATCH0) * (next data word = MATCH1)\\]"]
    #[inline(always)]
    pub fn enabled_datamatch_101(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_101)
    }
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(1st data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    #[inline(always)]
    pub fn enabled_datamatch_110(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_110)
    }
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(any data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    #[inline(always)]
    pub fn enabled_datamatch_111(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: SIN is used for input data and SOUT is used for output data"]
    SIN_IN_SOUT_OUT = 0,
    #[doc = "1: SIN is used for both input and output data, only half-duplex serial transfers are supported"]
    SIN_BOTH_IN_OUT = 1,
    #[doc = "2: SOUT is used for both input and output data, only half-duplex serial transfers are supported"]
    SOUT_BOTH_IN_OUT = 2,
    #[doc = "3: SOUT is used for input data and SIN is used for output data"]
    SOUT_IN_SIN_OUT = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINCFG` reader - Pin Configuration"]
pub struct PINCFG_R(crate::FieldReader<u8, PINCFG_A>);
impl PINCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PINCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::SIN_IN_SOUT_OUT,
            1 => PINCFG_A::SIN_BOTH_IN_OUT,
            2 => PINCFG_A::SOUT_BOTH_IN_OUT,
            3 => PINCFG_A::SOUT_IN_SIN_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIN_IN_SOUT_OUT`"]
    #[inline(always)]
    pub fn is_sin_in_sout_out(&self) -> bool {
        **self == PINCFG_A::SIN_IN_SOUT_OUT
    }
    #[doc = "Checks if the value of the field is `SIN_BOTH_IN_OUT`"]
    #[inline(always)]
    pub fn is_sin_both_in_out(&self) -> bool {
        **self == PINCFG_A::SIN_BOTH_IN_OUT
    }
    #[doc = "Checks if the value of the field is `SOUT_BOTH_IN_OUT`"]
    #[inline(always)]
    pub fn is_sout_both_in_out(&self) -> bool {
        **self == PINCFG_A::SOUT_BOTH_IN_OUT
    }
    #[doc = "Checks if the value of the field is `SOUT_IN_SIN_OUT`"]
    #[inline(always)]
    pub fn is_sout_in_sin_out(&self) -> bool {
        **self == PINCFG_A::SOUT_IN_SIN_OUT
    }
}
impl core::ops::Deref for PINCFG_R {
    type Target = crate::FieldReader<u8, PINCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINCFG` writer - Pin Configuration"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    #[inline(always)]
    pub fn sin_in_sout_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SIN_IN_SOUT_OUT)
    }
    #[doc = "SIN is used for both input and output data, only half-duplex serial transfers are supported"]
    #[inline(always)]
    pub fn sin_both_in_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SIN_BOTH_IN_OUT)
    }
    #[doc = "SOUT is used for both input and output data, only half-duplex serial transfers are supported"]
    #[inline(always)]
    pub fn sout_both_in_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SOUT_BOTH_IN_OUT)
    }
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    #[inline(always)]
    pub fn sout_in_sin_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SOUT_IN_SIN_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCFG_A {
    #[doc = "0: Output data retains last value when chip select is negated"]
    RETAIN_LASTVALUE = 0,
    #[doc = "1: Output data is tristated when chip select is negated"]
    TRISTATED = 1,
}
impl From<OUTCFG_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTCFG` reader - Output Configuration"]
pub struct OUTCFG_R(crate::FieldReader<bool, OUTCFG_A>);
impl OUTCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCFG_A {
        match self.bits {
            false => OUTCFG_A::RETAIN_LASTVALUE,
            true => OUTCFG_A::TRISTATED,
        }
    }
    #[doc = "Checks if the value of the field is `RETAIN_LASTVALUE`"]
    #[inline(always)]
    pub fn is_retain_lastvalue(&self) -> bool {
        **self == OUTCFG_A::RETAIN_LASTVALUE
    }
    #[doc = "Checks if the value of the field is `TRISTATED`"]
    #[inline(always)]
    pub fn is_tristated(&self) -> bool {
        **self == OUTCFG_A::TRISTATED
    }
}
impl core::ops::Deref for OUTCFG_R {
    type Target = crate::FieldReader<bool, OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTCFG` writer - Output Configuration"]
pub struct OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output data retains last value when chip select is negated"]
    #[inline(always)]
    pub fn retain_lastvalue(self) -> &'a mut W {
        self.variant(OUTCFG_A::RETAIN_LASTVALUE)
    }
    #[doc = "Output data is tristated when chip select is negated"]
    #[inline(always)]
    pub fn tristated(self) -> &'a mut W {
        self.variant(OUTCFG_A::TRISTATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Peripheral Chip Select Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSCFG_A {
    #[doc = "0: PCS\\[3:2\\]
are configured for chip select function"]
    CHIP_SELECT = 0,
    #[doc = "1: PCS\\[3:2\\]
are configured for half-duplex 4-bit transfers (PCS\\[3:2\\]
= DATA\\[3:2\\])"]
    HALFDUPLEX4BIT = 1,
}
impl From<PCSCFG_A> for bool {
    #[inline(always)]
    fn from(variant: PCSCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCSCFG` reader - Peripheral Chip Select Configuration"]
pub struct PCSCFG_R(crate::FieldReader<bool, PCSCFG_A>);
impl PCSCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCSCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSCFG_A {
        match self.bits {
            false => PCSCFG_A::CHIP_SELECT,
            true => PCSCFG_A::HALFDUPLEX4BIT,
        }
    }
    #[doc = "Checks if the value of the field is `CHIP_SELECT`"]
    #[inline(always)]
    pub fn is_chip_select(&self) -> bool {
        **self == PCSCFG_A::CHIP_SELECT
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX4BIT`"]
    #[inline(always)]
    pub fn is_halfduplex4bit(&self) -> bool {
        **self == PCSCFG_A::HALFDUPLEX4BIT
    }
}
impl core::ops::Deref for PCSCFG_R {
    type Target = crate::FieldReader<bool, PCSCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSCFG` writer - Peripheral Chip Select Configuration"]
pub struct PCSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PCS\\[3:2\\]
are configured for chip select function"]
    #[inline(always)]
    pub fn chip_select(self) -> &'a mut W {
        self.variant(PCSCFG_A::CHIP_SELECT)
    }
    #[doc = "PCS\\[3:2\\]
are configured for half-duplex 4-bit transfers (PCS\\[3:2\\]
= DATA\\[3:2\\])"]
    #[inline(always)]
    pub fn halfduplex4bit(self) -> &'a mut W {
        self.variant(PCSCFG_A::HALFDUPLEX4BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    pub fn autopcs(&self) -> AUTOPCS_R {
        AUTOPCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    pub fn nostall(&self) -> NOSTALL_R {
        NOSTALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub fn pcspol(&self) -> PCSPOL_R {
        PCSPOL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Output Configuration"]
    #[inline(always)]
    pub fn outcfg(&self) -> OUTCFG_R {
        OUTCFG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub fn pcscfg(&self) -> PCSCFG_R {
        PCSCFG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    pub fn autopcs(&mut self) -> AUTOPCS_W {
        AUTOPCS_W { w: self }
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    pub fn nostall(&mut self) -> NOSTALL_W {
        NOSTALL_W { w: self }
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub fn pcspol(&mut self) -> PCSPOL_W {
        PCSPOL_W { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 26 - Output Configuration"]
    #[inline(always)]
    pub fn outcfg(&mut self) -> OUTCFG_W {
        OUTCFG_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub fn pcscfg(&mut self) -> PCSCFG_W {
        PCSCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
