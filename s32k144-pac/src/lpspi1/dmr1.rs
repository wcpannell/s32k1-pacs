#[doc = "Register `DMR1` reader"]
pub struct R(crate::R<DMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMR1` writer"]
pub struct W(crate::W<DMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMR1_SPEC>;
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
impl From<crate::W<DMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH1` reader - Match 1 Value"]
pub struct MATCH1_R(crate::FieldReader<u32, u32>);
impl MATCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MATCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCH1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH1` writer - Match 1 Value"]
pub struct MATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&self) -> MATCH1_R {
        MATCH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&mut self) -> MATCH1_W {
        MATCH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Match Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmr1](index.html) module"]
pub struct DMR1_SPEC;
impl crate::RegisterSpec for DMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmr1::R](R) reader structure"]
impl crate::Readable for DMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmr1::W](W) writer structure"]
impl crate::Writable for DMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMR1 to value 0"]
impl crate::Resettable for DMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
