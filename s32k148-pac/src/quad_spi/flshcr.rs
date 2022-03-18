#[doc = "Register `FLSHCR` reader"]
pub struct R(crate::R<FLSHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSHCR` writer"]
pub struct W(crate::W<FLSHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSHCR_SPEC>;
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
impl From<crate::W<FLSHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCSS` reader - Serial flash CS setup time in terms of serial flash clock cycles"]
pub struct TCSS_R(crate::FieldReader<u8, u8>);
impl TCSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCSS` writer - Serial flash CS setup time in terms of serial flash clock cycles"]
pub struct TCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TCSH` reader - Serial flash CS hold time in terms of serial flash clock cycles"]
pub struct TCSH_R(crate::FieldReader<u8, u8>);
impl TCSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCSH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCSH` writer - Serial flash CS hold time in terms of serial flash clock cycles"]
pub struct TCSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Serial flash data in hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TDH_A {
    #[doc = "0: Data aligned with the posedge of Internal reference clock of QuadSPI"]
    _00 = 0,
    #[doc = "1: Data aligned with 2x serial flash half clock"]
    _01 = 1,
}
impl From<TDH_A> for u8 {
    #[inline(always)]
    fn from(variant: TDH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TDH` reader - Serial flash data in hold time"]
pub struct TDH_R(crate::FieldReader<u8, TDH_A>);
impl TDH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TDH_A> {
        match self.bits {
            0 => Some(TDH_A::_00),
            1 => Some(TDH_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TDH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TDH_A::_01
    }
}
impl core::ops::Deref for TDH_R {
    type Target = crate::FieldReader<u8, TDH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDH` writer - Serial flash data in hold time"]
pub struct TDH_W<'a> {
    w: &'a mut W,
}
impl<'a> TDH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data aligned with the posedge of Internal reference clock of QuadSPI"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TDH_A::_00)
    }
    #[doc = "Data aligned with 2x serial flash half clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TDH_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcsh(&self) -> TCSH_R {
        TCSH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline(always)]
    pub fn tdh(&self) -> TDH_R {
        TDH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcss(&mut self) -> TCSS_W {
        TCSS_W { w: self }
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcsh(&mut self) -> TCSH_W {
        TCSH_W { w: self }
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline(always)]
    pub fn tdh(&mut self) -> TDH_W {
        TDH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr](index.html) module"]
pub struct FLSHCR_SPEC;
impl crate::RegisterSpec for FLSHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flshcr::R](R) reader structure"]
impl crate::Readable for FLSHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flshcr::W](W) writer structure"]
impl crate::Writable for FLSHCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLSHCR to value 0x0303"]
impl crate::Resettable for FLSHCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0303
    }
}
