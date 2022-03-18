#[doc = "Register `STAR` reader"]
pub struct R(crate::R<STAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAR` writer"]
pub struct W(crate::W<STAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAR_SPEC>;
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
impl From<crate::W<STAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNACK_A {
    #[doc = "0: Transmit ACK for received word."]
    _0 = 0,
    #[doc = "1: Transmit NACK for received word."]
    _1 = 1,
}
impl From<TXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: TXNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXNACK` reader - Transmit NACK"]
pub struct TXNACK_R(crate::FieldReader<bool, TXNACK_A>);
impl TXNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXNACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNACK_A {
        match self.bits {
            false => TXNACK_A::_0,
            true => TXNACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXNACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXNACK_A::_1
    }
}
impl core::ops::Deref for TXNACK_R {
    type Target = crate::FieldReader<bool, TXNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXNACK` writer - Transmit NACK"]
pub struct TXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXNACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit ACK for received word."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXNACK_A::_0)
    }
    #[doc = "Transmit NACK for received word."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXNACK_A::_1)
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
impl R {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    pub fn txnack(&self) -> TXNACK_R {
        TXNACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    pub fn txnack(&mut self) -> TXNACK_W {
        TXNACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Transmit ACK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](index.html) module"]
pub struct STAR_SPEC;
impl crate::RegisterSpec for STAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [star::R](R) reader structure"]
impl crate::Readable for STAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [star::W](W) writer structure"]
impl crate::Writable for STAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAR to value 0"]
impl crate::Resettable for STAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
