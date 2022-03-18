#[doc = "Register `OPACRH` reader"]
pub struct R(crate::R<OPACRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACRH` writer"]
pub struct W(crate::W<OPACRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACRH_SPEC>;
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
impl From<crate::W<OPACRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP2_0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    TP2_1 = 1,
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
            false => TP2_A::TP2_0,
            true => TP2_A::TP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TP2_0`"]
    #[inline(always)]
    pub fn is_tp2_0(&self) -> bool {
        **self == TP2_A::TP2_0
    }
    #[doc = "Checks if the value of the field is `TP2_1`"]
    #[inline(always)]
    pub fn is_tp2_1(&self) -> bool {
        **self == TP2_A::TP2_1
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
    pub fn tp2_0(self) -> &'a mut W {
        self.variant(TP2_A::TP2_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn tp2_1(self) -> &'a mut W {
        self.variant(TP2_A::TP2_1)
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
    WP2_0 = 0,
    #[doc = "1: This peripheral is write protected."]
    WP2_1 = 1,
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
            false => WP2_A::WP2_0,
            true => WP2_A::WP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP2_0`"]
    #[inline(always)]
    pub fn is_wp2_0(&self) -> bool {
        **self == WP2_A::WP2_0
    }
    #[doc = "Checks if the value of the field is `WP2_1`"]
    #[inline(always)]
    pub fn is_wp2_1(&self) -> bool {
        **self == WP2_A::WP2_1
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
    pub fn wp2_0(self) -> &'a mut W {
        self.variant(WP2_A::WP2_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn wp2_1(self) -> &'a mut W {
        self.variant(WP2_A::WP2_1)
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
    SP2_0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    SP2_1 = 1,
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
            false => SP2_A::SP2_0,
            true => SP2_A::SP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SP2_0`"]
    #[inline(always)]
    pub fn is_sp2_0(&self) -> bool {
        **self == SP2_A::SP2_0
    }
    #[doc = "Checks if the value of the field is `SP2_1`"]
    #[inline(always)]
    pub fn is_sp2_1(&self) -> bool {
        **self == SP2_A::SP2_1
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
    pub fn sp2_0(self) -> &'a mut W {
        self.variant(SP2_A::SP2_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn sp2_1(self) -> &'a mut W {
        self.variant(SP2_A::SP2_1)
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
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrh](index.html) module"]
pub struct OPACRH_SPEC;
impl crate::RegisterSpec for OPACRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacrh::R](R) reader structure"]
impl crate::Readable for OPACRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacrh::W](W) writer structure"]
impl crate::Writable for OPACRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACRH to value 0x0040_0000"]
impl crate::Resettable for OPACRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0000
    }
}
