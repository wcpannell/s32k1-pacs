#[doc = "Register `FCSESTAT1` reader"]
pub struct R(crate::R<FCSESTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSESTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSESTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSESTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    #[doc = "0: CSEc command processing has completed"]
    BSY_0 = 0,
    #[doc = "1: CSEc command processing is in progress"]
    BSY_1 = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub struct BSY_R(crate::FieldReader<bool, BSY_A>);
impl BSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::BSY_0,
            true => BSY_A::BSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BSY_0`"]
    #[inline(always)]
    pub fn is_bsy_0(&self) -> bool {
        **self == BSY_A::BSY_0
    }
    #[doc = "Checks if the value of the field is `BSY_1`"]
    #[inline(always)]
    pub fn is_bsy_1(&self) -> bool {
        **self == BSY_A::BSY_1
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, BSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Secure Boot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SB_A {
    #[doc = "0: Secure boot not activated"]
    SB_0 = 0,
    #[doc = "1: Secure boot is activated"]
    SB_1 = 1,
}
impl From<SB_A> for bool {
    #[inline(always)]
    fn from(variant: SB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SB` reader - Secure Boot"]
pub struct SB_R(crate::FieldReader<bool, SB_A>);
impl SB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB_A {
        match self.bits {
            false => SB_A::SB_0,
            true => SB_A::SB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SB_0`"]
    #[inline(always)]
    pub fn is_sb_0(&self) -> bool {
        **self == SB_A::SB_0
    }
    #[doc = "Checks if the value of the field is `SB_1`"]
    #[inline(always)]
    pub fn is_sb_1(&self) -> bool {
        **self == SB_A::SB_1
    }
}
impl core::ops::Deref for SB_R {
    type Target = crate::FieldReader<bool, SB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Secure Boot Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIN_A {
    #[doc = "0: Secure boot personalization not completed"]
    BIN_0 = 0,
    #[doc = "1: Secure boot personalization has completed"]
    BIN_1 = 1,
}
impl From<BIN_A> for bool {
    #[inline(always)]
    fn from(variant: BIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIN` reader - Secure Boot Initialization"]
pub struct BIN_R(crate::FieldReader<bool, BIN_A>);
impl BIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIN_A {
        match self.bits {
            false => BIN_A::BIN_0,
            true => BIN_A::BIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIN_0`"]
    #[inline(always)]
    pub fn is_bin_0(&self) -> bool {
        **self == BIN_A::BIN_0
    }
    #[doc = "Checks if the value of the field is `BIN_1`"]
    #[inline(always)]
    pub fn is_bin_1(&self) -> bool {
        **self == BIN_A::BIN_1
    }
}
impl core::ops::Deref for BIN_R {
    type Target = crate::FieldReader<bool, BIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Secure Boot Finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFN_A {
    #[doc = "0: Secure Boot is not finished"]
    BFN_0 = 0,
    #[doc = "1: Secure Boot has finished"]
    BFN_1 = 1,
}
impl From<BFN_A> for bool {
    #[inline(always)]
    fn from(variant: BFN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFN` reader - Secure Boot Finished"]
pub struct BFN_R(crate::FieldReader<bool, BFN_A>);
impl BFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFN_A {
        match self.bits {
            false => BFN_A::BFN_0,
            true => BFN_A::BFN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFN_0`"]
    #[inline(always)]
    pub fn is_bfn_0(&self) -> bool {
        **self == BFN_A::BFN_0
    }
    #[doc = "Checks if the value of the field is `BFN_1`"]
    #[inline(always)]
    pub fn is_bfn_1(&self) -> bool {
        **self == BFN_A::BFN_1
    }
}
impl core::ops::Deref for BFN_R {
    type Target = crate::FieldReader<bool, BFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Secure Boot OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOK_A {
    #[doc = "0: Secure boot is not complete, or secure boot failure"]
    BOK_0 = 0,
    #[doc = "1: Secure boot was successful"]
    BOK_1 = 1,
}
impl From<BOK_A> for bool {
    #[inline(always)]
    fn from(variant: BOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOK` reader - Secure Boot OK"]
pub struct BOK_R(crate::FieldReader<bool, BOK_A>);
impl BOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOK_A {
        match self.bits {
            false => BOK_A::BOK_0,
            true => BOK_A::BOK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOK_0`"]
    #[inline(always)]
    pub fn is_bok_0(&self) -> bool {
        **self == BOK_A::BOK_0
    }
    #[doc = "Checks if the value of the field is `BOK_1`"]
    #[inline(always)]
    pub fn is_bok_1(&self) -> bool {
        **self == BOK_A::BOK_1
    }
}
impl core::ops::Deref for BOK_R {
    type Target = crate::FieldReader<bool, BOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Random Number Generator Initialized\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIN_A {
    #[doc = "0: Random number generator is not initialized"]
    RIN_0 = 0,
    #[doc = "1: Random number generator is initialized"]
    RIN_1 = 1,
}
impl From<RIN_A> for bool {
    #[inline(always)]
    fn from(variant: RIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIN` reader - Random Number Generator Initialized"]
pub struct RIN_R(crate::FieldReader<bool, RIN_A>);
impl RIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIN_A {
        match self.bits {
            false => RIN_A::RIN_0,
            true => RIN_A::RIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIN_0`"]
    #[inline(always)]
    pub fn is_rin_0(&self) -> bool {
        **self == RIN_A::RIN_0
    }
    #[doc = "Checks if the value of the field is `RIN_1`"]
    #[inline(always)]
    pub fn is_rin_1(&self) -> bool {
        **self == RIN_A::RIN_1
    }
}
impl core::ops::Deref for RIN_R {
    type Target = crate::FieldReader<bool, RIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDB_A {
    #[doc = "0: External debugger not attached"]
    EDB_0 = 0,
    #[doc = "1: External debugger is attached"]
    EDB_1 = 1,
}
impl From<EDB_A> for bool {
    #[inline(always)]
    fn from(variant: EDB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDB` reader - External Debug"]
pub struct EDB_R(crate::FieldReader<bool, EDB_A>);
impl EDB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDB_A {
        match self.bits {
            false => EDB_A::EDB_0,
            true => EDB_A::EDB_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDB_0`"]
    #[inline(always)]
    pub fn is_edb_0(&self) -> bool {
        **self == EDB_A::EDB_0
    }
    #[doc = "Checks if the value of the field is `EDB_1`"]
    #[inline(always)]
    pub fn is_edb_1(&self) -> bool {
        **self == EDB_A::EDB_1
    }
}
impl core::ops::Deref for EDB_R {
    type Target = crate::FieldReader<bool, EDB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDB_A {
    #[doc = "0: Internal debug functions are disabled"]
    IDB_0 = 0,
    #[doc = "1: Internal debugger functions are enabled"]
    IDB_1 = 1,
}
impl From<IDB_A> for bool {
    #[inline(always)]
    fn from(variant: IDB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDB` reader - Internal Debug"]
pub struct IDB_R(crate::FieldReader<bool, IDB_A>);
impl IDB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDB_A {
        match self.bits {
            false => IDB_A::IDB_0,
            true => IDB_A::IDB_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDB_0`"]
    #[inline(always)]
    pub fn is_idb_0(&self) -> bool {
        **self == IDB_A::IDB_0
    }
    #[doc = "Checks if the value of the field is `IDB_1`"]
    #[inline(always)]
    pub fn is_idb_1(&self) -> bool {
        **self == IDB_A::IDB_1
    }
}
impl core::ops::Deref for IDB_R {
    type Target = crate::FieldReader<bool, IDB_A>;
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
#[doc = "Flash CSEc Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcsestat1](index.html) module"]
pub struct FCSESTAT1_SPEC;
impl crate::RegisterSpec for FCSESTAT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcsestat1::R](R) reader structure"]
impl crate::Readable for FCSESTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCSESTAT1 to value 0"]
impl crate::Resettable for FCSESTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
