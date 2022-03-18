#[doc = "Register `LCKCR` reader"]
pub struct R(crate::R<LCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCKCR` writer"]
pub struct W(crate::W<LCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKCR_SPEC>;
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
impl From<crate::W<LCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Locks the LUT when the following condition is met: This register is written just after the LUTKEYLUT Key Register The LUT key register was written with 0x5AF05AF0 key"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Locks the LUT when the following condition is met: This register is written just after the LUTKEYLUT Key Register The LUT key register was written with 0x5AF05AF0 key"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `UNLOCK` reader - Unlocks the LUT when the following two conditions are met: 1"]
pub struct UNLOCK_R(crate::FieldReader<bool, bool>);
impl UNLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCK` writer - Unlocks the LUT when the following two conditions are met: 1"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Locks the LUT when the following condition is met: This register is written just after the LUTKEYLUT Key Register The LUT key register was written with 0x5AF05AF0 key"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Unlocks the LUT when the following two conditions are met: 1"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Locks the LUT when the following condition is met: This register is written just after the LUTKEYLUT Key Register The LUT key register was written with 0x5AF05AF0 key"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 1 - Unlocks the LUT when the following two conditions are met: 1"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Lock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckcr](index.html) module"]
pub struct LCKCR_SPEC;
impl crate::RegisterSpec for LCKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lckcr::R](R) reader structure"]
impl crate::Readable for LCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lckcr::W](W) writer structure"]
impl crate::Writable for LCKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCKCR to value 0x02"]
impl crate::Resettable for LCKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
