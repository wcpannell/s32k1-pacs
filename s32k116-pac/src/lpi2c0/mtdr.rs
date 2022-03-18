#[doc = "Register `MTDR` reader"]
pub struct R(crate::R<MTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTDR` writer"]
pub struct W(crate::W<MTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTDR_SPEC>;
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
impl From<crate::W<MTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Transmit Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Command Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_AW {
    #[doc = "0: Transmit DATA\\[7:0\\]."]
    _000 = 0,
    #[doc = "1: Receive (DATA\\[7:0\\]
+ 1) bytes."]
    _001 = 1,
    #[doc = "2: Generate STOP condition."]
    _010 = 2,
    #[doc = "3: Receive and discard (DATA\\[7:0\\]
+ 1) bytes."]
    _011 = 3,
    #[doc = "4: Generate (repeated) START and transmit address in DATA\\[7:0\\]."]
    _100 = 4,
    #[doc = "5: Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    _101 = 5,
    #[doc = "6: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode."]
    _110 = 6,
    #[doc = "7: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    _111 = 7,
}
impl From<CMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` writer - Command Data"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Transmit DATA\\[7:0\\]."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CMD_AW::_000)
    }
    #[doc = "Receive (DATA\\[7:0\\]
+ 1) bytes."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CMD_AW::_001)
    }
    #[doc = "Generate STOP condition."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CMD_AW::_010)
    }
    #[doc = "Receive and discard (DATA\\[7:0\\]
+ 1) bytes."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CMD_AW::_011)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CMD_AW::_100)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CMD_AW::_101)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CMD_AW::_110)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CMD_AW::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtdr](index.html) module"]
pub struct MTDR_SPEC;
impl crate::RegisterSpec for MTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtdr::R](R) reader structure"]
impl crate::Readable for MTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtdr::W](W) writer structure"]
impl crate::Writable for MTDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTDR to value 0"]
impl crate::Resettable for MTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
