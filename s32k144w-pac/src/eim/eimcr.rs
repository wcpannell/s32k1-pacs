#[doc = "Register `EIMCR` reader"]
pub struct R(crate::R<EIMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMCR` writer"]
pub struct W(crate::W<EIMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMCR_SPEC>;
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
impl From<crate::W<EIMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global Error Injection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEIEN_A {
    #[doc = "0: Disabled"]
    GEIEN_0 = 0,
    #[doc = "1: Enabled"]
    GEIEN_1 = 1,
}
impl From<GEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEIEN` reader - Global Error Injection Enable"]
pub struct GEIEN_R(crate::FieldReader<bool, GEIEN_A>);
impl GEIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEIEN_A {
        match self.bits {
            false => GEIEN_A::GEIEN_0,
            true => GEIEN_A::GEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GEIEN_0`"]
    #[inline(always)]
    pub fn is_geien_0(&self) -> bool {
        **self == GEIEN_A::GEIEN_0
    }
    #[doc = "Checks if the value of the field is `GEIEN_1`"]
    #[inline(always)]
    pub fn is_geien_1(&self) -> bool {
        **self == GEIEN_A::GEIEN_1
    }
}
impl core::ops::Deref for GEIEN_R {
    type Target = crate::FieldReader<bool, GEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEIEN` writer - Global Error Injection Enable"]
pub struct GEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn geien_0(self) -> &'a mut W {
        self.variant(GEIEN_A::GEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn geien_1(self) -> &'a mut W {
        self.variant(GEIEN_A::GEIEN_1)
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
    #[doc = "Bit 0 - Global Error Injection Enable"]
    #[inline(always)]
    pub fn geien(&self) -> GEIEN_R {
        GEIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Error Injection Enable"]
    #[inline(always)]
    pub fn geien(&mut self) -> GEIEN_W {
        GEIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Injection Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimcr](index.html) module"]
pub struct EIMCR_SPEC;
impl crate::RegisterSpec for EIMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eimcr::R](R) reader structure"]
impl crate::Readable for EIMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimcr::W](W) writer structure"]
impl crate::Writable for EIMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EIMCR to value 0"]
impl crate::Resettable for EIMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
