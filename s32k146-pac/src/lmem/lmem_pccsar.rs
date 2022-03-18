#[doc = "Register `LMEM_PCCSAR` reader"]
pub struct R(crate::R<LMEM_PCCSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMEM_PCCSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMEM_PCCSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMEM_PCCSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMEM_PCCSAR` writer"]
pub struct W(crate::W<LMEM_PCCSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMEM_PCCSAR_SPEC>;
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
impl From<crate::W<LMEM_PCCSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMEM_PCCSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGO_A {
    #[doc = "0: Write: no effect. Read: no line command active."]
    _0 = 0,
    #[doc = "1: Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    _1 = 1,
}
impl From<LGO_A> for bool {
    #[inline(always)]
    fn from(variant: LGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGO` reader - Initiate Cache Line Command"]
pub struct LGO_R(crate::FieldReader<bool, LGO_A>);
impl LGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGO_A {
        match self.bits {
            false => LGO_A::_0,
            true => LGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LGO_A::_1
    }
}
impl core::ops::Deref for LGO_R {
    type Target = crate::FieldReader<bool, LGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LGO` writer - Initiate Cache Line Command"]
pub struct LGO_W<'a> {
    w: &'a mut W,
}
impl<'a> LGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LGO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGO_A::_0)
    }
    #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGO_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PHYADDR` reader - Physical Address"]
pub struct PHYADDR_R(crate::FieldReader<u32, u32>);
impl PHYADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PHYADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYADDR` writer - Physical Address"]
pub struct PHYADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LGO_R {
        LGO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&mut self) -> LGO_W {
        LGO_W { w: self }
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PHYADDR_W {
        PHYADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache search address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmem_pccsar](index.html) module"]
pub struct LMEM_PCCSAR_SPEC;
impl crate::RegisterSpec for LMEM_PCCSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmem_pccsar::R](R) reader structure"]
impl crate::Readable for LMEM_PCCSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmem_pccsar::W](W) writer structure"]
impl crate::Writable for LMEM_PCCSAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMEM_PCCSAR to value 0"]
impl crate::Resettable for LMEM_PCCSAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
