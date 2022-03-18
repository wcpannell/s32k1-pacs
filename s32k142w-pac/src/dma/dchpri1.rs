#[doc = "Register `DCHPRI1` reader"]
pub struct R(crate::R<DCHPRI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCHPRI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCHPRI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCHPRI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCHPRI1` writer"]
pub struct W(crate::W<DCHPRI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCHPRI1_SPEC>;
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
impl From<crate::W<DCHPRI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCHPRI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPRI` reader - Channel n Arbitration Priority"]
pub struct CHPRI_R(crate::FieldReader<u8, u8>);
impl CHPRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHPRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPRI` writer - Channel n Arbitration Priority"]
pub struct CHPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Disable Preempt Ability. This field resets to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPA_A {
    #[doc = "0: Channel n can suspend a lower priority channel."]
    DPA_0 = 0,
    #[doc = "1: Channel n cannot suspend any channel, regardless of channel priority."]
    DPA_1 = 1,
}
impl From<DPA_A> for bool {
    #[inline(always)]
    fn from(variant: DPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPA` reader - Disable Preempt Ability. This field resets to 0."]
pub struct DPA_R(crate::FieldReader<bool, DPA_A>);
impl DPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPA_A {
        match self.bits {
            false => DPA_A::DPA_0,
            true => DPA_A::DPA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPA_0`"]
    #[inline(always)]
    pub fn is_dpa_0(&self) -> bool {
        **self == DPA_A::DPA_0
    }
    #[doc = "Checks if the value of the field is `DPA_1`"]
    #[inline(always)]
    pub fn is_dpa_1(&self) -> bool {
        **self == DPA_A::DPA_1
    }
}
impl core::ops::Deref for DPA_R {
    type Target = crate::FieldReader<bool, DPA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPA` writer - Disable Preempt Ability. This field resets to 0."]
pub struct DPA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel n can suspend a lower priority channel."]
    #[inline(always)]
    pub fn dpa_0(self) -> &'a mut W {
        self.variant(DPA_A::DPA_0)
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    #[inline(always)]
    pub fn dpa_1(self) -> &'a mut W {
        self.variant(DPA_A::DPA_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable Channel Preemption. This field resets to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECP_A {
    #[doc = "0: Channel n cannot be suspended by a higher priority channel's service request."]
    ECP_0 = 0,
    #[doc = "1: Channel n can be temporarily suspended by the service request of a higher priority channel."]
    ECP_1 = 1,
}
impl From<ECP_A> for bool {
    #[inline(always)]
    fn from(variant: ECP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECP` reader - Enable Channel Preemption. This field resets to 0."]
pub struct ECP_R(crate::FieldReader<bool, ECP_A>);
impl ECP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECP_A {
        match self.bits {
            false => ECP_A::ECP_0,
            true => ECP_A::ECP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECP_0`"]
    #[inline(always)]
    pub fn is_ecp_0(&self) -> bool {
        **self == ECP_A::ECP_0
    }
    #[doc = "Checks if the value of the field is `ECP_1`"]
    #[inline(always)]
    pub fn is_ecp_1(&self) -> bool {
        **self == ECP_A::ECP_1
    }
}
impl core::ops::Deref for ECP_R {
    type Target = crate::FieldReader<bool, ECP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECP` writer - Enable Channel Preemption. This field resets to 0."]
pub struct ECP_W<'a> {
    w: &'a mut W,
}
impl<'a> ECP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
    #[inline(always)]
    pub fn ecp_0(self) -> &'a mut W {
        self.variant(ECP_A::ECP_0)
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
    #[inline(always)]
    pub fn ecp_1(self) -> &'a mut W {
        self.variant(ECP_A::ECP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub fn dpa(&self) -> DPA_R {
        DPA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub fn ecp(&self) -> ECP_R {
        ECP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&mut self) -> CHPRI_W {
        CHPRI_W { w: self }
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub fn dpa(&mut self) -> DPA_W {
        DPA_W { w: self }
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub fn ecp(&mut self) -> ECP_W {
        ECP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri1](index.html) module"]
pub struct DCHPRI1_SPEC;
impl crate::RegisterSpec for DCHPRI1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dchpri1::R](R) reader structure"]
impl crate::Readable for DCHPRI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dchpri1::W](W) writer structure"]
impl crate::Writable for DCHPRI1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCHPRI1 to value 0x01"]
impl crate::Resettable for DCHPRI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
