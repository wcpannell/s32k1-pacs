#[doc = "Register `FCSESTAT` reader"]
pub struct R(crate::R<FCSESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub struct BSY_R(crate::FieldReader<bool, bool>);
impl BSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB` reader - Secure Boot"]
pub struct SB_R(crate::FieldReader<bool, bool>);
impl SB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIN` reader - Secure Boot Initialization"]
pub struct BIN_R(crate::FieldReader<bool, bool>);
impl BIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFN` reader - Secure Boot Finished"]
pub struct BFN_R(crate::FieldReader<bool, bool>);
impl BFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOK` reader - Secure Boot OK"]
pub struct BOK_R(crate::FieldReader<bool, bool>);
impl BOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIN` reader - Random Number Generator Initialized"]
pub struct RIN_R(crate::FieldReader<bool, bool>);
impl RIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDB` reader - External Debug"]
pub struct EDB_R(crate::FieldReader<bool, bool>);
impl EDB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDB` reader - Internal Debug"]
pub struct IDB_R(crate::FieldReader<bool, bool>);
impl IDB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure Boot"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure Boot Initialization"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure Boot Finished"]
    #[inline(always)]
    pub fn bfn(&self) -> BFN_R {
        BFN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure Boot OK"]
    #[inline(always)]
    pub fn bok(&self) -> BOK_R {
        BOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Random Number Generator Initialized"]
    #[inline(always)]
    pub fn rin(&self) -> RIN_R {
        RIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Debug"]
    #[inline(always)]
    pub fn edb(&self) -> EDB_R {
        EDB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal Debug"]
    #[inline(always)]
    pub fn idb(&self) -> IDB_R {
        IDB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Flash CSEc Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcsestat](index.html) module"]
pub struct FCSESTAT_SPEC;
impl crate::RegisterSpec for FCSESTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcsestat::R](R) reader structure"]
impl crate::Readable for FCSESTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCSESTAT to value 0"]
impl crate::Resettable for FCSESTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
