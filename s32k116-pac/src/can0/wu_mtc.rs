#[doc = "Register `WU_MTC` reader"]
pub struct R(crate::R<WU_MTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WU_MTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WU_MTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WU_MTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WU_MTC` writer"]
pub struct W(crate::W<WU_MTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WU_MTC_SPEC>;
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
impl From<crate::W<WU_MTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WU_MTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCOUNTER` reader - Number of Matches while in Pretended Networking"]
pub struct MCOUNTER_R(crate::FieldReader<u8, u8>);
impl MCOUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCOUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOUNTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wake Up by Match Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMF_A {
    #[doc = "0: No wake up by match event detected"]
    _0 = 0,
    #[doc = "1: Wake up by match event detected"]
    _1 = 1,
}
impl From<WUMF_A> for bool {
    #[inline(always)]
    fn from(variant: WUMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUMF` reader - Wake Up by Match Flag Bit"]
pub struct WUMF_R(crate::FieldReader<bool, WUMF_A>);
impl WUMF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUMF_A {
        match self.bits {
            false => WUMF_A::_0,
            true => WUMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUMF_A::_1
    }
}
impl core::ops::Deref for WUMF_R {
    type Target = crate::FieldReader<bool, WUMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUMF` writer - Wake Up by Match Flag Bit"]
pub struct WUMF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUMF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No wake up by match event detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUMF_A::_0)
    }
    #[doc = "Wake up by match event detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUMF_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Wake Up by Timeout Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOF_A {
    #[doc = "0: No wake up by timeout event detected"]
    _0 = 0,
    #[doc = "1: Wake up by timeout event detected"]
    _1 = 1,
}
impl From<WTOF_A> for bool {
    #[inline(always)]
    fn from(variant: WTOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTOF` reader - Wake Up by Timeout Flag Bit"]
pub struct WTOF_R(crate::FieldReader<bool, WTOF_A>);
impl WTOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WTOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTOF_A {
        match self.bits {
            false => WTOF_A::_0,
            true => WTOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WTOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WTOF_A::_1
    }
}
impl core::ops::Deref for WTOF_R {
    type Target = crate::FieldReader<bool, WTOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTOF` writer - Wake Up by Timeout Flag Bit"]
pub struct WTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No wake up by timeout event detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTOF_A::_0)
    }
    #[doc = "Wake up by timeout event detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTOF_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Number of Matches while in Pretended Networking"]
    #[inline(always)]
    pub fn mcounter(&self) -> MCOUNTER_R {
        MCOUNTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline(always)]
    pub fn wumf(&self) -> WUMF_R {
        WUMF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline(always)]
    pub fn wtof(&self) -> WTOF_R {
        WTOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline(always)]
    pub fn wumf(&mut self) -> WUMF_W {
        WUMF_W { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline(always)]
    pub fn wtof(&mut self) -> WTOF_W {
        WTOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking Wake Up Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wu_mtc](index.html) module"]
pub struct WU_MTC_SPEC;
impl crate::RegisterSpec for WU_MTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wu_mtc::R](R) reader structure"]
impl crate::Readable for WU_MTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wu_mtc::W](W) writer structure"]
impl crate::Writable for WU_MTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WU_MTC to value 0"]
impl crate::Resettable for WU_MTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
