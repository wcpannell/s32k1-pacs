#[doc = "Register `MIBC` reader"]
pub struct R(crate::R<MIBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIBC` writer"]
pub struct W(crate::W<MIBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIBC_SPEC>;
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
impl From<crate::W<MIBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MIB Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_CLEAR_A {
    #[doc = "0: See note above."]
    _0 = 0,
    #[doc = "1: All statistics counters are reset to 0."]
    _1 = 1,
}
impl From<MIB_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIB_CLEAR` reader - MIB Clear"]
pub struct MIB_CLEAR_R(crate::FieldReader<bool, MIB_CLEAR_A>);
impl MIB_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIB_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_CLEAR_A {
        match self.bits {
            false => MIB_CLEAR_A::_0,
            true => MIB_CLEAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MIB_CLEAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MIB_CLEAR_A::_1
    }
}
impl core::ops::Deref for MIB_CLEAR_R {
    type Target = crate::FieldReader<bool, MIB_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIB_CLEAR` writer - MIB Clear"]
pub struct MIB_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIB_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "See note above."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::_0)
    }
    #[doc = "All statistics counters are reset to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "MIB Idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_IDLE_A {
    #[doc = "0: The MIB block is updating MIB counters."]
    _0 = 0,
    #[doc = "1: The MIB block is not currently updating any MIB counters."]
    _1 = 1,
}
impl From<MIB_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIB_IDLE` reader - MIB Idle"]
pub struct MIB_IDLE_R(crate::FieldReader<bool, MIB_IDLE_A>);
impl MIB_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIB_IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_IDLE_A {
        match self.bits {
            false => MIB_IDLE_A::_0,
            true => MIB_IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MIB_IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MIB_IDLE_A::_1
    }
}
impl core::ops::Deref for MIB_IDLE_R {
    type Target = crate::FieldReader<bool, MIB_IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable MIB Logic\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_DIS_A {
    #[doc = "0: MIB logic is enabled."]
    _0 = 0,
    #[doc = "1: MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    _1 = 1,
}
impl From<MIB_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIB_DIS` reader - Disable MIB Logic"]
pub struct MIB_DIS_R(crate::FieldReader<bool, MIB_DIS_A>);
impl MIB_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIB_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_DIS_A {
        match self.bits {
            false => MIB_DIS_A::_0,
            true => MIB_DIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MIB_DIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MIB_DIS_A::_1
    }
}
impl core::ops::Deref for MIB_DIS_R {
    type Target = crate::FieldReader<bool, MIB_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIB_DIS` writer - Disable MIB Logic"]
pub struct MIB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIB_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MIB logic is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MIB_DIS_A::_0)
    }
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MIB_DIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&self) -> MIB_CLEAR_R {
        MIB_CLEAR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline(always)]
    pub fn mib_idle(&self) -> MIB_IDLE_R {
        MIB_IDLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&self) -> MIB_DIS_R {
        MIB_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&mut self) -> MIB_CLEAR_W {
        MIB_CLEAR_W { w: self }
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&mut self) -> MIB_DIS_W {
        MIB_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mibc](index.html) module"]
pub struct MIBC_SPEC;
impl crate::RegisterSpec for MIBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mibc::R](R) reader structure"]
impl crate::Readable for MIBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mibc::W](W) writer structure"]
impl crate::Writable for MIBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIBC to value 0xc000_0000"]
impl crate::Resettable for MIBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
