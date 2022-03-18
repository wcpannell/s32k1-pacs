#[doc = "Register `MOD_MIRROR` reader"]
pub struct R(crate::R<MOD_MIRROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_MIRROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_MIRROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_MIRROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD_MIRROR` writer"]
pub struct W(crate::W<MOD_MIRROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_MIRROR_SPEC>;
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
impl From<crate::W<MOD_MIRROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_MIRROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACMOD` reader - Modulo Fractional Value"]
pub struct FRACMOD_R(crate::FieldReader<u8, u8>);
impl FRACMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRACMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACMOD` writer - Modulo Fractional Value"]
pub struct FRACMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `MOD` reader - Mirror of the Modulo Integer Value"]
pub struct MOD_R(crate::FieldReader<u16, u16>);
impl MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD` writer - Mirror of the Modulo Integer Value"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Modulo Fractional Value"]
    #[inline(always)]
    pub fn fracmod(&self) -> FRACMOD_R {
        FRACMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Mirror of the Modulo Integer Value"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:15 - Modulo Fractional Value"]
    #[inline(always)]
    pub fn fracmod(&mut self) -> FRACMOD_W {
        FRACMOD_W { w: self }
    }
    #[doc = "Bits 16:31 - Mirror of the Modulo Integer Value"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mirror of Modulo Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_mirror](index.html) module"]
pub struct MOD_MIRROR_SPEC;
impl crate::RegisterSpec for MOD_MIRROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_mirror::R](R) reader structure"]
impl crate::Readable for MOD_MIRROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_mirror::W](W) writer structure"]
impl crate::Writable for MOD_MIRROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOD_MIRROR to value 0"]
impl crate::Resettable for MOD_MIRROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
