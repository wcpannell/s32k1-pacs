#[doc = "Register `LPOCLKS` reader"]
pub struct R(crate::R<LPOCLKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOCLKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOCLKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOCLKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPOCLKS` writer"]
pub struct W(crate::W<LPOCLKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPOCLKS_SPEC>;
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
impl From<crate::W<LPOCLKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPOCLKS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1 kHz LPO_CLK enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO1KCLKEN_A {
    #[doc = "0: Disable 1 kHz LPO_CLK output"]
    LPO1KCLKEN_0 = 0,
    #[doc = "1: Enable 1 kHz LPO_CLK output"]
    LPO1KCLKEN_1 = 1,
}
impl From<LPO1KCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPO1KCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPO1KCLKEN` reader - 1 kHz LPO_CLK enable"]
pub struct LPO1KCLKEN_R(crate::FieldReader<bool, LPO1KCLKEN_A>);
impl LPO1KCLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPO1KCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPO1KCLKEN_A {
        match self.bits {
            false => LPO1KCLKEN_A::LPO1KCLKEN_0,
            true => LPO1KCLKEN_A::LPO1KCLKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPO1KCLKEN_0`"]
    #[inline(always)]
    pub fn is_lpo1kclken_0(&self) -> bool {
        **self == LPO1KCLKEN_A::LPO1KCLKEN_0
    }
    #[doc = "Checks if the value of the field is `LPO1KCLKEN_1`"]
    #[inline(always)]
    pub fn is_lpo1kclken_1(&self) -> bool {
        **self == LPO1KCLKEN_A::LPO1KCLKEN_1
    }
}
impl core::ops::Deref for LPO1KCLKEN_R {
    type Target = crate::FieldReader<bool, LPO1KCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPO1KCLKEN` writer - 1 kHz LPO_CLK enable"]
pub struct LPO1KCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO1KCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPO1KCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable 1 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn lpo1kclken_0(self) -> &'a mut W {
        self.variant(LPO1KCLKEN_A::LPO1KCLKEN_0)
    }
    #[doc = "Enable 1 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn lpo1kclken_1(self) -> &'a mut W {
        self.variant(LPO1KCLKEN_A::LPO1KCLKEN_1)
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
#[doc = "32 kHz LPO_CLK enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO32KCLKEN_A {
    #[doc = "0: Disable 32 kHz LPO_CLK output"]
    LPO32KCLKEN_0 = 0,
    #[doc = "1: Enable 32 kHz LPO_CLK output"]
    LPO32KCLKEN_1 = 1,
}
impl From<LPO32KCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPO32KCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPO32KCLKEN` reader - 32 kHz LPO_CLK enable"]
pub struct LPO32KCLKEN_R(crate::FieldReader<bool, LPO32KCLKEN_A>);
impl LPO32KCLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPO32KCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPO32KCLKEN_A {
        match self.bits {
            false => LPO32KCLKEN_A::LPO32KCLKEN_0,
            true => LPO32KCLKEN_A::LPO32KCLKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPO32KCLKEN_0`"]
    #[inline(always)]
    pub fn is_lpo32kclken_0(&self) -> bool {
        **self == LPO32KCLKEN_A::LPO32KCLKEN_0
    }
    #[doc = "Checks if the value of the field is `LPO32KCLKEN_1`"]
    #[inline(always)]
    pub fn is_lpo32kclken_1(&self) -> bool {
        **self == LPO32KCLKEN_A::LPO32KCLKEN_1
    }
}
impl core::ops::Deref for LPO32KCLKEN_R {
    type Target = crate::FieldReader<bool, LPO32KCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPO32KCLKEN` writer - 32 kHz LPO_CLK enable"]
pub struct LPO32KCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO32KCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPO32KCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable 32 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn lpo32kclken_0(self) -> &'a mut W {
        self.variant(LPO32KCLKEN_A::LPO32KCLKEN_0)
    }
    #[doc = "Enable 32 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn lpo32kclken_1(self) -> &'a mut W {
        self.variant(LPO32KCLKEN_A::LPO32KCLKEN_1)
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
#[doc = "LPO clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPOCLKSEL_A {
    #[doc = "0: 128 kHz LPO_CLK"]
    LPOCLKSEL_0 = 0,
    #[doc = "1: No clock"]
    LPOCLKSEL_1 = 1,
    #[doc = "2: 32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    LPOCLKSEL_2 = 2,
    #[doc = "3: 1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    LPOCLKSEL_3 = 3,
}
impl From<LPOCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPOCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPOCLKSEL` reader - LPO clock source select"]
pub struct LPOCLKSEL_R(crate::FieldReader<u8, LPOCLKSEL_A>);
impl LPOCLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPOCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOCLKSEL_A {
        match self.bits {
            0 => LPOCLKSEL_A::LPOCLKSEL_0,
            1 => LPOCLKSEL_A::LPOCLKSEL_1,
            2 => LPOCLKSEL_A::LPOCLKSEL_2,
            3 => LPOCLKSEL_A::LPOCLKSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPOCLKSEL_0`"]
    #[inline(always)]
    pub fn is_lpoclksel_0(&self) -> bool {
        **self == LPOCLKSEL_A::LPOCLKSEL_0
    }
    #[doc = "Checks if the value of the field is `LPOCLKSEL_1`"]
    #[inline(always)]
    pub fn is_lpoclksel_1(&self) -> bool {
        **self == LPOCLKSEL_A::LPOCLKSEL_1
    }
    #[doc = "Checks if the value of the field is `LPOCLKSEL_2`"]
    #[inline(always)]
    pub fn is_lpoclksel_2(&self) -> bool {
        **self == LPOCLKSEL_A::LPOCLKSEL_2
    }
    #[doc = "Checks if the value of the field is `LPOCLKSEL_3`"]
    #[inline(always)]
    pub fn is_lpoclksel_3(&self) -> bool {
        **self == LPOCLKSEL_A::LPOCLKSEL_3
    }
}
impl core::ops::Deref for LPOCLKSEL_R {
    type Target = crate::FieldReader<u8, LPOCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOCLKSEL` writer - LPO clock source select"]
pub struct LPOCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn lpoclksel_0(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::LPOCLKSEL_0)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn lpoclksel_1(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::LPOCLKSEL_1)
    }
    #[doc = "32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn lpoclksel_2(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::LPOCLKSEL_2)
    }
    #[doc = "1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn lpoclksel_3(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::LPOCLKSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "32 kHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCLKSEL_A {
    #[doc = "0: SOSCDIV1_CLK"]
    RTCCLKSEL_0 = 0,
    #[doc = "1: 32 kHz LPO_CLK"]
    RTCCLKSEL_1 = 1,
    #[doc = "2: 32 kHz RTC_CLKIN clock"]
    RTCCLKSEL_2 = 2,
    #[doc = "3: FIRCDIV1_CLK"]
    RTCCLKSEL_3 = 3,
}
impl From<RTCCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCCLKSEL` reader - 32 kHz clock source select"]
pub struct RTCCLKSEL_R(crate::FieldReader<u8, RTCCLKSEL_A>);
impl RTCCLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKSEL_A {
        match self.bits {
            0 => RTCCLKSEL_A::RTCCLKSEL_0,
            1 => RTCCLKSEL_A::RTCCLKSEL_1,
            2 => RTCCLKSEL_A::RTCCLKSEL_2,
            3 => RTCCLKSEL_A::RTCCLKSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCLKSEL_0`"]
    #[inline(always)]
    pub fn is_rtcclksel_0(&self) -> bool {
        **self == RTCCLKSEL_A::RTCCLKSEL_0
    }
    #[doc = "Checks if the value of the field is `RTCCLKSEL_1`"]
    #[inline(always)]
    pub fn is_rtcclksel_1(&self) -> bool {
        **self == RTCCLKSEL_A::RTCCLKSEL_1
    }
    #[doc = "Checks if the value of the field is `RTCCLKSEL_2`"]
    #[inline(always)]
    pub fn is_rtcclksel_2(&self) -> bool {
        **self == RTCCLKSEL_A::RTCCLKSEL_2
    }
    #[doc = "Checks if the value of the field is `RTCCLKSEL_3`"]
    #[inline(always)]
    pub fn is_rtcclksel_3(&self) -> bool {
        **self == RTCCLKSEL_A::RTCCLKSEL_3
    }
}
impl core::ops::Deref for RTCCLKSEL_R {
    type Target = crate::FieldReader<u8, RTCCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCLKSEL` writer - 32 kHz clock source select"]
pub struct RTCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SOSCDIV1_CLK"]
    #[inline(always)]
    pub fn rtcclksel_0(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::RTCCLKSEL_0)
    }
    #[doc = "32 kHz LPO_CLK"]
    #[inline(always)]
    pub fn rtcclksel_1(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::RTCCLKSEL_1)
    }
    #[doc = "32 kHz RTC_CLKIN clock"]
    #[inline(always)]
    pub fn rtcclksel_2(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::RTCCLKSEL_2)
    }
    #[doc = "FIRCDIV1_CLK"]
    #[inline(always)]
    pub fn rtcclksel_3(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::RTCCLKSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo1kclken(&self) -> LPO1KCLKEN_R {
        LPO1KCLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo32kclken(&self) -> LPO32KCLKEN_R {
        LPO32KCLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline(always)]
    pub fn lpoclksel(&self) -> LPOCLKSEL_R {
        LPOCLKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline(always)]
    pub fn rtcclksel(&self) -> RTCCLKSEL_R {
        RTCCLKSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo1kclken(&mut self) -> LPO1KCLKEN_W {
        LPO1KCLKEN_W { w: self }
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo32kclken(&mut self) -> LPO32KCLKEN_W {
        LPO32KCLKEN_W { w: self }
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline(always)]
    pub fn lpoclksel(&mut self) -> LPOCLKSEL_W {
        LPOCLKSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline(always)]
    pub fn rtcclksel(&mut self) -> RTCCLKSEL_W {
        RTCCLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPO Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpoclks](index.html) module"]
pub struct LPOCLKS_SPEC;
impl crate::RegisterSpec for LPOCLKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpoclks::R](R) reader structure"]
impl crate::Readable for LPOCLKS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpoclks::W](W) writer structure"]
impl crate::Writable for LPOCLKS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPOCLKS to value 0x03"]
impl crate::Resettable for LPOCLKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
