#[doc = "Register `CH%sDLY5` reader"]
pub struct R(crate::R<CHDLY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDLY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDLY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDLY5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sDLY5` writer"]
pub struct W(crate::W<CHDLY5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDLY5_SPEC>;
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
impl From<crate::W<CHDLY5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDLY5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - PDB Channel Delay"]
pub struct DLY_R(crate::FieldReader<u16, u16>);
impl DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY` writer - PDB Channel Delay"]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Delay 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly5](index.html) module"]
pub struct CHDLY5_SPEC;
impl crate::RegisterSpec for CHDLY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdly5::R](R) reader structure"]
impl crate::Readable for CHDLY5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdly5::W](W) writer structure"]
impl crate::Writable for CHDLY5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%sDLY5 to value 0"]
impl crate::Resettable for CHDLY5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
