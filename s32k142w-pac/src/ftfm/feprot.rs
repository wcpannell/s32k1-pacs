#[doc = "Register `FEPROT` reader"]
pub struct R(crate::R<FEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEPROT` writer"]
pub struct W(crate::W<FEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEPROT_SPEC>;
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
impl From<crate::W<FEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPROT` reader - EEPROM Region Protect"]
pub struct EPROT_R(crate::FieldReader<u8, u8>);
impl EPROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPROT` writer - EEPROM Region Protect"]
pub struct EPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> EPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&mut self) -> EPROT_W {
        EPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feprot](index.html) module"]
pub struct FEPROT_SPEC;
impl crate::RegisterSpec for FEPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [feprot::R](R) reader structure"]
impl crate::Readable for FEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feprot::W](W) writer structure"]
impl crate::Writable for FEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEPROT to value 0"]
impl crate::Resettable for FEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
