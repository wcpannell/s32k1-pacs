#[doc = "Register `FIRCDIV` reader"]
pub struct R(crate::R<FIRCDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIRCDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIRCDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIRCDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIRCDIV` writer"]
pub struct W(crate::W<FIRCDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRCDIV_SPEC>;
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
impl From<crate::W<FIRCDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRCDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fast IRC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIRCDIV1_A {
    #[doc = "0: Output disabled"]
    _000 = 0,
    #[doc = "1: Divide by 1"]
    _001 = 1,
    #[doc = "2: Divide by 2"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 8"]
    _100 = 4,
    #[doc = "5: Divide by 16"]
    _101 = 5,
    #[doc = "6: Divide by 32"]
    _110 = 6,
    #[doc = "7: Divide by 64"]
    _111 = 7,
}
impl From<FIRCDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: FIRCDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIRCDIV1` reader - Fast IRC Clock Divide 1"]
pub struct FIRCDIV1_R(crate::FieldReader<u8, FIRCDIV1_A>);
impl FIRCDIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIRCDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCDIV1_A {
        match self.bits {
            0 => FIRCDIV1_A::_000,
            1 => FIRCDIV1_A::_001,
            2 => FIRCDIV1_A::_010,
            3 => FIRCDIV1_A::_011,
            4 => FIRCDIV1_A::_100,
            5 => FIRCDIV1_A::_101,
            6 => FIRCDIV1_A::_110,
            7 => FIRCDIV1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FIRCDIV1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FIRCDIV1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == FIRCDIV1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == FIRCDIV1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == FIRCDIV1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == FIRCDIV1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == FIRCDIV1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == FIRCDIV1_A::_111
    }
}
impl core::ops::Deref for FIRCDIV1_R {
    type Target = crate::FieldReader<u8, FIRCDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIRCDIV1` writer - Fast IRC Clock Divide 1"]
pub struct FIRCDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRCDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIRCDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FIRCDIV1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Fast IRC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIRCDIV2_A {
    #[doc = "0: Output disabled"]
    _000 = 0,
    #[doc = "1: Divide by 1"]
    _001 = 1,
    #[doc = "2: Divide by 2"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 8"]
    _100 = 4,
    #[doc = "5: Divide by 16"]
    _101 = 5,
    #[doc = "6: Divide by 32"]
    _110 = 6,
    #[doc = "7: Divide by 64"]
    _111 = 7,
}
impl From<FIRCDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: FIRCDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIRCDIV2` reader - Fast IRC Clock Divide 2"]
pub struct FIRCDIV2_R(crate::FieldReader<u8, FIRCDIV2_A>);
impl FIRCDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIRCDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRCDIV2_A {
        match self.bits {
            0 => FIRCDIV2_A::_000,
            1 => FIRCDIV2_A::_001,
            2 => FIRCDIV2_A::_010,
            3 => FIRCDIV2_A::_011,
            4 => FIRCDIV2_A::_100,
            5 => FIRCDIV2_A::_101,
            6 => FIRCDIV2_A::_110,
            7 => FIRCDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FIRCDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FIRCDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == FIRCDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == FIRCDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == FIRCDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == FIRCDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == FIRCDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == FIRCDIV2_A::_111
    }
}
impl core::ops::Deref for FIRCDIV2_R {
    type Target = crate::FieldReader<u8, FIRCDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIRCDIV2` writer - Fast IRC Clock Divide 2"]
pub struct FIRCDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRCDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIRCDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FIRCDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
    #[inline(always)]
    pub fn fircdiv1(&self) -> FIRCDIV1_R {
        FIRCDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
    #[inline(always)]
    pub fn fircdiv2(&self) -> FIRCDIV2_R {
        FIRCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
    #[inline(always)]
    pub fn fircdiv1(&mut self) -> FIRCDIV1_W {
        FIRCDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
    #[inline(always)]
    pub fn fircdiv2(&mut self) -> FIRCDIV2_W {
        FIRCDIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fircdiv](index.html) module"]
pub struct FIRCDIV_SPEC;
impl crate::RegisterSpec for FIRCDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fircdiv::R](R) reader structure"]
impl crate::Readable for FIRCDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fircdiv::W](W) writer structure"]
impl crate::Writable for FIRCDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIRCDIV to value 0"]
impl crate::Resettable for FIRCDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
