#[doc = "Register `FLT_ID1` reader"]
pub struct R(crate::R<FLT_ID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_ID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_ID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_ID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT_ID1` writer"]
pub struct W(crate::W<FLT_ID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_ID1_SPEC>;
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
impl From<crate::W<FLT_ID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_ID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT_ID1` reader - ID Filter 1 for Pretended Networking filtering"]
pub struct FLT_ID1_R(crate::FieldReader<u32, u32>);
impl FLT_ID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FLT_ID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_ID1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_ID1` writer - ID Filter 1 for Pretended Networking filtering"]
pub struct FLT_ID1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_ID1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Remote Transmission Request Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_RTR_A {
    #[doc = "0: Reject remote frame (accept data frame)"]
    _0 = 0,
    #[doc = "1: Accept remote frame"]
    _1 = 1,
}
impl From<FLT_RTR_A> for bool {
    #[inline(always)]
    fn from(variant: FLT_RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT_RTR` reader - Remote Transmission Request Filter"]
pub struct FLT_RTR_R(crate::FieldReader<bool, FLT_RTR_A>);
impl FLT_RTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT_RTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_RTR_A {
        match self.bits {
            false => FLT_RTR_A::_0,
            true => FLT_RTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLT_RTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLT_RTR_A::_1
    }
}
impl core::ops::Deref for FLT_RTR_R {
    type Target = crate::FieldReader<bool, FLT_RTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_RTR` writer - Remote Transmission Request Filter"]
pub struct FLT_RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_RTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_RTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reject remote frame (accept data frame)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_RTR_A::_0)
    }
    #[doc = "Accept remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_RTR_A::_1)
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
#[doc = "ID Extended Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_IDE_A {
    #[doc = "0: Accept standard frame format"]
    _0 = 0,
    #[doc = "1: Accept extended frame format"]
    _1 = 1,
}
impl From<FLT_IDE_A> for bool {
    #[inline(always)]
    fn from(variant: FLT_IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT_IDE` reader - ID Extended Filter"]
pub struct FLT_IDE_R(crate::FieldReader<bool, FLT_IDE_A>);
impl FLT_IDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLT_IDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_IDE_A {
        match self.bits {
            false => FLT_IDE_A::_0,
            true => FLT_IDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLT_IDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLT_IDE_A::_1
    }
}
impl core::ops::Deref for FLT_IDE_R {
    type Target = crate::FieldReader<bool, FLT_IDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_IDE` writer - ID Extended Filter"]
pub struct FLT_IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_IDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_IDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accept standard frame format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_IDE_A::_0)
    }
    #[doc = "Accept extended frame format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_IDE_A::_1)
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
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline(always)]
    pub fn flt_id1(&self) -> FLT_ID1_R {
        FLT_ID1_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline(always)]
    pub fn flt_rtr(&self) -> FLT_RTR_R {
        FLT_RTR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline(always)]
    pub fn flt_ide(&self) -> FLT_IDE_R {
        FLT_IDE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline(always)]
    pub fn flt_id1(&mut self) -> FLT_ID1_W {
        FLT_ID1_W { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline(always)]
    pub fn flt_rtr(&mut self) -> FLT_RTR_W {
        FLT_RTR_W { w: self }
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline(always)]
    pub fn flt_ide(&mut self) -> FLT_IDE_W {
        FLT_IDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking ID Filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_id1](index.html) module"]
pub struct FLT_ID1_SPEC;
impl crate::RegisterSpec for FLT_ID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt_id1::R](R) reader structure"]
impl crate::Readable for FLT_ID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt_id1::W](W) writer structure"]
impl crate::Writable for FLT_ID1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT_ID1 to value 0"]
impl crate::Resettable for FLT_ID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
