#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Compensation Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCR_A {
    #[doc = "128: Time Prescaler Register overflows every 32896 clock cycles."]
    _10000000 = 128,
    #[doc = "129: Time Prescaler Register overflows every 32895 clock cycles."]
    _10000001 = 129,
    #[doc = "255: Time Prescaler Register overflows every 32769 clock cycles."]
    _11111111 = 255,
    #[doc = "0: Time Prescaler Register overflows every 32768 clock cycles."]
    _00000000 = 0,
    #[doc = "1: Time Prescaler Register overflows every 32767 clock cycles."]
    _00000001 = 1,
    #[doc = "126: Time Prescaler Register overflows every 32642 clock cycles."]
    _01111110 = 126,
    #[doc = "127: Time Prescaler Register overflows every 32641 clock cycles."]
    _01111111 = 127,
}
impl From<TCR_A> for u8 {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCR` reader - Time Compensation Register"]
pub struct TCR_R(crate::FieldReader<u8, TCR_A>);
impl TCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCR_A> {
        match self.bits {
            128 => Some(TCR_A::_10000000),
            129 => Some(TCR_A::_10000001),
            255 => Some(TCR_A::_11111111),
            0 => Some(TCR_A::_00000000),
            1 => Some(TCR_A::_00000001),
            126 => Some(TCR_A::_01111110),
            127 => Some(TCR_A::_01111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline(always)]
    pub fn is_10000000(&self) -> bool {
        **self == TCR_A::_10000000
    }
    #[doc = "Checks if the value of the field is `_10000001`"]
    #[inline(always)]
    pub fn is_10000001(&self) -> bool {
        **self == TCR_A::_10000001
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline(always)]
    pub fn is_11111111(&self) -> bool {
        **self == TCR_A::_11111111
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline(always)]
    pub fn is_00000000(&self) -> bool {
        **self == TCR_A::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline(always)]
    pub fn is_00000001(&self) -> bool {
        **self == TCR_A::_00000001
    }
    #[doc = "Checks if the value of the field is `_01111110`"]
    #[inline(always)]
    pub fn is_01111110(&self) -> bool {
        **self == TCR_A::_01111110
    }
    #[doc = "Checks if the value of the field is `_01111111`"]
    #[inline(always)]
    pub fn is_01111111(&self) -> bool {
        **self == TCR_A::_01111111
    }
}
impl core::ops::Deref for TCR_R {
    type Target = crate::FieldReader<u8, TCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCR` writer - Time Compensation Register"]
pub struct TCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    #[inline(always)]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(TCR_A::_10000000)
    }
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    #[inline(always)]
    pub fn _10000001(self) -> &'a mut W {
        self.variant(TCR_A::_10000001)
    }
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    #[inline(always)]
    pub fn _11111111(self) -> &'a mut W {
        self.variant(TCR_A::_11111111)
    }
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    #[inline(always)]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(TCR_A::_00000000)
    }
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    #[inline(always)]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(TCR_A::_00000001)
    }
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    #[inline(always)]
    pub fn _01111110(self) -> &'a mut W {
        self.variant(TCR_A::_01111110)
    }
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    #[inline(always)]
    pub fn _01111111(self) -> &'a mut W {
        self.variant(TCR_A::_01111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CIR` reader - Compensation Interval Register"]
pub struct CIR_R(crate::FieldReader<u8, u8>);
impl CIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIR` writer - Compensation Interval Register"]
pub struct CIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TCV` reader - Time Compensation Value"]
pub struct TCV_R(crate::FieldReader<u8, u8>);
impl TCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIC` reader - Compensation Interval Counter"]
pub struct CIC_R(crate::FieldReader<u8, u8>);
impl CIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    pub fn cir(&self) -> CIR_R {
        CIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Time Compensation Value"]
    #[inline(always)]
    pub fn tcv(&self) -> TCV_R {
        TCV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compensation Interval Counter"]
    #[inline(always)]
    pub fn cic(&self) -> CIC_R {
        CIC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W {
        TCR_W { w: self }
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    pub fn cir(&mut self) -> CIR_W {
        CIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
