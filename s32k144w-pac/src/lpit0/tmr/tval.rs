#[doc = "Register `TVAL` reader"]
pub struct R(crate::R<TVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TVAL` writer"]
pub struct W(crate::W<TVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVAL_SPEC>;
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
impl From<crate::W<TVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TMR_VAL_A {
    #[doc = "0: Invalid load value in compare mode"]
    TMR_VAL_0 = 0,
    #[doc = "1: Invalid load value in compare mode"]
    TMR_VAL_1 = 1,
    #[doc = "2: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_2 = 2,
    #[doc = "3: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_3 = 3,
    #[doc = "4: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_4 = 4,
    #[doc = "5: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_5 = 5,
    #[doc = "6: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_6 = 6,
    #[doc = "7: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_7 = 7,
    #[doc = "8: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_8 = 8,
    #[doc = "9: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    TMR_VAL_9 = 9,
}
impl From<TMR_VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: TMR_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR_VAL` reader - Timer Value"]
pub struct TMR_VAL_R(crate::FieldReader<u32, TMR_VAL_A>);
impl TMR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TMR_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR_VAL_A> {
        match self.bits {
            0 => Some(TMR_VAL_A::TMR_VAL_0),
            1 => Some(TMR_VAL_A::TMR_VAL_1),
            2 => Some(TMR_VAL_A::TMR_VAL_2),
            3 => Some(TMR_VAL_A::TMR_VAL_3),
            4 => Some(TMR_VAL_A::TMR_VAL_4),
            5 => Some(TMR_VAL_A::TMR_VAL_5),
            6 => Some(TMR_VAL_A::TMR_VAL_6),
            7 => Some(TMR_VAL_A::TMR_VAL_7),
            8 => Some(TMR_VAL_A::TMR_VAL_8),
            9 => Some(TMR_VAL_A::TMR_VAL_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_0`"]
    #[inline(always)]
    pub fn is_tmr_val_0(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_0
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_1`"]
    #[inline(always)]
    pub fn is_tmr_val_1(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_1
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_2`"]
    #[inline(always)]
    pub fn is_tmr_val_2(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_2
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_3`"]
    #[inline(always)]
    pub fn is_tmr_val_3(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_3
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_4`"]
    #[inline(always)]
    pub fn is_tmr_val_4(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_4
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_5`"]
    #[inline(always)]
    pub fn is_tmr_val_5(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_5
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_6`"]
    #[inline(always)]
    pub fn is_tmr_val_6(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_6
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_7`"]
    #[inline(always)]
    pub fn is_tmr_val_7(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_7
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_8`"]
    #[inline(always)]
    pub fn is_tmr_val_8(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_8
    }
    #[doc = "Checks if the value of the field is `TMR_VAL_9`"]
    #[inline(always)]
    pub fn is_tmr_val_9(&self) -> bool {
        **self == TMR_VAL_A::TMR_VAL_9
    }
}
impl core::ops::Deref for TMR_VAL_R {
    type Target = crate::FieldReader<u32, TMR_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_VAL` writer - Timer Value"]
pub struct TMR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Invalid load value in compare mode"]
    #[inline(always)]
    pub fn tmr_val_0(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_0)
    }
    #[doc = "Invalid load value in compare mode"]
    #[inline(always)]
    pub fn tmr_val_1(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_1)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_2(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_2)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_3(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_3)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_4(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_4)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_5(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_5)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_6(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_6)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_7(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_7)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_8(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_8)
    }
    #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
    #[inline(always)]
    pub fn tmr_val_9(self) -> &'a mut W {
        self.variant(TMR_VAL_A::TMR_VAL_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&self) -> TMR_VAL_R {
        TMR_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&mut self) -> TMR_VAL_W {
        TMR_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval](index.html) module"]
pub struct TVAL_SPEC;
impl crate::RegisterSpec for TVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tval::R](R) reader structure"]
impl crate::Readable for TVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tval::W](W) writer structure"]
impl crate::Writable for TVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TVAL to value 0"]
impl crate::Resettable for TVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
