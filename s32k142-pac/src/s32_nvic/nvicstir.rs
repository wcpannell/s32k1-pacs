#[doc = "Register `NVICSTIR` writer"]
pub struct W(crate::W<NVICSTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICSTIR_SPEC>;
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
impl From<crate::W<NVICSTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICSTIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTID` writer - Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
pub struct INTID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:8 - Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    #[inline(always)]
    pub fn intid(&mut self) -> INTID_W {
        INTID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Interrupt Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicstir](index.html) module"]
pub struct NVICSTIR_SPEC;
impl crate::RegisterSpec for NVICSTIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nvicstir::W](W) writer structure"]
impl crate::Writable for NVICSTIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICSTIR to value 0"]
impl crate::Resettable for NVICSTIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
