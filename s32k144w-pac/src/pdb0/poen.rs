#[doc = "Register `POEN` reader"]
pub struct R(crate::R<POEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN` writer"]
pub struct W(crate::W<POEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN_SPEC>;
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
impl From<crate::W<POEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POEN_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    POEN_0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    POEN_1 = 1,
}
impl From<POEN_A> for u8 {
    #[inline(always)]
    fn from(variant: POEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POEN` reader - PDB Pulse-Out Enable"]
pub struct POEN_R(crate::FieldReader<u8, POEN_A>);
impl POEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POEN_A> {
        match self.bits {
            0 => Some(POEN_A::POEN_0),
            1 => Some(POEN_A::POEN_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POEN_0`"]
    #[inline(always)]
    pub fn is_poen_0(&self) -> bool {
        **self == POEN_A::POEN_0
    }
    #[doc = "Checks if the value of the field is `POEN_1`"]
    #[inline(always)]
    pub fn is_poen_1(&self) -> bool {
        **self == POEN_A::POEN_1
    }
}
impl core::ops::Deref for POEN_R {
    type Target = crate::FieldReader<u8, POEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POEN` writer - PDB Pulse-Out Enable"]
pub struct POEN_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn poen_0(self) -> &'a mut W {
        self.variant(POEN_A::POEN_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn poen_1(self) -> &'a mut W {
        self.variant(POEN_A::POEN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&mut self) -> POEN_W {
        POEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse-Out n Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen](index.html) module"]
pub struct POEN_SPEC;
impl crate::RegisterSpec for POEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen::R](R) reader structure"]
impl crate::Readable for POEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen::W](W) writer structure"]
impl crate::Writable for POEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POEN to value 0"]
impl crate::Resettable for POEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
