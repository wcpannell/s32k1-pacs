#[doc = "Register `PDB1` reader"]
pub struct R(crate::R<PDB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDB1` writer"]
pub struct W(crate::W<PDB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDB1_SPEC>;
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
impl From<crate::W<PDB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - Trigger MUX Input 0 Source Select"]
pub struct SEL0_R(crate::FieldReader<u8, u8>);
impl SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL0` writer - Trigger MUX Input 0 Source Select"]
pub struct SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "TRGMUX register lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: Register can be written."]
    UNLOCKED = 0,
    #[doc = "1: Register cannot be written until the next system Reset."]
    LOCKED = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - TRGMUX register lock."]
pub struct LK_R(crate::FieldReader<bool, LK_A>);
impl LK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::UNLOCKED,
            true => LK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LK_A::LOCKED
    }
}
impl core::ops::Deref for LK_R {
    type Target = crate::FieldReader<bool, LK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LK` writer - TRGMUX register lock."]
pub struct LK_W<'a> {
    w: &'a mut W,
}
impl<'a> LK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register can be written."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LK_A::UNLOCKED)
    }
    #[doc = "Register cannot be written until the next system Reset."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - TRGMUX register lock."]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
    #[inline(always)]
    pub fn sel0(&mut self) -> SEL0_W {
        SEL0_W { w: self }
    }
    #[doc = "Bit 31 - TRGMUX register lock."]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRGMUX PDB1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdb1](index.html) module"]
pub struct PDB1_SPEC;
impl crate::RegisterSpec for PDB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdb1::R](R) reader structure"]
impl crate::Readable for PDB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdb1::W](W) writer structure"]
impl crate::Writable for PDB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDB1 to value 0"]
impl crate::Resettable for PDB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
