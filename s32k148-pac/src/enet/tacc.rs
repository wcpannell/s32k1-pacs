#[doc = "Register `TACC` reader"]
pub struct R(crate::R<TACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACC` writer"]
pub struct W(crate::W<TACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACC_SPEC>;
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
impl From<crate::W<TACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX FIFO Shift-16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIFT16_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    _1 = 1,
}
impl From<SHIFT16_A> for bool {
    #[inline(always)]
    fn from(variant: SHIFT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHIFT16` reader - TX FIFO Shift-16"]
pub struct SHIFT16_R(crate::FieldReader<bool, SHIFT16_A>);
impl SHIFT16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHIFT16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIFT16_A {
        match self.bits {
            false => SHIFT16_A::_0,
            true => SHIFT16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SHIFT16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SHIFT16_A::_1
    }
}
impl core::ops::Deref for SHIFT16_R {
    type Target = crate::FieldReader<bool, SHIFT16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT16` writer - TX FIFO Shift-16"]
pub struct SHIFT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIFT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHIFT16_A::_0)
    }
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHIFT16_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Enables insertion of IP header checksum.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCHK_A {
    #[doc = "0: Checksum is not inserted."]
    _0 = 0,
    #[doc = "1: If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    _1 = 1,
}
impl From<IPCHK_A> for bool {
    #[inline(always)]
    fn from(variant: IPCHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPCHK` reader - Enables insertion of IP header checksum."]
pub struct IPCHK_R(crate::FieldReader<bool, IPCHK_A>);
impl IPCHK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPCHK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCHK_A {
        match self.bits {
            false => IPCHK_A::_0,
            true => IPCHK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IPCHK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IPCHK_A::_1
    }
}
impl core::ops::Deref for IPCHK_R {
    type Target = crate::FieldReader<bool, IPCHK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCHK` writer - Enables insertion of IP header checksum."]
pub struct IPCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPCHK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Checksum is not inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPCHK_A::_0)
    }
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPCHK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enables insertion of protocol checksum.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCHK_A {
    #[doc = "0: Checksum not inserted."]
    _0 = 0,
    #[doc = "1: If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    _1 = 1,
}
impl From<PROCHK_A> for bool {
    #[inline(always)]
    fn from(variant: PROCHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCHK` reader - Enables insertion of protocol checksum."]
pub struct PROCHK_R(crate::FieldReader<bool, PROCHK_A>);
impl PROCHK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROCHK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROCHK_A {
        match self.bits {
            false => PROCHK_A::_0,
            true => PROCHK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROCHK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROCHK_A::_1
    }
}
impl core::ops::Deref for PROCHK_R {
    type Target = crate::FieldReader<bool, PROCHK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCHK` writer - Enables insertion of protocol checksum."]
pub struct PROCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROCHK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Checksum not inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROCHK_A::_0)
    }
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROCHK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&self) -> SHIFT16_R {
        SHIFT16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline(always)]
    pub fn ipchk(&self) -> IPCHK_R {
        IPCHK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline(always)]
    pub fn prochk(&self) -> PROCHK_R {
        PROCHK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&mut self) -> SHIFT16_W {
        SHIFT16_W { w: self }
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline(always)]
    pub fn ipchk(&mut self) -> IPCHK_W {
        IPCHK_W { w: self }
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline(always)]
    pub fn prochk(&mut self) -> PROCHK_W {
        PROCHK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Accelerator Function Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tacc](index.html) module"]
pub struct TACC_SPEC;
impl crate::RegisterSpec for TACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tacc::R](R) reader structure"]
impl crate::Readable for TACC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tacc::W](W) writer structure"]
impl crate::Writable for TACC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACC to value 0"]
impl crate::Resettable for TACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
