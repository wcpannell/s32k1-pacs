#[doc = "Register `FCT0` reader"]
pub struct R(crate::R<FCT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCT0` writer"]
pub struct W(crate::W<FCT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCT0_SPEC>;
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
impl From<crate::W<FCT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: Disabled."]
    _0000 = 0,
    #[doc = "4: Instruction fetch."]
    _0100 = 4,
    #[doc = "5: Data operand read."]
    _0101 = 5,
    #[doc = "6: Data operand write."]
    _0110 = 6,
    #[doc = "7: Data operand (read + write)."]
    _0111 = 7,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNCTION` reader - Function"]
pub struct FUNCTION_R(crate::FieldReader<u8, FUNCTION_A>);
impl FUNCTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNCTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNCTION_A> {
        match self.bits {
            0 => Some(FUNCTION_A::_0000),
            4 => Some(FUNCTION_A::_0100),
            5 => Some(FUNCTION_A::_0101),
            6 => Some(FUNCTION_A::_0110),
            7 => Some(FUNCTION_A::_0111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == FUNCTION_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == FUNCTION_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == FUNCTION_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == FUNCTION_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == FUNCTION_A::_0111
    }
}
impl core::ops::Deref for FUNCTION_R {
    type Target = crate::FieldReader<u8, FUNCTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNCTION` writer - Function"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0000)
    }
    #[doc = "Instruction fetch."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0100)
    }
    #[doc = "Data operand read."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0101)
    }
    #[doc = "Data operand write."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0110)
    }
    #[doc = "Data operand (read + write)."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(FUNCTION_A::_0111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Data Value Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAVMATCH_A {
    #[doc = "0: Perform address comparison."]
    _0 = 0,
    #[doc = "1: Perform data value comparison."]
    _1 = 1,
}
impl From<DATAVMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: DATAVMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAVMATCH` reader - Data Value Match"]
pub struct DATAVMATCH_R(crate::FieldReader<bool, DATAVMATCH_A>);
impl DATAVMATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAVMATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAVMATCH_A {
        match self.bits {
            false => DATAVMATCH_A::_0,
            true => DATAVMATCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATAVMATCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATAVMATCH_A::_1
    }
}
impl core::ops::Deref for DATAVMATCH_R {
    type Target = crate::FieldReader<bool, DATAVMATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAVMATCH` writer - Data Value Match"]
pub struct DATAVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAVMATCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Perform address comparison."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATAVMATCH_A::_0)
    }
    #[doc = "Perform data value comparison."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATAVMATCH_A::_1)
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
#[doc = "Data Value Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAVSIZE_A {
    #[doc = "0: Byte."]
    _00 = 0,
    #[doc = "1: Halfword."]
    _01 = 1,
    #[doc = "2: Word."]
    _10 = 2,
    #[doc = "3: Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    _11 = 3,
}
impl From<DATAVSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAVSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATAVSIZE` reader - Data Value Size"]
pub struct DATAVSIZE_R(crate::FieldReader<u8, DATAVSIZE_A>);
impl DATAVSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATAVSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAVSIZE_A {
        match self.bits {
            0 => DATAVSIZE_A::_00,
            1 => DATAVSIZE_A::_01,
            2 => DATAVSIZE_A::_10,
            3 => DATAVSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == DATAVSIZE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == DATAVSIZE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == DATAVSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DATAVSIZE_A::_11
    }
}
impl core::ops::Deref for DATAVSIZE_R {
    type Target = crate::FieldReader<u8, DATAVSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAVSIZE` writer - Data Value Size"]
pub struct DATAVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAVSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_00)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_01)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_10)
    }
    #[doc = "Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DATAVSIZE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `DATAVADDR0` reader - Data Value Address 0"]
pub struct DATAVADDR0_R(crate::FieldReader<u8, u8>);
impl DATAVADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATAVADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAVADDR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAVADDR0` writer - Data Value Address 0"]
pub struct DATAVADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Comparator match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCHED_A {
    #[doc = "0: No match."]
    _0 = 0,
    #[doc = "1: Match occurred."]
    _1 = 1,
}
impl From<MATCHED_A> for bool {
    #[inline(always)]
    fn from(variant: MATCHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCHED` reader - Comparator match"]
pub struct MATCHED_R(crate::FieldReader<bool, MATCHED_A>);
impl MATCHED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MATCHED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCHED_A {
        match self.bits {
            false => MATCHED_A::_0,
            true => MATCHED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MATCHED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MATCHED_A::_1
    }
}
impl core::ops::Deref for MATCHED_R {
    type Target = crate::FieldReader<bool, MATCHED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    pub fn datavmatch(&self) -> DATAVMATCH_R {
        DATAVMATCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    pub fn datavaddr0(&self) -> DATAVADDR0_R {
        DATAVADDR0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Comparator match"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    pub fn datavmatch(&mut self) -> DATAVMATCH_W {
        DATAVMATCH_W { w: self }
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    pub fn datavsize(&mut self) -> DATAVSIZE_W {
        DATAVSIZE_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    pub fn datavaddr0(&mut self) -> DATAVADDR0_W {
        DATAVADDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB_DWT Comparator Function Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fct0](index.html) module"]
pub struct FCT0_SPEC;
impl crate::RegisterSpec for FCT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fct0::R](R) reader structure"]
impl crate::Readable for FCT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fct0::W](W) writer structure"]
impl crate::Writable for FCT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCT0 to value 0"]
impl crate::Resettable for FCT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
