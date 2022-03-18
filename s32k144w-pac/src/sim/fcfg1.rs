#[doc = "Register `FCFG1` reader"]
pub struct R(crate::R<FCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG1` writer"]
pub struct W(crate::W<FCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG1_SPEC>;
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
impl From<crate::W<FCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEPART` reader - FlexNVM partition"]
pub struct DEPART_R(crate::FieldReader<u8, u8>);
impl DEPART_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEPART_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EEE SRAM SIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEERAMSIZE_A {
    #[doc = "2: 4 KB"]
    EEERAMSIZE_2 = 2,
    #[doc = "3: 2 KB"]
    EEERAMSIZE_3 = 3,
    #[doc = "4: 1 KB"]
    EEERAMSIZE_4 = 4,
    #[doc = "5: 512 Bytes"]
    EEERAMSIZE_5 = 5,
    #[doc = "6: 256 Bytes"]
    EEERAMSIZE_6 = 6,
    #[doc = "7: 128 Bytes"]
    EEERAMSIZE_7 = 7,
    #[doc = "8: 64 Bytes"]
    EEERAMSIZE_8 = 8,
    #[doc = "9: 32 Bytes"]
    EEERAMSIZE_9 = 9,
    #[doc = "15: 0 Bytes"]
    EEERAMSIZE_15 = 15,
}
impl From<EEERAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EEERAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEERAMSIZE` reader - EEE SRAM SIZE"]
pub struct EEERAMSIZE_R(crate::FieldReader<u8, EEERAMSIZE_A>);
impl EEERAMSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EEERAMSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EEERAMSIZE_A> {
        match self.bits {
            2 => Some(EEERAMSIZE_A::EEERAMSIZE_2),
            3 => Some(EEERAMSIZE_A::EEERAMSIZE_3),
            4 => Some(EEERAMSIZE_A::EEERAMSIZE_4),
            5 => Some(EEERAMSIZE_A::EEERAMSIZE_5),
            6 => Some(EEERAMSIZE_A::EEERAMSIZE_6),
            7 => Some(EEERAMSIZE_A::EEERAMSIZE_7),
            8 => Some(EEERAMSIZE_A::EEERAMSIZE_8),
            9 => Some(EEERAMSIZE_A::EEERAMSIZE_9),
            15 => Some(EEERAMSIZE_A::EEERAMSIZE_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_2`"]
    #[inline(always)]
    pub fn is_eeeramsize_2(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_2
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_3`"]
    #[inline(always)]
    pub fn is_eeeramsize_3(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_3
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_4`"]
    #[inline(always)]
    pub fn is_eeeramsize_4(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_4
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_5`"]
    #[inline(always)]
    pub fn is_eeeramsize_5(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_5
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_6`"]
    #[inline(always)]
    pub fn is_eeeramsize_6(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_6
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_7`"]
    #[inline(always)]
    pub fn is_eeeramsize_7(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_7
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_8`"]
    #[inline(always)]
    pub fn is_eeeramsize_8(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_8
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_9`"]
    #[inline(always)]
    pub fn is_eeeramsize_9(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_9
    }
    #[doc = "Checks if the value of the field is `EEERAMSIZE_15`"]
    #[inline(always)]
    pub fn is_eeeramsize_15(&self) -> bool {
        **self == EEERAMSIZE_A::EEERAMSIZE_15
    }
}
impl core::ops::Deref for EEERAMSIZE_R {
    type Target = crate::FieldReader<u8, EEERAMSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:15 - FlexNVM partition"]
    #[inline(always)]
    pub fn depart(&self) -> DEPART_R {
        DEPART_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEE SRAM SIZE"]
    #[inline(always)]
    pub fn eeeramsize(&self) -> EEERAMSIZE_R {
        EEERAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1](index.html) module"]
pub struct FCFG1_SPEC;
impl crate::RegisterSpec for FCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg1::R](R) reader structure"]
impl crate::Readable for FCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](W) writer structure"]
impl crate::Writable for FCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFG1 to value 0"]
impl crate::Resettable for FCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
