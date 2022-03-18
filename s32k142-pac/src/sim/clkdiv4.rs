#[doc = "Register `CLKDIV4` reader"]
pub struct R(crate::R<CLKDIV4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV4` writer"]
pub struct W(crate::W<CLKDIV4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV4_SPEC>;
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
impl From<crate::W<CLKDIV4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEFRAC` reader - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
pub struct TRACEFRAC_R(crate::FieldReader<bool, bool>);
impl TRACEFRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRACEFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACEFRAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEFRAC` writer - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
pub struct TRACEFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEFRAC_W<'a> {
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
#[doc = "Field `TRACEDIV` reader - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
pub struct TRACEDIV_R(crate::FieldReader<u8, u8>);
impl TRACEDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRACEDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACEDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEDIV` writer - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Debug Trace Divider control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACEDIVEN_A {
    #[doc = "0: Debug trace divider disabled"]
    _0 = 0,
    #[doc = "1: Debug trace divider enabled"]
    _1 = 1,
}
impl From<TRACEDIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACEDIVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACEDIVEN` reader - Debug Trace Divider control"]
pub struct TRACEDIVEN_R(crate::FieldReader<bool, TRACEDIVEN_A>);
impl TRACEDIVEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRACEDIVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEDIVEN_A {
        match self.bits {
            false => TRACEDIVEN_A::_0,
            true => TRACEDIVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRACEDIVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRACEDIVEN_A::_1
    }
}
impl core::ops::Deref for TRACEDIVEN_R {
    type Target = crate::FieldReader<bool, TRACEDIVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEDIVEN` writer - Debug Trace Divider control"]
pub struct TRACEDIVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEDIVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Debug trace divider disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACEDIVEN_A::_0)
    }
    #[doc = "Debug trace divider enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACEDIVEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
    #[inline(always)]
    pub fn tracefrac(&self) -> TRACEFRAC_R {
        TRACEFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Debug Trace Divider control"]
    #[inline(always)]
    pub fn tracediven(&self) -> TRACEDIVEN_R {
        TRACEDIVEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
    #[inline(always)]
    pub fn tracefrac(&mut self) -> TRACEFRAC_W {
        TRACEFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
    #[doc = "Bit 28 - Debug Trace Divider control"]
    #[inline(always)]
    pub fn tracediven(&mut self) -> TRACEDIVEN_W {
        TRACEDIVEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Divider Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv4](index.html) module"]
pub struct CLKDIV4_SPEC;
impl crate::RegisterSpec for CLKDIV4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv4::R](R) reader structure"]
impl crate::Readable for CLKDIV4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv4::W](W) writer structure"]
impl crate::Writable for CLKDIV4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV4 to value 0x1000_0000"]
impl crate::Resettable for CLKDIV4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
