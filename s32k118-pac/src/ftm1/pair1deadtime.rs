#[doc = "Register `PAIR1DEADTIME` reader"]
pub struct R(crate::R<PAIR1DEADTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIR1DEADTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIR1DEADTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIR1DEADTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAIR1DEADTIME` writer"]
pub struct W(crate::W<PAIR1DEADTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAIR1DEADTIME_SPEC>;
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
impl From<crate::W<PAIR1DEADTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAIR1DEADTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTVAL` reader - Deadtime Value"]
pub struct DTVAL_R(crate::FieldReader<u8, u8>);
impl DTVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTVAL` writer - Deadtime Value"]
pub struct DTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Deadtime Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPS_A {
    #[doc = "0: Divide the FTM input clock by 1."]
    _0X = 0,
    #[doc = "2: Divide the FTM input clock by 4."]
    _10 = 2,
    #[doc = "3: Divide the FTM input clock by 16."]
    _11 = 3,
}
impl From<DTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPS` reader - Deadtime Prescaler Value"]
pub struct DTPS_R(crate::FieldReader<u8, DTPS_A>);
impl DTPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPS_A> {
        match self.bits {
            0 => Some(DTPS_A::_0X),
            2 => Some(DTPS_A::_10),
            3 => Some(DTPS_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X`"]
    #[inline(always)]
    pub fn is_0x(&self) -> bool {
        **self == DTPS_A::_0X
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == DTPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DTPS_A::_11
    }
}
impl core::ops::Deref for DTPS_R {
    type Target = crate::FieldReader<u8, DTPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPS` writer - Deadtime Prescaler Value"]
pub struct DTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide the FTM input clock by 1."]
    #[inline(always)]
    pub fn _0x(self) -> &'a mut W {
        self.variant(DTPS_A::_0X)
    }
    #[doc = "Divide the FTM input clock by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTPS_A::_10)
    }
    #[doc = "Divide the FTM input clock by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DTPS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DTVALEX` reader - Extended Deadtime Value"]
pub struct DTVALEX_R(crate::FieldReader<u8, u8>);
impl DTVALEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTVALEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTVALEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTVALEX` writer - Extended Deadtime Value"]
pub struct DTVALEX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTVALEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&self) -> DTVAL_R {
        DTVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&self) -> DTPS_R {
        DTPS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline(always)]
    pub fn dtvalex(&self) -> DTVALEX_R {
        DTVALEX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&mut self) -> DTVAL_W {
        DTVAL_W { w: self }
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&mut self) -> DTPS_W {
        DTPS_W { w: self }
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline(always)]
    pub fn dtvalex(&mut self) -> DTVALEX_W {
        DTVALEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pair 1 Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pair1deadtime](index.html) module"]
pub struct PAIR1DEADTIME_SPEC;
impl crate::RegisterSpec for PAIR1DEADTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pair1deadtime::R](R) reader structure"]
impl crate::Readable for PAIR1DEADTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pair1deadtime::W](W) writer structure"]
impl crate::Writable for PAIR1DEADTIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAIR1DEADTIME to value 0"]
impl crate::Resettable for PAIR1DEADTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
