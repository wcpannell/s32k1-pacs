#[doc = "Register `PMPROT` reader"]
pub struct R(crate::R<PMPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMPROT` writer"]
pub struct W(crate::W<PMPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMPROT_SPEC>;
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
impl From<crate::W<PMPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLP_A {
    #[doc = "0: VLPR and VLPS are not allowed."]
    AVLP_0 = 0,
    #[doc = "1: VLPR and VLPS are allowed."]
    AVLP_1 = 1,
}
impl From<AVLP_A> for bool {
    #[inline(always)]
    fn from(variant: AVLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLP` reader - Allow Very-Low-Power Modes"]
pub struct AVLP_R(crate::FieldReader<bool, AVLP_A>);
impl AVLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLP_A {
        match self.bits {
            false => AVLP_A::AVLP_0,
            true => AVLP_A::AVLP_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVLP_0`"]
    #[inline(always)]
    pub fn is_avlp_0(&self) -> bool {
        **self == AVLP_A::AVLP_0
    }
    #[doc = "Checks if the value of the field is `AVLP_1`"]
    #[inline(always)]
    pub fn is_avlp_1(&self) -> bool {
        **self == AVLP_A::AVLP_1
    }
}
impl core::ops::Deref for AVLP_R {
    type Target = crate::FieldReader<bool, AVLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVLP` writer - Allow Very-Low-Power Modes"]
pub struct AVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VLPR and VLPS are not allowed."]
    #[inline(always)]
    pub fn avlp_0(self) -> &'a mut W {
        self.variant(AVLP_A::AVLP_0)
    }
    #[doc = "VLPR and VLPS are allowed."]
    #[inline(always)]
    pub fn avlp_1(self) -> &'a mut W {
        self.variant(AVLP_A::AVLP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AVLP_R {
        AVLP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&mut self) -> AVLP_W {
        AVLP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmprot](index.html) module"]
pub struct PMPROT_SPEC;
impl crate::RegisterSpec for PMPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmprot::R](R) reader structure"]
impl crate::Readable for PMPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmprot::W](W) writer structure"]
impl crate::Writable for PMPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMPROT to value 0"]
impl crate::Resettable for PMPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
