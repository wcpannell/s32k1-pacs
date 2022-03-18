#[doc = "Register `TCD1_CITER_ELINKNO` reader"]
pub struct R(crate::R<CITER_TCD1_CITER_ELINKNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CITER_TCD1_CITER_ELINKNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CITER_TCD1_CITER_ELINKNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CITER_TCD1_CITER_ELINKNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD1_CITER_ELINKNO` writer"]
pub struct W(crate::W<CITER_TCD1_CITER_ELINKNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CITER_TCD1_CITER_ELINKNO_SPEC>;
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
impl From<crate::W<CITER_TCD1_CITER_ELINKNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CITER_TCD1_CITER_ELINKNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CITER` reader - Current Major Iteration Count"]
pub struct CITER_R(crate::FieldReader<u16, u16>);
impl CITER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CITER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITER` writer - Current Major Iteration Count"]
pub struct CITER_W<'a> {
    w: &'a mut W,
}
impl<'a> CITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u16 & 0x7fff);
        self.w
    }
}
#[doc = "Enable channel-to-channel linking on minor-loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled"]
    ELINK_0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    ELINK_1 = 1,
}
impl From<ELINK_A> for bool {
    #[inline(always)]
    fn from(variant: ELINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELINK` reader - Enable channel-to-channel linking on minor-loop complete"]
pub struct ELINK_R(crate::FieldReader<bool, ELINK_A>);
impl ELINK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELINK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELINK_A {
        match self.bits {
            false => ELINK_A::ELINK_0,
            true => ELINK_A::ELINK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ELINK_0`"]
    #[inline(always)]
    pub fn is_elink_0(&self) -> bool {
        **self == ELINK_A::ELINK_0
    }
    #[doc = "Checks if the value of the field is `ELINK_1`"]
    #[inline(always)]
    pub fn is_elink_1(&self) -> bool {
        **self == ELINK_A::ELINK_1
    }
}
impl core::ops::Deref for ELINK_R {
    type Target = crate::FieldReader<bool, ELINK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELINK` writer - Enable channel-to-channel linking on minor-loop complete"]
pub struct ELINK_W<'a> {
    w: &'a mut W,
}
impl<'a> ELINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ELINK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn elink_0(self) -> &'a mut W {
        self.variant(ELINK_A::ELINK_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn elink_1(self) -> &'a mut W {
        self.variant(ELINK_A::ELINK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&self) -> CITER_R {
        CITER_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ELINK_R {
        ELINK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&mut self) -> CITER_W {
        CITER_W { w: self }
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&mut self) -> ELINK_W {
        ELINK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [citer_tcd1_citer_elinkno](index.html) module"]
pub struct CITER_TCD1_CITER_ELINKNO_SPEC;
impl crate::RegisterSpec for CITER_TCD1_CITER_ELINKNO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [citer_tcd1_citer_elinkno::R](R) reader structure"]
impl crate::Readable for CITER_TCD1_CITER_ELINKNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [citer_tcd1_citer_elinkno::W](W) writer structure"]
impl crate::Writable for CITER_TCD1_CITER_ELINKNO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD1_CITER_ELINKNO to value 0"]
impl crate::Resettable for CITER_TCD1_CITER_ELINKNO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
