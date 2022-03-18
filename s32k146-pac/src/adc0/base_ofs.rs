#[doc = "Register `BASE_OFS` reader"]
pub struct R(crate::R<BASE_OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE_OFS` writer"]
pub struct W(crate::W<BASE_OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_OFS_SPEC>;
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
impl From<crate::W<BASE_OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BA_OFS` reader - Base Offset Error Correction Value"]
pub struct BA_OFS_R(crate::FieldReader<u8, u8>);
impl BA_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BA_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BA_OFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BA_OFS` writer - Base Offset Error Correction Value"]
pub struct BA_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Base Offset Error Correction Value"]
    #[inline(always)]
    pub fn ba_ofs(&self) -> BA_OFS_R {
        BA_OFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base Offset Error Correction Value"]
    #[inline(always)]
    pub fn ba_ofs(&mut self) -> BA_OFS_W {
        BA_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BASE Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_ofs](index.html) module"]
pub struct BASE_OFS_SPEC;
impl crate::RegisterSpec for BASE_OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base_ofs::R](R) reader structure"]
impl crate::Readable for BASE_OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base_ofs::W](W) writer structure"]
impl crate::Writable for BASE_OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASE_OFS to value 0x40"]
impl crate::Resettable for BASE_OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
