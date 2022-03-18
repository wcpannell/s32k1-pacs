#[doc = "Register `GPCLR` reader"]
pub struct R(crate::R<GPCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCLR` writer"]
pub struct W(crate::W<GPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCLR_SPEC>;
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
impl From<crate::W<GPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPWD` reader - Global Pin Write Data"]
pub struct GPWD_R(crate::FieldReader<u16, u16>);
impl GPWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPWD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPWD` writer - Global Pin Write Data"]
pub struct GPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPWE_A {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    GPWE_0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    GPWE_1 = 1,
}
impl From<GPWE_A> for u16 {
    #[inline(always)]
    fn from(variant: GPWE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPWE` reader - Global Pin Write Enable"]
pub struct GPWE_R(crate::FieldReader<u16, GPWE_A>);
impl GPWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GPWE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPWE_A> {
        match self.bits {
            0 => Some(GPWE_A::GPWE_0),
            1 => Some(GPWE_A::GPWE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPWE_0`"]
    #[inline(always)]
    pub fn is_gpwe_0(&self) -> bool {
        **self == GPWE_A::GPWE_0
    }
    #[doc = "Checks if the value of the field is `GPWE_1`"]
    #[inline(always)]
    pub fn is_gpwe_1(&self) -> bool {
        **self == GPWE_A::GPWE_1
    }
}
impl core::ops::Deref for GPWE_R {
    type Target = crate::FieldReader<u16, GPWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPWE` writer - Global Pin Write Enable"]
pub struct GPWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn gpwe_0(self) -> &'a mut W {
        self.variant(GPWE_A::GPWE_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn gpwe_1(self) -> &'a mut W {
        self.variant(GPWE_A::GPWE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    pub fn gpwd(&self) -> GPWD_R {
        GPWD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe(&self) -> GPWE_R {
        GPWE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    pub fn gpwd(&mut self) -> GPWD_W {
        GPWD_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe(&mut self) -> GPWE_W {
        GPWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Pin Control Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpclr](index.html) module"]
pub struct GPCLR_SPEC;
impl crate::RegisterSpec for GPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpclr::R](R) reader structure"]
impl crate::Readable for GPCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpclr::W](W) writer structure"]
impl crate::Writable for GPCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCLR to value 0"]
impl crate::Resettable for GPCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
