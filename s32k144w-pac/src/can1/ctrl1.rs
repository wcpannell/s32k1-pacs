#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROPSEG` reader - Propagation Segment"]
pub struct PROPSEG_R(crate::FieldReader<u8, u8>);
impl PROPSEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROPSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROPSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROPSEG` writer - Propagation Segment"]
pub struct PROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Listen-Only Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOM_A {
    #[doc = "0: Listen-Only mode is deactivated."]
    LISTEN_ONLY_MODE_DISABLED = 0,
    #[doc = "1: FlexCAN module operates in Listen-Only mode."]
    LISTEN_ONLY_MODE_ENABLED = 1,
}
impl From<LOM_A> for bool {
    #[inline(always)]
    fn from(variant: LOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOM` reader - Listen-Only Mode"]
pub struct LOM_R(crate::FieldReader<bool, LOM_A>);
impl LOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOM_A {
        match self.bits {
            false => LOM_A::LISTEN_ONLY_MODE_DISABLED,
            true => LOM_A::LISTEN_ONLY_MODE_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `LISTEN_ONLY_MODE_DISABLED`"]
    #[inline(always)]
    pub fn is_listen_only_mode_disabled(&self) -> bool {
        **self == LOM_A::LISTEN_ONLY_MODE_DISABLED
    }
    #[doc = "Checks if the value of the field is `LISTEN_ONLY_MODE_ENABLED`"]
    #[inline(always)]
    pub fn is_listen_only_mode_enabled(&self) -> bool {
        **self == LOM_A::LISTEN_ONLY_MODE_ENABLED
    }
}
impl core::ops::Deref for LOM_R {
    type Target = crate::FieldReader<bool, LOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOM` writer - Listen-Only Mode"]
pub struct LOM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Listen-Only mode is deactivated."]
    #[inline(always)]
    pub fn listen_only_mode_disabled(self) -> &'a mut W {
        self.variant(LOM_A::LISTEN_ONLY_MODE_DISABLED)
    }
    #[doc = "FlexCAN module operates in Listen-Only mode."]
    #[inline(always)]
    pub fn listen_only_mode_enabled(self) -> &'a mut W {
        self.variant(LOM_A::LISTEN_ONLY_MODE_ENABLED)
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
#[doc = "Lowest Buffer Transmitted First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBUF_A {
    #[doc = "0: Buffer with highest priority is transmitted first."]
    HIGHEST_BUFFER_FIRST = 0,
    #[doc = "1: Lowest number buffer is transmitted first."]
    LOWEST_BUFFER_FIRST = 1,
}
impl From<LBUF_A> for bool {
    #[inline(always)]
    fn from(variant: LBUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBUF` reader - Lowest Buffer Transmitted First"]
pub struct LBUF_R(crate::FieldReader<bool, LBUF_A>);
impl LBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBUF_A {
        match self.bits {
            false => LBUF_A::HIGHEST_BUFFER_FIRST,
            true => LBUF_A::LOWEST_BUFFER_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHEST_BUFFER_FIRST`"]
    #[inline(always)]
    pub fn is_highest_buffer_first(&self) -> bool {
        **self == LBUF_A::HIGHEST_BUFFER_FIRST
    }
    #[doc = "Checks if the value of the field is `LOWEST_BUFFER_FIRST`"]
    #[inline(always)]
    pub fn is_lowest_buffer_first(&self) -> bool {
        **self == LBUF_A::LOWEST_BUFFER_FIRST
    }
}
impl core::ops::Deref for LBUF_R {
    type Target = crate::FieldReader<bool, LBUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBUF` writer - Lowest Buffer Transmitted First"]
pub struct LBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Buffer with highest priority is transmitted first."]
    #[inline(always)]
    pub fn highest_buffer_first(self) -> &'a mut W {
        self.variant(LBUF_A::HIGHEST_BUFFER_FIRST)
    }
    #[doc = "Lowest number buffer is transmitted first."]
    #[inline(always)]
    pub fn lowest_buffer_first(self) -> &'a mut W {
        self.variant(LBUF_A::LOWEST_BUFFER_FIRST)
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
#[doc = "Timer Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSYN_A {
    #[doc = "0: Timer sync feature disabled"]
    TIMER_SYNC_DISABLED = 0,
    #[doc = "1: Timer sync feature enabled"]
    TIMER_SYNC_ENABLED = 1,
}
impl From<TSYN_A> for bool {
    #[inline(always)]
    fn from(variant: TSYN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSYN` reader - Timer Sync"]
pub struct TSYN_R(crate::FieldReader<bool, TSYN_A>);
impl TSYN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSYN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSYN_A {
        match self.bits {
            false => TSYN_A::TIMER_SYNC_DISABLED,
            true => TSYN_A::TIMER_SYNC_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_DISABLED`"]
    #[inline(always)]
    pub fn is_timer_sync_disabled(&self) -> bool {
        **self == TSYN_A::TIMER_SYNC_DISABLED
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_ENABLED`"]
    #[inline(always)]
    pub fn is_timer_sync_enabled(&self) -> bool {
        **self == TSYN_A::TIMER_SYNC_ENABLED
    }
}
impl core::ops::Deref for TSYN_R {
    type Target = crate::FieldReader<bool, TSYN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSYN` writer - Timer Sync"]
pub struct TSYN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSYN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSYN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer sync feature disabled"]
    #[inline(always)]
    pub fn timer_sync_disabled(self) -> &'a mut W {
        self.variant(TSYN_A::TIMER_SYNC_DISABLED)
    }
    #[doc = "Timer sync feature enabled"]
    #[inline(always)]
    pub fn timer_sync_enabled(self) -> &'a mut W {
        self.variant(TSYN_A::TIMER_SYNC_ENABLED)
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
#[doc = "Bus Off Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFREC_A {
    #[doc = "0: Automatic recovering from Bus Off state enabled."]
    AUTO_RECOVER_ENABLED = 0,
    #[doc = "1: Automatic recovering from Bus Off state disabled."]
    AUTO_RECOVER_DISABLED = 1,
}
impl From<BOFFREC_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFREC` reader - Bus Off Recovery"]
pub struct BOFFREC_R(crate::FieldReader<bool, BOFFREC_A>);
impl BOFFREC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFFREC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFREC_A {
        match self.bits {
            false => BOFFREC_A::AUTO_RECOVER_ENABLED,
            true => BOFFREC_A::AUTO_RECOVER_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_RECOVER_ENABLED`"]
    #[inline(always)]
    pub fn is_auto_recover_enabled(&self) -> bool {
        **self == BOFFREC_A::AUTO_RECOVER_ENABLED
    }
    #[doc = "Checks if the value of the field is `AUTO_RECOVER_DISABLED`"]
    #[inline(always)]
    pub fn is_auto_recover_disabled(&self) -> bool {
        **self == BOFFREC_A::AUTO_RECOVER_DISABLED
    }
}
impl core::ops::Deref for BOFFREC_R {
    type Target = crate::FieldReader<bool, BOFFREC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFFREC` writer - Bus Off Recovery"]
pub struct BOFFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFREC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic recovering from Bus Off state enabled."]
    #[inline(always)]
    pub fn auto_recover_enabled(self) -> &'a mut W {
        self.variant(BOFFREC_A::AUTO_RECOVER_ENABLED)
    }
    #[doc = "Automatic recovering from Bus Off state disabled."]
    #[inline(always)]
    pub fn auto_recover_disabled(self) -> &'a mut W {
        self.variant(BOFFREC_A::AUTO_RECOVER_DISABLED)
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
#[doc = "CAN Bit Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_A {
    #[doc = "0: Just one sample is used to determine the bit value."]
    ONE_SAMPLE = 0,
    #[doc = "1: Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples; a majority rule is used."]
    THREE_SAMPLE = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP` reader - CAN Bit Sampling"]
pub struct SMP_R(crate::FieldReader<bool, SMP_A>);
impl SMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::ONE_SAMPLE,
            true => SMP_A::THREE_SAMPLE,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SAMPLE`"]
    #[inline(always)]
    pub fn is_one_sample(&self) -> bool {
        **self == SMP_A::ONE_SAMPLE
    }
    #[doc = "Checks if the value of the field is `THREE_SAMPLE`"]
    #[inline(always)]
    pub fn is_three_sample(&self) -> bool {
        **self == SMP_A::THREE_SAMPLE
    }
}
impl core::ops::Deref for SMP_R {
    type Target = crate::FieldReader<bool, SMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP` writer - CAN Bit Sampling"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Just one sample is used to determine the bit value."]
    #[inline(always)]
    pub fn one_sample(self) -> &'a mut W {
        self.variant(SMP_A::ONE_SAMPLE)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples; a majority rule is used."]
    #[inline(always)]
    pub fn three_sample(self) -> &'a mut W {
        self.variant(SMP_A::THREE_SAMPLE)
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
#[doc = "Rx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNMSK_A {
    #[doc = "0: Rx Warning interrupt disabled."]
    RX_WARNING_INT_DISABLED = 0,
    #[doc = "1: Rx Warning interrupt enabled."]
    RX_WARNING_INT_ENABLED = 1,
}
impl From<RWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWRNMSK` reader - Rx Warning Interrupt Mask"]
pub struct RWRNMSK_R(crate::FieldReader<bool, RWRNMSK_A>);
impl RWRNMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWRNMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNMSK_A {
        match self.bits {
            false => RWRNMSK_A::RX_WARNING_INT_DISABLED,
            true => RWRNMSK_A::RX_WARNING_INT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `RX_WARNING_INT_DISABLED`"]
    #[inline(always)]
    pub fn is_rx_warning_int_disabled(&self) -> bool {
        **self == RWRNMSK_A::RX_WARNING_INT_DISABLED
    }
    #[doc = "Checks if the value of the field is `RX_WARNING_INT_ENABLED`"]
    #[inline(always)]
    pub fn is_rx_warning_int_enabled(&self) -> bool {
        **self == RWRNMSK_A::RX_WARNING_INT_ENABLED
    }
}
impl core::ops::Deref for RWRNMSK_R {
    type Target = crate::FieldReader<bool, RWRNMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWRNMSK` writer - Rx Warning Interrupt Mask"]
pub struct RWRNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWRNMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWRNMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rx Warning interrupt disabled."]
    #[inline(always)]
    pub fn rx_warning_int_disabled(self) -> &'a mut W {
        self.variant(RWRNMSK_A::RX_WARNING_INT_DISABLED)
    }
    #[doc = "Rx Warning interrupt enabled."]
    #[inline(always)]
    pub fn rx_warning_int_enabled(self) -> &'a mut W {
        self.variant(RWRNMSK_A::RX_WARNING_INT_ENABLED)
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
#[doc = "Tx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNMSK_A {
    #[doc = "0: Tx Warning interrupt disabled."]
    TX_WARNING_INT_DISABLED = 0,
    #[doc = "1: Tx Warning interrupt enabled."]
    TX_WARNING_INT_ENABLED = 1,
}
impl From<TWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWRNMSK` reader - Tx Warning Interrupt Mask"]
pub struct TWRNMSK_R(crate::FieldReader<bool, TWRNMSK_A>);
impl TWRNMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWRNMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNMSK_A {
        match self.bits {
            false => TWRNMSK_A::TX_WARNING_INT_DISABLED,
            true => TWRNMSK_A::TX_WARNING_INT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `TX_WARNING_INT_DISABLED`"]
    #[inline(always)]
    pub fn is_tx_warning_int_disabled(&self) -> bool {
        **self == TWRNMSK_A::TX_WARNING_INT_DISABLED
    }
    #[doc = "Checks if the value of the field is `TX_WARNING_INT_ENABLED`"]
    #[inline(always)]
    pub fn is_tx_warning_int_enabled(&self) -> bool {
        **self == TWRNMSK_A::TX_WARNING_INT_ENABLED
    }
}
impl core::ops::Deref for TWRNMSK_R {
    type Target = crate::FieldReader<bool, TWRNMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWRNMSK` writer - Tx Warning Interrupt Mask"]
