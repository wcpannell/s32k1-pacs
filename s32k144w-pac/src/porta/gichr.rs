#[doc = "Register `GICHR` reader"]
pub struct R(crate::R<GICHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICHR` writer"]
pub struct W(crate::W<GICHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICHR_SPEC>;
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
impl From<crate::W<GICHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global Interrupt Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GIWE_A {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    GIWE_0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    GIWE_1 = 1,
}
impl From<GIWE_A> for u16 {
    #[inline(always)]
    fn from(variant: GIWE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GIWE` reader - Global Interrupt Write Enable"]
pub struct GIWE_R(crate::FieldReader<u16, GIWE_A>);
impl GIWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GIWE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GIWE_A> {
        match self.bits {
            0 => Some(GIWE_A::GIWE_0),
            1 => Some(GIWE_A::GIWE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GIWE_0`"]
    #[inline(always)]
    pub fn is_giwe_0(&self) -> bool {
        **self == GIWE_A::GIWE_0
    }
    #[doc = "Checks if the value of the field is `GIWE_1`"]
    #[inline(always)]
    pub fn is_giwe_1(&self) -> bool {
        **self == GIWE_A::GIWE_1
    }
}
impl core::ops::Deref for GIWE_R {
    type Target = crate::FieldReader<u16, GIWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIWE` writer - Global Interrupt Write Enable"]
pub struct GIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIWE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn giwe_0(self) -> &'a mut W {
        self.variant(GIWE_A::GIWE_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn giwe_1(self) -> &'a mut W {
        self.variant(GIWE_A::GIWE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `GIWD` reader - Global Interrupt Write Data"]
pub struct GIWD_R(crate::FieldReader<u16, u16>);
impl GIWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GIWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIWD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIWD` writer - Global Interrupt Write Data"]
pub struct GIWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe(&self) -> GIWE_R {
        GIWE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&self) -> GIWD_R {
        GIWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe(&mut self) -> GIWE_W {
        GIWE_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&mut self) -> GIWD_W {
        GIWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Interrupt Control High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gichr](index.html) module"]
pub struct GICHR_SPEC;
impl crate::RegisterSpec for GICHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gichr::R](R) reader structure"]
impl crate::Readable for GICHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gichr::W](W) writer structure"]
impl crate::Writable for GICHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICHR to value 0"]
impl crate::Resettable for GICHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
