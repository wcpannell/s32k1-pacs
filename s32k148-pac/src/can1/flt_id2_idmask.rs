#[doc = "Register `FLT_ID2_IDMASK` reader"]
pub struct R(crate::R<FLT_ID2_IDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_ID2_IDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_ID2_IDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_ID2_IDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT_ID2_IDMASK` writer"]
pub struct W(crate::W<FLT_ID2_IDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_ID2_IDMASK_SPEC>;
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
impl From<crate::W<FLT_ID2_IDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_ID2_IDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT_ID2_IDMASK` reader - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
pub struct FLT_ID2_IDMASK_R(crate::FieldReader<u32, u32>);
impl FLT_ID2_IDMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FLT_ID2_IDMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_ID2_IDMASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_ID2_IDMASK` writer - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
pub struct FLT_ID2_IDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_ID2_IDMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Remote Transmission Request Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_MSK_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    _1 = 1,
}
impl From<RTR_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR_MSK` reader - Remote Transmission Request Mask Bit"]
pub struct RTR_MSK_R(crate::FieldReader<bool, RTR_MSK_A>);
impl RTR_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTR_MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_MSK_A {
        match self.bits {
            false => RTR_MSK_A::_0,
            true => RTR_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTR_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTR_MSK_A::_1
    }
}
impl core::ops::Deref for RTR_MSK_R {
    type Target = crate::FieldReader<bool, RTR_MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR_MSK` writer - Remote Transmission Request Mask Bit"]
pub struct RTR_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTR_MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTR_MSK_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTR_MSK_A::_1)
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
#[doc = "ID Extended Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_MSK_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    _1 = 1,
}
impl From<IDE_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE_MSK` reader - ID Extended Mask Bit"]
pub struct IDE_MSK_R(crate::FieldReader<bool, IDE_MSK_A>);
impl IDE_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDE_MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_MSK_A {
        match self.bits {
            false => IDE_MSK_A::_0,
            true => IDE_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDE_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDE_MSK_A::_1
    }
}
impl core::ops::Deref for IDE_MSK_R {
    type Target = crate::FieldReader<bool, IDE_MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDE_MSK` writer - ID Extended Mask Bit"]
pub struct IDE_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDE_MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDE_MSK_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDE_MSK_A::_1)
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
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline(always)]
    pub fn flt_id2_idmask(&self) -> FLT_ID2_IDMASK_R {
        FLT_ID2_IDMASK_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline(always)]
    pub fn rtr_msk(&self) -> RTR_MSK_R {
        RTR_MSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline(always)]
    pub fn ide_msk(&self) -> IDE_MSK_R {
        IDE_MSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline(always)]
    pub fn flt_id2_idmask(&mut self) -> FLT_ID2_IDMASK_W {
        FLT_ID2_IDMASK_W { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline(always)]
    pub fn rtr_msk(&mut self) -> RTR_MSK_W {
        RTR_MSK_W { w: self }
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline(always)]
    pub fn ide_msk(&mut self) -> IDE_MSK_W {
        IDE_MSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_id2_idmask](index.html) module"]
pub struct FLT_ID2_IDMASK_SPEC;
impl crate::RegisterSpec for FLT_ID2_IDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt_id2_idmask::R](R) reader structure"]
impl crate::Readable for FLT_ID2_IDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt_id2_idmask::W](W) writer structure"]
impl crate::Writable for FLT_ID2_IDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT_ID2_IDMASK to value 0"]
impl crate::Resettable for FLT_ID2_IDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
