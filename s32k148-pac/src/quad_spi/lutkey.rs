#[doc = "Register `LUTKEY` reader"]
pub struct R(crate::R<LUTKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUTKEY` writer"]
pub struct W(crate::W<LUTKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTKEY_SPEC>;
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
impl From<crate::W<LUTKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - The key to lock or unlock the LUT. The KEY is 0x5AF05AF0. The read value is always 0x5AF05AF0"]
pub struct KEY_R(crate::FieldReader<u32, u32>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - The key to lock or unlock the LUT. The KEY is 0x5AF05AF0. The read value is always 0x5AF05AF0"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The key to lock or unlock the LUT. The KEY is 0x5AF05AF0. The read value is always 0x5AF05AF0"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The key to lock or unlock the LUT. The KEY is 0x5AF05AF0. The read value is always 0x5AF05AF0"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutkey](index.html) module"]
pub struct LUTKEY_SPEC;
impl crate::RegisterSpec for LUTKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lutkey::R](R) reader structure"]
impl crate::Readable for LUTKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lutkey::W](W) writer structure"]
impl crate::Writable for LUTKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUTKEY to value 0x5af0_5af0"]
impl crate::Resettable for LUTKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5af0_5af0
    }
}
