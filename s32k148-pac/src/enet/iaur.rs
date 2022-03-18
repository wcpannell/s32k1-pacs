#[doc = "Register `IAUR` reader"]
pub struct R(crate::R<IAUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IAUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IAUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IAUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IAUR` writer"]
pub struct W(crate::W<IAUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IAUR_SPEC>;
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
impl From<crate::W<IAUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IAUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IADDR1` reader - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
pub struct IADDR1_R(crate::FieldReader<u32, u32>);
impl IADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IADDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IADDR1` writer - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
pub struct IADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> IADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr1(&self) -> IADDR1_R {
        IADDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr1(&mut self) -> IADDR1_W {
        IADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Individual Upper Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iaur](index.html) module"]
pub struct IAUR_SPEC;
impl crate::RegisterSpec for IAUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iaur::R](R) reader structure"]
impl crate::Readable for IAUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iaur::W](W) writer structure"]
impl crate::Writable for IAUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IAUR to value 0"]
impl crate::Resettable for IAUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
