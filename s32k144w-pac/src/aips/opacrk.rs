#[doc = "Register `OPACRK` reader"]
pub struct R(crate::R<OPACRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACRK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACRK` writer"]
pub struct W(crate::W<OPACRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACRK_SPEC>;
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
impl From<crate::W<OPACRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP3_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP3_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP3_1 = 1,
}
impl From<TP3_A> for bool {
    #[inline(always)]
    fn from(variant: TP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP3` reader - Trusted Protect"]
pub struct TP3_R(crate::FieldReader<bool, TP3_A>);
impl TP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP3_A {
        match self.bits {
            false => TP3_A::TP3_0,
            true => TP3_A::TP3_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP3_0`"]
    #[inline(always)]
    pub fn is_tp3_0(&self) -> bool {
        **self == TP3_A::TP3_0
    }
    #[doc = "Checks if the value of the field is `TP3_1`"]
    #[inline(always)]
    pub fn is_tp3_1(&self) -> bool {
        **self == TP3_A::TP3_1
    }
}
impl core::ops::Deref for TP3_R {
    type Target = crate::FieldReader<bool, TP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP3` writer - Trusted Protect"]
pub struct TP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp3_0(self) -> &'a mut W {
        self.variant(TP3_A::TP3_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp3_1(self) -> &'a mut W {
        self.variant(TP3_A::TP3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP3_A {
    #[doc = "0: This peripheral allows write accesses."]
    WP3_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP3_1 = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Write Protect"]
pub struct WP3_R(crate::FieldReader<bool, WP3_A>);
impl WP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::WP3_0,
            true => WP3_A::WP3_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP3_0`"]
    #[inline(always)]
    pub fn is_wp3_0(&self) -> bool {
        **self == WP3_A::WP3_0
    }
    #[doc = "Checks if the value of the field is `WP3_1`"]
    #[inline(always)]
    pub fn is_wp3_1(&self) -> bool {
        **self == WP3_A::WP3_1
    }
}
impl core::ops::Deref for WP3_R {
    type Target = crate::FieldReader<bool, WP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP3` writer - Write Protect"]
pub struct WP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn wp3_0(self) -> &'a mut W {
        self.variant(WP3_A::WP3_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp3_1(self) -> &'a mut W {
        self.variant(WP3_A::WP3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP3_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    SP3_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP3_1 = 1,
}
impl From<SP3_A> for bool {
    #[inline(always)]
    fn from(variant: SP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP3` reader - Supervisor Protect"]
pub struct SP3_R(crate::FieldReader<bool, SP3_A>);
impl SP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP3_A {
        match self.bits {
            false => SP3_A::SP3_0,
            true => SP3_A::SP3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP3_0`"]
    #[inline(always)]
    pub fn is_sp3_0(&self) -> bool {
        **self == SP3_A::SP3_0
    }
    #[doc = "Checks if the value of the field is `SP3_1`"]
    #[inline(always)]
    pub fn is_sp3_1(&self) -> bool {
        **self == SP3_A::SP3_1
    }
}
impl core::ops::Deref for SP3_R {
    type Target = crate::FieldReader<bool, SP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP3` writer - Supervisor Protect"]
pub struct SP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp3_0(self) -> &'a mut W {
        self.variant(SP3_A::SP3_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp3_1(self) -> &'a mut W {
        self.variant(SP3_A::SP3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    pub fn tp3(&self) -> TP3_R {
        TP3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp3(&self) -> SP3_R {
        SP3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    pub fn tp3(&mut self) -> TP3_W {
        TP3_W { w: self }
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W {
        WP3_W { w: self }
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp3(&mut self) -> SP3_W {
        SP3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrk](index.html) module"]
pub struct OPACRK_SPEC;
impl crate::RegisterSpec for OPACRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacrk::R](R) reader structure"]
impl crate::Readable for OPACRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacrk::W](W) writer structure"]
impl crate::Writable for OPACRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACRK to value 0x0004_0000"]
impl crate::Resettable for OPACRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0000
    }
}
