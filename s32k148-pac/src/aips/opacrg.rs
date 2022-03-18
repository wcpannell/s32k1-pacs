#[doc = "Register `OPACRG` reader"]
pub struct R(crate::R<OPACRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACRG` writer"]
pub struct W(crate::W<OPACRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACRG_SPEC>;
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
impl From<crate::W<OPACRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP5_A> for bool {
    #[inline(always)]
    fn from(variant: TP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP5` reader - Trusted Protect"]
pub struct TP5_R(crate::FieldReader<bool, TP5_A>);
impl TP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP5_A {
        match self.bits {
            false => TP5_A::_0,
            true => TP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP5_A::_1
    }
}
impl core::ops::Deref for TP5_R {
    type Target = crate::FieldReader<bool, TP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP5` writer - Trusted Protect"]
pub struct TP5_W<'a> {
    w: &'a mut W,
}
impl<'a> TP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP5_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP5_A> for bool {
    #[inline(always)]
    fn from(variant: WP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP5` reader - Write Protect"]
pub struct WP5_R(crate::FieldReader<bool, WP5_A>);
impl WP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP5_A {
        match self.bits {
            false => WP5_A::_0,
            true => WP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP5_A::_1
    }
}
impl core::ops::Deref for WP5_R {
    type Target = crate::FieldReader<bool, WP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP5` writer - Write Protect"]
pub struct WP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP5_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP5_A> for bool {
    #[inline(always)]
    fn from(variant: SP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP5` reader - Supervisor Protect"]
pub struct SP5_R(crate::FieldReader<bool, SP5_A>);
impl SP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP5_A {
        match self.bits {
            false => SP5_A::_0,
            true => SP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP5_A::_1
    }
}
impl core::ops::Deref for SP5_R {
    type Target = crate::FieldReader<bool, SP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP5` writer - Supervisor Protect"]
pub struct SP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP4_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP4_A> for bool {
    #[inline(always)]
    fn from(variant: TP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP4` reader - Trusted Protect"]
pub struct TP4_R(crate::FieldReader<bool, TP4_A>);
impl TP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP4_A {
        match self.bits {
            false => TP4_A::_0,
            true => TP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP4_A::_1
    }
}
impl core::ops::Deref for TP4_R {
    type Target = crate::FieldReader<bool, TP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP4` writer - Trusted Protect"]
pub struct TP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP4_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP4_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP4_A> for bool {
    #[inline(always)]
    fn from(variant: WP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP4` reader - Write Protect"]
pub struct WP4_R(crate::FieldReader<bool, WP4_A>);
impl WP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP4_A {
        match self.bits {
            false => WP4_A::_0,
            true => WP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP4_A::_1
    }
}
impl core::ops::Deref for WP4_R {
    type Target = crate::FieldReader<bool, WP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP4` writer - Write Protect"]
pub struct WP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP4_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP4_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP4_A> for bool {
    #[inline(always)]
    fn from(variant: SP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP4` reader - Supervisor Protect"]
pub struct SP4_R(crate::FieldReader<bool, SP4_A>);
impl SP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP4_A {
        match self.bits {
            false => SP4_A::_0,
            true => SP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP4_A::_1
    }
}
impl core::ops::Deref for SP4_R {
    type Target = crate::FieldReader<bool, SP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP4` writer - Supervisor Protect"]
pub struct SP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP4_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP2_A> for bool {
    #[inline(always)]
    fn from(variant: TP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP2` reader - Trusted Protect"]
pub struct TP2_R(crate::FieldReader<bool, TP2_A>);
impl TP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP2_A {
        match self.bits {
            false => TP2_A::_0,
            true => TP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP2_A::_1
    }
}
impl core::ops::Deref for TP2_R {
    type Target = crate::FieldReader<bool, TP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP2` writer - Trusted Protect"]
pub struct TP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP2_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP2_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP2_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Write Protect"]
pub struct WP2_R(crate::FieldReader<bool, WP2_A>);
impl WP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::_0,
            true => WP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP2_A::_1
    }
}
impl core::ops::Deref for WP2_R {
    type Target = crate::FieldReader<bool, WP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP2` writer - Write Protect"]
pub struct WP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP2_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP2_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP2_A> for bool {
    #[inline(always)]
    fn from(variant: SP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP2` reader - Supervisor Protect"]
pub struct SP2_R(crate::FieldReader<bool, SP2_A>);
impl SP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP2_A {
        match self.bits {
            false => SP2_A::_0,
            true => SP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP2_A::_1
    }
}
impl core::ops::Deref for SP2_R {
    type Target = crate::FieldReader<bool, SP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP2` writer - Supervisor Protect"]
pub struct SP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP2_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&self) -> TP5_R {
        TP5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&self) -> SP5_R {
        SP5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    pub fn tp4(&self) -> TP4_R {
        TP4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp4(&self) -> SP4_R {
        SP4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    pub fn tp2(&self) -> TP2_R {
        TP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp2(&self) -> SP2_R {
        SP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&mut self) -> TP5_W {
        TP5_W { w: self }
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W {
        WP5_W { w: self }
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&mut self) -> SP5_W {
        SP5_W { w: self }
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    pub fn tp4(&mut self) -> TP4_W {
        TP4_W { w: self }
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    pub fn wp4(&mut self) -> WP4_W {
        WP4_W { w: self }
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp4(&mut self) -> SP4_W {
        SP4_W { w: self }
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    pub fn tp2(&mut self) -> TP2_W {
        TP2_W { w: self }
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W {
        WP2_W { w: self }
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp2(&mut self) -> SP2_W {
        SP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrg](index.html) module"]
pub struct OPACRG_SPEC;
impl crate::RegisterSpec for OPACRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacrg::R](R) reader structure"]
impl crate::Readable for OPACRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacrg::W](W) writer structure"]
impl crate::Writable for OPACRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACRG to value 0x0040_4400"]
impl crate::Resettable for OPACRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_4400
    }
}
