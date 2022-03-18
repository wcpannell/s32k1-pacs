#[doc = "Register `OPACRE` reader"]
pub struct R(crate::R<OPACRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACRE` writer"]
pub struct W(crate::W<OPACRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACRE_SPEC>;
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
impl From<crate::W<OPACRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACRE_SPEC>) -> Self {
        W(writer)
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
pub enum TP0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP0_1 = 1,
}
impl From<TP0_A> for bool {
    #[inline(always)]
    fn from(variant: TP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP0` reader - Trusted Protect"]
pub struct TP0_R(crate::FieldReader<bool, TP0_A>);
impl TP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP0_A {
        match self.bits {
            false => TP0_A::TP0_0,
            true => TP0_A::TP0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP0_0`"]
    #[inline(always)]
    pub fn is_tp0_0(&self) -> bool {
        **self == TP0_A::TP0_0
    }
    #[doc = "Checks if the value of the field is `TP0_1`"]
    #[inline(always)]
    pub fn is_tp0_1(&self) -> bool {
        **self == TP0_A::TP0_1
    }
}
impl core::ops::Deref for TP0_R {
    type Target = crate::FieldReader<bool, TP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP0` writer - Trusted Protect"]
pub struct TP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0_0(self) -> &'a mut W {
        self.variant(TP0_A::TP0_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp0_1(self) -> &'a mut W {
        self.variant(TP0_A::TP0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP0_A {
    #[doc = "0: This peripheral allows write accesses."]
    WP0_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP0_1 = 1,
}
impl From<WP0_A> for bool {
    #[inline(always)]
    fn from(variant: WP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP0` reader - Write Protect"]
pub struct WP0_R(crate::FieldReader<bool, WP0_A>);
impl WP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP0_A {
        match self.bits {
            false => WP0_A::WP0_0,
            true => WP0_A::WP0_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP0_0`"]
    #[inline(always)]
    pub fn is_wp0_0(&self) -> bool {
        **self == WP0_A::WP0_0
    }
    #[doc = "Checks if the value of the field is `WP0_1`"]
    #[inline(always)]
    pub fn is_wp0_1(&self) -> bool {
        **self == WP0_A::WP0_1
    }
}
impl core::ops::Deref for WP0_R {
    type Target = crate::FieldReader<bool, WP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP0` writer - Write Protect"]
pub struct WP0_W<'a> {
    w: &'a mut W,
}
impl<'a> WP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn wp0_0(self) -> &'a mut W {
        self.variant(WP0_A::WP0_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp0_1(self) -> &'a mut W {
        self.variant(WP0_A::WP0_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    SP0_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP0_1 = 1,
}
impl From<SP0_A> for bool {
    #[inline(always)]
    fn from(variant: SP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP0` reader - Supervisor Protect"]
pub struct SP0_R(crate::FieldReader<bool, SP0_A>);
impl SP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP0_A {
        match self.bits {
            false => SP0_A::SP0_0,
            true => SP0_A::SP0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP0_0`"]
    #[inline(always)]
    pub fn is_sp0_0(&self) -> bool {
        **self == SP0_A::SP0_0
    }
    #[doc = "Checks if the value of the field is `SP0_1`"]
    #[inline(always)]
    pub fn is_sp0_1(&self) -> bool {
        **self == SP0_A::SP0_1
    }
}
impl core::ops::Deref for SP0_R {
    type Target = crate::FieldReader<bool, SP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP0` writer - Supervisor Protect"]
pub struct SP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp0_0(self) -> &'a mut W {
        self.variant(SP0_A::SP0_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp0_1(self) -> &'a mut W {
        self.variant(SP0_A::SP0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
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
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&self) -> TP0_R {
        TP0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&self) -> WP0_R {
        WP0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&self) -> SP0_R {
        SP0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
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
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&mut self) -> TP0_W {
        TP0_W { w: self }
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&mut self) -> WP0_W {
        WP0_W { w: self }
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&mut self) -> SP0_W {
        SP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacre](index.html) module"]
pub struct OPACRE_SPEC;
impl crate::RegisterSpec for OPACRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacre::R](R) reader structure"]
impl crate::Readable for OPACRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacre::W](W) writer structure"]
impl crate::Writable for OPACRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACRE to value 0x4000_0040"]
impl crate::Resettable for OPACRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0040
    }
}
