#[doc = "Register `PCCRMR` reader"]
pub struct R(crate::R<PCCRMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCRMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCRMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCRMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCRMR` writer"]
pub struct W(crate::W<PCCRMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCRMR_SPEC>;
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
impl From<crate::W<PCCRMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCRMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Region 15 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R15_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R15_A> for u8 {
    #[inline(always)]
    fn from(variant: R15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R15` reader - Region 15 mode"]
pub struct R15_R(crate::FieldReader<u8, R15_A>);
impl R15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R15_A {
        match self.bits {
            0 => R15_A::_00,
            1 => R15_A::_01,
            2 => R15_A::_10,
            3 => R15_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R15_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R15_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R15_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R15_A::_11
    }
}
impl core::ops::Deref for R15_R {
    type Target = crate::FieldReader<u8, R15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R15` writer - Region 15 mode"]
pub struct R15_W<'a> {
    w: &'a mut W,
}
impl<'a> R15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R15_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R15_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R15_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R15_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Region 14 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R14_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R14_A> for u8 {
    #[inline(always)]
    fn from(variant: R14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R14` reader - Region 14 mode"]
pub struct R14_R(crate::FieldReader<u8, R14_A>);
impl R14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R14_A {
        match self.bits {
            0 => R14_A::_00,
            1 => R14_A::_01,
            2 => R14_A::_10,
            3 => R14_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R14_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R14_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R14_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R14_A::_11
    }
}
impl core::ops::Deref for R14_R {
    type Target = crate::FieldReader<u8, R14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R14` writer - Region 14 mode"]
pub struct R14_W<'a> {
    w: &'a mut W,
}
impl<'a> R14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R14_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R14_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R14_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R14_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Region 13 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R13_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R13_A> for u8 {
    #[inline(always)]
    fn from(variant: R13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R13` reader - Region 13 mode"]
pub struct R13_R(crate::FieldReader<u8, R13_A>);
impl R13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R13_A {
        match self.bits {
            0 => R13_A::_00,
            1 => R13_A::_01,
            2 => R13_A::_10,
            3 => R13_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R13_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R13_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R13_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R13_A::_11
    }
}
impl core::ops::Deref for R13_R {
    type Target = crate::FieldReader<u8, R13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R13` writer - Region 13 mode"]
pub struct R13_W<'a> {
    w: &'a mut W,
}
impl<'a> R13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R13_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R13_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R13_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R13_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Region 12 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R12_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R12_A> for u8 {
    #[inline(always)]
    fn from(variant: R12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R12` reader - Region 12 mode"]
pub struct R12_R(crate::FieldReader<u8, R12_A>);
impl R12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R12_A {
        match self.bits {
            0 => R12_A::_00,
            1 => R12_A::_01,
            2 => R12_A::_10,
            3 => R12_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R12_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R12_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R12_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R12_A::_11
    }
}
impl core::ops::Deref for R12_R {
    type Target = crate::FieldReader<u8, R12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R12` writer - Region 12 mode"]
pub struct R12_W<'a> {
    w: &'a mut W,
}
impl<'a> R12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R12_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R12_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R12_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R12_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Region 11 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R11_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R11_A> for u8 {
    #[inline(always)]
    fn from(variant: R11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R11` reader - Region 11 mode"]
pub struct R11_R(crate::FieldReader<u8, R11_A>);
impl R11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R11_A {
        match self.bits {
            0 => R11_A::_00,
            1 => R11_A::_01,
            2 => R11_A::_10,
            3 => R11_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R11_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R11_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R11_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R11_A::_11
    }
}
impl core::ops::Deref for R11_R {
    type Target = crate::FieldReader<u8, R11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R11` writer - Region 11 mode"]
pub struct R11_W<'a> {
    w: &'a mut W,
}
impl<'a> R11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R11_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R11_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R11_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R11_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Region 10 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R10_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R10_A> for u8 {
    #[inline(always)]
    fn from(variant: R10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R10` reader - Region 10 mode"]
pub struct R10_R(crate::FieldReader<u8, R10_A>);
impl R10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R10_A {
        match self.bits {
            0 => R10_A::_00,
            1 => R10_A::_01,
            2 => R10_A::_10,
            3 => R10_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R10_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R10_A::_11
    }
}
impl core::ops::Deref for R10_R {
    type Target = crate::FieldReader<u8, R10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R10` writer - Region 10 mode"]
pub struct R10_W<'a> {
    w: &'a mut W,
}
impl<'a> R10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R10_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R10_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R10_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R10_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Region 9 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R9_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R9_A> for u8 {
    #[inline(always)]
    fn from(variant: R9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R9` reader - Region 9 mode"]
pub struct R9_R(crate::FieldReader<u8, R9_A>);
impl R9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R9_A {
        match self.bits {
            0 => R9_A::_00,
            1 => R9_A::_01,
            2 => R9_A::_10,
            3 => R9_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R9_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R9_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R9_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R9_A::_11
    }
}
impl core::ops::Deref for R9_R {
    type Target = crate::FieldReader<u8, R9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R9` writer - Region 9 mode"]
pub struct R9_W<'a> {
    w: &'a mut W,
}
impl<'a> R9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R9_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R9_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R9_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R9_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Region 8 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R8_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R8_A> for u8 {
    #[inline(always)]
    fn from(variant: R8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R8` reader - Region 8 mode"]
pub struct R8_R(crate::FieldReader<u8, R8_A>);
impl R8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R8_A {
        match self.bits {
            0 => R8_A::_00,
            1 => R8_A::_01,
            2 => R8_A::_10,
            3 => R8_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R8_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R8_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R8_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R8_A::_11
    }
}
impl core::ops::Deref for R8_R {
    type Target = crate::FieldReader<u8, R8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R8` writer - Region 8 mode"]
pub struct R8_W<'a> {
    w: &'a mut W,
}
impl<'a> R8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R8_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R8_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R8_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R8_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Region 7 mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R7_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R7_A> for u8 {
    #[inline(always)]
    fn from(variant: R7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R7` reader - Region 7 mode"]
pub struct R7_R(crate::FieldReader<u8, R7_A>);
impl R7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R7_A {
        match self.bits {
            0 => R7_A::_00,
            1 => R7_A::_01,
            2 => R7_A::_10,
            3 => R7_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R7_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R7_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R7_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R7_A::_11
    }
}
impl core::ops::Deref for R7_R {
    type Target = crate::FieldReader<u8, R7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R7` writer - Region 7 mode"]
pub struct R7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R7_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R7_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R7_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R7_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Region 6 mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R6_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R6_A> for u8 {
    #[inline(always)]
    fn from(variant: R6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R6` reader - Region 6 mode"]
pub struct R6_R(crate::FieldReader<u8, R6_A>);
impl R6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R6_A {
        match self.bits {
            0 => R6_A::_00,
            1 => R6_A::_01,
            2 => R6_A::_10,
            3 => R6_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R6_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R6_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R6_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R6_A::_11
    }
}
impl core::ops::Deref for R6_R {
    type Target = crate::FieldReader<u8, R6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R6` writer - Region 6 mode"]
pub struct R6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R6_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R6_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R6_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R6_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Region 5 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R5_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R5_A> for u8 {
    #[inline(always)]
    fn from(variant: R5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R5` reader - Region 5 mode"]
pub struct R5_R(crate::FieldReader<u8, R5_A>);
impl R5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R5_A {
        match self.bits {
            0 => R5_A::_00,
            1 => R5_A::_01,
            2 => R5_A::_10,
            3 => R5_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R5_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R5_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R5_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R5_A::_11
    }
}
impl core::ops::Deref for R5_R {
    type Target = crate::FieldReader<u8, R5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R5` writer - Region 5 mode"]
pub struct R5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R5_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R5_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R5_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R5_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Region 4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R4_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R4_A> for u8 {
    #[inline(always)]
    fn from(variant: R4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R4` reader - Region 4 mode"]
pub struct R4_R(crate::FieldReader<u8, R4_A>);
impl R4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R4_A {
        match self.bits {
            0 => R4_A::_00,
            1 => R4_A::_01,
            2 => R4_A::_10,
            3 => R4_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R4_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R4_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R4_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R4_A::_11
    }
}
impl core::ops::Deref for R4_R {
    type Target = crate::FieldReader<u8, R4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R4` writer - Region 4 mode"]
pub struct R4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R4_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R4_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R4_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R4_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Region 3 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R3_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R3_A> for u8 {
    #[inline(always)]
    fn from(variant: R3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R3` reader - Region 3 mode"]
pub struct R3_R(crate::FieldReader<u8, R3_A>);
impl R3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R3_A {
        match self.bits {
            0 => R3_A::_00,
            1 => R3_A::_01,
            2 => R3_A::_10,
            3 => R3_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R3_A::_11
    }
}
impl core::ops::Deref for R3_R {
    type Target = crate::FieldReader<u8, R3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R3` writer - Region 3 mode"]
pub struct R3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R3_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R3_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R3_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R3_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Region 2 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R2_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R2_A> for u8 {
    #[inline(always)]
    fn from(variant: R2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R2` reader - Region 2 mode"]
pub struct R2_R(crate::FieldReader<u8, R2_A>);
impl R2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R2_A {
        match self.bits {
            0 => R2_A::_00,
            1 => R2_A::_01,
            2 => R2_A::_10,
            3 => R2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R2_A::_11
    }
}
impl core::ops::Deref for R2_R {
    type Target = crate::FieldReader<u8, R2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R2` writer - Region 2 mode"]
pub struct R2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R2_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R2_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R2_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R2_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Region 1 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R1_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R1_A> for u8 {
    #[inline(always)]
    fn from(variant: R1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R1` reader - Region 1 mode"]
pub struct R1_R(crate::FieldReader<u8, R1_A>);
impl R1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R1_A {
        match self.bits {
            0 => R1_A::_00,
            1 => R1_A::_01,
            2 => R1_A::_10,
            3 => R1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R1_A::_11
    }
}
impl core::ops::Deref for R1_R {
    type Target = crate::FieldReader<u8, R1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R1` writer - Region 1 mode"]
pub struct R1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R1_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R1_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R1_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Region 0 mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R0_A {
    #[doc = "0: Non-cacheable"]
    _00 = 0,
    #[doc = "1: Non-cacheable"]
    _01 = 1,
    #[doc = "2: Write-through"]
    _10 = 2,
    #[doc = "3: Write-back"]
    _11 = 3,
}
impl From<R0_A> for u8 {
    #[inline(always)]
    fn from(variant: R0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R0` reader - Region 0 mode"]
pub struct R0_R(crate::FieldReader<u8, R0_A>);
impl R0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R0_A {
        match self.bits {
            0 => R0_A::_00,
            1 => R0_A::_01,
            2 => R0_A::_10,
            3 => R0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == R0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == R0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == R0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == R0_A::_11
    }
}
impl core::ops::Deref for R0_R {
    type Target = crate::FieldReader<u8, R0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0` writer - Region 0 mode"]
pub struct R0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: R0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(R0_A::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(R0_A::_01)
    }
    #[doc = "Write-through"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(R0_A::_10)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(R0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Region 15 mode"]
    #[inline(always)]
    pub fn r15(&self) -> R15_R {
        R15_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Region 14 mode"]
    #[inline(always)]
    pub fn r14(&self) -> R14_R {
        R14_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Region 13 mode"]
    #[inline(always)]
    pub fn r13(&self) -> R13_R {
        R13_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Region 12 mode"]
    #[inline(always)]
    pub fn r12(&self) -> R12_R {
        R12_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Region 11 mode"]
    #[inline(always)]
    pub fn r11(&self) -> R11_R {
        R11_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Region 10 mode"]
    #[inline(always)]
    pub fn r10(&self) -> R10_R {
        R10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Region 9 mode"]
    #[inline(always)]
    pub fn r9(&self) -> R9_R {
        R9_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Region 8 mode"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Region 7 mode"]
    #[inline(always)]
    pub fn r7(&self) -> R7_R {
        R7_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Region 6 mode"]
    #[inline(always)]
    pub fn r6(&self) -> R6_R {
        R6_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Region 5 mode"]
    #[inline(always)]
    pub fn r5(&self) -> R5_R {
        R5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Region 4 mode"]
    #[inline(always)]
    pub fn r4(&self) -> R4_R {
        R4_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Region 3 mode"]
    #[inline(always)]
    pub fn r3(&self) -> R3_R {
        R3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Region 2 mode"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Region 1 mode"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Region 0 mode"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 15 mode"]
    #[inline(always)]
    pub fn r15(&mut self) -> R15_W {
        R15_W { w: self }
    }
    #[doc = "Bits 2:3 - Region 14 mode"]
    #[inline(always)]
    pub fn r14(&mut self) -> R14_W {
        R14_W { w: self }
    }
    #[doc = "Bits 4:5 - Region 13 mode"]
    #[inline(always)]
    pub fn r13(&mut self) -> R13_W {
        R13_W { w: self }
    }
    #[doc = "Bits 6:7 - Region 12 mode"]
    #[inline(always)]
    pub fn r12(&mut self) -> R12_W {
        R12_W { w: self }
    }
    #[doc = "Bits 8:9 - Region 11 mode"]
    #[inline(always)]
    pub fn r11(&mut self) -> R11_W {
        R11_W { w: self }
    }
    #[doc = "Bits 10:11 - Region 10 mode"]
    #[inline(always)]
    pub fn r10(&mut self) -> R10_W {
        R10_W { w: self }
    }
    #[doc = "Bits 12:13 - Region 9 mode"]
    #[inline(always)]
    pub fn r9(&mut self) -> R9_W {
        R9_W { w: self }
    }
    #[doc = "Bits 14:15 - Region 8 mode"]
    #[inline(always)]
    pub fn r8(&mut self) -> R8_W {
        R8_W { w: self }
    }
    #[doc = "Bits 16:17 - Region 7 mode"]
    #[inline(always)]
    pub fn r7(&mut self) -> R7_W {
        R7_W { w: self }
    }
    #[doc = "Bits 18:19 - Region 6 mode"]
    #[inline(always)]
    pub fn r6(&mut self) -> R6_W {
        R6_W { w: self }
    }
    #[doc = "Bits 20:21 - Region 5 mode"]
    #[inline(always)]
    pub fn r5(&mut self) -> R5_W {
        R5_W { w: self }
    }
    #[doc = "Bits 22:23 - Region 4 mode"]
    #[inline(always)]
    pub fn r4(&mut self) -> R4_W {
        R4_W { w: self }
    }
    #[doc = "Bits 24:25 - Region 3 mode"]
    #[inline(always)]
    pub fn r3(&mut self) -> R3_W {
        R3_W { w: self }
    }
    #[doc = "Bits 26:27 - Region 2 mode"]
    #[inline(always)]
    pub fn r2(&mut self) -> R2_W {
        R2_W { w: self }
    }
    #[doc = "Bits 28:29 - Region 1 mode"]
    #[inline(always)]
    pub fn r1(&mut self) -> R1_W {
        R1_W { w: self }
    }
    #[doc = "Bits 30:31 - Region 0 mode"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache regions mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccrmr](index.html) module"]
pub struct PCCRMR_SPEC;
impl crate::RegisterSpec for PCCRMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccrmr::R](R) reader structure"]
impl crate::Readable for PCCRMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccrmr::W](W) writer structure"]
impl crate::Writable for PCCRMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCCRMR to value 0xaa0f_a000"]
impl crate::Resettable for PCCRMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaa0f_a000
    }
}
