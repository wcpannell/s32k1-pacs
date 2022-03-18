#[doc = "Register `SPLLDIV` reader"]
pub struct R(crate::R<SPLLDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLLDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLLDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLLDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPLLDIV` writer"]
pub struct W(crate::W<SPLLDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLLDIV_SPEC>;
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
impl From<crate::W<SPLLDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLLDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System PLL Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPLLDIV1_A {
    #[doc = "0: Clock disabled"]
    SPLLDIV1_0 = 0,
    #[doc = "1: Divide by 1"]
    SPLLDIV1_1 = 1,
    #[doc = "2: Divide by 2"]
    SPLLDIV1_2 = 2,
    #[doc = "3: Divide by 4"]
    SPLLDIV1_3 = 3,
    #[doc = "4: Divide by 8"]
    SPLLDIV1_4 = 4,
    #[doc = "5: Divide by 16"]
    SPLLDIV1_5 = 5,
    #[doc = "6: Divide by 32"]
    SPLLDIV1_6 = 6,
    #[doc = "7: Divide by 64"]
    SPLLDIV1_7 = 7,
}
impl From<SPLLDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SPLLDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPLLDIV1` reader - System PLL Clock Divide 1"]
pub struct SPLLDIV1_R(crate::FieldReader<u8, SPLLDIV1_A>);
impl SPLLDIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPLLDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLDIV1_A {
        match self.bits {
            0 => SPLLDIV1_A::SPLLDIV1_0,
            1 => SPLLDIV1_A::SPLLDIV1_1,
            2 => SPLLDIV1_A::SPLLDIV1_2,
            3 => SPLLDIV1_A::SPLLDIV1_3,
            4 => SPLLDIV1_A::SPLLDIV1_4,
            5 => SPLLDIV1_A::SPLLDIV1_5,
            6 => SPLLDIV1_A::SPLLDIV1_6,
            7 => SPLLDIV1_A::SPLLDIV1_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_0`"]
    #[inline(always)]
    pub fn is_splldiv1_0(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_0
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_1`"]
    #[inline(always)]
    pub fn is_splldiv1_1(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_1
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_2`"]
    #[inline(always)]
    pub fn is_splldiv1_2(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_2
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_3`"]
    #[inline(always)]
    pub fn is_splldiv1_3(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_3
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_4`"]
    #[inline(always)]
    pub fn is_splldiv1_4(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_4
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_5`"]
    #[inline(always)]
    pub fn is_splldiv1_5(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_5
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_6`"]
    #[inline(always)]
    pub fn is_splldiv1_6(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_6
    }
    #[doc = "Checks if the value of the field is `SPLLDIV1_7`"]
    #[inline(always)]
    pub fn is_splldiv1_7(&self) -> bool {
        **self == SPLLDIV1_A::SPLLDIV1_7
    }
}
impl core::ops::Deref for SPLLDIV1_R {
    type Target = crate::FieldReader<u8, SPLLDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLDIV1` writer - System PLL Clock Divide 1"]
pub struct SPLLDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn splldiv1_0(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_0)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn splldiv1_1(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn splldiv1_2(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn splldiv1_3(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn splldiv1_4(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_4)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn splldiv1_5(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_5)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn splldiv1_6(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_6)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn splldiv1_7(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::SPLLDIV1_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "System PLL Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPLLDIV2_A {
    #[doc = "0: Clock disabled"]
    SPLLDIV2_0 = 0,
    #[doc = "1: Divide by 1"]
    SPLLDIV2_1 = 1,
    #[doc = "2: Divide by 2"]
    SPLLDIV2_2 = 2,
    #[doc = "3: Divide by 4"]
    SPLLDIV2_3 = 3,
    #[doc = "4: Divide by 8"]
    SPLLDIV2_4 = 4,
    #[doc = "5: Divide by 16"]
    SPLLDIV2_5 = 5,
    #[doc = "6: Divide by 32"]
    SPLLDIV2_6 = 6,
    #[doc = "7: Divide by 64"]
    SPLLDIV2_7 = 7,
}
impl From<SPLLDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SPLLDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPLLDIV2` reader - System PLL Clock Divide 2"]
pub struct SPLLDIV2_R(crate::FieldReader<u8, SPLLDIV2_A>);
impl SPLLDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPLLDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLDIV2_A {
        match self.bits {
            0 => SPLLDIV2_A::SPLLDIV2_0,
            1 => SPLLDIV2_A::SPLLDIV2_1,
            2 => SPLLDIV2_A::SPLLDIV2_2,
            3 => SPLLDIV2_A::SPLLDIV2_3,
            4 => SPLLDIV2_A::SPLLDIV2_4,
            5 => SPLLDIV2_A::SPLLDIV2_5,
            6 => SPLLDIV2_A::SPLLDIV2_6,
            7 => SPLLDIV2_A::SPLLDIV2_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_0`"]
    #[inline(always)]
    pub fn is_splldiv2_0(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_0
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_1`"]
    #[inline(always)]
    pub fn is_splldiv2_1(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_1
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_2`"]
    #[inline(always)]
    pub fn is_splldiv2_2(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_2
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_3`"]
    #[inline(always)]
    pub fn is_splldiv2_3(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_3
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_4`"]
    #[inline(always)]
    pub fn is_splldiv2_4(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_4
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_5`"]
    #[inline(always)]
    pub fn is_splldiv2_5(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_5
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_6`"]
    #[inline(always)]
    pub fn is_splldiv2_6(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_6
    }
    #[doc = "Checks if the value of the field is `SPLLDIV2_7`"]
    #[inline(always)]
    pub fn is_splldiv2_7(&self) -> bool {
        **self == SPLLDIV2_A::SPLLDIV2_7
    }
}
impl core::ops::Deref for SPLLDIV2_R {
    type Target = crate::FieldReader<u8, SPLLDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLDIV2` writer - System PLL Clock Divide 2"]
pub struct SPLLDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn splldiv2_0(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_0)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn splldiv2_1(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn splldiv2_2(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn splldiv2_3(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn splldiv2_4(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_4)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn splldiv2_5(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_5)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn splldiv2_6(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_6)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn splldiv2_7(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::SPLLDIV2_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL Clock Divide 1"]
    #[inline(always)]
    pub fn splldiv1(&self) -> SPLLDIV1_R {
        SPLLDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - System PLL Clock Divide 2"]
    #[inline(always)]
    pub fn splldiv2(&self) -> SPLLDIV2_R {
        SPLLDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL Clock Divide 1"]
    #[inline(always)]
    pub fn splldiv1(&mut self) -> SPLLDIV1_W {
        SPLLDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - System PLL Clock Divide 2"]
    #[inline(always)]
    pub fn splldiv2(&mut self) -> SPLLDIV2_W {
        SPLLDIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [splldiv](index.html) module"]
pub struct SPLLDIV_SPEC;
impl crate::RegisterSpec for SPLLDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [splldiv::R](R) reader structure"]
impl crate::Readable for SPLLDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [splldiv::W](W) writer structure"]
impl crate::Writable for SPLLDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPLLDIV to value 0"]
impl crate::Resettable for SPLLDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
