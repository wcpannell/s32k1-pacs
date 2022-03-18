#[doc = "Register `PINCFG` reader"]
pub struct R(crate::R<PINCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINCFG` writer"]
pub struct W(crate::W<PINCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINCFG_SPEC>;
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
impl From<crate::W<PINCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Input trigger is disabled."]
    TRGSEL_0 = 0,
    #[doc = "1: Input trigger is used instead of RXD pin input."]
    TRGSEL_1 = 1,
    #[doc = "2: Input trigger is used instead of CTS_B pin input."]
    TRGSEL_2 = 2,
    #[doc = "3: Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    TRGSEL_3 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub struct TRGSEL_R(crate::FieldReader<u8, TRGSEL_A>);
impl TRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::TRGSEL_0,
            1 => TRGSEL_A::TRGSEL_1,
            2 => TRGSEL_A::TRGSEL_2,
            3 => TRGSEL_A::TRGSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL_0`"]
    #[inline(always)]
    pub fn is_trgsel_0(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `TRGSEL_1`"]
    #[inline(always)]
    pub fn is_trgsel_1(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_1
    }
    #[doc = "Checks if the value of the field is `TRGSEL_2`"]
    #[inline(always)]
    pub fn is_trgsel_2(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_2
    }
    #[doc = "Checks if the value of the field is `TRGSEL_3`"]
    #[inline(always)]
    pub fn is_trgsel_3(&self) -> bool {
        **self == TRGSEL_A::TRGSEL_3
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input trigger is disabled."]
    #[inline(always)]
    pub fn trgsel_0(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_0)
    }
    #[doc = "Input trigger is used instead of RXD pin input."]
    #[inline(always)]
    pub fn trgsel_1(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_1)
    }
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    #[inline(always)]
    pub fn trgsel_2(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_2)
    }
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    #[inline(always)]
    pub fn trgsel_3(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Pin Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg](index.html) module"]
pub struct PINCFG_SPEC;
impl crate::RegisterSpec for PINCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pincfg::R](R) reader structure"]
impl crate::Readable for PINCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pincfg::W](W) writer structure"]
impl crate::Writable for PINCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINCFG to value 0"]
impl crate::Resettable for PINCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
