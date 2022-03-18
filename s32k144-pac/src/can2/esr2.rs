#[doc = "Register `ESR2` reader"]
pub struct R(crate::R<ESR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Inactive Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMB_A {
    #[doc = "0: If ESR2\\[VPS\\]
is asserted, the ESR2\\[LPTM\\]
is not an inactive Mailbox."]
    _0 = 0,
    #[doc = "1: If ESR2\\[VPS\\]
is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    _1 = 1,
}
impl From<IMB_A> for bool {
    #[inline(always)]
    fn from(variant: IMB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMB` reader - Inactive Mailbox"]
pub struct IMB_R(crate::FieldReader<bool, IMB_A>);
impl IMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IMB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMB_A {
        match self.bits {
            false => IMB_A::_0,
            true => IMB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMB_A::_1
    }
}
impl core::ops::Deref for IMB_R {
    type Target = crate::FieldReader<bool, IMB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Valid Priority Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPS_A {
    #[doc = "0: Contents of IMB and LPTM are invalid."]
    _0 = 0,
    #[doc = "1: Contents of IMB and LPTM are valid."]
    _1 = 1,
}
impl From<VPS_A> for bool {
    #[inline(always)]
    fn from(variant: VPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPS` reader - Valid Priority Status"]
pub struct VPS_R(crate::FieldReader<bool, VPS_A>);
impl VPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_A {
        match self.bits {
            false => VPS_A::_0,
            true => VPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VPS_A::_1
    }
}
impl core::ops::Deref for VPS_R {
    type Target = crate::FieldReader<bool, VPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTM` reader - Lowest Priority Tx Mailbox"]
pub struct LPTM_R(crate::FieldReader<u8, u8>);
impl LPTM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 13 - Inactive Mailbox"]
    #[inline(always)]
    pub fn imb(&self) -> IMB_R {
        IMB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Valid Priority Status"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Lowest Priority Tx Mailbox"]
    #[inline(always)]
    pub fn lptm(&self) -> LPTM_R {
        LPTM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Error and Status 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr2](index.html) module"]
pub struct ESR2_SPEC;
impl crate::RegisterSpec for ESR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr2::R](R) reader structure"]
impl crate::Readable for ESR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESR2 to value 0"]
impl crate::Resettable for ESR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
