#[doc = "Register `DER` reader"]
pub struct R(crate::R<DER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DER` writer"]
pub struct W(crate::W<DER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DER_SPEC>;
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
impl From<crate::W<DER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDDE_A {
    #[doc = "0: DMA request is disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request is enabled"]
    ENABLED = 1,
}
impl From<TDDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDDE` reader - Transmit Data DMA Enable"]
pub struct TDDE_R(crate::FieldReader<bool, TDDE_A>);
impl TDDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDDE_A {
        match self.bits {
            false => TDDE_A::DISABLED,
            true => TDDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDDE_A::ENABLED
    }
}
impl core::ops::Deref for TDDE_R {
    type Target = crate::FieldReader<bool, TDDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDDE` writer - Transmit Data DMA Enable"]
pub struct TDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDDE_A::DISABLED)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDDE_A::ENABLED)
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
#[doc = "Receive Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDDE_A {
    #[doc = "0: DMA request is disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request is enabled"]
    ENABLED = 1,
}
impl From<RDDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDDE` reader - Receive Data DMA Enable"]
pub struct RDDE_R(crate::FieldReader<bool, RDDE_A>);
impl RDDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDDE_A {
        match self.bits {
            false => RDDE_A::DISABLED,
            true => RDDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RDDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RDDE_A::ENABLED
    }
}
impl core::ops::Deref for RDDE_R {
    type Target = crate::FieldReader<bool, RDDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDDE` writer - Receive Data DMA Enable"]
pub struct RDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDDE_A::DISABLED)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    pub fn tdde(&self) -> TDDE_R {
        TDDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    pub fn rdde(&self) -> RDDE_R {
        RDDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    pub fn tdde(&mut self) -> TDDE_W {
        TDDE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    pub fn rdde(&mut self) -> RDDE_W {
        RDDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [der](index.html) module"]
pub struct DER_SPEC;
impl crate::RegisterSpec for DER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [der::R](R) reader structure"]
impl crate::Readable for DER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [der::W](W) writer structure"]
impl crate::Writable for DER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DER to value 0"]
impl crate::Resettable for DER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
