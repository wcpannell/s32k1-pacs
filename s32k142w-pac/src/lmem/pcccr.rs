#[doc = "Register `PCCCR` reader"]
pub struct R(crate::R<PCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCCR` writer"]
pub struct W(crate::W<PCCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCCR_SPEC>;
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
impl From<crate::W<PCCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCACHE_A {
    #[doc = "0: Cache disabled"]
    ENCACHE_0 = 0,
    #[doc = "1: Cache enabled"]
    ENCACHE_1 = 1,
}
impl From<ENCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: ENCACHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENCACHE` reader - Cache enable"]
pub struct ENCACHE_R(crate::FieldReader<bool, ENCACHE_A>);
impl ENCACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCACHE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCACHE_A {
        match self.bits {
            false => ENCACHE_A::ENCACHE_0,
            true => ENCACHE_A::ENCACHE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENCACHE_0`"]
    #[inline(always)]
    pub fn is_encache_0(&self) -> bool {
        **self == ENCACHE_A::ENCACHE_0
    }
    #[doc = "Checks if the value of the field is `ENCACHE_1`"]
    #[inline(always)]
    pub fn is_encache_1(&self) -> bool {
        **self == ENCACHE_A::ENCACHE_1
    }
}
impl core::ops::Deref for ENCACHE_R {
    type Target = crate::FieldReader<bool, ENCACHE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCACHE` writer - Cache enable"]
pub struct ENCACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCACHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCACHE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cache disabled"]
    #[inline(always)]
    pub fn encache_0(self) -> &'a mut W {
        self.variant(ENCACHE_A::ENCACHE_0)
    }
    #[doc = "Cache enabled"]
    #[inline(always)]
    pub fn encache_1(self) -> &'a mut W {
        self.variant(ENCACHE_A::ENCACHE_1)
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
#[doc = "Field `PCCR2` reader - Forces all cacheable spaces to write through"]
pub struct PCCR2_R(crate::FieldReader<bool, bool>);
impl PCCR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCCR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCCR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCCR2` writer - Forces all cacheable spaces to write through"]
pub struct PCCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PCCR3` reader - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
pub struct PCCR3_R(crate::FieldReader<bool, bool>);
impl PCCR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCCR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCCR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCCR3` writer - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
pub struct PCCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCR3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Invalidate Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW0_A {
    #[doc = "0: No operation"]
    INVW0_0 = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 0."]
    INVW0_1 = 1,
}
impl From<INVW0_A> for bool {
    #[inline(always)]
    fn from(variant: INVW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVW0` reader - Invalidate Way 0"]
pub struct INVW0_R(crate::FieldReader<bool, INVW0_A>);
impl INVW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVW0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW0_A {
        match self.bits {
            false => INVW0_A::INVW0_0,
            true => INVW0_A::INVW0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVW0_0`"]
    #[inline(always)]
    pub fn is_invw0_0(&self) -> bool {
        **self == INVW0_A::INVW0_0
    }
    #[doc = "Checks if the value of the field is `INVW0_1`"]
    #[inline(always)]
    pub fn is_invw0_1(&self) -> bool {
        **self == INVW0_A::INVW0_1
    }
}
impl core::ops::Deref for INVW0_R {
    type Target = crate::FieldReader<bool, INVW0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVW0` writer - Invalidate Way 0"]
pub struct INVW0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn invw0_0(self) -> &'a mut W {
        self.variant(INVW0_A::INVW0_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline(always)]
    pub fn invw0_1(self) -> &'a mut W {
        self.variant(INVW0_A::INVW0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Push Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW0_A {
    #[doc = "0: No operation"]
    PUSHW0_0 = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 0"]
    PUSHW0_1 = 1,
}
impl From<PUSHW0_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSHW0` reader - Push Way 0"]
pub struct PUSHW0_R(crate::FieldReader<bool, PUSHW0_A>);
impl PUSHW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUSHW0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW0_A {
        match self.bits {
            false => PUSHW0_A::PUSHW0_0,
            true => PUSHW0_A::PUSHW0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHW0_0`"]
    #[inline(always)]
    pub fn is_pushw0_0(&self) -> bool {
        **self == PUSHW0_A::PUSHW0_0
    }
    #[doc = "Checks if the value of the field is `PUSHW0_1`"]
    #[inline(always)]
    pub fn is_pushw0_1(&self) -> bool {
        **self == PUSHW0_A::PUSHW0_1
    }
}
impl core::ops::Deref for PUSHW0_R {
    type Target = crate::FieldReader<bool, PUSHW0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUSHW0` writer - Push Way 0"]
pub struct PUSHW0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSHW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUSHW0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn pushw0_0(self) -> &'a mut W {
        self.variant(PUSHW0_A::PUSHW0_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline(always)]
    pub fn pushw0_1(self) -> &'a mut W {
        self.variant(PUSHW0_A::PUSHW0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Invalidate Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW1_A {
    #[doc = "0: No operation"]
    INVW1_0 = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 1"]
    INVW1_1 = 1,
}
impl From<INVW1_A> for bool {
    #[inline(always)]
    fn from(variant: INVW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVW1` reader - Invalidate Way 1"]
pub struct INVW1_R(crate::FieldReader<bool, INVW1_A>);
impl INVW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVW1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW1_A {
        match self.bits {
            false => INVW1_A::INVW1_0,
            true => INVW1_A::INVW1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVW1_0`"]
    #[inline(always)]
    pub fn is_invw1_0(&self) -> bool {
        **self == INVW1_A::INVW1_0
    }
    #[doc = "Checks if the value of the field is `INVW1_1`"]
    #[inline(always)]
    pub fn is_invw1_1(&self) -> bool {
        **self == INVW1_A::INVW1_1
    }
}
impl core::ops::Deref for INVW1_R {
    type Target = crate::FieldReader<bool, INVW1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVW1` writer - Invalidate Way 1"]
pub struct INVW1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn invw1_0(self) -> &'a mut W {
        self.variant(INVW1_A::INVW1_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline(always)]
    pub fn invw1_1(self) -> &'a mut W {
        self.variant(INVW1_A::INVW1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Push Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW1_A {
    #[doc = "0: No operation"]
    PUSHW1_0 = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 1"]
    PUSHW1_1 = 1,
}
impl From<PUSHW1_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSHW1` reader - Push Way 1"]
pub struct PUSHW1_R(crate::FieldReader<bool, PUSHW1_A>);
impl PUSHW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUSHW1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW1_A {
        match self.bits {
            false => PUSHW1_A::PUSHW1_0,
            true => PUSHW1_A::PUSHW1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHW1_0`"]
    #[inline(always)]
    pub fn is_pushw1_0(&self) -> bool {
        **self == PUSHW1_A::PUSHW1_0
    }
    #[doc = "Checks if the value of the field is `PUSHW1_1`"]
    #[inline(always)]
    pub fn is_pushw1_1(&self) -> bool {
        **self == PUSHW1_A::PUSHW1_1
    }
}
impl core::ops::Deref for PUSHW1_R {
    type Target = crate::FieldReader<bool, PUSHW1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUSHW1` writer - Push Way 1"]
pub struct PUSHW1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSHW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUSHW1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn pushw1_0(self) -> &'a mut W {
        self.variant(PUSHW1_A::PUSHW1_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline(always)]
    pub fn pushw1_1(self) -> &'a mut W {
        self.variant(PUSHW1_A::PUSHW1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Initiate Cache Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GO_A {
    #[doc = "0: Write: no effect. Read: no cache command active."]
    GO_0 = 0,
    #[doc = "1: Write: initiate command indicated by bits 27-24. Read: cache command active."]
    GO_1 = 1,
}
impl From<GO_A> for bool {
    #[inline(always)]
    fn from(variant: GO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GO` reader - Initiate Cache Command"]
pub struct GO_R(crate::FieldReader<bool, GO_A>);
impl GO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GO_A {
        match self.bits {
            false => GO_A::GO_0,
            true => GO_A::GO_1,
        }
    }
    #[doc = "Checks if the value of the field is `GO_0`"]
    #[inline(always)]
    pub fn is_go_0(&self) -> bool {
        **self == GO_A::GO_0
    }
    #[doc = "Checks if the value of the field is `GO_1`"]
    #[inline(always)]
    pub fn is_go_1(&self) -> bool {
        **self == GO_A::GO_1
    }
}
impl core::ops::Deref for GO_R {
    type Target = crate::FieldReader<bool, GO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO` writer - Initiate Cache Command"]
pub struct GO_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline(always)]
    pub fn go_0(self) -> &'a mut W {
        self.variant(GO_A::GO_0)
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline(always)]
    pub fn go_1(self) -> &'a mut W {
        self.variant(GO_A::GO_1)
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&self) -> ENCACHE_R {
        ENCACHE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline(always)]
    pub fn pccr2(&self) -> PCCR2_R {
        PCCR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline(always)]
    pub fn pccr3(&self) -> PCCR3_R {
        PCCR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&self) -> INVW0_R {
        INVW0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&self) -> PUSHW0_R {
        PUSHW0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&self) -> INVW1_R {
        INVW1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&self) -> PUSHW1_R {
        PUSHW1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&mut self) -> ENCACHE_W {
        ENCACHE_W { w: self }
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline(always)]
    pub fn pccr2(&mut self) -> PCCR2_W {
        PCCR2_W { w: self }
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline(always)]
    pub fn pccr3(&mut self) -> PCCR3_W {
        PCCR3_W { w: self }
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&mut self) -> INVW0_W {
        INVW0_W { w: self }
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&mut self) -> PUSHW0_W {
        PUSHW0_W { w: self }
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&mut self) -> INVW1_W {
        INVW1_W { w: self }
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&mut self) -> PUSHW1_W {
        PUSHW1_W { w: self }
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&mut self) -> GO_W {
        GO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcccr](index.html) module"]
pub struct PCCCR_SPEC;
impl crate::RegisterSpec for PCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcccr::R](R) reader structure"]
impl crate::Readable for PCCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcccr::W](W) writer structure"]
impl crate::Writable for PCCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCCCR to value 0"]
impl crate::Resettable for PCCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
