#[doc = "Register `SOSCDIV` reader"]
pub struct R(crate::R<SOSCDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOSCDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOSCDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOSCDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOSCDIV` writer"]
pub struct W(crate::W<SOSCDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOSCDIV_SPEC>;
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
impl From<crate::W<SOSCDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOSCDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System OSC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOSCDIV1_A {
    #[doc = "0: Output disabled"]
    SOSCDIV1_0 = 0,
    #[doc = "1: Divide by 1"]
    SOSCDIV1_1 = 1,
    #[doc = "2: Divide by 2"]
    SOSCDIV1_2 = 2,
    #[doc = "3: Divide by 4"]
    SOSCDIV1_3 = 3,
    #[doc = "4: Divide by 8"]
    SOSCDIV1_4 = 4,
    #[doc = "5: Divide by 16"]
    SOSCDIV1_5 = 5,
    #[doc = "6: Divide by 32"]
    SOSCDIV1_6 = 6,
    #[doc = "7: Divide by 64"]
    SOSCDIV1_7 = 7,
}
impl From<SOSCDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SOSCDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOSCDIV1` reader - System OSC Clock Divide 1"]
pub struct SOSCDIV1_R(crate::FieldReader<u8, SOSCDIV1_A>);
impl SOSCDIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOSCDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCDIV1_A {
        match self.bits {
            0 => SOSCDIV1_A::SOSCDIV1_0,
            1 => SOSCDIV1_A::SOSCDIV1_1,
            2 => SOSCDIV1_A::SOSCDIV1_2,
            3 => SOSCDIV1_A::SOSCDIV1_3,
            4 => SOSCDIV1_A::SOSCDIV1_4,
            5 => SOSCDIV1_A::SOSCDIV1_5,
            6 => SOSCDIV1_A::SOSCDIV1_6,
            7 => SOSCDIV1_A::SOSCDIV1_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_0`"]
    #[inline(always)]
    pub fn is_soscdiv1_0(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_0
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_1`"]
    #[inline(always)]
    pub fn is_soscdiv1_1(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_1
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_2`"]
    #[inline(always)]
    pub fn is_soscdiv1_2(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_2
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_3`"]
    #[inline(always)]
    pub fn is_soscdiv1_3(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_3
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_4`"]
    #[inline(always)]
    pub fn is_soscdiv1_4(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_4
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_5`"]
    #[inline(always)]
    pub fn is_soscdiv1_5(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_5
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_6`"]
    #[inline(always)]
    pub fn is_soscdiv1_6(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_6
    }
    #[doc = "Checks if the value of the field is `SOSCDIV1_7`"]
    #[inline(always)]
    pub fn is_soscdiv1_7(&self) -> bool {
        **self == SOSCDIV1_A::SOSCDIV1_7
    }
}
impl core::ops::Deref for SOSCDIV1_R {
    type Target = crate::FieldReader<u8, SOSCDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCDIV1` writer - System OSC Clock Divide 1"]
pub struct SOSCDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn soscdiv1_0(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_0)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn soscdiv1_1(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn soscdiv1_2(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn soscdiv1_3(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn soscdiv1_4(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_4)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn soscdiv1_5(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_5)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn soscdiv1_6(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_6)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn soscdiv1_7(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::SOSCDIV1_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "System OSC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOSCDIV2_A {
    #[doc = "0: Output disabled"]
    SOSCDIV2_0 = 0,
    #[doc = "1: Divide by 1"]
    SOSCDIV2_1 = 1,
    #[doc = "2: Divide by 2"]
    SOSCDIV2_2 = 2,
    #[doc = "3: Divide by 4"]
    SOSCDIV2_3 = 3,
    #[doc = "4: Divide by 8"]
    SOSCDIV2_4 = 4,
    #[doc = "5: Divide by 16"]
    SOSCDIV2_5 = 5,
    #[doc = "6: Divide by 32"]
    SOSCDIV2_6 = 6,
    #[doc = "7: Divide by 64"]
    SOSCDIV2_7 = 7,
}
impl From<SOSCDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SOSCDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOSCDIV2` reader - System OSC Clock Divide 2"]
pub struct SOSCDIV2_R(crate::FieldReader<u8, SOSCDIV2_A>);
impl SOSCDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOSCDIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCDIV2_A {
        match self.bits {
            0 => SOSCDIV2_A::SOSCDIV2_0,
            1 => SOSCDIV2_A::SOSCDIV2_1,
            2 => SOSCDIV2_A::SOSCDIV2_2,
            3 => SOSCDIV2_A::SOSCDIV2_3,
            4 => SOSCDIV2_A::SOSCDIV2_4,
            5 => SOSCDIV2_A::SOSCDIV2_5,
            6 => SOSCDIV2_A::SOSCDIV2_6,
            7 => SOSCDIV2_A::SOSCDIV2_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_0`"]
    #[inline(always)]
    pub fn is_soscdiv2_0(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_0
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_1`"]
    #[inline(always)]
    pub fn is_soscdiv2_1(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_1
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_2`"]
    #[inline(always)]
    pub fn is_soscdiv2_2(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_2
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_3`"]
    #[inline(always)]
    pub fn is_soscdiv2_3(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_3
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_4`"]
    #[inline(always)]
    pub fn is_soscdiv2_4(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_4
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_5`"]
    #[inline(always)]
    pub fn is_soscdiv2_5(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_5
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_6`"]
    #[inline(always)]
    pub fn is_soscdiv2_6(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_6
    }
    #[doc = "Checks if the value of the field is `SOSCDIV2_7`"]
    #[inline(always)]
    pub fn is_soscdiv2_7(&self) -> bool {
        **self == SOSCDIV2_A::SOSCDIV2_7
    }
}
impl core::ops::Deref for SOSCDIV2_R {
    type Target = crate::FieldReader<u8, SOSCDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCDIV2` writer - System OSC Clock Divide 2"]
pub struct SOSCDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn soscdiv2_0(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_0)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn soscdiv2_1(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn soscdiv2_2(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn soscdiv2_3(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn soscdiv2_4(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_4)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn soscdiv2_5(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_5)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn soscdiv2_6(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_6)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn soscdiv2_7(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::SOSCDIV2_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline(always)]
    pub fn soscdiv1(&self) -> SOSCDIV1_R {
        SOSCDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline(always)]
    pub fn soscdiv2(&self) -> SOSCDIV2_R {
        SOSCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline(always)]
    pub fn soscdiv1(&mut self) -> SOSCDIV1_W {
        SOSCDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline(always)]
    pub fn soscdiv2(&mut self) -> SOSCDIV2_W {
        SOSCDIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System OSC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soscdiv](index.html) module"]
pub struct SOSCDIV_SPEC;
impl crate::RegisterSpec for SOSCDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soscdiv::R](R) reader structure"]
impl crate::Readable for SOSCDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soscdiv::W](W) writer structure"]
impl crate::Writable for SOSCDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOSCDIV to value 0"]
impl crate::Resettable for SOSCDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
