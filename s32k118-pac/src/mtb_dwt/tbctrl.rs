#[doc = "Register `TBCTRL` reader"]
pub struct R(crate::R<TBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBCTRL` writer"]
pub struct W(crate::W<TBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBCTRL_SPEC>;
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
impl From<crate::W<TBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Action based on Comparator 0 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP0_A {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    _0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    _1 = 1,
}
impl From<ACOMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ACOMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACOMP0` reader - Action based on Comparator 0 match"]
pub struct ACOMP0_R(crate::FieldReader<bool, ACOMP0_A>);
impl ACOMP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOMP0_A {
        match self.bits {
            false => ACOMP0_A::_0,
            true => ACOMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACOMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACOMP0_A::_1
    }
}
impl core::ops::Deref for ACOMP0_R {
    type Target = crate::FieldReader<bool, ACOMP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACOMP0` writer - Action based on Comparator 0 match"]
pub struct ACOMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOMP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP0_A::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP0_A::_1)
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
#[doc = "Action based on Comparator 1 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOMP1_A {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    _0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    _1 = 1,
}
impl From<ACOMP1_A> for bool {
    #[inline(always)]
    fn from(variant: ACOMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACOMP1` reader - Action based on Comparator 1 match"]
pub struct ACOMP1_R(crate::FieldReader<bool, ACOMP1_A>);
impl ACOMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOMP1_A {
        match self.bits {
            false => ACOMP1_A::_0,
            true => ACOMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACOMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACOMP1_A::_1
    }
}
impl core::ops::Deref for ACOMP1_R {
    type Target = crate::FieldReader<bool, ACOMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACOMP1` writer - Action based on Comparator 1 match"]
pub struct ACOMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOMP1_A::_0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOMP1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `NUMCOMP` reader - Number of Comparators"]
pub struct NUMCOMP_R(crate::FieldReader<u8, u8>);
impl NUMCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMCOMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    pub fn acomp0(&self) -> ACOMP0_R {
        ACOMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    pub fn acomp1(&self) -> ACOMP1_R {
        ACOMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Number of Comparators"]
    #[inline(always)]
    pub fn numcomp(&self) -> NUMCOMP_R {
        NUMCOMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    pub fn acomp0(&mut self) -> ACOMP0_W {
        ACOMP0_W { w: self }
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    pub fn acomp1(&mut self) -> ACOMP1_W {
        ACOMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB_DWT Trace Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbctrl](index.html) module"]
pub struct TBCTRL_SPEC;
impl crate::RegisterSpec for TBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbctrl::R](R) reader structure"]
impl crate::Readable for TBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbctrl::W](W) writer structure"]
impl crate::Writable for TBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBCTRL to value 0x2000_0000"]
impl crate::Resettable for TBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
