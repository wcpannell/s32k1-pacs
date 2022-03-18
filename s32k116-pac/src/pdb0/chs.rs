#[doc = "Register `CH%sS` reader"]
pub struct R(crate::R<CHS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sS` writer"]
pub struct W(crate::W<CHS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHS_SPEC>;
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
impl From<crate::W<CHS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERR_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERR` reader - PDB Channel Sequence Error Flags"]
pub struct ERR_R(crate::FieldReader<u8, ERR_A>);
impl ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERR_A> {
        match self.bits {
            0 => Some(ERR_A::_0),
            1 => Some(ERR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR_A::_1
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<u8, ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - PDB Channel Sequence Error Flags"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CF` reader - PDB Channel Flags"]
pub struct CF_R(crate::FieldReader<u8, u8>);
impl CF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF` writer - PDB Channel Flags"]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chs](index.html) module"]
pub struct CHS_SPEC;
impl crate::RegisterSpec for CHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chs::R](R) reader structure"]
impl crate::Readable for CHS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chs::W](W) writer structure"]
impl crate::Writable for CHS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%sS to value 0"]
impl crate::Resettable for CHS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
