#[doc = "Register `CTRL1_PN` reader"]
pub struct R(crate::R<CTRL1_PN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_PN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_PN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_PN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1_PN` writer"]
pub struct W(crate::W<CTRL1_PN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_PN_SPEC>;
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
impl From<crate::W<CTRL1_PN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_PN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filtering Combination Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCS_A {
    #[doc = "0: Message ID filtering only"]
    ID_FILTERING = 0,
    #[doc = "1: Message ID filtering and payload filtering"]
    ID_PAYLOAD_FILTERING = 1,
    #[doc = "2: Message ID filtering occurring a specified number of times"]
    ID_FILTERING_NUMBER = 2,
    #[doc = "3: Message ID filtering and payload filtering a specified number of times"]
    ID_PAYLOAD_FILTERING_NUMBER = 3,
}
impl From<FCS_A> for u8 {
    #[inline(always)]
    fn from(variant: FCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCS` reader - Filtering Combination Selection"]
pub struct FCS_R(crate::FieldReader<u8, FCS_A>);
impl FCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCS_A {
        match self.bits {
            0 => FCS_A::ID_FILTERING,
            1 => FCS_A::ID_PAYLOAD_FILTERING,
            2 => FCS_A::ID_FILTERING_NUMBER,
            3 => FCS_A::ID_PAYLOAD_FILTERING_NUMBER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_FILTERING`"]
    #[inline(always)]
    pub fn is_id_filtering(&self) -> bool {
        **self == FCS_A::ID_FILTERING
    }
    #[doc = "Checks if the value of the field is `ID_PAYLOAD_FILTERING`"]
    #[inline(always)]
    pub fn is_id_payload_filtering(&self) -> bool {
        **self == FCS_A::ID_PAYLOAD_FILTERING
    }
    #[doc = "Checks if the value of the field is `ID_FILTERING_NUMBER`"]
    #[inline(always)]
    pub fn is_id_filtering_number(&self) -> bool {
        **self == FCS_A::ID_FILTERING_NUMBER
    }
    #[doc = "Checks if the value of the field is `ID_PAYLOAD_FILTERING_NUMBER`"]
    #[inline(always)]
    pub fn is_id_payload_filtering_number(&self) -> bool {
        **self == FCS_A::ID_PAYLOAD_FILTERING_NUMBER
    }
}
impl core::ops::Deref for FCS_R {
    type Target = crate::FieldReader<u8, FCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCS` writer - Filtering Combination Selection"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Message ID filtering only"]
    #[inline(always)]
    pub fn id_filtering(self) -> &'a mut W {
        self.variant(FCS_A::ID_FILTERING)
    }
    #[doc = "Message ID filtering and payload filtering"]
    #[inline(always)]
    pub fn id_payload_filtering(self) -> &'a mut W {
        self.variant(FCS_A::ID_PAYLOAD_FILTERING)
    }
    #[doc = "Message ID filtering occurring a specified number of times"]
    #[inline(always)]
    pub fn id_filtering_number(self) -> &'a mut W {
        self.variant(FCS_A::ID_FILTERING_NUMBER)
    }
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    #[inline(always)]
    pub fn id_payload_filtering_number(self) -> &'a mut W {
        self.variant(FCS_A::ID_PAYLOAD_FILTERING_NUMBER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "ID Filtering Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDFS_A {
    #[doc = "0: Match upon ID contents against an exact target value"]
    MATCH_EXACT = 0,
    #[doc = "1: Match upon an ID value greater than or equal to a specified target value"]
    MATCH_GTE = 1,
    #[doc = "2: Match upon an ID value smaller than or equal to a specified target value"]
    MATCH_LTE = 2,
    #[doc = "3: Match upon an ID value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
    MATCH_RANGE = 3,
}
impl From<IDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IDFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDFS` reader - ID Filtering Selection"]
pub struct IDFS_R(crate::FieldReader<u8, IDFS_A>);
impl IDFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDFS_A {
        match self.bits {
            0 => IDFS_A::MATCH_EXACT,
            1 => IDFS_A::MATCH_GTE,
            2 => IDFS_A::MATCH_LTE,
            3 => IDFS_A::MATCH_RANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_EXACT`"]
    #[inline(always)]
    pub fn is_match_exact(&self) -> bool {
        **self == IDFS_A::MATCH_EXACT
    }
    #[doc = "Checks if the value of the field is `MATCH_GTE`"]
    #[inline(always)]
    pub fn is_match_gte(&self) -> bool {
        **self == IDFS_A::MATCH_GTE
    }
    #[doc = "Checks if the value of the field is `MATCH_LTE`"]
    #[inline(always)]
    pub fn is_match_lte(&self) -> bool {
        **self == IDFS_A::MATCH_LTE
    }
    #[doc = "Checks if the value of the field is `MATCH_RANGE`"]
    #[inline(always)]
    pub fn is_match_range(&self) -> bool {
        **self == IDFS_A::MATCH_RANGE
    }
}
impl core::ops::Deref for IDFS_R {
    type Target = crate::FieldReader<u8, IDFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDFS` writer - ID Filtering Selection"]
pub struct IDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDFS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Match upon ID contents against an exact target value"]
    #[inline(always)]
    pub fn match_exact(self) -> &'a mut W {
        self.variant(IDFS_A::MATCH_EXACT)
    }
    #[doc = "Match upon an ID value greater than or equal to a specified target value"]
    #[inline(always)]
    pub fn match_gte(self) -> &'a mut W {
        self.variant(IDFS_A::MATCH_GTE)
    }
    #[doc = "Match upon an ID value smaller than or equal to a specified target value"]
    #[inline(always)]
    pub fn match_lte(self) -> &'a mut W {
        self.variant(IDFS_A::MATCH_LTE)
    }
    #[doc = "Match upon an ID value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
    #[inline(always)]
    pub fn match_range(self) -> &'a mut W {
        self.variant(IDFS_A::MATCH_RANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Payload Filtering Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLFS_A {
    #[doc = "0: Match upon a payload contents against an exact target value"]
    MATCH_EXACT = 0,
    #[doc = "1: Match upon a payload value greater than or equal to a specified target value"]
    MATCH_GTE = 1,
    #[doc = "2: Match upon a payload value smaller than or equal to a specified target value"]
    MATCH_LTE = 2,
    #[doc = "3: Match upon a payload value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
    MATCH_RANGE = 3,
}
impl From<PLFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLFS` reader - Payload Filtering Selection"]
pub struct PLFS_R(crate::FieldReader<u8, PLFS_A>);
impl PLFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLFS_A {
        match self.bits {
            0 => PLFS_A::MATCH_EXACT,
            1 => PLFS_A::MATCH_GTE,
            2 => PLFS_A::MATCH_LTE,
            3 => PLFS_A::MATCH_RANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_EXACT`"]
    #[inline(always)]
    pub fn is_match_exact(&self) -> bool {
        **self == PLFS_A::MATCH_EXACT
    }
    #[doc = "Checks if the value of the field is `MATCH_GTE`"]
    #[inline(always)]
    pub fn is_match_gte(&self) -> bool {
        **self == PLFS_A::MATCH_GTE
    }
    #[doc = "Checks if the value of the field is `MATCH_LTE`"]
    #[inline(always)]
    pub fn is_match_lte(&self) -> bool {
        **self == PLFS_A::MATCH_LTE
    }
    #[doc = "Checks if the value of the field is `MATCH_RANGE`"]
    #[inline(always)]
    pub fn is_match_range(&self) -> bool {
        **self == PLFS_A::MATCH_RANGE
    }
}
impl core::ops::Deref for PLFS_R {
    type Target = crate::FieldReader<u8, PLFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLFS` writer - Payload Filtering Selection"]
pub struct PLFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLFS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Match upon a payload contents against an exact target value"]
    #[inline(always)]
    pub fn match_exact(self) -> &'a mut W {
        self.variant(PLFS_A::MATCH_EXACT)
    }
    #[doc = "Match upon a payload value greater than or equal to a specified target value"]
    #[inline(always)]
    pub fn match_gte(self) -> &'a mut W {
        self.variant(PLFS_A::MATCH_GTE)
    }
    #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
    #[inline(always)]
    pub fn match_lte(self) -> &'a mut W {
        self.variant(PLFS_A::MATCH_LTE)
    }
    #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
    #[inline(always)]
    pub fn match_range(self) -> &'a mut W {
        self.variant(PLFS_A::MATCH_RANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Number of Messages Matching the Same Filtering Criteria\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMATCH_A {
    #[doc = "1: Received message must match the predefined filtering criteria for ID and/or PL once before generating a wakeup event."]
    MATCH_1 = 1,
    #[doc = "2: Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wakeup event."]
    MATCH_2 = 2,
    #[doc = "255: Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wakeup event."]
    MATCH_255 = 255,
}
impl From<NMATCH_A> for u8 {
    #[inline(always)]
    fn from(variant: NMATCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NMATCH` reader - Number of Messages Matching the Same Filtering Criteria"]
pub struct NMATCH_R(crate::FieldReader<u8, NMATCH_A>);
impl NMATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NMATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMATCH_A> {
        match self.bits {
            1 => Some(NMATCH_A::MATCH_1),
            2 => Some(NMATCH_A::MATCH_2),
            255 => Some(NMATCH_A::MATCH_255),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_1`"]
    #[inline(always)]
    pub fn is_match_1(&self) -> bool {
        **self == NMATCH_A::MATCH_1
    }
    #[doc = "Checks if the value of the field is `MATCH_2`"]
    #[inline(always)]
    pub fn is_match_2(&self) -> bool {
        **self == NMATCH_A::MATCH_2
    }
    #[doc = "Checks if the value of the field is `MATCH_255`"]
    #[inline(always)]
    pub fn is_match_255(&self) -> bool {
        **self == NMATCH_A::MATCH_255
    }
}
impl core::ops::Deref for NMATCH_R {
    type Target = crate::FieldReader<u8, NMATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMATCH` writer - Number of Messages Matching the Same Filtering Criteria"]
pub struct NMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMATCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wakeup event."]
    #[inline(always)]
    pub fn match_1(self) -> &'a mut W {
        self.variant(NMATCH_A::MATCH_1)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wakeup event."]
    #[inline(always)]
    pub fn match_2(self) -> &'a mut W {
        self.variant(NMATCH_A::MATCH_2)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wakeup event."]
    #[inline(always)]
    pub fn match_255(self) -> &'a mut W {
        self.variant(NMATCH_A::MATCH_255)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Wake Up by Match Flag Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMF_MSK_A {
    #[doc = "0: Wakeup match event is disabled"]
    DISABLE = 0,
    #[doc = "1: Wakeup match event is enabled"]
    ENABLE = 1,
}
impl From<WUMF_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: WUMF_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUMF_MSK` reader - Wake Up by Match Flag Mask Bit"]
pub struct WUMF_MSK_R(crate::FieldReader<bool, WUMF_MSK_A>);
impl WUMF_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUMF_MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUMF_MSK_A {
        match self.bits {
            false => WUMF_MSK_A::DISABLE,
            true => WUMF_MSK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WUMF_MSK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WUMF_MSK_A::ENABLE
    }
}
impl core::ops::Deref for WUMF_MSK_R {
    type Target = crate::FieldReader<bool, WUMF_MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUMF_MSK` writer - Wake Up by Match Flag Mask Bit"]
pub struct WUMF_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WUMF_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUMF_MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wakeup match event is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WUMF_MSK_A::DISABLE)
    }
    #[doc = "Wakeup match event is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WUMF_MSK_A::ENABLE)
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
#[doc = "Wake Up by Timeout Flag Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOF_MSK_A {
    #[doc = "0: Timeout wakeup event is disabled"]
    DISABLE = 0,
    #[doc = "1: Timeout wakeup event is enabled"]
    ENABLE = 1,
}
impl From<WTOF_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: WTOF_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTOF_MSK` reader - Wake Up by Timeout Flag Mask Bit"]
pub struct WTOF_MSK_R(crate::FieldReader<bool, WTOF_MSK_A>);
impl WTOF_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WTOF_MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTOF_MSK_A {
        match self.bits {
            false => WTOF_MSK_A::DISABLE,
            true => WTOF_MSK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WTOF_MSK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WTOF_MSK_A::ENABLE
    }
}
impl core::ops::Deref for WTOF_MSK_R {
    type Target = crate::FieldReader<bool, WTOF_MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTOF_MSK` writer - Wake Up by Timeout Flag Mask Bit"]
pub struct WTOF_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOF_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTOF_MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timeout wakeup event is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WTOF_MSK_A::DISABLE)
    }
    #[doc = "Timeout wakeup event is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WTOF_MSK_A::ENABLE)
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
impl R {
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline(always)]
    pub fn idfs(&self) -> IDFS_R {
        IDFS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline(always)]
    pub fn plfs(&self) -> PLFS_R {
        PLFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline(always)]
    pub fn nmatch(&self) -> NMATCH_R {
        NMATCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline(always)]
    pub fn wumf_msk(&self) -> WUMF_MSK_R {
        WUMF_MSK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline(always)]
    pub fn wtof_msk(&self) -> WTOF_MSK_R {
        WTOF_MSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline(always)]
    pub fn idfs(&mut self) -> IDFS_W {
        IDFS_W { w: self }
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline(always)]
    pub fn plfs(&mut self) -> PLFS_W {
        PLFS_W { w: self }
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline(always)]
    pub fn nmatch(&mut self) -> NMATCH_W {
        NMATCH_W { w: self }
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline(always)]
    pub fn wumf_msk(&mut self) -> WUMF_MSK_W {
        WUMF_MSK_W { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline(always)]
    pub fn wtof_msk(&mut self) -> WTOF_MSK_W {
        WTOF_MSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1_pn](index.html) module"]
pub struct CTRL1_PN_SPEC;
impl crate::RegisterSpec for CTRL1_PN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1_pn::R](R) reader structure"]
impl crate::Readable for CTRL1_PN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1_pn::W](W) writer structure"]
impl crate::Writable for CTRL1_PN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1_PN to value 0x0100"]
impl crate::Resettable for CTRL1_PN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
