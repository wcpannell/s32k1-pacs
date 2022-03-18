#[doc = "Register `TIMCFG1` reader"]
pub struct R(crate::R<TIMCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCFG1` writer"]
pub struct W(crate::W<TIMCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCFG1_SPEC>;
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
impl From<crate::W<TIMCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Start Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTART_A {
    #[doc = "0: Start bit disabled"]
    _0 = 0,
    #[doc = "1: Start bit enabled"]
    _1 = 1,
}
impl From<TSTART_A> for bool {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTART` reader - Timer Start Bit"]
pub struct TSTART_R(crate::FieldReader<bool, TSTART_A>);
impl TSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTART_A {
        match self.bits {
            false => TSTART_A::_0,
            true => TSTART_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSTART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSTART_A::_1
    }
}
impl core::ops::Deref for TSTART_R {
    type Target = crate::FieldReader<bool, TSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTART` writer - Timer Start Bit"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start bit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTART_A::_0)
    }
    #[doc = "Start bit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTART_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Timer Stop Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: Stop bit disabled"]
    _0 = 0,
    #[doc = "1: Stop bit is enabled on timer compare"]
    _1 = 1,
    #[doc = "2: Stop bit is enabled on timer disable"]
    _10 = 2,
    #[doc = "3: Stop bit is enabled on timer compare and timer disable"]
    _11 = 3,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSTOP` reader - Timer Stop Bit"]
pub struct TSTOP_R(crate::FieldReader<u8, TSTOP_A>);
impl TSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTOP_A {
        match self.bits {
            0 => TSTOP_A::_0,
            1 => TSTOP_A::_1,
            2 => TSTOP_A::_10,
            3 => TSTOP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSTOP_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TSTOP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TSTOP_A::_11
    }
}
impl core::ops::Deref for TSTOP_R {
    type Target = crate::FieldReader<u8, TSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTOP` writer - Timer Stop Bit"]
pub struct TSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTOP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Stop bit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTOP_A::_0)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTOP_A::_1)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTOP_A::_10)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTOP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMENA_A {
    #[doc = "0: Timer always enabled"]
    _0 = 0,
    #[doc = "1: Timer enabled on Timer N-1 enable"]
    _1 = 1,
    #[doc = "2: Timer enabled on Trigger high"]
    _10 = 2,
    #[doc = "3: Timer enabled on Trigger high and Pin high"]
    _11 = 3,
    #[doc = "4: Timer enabled on Pin rising edge"]
    _100 = 4,
    #[doc = "5: Timer enabled on Pin rising edge and Trigger high"]
    _101 = 5,
    #[doc = "6: Timer enabled on Trigger rising edge"]
    _110 = 6,
    #[doc = "7: Timer enabled on Trigger rising or falling edge"]
    _111 = 7,
}
impl From<TIMENA_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMENA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMENA` reader - Timer Enable"]
pub struct TIMENA_R(crate::FieldReader<u8, TIMENA_A>);
impl TIMENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMENA_A {
        match self.bits {
            0 => TIMENA_A::_0,
            1 => TIMENA_A::_1,
            2 => TIMENA_A::_10,
            3 => TIMENA_A::_11,
            4 => TIMENA_A::_100,
            5 => TIMENA_A::_101,
            6 => TIMENA_A::_110,
            7 => TIMENA_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMENA_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMENA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMENA_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == TIMENA_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == TIMENA_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == TIMENA_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == TIMENA_A::_111
    }
}
impl core::ops::Deref for TIMENA_R {
    type Target = crate::FieldReader<u8, TIMENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMENA` writer - Timer Enable"]
pub struct TIMENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMENA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer always enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMENA_A::_0)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMENA_A::_1)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMENA_A::_10)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMENA_A::_11)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMENA_A::_100)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMENA_A::_101)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMENA_A::_110)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMENA_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Timer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMDIS_A {
    #[doc = "0: Timer never disabled"]
    _0 = 0,
    #[doc = "1: Timer disabled on Timer N-1 disable"]
    _1 = 1,
    #[doc = "2: Timer disabled on Timer compare"]
    _10 = 2,
    #[doc = "3: Timer disabled on Timer compare and Trigger Low"]
    _11 = 3,
    #[doc = "4: Timer disabled on Pin rising or falling edge"]
    _100 = 4,
    #[doc = "5: Timer disabled on Pin rising or falling edge provided Trigger is high"]
    _101 = 5,
    #[doc = "6: Timer disabled on Trigger falling edge"]
    _110 = 6,
}
impl From<TIMDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMDIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMDIS` reader - Timer Disable"]
pub struct TIMDIS_R(crate::FieldReader<u8, TIMDIS_A>);
impl TIMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMDIS_A> {
        match self.bits {
            0 => Some(TIMDIS_A::_0),
            1 => Some(TIMDIS_A::_1),
            2 => Some(TIMDIS_A::_10),
            3 => Some(TIMDIS_A::_11),
            4 => Some(TIMDIS_A::_100),
            5 => Some(TIMDIS_A::_101),
            6 => Some(TIMDIS_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMDIS_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMDIS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMDIS_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == TIMDIS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == TIMDIS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == TIMDIS_A::_110
    }
}
impl core::ops::Deref for TIMDIS_R {
    type Target = crate::FieldReader<u8, TIMDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMDIS` writer - Timer Disable"]
pub struct TIMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMDIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer never disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMDIS_A::_0)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMDIS_A::_1)
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMDIS_A::_10)
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMDIS_A::_11)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMDIS_A::_100)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMDIS_A::_101)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMDIS_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMRST_A {
    #[doc = "0: Timer never reset"]
    _0 = 0,
    #[doc = "2: Timer reset on Timer Pin equal to Timer Output"]
    _10 = 2,
    #[doc = "3: Timer reset on Timer Trigger equal to Timer Output"]
    _11 = 3,
    #[doc = "4: Timer reset on Timer Pin rising edge"]
    _100 = 4,
    #[doc = "6: Timer reset on Trigger rising edge"]
    _110 = 6,
    #[doc = "7: Timer reset on Trigger rising or falling edge"]
    _111 = 7,
}
impl From<TIMRST_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMRST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMRST` reader - Timer Reset"]
pub struct TIMRST_R(crate::FieldReader<u8, TIMRST_A>);
impl TIMRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMRST_A> {
        match self.bits {
            0 => Some(TIMRST_A::_0),
            2 => Some(TIMRST_A::_10),
            3 => Some(TIMRST_A::_11),
            4 => Some(TIMRST_A::_100),
            6 => Some(TIMRST_A::_110),
            7 => Some(TIMRST_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMRST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMRST_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == TIMRST_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == TIMRST_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == TIMRST_A::_111
    }
}
impl core::ops::Deref for TIMRST_R {
    type Target = crate::FieldReader<u8, TIMRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub struct TIMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMRST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer never reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMRST_A::_0)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMRST_A::_10)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMRST_A::_11)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMRST_A::_100)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMRST_A::_110)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMRST_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Timer Decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMDEC_A {
    #[doc = "0: Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    _0 = 0,
    #[doc = "1: Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    _1 = 1,
    #[doc = "2: Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    _10 = 2,
    #[doc = "3: Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    _11 = 3,
}
impl From<TIMDEC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMDEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMDEC` reader - Timer Decrement"]
pub struct TIMDEC_R(crate::FieldReader<u8, TIMDEC_A>);
impl TIMDEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMDEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMDEC_A {
        match self.bits {
            0 => TIMDEC_A::_0,
            1 => TIMDEC_A::_1,
            2 => TIMDEC_A::_10,
            3 => TIMDEC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMDEC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMDEC_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMDEC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMDEC_A::_11
    }
}
impl core::ops::Deref for TIMDEC_R {
    type Target = crate::FieldReader<u8, TIMDEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMDEC` writer - Timer Decrement"]
pub struct TIMDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMDEC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMDEC_A::_0)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMDEC_A::_1)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMDEC_A::_10)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMDEC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Timer Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMOUT_A {
    #[doc = "0: Timer output is logic one when enabled and is not affected by timer reset"]
    _0 = 0,
    #[doc = "1: Timer output is logic zero when enabled and is not affected by timer reset"]
    _1 = 1,
    #[doc = "2: Timer output is logic one when enabled and on timer reset"]
    _10 = 2,
    #[doc = "3: Timer output is logic zero when enabled and on timer reset"]
    _11 = 3,
}
impl From<TIMOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMOUT` reader - Timer Output"]
pub struct TIMOUT_R(crate::FieldReader<u8, TIMOUT_A>);
impl TIMOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOUT_A {
        match self.bits {
            0 => TIMOUT_A::_0,
            1 => TIMOUT_A::_1,
            2 => TIMOUT_A::_10,
            3 => TIMOUT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMOUT_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMOUT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMOUT_A::_11
    }
}
impl core::ops::Deref for TIMOUT_R {
    type Target = crate::FieldReader<u8, TIMOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMOUT` writer - Timer Output"]
pub struct TIMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOUT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMOUT_A::_0)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMOUT_A::_1)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMOUT_A::_10)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMOUT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    pub fn timena(&self) -> TIMENA_R {
        TIMENA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    pub fn timdis(&self) -> TIMDIS_R {
        TIMDIS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    pub fn timrst(&self) -> TIMRST_R {
        TIMRST_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    pub fn timdec(&self) -> TIMDEC_R {
        TIMDEC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TSTOP_W {
        TSTOP_W { w: self }
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    pub fn timena(&mut self) -> TIMENA_W {
        TIMENA_W { w: self }
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    pub fn timdis(&mut self) -> TIMDIS_W {
        TIMDIS_W { w: self }
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    pub fn timrst(&mut self) -> TIMRST_W {
        TIMRST_W { w: self }
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    pub fn timdec(&mut self) -> TIMDEC_W {
        TIMDEC_W { w: self }
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg1](index.html) module"]
pub struct TIMCFG1_SPEC;
impl crate::RegisterSpec for TIMCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timcfg1::R](R) reader structure"]
impl crate::Readable for TIMCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timcfg1::W](W) writer structure"]
impl crate::Writable for TIMCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCFG1 to value 0"]
impl crate::Resettable for TIMCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
