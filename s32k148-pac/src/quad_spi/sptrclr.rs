#[doc = "Register `SPTRCLR` reader"]
pub struct R(crate::R<SPTRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPTRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPTRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPTRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPTRCLR` writer"]
pub struct W(crate::W<SPTRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPTRCLR_SPEC>;
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
impl From<crate::W<SPTRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPTRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFPTRC` writer - Buffer Pointer Clear: 1: Clears the sequence pointer for AHB accesses as defined in QuadSPI_BFGENCR"]
pub struct BFPTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BFPTRC_W<'a> {
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
#[doc = "Field `IPPTRC` writer - IP Pointer Clear: 1: Clears the sequence pointer for IP accesses as defined in QuadSPI_IPCR This is a self-clearing field"]
pub struct IPPTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPPTRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Pointer Clear: 1: Clears the sequence pointer for AHB accesses as defined in QuadSPI_BFGENCR"]
    #[inline(always)]
    pub fn bfptrc(&mut self) -> BFPTRC_W {
        BFPTRC_W { w: self }
    }
    #[doc = "Bit 8 - IP Pointer Clear: 1: Clears the sequence pointer for IP accesses as defined in QuadSPI_IPCR This is a self-clearing field"]
    #[inline(always)]
    pub fn ipptrc(&mut self) -> IPPTRC_W {
        IPPTRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Pointer Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrclr](index.html) module"]
pub struct SPTRCLR_SPEC;
impl crate::RegisterSpec for SPTRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sptrclr::R](R) reader structure"]
impl crate::Readable for SPTRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sptrclr::W](W) writer structure"]
impl crate::Writable for SPTRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPTRCLR to value 0"]
impl crate::Resettable for SPTRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
