#[doc = "Register `TFWR` reader"]
pub struct R(crate::R<TFWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFWR` writer"]
pub struct W(crate::W<TFWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFWR_SPEC>;
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
impl From<crate::W<TFWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit FIFO Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TFWR_A {
    #[doc = "0: 64 bytes written."]
    _000000 = 0,
    #[doc = "1: 64 bytes written."]
    _000001 = 1,
    #[doc = "2: 128 bytes written."]
    _000010 = 2,
    #[doc = "3: 192 bytes written."]
    _000011 = 3,
    #[doc = "31: 1984 bytes written."]
    _011111 = 31,
}
impl From<TFWR_A> for u8 {
    #[inline(always)]
    fn from(variant: TFWR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TFWR` reader - Transmit FIFO Write"]
pub struct TFWR_R(crate::FieldReader<u8, TFWR_A>);
impl TFWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TFWR_A> {
        match self.bits {
            0 => Some(TFWR_A::_000000),
            1 => Some(TFWR_A::_000001),
            2 => Some(TFWR_A::_000010),
            3 => Some(TFWR_A::_000011),
            31 => Some(TFWR_A::_011111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        **self == TFWR_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        **self == TFWR_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        **self == TFWR_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        **self == TFWR_A::_000011
    }
    #[doc = "Checks if the value of the field is `_011111`"]
    #[inline(always)]
    pub fn is_011111(&self) -> bool {
        **self == TFWR_A::_011111
    }
}
impl core::ops::Deref for TFWR_R {
    type Target = crate::FieldReader<u8, TFWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFWR` writer - Transmit FIFO Write"]
pub struct TFWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFWR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TFWR_A::_000000)
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(TFWR_A::_000001)
    }
    #[doc = "128 bytes written."]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(TFWR_A::_000010)
    }
    #[doc = "192 bytes written."]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut W {
        self.variant(TFWR_A::_000011)
    }
    #[doc = "1984 bytes written."]
    #[inline(always)]
    pub fn _011111(self) -> &'a mut W {
        self.variant(TFWR_A::_011111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Store And Forward Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRFWD_A {
    #[doc = "0: Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<STRFWD_A> for bool {
    #[inline(always)]
    fn from(variant: STRFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRFWD` reader - Store And Forward Enable"]
pub struct STRFWD_R(crate::FieldReader<bool, STRFWD_A>);
impl STRFWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STRFWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRFWD_A {
        match self.bits {
            false => STRFWD_A::_0,
            true => STRFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STRFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STRFWD_A::_1
    }
}
impl core::ops::Deref for STRFWD_R {
    type Target = crate::FieldReader<bool, STRFWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRFWD` writer - Store And Forward Enable"]
pub struct STRFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> STRFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRFWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STRFWD_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STRFWD_A::_1)
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
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&self) -> TFWR_R {
        TFWR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&self) -> STRFWD_R {
        STRFWD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&mut self) -> TFWR_W {
        TFWR_W { w: self }
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&mut self) -> STRFWD_W {
        STRFWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfwr](index.html) module"]
pub struct TFWR_SPEC;
impl crate::RegisterSpec for TFWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfwr::R](R) reader structure"]
impl crate::Readable for TFWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfwr::W](W) writer structure"]
impl crate::Writable for TFWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFWR to value 0"]
impl crate::Resettable for TFWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
