#[doc = "Register `TCD8_ATTR` reader"]
pub struct R(crate::R<TCD8_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD8_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD8_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD8_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD8_ATTR` writer"]
pub struct W(crate::W<TCD8_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD8_ATTR_SPEC>;
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
impl From<crate::W<TCD8_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD8_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - Destination data transfer size"]
pub struct DSIZE_R(crate::FieldReader<u8, u8>);
impl DSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIZE` writer - Destination data transfer size"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `DMOD` reader - Destination Address Modulo"]
pub struct DMOD_R(crate::FieldReader<u8, u8>);
impl DMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMOD` writer - Destination Address Modulo"]
pub struct DMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u16 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 8-bit"]
    _0 = 0,
    #[doc = "1: 16-bit"]
    _1 = 1,
    #[doc = "2: 32-bit"]
    _10 = 2,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSIZE` reader - Source data transfer size"]
pub struct SSIZE_R(crate::FieldReader<u8, SSIZE_A>);
impl SSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSIZE_A> {
        match self.bits {
            0 => Some(SSIZE_A::_0),
            1 => Some(SSIZE_A::_1),
            2 => Some(SSIZE_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSIZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SSIZE_A::_10
    }
}
impl core::ops::Deref for SSIZE_R {
    type Target = crate::FieldReader<u8, SSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIZE` writer - Source data transfer size"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIZE_A::_0)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIZE_A::_1)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSIZE_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
#[doc = "Source Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Source address modulo feature is disabled"]
    _0 = 0,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD` reader - Source Address Modulo"]
pub struct SMOD_R(crate::FieldReader<u8, SMOD_A>);
impl SMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMOD_A::_0
    }
}
impl core::ops::Deref for SMOD_R {
    type Target = crate::FieldReader<u8, SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD` writer - Source Address Modulo"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMOD_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u16 & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&mut self) -> DMOD_W {
        DMOD_W { w: self }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_attr](index.html) module"]
pub struct TCD8_ATTR_SPEC;
impl crate::RegisterSpec for TCD8_ATTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd8_attr::R](R) reader structure"]
impl crate::Readable for TCD8_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd8_attr::W](W) writer structure"]
impl crate::Writable for TCD8_ATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD8_ATTR to value 0"]
impl crate::Resettable for TCD8_ATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
