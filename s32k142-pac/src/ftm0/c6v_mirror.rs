#[doc = "Register `C6V_MIRROR` reader"]
pub struct R(crate::R<C6V_MIRROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C6V_MIRROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C6V_MIRROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C6V_MIRROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C6V_MIRROR` writer"]
pub struct W(crate::W<C6V_MIRROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C6V_MIRROR_SPEC>;
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
impl From<crate::W<C6V_MIRROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C6V_MIRROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACVAL` reader - Channel (n) Match Fractional Value"]
pub struct FRACVAL_R(crate::FieldReader<u8, u8>);
impl FRACVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRACVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACVAL` writer - Channel (n) Match Fractional Value"]
pub struct FRACVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `VAL` reader - Mirror of the Channel (n) Match Integer Value"]
pub struct VAL_R(crate::FieldReader<u16, u16>);
impl VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - Mirror of the Channel (n) Match Integer Value"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Channel (n) Match Fractional Value"]
    #[inline(always)]
    pub fn fracval(&self) -> FRACVAL_R {
        FRACVAL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Mirror of the Channel (n) Match Integer Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:15 - Channel (n) Match Fractional Value"]
    #[inline(always)]
    pub fn fracval(&mut self) -> FRACVAL_W {
        FRACVAL_W { w: self }
    }
    #[doc = "Bits 16:31 - Mirror of the Channel (n) Match Integer Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mirror of Channel (n) Match Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6v_mirror](index.html) module"]
pub struct C6V_MIRROR_SPEC;
impl crate::RegisterSpec for C6V_MIRROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c6v_mirror::R](R) reader structure"]
impl crate::Readable for C6V_MIRROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c6v_mirror::W](W) writer structure"]
impl crate::Writable for C6V_MIRROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C6V_MIRROR to value 0"]
impl crate::Resettable for C6V_MIRROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
