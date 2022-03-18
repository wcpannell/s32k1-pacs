#[doc = "Register `PODLY` reader"]
pub struct R(crate::R<PDB1_PODLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDB1_PODLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDB1_PODLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDB1_PODLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PODLY` writer"]
pub struct W(crate::W<PDB1_PODLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDB1_PODLY_SPEC>;
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
impl From<crate::W<PDB1_PODLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDB1_PODLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY2` reader - PDB Pulse-Out Delay 2"]
pub struct DLY2_R(crate::FieldReader<u16, u16>);
impl DLY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DLY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY2` writer - PDB Pulse-Out Delay 2"]
pub struct DLY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DLY1` reader - PDB Pulse-Out Delay 1"]
pub struct DLY1_R(crate::FieldReader<u16, u16>);
impl DLY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DLY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY1` writer - PDB Pulse-Out Delay 1"]
pub struct DLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    pub fn dly2(&self) -> DLY2_R {
        DLY2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    pub fn dly1(&self) -> DLY1_R {
        DLY1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    pub fn dly2(&mut self) -> DLY2_W {
        DLY2_W { w: self }
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    pub fn dly1(&mut self) -> DLY1_W {
        DLY1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse-Out n Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdb1_podly](index.html) module"]
pub struct PDB1_PODLY_SPEC;
impl crate::RegisterSpec for PDB1_PODLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdb1_podly::R](R) reader structure"]
impl crate::Readable for PDB1_PODLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdb1_podly::W](W) writer structure"]
impl crate::Writable for PDB1_PODLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PODLY to value 0"]
impl crate::Resettable for PDB1_PODLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
