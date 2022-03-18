#[doc = "Register `SRS` reader"]
pub struct R(crate::R<SRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Low-Voltage Detect Reset or High-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_A {
    #[doc = "0: Reset not caused by LVD trip, HVD trip or POR"]
    LVD_0 = 0,
    #[doc = "1: Reset caused by LVD trip, HVD trip or POR"]
    LVD_1 = 1,
}
impl From<LVD_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD` reader - Low-Voltage Detect Reset or High-Voltage Detect Reset"]
pub struct LVD_R(crate::FieldReader<bool, LVD_A>);
impl LVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_A {
        match self.bits {
            false => LVD_A::LVD_0,
            true => LVD_A::LVD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LVD_0`"]
    #[inline(always)]
    pub fn is_lvd_0(&self) -> bool {
        **self == LVD_A::LVD_0
    }
    #[doc = "Checks if the value of the field is `LVD_1`"]
    #[inline(always)]
    pub fn is_lvd_1(&self) -> bool {
        **self == LVD_A::LVD_1
    }
}
impl core::ops::Deref for LVD_R {
    type Target = crate::FieldReader<bool, LVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    LOC_0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    LOC_1 = 1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOC` reader - Loss-of-Clock Reset"]
pub struct LOC_R(crate::FieldReader<bool, LOC_A>);
impl LOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::LOC_0,
            true => LOC_A::LOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOC_0`"]
    #[inline(always)]
    pub fn is_loc_0(&self) -> bool {
        **self == LOC_A::LOC_0
    }
    #[doc = "Checks if the value of the field is `LOC_1`"]
    #[inline(always)]
    pub fn is_loc_1(&self) -> bool {
        **self == LOC_A::LOC_1
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<bool, LOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL/FLL"]
    LOL_0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL/FLL"]
    LOL_1 = 1,
}
impl From<LOL_A> for bool {
    #[inline(always)]
    fn from(variant: LOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - Loss-of-Lock Reset"]
pub struct LOL_R(crate::FieldReader<bool, LOL_A>);
impl LOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOL_A {
        match self.bits {
            false => LOL_A::LOL_0,
            true => LOL_A::LOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOL_0`"]
    #[inline(always)]
    pub fn is_lol_0(&self) -> bool {
        **self == LOL_A::LOL_0
    }
    #[doc = "Checks if the value of the field is `LOL_1`"]
    #[inline(always)]
    pub fn is_lol_1(&self) -> bool {
        **self == LOL_A::LOL_1
    }
}
impl core::ops::Deref for LOL_R {
    type Target = crate::FieldReader<bool, LOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    WDOG_0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    WDOG_1 = 1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog"]
pub struct WDOG_R(crate::FieldReader<bool, WDOG_A>);
impl WDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::WDOG_0,
            true => WDOG_A::WDOG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG_0`"]
    #[inline(always)]
    pub fn is_wdog_0(&self) -> bool {
        **self == WDOG_A::WDOG_0
    }
    #[doc = "Checks if the value of the field is `WDOG_1`"]
    #[inline(always)]
    pub fn is_wdog_1(&self) -> bool {
        **self == WDOG_A::WDOG_1
    }
}
impl core::ops::Deref for WDOG_R {
    type Target = crate::FieldReader<bool, WDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    PIN_0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    PIN_1 = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External Reset Pin"]
pub struct PIN_R(crate::FieldReader<bool, PIN_A>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::PIN_0,
            true => PIN_A::PIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_0`"]
    #[inline(always)]
    pub fn is_pin_0(&self) -> bool {
        **self == PIN_A::PIN_0
    }
    #[doc = "Checks if the value of the field is `PIN_1`"]
    #[inline(always)]
    pub fn is_pin_1(&self) -> bool {
        **self == PIN_A::PIN_1
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: Reset not caused by POR"]
    POR_0 = 0,
    #[doc = "1: Reset caused by POR"]
    POR_1 = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - Power-On Reset"]
pub struct POR_R(crate::FieldReader<bool, POR_A>);
impl POR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::POR_0,
            true => POR_A::POR_1,
        }
    }
    #[doc = "Checks if the value of the field is `POR_0`"]
    #[inline(always)]
    pub fn is_por_0(&self) -> bool {
        **self == POR_A::POR_0
    }
    #[doc = "Checks if the value of the field is `POR_1`"]
    #[inline(always)]
    pub fn is_por_1(&self) -> bool {
        **self == POR_A::POR_1
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, POR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_A {
    #[doc = "0: Reset not caused by JTAG"]
    JTAG_0 = 0,
    #[doc = "1: Reset caused by JTAG"]
    JTAG_1 = 1,
}
impl From<JTAG_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAG` reader - JTAG generated reset"]
pub struct JTAG_R(crate::FieldReader<bool, JTAG_A>);
impl JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_A {
        match self.bits {
            false => JTAG_A::JTAG_0,
            true => JTAG_A::JTAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_0`"]
    #[inline(always)]
    pub fn is_jtag_0(&self) -> bool {
        **self == JTAG_A::JTAG_0
    }
    #[doc = "Checks if the value of the field is `JTAG_1`"]
    #[inline(always)]
    pub fn is_jtag_1(&self) -> bool {
        **self == JTAG_A::JTAG_1
    }
}
impl core::ops::Deref for JTAG_R {
    type Target = crate::FieldReader<bool, JTAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    LOCKUP_0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    LOCKUP_1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup"]
pub struct LOCKUP_R(crate::FieldReader<bool, LOCKUP_A>);
impl LOCKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::LOCKUP_0,
            true => LOCKUP_A::LOCKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_0`"]
    #[inline(always)]
    pub fn is_lockup_0(&self) -> bool {
        **self == LOCKUP_A::LOCKUP_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_1`"]
    #[inline(always)]
    pub fn is_lockup_1(&self) -> bool {
        **self == LOCKUP_A::LOCKUP_1
    }
}
impl core::ops::Deref for LOCKUP_R {
    type Target = crate::FieldReader<bool, LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    SW_0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    SW_1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - Software"]
pub struct SW_R(crate::FieldReader<bool, SW_A>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::SW_0,
            true => SW_A::SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_0`"]
    #[inline(always)]
    pub fn is_sw_0(&self) -> bool {
        **self == SW_A::SW_0
    }
    #[doc = "Checks if the value of the field is `SW_1`"]
    #[inline(always)]
    pub fn is_sw_1(&self) -> bool {
        **self == SW_A::SW_1
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_AP_A {
    #[doc = "0: Reset was not caused by host debugger system setting of the System Reset Request bit"]
    MDM_AP_0 = 0,
    #[doc = "1: Reset was caused by host debugger system setting of the System Reset Request bit"]
    MDM_AP_1 = 1,
}
impl From<MDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: MDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDM_AP` reader - MDM-AP System Reset Request"]
pub struct MDM_AP_R(crate::FieldReader<bool, MDM_AP_A>);
impl MDM_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDM_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDM_AP_A {
        match self.bits {
            false => MDM_AP_A::MDM_AP_0,
            true => MDM_AP_A::MDM_AP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDM_AP_0`"]
    #[inline(always)]
    pub fn is_mdm_ap_0(&self) -> bool {
        **self == MDM_AP_A::MDM_AP_0
    }
    #[doc = "Checks if the value of the field is `MDM_AP_1`"]
    #[inline(always)]
    pub fn is_mdm_ap_1(&self) -> bool {
        **self == MDM_AP_A::MDM_AP_1
    }
}
impl core::ops::Deref for MDM_AP_R {
    type Target = crate::FieldReader<bool, MDM_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stop Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    SACKERR_0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    SACKERR_1 = 1,
}
impl From<SACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKERR` reader - Stop Acknowledge Error"]
pub struct SACKERR_R(crate::FieldReader<bool, SACKERR_A>);
impl SACKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKERR_A {
        match self.bits {
            false => SACKERR_A::SACKERR_0,
            true => SACKERR_A::SACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SACKERR_0`"]
    #[inline(always)]
    pub fn is_sackerr_0(&self) -> bool {
        **self == SACKERR_A::SACKERR_0
    }
    #[doc = "Checks if the value of the field is `SACKERR_1`"]
    #[inline(always)]
    pub fn is_sackerr_1(&self) -> bool {
        **self == SACKERR_A::SACKERR_1
    }
}
impl core::ops::Deref for SACKERR_R {
    type Target = crate::FieldReader<bool, SACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Low-Voltage Detect Reset or High-Voltage Detect Reset"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn lol(&self) -> LOL_R {
        LOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core Lockup"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MDM_AP_R {
        MDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stop Acknowledge Error"]
    #[inline(always)]
    pub fn sackerr(&self) -> SACKERR_R {
        SACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](index.html) module"]
pub struct SRS_SPEC;
impl crate::RegisterSpec for SRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srs::R](R) reader structure"]
impl crate::Readable for SRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRS to value 0x82"]
impl crate::Resettable for SRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}
