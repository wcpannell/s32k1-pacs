#[doc = "Register `LMPECR` reader"]
pub struct R(crate::R<LMPECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMPECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMPECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMPECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMPECR` writer"]
pub struct W(crate::W<LMPECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMPECR_SPEC>;
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
impl From<crate::W<LMPECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMPECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable RAM ECC Noncorrectable Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERNCR_A {
    #[doc = "0: Reporting disabled"]
    ERNCR_0 = 0,
    #[doc = "1: Reporting enabled"]
    ERNCR_1 = 1,
}
impl From<ERNCR_A> for bool {
    #[inline(always)]
    fn from(variant: ERNCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERNCR` reader - Enable RAM ECC Noncorrectable Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
pub struct ERNCR_R(crate::FieldReader<bool, ERNCR_A>);
impl ERNCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERNCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERNCR_A {
        match self.bits {
            false => ERNCR_A::ERNCR_0,
            true => ERNCR_A::ERNCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERNCR_0`"]
    #[inline(always)]
    pub fn is_erncr_0(&self) -> bool {
        **self == ERNCR_A::ERNCR_0
    }
    #[doc = "Checks if the value of the field is `ERNCR_1`"]
    #[inline(always)]
    pub fn is_erncr_1(&self) -> bool {
        **self == ERNCR_A::ERNCR_1
    }
}
impl core::ops::Deref for ERNCR_R {
    type Target = crate::FieldReader<bool, ERNCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERNCR` writer - Enable RAM ECC Noncorrectable Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
pub struct ERNCR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERNCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERNCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn erncr_0(self) -> &'a mut W {
        self.variant(ERNCR_A::ERNCR_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn erncr_1(self) -> &'a mut W {
        self.variant(ERNCR_A::ERNCR_1)
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
#[doc = "Enable RAM ECC 1 Bit Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER1BR_A {
    #[doc = "0: Reporting disabled"]
    ER1BR_0 = 0,
    #[doc = "1: Reporting enabled"]
    ER1BR_1 = 1,
}
impl From<ER1BR_A> for bool {
    #[inline(always)]
    fn from(variant: ER1BR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ER1BR` reader - Enable RAM ECC 1 Bit Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
pub struct ER1BR_R(crate::FieldReader<bool, ER1BR_A>);
impl ER1BR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ER1BR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ER1BR_A {
        match self.bits {
            false => ER1BR_A::ER1BR_0,
            true => ER1BR_A::ER1BR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ER1BR_0`"]
    #[inline(always)]
    pub fn is_er1br_0(&self) -> bool {
        **self == ER1BR_A::ER1BR_0
    }
    #[doc = "Checks if the value of the field is `ER1BR_1`"]
    #[inline(always)]
    pub fn is_er1br_1(&self) -> bool {
        **self == ER1BR_A::ER1BR_1
    }
}
impl core::ops::Deref for ER1BR_R {
    type Target = crate::FieldReader<bool, ER1BR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ER1BR` writer - Enable RAM ECC 1 Bit Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
pub struct ER1BR_W<'a> {
    w: &'a mut W,
}
impl<'a> ER1BR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ER1BR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn er1br_0(self) -> &'a mut W {
        self.variant(ER1BR_A::ER1BR_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn er1br_1(self) -> &'a mut W {
        self.variant(ER1BR_A::ER1BR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable Cache Parity Reporting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECPR_A {
    #[doc = "0: Reporting disabled"]
    ECPR_0 = 0,
    #[doc = "1: Reporting enabled"]
    ECPR_1 = 1,
}
impl From<ECPR_A> for bool {
    #[inline(always)]
    fn from(variant: ECPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECPR` reader - Enable Cache Parity Reporting"]
pub struct ECPR_R(crate::FieldReader<bool, ECPR_A>);
impl ECPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECPR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECPR_A {
        match self.bits {
            false => ECPR_A::ECPR_0,
            true => ECPR_A::ECPR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECPR_0`"]
    #[inline(always)]
    pub fn is_ecpr_0(&self) -> bool {
        **self == ECPR_A::ECPR_0
    }
    #[doc = "Checks if the value of the field is `ECPR_1`"]
    #[inline(always)]
    pub fn is_ecpr_1(&self) -> bool {
        **self == ECPR_A::ECPR_1
    }
}
impl core::ops::Deref for ECPR_R {
    type Target = crate::FieldReader<bool, ECPR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPR` writer - Enable Cache Parity Reporting"]
pub struct ECPR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECPR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn ecpr_0(self) -> &'a mut W {
        self.variant(ECPR_A::ECPR_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn ecpr_1(self) -> &'a mut W {
        self.variant(ECPR_A::ECPR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
    #[inline(always)]
    pub fn erncr(&self) -> ERNCR_R {
        ERNCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
    #[inline(always)]
    pub fn er1br(&self) -> ER1BR_R {
        ER1BR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline(always)]
    pub fn ecpr(&self) -> ECPR_R {
        ECPR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
    #[inline(always)]
    pub fn erncr(&mut self) -> ERNCR_W {
        ERNCR_W { w: self }
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting This bit field is Reserved and Read-Only 0 for S32K11x variants. This bit field cannot mask ECC reporting, as a result the ECC would always be reported."]
    #[inline(always)]
    pub fn er1br(&mut self) -> ER1BR_W {
        ER1BR_W { w: self }
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline(always)]
    pub fn ecpr(&mut self) -> ECPR_W {
        ECPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LMEM Parity and ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmpecr](index.html) module"]
pub struct LMPECR_SPEC;
impl crate::RegisterSpec for LMPECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmpecr::R](R) reader structure"]
impl crate::Readable for LMPECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmpecr::W](W) writer structure"]
impl crate::Writable for LMPECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMPECR to value 0"]
impl crate::Resettable for LMPECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
