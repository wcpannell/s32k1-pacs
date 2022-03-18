#[doc = "Register `C0` reader"]
pub struct R(crate::R<C0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0` writer"]
pub struct W(crate::W<C0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0_SPEC>;
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
impl From<crate::W<C0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hysteresis value with each level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYSTCTR_A {
    #[doc = "0: The hard block output has level 0 hysteresis internally."]
    HYSTCTR_0 = 0,
    #[doc = "1: The hard block output has level 1 hysteresis internally."]
    HYSTCTR_1 = 1,
    #[doc = "2: The hard block output has level 2 hysteresis internally."]
    HYSTCTR_2 = 2,
    #[doc = "3: The hard block output has level 3 hysteresis internally."]
    HYSTCTR_3 = 3,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control. See chip data sheet to get the actual hysteresis value with each level"]
pub struct HYSTCTR_R(crate::FieldReader<u8, HYSTCTR_A>);
impl HYSTCTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HYSTCTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::HYSTCTR_0,
            1 => HYSTCTR_A::HYSTCTR_1,
            2 => HYSTCTR_A::HYSTCTR_2,
            3 => HYSTCTR_A::HYSTCTR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
    #[inline(always)]
    pub fn is_hystctr_0(&self) -> bool {
        **self == HYSTCTR_A::HYSTCTR_0
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
    #[inline(always)]
    pub fn is_hystctr_1(&self) -> bool {
        **self == HYSTCTR_A::HYSTCTR_1
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
    #[inline(always)]
    pub fn is_hystctr_2(&self) -> bool {
        **self == HYSTCTR_A::HYSTCTR_2
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
    #[inline(always)]
    pub fn is_hystctr_3(&self) -> bool {
        **self == HYSTCTR_A::HYSTCTR_3
    }
}
impl core::ops::Deref for HYSTCTR_R {
    type Target = crate::FieldReader<u8, HYSTCTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control. See chip data sheet to get the actual hysteresis value with each level"]
pub struct HYSTCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSTCTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The hard block output has level 0 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_0(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_0)
    }
    #[doc = "The hard block output has level 1 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_1(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_1)
    }
    #[doc = "The hard block output has level 2 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_2(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_2)
    }
    #[doc = "The hard block output has level 3 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_3(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Comparator hard block offset control. See chip data sheet to get the actual offset value with each level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSET_A {
    #[doc = "0: The comparator hard block output has level 0 offset internally."]
    OFFSET_0 = 0,
    #[doc = "1: The comparator hard block output has level 1 offset internally."]
    OFFSET_1 = 1,
}
impl From<OFFSET_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSET` reader - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
pub struct OFFSET_R(crate::FieldReader<bool, OFFSET_A>);
impl OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSET_A {
        match self.bits {
            false => OFFSET_A::OFFSET_0,
            true => OFFSET_A::OFFSET_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_0`"]
    #[inline(always)]
    pub fn is_offset_0(&self) -> bool {
        **self == OFFSET_A::OFFSET_0
    }
    #[doc = "Checks if the value of the field is `OFFSET_1`"]
    #[inline(always)]
    pub fn is_offset_1(&self) -> bool {
        **self == OFFSET_A::OFFSET_1
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<bool, OFFSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The comparator hard block output has level 0 offset internally."]
    #[inline(always)]
    pub fn offset_0(self) -> &'a mut W {
        self.variant(OFFSET_A::OFFSET_0)
    }
    #[doc = "The comparator hard block output has level 1 offset internally."]
    #[inline(always)]
    pub fn offset_1(self) -> &'a mut W {
        self.variant(OFFSET_A::OFFSET_1)
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
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0,
    #[doc = "1: 1 consecutive sample must agree (comparator output is simply sampled)."]
    FILTER_CNT_1 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    FILTER_CNT_2 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    FILTER_CNT_3 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    FILTER_CNT_4 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    FILTER_CNT_5 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    FILTER_CNT_6 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    FILTER_CNT_7 = 7,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTER_CNT` reader - Filter Sample Count"]
pub struct FILTER_CNT_R(crate::FieldReader<u8, FILTER_CNT_A>);
impl FILTER_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::FILTER_CNT_0,
            1 => FILTER_CNT_A::FILTER_CNT_1,
            2 => FILTER_CNT_A::FILTER_CNT_2,
            3 => FILTER_CNT_A::FILTER_CNT_3,
            4 => FILTER_CNT_A::FILTER_CNT_4,
            5 => FILTER_CNT_A::FILTER_CNT_5,
            6 => FILTER_CNT_A::FILTER_CNT_6,
            7 => FILTER_CNT_A::FILTER_CNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_0`"]
    #[inline(always)]
    pub fn is_filter_cnt_0(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_0
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_1`"]
    #[inline(always)]
    pub fn is_filter_cnt_1(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_1
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_2`"]
    #[inline(always)]
    pub fn is_filter_cnt_2(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_2
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_3`"]
    #[inline(always)]
    pub fn is_filter_cnt_3(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_3
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_4`"]
    #[inline(always)]
    pub fn is_filter_cnt_4(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_4
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_5`"]
    #[inline(always)]
    pub fn is_filter_cnt_5(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_5
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_6`"]
    #[inline(always)]
    pub fn is_filter_cnt_6(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_6
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_7`"]
    #[inline(always)]
    pub fn is_filter_cnt_7(&self) -> bool {
        **self == FILTER_CNT_A::FILTER_CNT_7
    }
}
impl core::ops::Deref for FILTER_CNT_R {
    type Target = crate::FieldReader<u8, FILTER_CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_CNT` writer - Filter Sample Count"]
pub struct FILTER_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_CNT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn filter_cnt_0(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_0)
    }
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    #[inline(always)]
    pub fn filter_cnt_1(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_2(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_3(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_4(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_5(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_6(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_7(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Analog Comparator is disabled."]
    EN_0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        **self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        **self == EN_A::EN_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
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
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPE_A {
    #[doc = "0: When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    OPE_0 = 0,
    #[doc = "1: When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    OPE_1 = 1,
}
impl From<OPE_A> for bool {
    #[inline(always)]
    fn from(variant: OPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub struct OPE_R(crate::FieldReader<bool, OPE_A>);
impl OPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPE_A {
        match self.bits {
            false => OPE_A::OPE_0,
            true => OPE_A::OPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OPE_0`"]
    #[inline(always)]
    pub fn is_ope_0(&self) -> bool {
        **self == OPE_A::OPE_0
    }
    #[doc = "Checks if the value of the field is `OPE_1`"]
    #[inline(always)]
    pub fn is_ope_1(&self) -> bool {
        **self == OPE_A::OPE_1
    }
}
impl core::ops::Deref for OPE_R {
    type Target = crate::FieldReader<bool, OPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub struct OPE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    #[inline(always)]
    pub fn ope_0(self) -> &'a mut W {
        self.variant(OPE_A::OPE_0)
    }
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    #[inline(always)]
    pub fn ope_1(self) -> &'a mut W {
        self.variant(OPE_A::OPE_1)
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
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COS_A {
    #[doc = "0: Set CMPO to equal COUT (filtered comparator output)."]
    COS_0 = 0,
    #[doc = "1: Set CMPO to equal COUTA (unfiltered comparator output)."]
    COS_1 = 1,
}
impl From<COS_A> for bool {
    #[inline(always)]
    fn from(variant: COS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub struct COS_R(crate::FieldReader<bool, COS_A>);
impl COS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COS_A {
        match self.bits {
            false => COS_A::COS_0,
            true => COS_A::COS_1,
        }
    }
    #[doc = "Checks if the value of the field is `COS_0`"]
    #[inline(always)]
    pub fn is_cos_0(&self) -> bool {
        **self == COS_A::COS_0
    }
    #[doc = "Checks if the value of the field is `COS_1`"]
    #[inline(always)]
    pub fn is_cos_1(&self) -> bool {
        **self == COS_A::COS_1
    }
}
impl core::ops::Deref for COS_R {
    type Target = crate::FieldReader<bool, COS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub struct COS_W<'a> {
    w: &'a mut W,
}
impl<'a> COS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    #[inline(always)]
    pub fn cos_0(self) -> &'a mut W {
        self.variant(COS_A::COS_0)
    }
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    #[inline(always)]
    pub fn cos_1(self) -> &'a mut W {
        self.variant(COS_A::COS_1)
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
#[doc = "Comparator invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVT_A {
    #[doc = "0: Does not invert the comparator output."]
    INVT_0 = 0,
    #[doc = "1: Inverts the comparator output."]
    INVT_1 = 1,
}
impl From<INVT_A> for bool {
    #[inline(always)]
    fn from(variant: INVT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVT` reader - Comparator invert"]
pub struct INVT_R(crate::FieldReader<bool, INVT_A>);
impl INVT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVT_A {
        match self.bits {
            false => INVT_A::INVT_0,
            true => INVT_A::INVT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVT_0`"]
    #[inline(always)]
    pub fn is_invt_0(&self) -> bool {
        **self == INVT_A::INVT_0
    }
    #[doc = "Checks if the value of the field is `INVT_1`"]
    #[inline(always)]
    pub fn is_invt_1(&self) -> bool {
        **self == INVT_A::INVT_1
    }
}
impl core::ops::Deref for INVT_R {
    type Target = crate::FieldReader<bool, INVT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVT` writer - Comparator invert"]
pub struct INVT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn invt_0(self) -> &'a mut W {
        self.variant(INVT_A::INVT_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn invt_1(self) -> &'a mut W {
        self.variant(INVT_A::INVT_1)
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
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODE_A {
    #[doc = "0: Low Speed (LS) comparison mode is selected."]
    PMODE_0 = 0,
    #[doc = "1: High Speed (HS) comparison mode is selected, in VLPx mode, or Stop mode switched to Low Speed (LS) mode."]
    PMODE_1 = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub struct PMODE_R(crate::FieldReader<bool, PMODE_A>);
impl PMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::PMODE_0,
            true => PMODE_A::PMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMODE_0`"]
    #[inline(always)]
    pub fn is_pmode_0(&self) -> bool {
        **self == PMODE_A::PMODE_0
    }
    #[doc = "Checks if the value of the field is `PMODE_1`"]
    #[inline(always)]
    pub fn is_pmode_1(&self) -> bool {
        **self == PMODE_A::PMODE_1
    }
}
impl core::ops::Deref for PMODE_R {
    type Target = crate::FieldReader<bool, PMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub struct PMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Speed (LS) comparison mode is selected."]
    #[inline(always)]
    pub fn pmode_0(self) -> &'a mut W {
        self.variant(PMODE_A::PMODE_0)
    }
    #[doc = "High Speed (HS) comparison mode is selected, in VLPx mode, or Stop mode switched to Low Speed (LS) mode."]
    #[inline(always)]
    pub fn pmode_1(self) -> &'a mut W {
        self.variant(PMODE_A::PMODE_1)
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
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_A {
    #[doc = "0: Windowing mode is not selected."]
    WE_0 = 0,
    #[doc = "1: Windowing mode is selected."]
    WE_1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub struct WE_R(crate::FieldReader<bool, WE_A>);
impl WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::WE_0,
            true => WE_A::WE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WE_0`"]
    #[inline(always)]
    pub fn is_we_0(&self) -> bool {
        **self == WE_A::WE_0
    }
    #[doc = "Checks if the value of the field is `WE_1`"]
    #[inline(always)]
    pub fn is_we_1(&self) -> bool {
        **self == WE_A::WE_1
    }
}
impl core::ops::Deref for WE_R {
    type Target = crate::FieldReader<bool, WE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn we_0(self) -> &'a mut W {
        self.variant(WE_A::WE_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn we_1(self) -> &'a mut W {
        self.variant(WE_A::WE_1)
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
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    #[doc = "0: Sampling mode is not selected."]
    SE_0 = 0,
    #[doc = "1: Sampling mode is selected."]
    SE_1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub struct SE_R(crate::FieldReader<bool, SE_A>);
impl SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::SE_0,
            true => SE_A::SE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SE_0`"]
    #[inline(always)]
    pub fn is_se_0(&self) -> bool {
        **self == SE_A::SE_0
    }
    #[doc = "Checks if the value of the field is `SE_1`"]
    #[inline(always)]
    pub fn is_se_1(&self) -> bool {
        **self == SE_A::SE_1
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<bool, SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn se_0(self) -> &'a mut W {
        self.variant(SE_A::SE_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn se_1(self) -> &'a mut W {
        self.variant(SE_A::SE_1)
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
#[doc = "Field `FPR` reader - Filter Sample Period"]
pub struct FPR_R(crate::FieldReader<u8, u8>);
impl FPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPR` writer - Filter Sample Period"]
pub struct FPR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `COUT` reader - Analog Comparator Output"]
pub struct COUT_R(crate::FieldReader<bool, bool>);
impl COUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFF_A {
    #[doc = "0: A falling edge has not been detected on COUT."]
    CFF_0 = 0,
    #[doc = "1: A falling edge on COUT has occurred."]
    CFF_1 = 1,
}
impl From<CFF_A> for bool {
    #[inline(always)]
    fn from(variant: CFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFF` reader - Analog Comparator Flag Falling"]
pub struct CFF_R(crate::FieldReader<bool, CFF_A>);
impl CFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFF_A {
        match self.bits {
            false => CFF_A::CFF_0,
            true => CFF_A::CFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFF_0`"]
    #[inline(always)]
    pub fn is_cff_0(&self) -> bool {
        **self == CFF_A::CFF_0
    }
    #[doc = "Checks if the value of the field is `CFF_1`"]
    #[inline(always)]
    pub fn is_cff_1(&self) -> bool {
        **self == CFF_A::CFF_1
    }
}
impl core::ops::Deref for CFF_R {
    type Target = crate::FieldReader<bool, CFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFF` writer - Analog Comparator Flag Falling"]
pub struct CFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A falling edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cff_0(self) -> &'a mut W {
        self.variant(CFF_A::CFF_0)
    }
    #[doc = "A falling edge on COUT has occurred."]
    #[inline(always)]
    pub fn cff_1(self) -> &'a mut W {
        self.variant(CFF_A::CFF_1)
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
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR_A {
    #[doc = "0: A rising edge has not been detected on COUT."]
    CFR_0 = 0,
    #[doc = "1: A rising edge on COUT has occurred."]
    CFR_1 = 1,
}
impl From<CFR_A> for bool {
    #[inline(always)]
    fn from(variant: CFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFR` reader - Analog Comparator Flag Rising"]
pub struct CFR_R(crate::FieldReader<bool, CFR_A>);
impl CFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFR_A {
        match self.bits {
            false => CFR_A::CFR_0,
            true => CFR_A::CFR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFR_0`"]
    #[inline(always)]
    pub fn is_cfr_0(&self) -> bool {
        **self == CFR_A::CFR_0
    }
    #[doc = "Checks if the value of the field is `CFR_1`"]
    #[inline(always)]
    pub fn is_cfr_1(&self) -> bool {
        **self == CFR_A::CFR_1
    }
}
impl core::ops::Deref for CFR_R {
    type Target = crate::FieldReader<bool, CFR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFR` writer - Analog Comparator Flag Rising"]
pub struct CFR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A rising edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cfr_0(self) -> &'a mut W {
        self.variant(CFR_A::CFR_0)
    }
    #[doc = "A rising edge on COUT has occurred."]
    #[inline(always)]
    pub fn cfr_1(self) -> &'a mut W {
        self.variant(CFR_A::CFR_1)
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
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEF_A {
    #[doc = "0: Interrupt is disabled."]
    IEF_0 = 0,
    #[doc = "1: Interrupt is enabled."]
    IEF_1 = 1,
}
impl From<IEF_A> for bool {
    #[inline(always)]
    fn from(variant: IEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEF` reader - Comparator Interrupt Enable Falling"]
pub struct IEF_R(crate::FieldReader<bool, IEF_A>);
impl IEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEF_A {
        match self.bits {
            false => IEF_A::IEF_0,
            true => IEF_A::IEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEF_0`"]
    #[inline(always)]
    pub fn is_ief_0(&self) -> bool {
        **self == IEF_A::IEF_0
    }
    #[doc = "Checks if the value of the field is `IEF_1`"]
    #[inline(always)]
    pub fn is_ief_1(&self) -> bool {
        **self == IEF_A::IEF_1
    }
}
impl core::ops::Deref for IEF_R {
    type Target = crate::FieldReader<bool, IEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEF` writer - Comparator Interrupt Enable Falling"]
pub struct IEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ief_0(self) -> &'a mut W {
        self.variant(IEF_A::IEF_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ief_1(self) -> &'a mut W {
        self.variant(IEF_A::IEF_1)
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
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IER_A {
    #[doc = "0: Interrupt is disabled."]
    IER_0 = 0,
    #[doc = "1: Interrupt is enabled."]
    IER_1 = 1,
}
impl From<IER_A> for bool {
    #[inline(always)]
    fn from(variant: IER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IER` reader - Comparator Interrupt Enable Rising"]
pub struct IER_R(crate::FieldReader<bool, IER_A>);
impl IER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IER_A {
        match self.bits {
            false => IER_A::IER_0,
            true => IER_A::IER_1,
        }
    }
    #[doc = "Checks if the value of the field is `IER_0`"]
    #[inline(always)]
    pub fn is_ier_0(&self) -> bool {
        **self == IER_A::IER_0
    }
    #[doc = "Checks if the value of the field is `IER_1`"]
    #[inline(always)]
    pub fn is_ier_1(&self) -> bool {
        **self == IER_A::IER_1
    }
}
impl core::ops::Deref for IER_R {
    type Target = crate::FieldReader<bool, IER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IER` writer - Comparator Interrupt Enable Rising"]
pub struct IER_W<'a> {
    w: &'a mut W,
}
impl<'a> IER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ier_0(self) -> &'a mut W {
        self.variant(IER_A::IER_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ier_1(self) -> &'a mut W {
        self.variant(IER_A::IER_1)
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
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    DMAEN_0 = 0,
    #[doc = "1: DMA is enabled."]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        **self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        **self == DMAEN_A::DMAEN_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
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
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hysteresis value with each level"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OPE_R {
        OPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    pub fn invt(&self) -> INVT_R {
        INVT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CFF_R {
        CFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CFR_R {
        CFR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IER_R {
        IER_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hysteresis value with each level"]
    #[inline(always)]
    pub fn hystctr(&mut self) -> HYSTCTR_W {
        HYSTCTR_W { w: self }
    }
    #[doc = "Bit 2 - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W {
        FILTER_CNT_W { w: self }
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&mut self) -> OPE_W {
        OPE_W { w: self }
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&mut self) -> COS_W {
        COS_W { w: self }
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    pub fn invt(&mut self) -> INVT_W {
        INVT_W { w: self }
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W { w: self }
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    pub fn fpr(&mut self) -> FPR_W {
        FPR_W { w: self }
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&mut self) -> CFF_W {
        CFF_W { w: self }
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&mut self) -> CFR_W {
        CFR_W { w: self }
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&mut self) -> IEF_W {
        IEF_W { w: self }
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&mut self) -> IER_W {
        IER_W { w: self }
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](index.html) module"]
pub struct C0_SPEC;
impl crate::RegisterSpec for C0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0::R](R) reader structure"]
impl crate::Readable for C0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0::W](W) writer structure"]
impl crate::Writable for C0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
