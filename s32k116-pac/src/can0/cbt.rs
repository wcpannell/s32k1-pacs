#[doc = "Register `CBT` reader"]
pub struct R(crate::R<CBT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBT` writer"]
pub struct W(crate::W<CBT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBT_SPEC>;
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
impl From<crate::W<CBT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPSEG2` reader - Extended Phase Segment 2"]
pub struct EPSEG2_R(crate::FieldReader<u8, u8>);
impl EPSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSEG2` writer - Extended Phase Segment 2"]
pub struct EPSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `EPSEG1` reader - Extended Phase Segment 1"]
pub struct EPSEG1_R(crate::FieldReader<u8, u8>);
impl EPSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSEG1` writer - Extended Phase Segment 1"]
pub struct EPSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `EPROPSEG` reader - Extended Propagation Segment"]
pub struct EPROPSEG_R(crate::FieldReader<u8, u8>);
impl EPROPSEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPROPSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPROPSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPROPSEG` writer - Extended Propagation Segment"]
pub struct EPROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> EPROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `ERJW` reader - Extended Resync Jump Width"]
pub struct ERJW_R(crate::FieldReader<u8, u8>);
impl ERJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERJW` writer - Extended Resync Jump Width"]
pub struct ERJW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `EPRESDIV` reader - Extended Prescaler Division Factor"]
pub struct EPRESDIV_R(crate::FieldReader<u16, u16>);
impl EPRESDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EPRESDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRESDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRESDIV` writer - Extended Prescaler Division Factor"]
pub struct EPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 21)) | ((value as u32 & 0x03ff) << 21);
        self.w
    }
}
#[doc = "Bit Timing Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTF_A {
    #[doc = "0: Extended bit time definitions disabled."]
    _0 = 0,
    #[doc = "1: Extended bit time definitions enabled."]
    _1 = 1,
}
impl From<BTF_A> for bool {
    #[inline(always)]
    fn from(variant: BTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTF` reader - Bit Timing Format Enable"]
pub struct BTF_R(crate::FieldReader<bool, BTF_A>);
impl BTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTF_A {
        match self.bits {
            false => BTF_A::_0,
            true => BTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BTF_A::_1
    }
}
impl core::ops::Deref for BTF_R {
    type Target = crate::FieldReader<bool, BTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTF` writer - Bit Timing Format Enable"]
pub struct BTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Extended bit time definitions disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTF_A::_0)
    }
    #[doc = "Extended bit time definitions enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline(always)]
    pub fn epseg2(&self) -> EPSEG2_R {
        EPSEG2_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline(always)]
    pub fn epseg1(&self) -> EPSEG1_R {
        EPSEG1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline(always)]
    pub fn epropseg(&self) -> EPROPSEG_R {
        EPROPSEG_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline(always)]
    pub fn erjw(&self) -> ERJW_R {
        ERJW_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline(always)]
    pub fn epresdiv(&self) -> EPRESDIV_R {
        EPRESDIV_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline(always)]
    pub fn epseg2(&mut self) -> EPSEG2_W {
        EPSEG2_W { w: self }
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline(always)]
    pub fn epseg1(&mut self) -> EPSEG1_W {
        EPSEG1_W { w: self }
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline(always)]
    pub fn epropseg(&mut self) -> EPROPSEG_W {
        EPROPSEG_W { w: self }
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline(always)]
    pub fn erjw(&mut self) -> ERJW_W {
        ERJW_W { w: self }
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline(always)]
    pub fn epresdiv(&mut self) -> EPRESDIV_W {
        EPRESDIV_W { w: self }
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline(always)]
    pub fn btf(&mut self) -> BTF_W {
        BTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbt](index.html) module"]
pub struct CBT_SPEC;
impl crate::RegisterSpec for CBT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbt::R](R) reader structure"]
impl crate::Readable for CBT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbt::W](W) writer structure"]
impl crate::Writable for CBT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CBT to value 0"]
impl crate::Resettable for CBT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
