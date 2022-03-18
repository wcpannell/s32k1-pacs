#[doc = "Register `LUT%s` reader"]
pub struct R(crate::R<LUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT%s` writer"]
pub struct W(crate::W<LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_SPEC>;
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
impl From<crate::W<LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPRND0` reader - Operand for INSTR0."]
pub struct OPRND0_R(crate::FieldReader<u8, u8>);
impl OPRND0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPRND0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPRND0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPRND0` writer - Operand for INSTR0."]
pub struct OPRND0_W<'a> {
    w: &'a mut W,
}
impl<'a> OPRND0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Pad information for INSTR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD0_A {
    #[doc = "0: 1 Pad"]
    _00 = 0,
    #[doc = "1: 2 Pads"]
    _01 = 1,
    #[doc = "2: 4 Pads"]
    _10 = 2,
    #[doc = "3: 8 Pads"]
    _11 = 3,
}
impl From<PAD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD0` reader - Pad information for INSTR0."]
pub struct PAD0_R(crate::FieldReader<u8, PAD0_A>);
impl PAD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0_A {
        match self.bits {
            0 => PAD0_A::_00,
            1 => PAD0_A::_01,
            2 => PAD0_A::_10,
            3 => PAD0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PAD0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PAD0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PAD0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PAD0_A::_11
    }
}
impl core::ops::Deref for PAD0_R {
    type Target = crate::FieldReader<u8, PAD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0` writer - Pad information for INSTR0."]
pub struct PAD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 Pad"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PAD0_A::_00)
    }
    #[doc = "2 Pads"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PAD0_A::_01)
    }
    #[doc = "4 Pads"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PAD0_A::_10)
    }
    #[doc = "8 Pads"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PAD0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `INSTR0` reader - Instruction 0"]
pub struct INSTR0_R(crate::FieldReader<u8, u8>);
impl INSTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTR0` writer - Instruction 0"]
pub struct INSTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `OPRND1` reader - Operand for INSTR1."]
pub struct OPRND1_R(crate::FieldReader<u8, u8>);
impl OPRND1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPRND1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPRND1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPRND1` writer - Operand for INSTR1."]
pub struct OPRND1_W<'a> {
    w: &'a mut W,
}
impl<'a> OPRND1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Pad information for INSTR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD1_A {
    #[doc = "0: 1 Pad"]
    _00 = 0,
    #[doc = "1: 2 Pads"]
    _01 = 1,
    #[doc = "2: 4 Pads"]
    _10 = 2,
    #[doc = "3: 8 Pads"]
    _11 = 3,
}
impl From<PAD1_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD1` reader - Pad information for INSTR1."]
pub struct PAD1_R(crate::FieldReader<u8, PAD1_A>);
impl PAD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1_A {
        match self.bits {
            0 => PAD1_A::_00,
            1 => PAD1_A::_01,
            2 => PAD1_A::_10,
            3 => PAD1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PAD1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PAD1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PAD1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PAD1_A::_11
    }
}
impl core::ops::Deref for PAD1_R {
    type Target = crate::FieldReader<u8, PAD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1` writer - Pad information for INSTR1."]
pub struct PAD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 Pad"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PAD1_A::_00)
    }
    #[doc = "2 Pads"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PAD1_A::_01)
    }
    #[doc = "4 Pads"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PAD1_A::_10)
    }
    #[doc = "8 Pads"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PAD1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INSTR1` reader - Instruction 1"]
pub struct INSTR1_R(crate::FieldReader<u8, u8>);
impl INSTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INSTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTR1` writer - Instruction 1"]
pub struct INSTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Operand for INSTR0."]
    #[inline(always)]
    pub fn oprnd0(&self) -> OPRND0_R {
        OPRND0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Pad information for INSTR0."]
    #[inline(always)]
    pub fn pad0(&self) -> PAD0_R {
        PAD0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:15 - Instruction 0"]
    #[inline(always)]
    pub fn instr0(&self) -> INSTR0_R {
        INSTR0_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Operand for INSTR1."]
    #[inline(always)]
    pub fn oprnd1(&self) -> OPRND1_R {
        OPRND1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Pad information for INSTR1."]
    #[inline(always)]
    pub fn pad1(&self) -> PAD1_R {
        PAD1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:31 - Instruction 1"]
    #[inline(always)]
    pub fn instr1(&self) -> INSTR1_R {
        INSTR1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Operand for INSTR0."]
    #[inline(always)]
    pub fn oprnd0(&mut self) -> OPRND0_W {
        OPRND0_W { w: self }
    }
    #[doc = "Bits 8:9 - Pad information for INSTR0."]
    #[inline(always)]
    pub fn pad0(&mut self) -> PAD0_W {
        PAD0_W { w: self }
    }
    #[doc = "Bits 10:15 - Instruction 0"]
    #[inline(always)]
    pub fn instr0(&mut self) -> INSTR0_W {
        INSTR0_W { w: self }
    }
    #[doc = "Bits 16:23 - Operand for INSTR1."]
    #[inline(always)]
    pub fn oprnd1(&mut self) -> OPRND1_W {
        OPRND1_W { w: self }
    }
    #[doc = "Bits 24:25 - Pad information for INSTR1."]
    #[inline(always)]
    pub fn pad1(&mut self) -> PAD1_W {
        PAD1_W { w: self }
    }
    #[doc = "Bits 26:31 - Instruction 1"]
    #[inline(always)]
    pub fn instr1(&mut self) -> INSTR1_W {
        INSTR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Look-up Table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut](index.html) module"]
pub struct LUT_SPEC;
impl crate::RegisterSpec for LUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut::R](R) reader structure"]
impl crate::Readable for LUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut::W](W) writer structure"]
impl crate::Writable for LUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT%s to value 0"]
impl crate::Resettable for LUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
