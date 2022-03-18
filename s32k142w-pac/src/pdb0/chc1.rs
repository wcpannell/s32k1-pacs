#[doc = "Register `CH%sC1` reader"]
pub struct R(crate::R<CHC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sC1` writer"]
pub struct W(crate::W<CHC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHC1_SPEC>;
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
impl From<crate::W<CHC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EN_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    EN_0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    EN_1 = 1,
}
impl From<EN_A> for u8 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EN` reader - PDB Channel Pre-Trigger Enable"]
pub struct EN_R(crate::FieldReader<u8, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EN_A> {
        match self.bits {
            0 => Some(EN_A::EN_0),
            1 => Some(EN_A::EN_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        **self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        **self == EN_A::EN_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<u8, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - PDB Channel Pre-Trigger Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    TOS_0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    TOS_1 = 1,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOS` reader - PDB Channel Pre-Trigger Output Select"]
pub struct TOS_R(crate::FieldReader<u8, TOS_A>);
impl TOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOS_A> {
        match self.bits {
            0 => Some(TOS_A::TOS_0),
            1 => Some(TOS_A::TOS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOS_0`"]
    #[inline(always)]
    pub fn is_tos_0(&self) -> bool {
        **self == TOS_A::TOS_0
    }
    #[doc = "Checks if the value of the field is `TOS_1`"]
    #[inline(always)]
    pub fn is_tos_1(&self) -> bool {
        **self == TOS_A::TOS_1
    }
}
impl core::ops::Deref for TOS_R {
    type Target = crate::FieldReader<u8, TOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOS` writer - PDB Channel Pre-Trigger Output Select"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn tos_0(self) -> &'a mut W {
        self.variant(TOS_A::TOS_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn tos_1(self) -> &'a mut W {
        self.variant(TOS_A::TOS_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    BB_0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    BB_1 = 1,
}
impl From<BB_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BB` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub struct BB_R(crate::FieldReader<u8, BB_A>);
impl BB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BB_A> {
        match self.bits {
            0 => Some(BB_A::BB_0),
            1 => Some(BB_A::BB_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BB_0`"]
    #[inline(always)]
    pub fn is_bb_0(&self) -> bool {
        **self == BB_A::BB_0
    }
    #[doc = "Checks if the value of the field is `BB_1`"]
    #[inline(always)]
    pub fn is_bb_1(&self) -> bool {
        **self == BB_A::BB_1
    }
}
impl core::ops::Deref for BB_R {
    type Target = crate::FieldReader<u8, BB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub struct BB_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn bb_0(self) -> &'a mut W {
        self.variant(BB_A::BB_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn bb_1(self) -> &'a mut W {
        self.variant(BB_A::BB_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W {
        BB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc1](index.html) module"]
pub struct CHC1_SPEC;
impl crate::RegisterSpec for CHC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chc1::R](R) reader structure"]
impl crate::Readable for CHC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chc1::W](W) writer structure"]
impl crate::Writable for CHC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%sC1 to value 0"]
impl crate::Resettable for CHC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
