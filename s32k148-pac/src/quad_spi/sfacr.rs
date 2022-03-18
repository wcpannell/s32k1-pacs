#[doc = "Register `SFACR` reader"]
pub struct R(crate::R<SFACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFACR` writer"]
pub struct W(crate::W<SFACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFACR_SPEC>;
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
impl From<crate::W<SFACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAS` reader - Column Address Space"]
pub struct CAS_R(crate::FieldReader<u8, u8>);
impl CAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAS` writer - Column Address Space"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Word Addressable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WA_A {
    #[doc = "0: Byte addressable serial flash mode."]
    _0 = 0,
    #[doc = "1: Word (2 byte) addressable serial flash mode."]
    _1 = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Word Addressable"]
pub struct WA_R(crate::FieldReader<bool, WA_A>);
impl WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::_0,
            true => WA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WA_A::_1
    }
}
impl core::ops::Deref for WA_R {
    type Target = crate::FieldReader<bool, WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA` writer - Word Addressable"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Byte addressable serial flash mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WA_A::_0)
    }
    #[doc = "Word (2 byte) addressable serial flash mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Column Address Space"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Word Addressable"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Column Address Space"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 16 - Word Addressable"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Flash Address Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfacr](index.html) module"]
pub struct SFACR_SPEC;
impl crate::RegisterSpec for SFACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfacr::R](R) reader structure"]
impl crate::Readable for SFACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfacr::W](W) writer structure"]
impl crate::Writable for SFACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFACR to value 0"]
impl crate::Resettable for SFACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
