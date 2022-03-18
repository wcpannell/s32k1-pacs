#[doc = "Register `FDPROT` reader"]
pub struct R(crate::R<FDPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDPROT` writer"]
pub struct W(crate::W<FDPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDPROT_SPEC>;
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
impl From<crate::W<FDPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DPROT_A {
    #[doc = "0: Data Flash region is protected"]
    _00000000 = 0,
    #[doc = "1: Data Flash region is not protected"]
    _00000001 = 1,
}
impl From<DPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: DPROT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DPROT` reader - Data Flash Region Protect"]
pub struct DPROT_R(crate::FieldReader<u8, DPROT_A>);
impl DPROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DPROT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DPROT_A> {
        match self.bits {
            0 => Some(DPROT_A::_00000000),
            1 => Some(DPROT_A::_00000001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline(always)]
    pub fn is_00000000(&self) -> bool {
        **self == DPROT_A::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline(always)]
    pub fn is_00000001(&self) -> bool {
        **self == DPROT_A::_00000001
    }
}
impl core::ops::Deref for DPROT_R {
    type Target = crate::FieldReader<u8, DPROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPROT` writer - Data Flash Region Protect"]
pub struct DPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data Flash region is protected"]
    #[inline(always)]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(DPROT_A::_00000000)
    }
    #[doc = "Data Flash region is not protected"]
    #[inline(always)]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(DPROT_A::_00000001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&mut self) -> DPROT_W {
        DPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdprot](index.html) module"]
pub struct FDPROT_SPEC;
impl crate::RegisterSpec for FDPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fdprot::R](R) reader structure"]
impl crate::Readable for FDPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdprot::W](W) writer structure"]
impl crate::Writable for FDPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDPROT to value 0"]
impl crate::Resettable for FDPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
