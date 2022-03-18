#[doc = "Register `SASR` reader"]
pub struct R(crate::R<SASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RADDR` reader - Received Address"]
pub struct RADDR_R(crate::FieldReader<u16, u16>);
impl RADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Address Not Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANV_A {
    #[doc = "0: RADDR is valid."]
    _0 = 0,
    #[doc = "1: RADDR is not valid."]
    _1 = 1,
}
impl From<ANV_A> for bool {
    #[inline(always)]
    fn from(variant: ANV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANV` reader - Address Not Valid"]
pub struct ANV_R(crate::FieldReader<bool, ANV_A>);
impl ANV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANV_A {
        match self.bits {
            false => ANV_A::_0,
            true => ANV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ANV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ANV_A::_1
    }
}
impl core::ops::Deref for ANV_R {
    type Target = crate::FieldReader<bool, ANV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Received Address"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - Address Not Valid"]
    #[inline(always)]
    pub fn anv(&self) -> ANV_R {
        ANV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
#[doc = "Slave Address Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sasr](index.html) module"]
pub struct SASR_SPEC;
impl crate::RegisterSpec for SASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sasr::R](R) reader structure"]
impl crate::Readable for SASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SASR to value 0x4000"]
impl crate::Resettable for SASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
