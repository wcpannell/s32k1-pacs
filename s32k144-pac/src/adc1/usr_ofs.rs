#[doc = "Register `USR_OFS` reader"]
pub struct R(crate::R<USR_OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USR_OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USR_OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USR_OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USR_OFS` writer"]
pub struct W(crate::W<USR_OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USR_OFS_SPEC>;
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
impl From<crate::W<USR_OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USR_OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_OFS` reader - USER Offset Error Correction Value"]
pub struct USR_OFS_R(crate::FieldReader<u8, u8>);
impl USR_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_OFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_OFS` writer - USER Offset Error Correction Value"]
pub struct USR_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USER Offset Error Correction Value"]
    #[inline(always)]
    pub fn usr_ofs(&self) -> USR_OFS_R {
        USR_OFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USER Offset Error Correction Value"]
    #[inline(always)]
    pub fn usr_ofs(&mut self) -> USR_OFS_W {
        USR_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USER Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usr_ofs](index.html) module"]
pub struct USR_OFS_SPEC;
impl crate::RegisterSpec for USR_OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usr_ofs::R](R) reader structure"]
impl crate::Readable for USR_OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usr_ofs::W](W) writer structure"]
impl crate::Writable for USR_OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USR_OFS to value 0"]
impl crate::Resettable for USR_OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
