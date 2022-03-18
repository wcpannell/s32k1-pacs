#[doc = "Register `C1V` reader"]
pub struct R(crate::R<C1V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1V` writer"]
pub struct W(crate::W<C1V_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1V_SPEC>;
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
impl From<crate::W<C1V_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1V_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - Channel Value"]
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
#[doc = "Field `VAL` writer - Channel Value"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Value"]
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
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1v](index.html) module"]
pub struct C1V_SPEC;
impl crate::RegisterSpec for C1V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1v::R](R) reader structure"]
impl crate::Readable for C1V_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1v::W](W) writer structure"]
impl crate::Writable for C1V_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1V to value 0"]
impl crate::Resettable for C1V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
