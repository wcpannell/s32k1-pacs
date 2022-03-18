#[doc = "Register `SIRCDIV` reader"]
pub struct R(crate::R<SIRCDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIRCDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIRCDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIRCDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIRCDIV` writer"]
pub struct W(crate::W<SIRCDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIRCDIV_SPEC>;
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
impl From<crate::W<SIRCDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIRCDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slow IRC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIRCDIV1_A {
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
impl From<SIRCDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SIRCDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIRCDIV1` reader - Slow IRC Clock Divide 1"]
pub struct SIRCDIV1_R(crate::FieldReader<u8, SIRCDIV1_A>);
impl SIRCDIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIRCDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCDIV1_A {
        match self.bits {
            0 => SIRCDIV1_A::_000,
            1 => SIRCDIV1_A::_001,
            2 => SIRCDIV1_A::_010,
            3 => SIRCDIV1_A::_011,
            4 => SIRCDIV1_A::_100,
            5 => SIRCDIV1_A::_101,
            6 => SIRCDIV1_A::_110,
            7 => SIRCDIV1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == SIRCDIV1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == SIRCDIV1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == SIRCDIV1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == SIRCDIV1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == SIRCDIV1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == SIRCDIV1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == SIRCDIV1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == SIRCDIV1_A::_111
    }
}
impl core::ops::Deref for SIRCDIV1_R {
    type Target = crate::FieldReader<u8, SIRCDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRCDIV1` writer - Slow IRC Clock Divide 1"]
pub struct SIRCDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Slow IRC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIRCDIV2_A {
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
impl From<SIRCDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SIRCDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIRCDIV2` reader - Slow IRC Clock Divide 2"]
pub struct SIRCDIV2_R(crate::FieldReader<u8, SIRCDIV2_A>);
impl SIRCDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIRCDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCDIV2_A {
        match self.bits {
            0 => SIRCDIV2_A::_000,
            1 => SIRCDIV2_A::_001,
            2 => SIRCDIV2_A::_010,
            3 => SIRCDIV2_A::_011,
            4 => SIRCDIV2_A::_100,
            5 => SIRCDIV2_A::_101,
            6 => SIRCDIV2_A::_110,
            7 => SIRCDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == SIRCDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == SIRCDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == SIRCDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == SIRCDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == SIRCDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == SIRCDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == SIRCDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == SIRCDIV2_A::_111
    }
}
impl core::ops::Deref for SIRCDIV2_R {
    type Target = crate::FieldReader<u8, SIRCDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRCDIV2` writer - Slow IRC Clock Divide 2"]
pub struct SIRCDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline(always)]
    pub fn sircdiv1(&self) -> SIRCDIV1_R {
        SIRCDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline(always)]
    pub fn sircdiv2(&self) -> SIRCDIV2_R {
        SIRCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline(always)]
    pub fn sircdiv1(&mut self) -> SIRCDIV1_W {
        SIRCDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline(always)]
    pub fn sircdiv2(&mut self) -> SIRCDIV2_W {
        SIRCDIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sircdiv](index.html) module"]
pub struct SIRCDIV_SPEC;
impl crate::RegisterSpec for SIRCDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sircdiv::R](R) reader structure"]
impl crate::Readable for SIRCDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sircdiv::W](W) writer structure"]
impl crate::Writable for SIRCDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIRCDIV to value 0"]
impl crate::Resettable for SIRCDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
