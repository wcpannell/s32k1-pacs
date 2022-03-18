#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Existence of SRS\\[WAKEUP\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWAKEUP_A {
    #[doc = "0: The feature is not available."]
    EWAKEUP_0 = 0,
    #[doc = "1: The feature is available."]
    EWAKEUP_1 = 1,
}
impl From<EWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: EWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWAKEUP` reader - Existence of SRS\\[WAKEUP\\]
status indication feature"]
pub struct EWAKEUP_R(crate::FieldReader<bool, EWAKEUP_A>);
impl EWAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWAKEUP_A {
        match self.bits {
            false => EWAKEUP_A::EWAKEUP_0,
            true => EWAKEUP_A::EWAKEUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EWAKEUP_0`"]
    #[inline(always)]
    pub fn is_ewakeup_0(&self) -> bool {
        **self == EWAKEUP_A::EWAKEUP_0
    }
    #[doc = "Checks if the value of the field is `EWAKEUP_1`"]
    #[inline(always)]
    pub fn is_ewakeup_1(&self) -> bool {
        **self == EWAKEUP_A::EWAKEUP_1
    }
}
impl core::ops::Deref for EWAKEUP_R {
    type Target = crate::FieldReader<bool, EWAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[LVD\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELVD_A {
    #[doc = "0: The feature is not available."]
    ELVD_0 = 0,
    #[doc = "1: The feature is available."]
    ELVD_1 = 1,
}
impl From<ELVD_A> for bool {
    #[inline(always)]
    fn from(variant: ELVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELVD` reader - Existence of SRS\\[LVD\\]
status indication feature"]
pub struct ELVD_R(crate::FieldReader<bool, ELVD_A>);
impl ELVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELVD_A {
        match self.bits {
            false => ELVD_A::ELVD_0,
            true => ELVD_A::ELVD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ELVD_0`"]
    #[inline(always)]
    pub fn is_elvd_0(&self) -> bool {
        **self == ELVD_A::ELVD_0
    }
    #[doc = "Checks if the value of the field is `ELVD_1`"]
    #[inline(always)]
    pub fn is_elvd_1(&self) -> bool {
        **self == ELVD_A::ELVD_1
    }
}
impl core::ops::Deref for ELVD_R {
    type Target = crate::FieldReader<bool, ELVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[LOC\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOC_A {
    #[doc = "0: The feature is not available."]
    ELOC_0 = 0,
    #[doc = "1: The feature is available."]
    ELOC_1 = 1,
}
impl From<ELOC_A> for bool {
    #[inline(always)]
    fn from(variant: ELOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELOC` reader - Existence of SRS\\[LOC\\]
status indication feature"]
pub struct ELOC_R(crate::FieldReader<bool, ELOC_A>);
impl ELOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOC_A {
        match self.bits {
            false => ELOC_A::ELOC_0,
            true => ELOC_A::ELOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ELOC_0`"]
    #[inline(always)]
    pub fn is_eloc_0(&self) -> bool {
        **self == ELOC_A::ELOC_0
    }
    #[doc = "Checks if the value of the field is `ELOC_1`"]
    #[inline(always)]
    pub fn is_eloc_1(&self) -> bool {
        **self == ELOC_A::ELOC_1
    }
}
impl core::ops::Deref for ELOC_R {
    type Target = crate::FieldReader<bool, ELOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[LOL\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOL_A {
    #[doc = "0: The feature is not available."]
    ELOL_0 = 0,
    #[doc = "1: The feature is available."]
    ELOL_1 = 1,
}
impl From<ELOL_A> for bool {
    #[inline(always)]
    fn from(variant: ELOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELOL` reader - Existence of SRS\\[LOL\\]
status indication feature"]
pub struct ELOL_R(crate::FieldReader<bool, ELOL_A>);
impl ELOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOL_A {
        match self.bits {
            false => ELOL_A::ELOL_0,
            true => ELOL_A::ELOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ELOL_0`"]
    #[inline(always)]
    pub fn is_elol_0(&self) -> bool {
        **self == ELOL_A::ELOL_0
    }
    #[doc = "Checks if the value of the field is `ELOL_1`"]
    #[inline(always)]
    pub fn is_elol_1(&self) -> bool {
        **self == ELOL_A::ELOL_1
    }
}
impl core::ops::Deref for ELOL_R {
    type Target = crate::FieldReader<bool, ELOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[CMU_LOC\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECMU_LOC_A {
    #[doc = "0: The feature is not available."]
    ECMU_LOC_0 = 0,
    #[doc = "1: The feature is available."]
    ECMU_LOC_1 = 1,
}
impl From<ECMU_LOC_A> for bool {
    #[inline(always)]
    fn from(variant: ECMU_LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECMU_LOC` reader - Existence of SRS\\[CMU_LOC\\]
status indication feature"]
pub struct ECMU_LOC_R(crate::FieldReader<bool, ECMU_LOC_A>);
impl ECMU_LOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECMU_LOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECMU_LOC_A {
        match self.bits {
            false => ECMU_LOC_A::ECMU_LOC_0,
            true => ECMU_LOC_A::ECMU_LOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECMU_LOC_0`"]
    #[inline(always)]
    pub fn is_ecmu_loc_0(&self) -> bool {
        **self == ECMU_LOC_A::ECMU_LOC_0
    }
    #[doc = "Checks if the value of the field is `ECMU_LOC_1`"]
    #[inline(always)]
    pub fn is_ecmu_loc_1(&self) -> bool {
        **self == ECMU_LOC_A::ECMU_LOC_1
    }
}
impl core::ops::Deref for ECMU_LOC_R {
    type Target = crate::FieldReader<bool, ECMU_LOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[WDOG\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWDOG_A {
    #[doc = "0: The feature is not available."]
    EWDOG_0 = 0,
    #[doc = "1: The feature is available."]
    EWDOG_1 = 1,
}
impl From<EWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: EWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWDOG` reader - Existence of SRS\\[WDOG\\]
status indication feature"]
pub struct EWDOG_R(crate::FieldReader<bool, EWDOG_A>);
impl EWDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWDOG_A {
        match self.bits {
            false => EWDOG_A::EWDOG_0,
            true => EWDOG_A::EWDOG_1,
        }
    }
    #[doc = "Checks if the value of the field is `EWDOG_0`"]
    #[inline(always)]
    pub fn is_ewdog_0(&self) -> bool {
        **self == EWDOG_A::EWDOG_0
    }
    #[doc = "Checks if the value of the field is `EWDOG_1`"]
    #[inline(always)]
    pub fn is_ewdog_1(&self) -> bool {
        **self == EWDOG_A::EWDOG_1
    }
}
impl core::ops::Deref for EWDOG_R {
    type Target = crate::FieldReader<bool, EWDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[PIN\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN_A {
    #[doc = "0: The feature is not available."]
    EPIN_0 = 0,
    #[doc = "1: The feature is available."]
    EPIN_1 = 1,
}
impl From<EPIN_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN` reader - Existence of SRS\\[PIN\\]
status indication feature"]
pub struct EPIN_R(crate::FieldReader<bool, EPIN_A>);
impl EPIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN_A {
        match self.bits {
            false => EPIN_A::EPIN_0,
            true => EPIN_A::EPIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPIN_0`"]
    #[inline(always)]
    pub fn is_epin_0(&self) -> bool {
        **self == EPIN_A::EPIN_0
    }
    #[doc = "Checks if the value of the field is `EPIN_1`"]
    #[inline(always)]
    pub fn is_epin_1(&self) -> bool {
        **self == EPIN_A::EPIN_1
    }
}
impl core::ops::Deref for EPIN_R {
    type Target = crate::FieldReader<bool, EPIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[POR\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOR_A {
    #[doc = "0: The feature is not available."]
    EPOR_0 = 0,
    #[doc = "1: The feature is available."]
    EPOR_1 = 1,
}
impl From<EPOR_A> for bool {
    #[inline(always)]
    fn from(variant: EPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOR` reader - Existence of SRS\\[POR\\]
status indication feature"]
pub struct EPOR_R(crate::FieldReader<bool, EPOR_A>);
impl EPOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOR_A {
        match self.bits {
            false => EPOR_A::EPOR_0,
            true => EPOR_A::EPOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPOR_0`"]
    #[inline(always)]
    pub fn is_epor_0(&self) -> bool {
        **self == EPOR_A::EPOR_0
    }
    #[doc = "Checks if the value of the field is `EPOR_1`"]
    #[inline(always)]
    pub fn is_epor_1(&self) -> bool {
        **self == EPOR_A::EPOR_1
    }
}
impl core::ops::Deref for EPOR_R {
    type Target = crate::FieldReader<bool, EPOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[JTAG\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EJTAG_A {
    #[doc = "0: The feature is not available."]
    EJTAG_0 = 0,
    #[doc = "1: The feature is available."]
    EJTAG_1 = 1,
}
impl From<EJTAG_A> for bool {
    #[inline(always)]
    fn from(variant: EJTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EJTAG` reader - Existence of SRS\\[JTAG\\]
status indication feature"]
pub struct EJTAG_R(crate::FieldReader<bool, EJTAG_A>);
impl EJTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EJTAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EJTAG_A {
        match self.bits {
            false => EJTAG_A::EJTAG_0,
            true => EJTAG_A::EJTAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `EJTAG_0`"]
    #[inline(always)]
    pub fn is_ejtag_0(&self) -> bool {
        **self == EJTAG_A::EJTAG_0
    }
    #[doc = "Checks if the value of the field is `EJTAG_1`"]
    #[inline(always)]
    pub fn is_ejtag_1(&self) -> bool {
        **self == EJTAG_A::EJTAG_1
    }
}
impl core::ops::Deref for EJTAG_R {
    type Target = crate::FieldReader<bool, EJTAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[LOCKUP\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOCKUP_A {
    #[doc = "0: The feature is not available."]
    ELOCKUP_0 = 0,
    #[doc = "1: The feature is available."]
    ELOCKUP_1 = 1,
}
impl From<ELOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: ELOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELOCKUP` reader - Existence of SRS\\[LOCKUP\\]
status indication feature"]
pub struct ELOCKUP_R(crate::FieldReader<bool, ELOCKUP_A>);
impl ELOCKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOCKUP_A {
        match self.bits {
            false => ELOCKUP_A::ELOCKUP_0,
            true => ELOCKUP_A::ELOCKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ELOCKUP_0`"]
    #[inline(always)]
    pub fn is_elockup_0(&self) -> bool {
        **self == ELOCKUP_A::ELOCKUP_0
    }
    #[doc = "Checks if the value of the field is `ELOCKUP_1`"]
    #[inline(always)]
    pub fn is_elockup_1(&self) -> bool {
        **self == ELOCKUP_A::ELOCKUP_1
    }
}
impl core::ops::Deref for ELOCKUP_R {
    type Target = crate::FieldReader<bool, ELOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[SW\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESW_A {
    #[doc = "0: The feature is not available."]
    ESW_0 = 0,
    #[doc = "1: The feature is available."]
    ESW_1 = 1,
}
impl From<ESW_A> for bool {
    #[inline(always)]
    fn from(variant: ESW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESW` reader - Existence of SRS\\[SW\\]
status indication feature"]
pub struct ESW_R(crate::FieldReader<bool, ESW_A>);
impl ESW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESW_A {
        match self.bits {
            false => ESW_A::ESW_0,
            true => ESW_A::ESW_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESW_0`"]
    #[inline(always)]
    pub fn is_esw_0(&self) -> bool {
        **self == ESW_A::ESW_0
    }
    #[doc = "Checks if the value of the field is `ESW_1`"]
    #[inline(always)]
    pub fn is_esw_1(&self) -> bool {
        **self == ESW_A::ESW_1
    }
}
impl core::ops::Deref for ESW_R {
    type Target = crate::FieldReader<bool, ESW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[MDM_AP\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMDM_AP_A {
    #[doc = "0: The feature is not available."]
    EMDM_AP_0 = 0,
    #[doc = "1: The feature is available."]
    EMDM_AP_1 = 1,
}
impl From<EMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: EMDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMDM_AP` reader - Existence of SRS\\[MDM_AP\\]
status indication feature"]
pub struct EMDM_AP_R(crate::FieldReader<bool, EMDM_AP_A>);
impl EMDM_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMDM_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMDM_AP_A {
        match self.bits {
            false => EMDM_AP_A::EMDM_AP_0,
            true => EMDM_AP_A::EMDM_AP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EMDM_AP_0`"]
    #[inline(always)]
    pub fn is_emdm_ap_0(&self) -> bool {
        **self == EMDM_AP_A::EMDM_AP_0
    }
    #[doc = "Checks if the value of the field is `EMDM_AP_1`"]
    #[inline(always)]
    pub fn is_emdm_ap_1(&self) -> bool {
        **self == EMDM_AP_A::EMDM_AP_1
    }
}
impl core::ops::Deref for EMDM_AP_R {
    type Target = crate::FieldReader<bool, EMDM_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[SACKERR\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESACKERR_A {
    #[doc = "0: The feature is not available."]
    ESACKERR_0 = 0,
    #[doc = "1: The feature is available."]
    ESACKERR_1 = 1,
}
impl From<ESACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ESACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESACKERR` reader - Existence of SRS\\[SACKERR\\]
status indication feature"]
pub struct ESACKERR_R(crate::FieldReader<bool, ESACKERR_A>);
impl ESACKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESACKERR_A {
        match self.bits {
            false => ESACKERR_A::ESACKERR_0,
            true => ESACKERR_A::ESACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESACKERR_0`"]
    #[inline(always)]
    pub fn is_esackerr_0(&self) -> bool {
        **self == ESACKERR_A::ESACKERR_0
    }
    #[doc = "Checks if the value of the field is `ESACKERR_1`"]
    #[inline(always)]
    pub fn is_esackerr_1(&self) -> bool {
        **self == ESACKERR_A::ESACKERR_1
    }
}
impl core::ops::Deref for ESACKERR_R {
    type Target = crate::FieldReader<bool, ESACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[TAMPER\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETAMPER_A {
    #[doc = "0: The feature is not available."]
    ETAMPER_0 = 0,
    #[doc = "1: The feature is available."]
    ETAMPER_1 = 1,
}
impl From<ETAMPER_A> for bool {
    #[inline(always)]
    fn from(variant: ETAMPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETAMPER` reader - Existence of SRS\\[TAMPER\\]
status indication feature"]
pub struct ETAMPER_R(crate::FieldReader<bool, ETAMPER_A>);
impl ETAMPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETAMPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETAMPER_A {
        match self.bits {
            false => ETAMPER_A::ETAMPER_0,
            true => ETAMPER_A::ETAMPER_1,
        }
    }
    #[doc = "Checks if the value of the field is `ETAMPER_0`"]
    #[inline(always)]
    pub fn is_etamper_0(&self) -> bool {
        **self == ETAMPER_A::ETAMPER_0
    }
    #[doc = "Checks if the value of the field is `ETAMPER_1`"]
    #[inline(always)]
    pub fn is_etamper_1(&self) -> bool {
        **self == ETAMPER_A::ETAMPER_1
    }
}
impl core::ops::Deref for ETAMPER_R {
    type Target = crate::FieldReader<bool, ETAMPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Existence of SRS\\[CORE1\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECORE1_A {
    #[doc = "0: The feature is not available."]
    ECORE1_0 = 0,
    #[doc = "1: The feature is available."]
    ECORE1_1 = 1,
}
impl From<ECORE1_A> for bool {
    #[inline(always)]
    fn from(variant: ECORE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECORE1` reader - Existence of SRS\\[CORE1\\]
status indication feature"]
pub struct ECORE1_R(crate::FieldReader<bool, ECORE1_A>);
impl ECORE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECORE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECORE1_A {
        match self.bits {
            false => ECORE1_A::ECORE1_0,
            true => ECORE1_A::ECORE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECORE1_0`"]
    #[inline(always)]
    pub fn is_ecore1_0(&self) -> bool {
        **self == ECORE1_A::ECORE1_0
    }
    #[doc = "Checks if the value of the field is `ECORE1_1`"]
    #[inline(always)]
    pub fn is_ecore1_1(&self) -> bool {
        **self == ECORE1_A::ECORE1_1
    }
}
impl core::ops::Deref for ECORE1_R {
    type Target = crate::FieldReader<bool, ECORE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Existence of SRS\\[WAKEUP\\]
status indication feature"]
    #[inline(always)]
    pub fn ewakeup(&self) -> EWAKEUP_R {
        EWAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Existence of SRS\\[LVD\\]
status indication feature"]
    #[inline(always)]
    pub fn elvd(&self) -> ELVD_R {
        ELVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Existence of SRS\\[LOC\\]
status indication feature"]
    #[inline(always)]
    pub fn eloc(&self) -> ELOC_R {
        ELOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Existence of SRS\\[LOL\\]
status indication feature"]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Existence of SRS\\[CMU_LOC\\]
status indication feature"]
    #[inline(always)]
    pub fn ecmu_loc(&self) -> ECMU_LOC_R {
        ECMU_LOC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Existence of SRS\\[WDOG\\]
status indication feature"]
    #[inline(always)]
    pub fn ewdog(&self) -> EWDOG_R {
        EWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Existence of SRS\\[PIN\\]
status indication feature"]
    #[inline(always)]
    pub fn epin(&self) -> EPIN_R {
        EPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Existence of SRS\\[POR\\]
status indication feature"]
    #[inline(always)]
    pub fn epor(&self) -> EPOR_R {
        EPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Existence of SRS\\[JTAG\\]
status indication feature"]
    #[inline(always)]
    pub fn ejtag(&self) -> EJTAG_R {
        EJTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Existence of SRS\\[LOCKUP\\]
status indication feature"]
    #[inline(always)]
    pub fn elockup(&self) -> ELOCKUP_R {
        ELOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Existence of SRS\\[SW\\]
status indication feature"]
    #[inline(always)]
    pub fn esw(&self) -> ESW_R {
        ESW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Existence of SRS\\[MDM_AP\\]
status indication feature"]
    #[inline(always)]
    pub fn emdm_ap(&self) -> EMDM_AP_R {
        EMDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Existence of SRS\\[SACKERR\\]
status indication feature"]
    #[inline(always)]
    pub fn esackerr(&self) -> ESACKERR_R {
        ESACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Existence of SRS\\[TAMPER\\]
status indication feature"]
    #[inline(always)]
    pub fn etamper(&self) -> ETAMPER_R {
        ETAMPER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Existence of SRS\\[CORE1\\]
status indication feature"]
    #[inline(always)]
    pub fn ecore1(&self) -> ECORE1_R {
        ECORE1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x2fee"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2fee
    }
}