pub struct TWRNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWRNMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWRNMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx Warning interrupt disabled."]
    #[inline(always)]
    pub fn tx_warning_int_disabled(self) -> &'a mut W {
        self.variant(TWRNMSK_A::TX_WARNING_INT_DISABLED)
    }
    #[doc = "Tx Warning interrupt enabled."]
    #[inline(always)]
    pub fn tx_warning_int_enabled(self) -> &'a mut W {
        self.variant(TWRNMSK_A::TX_WARNING_INT_ENABLED)
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
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPB_A {
    #[doc = "0: Loop Back disabled."]
    LOOPBACK_DISABLED = 0,
    #[doc = "1: Loop Back enabled."]
    LOOPBACK_ENABLED = 1,
}
impl From<LPB_A> for bool {
    #[inline(always)]
    fn from(variant: LPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPB` reader - Loop Back Mode"]
pub struct LPB_R(crate::FieldReader<bool, LPB_A>);
impl LPB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPB_A {
        match self.bits {
            false => LPB_A::LOOPBACK_DISABLED,
            true => LPB_A::LOOPBACK_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `LOOPBACK_DISABLED`"]
    #[inline(always)]
    pub fn is_loopback_disabled(&self) -> bool {
        **self == LPB_A::LOOPBACK_DISABLED
    }
    #[doc = "Checks if the value of the field is `LOOPBACK_ENABLED`"]
    #[inline(always)]
    pub fn is_loopback_enabled(&self) -> bool {
        **self == LPB_A::LOOPBACK_ENABLED
    }
}
impl core::ops::Deref for LPB_R {
    type Target = crate::FieldReader<bool, LPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPB` writer - Loop Back Mode"]
pub struct LPB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loop Back disabled."]
    #[inline(always)]
    pub fn loopback_disabled(self) -> &'a mut W {
        self.variant(LPB_A::LOOPBACK_DISABLED)
    }
    #[doc = "Loop Back enabled."]
    #[inline(always)]
    pub fn loopback_enabled(self) -> &'a mut W {
        self.variant(LPB_A::LOOPBACK_ENABLED)
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
#[doc = "CAN Engine Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    OSCILLATOR_CLOCK = 0,
    #[doc = "1: The CAN engine clock source is the peripheral clock."]
    PERIPHERAL_CLOCK = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - CAN Engine Clock Source"]
pub struct CLKSRC_R(crate::FieldReader<bool, CLKSRC_A>);
impl CLKSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::OSCILLATOR_CLOCK,
            true => CLKSRC_A::PERIPHERAL_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_CLOCK`"]
    #[inline(always)]
    pub fn is_oscillator_clock(&self) -> bool {
        **self == CLKSRC_A::OSCILLATOR_CLOCK
    }
    #[doc = "Checks if the value of the field is `PERIPHERAL_CLOCK`"]
    #[inline(always)]
    pub fn is_peripheral_clock(&self) -> bool {
        **self == CLKSRC_A::PERIPHERAL_CLOCK
    }
}
impl core::ops::Deref for CLKSRC_R {
    type Target = crate::FieldReader<bool, CLKSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSRC` writer - CAN Engine Clock Source"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    #[inline(always)]
    pub fn oscillator_clock(self) -> &'a mut W {
        self.variant(CLKSRC_A::OSCILLATOR_CLOCK)
    }
    #[doc = "The CAN engine clock source is the peripheral clock."]
    #[inline(always)]
    pub fn peripheral_clock(self) -> &'a mut W {
        self.variant(CLKSRC_A::PERIPHERAL_CLOCK)
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
#[doc = "Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSK_A {
    #[doc = "0: Error interrupt disabled."]
    ERROR_INT_DISABLED = 0,
    #[doc = "1: Error interrupt enabled."]
    ERROR_INT_ENABLED = 1,
}
impl From<ERRMSK_A> for bool {
    #[inline(always)]
    fn from(variant: ERRMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRMSK` reader - Error Interrupt Mask"]
pub struct ERRMSK_R(crate::FieldReader<bool, ERRMSK_A>);
impl ERRMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRMSK_A {
        match self.bits {
            false => ERRMSK_A::ERROR_INT_DISABLED,
            true => ERRMSK_A::ERROR_INT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ERROR_INT_DISABLED`"]
    #[inline(always)]
    pub fn is_error_int_disabled(&self) -> bool {
        **self == ERRMSK_A::ERROR_INT_DISABLED
    }
    #[doc = "Checks if the value of the field is `ERROR_INT_ENABLED`"]
    #[inline(always)]
    pub fn is_error_int_enabled(&self) -> bool {
        **self == ERRMSK_A::ERROR_INT_ENABLED
    }
}
impl core::ops::Deref for ERRMSK_R {
    type Target = crate::FieldReader<bool, ERRMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRMSK` writer - Error Interrupt Mask"]
pub struct ERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error interrupt disabled."]
    #[inline(always)]
    pub fn error_int_disabled(self) -> &'a mut W {
        self.variant(ERRMSK_A::ERROR_INT_DISABLED)
    }
    #[doc = "Error interrupt enabled."]
    #[inline(always)]
    pub fn error_int_enabled(self) -> &'a mut W {
        self.variant(ERRMSK_A::ERROR_INT_ENABLED)
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
#[doc = "Bus Off Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFMSK_A {
    #[doc = "0: Bus Off interrupt disabled."]
    BUS_OFF_INT_DISABLED = 0,
    #[doc = "1: Bus Off interrupt enabled."]
    BUS_OFF_INT_ENABLED = 1,
}
impl From<BOFFMSK_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFMSK` reader - Bus Off Interrupt Mask"]
pub struct BOFFMSK_R(crate::FieldReader<bool, BOFFMSK_A>);
impl BOFFMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFFMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFMSK_A {
        match self.bits {
            false => BOFFMSK_A::BUS_OFF_INT_DISABLED,
            true => BOFFMSK_A::BUS_OFF_INT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_INT_DISABLED`"]
    #[inline(always)]
    pub fn is_bus_off_int_disabled(&self) -> bool {
        **self == BOFFMSK_A::BUS_OFF_INT_DISABLED
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_INT_ENABLED`"]
    #[inline(always)]
    pub fn is_bus_off_int_enabled(&self) -> bool {
        **self == BOFFMSK_A::BUS_OFF_INT_ENABLED
    }
}
impl core::ops::Deref for BOFFMSK_R {
    type Target = crate::FieldReader<bool, BOFFMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFFMSK` writer - Bus Off Interrupt Mask"]
pub struct BOFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus Off interrupt disabled."]
    #[inline(always)]
    pub fn bus_off_int_disabled(self) -> &'a mut W {
        self.variant(BOFFMSK_A::BUS_OFF_INT_DISABLED)
    }
    #[doc = "Bus Off interrupt enabled."]
    #[inline(always)]
    pub fn bus_off_int_enabled(self) -> &'a mut W {
        self.variant(BOFFMSK_A::BUS_OFF_INT_ENABLED)
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
#[doc = "Field `PSEG2` reader - Phase Segment 2"]
pub struct PSEG2_R(crate::FieldReader<u8, u8>);
impl PSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEG2` writer - Phase Segment 2"]
pub struct PSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `PSEG1` reader - Phase Segment 1"]
pub struct PSEG1_R(crate::FieldReader<u8, u8>);
impl PSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEG1` writer - Phase Segment 1"]
pub struct PSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `RJW` reader - Resync Jump Width"]
pub struct RJW_R(crate::FieldReader<u8, u8>);
impl RJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RJW` writer - Resync Jump Width"]
pub struct RJW_W<'a> {
    w: &'a mut W,
}
impl<'a> RJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PRESDIV` reader - Prescaler Division Factor"]
pub struct PRESDIV_R(crate::FieldReader<u8, u8>);
impl PRESDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESDIV` writer - Prescaler Division Factor"]
pub struct PRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    pub fn propseg(&self) -> PROPSEG_R {
        PROPSEG_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    pub fn lbuf(&self) -> LBUF_R {
        LBUF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    pub fn tsyn(&self) -> TSYN_R {
        TSYN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    pub fn boffrec(&self) -> BOFFREC_R {
        BOFFREC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn rwrnmsk(&self) -> RWRNMSK_R {
        RWRNMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn twrnmsk(&self) -> TWRNMSK_R {
        TWRNMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    pub fn lpb(&self) -> LPB_R {
        LPB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errmsk(&self) -> ERRMSK_R {
        ERRMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bus Off Interrupt Mask"]
    #[inline(always)]
    pub fn boffmsk(&self) -> BOFFMSK_R {
        BOFFMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    pub fn pseg2(&self) -> PSEG2_R {
        PSEG2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    pub fn pseg1(&self) -> PSEG1_R {
        PSEG1_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    pub fn rjw(&self) -> RJW_R {
        RJW_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    pub fn presdiv(&self) -> PRESDIV_R {
        PRESDIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    pub fn propseg(&mut self) -> PROPSEG_W {
        PROPSEG_W { w: self }
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W {
        LOM_W { w: self }
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    pub fn lbuf(&mut self) -> LBUF_W {
        LBUF_W { w: self }
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    pub fn tsyn(&mut self) -> TSYN_W {
        TSYN_W { w: self }
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    pub fn boffrec(&mut self) -> BOFFREC_W {
        BOFFREC_W { w: self }
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn rwrnmsk(&mut self) -> RWRNMSK_W {
        RWRNMSK_W { w: self }
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn twrnmsk(&mut self) -> TWRNMSK_W {
        TWRNMSK_W { w: self }
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    pub fn lpb(&mut self) -> LPB_W {
        LPB_W { w: self }
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 14 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn errmsk(&mut self) -> ERRMSK_W {
        ERRMSK_W { w: self }
    }
    #[doc = "Bit 15 - Bus Off Interrupt Mask"]
    #[inline(always)]
    pub fn boffmsk(&mut self) -> BOFFMSK_W {
        BOFFMSK_W { w: self }
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    pub fn pseg2(&mut self) -> PSEG2_W {
        PSEG2_W { w: self }
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    pub fn pseg1(&mut self) -> PSEG1_W {
        PSEG1_W { w: self }
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    pub fn rjw(&mut self) -> RJW_W {
        RJW_W { w: self }
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    pub fn presdiv(&mut self) -> PRESDIV_W {
        PRESDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
