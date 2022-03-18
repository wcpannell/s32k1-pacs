#[doc = "Register `FERSTAT` reader"]
pub struct R(crate::R<FERSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FERSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FERSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FERSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FERSTAT` writer"]
pub struct W(crate::W<FERSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERSTAT_SPEC>;
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
impl From<crate::W<FERSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FERSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIF_A {
    #[doc = "0: Double bit fault not detected during a valid flash read access from the platform flash controller"]
    _0 = 0,
    #[doc = "1: Double bit fault detected (or FERCNFG\\[FDFD\\]
is set) during a valid flash read access from the platform flash controller"]
    _1 = 1,
}
impl From<DFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIF` reader - Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_R(crate::FieldReader<bool, DFDIF_A>);
impl DFDIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIF_A {
        match self.bits {
            false => DFDIF_A::_0,
            true => DFDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFDIF_A::_1
    }
}
impl core::ops::Deref for DFDIF_R {
    type Target = crate::FieldReader<bool, DFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFDIF` writer - Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Double bit fault not detected during a valid flash read access from the platform flash controller"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIF_A::_0)
    }
    #[doc = "Double bit fault detected (or FERCNFG\\[FDFD\\]
is set) during a valid flash read access from the platform flash controller"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&self) -> DFDIF_R {
        DFDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&mut self) -> DFDIF_W {
        DFDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ferstat](index.html) module"]
pub struct FERSTAT_SPEC;
impl crate::RegisterSpec for FERSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ferstat::R](R) reader structure"]
impl crate::Readable for FERSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ferstat::W](W) writer structure"]
impl crate::Writable for FERSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERSTAT to value 0"]
impl crate::Resettable for FERSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
