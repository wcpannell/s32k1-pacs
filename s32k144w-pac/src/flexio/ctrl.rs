#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FlexIO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXEN_A {
    #[doc = "0: FlexIO module is disabled."]
    FLEXEN_0 = 0,
    #[doc = "1: FlexIO module is enabled."]
    FLEXEN_1 = 1,
}
impl From<FLEXEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXEN` reader - FlexIO Enable"]
pub struct FLEXEN_R(crate::FieldReader<bool, FLEXEN_A>);
impl FLEXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLEXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXEN_A {
        match self.bits {
            false => FLEXEN_A::FLEXEN_0,
            true => FLEXEN_A::FLEXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXEN_0`"]
    #[inline(always)]
    pub fn is_flexen_0(&self) -> bool {
        **self == FLEXEN_A::FLEXEN_0
    }
    #[doc = "Checks if the value of the field is `FLEXEN_1`"]
    #[inline(always)]
    pub fn is_flexen_1(&self) -> bool {
        **self == FLEXEN_A::FLEXEN_1
    }
}
impl core::ops::Deref for FLEXEN_R {
    type Target = crate::FieldReader<bool, FLEXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXEN` writer - FlexIO Enable"]
pub struct FLEXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FlexIO module is disabled."]
    #[inline(always)]
    pub fn flexen_0(self) -> &'a mut W {
        self.variant(FLEXEN_A::FLEXEN_0)
    }
    #[doc = "FlexIO module is enabled."]
    #[inline(always)]
    pub fn flexen_1(self) -> &'a mut W {
        self.variant(FLEXEN_A::FLEXEN_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
    #[doc = "0: Software reset is disabled"]
    SWRST_0 = 0,
    #[doc = "1: Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    SWRST_1 = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, SWRST_A>);
impl SWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::SWRST_0,
            true => SWRST_A::SWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWRST_0`"]
    #[inline(always)]
    pub fn is_swrst_0(&self) -> bool {
        **self == SWRST_A::SWRST_0
    }
    #[doc = "Checks if the value of the field is `SWRST_1`"]
    #[inline(always)]
    pub fn is_swrst_1(&self) -> bool {
        **self == SWRST_A::SWRST_1
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, SWRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software reset is disabled"]
    #[inline(always)]
    pub fn swrst_0(self) -> &'a mut W {
        self.variant(SWRST_A::SWRST_0)
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline(always)]
    pub fn swrst_1(self) -> &'a mut W {
        self.variant(SWRST_A::SWRST_1)
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
#[doc = "Fast Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTACC_A {
    #[doc = "0: Configures for normal register accesses to FlexIO"]
    FASTACC_0 = 0,
    #[doc = "1: Configures for fast register accesses to FlexIO"]
    FASTACC_1 = 1,
}
impl From<FASTACC_A> for bool {
    #[inline(always)]
    fn from(variant: FASTACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTACC` reader - Fast Access"]
pub struct FASTACC_R(crate::FieldReader<bool, FASTACC_A>);
impl FASTACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FASTACC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTACC_A {
        match self.bits {
            false => FASTACC_A::FASTACC_0,
            true => FASTACC_A::FASTACC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FASTACC_0`"]
    #[inline(always)]
    pub fn is_fastacc_0(&self) -> bool {
        **self == FASTACC_A::FASTACC_0
    }
    #[doc = "Checks if the value of the field is `FASTACC_1`"]
    #[inline(always)]
    pub fn is_fastacc_1(&self) -> bool {
        **self == FASTACC_A::FASTACC_1
    }
}
impl core::ops::Deref for FASTACC_R {
    type Target = crate::FieldReader<bool, FASTACC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTACC` writer - Fast Access"]
pub struct FASTACC_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FASTACC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline(always)]
    pub fn fastacc_0(self) -> &'a mut W {
        self.variant(FASTACC_A::FASTACC_0)
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline(always)]
    pub fn fastacc_1(self) -> &'a mut W {
        self.variant(FASTACC_A::FASTACC_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGE_A {
    #[doc = "0: FlexIO is disabled in debug modes."]
    DBGE_0 = 0,
    #[doc = "1: FlexIO is enabled in debug modes"]
    DBGE_1 = 1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub struct DBGE_R(crate::FieldReader<bool, DBGE_A>);
impl DBGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::DBGE_0,
            true => DBGE_A::DBGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGE_0`"]
    #[inline(always)]
    pub fn is_dbge_0(&self) -> bool {
        **self == DBGE_A::DBGE_0
    }
    #[doc = "Checks if the value of the field is `DBGE_1`"]
    #[inline(always)]
    pub fn is_dbge_1(&self) -> bool {
        **self == DBGE_A::DBGE_1
    }
}
impl core::ops::Deref for DBGE_R {
    type Target = crate::FieldReader<bool, DBGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub struct DBGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline(always)]
    pub fn dbge_0(self) -> &'a mut W {
        self.variant(DBGE_A::DBGE_0)
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline(always)]
    pub fn dbge_1(self) -> &'a mut W {
        self.variant(DBGE_A::DBGE_1)
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
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
    #[doc = "0: FlexIO enabled in Doze modes."]
    DOZEN_0 = 0,
    #[doc = "1: FlexIO disabled in Doze modes."]
    DOZEN_1 = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub struct DOZEN_R(crate::FieldReader<bool, DOZEN_A>);
impl DOZEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::DOZEN_0,
            true => DOZEN_A::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        **self == DOZEN_A::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        **self == DOZEN_A::DOZEN_1
    }
}
impl core::ops::Deref for DOZEN_R {
    type Target = crate::FieldReader<bool, DOZEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub struct DOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_0)
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_1)
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
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    pub fn flexen(&self) -> FLEXEN_R {
        FLEXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    pub fn fastacc(&self) -> FASTACC_R {
        FASTACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    pub fn flexen(&mut self) -> FLEXEN_W {
        FLEXEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    pub fn fastacc(&mut self) -> FASTACC_W {
        FASTACC_W { w: self }
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&mut self) -> DBGE_W {
        DBGE_W { w: self }
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DOZEN_W {
        DOZEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FlexIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
