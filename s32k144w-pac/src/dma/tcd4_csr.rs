#[doc = "Register `TCD4_CSR` reader"]
pub struct R(crate::R<TCD4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD4_CSR` writer"]
pub struct W(crate::W<TCD4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD4_CSR_SPEC>;
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
impl From<crate::W<TCD4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: The channel is not explicitly started."]
    START_0 = 0,
    #[doc = "1: The channel is explicitly started via a software initiated service request."]
    START_1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Channel Start"]
pub struct START_R(crate::FieldReader<bool, START_A>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::START_0,
            true => START_A::START_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_0`"]
    #[inline(always)]
    pub fn is_start_0(&self) -> bool {
        **self == START_A::START_0
    }
    #[doc = "Checks if the value of the field is `START_1`"]
    #[inline(always)]
    pub fn is_start_1(&self) -> bool {
        **self == START_A::START_1
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Channel Start"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel is not explicitly started."]
    #[inline(always)]
    pub fn start_0(self) -> &'a mut W {
        self.variant(START_A::START_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request."]
    #[inline(always)]
    pub fn start_1(self) -> &'a mut W {
        self.variant(START_A::START_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Enable an interrupt when major iteration count completes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMAJOR_A {
    #[doc = "0: The end-of-major loop interrupt is disabled."]
    INTMAJOR_0 = 0,
    #[doc = "1: The end-of-major loop interrupt is enabled."]
    INTMAJOR_1 = 1,
}
impl From<INTMAJOR_A> for bool {
    #[inline(always)]
    fn from(variant: INTMAJOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTMAJOR` reader - Enable an interrupt when major iteration count completes."]
pub struct INTMAJOR_R(crate::FieldReader<bool, INTMAJOR_A>);
impl INTMAJOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTMAJOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMAJOR_A {
        match self.bits {
            false => INTMAJOR_A::INTMAJOR_0,
            true => INTMAJOR_A::INTMAJOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTMAJOR_0`"]
    #[inline(always)]
    pub fn is_intmajor_0(&self) -> bool {
        **self == INTMAJOR_A::INTMAJOR_0
    }
    #[doc = "Checks if the value of the field is `INTMAJOR_1`"]
    #[inline(always)]
    pub fn is_intmajor_1(&self) -> bool {
        **self == INTMAJOR_A::INTMAJOR_1
    }
}
impl core::ops::Deref for INTMAJOR_R {
    type Target = crate::FieldReader<bool, INTMAJOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTMAJOR` writer - Enable an interrupt when major iteration count completes."]
pub struct INTMAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMAJOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTMAJOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The end-of-major loop interrupt is disabled."]
    #[inline(always)]
    pub fn intmajor_0(self) -> &'a mut W {
        self.variant(INTMAJOR_A::INTMAJOR_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled."]
    #[inline(always)]
    pub fn intmajor_1(self) -> &'a mut W {
        self.variant(INTMAJOR_A::INTMAJOR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTHALF_A {
    #[doc = "0: The half-point interrupt is disabled."]
    INTHALF_0 = 0,
    #[doc = "1: The half-point interrupt is enabled."]
    INTHALF_1 = 1,
}
impl From<INTHALF_A> for bool {
    #[inline(always)]
    fn from(variant: INTHALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTHALF` reader - Enable an interrupt when major counter is half complete."]
pub struct INTHALF_R(crate::FieldReader<bool, INTHALF_A>);
impl INTHALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTHALF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTHALF_A {
        match self.bits {
            false => INTHALF_A::INTHALF_0,
            true => INTHALF_A::INTHALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTHALF_0`"]
    #[inline(always)]
    pub fn is_inthalf_0(&self) -> bool {
        **self == INTHALF_A::INTHALF_0
    }
    #[doc = "Checks if the value of the field is `INTHALF_1`"]
    #[inline(always)]
    pub fn is_inthalf_1(&self) -> bool {
        **self == INTHALF_A::INTHALF_1
    }
}
impl core::ops::Deref for INTHALF_R {
    type Target = crate::FieldReader<bool, INTHALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTHALF` writer - Enable an interrupt when major counter is half complete."]
pub struct INTHALF_W<'a> {
    w: &'a mut W,
}
impl<'a> INTHALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTHALF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The half-point interrupt is disabled."]
    #[inline(always)]
    pub fn inthalf_0(self) -> &'a mut W {
        self.variant(INTHALF_A::INTHALF_0)
    }
    #[doc = "The half-point interrupt is enabled."]
    #[inline(always)]
    pub fn inthalf_1(self) -> &'a mut W {
        self.variant(INTHALF_A::INTHALF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DREQ_A {
    #[doc = "0: The channel's ERQ bit is not affected."]
    DREQ_0 = 0,
    #[doc = "1: The channel's ERQ bit is cleared when the major loop is complete."]
    DREQ_1 = 1,
}
impl From<DREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREQ` reader - Disable Request"]
pub struct DREQ_R(crate::FieldReader<bool, DREQ_A>);
impl DREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DREQ_A {
        match self.bits {
            false => DREQ_A::DREQ_0,
            true => DREQ_A::DREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DREQ_0`"]
    #[inline(always)]
    pub fn is_dreq_0(&self) -> bool {
        **self == DREQ_A::DREQ_0
    }
    #[doc = "Checks if the value of the field is `DREQ_1`"]
    #[inline(always)]
    pub fn is_dreq_1(&self) -> bool {
        **self == DREQ_A::DREQ_1
    }
}
impl core::ops::Deref for DREQ_R {
    type Target = crate::FieldReader<bool, DREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREQ` writer - Disable Request"]
pub struct DREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel's ERQ bit is not affected."]
    #[inline(always)]
    pub fn dreq_0(self) -> &'a mut W {
        self.variant(DREQ_A::DREQ_0)
    }
    #[doc = "The channel's ERQ bit is cleared when the major loop is complete."]
    #[inline(always)]
    pub fn dreq_1(self) -> &'a mut W {
        self.variant(DREQ_A::DREQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESG_A {
    #[doc = "0: The current channel's TCD is normal format."]
    ESG_0 = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    ESG_1 = 1,
}
impl From<ESG_A> for bool {
    #[inline(always)]
    fn from(variant: ESG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESG` reader - Enable Scatter/Gather Processing"]
pub struct ESG_R(crate::FieldReader<bool, ESG_A>);
impl ESG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESG_A {
        match self.bits {
            false => ESG_A::ESG_0,
            true => ESG_A::ESG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESG_0`"]
    #[inline(always)]
    pub fn is_esg_0(&self) -> bool {
        **self == ESG_A::ESG_0
    }
    #[doc = "Checks if the value of the field is `ESG_1`"]
    #[inline(always)]
    pub fn is_esg_1(&self) -> bool {
        **self == ESG_A::ESG_1
    }
}
impl core::ops::Deref for ESG_R {
    type Target = crate::FieldReader<bool, ESG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESG` writer - Enable Scatter/Gather Processing"]
pub struct ESG_W<'a> {
    w: &'a mut W,
}
impl<'a> ESG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The current channel's TCD is normal format."]
    #[inline(always)]
    pub fn esg_0(self) -> &'a mut W {
        self.variant(ESG_A::ESG_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline(always)]
    pub fn esg_1(self) -> &'a mut W {
        self.variant(ESG_A::ESG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJORELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled."]
    MAJORELINK_0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled."]
    MAJORELINK_1 = 1,
}
impl From<MAJORELINK_A> for bool {
    #[inline(always)]
    fn from(variant: MAJORELINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAJORELINK` reader - Enable channel-to-channel linking on major loop complete"]
pub struct MAJORELINK_R(crate::FieldReader<bool, MAJORELINK_A>);
impl MAJORELINK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAJORELINK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAJORELINK_A {
        match self.bits {
            false => MAJORELINK_A::MAJORELINK_0,
            true => MAJORELINK_A::MAJORELINK_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAJORELINK_0`"]
    #[inline(always)]
    pub fn is_majorelink_0(&self) -> bool {
        **self == MAJORELINK_A::MAJORELINK_0
    }
    #[doc = "Checks if the value of the field is `MAJORELINK_1`"]
    #[inline(always)]
    pub fn is_majorelink_1(&self) -> bool {
        **self == MAJORELINK_A::MAJORELINK_1
    }
}
impl core::ops::Deref for MAJORELINK_R {
    type Target = crate::FieldReader<bool, MAJORELINK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJORELINK` writer - Enable channel-to-channel linking on major loop complete"]
pub struct MAJORELINK_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJORELINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAJORELINK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel-to-channel linking is disabled."]
    #[inline(always)]
    pub fn majorelink_0(self) -> &'a mut W {
        self.variant(MAJORELINK_A::MAJORELINK_0)
    }
    #[doc = "The channel-to-channel linking is enabled."]
    #[inline(always)]
    pub fn majorelink_1(self) -> &'a mut W {
        self.variant(MAJORELINK_A::MAJORELINK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ACTIVE` reader - Channel Active"]
pub struct ACTIVE_R(crate::FieldReader<bool, bool>);
impl ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` reader - Channel Done"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Channel Done"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `MAJORLINKCH` reader - Major Loop Link Channel Number"]
pub struct MAJORLINKCH_R(crate::FieldReader<u8, u8>);
impl MAJORLINKCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJORLINKCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJORLINKCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJORLINKCH` writer - Major Loop Link Channel Number"]
pub struct MAJORLINKCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJORLINKCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BWC_A {
    #[doc = "0: No eDMA engine stalls."]
    BWC_0 = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each R/W."]
    BWC_2 = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each R/W."]
    BWC_3 = 3,
}
impl From<BWC_A> for u8 {
    #[inline(always)]
    fn from(variant: BWC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BWC` reader - Bandwidth Control"]
pub struct BWC_R(crate::FieldReader<u8, BWC_A>);
impl BWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BWC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BWC_A> {
        match self.bits {
            0 => Some(BWC_A::BWC_0),
            2 => Some(BWC_A::BWC_2),
            3 => Some(BWC_A::BWC_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BWC_0`"]
    #[inline(always)]
    pub fn is_bwc_0(&self) -> bool {
        **self == BWC_A::BWC_0
    }
    #[doc = "Checks if the value of the field is `BWC_2`"]
    #[inline(always)]
    pub fn is_bwc_2(&self) -> bool {
        **self == BWC_A::BWC_2
    }
    #[doc = "Checks if the value of the field is `BWC_3`"]
    #[inline(always)]
    pub fn is_bwc_3(&self) -> bool {
        **self == BWC_A::BWC_3
    }
}
impl core::ops::Deref for BWC_R {
    type Target = crate::FieldReader<u8, BWC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWC` writer - Bandwidth Control"]
pub struct BWC_W<'a> {
    w: &'a mut W,
}
impl<'a> BWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No eDMA engine stalls."]
    #[inline(always)]
    pub fn bwc_0(self) -> &'a mut W {
        self.variant(BWC_A::BWC_0)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    #[inline(always)]
    pub fn bwc_2(self) -> &'a mut W {
        self.variant(BWC_A::BWC_2)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    #[inline(always)]
    pub fn bwc_3(self) -> &'a mut W {
        self.variant(BWC_A::BWC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u16 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub fn intmajor(&self) -> INTMAJOR_R {
        INTMAJOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> INTHALF_R {
        INTHALF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> ESG_R {
        ESG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MAJORELINK_R {
        MAJORELINK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MAJORLINKCH_R {
        MAJORLINKCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BWC_R {
        BWC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub fn intmajor(&mut self) -> INTMAJOR_W {
        INTMAJOR_W { w: self }
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&mut self) -> INTHALF_W {
        INTHALF_W { w: self }
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W {
        DREQ_W { w: self }
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&mut self) -> ESG_W {
        ESG_W { w: self }
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&mut self) -> MAJORELINK_W {
        MAJORELINK_W { w: self }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&mut self) -> MAJORLINKCH_W {
        MAJORLINKCH_W { w: self }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&mut self) -> BWC_W {
        BWC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_csr](index.html) module"]
pub struct TCD4_CSR_SPEC;
impl crate::RegisterSpec for TCD4_CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd4_csr::R](R) reader structure"]
impl crate::Readable for TCD4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd4_csr::W](W) writer structure"]
impl crate::Writable for TCD4_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD4_CSR to value 0"]
impl crate::Resettable for TCD4_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
