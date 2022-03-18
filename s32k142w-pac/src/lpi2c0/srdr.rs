#[doc = "Register `SRDR` reader"]
pub struct R(crate::R<SRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Receive Data"]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: The Receive Data Register is not empty"]
    RXEMPTY_0 = 0,
    #[doc = "1: The Receive Data Register is empty"]
    RXEMPTY_1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - RX Empty"]
pub struct RXEMPTY_R(crate::FieldReader<bool, RXEMPTY_A>);
impl RXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::RXEMPTY_0,
            true => RXEMPTY_A::RXEMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_0`"]
    #[inline(always)]
    pub fn is_rxempty_0(&self) -> bool {
        **self == RXEMPTY_A::RXEMPTY_0
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_1`"]
    #[inline(always)]
    pub fn is_rxempty_1(&self) -> bool {
        **self == RXEMPTY_A::RXEMPTY_1
    }
}
impl core::ops::Deref for RXEMPTY_R {
    type Target = crate::FieldReader<bool, RXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: Indicates this is not the first data word since a (repeated) START or STOP condition"]
    SOF_0 = 0,
    #[doc = "1: Indicates this is the first data word since a (repeated) START or STOP condition"]
    SOF_1 = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Start Of Frame"]
pub struct SOF_R(crate::FieldReader<bool, SOF_A>);
impl SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::SOF_0,
            true => SOF_A::SOF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_0`"]
    #[inline(always)]
    pub fn is_sof_0(&self) -> bool {
        **self == SOF_A::SOF_0
    }
    #[doc = "Checks if the value of the field is `SOF_1`"]
    #[inline(always)]
    pub fn is_sof_1(&self) -> bool {
        **self == SOF_A::SOF_1
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, SOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - RX Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Slave Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdr](index.html) module"]
pub struct SRDR_SPEC;
impl crate::RegisterSpec for SRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdr::R](R) reader structure"]
impl crate::Readable for SRDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRDR to value 0x4000"]
impl crate::Resettable for SRDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
