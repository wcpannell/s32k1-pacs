#[doc = "Register `MCFGR3` reader"]
pub struct R(crate::R<MCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR3` writer"]
pub struct W(crate::W<MCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR3_SPEC>;
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
impl From<crate::W<MCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOW` reader - Pin Low Timeout"]
pub struct PINLOW_R(crate::FieldReader<u16, u16>);
impl PINLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PINLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINLOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINLOW` writer - Pin Low Timeout"]
pub struct PINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> PINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    pub fn pinlow(&self) -> PINLOW_R {
        PINLOW_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    pub fn pinlow(&mut self) -> PINLOW_W {
        PINLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr3](index.html) module"]
pub struct MCFGR3_SPEC;
impl crate::RegisterSpec for MCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr3::R](R) reader structure"]
impl crate::Readable for MCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr3::W](W) writer structure"]
impl crate::Writable for MCFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCFGR3 to value 0"]
impl crate::Resettable for MCFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
