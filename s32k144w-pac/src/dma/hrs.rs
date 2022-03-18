#[doc = "Register `HRS` reader"]
pub struct R(crate::R<HRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for channel 0 is not present"]
    HRS0_0 = 0,
    #[doc = "1: A hardware service request for channel 0 is present"]
    HRS0_1 = 1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS0` reader - Hardware Request Status Channel 0"]
pub struct HRS0_R(crate::FieldReader<bool, HRS0_A>);
impl HRS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::HRS0_0,
            true => HRS0_A::HRS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS0_0`"]
    #[inline(always)]
    pub fn is_hrs0_0(&self) -> bool {
        **self == HRS0_A::HRS0_0
    }
    #[doc = "Checks if the value of the field is `HRS0_1`"]
    #[inline(always)]
    pub fn is_hrs0_1(&self) -> bool {
        **self == HRS0_A::HRS0_1
    }
}
impl core::ops::Deref for HRS0_R {
    type Target = crate::FieldReader<bool, HRS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for channel 1 is not present"]
    HRS1_0 = 0,
    #[doc = "1: A hardware service request for channel 1 is present"]
    HRS1_1 = 1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS1` reader - Hardware Request Status Channel 1"]
pub struct HRS1_R(crate::FieldReader<bool, HRS1_A>);
impl HRS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::HRS1_0,
            true => HRS1_A::HRS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS1_0`"]
    #[inline(always)]
    pub fn is_hrs1_0(&self) -> bool {
        **self == HRS1_A::HRS1_0
    }
    #[doc = "Checks if the value of the field is `HRS1_1`"]
    #[inline(always)]
    pub fn is_hrs1_1(&self) -> bool {
        **self == HRS1_A::HRS1_1
    }
}
impl core::ops::Deref for HRS1_R {
    type Target = crate::FieldReader<bool, HRS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for channel 2 is not present"]
    HRS2_0 = 0,
    #[doc = "1: A hardware service request for channel 2 is present"]
    HRS2_1 = 1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS2` reader - Hardware Request Status Channel 2"]
pub struct HRS2_R(crate::FieldReader<bool, HRS2_A>);
impl HRS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::HRS2_0,
            true => HRS2_A::HRS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS2_0`"]
    #[inline(always)]
    pub fn is_hrs2_0(&self) -> bool {
        **self == HRS2_A::HRS2_0
    }
    #[doc = "Checks if the value of the field is `HRS2_1`"]
    #[inline(always)]
    pub fn is_hrs2_1(&self) -> bool {
        **self == HRS2_A::HRS2_1
    }
}
impl core::ops::Deref for HRS2_R {
    type Target = crate::FieldReader<bool, HRS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for channel 3 is not present"]
    HRS3_0 = 0,
    #[doc = "1: A hardware service request for channel 3 is present"]
    HRS3_1 = 1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS3` reader - Hardware Request Status Channel 3"]
pub struct HRS3_R(crate::FieldReader<bool, HRS3_A>);
impl HRS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::HRS3_0,
            true => HRS3_A::HRS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS3_0`"]
    #[inline(always)]
    pub fn is_hrs3_0(&self) -> bool {
        **self == HRS3_A::HRS3_0
    }
    #[doc = "Checks if the value of the field is `HRS3_1`"]
    #[inline(always)]
    pub fn is_hrs3_1(&self) -> bool {
        **self == HRS3_A::HRS3_1
    }
}
impl core::ops::Deref for HRS3_R {
    type Target = crate::FieldReader<bool, HRS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for channel 4 is not present"]
    HRS4_0 = 0,
    #[doc = "1: A hardware service request for channel 4 is present"]
    HRS4_1 = 1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS4` reader - Hardware Request Status Channel 4"]
pub struct HRS4_R(crate::FieldReader<bool, HRS4_A>);
impl HRS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::HRS4_0,
            true => HRS4_A::HRS4_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS4_0`"]
    #[inline(always)]
    pub fn is_hrs4_0(&self) -> bool {
        **self == HRS4_A::HRS4_0
    }
    #[doc = "Checks if the value of the field is `HRS4_1`"]
    #[inline(always)]
    pub fn is_hrs4_1(&self) -> bool {
        **self == HRS4_A::HRS4_1
    }
}
impl core::ops::Deref for HRS4_R {
    type Target = crate::FieldReader<bool, HRS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for channel 5 is not present"]
    HRS5_0 = 0,
    #[doc = "1: A hardware service request for channel 5 is present"]
    HRS5_1 = 1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS5` reader - Hardware Request Status Channel 5"]
pub struct HRS5_R(crate::FieldReader<bool, HRS5_A>);
impl HRS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::HRS5_0,
            true => HRS5_A::HRS5_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS5_0`"]
    #[inline(always)]
    pub fn is_hrs5_0(&self) -> bool {
        **self == HRS5_A::HRS5_0
    }
    #[doc = "Checks if the value of the field is `HRS5_1`"]
    #[inline(always)]
    pub fn is_hrs5_1(&self) -> bool {
        **self == HRS5_A::HRS5_1
    }
}
impl core::ops::Deref for HRS5_R {
    type Target = crate::FieldReader<bool, HRS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for channel 6 is not present"]
    HRS6_0 = 0,
    #[doc = "1: A hardware service request for channel 6 is present"]
    HRS6_1 = 1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS6` reader - Hardware Request Status Channel 6"]
pub struct HRS6_R(crate::FieldReader<bool, HRS6_A>);
impl HRS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::HRS6_0,
            true => HRS6_A::HRS6_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS6_0`"]
    #[inline(always)]
    pub fn is_hrs6_0(&self) -> bool {
        **self == HRS6_A::HRS6_0
    }
    #[doc = "Checks if the value of the field is `HRS6_1`"]
    #[inline(always)]
    pub fn is_hrs6_1(&self) -> bool {
        **self == HRS6_A::HRS6_1
    }
}
impl core::ops::Deref for HRS6_R {
    type Target = crate::FieldReader<bool, HRS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for channel 7 is not present"]
    HRS7_0 = 0,
    #[doc = "1: A hardware service request for channel 7 is present"]
    HRS7_1 = 1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS7` reader - Hardware Request Status Channel 7"]
pub struct HRS7_R(crate::FieldReader<bool, HRS7_A>);
impl HRS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::HRS7_0,
            true => HRS7_A::HRS7_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS7_0`"]
    #[inline(always)]
    pub fn is_hrs7_0(&self) -> bool {
        **self == HRS7_A::HRS7_0
    }
    #[doc = "Checks if the value of the field is `HRS7_1`"]
    #[inline(always)]
    pub fn is_hrs7_1(&self) -> bool {
        **self == HRS7_A::HRS7_1
    }
}
impl core::ops::Deref for HRS7_R {
    type Target = crate::FieldReader<bool, HRS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for channel 8 is not present"]
    HRS8_0 = 0,
    #[doc = "1: A hardware service request for channel 8 is present"]
    HRS8_1 = 1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS8` reader - Hardware Request Status Channel 8"]
pub struct HRS8_R(crate::FieldReader<bool, HRS8_A>);
impl HRS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::HRS8_0,
            true => HRS8_A::HRS8_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS8_0`"]
    #[inline(always)]
    pub fn is_hrs8_0(&self) -> bool {
        **self == HRS8_A::HRS8_0
    }
    #[doc = "Checks if the value of the field is `HRS8_1`"]
    #[inline(always)]
    pub fn is_hrs8_1(&self) -> bool {
        **self == HRS8_A::HRS8_1
    }
}
impl core::ops::Deref for HRS8_R {
    type Target = crate::FieldReader<bool, HRS8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for channel 9 is not present"]
    HRS9_0 = 0,
    #[doc = "1: A hardware service request for channel 9 is present"]
    HRS9_1 = 1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS9` reader - Hardware Request Status Channel 9"]
pub struct HRS9_R(crate::FieldReader<bool, HRS9_A>);
impl HRS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::HRS9_0,
            true => HRS9_A::HRS9_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS9_0`"]
    #[inline(always)]
    pub fn is_hrs9_0(&self) -> bool {
        **self == HRS9_A::HRS9_0
    }
    #[doc = "Checks if the value of the field is `HRS9_1`"]
    #[inline(always)]
    pub fn is_hrs9_1(&self) -> bool {
        **self == HRS9_A::HRS9_1
    }
}
impl core::ops::Deref for HRS9_R {
    type Target = crate::FieldReader<bool, HRS9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for channel 10 is not present"]
    HRS10_0 = 0,
    #[doc = "1: A hardware service request for channel 10 is present"]
    HRS10_1 = 1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS10` reader - Hardware Request Status Channel 10"]
pub struct HRS10_R(crate::FieldReader<bool, HRS10_A>);
impl HRS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::HRS10_0,
            true => HRS10_A::HRS10_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS10_0`"]
    #[inline(always)]
    pub fn is_hrs10_0(&self) -> bool {
        **self == HRS10_A::HRS10_0
    }
    #[doc = "Checks if the value of the field is `HRS10_1`"]
    #[inline(always)]
    pub fn is_hrs10_1(&self) -> bool {
        **self == HRS10_A::HRS10_1
    }
}
impl core::ops::Deref for HRS10_R {
    type Target = crate::FieldReader<bool, HRS10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for channel 11 is not present"]
    HRS11_0 = 0,
    #[doc = "1: A hardware service request for channel 11 is present"]
    HRS11_1 = 1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS11` reader - Hardware Request Status Channel 11"]
pub struct HRS11_R(crate::FieldReader<bool, HRS11_A>);
impl HRS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::HRS11_0,
            true => HRS11_A::HRS11_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS11_0`"]
    #[inline(always)]
    pub fn is_hrs11_0(&self) -> bool {
        **self == HRS11_A::HRS11_0
    }
    #[doc = "Checks if the value of the field is `HRS11_1`"]
    #[inline(always)]
    pub fn is_hrs11_1(&self) -> bool {
        **self == HRS11_A::HRS11_1
    }
}
impl core::ops::Deref for HRS11_R {
    type Target = crate::FieldReader<bool, HRS11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for channel 12 is not present"]
    HRS12_0 = 0,
    #[doc = "1: A hardware service request for channel 12 is present"]
    HRS12_1 = 1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS12` reader - Hardware Request Status Channel 12"]
pub struct HRS12_R(crate::FieldReader<bool, HRS12_A>);
impl HRS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::HRS12_0,
            true => HRS12_A::HRS12_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS12_0`"]
    #[inline(always)]
    pub fn is_hrs12_0(&self) -> bool {
        **self == HRS12_A::HRS12_0
    }
    #[doc = "Checks if the value of the field is `HRS12_1`"]
    #[inline(always)]
    pub fn is_hrs12_1(&self) -> bool {
        **self == HRS12_A::HRS12_1
    }
}
impl core::ops::Deref for HRS12_R {
    type Target = crate::FieldReader<bool, HRS12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for channel 13 is not present"]
    HRS13_0 = 0,
    #[doc = "1: A hardware service request for channel 13 is present"]
    HRS13_1 = 1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS13` reader - Hardware Request Status Channel 13"]
pub struct HRS13_R(crate::FieldReader<bool, HRS13_A>);
impl HRS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::HRS13_0,
            true => HRS13_A::HRS13_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS13_0`"]
    #[inline(always)]
    pub fn is_hrs13_0(&self) -> bool {
        **self == HRS13_A::HRS13_0
    }
    #[doc = "Checks if the value of the field is `HRS13_1`"]
    #[inline(always)]
    pub fn is_hrs13_1(&self) -> bool {
        **self == HRS13_A::HRS13_1
    }
}
impl core::ops::Deref for HRS13_R {
    type Target = crate::FieldReader<bool, HRS13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for channel 14 is not present"]
    HRS14_0 = 0,
    #[doc = "1: A hardware service request for channel 14 is present"]
    HRS14_1 = 1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS14` reader - Hardware Request Status Channel 14"]
pub struct HRS14_R(crate::FieldReader<bool, HRS14_A>);
impl HRS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::HRS14_0,
            true => HRS14_A::HRS14_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS14_0`"]
    #[inline(always)]
    pub fn is_hrs14_0(&self) -> bool {
        **self == HRS14_A::HRS14_0
    }
    #[doc = "Checks if the value of the field is `HRS14_1`"]
    #[inline(always)]
    pub fn is_hrs14_1(&self) -> bool {
        **self == HRS14_A::HRS14_1
    }
}
impl core::ops::Deref for HRS14_R {
    type Target = crate::FieldReader<bool, HRS14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for channel 15 is not present"]
    HRS15_0 = 0,
    #[doc = "1: A hardware service request for channel 15 is present"]
    HRS15_1 = 1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS15` reader - Hardware Request Status Channel 15"]
pub struct HRS15_R(crate::FieldReader<bool, HRS15_A>);
impl HRS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRS15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::HRS15_0,
            true => HRS15_A::HRS15_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS15_0`"]
    #[inline(always)]
    pub fn is_hrs15_0(&self) -> bool {
        **self == HRS15_A::HRS15_0
    }
    #[doc = "Checks if the value of the field is `HRS15_1`"]
    #[inline(always)]
    pub fn is_hrs15_1(&self) -> bool {
        **self == HRS15_A::HRS15_1
    }
}
impl core::ops::Deref for HRS15_R {
    type Target = crate::FieldReader<bool, HRS15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](index.html) module"]
pub struct HRS_SPEC;
impl crate::RegisterSpec for HRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrs::R](R) reader structure"]
impl crate::Readable for HRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
