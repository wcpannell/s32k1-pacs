#[doc = "Register `PLASC` reader"]
pub struct R(crate::R<PLASC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLASC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLASC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLASC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASC_A {
    #[doc = "0: A bus slave connection to AXBS input port n is absent"]
    _0 = 0,
    #[doc = "1: A bus slave connection to AXBS input port n is present"]
    _1 = 1,
}
impl From<ASC_A> for u8 {
    #[inline(always)]
    fn from(variant: ASC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ASC` reader - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
pub struct ASC_R(crate::FieldReader<u8, ASC_A>);
impl ASC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ASC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASC_A> {
        match self.bits {
            0 => Some(ASC_A::_0),
            1 => Some(ASC_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ASC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ASC_A::_1
    }
}
impl core::ops::Deref for ASC_R {
    type Target = crate::FieldReader<u8, ASC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plasc](index.html) module"]
pub struct PLASC_SPEC;
impl crate::RegisterSpec for PLASC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [plasc::R](R) reader structure"]
impl crate::Readable for PLASC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLASC to value 0x07"]
impl crate::Resettable for PLASC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
