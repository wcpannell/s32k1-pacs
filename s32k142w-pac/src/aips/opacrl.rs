#[doc = "Register `OPACRL` reader"]
pub struct R(crate::R<OPACRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACRL` writer"]
pub struct W(crate::W<OPACRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACRL_SPEC>;
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
impl From<crate::W<OPACRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP7_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP7_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP7_1 = 1,
}
impl From<TP7_A> for bool {
    #[inline(always)]
    fn from(variant: TP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP7` reader - Trusted Protect"]
pub struct TP7_R(crate::FieldReader<bool, TP7_A>);
impl TP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP7_A {
        match self.bits {
            false => TP7_A::TP7_0,
            true => TP7_A::TP7_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP7_0`"]
    #[inline(always)]
    pub fn is_tp7_0(&self) -> bool {
        **self == TP7_A::TP7_0
    }
    #[doc = "Checks if the value of the field is `TP7_1`"]
    #[inline(always)]
    pub fn is_tp7_1(&self) -> bool {
        **self == TP7_A::TP7_1
    }
}
impl core::ops::Deref for TP7_R {
    type Target = crate::FieldReader<bool, TP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP7` writer - Trusted Protect"]
pub struct TP7_W<'a> {
    w: &'a mut W,
}
impl<'a> TP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp7_0(self) -> &'a mut W {
        self.variant(TP7_A::TP7_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp7_1(self) -> &'a mut W {
        self.variant(TP7_A::TP7_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP7_A {
    #[doc = "0: This peripheral allows write accesses."]
    WP7_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP7_1 = 1,
}
impl From<WP7_A> for bool {
    #[inline(always)]
    fn from(variant: WP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP7` reader - Write Protect"]
pub struct WP7_R(crate::FieldReader<bool, WP7_A>);
impl WP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP7_A {
        match self.bits {
            false => WP7_A::WP7_0,
            true => WP7_A::WP7_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP7_0`"]
    #[inline(always)]
    pub fn is_wp7_0(&self) -> bool {
        **self == WP7_A::WP7_0
    }
    #[doc = "Checks if the value of the field is `WP7_1`"]
    #[inline(always)]
    pub fn is_wp7_1(&self) -> bool {
        **self == WP7_A::WP7_1
    }
}
impl core::ops::Deref for WP7_R {
    type Target = crate::FieldReader<bool, WP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP7` writer - Write Protect"]
pub struct WP7_W<'a> {
    w: &'a mut W,
}
impl<'a> WP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn wp7_0(self) -> &'a mut W {
        self.variant(WP7_A::WP7_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp7_1(self) -> &'a mut W {
        self.variant(WP7_A::WP7_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP7_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    SP7_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP7_1 = 1,
}
impl From<SP7_A> for bool {
    #[inline(always)]
    fn from(variant: SP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP7` reader - Supervisor Protect"]
pub struct SP7_R(crate::FieldReader<bool, SP7_A>);
impl SP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP7_A {
        match self.bits {
            false => SP7_A::SP7_0,
            true => SP7_A::SP7_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP7_0`"]
    #[inline(always)]
    pub fn is_sp7_0(&self) -> bool {
        **self == SP7_A::SP7_0
    }
    #[doc = "Checks if the value of the field is `SP7_1`"]
    #[inline(always)]
    pub fn is_sp7_1(&self) -> bool {
        **self == SP7_A::SP7_1
    }
}
impl core::ops::Deref for SP7_R {
    type Target = crate::FieldReader<bool, SP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP7` writer - Supervisor Protect"]
pub struct SP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp7_0(self) -> &'a mut W {
        self.variant(SP7_A::SP7_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp7_1(self) -> &'a mut W {
        self.variant(SP7_A::SP7_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP6_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP6_1 = 1,
}
impl From<TP6_A> for bool {
    #[inline(always)]
    fn from(variant: TP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP6` reader - Trusted Protect"]
pub struct TP6_R(crate::FieldReader<bool, TP6_A>);
impl TP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP6_A {
        match self.bits {
            false => TP6_A::TP6_0,
            true => TP6_A::TP6_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP6_0`"]
    #[inline(always)]
    pub fn is_tp6_0(&self) -> bool {
        **self == TP6_A::TP6_0
    }
    #[doc = "Checks if the value of the field is `TP6_1`"]
    #[inline(always)]
    pub fn is_tp6_1(&self) -> bool {
        **self == TP6_A::TP6_1
    }
}
impl core::ops::Deref for TP6_R {
    type Target = crate::FieldReader<bool, TP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP6` writer - Trusted Protect"]
pub struct TP6_W<'a> {
    w: &'a mut W,
}
impl<'a> TP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp6_0(self) -> &'a mut W {
        self.variant(TP6_A::TP6_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp6_1(self) -> &'a mut W {
        self.variant(TP6_A::TP6_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP6_A {
    #[doc = "0: This peripheral allows write accesses."]
    WP6_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP6_1 = 1,
}
impl From<WP6_A> for bool {
    #[inline(always)]
    fn from(variant: WP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP6` reader - Write Protect"]
pub struct WP6_R(crate::FieldReader<bool, WP6_A>);
impl WP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP6_A {
        match self.bits {
            false => WP6_A::WP6_0,
            true => WP6_A::WP6_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP6_0`"]
    #[inline(always)]
    pub fn is_wp6_0(&self) -> bool {
        **self == WP6_A::WP6_0
    }
    #[doc = "Checks if the value of the field is `WP6_1`"]
    #[inline(always)]
    pub fn is_wp6_1(&self) -> bool {
        **self == WP6_A::WP6_1
    }
}
impl core::ops::Deref for WP6_R {
    type Target = crate::FieldReader<bool, WP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP6` writer - Write Protect"]
pub struct WP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn wp6_0(self) -> &'a mut W {
        self.variant(WP6_A::WP6_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp6_1(self) -> &'a mut W {
        self.variant(WP6_A::WP6_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP6_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    SP6_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP6_1 = 1,
}
impl From<SP6_A> for bool {
    #[inline(always)]
    fn from(variant: SP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP6` reader - Supervisor Protect"]
pub struct SP6_R(crate::FieldReader<bool, SP6_A>);
impl SP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP6_A {
        match self.bits {
            false => SP6_A::SP6_0,
            true => SP6_A::SP6_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP6_0`"]
    #[inline(always)]
    pub fn is_sp6_0(&self) -> bool {
        **self == SP6_A::SP6_0
    }
    #[doc = "Checks if the value of the field is `SP6_1`"]
    #[inline(always)]
    pub fn is_sp6_1(&self) -> bool {
        **self == SP6_A::SP6_1
    }
}
impl core::ops::Deref for SP6_R {
    type Target = crate::FieldReader<bool, SP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP6` writer - Supervisor Protect"]
pub struct SP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp6_0(self) -> &'a mut W {
        self.variant(SP6_A::SP6_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp6_1(self) -> &'a mut W {
        self.variant(SP6_A::SP6_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP5_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP5_1 = 1,
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
            false => TP5_A::TP5_0,
            true => TP5_A::TP5_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP5_0`"]
    #[inline(always)]
    pub fn is_tp5_0(&self) -> bool {
        **self == TP5_A::TP5_0
    }
    #[doc = "Checks if the value of the field is `TP5_1`"]
    #[inline(always)]
    pub fn is_tp5_1(&self) -> bool {
        **self == TP5_A::TP5_1
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
    pub fn tp5_0(self) -> &'a mut W {
        self.variant(TP5_A::TP5_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp5_1(self) -> &'a mut W {
        self.variant(TP5_A::TP5_1)
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
    WP5_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP5_1 = 1,
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
            false => WP5_A::WP5_0,
            true => WP5_A::WP5_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP5_0`"]
    #[inline(always)]
    pub fn is_wp5_0(&self) -> bool {
        **self == WP5_A::WP5_0
    }
    #[doc = "Checks if the value of the field is `WP5_1`"]
    #[inline(always)]
    pub fn is_wp5_1(&self) -> bool {
        **self == WP5_A::WP5_1
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
    pub fn wp5_0(self) -> &'a mut W {
        self.variant(WP5_A::WP5_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp5_1(self) -> &'a mut W {
        self.variant(WP5_A::WP5_1)
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
    SP5_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP5_1 = 1,
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
            false => SP5_A::SP5_0,
            true => SP5_A::SP5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP5_0`"]
    #[inline(always)]
    pub fn is_sp5_0(&self) -> bool {
        **self == SP5_A::SP5_0
    }
    #[doc = "Checks if the value of the field is `SP5_1`"]
    #[inline(always)]
    pub fn is_sp5_1(&self) -> bool {
        **self == SP5_A::SP5_1
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
    pub fn sp5_0(self) -> &'a mut W {
        self.variant(SP5_A::SP5_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp5_1(self) -> &'a mut W {
        self.variant(SP5_A::SP5_1)
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
impl R {
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    pub fn tp7(&self) -> TP7_R {
        TP7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    pub fn wp7(&self) -> WP7_R {
        WP7_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp7(&self) -> SP7_R {
        SP7_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&self) -> TP6_R {
        TP6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&self) -> SP6_R {
        SP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
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
}
impl W {
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    pub fn tp7(&mut self) -> TP7_W {
        TP7_W { w: self }
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    pub fn wp7(&mut self) -> WP7_W {
        WP7_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp7(&mut self) -> SP7_W {
        SP7_W { w: self }
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&mut self) -> TP6_W {
        TP6_W { w: self }
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&mut self) -> WP6_W {
        WP6_W { w: self }
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&mut self) -> SP6_W {
        SP6_W { w: self }
    }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrl](index.html) module"]
pub struct OPACRL_SPEC;
impl crate::RegisterSpec for OPACRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacrl::R](R) reader structure"]
impl crate::Readable for OPACRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacrl::W](W) writer structure"]
impl crate::Writable for OPACRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACRL to value 0x0444"]
impl crate::Resettable for OPACRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0444
    }
}
