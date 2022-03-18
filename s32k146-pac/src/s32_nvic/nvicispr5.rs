#[doc = "Register `NVICISPR5` reader"]
pub struct R(crate::R<NVICISPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICISPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICISPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICISPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICISPR5` writer"]
pub struct W(crate::W<NVICISPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICISPR5_SPEC>;
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
impl From<crate::W<NVICISPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICISPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits"]
pub struct SETPEND_R(crate::FieldReader<u32, u32>);
impl SETPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SETPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETPEND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits"]
pub struct SETPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits"]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits"]
    #[inline(always)]
    pub fn setpend(&mut self) -> SETPEND_W {
        SETPEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set Pending Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicispr5](index.html) module"]
pub struct NVICISPR5_SPEC;
impl crate::RegisterSpec for NVICISPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvicispr5::R](R) reader structure"]
impl crate::Readable for NVICISPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicispr5::W](W) writer structure"]
impl crate::Writable for NVICISPR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICISPR5 to value 0"]
impl crate::Resettable for NVICISPR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
