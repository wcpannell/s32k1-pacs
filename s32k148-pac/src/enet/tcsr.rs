#[doc = "Register `TCSR%s` reader"]
pub struct R(crate::R<TCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCSR%s` writer"]
pub struct W(crate::W<TCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCSR_SPEC>;
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
impl From<crate::W<TCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: DMA request is disabled"]
    _0 = 0,
    #[doc = "1: DMA request is enabled"]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Timer DMA Request Enable"]
pub struct TDRE_R(crate::FieldReader<bool, TDRE_A>);
impl TDRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDRE_A::_1
    }
}
impl core::ops::Deref for TDRE_R {
    type Target = crate::FieldReader<bool, TDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRE` writer - Timer DMA Request Enable"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_A::_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_A::_1)
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
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: Timer Channel is disabled."]
    _0000 = 0,
    #[doc = "1: Timer Channel is configured for Input Capture on rising edge."]
    _0001 = 1,
    #[doc = "2: Timer Channel is configured for Input Capture on falling edge."]
    _0010 = 2,
    #[doc = "3: Timer Channel is configured for Input Capture on both edges."]
    _0011 = 3,
    #[doc = "4: Timer Channel is configured for Output Compare - software only."]
    _0100 = 4,
    #[doc = "5: Timer Channel is configured for Output Compare - toggle output on compare."]
    _0101 = 5,
    #[doc = "6: Timer Channel is configured for Output Compare - clear output on compare."]
    _0110 = 6,
    #[doc = "7: Timer Channel is configured for Output Compare - set output on compare."]
    _0111 = 7,
    #[doc = "10: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    _1010 = 10,
    #[doc = "14: Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    _1110 = 14,
    #[doc = "15: Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    _1111 = 15,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMODE` reader - Timer Mode"]
pub struct TMODE_R(crate::FieldReader<u8, TMODE_A>);
impl TMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMODE_A> {
        match self.bits {
            0 => Some(TMODE_A::_0000),
            1 => Some(TMODE_A::_0001),
            2 => Some(TMODE_A::_0010),
            3 => Some(TMODE_A::_0011),
            4 => Some(TMODE_A::_0100),
            5 => Some(TMODE_A::_0101),
            6 => Some(TMODE_A::_0110),
            7 => Some(TMODE_A::_0111),
            10 => Some(TMODE_A::_1010),
            14 => Some(TMODE_A::_1110),
            15 => Some(TMODE_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == TMODE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == TMODE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == TMODE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == TMODE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == TMODE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == TMODE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == TMODE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == TMODE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == TMODE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == TMODE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == TMODE_A::_1111
    }
}
impl core::ops::Deref for TMODE_R {
    type Target = crate::FieldReader<u8, TMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMODE` writer - Timer Mode"]
pub struct TMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Channel is disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TMODE_A::_0000)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TMODE_A::_0001)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TMODE_A::_0010)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TMODE_A::_0011)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TMODE_A::_0100)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TMODE_A::_0101)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TMODE_A::_0110)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TMODE_A::_0111)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TMODE_A::_1010)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TMODE_A::_1110)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TMODE_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub struct TIE_R(crate::FieldReader<bool, TIE_A>);
impl TIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIE_A::_1
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
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
#[doc = "Timer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_A {
    #[doc = "0: Input Capture or Output Compare has not occurred."]
    _0 = 0,
    #[doc = "1: Input Capture or Output Compare has occurred."]
    _1 = 1,
}
impl From<TF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF` reader - Timer Flag"]
pub struct TF_R(crate::FieldReader<bool, TF_A>);
impl TF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_A {
        match self.bits {
            false => TF_A::_0,
            true => TF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TF_A::_1
    }
}
impl core::ops::Deref for TF_R {
    type Target = crate::FieldReader<bool, TF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF` writer - Timer Flag"]
pub struct TF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF_A::_0)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF_A::_1)
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
impl R {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&mut self) -> TMODE_W {
        TMODE_W { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr](index.html) module"]
pub struct TCSR_SPEC;
impl crate::RegisterSpec for TCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcsr::R](R) reader structure"]
impl crate::Readable for TCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcsr::W](W) writer structure"]
impl crate::Writable for TCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCSR%s to value 0"]
impl crate::Resettable for TCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
