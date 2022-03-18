#[doc = "Register `FCT1` reader"]
pub struct R(crate::R<FCT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCT1` writer"]
pub struct W(crate::W<FCT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCT1_SPEC>;
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
impl From<crate::W<FCT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCT1_SPEC>) -> Self {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB_DWT Comparator Function Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fct1](index.html) module"]
pub struct FCT1_SPEC;
impl crate::RegisterSpec for FCT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fct1::R](R) reader structure"]
impl crate::Readable for FCT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fct1::W](W) writer structure"]
impl crate::Writable for FCT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCT1 to value 0"]
impl crate::Resettable for FCT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
