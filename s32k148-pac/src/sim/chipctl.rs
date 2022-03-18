#[doc = "Register `CHIPCTL` reader"]
pub struct R(crate::R<CHIPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPCTL` writer"]
pub struct W(crate::W<CHIPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPCTL_SPEC>;
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
impl From<crate::W<CHIPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC interleave channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_INTERLEAVE_EN_A {
    #[doc = "0: Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    _0000 = 0,
    #[doc = "8: PTB14 to ADC1_SE9 and ADC0_SE9"]
    _1XXX = 8,
    #[doc = "4: PTB13 to ADC1_SE8 and ADC0_SE8"]
    X1XX = 4,
    #[doc = "2: PTB1 to ADC0_SE5 and ADC1_SE15"]
    XX1X = 2,
    #[doc = "1: PTB0 to ADC0_SE4 and ADC1_SE14"]
    XXX1 = 1,
}
impl From<ADC_INTERLEAVE_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_INTERLEAVE_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_INTERLEAVE_EN` reader - ADC interleave channel enable"]
pub struct ADC_INTERLEAVE_EN_R(crate::FieldReader<u8, ADC_INTERLEAVE_EN_A>);
impl ADC_INTERLEAVE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_INTERLEAVE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_INTERLEAVE_EN_A> {
        match self.bits {
            0 => Some(ADC_INTERLEAVE_EN_A::_0000),
            8 => Some(ADC_INTERLEAVE_EN_A::_1XXX),
            4 => Some(ADC_INTERLEAVE_EN_A::X1XX),
            2 => Some(ADC_INTERLEAVE_EN_A::XX1X),
            1 => Some(ADC_INTERLEAVE_EN_A::XXX1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1XXX`"]
    #[inline(always)]
    pub fn is_1xxx(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::_1XXX
    }
    #[doc = "Checks if the value of the field is `X1XX`"]
    #[inline(always)]
    pub fn is_x1xx(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::X1XX
    }
    #[doc = "Checks if the value of the field is `XX1X`"]
    #[inline(always)]
    pub fn is_xx1x(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::XX1X
    }
    #[doc = "Checks if the value of the field is `XXX1`"]
    #[inline(always)]
    pub fn is_xxx1(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::XXX1
    }
}
impl core::ops::Deref for ADC_INTERLEAVE_EN_R {
    type Target = crate::FieldReader<u8, ADC_INTERLEAVE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_INTERLEAVE_EN` writer - ADC interleave channel enable"]
pub struct ADC_INTERLEAVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INTERLEAVE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_INTERLEAVE_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::_0000)
    }
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"]
    #[inline(always)]
    pub fn _1xxx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::_1XXX)
    }
    #[doc = "PTB13 to ADC1_SE8 and ADC0_SE8"]
    #[inline(always)]
    pub fn x1xx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::X1XX)
    }
    #[doc = "PTB1 to ADC0_SE5 and ADC1_SE15"]
    #[inline(always)]
    pub fn xx1x(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::XX1X)
    }
    #[doc = "PTB0 to ADC0_SE4 and ADC1_SE14"]
    #[inline(always)]
    pub fn xxx1(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::XXX1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "CLKOUT Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: SCG CLKOUT"]
    _0000 = 0,
    #[doc = "2: SOSC DIV2 CLK"]
    _0010 = 2,
    #[doc = "4: SIRC DIV2 CLK"]
    _0100 = 4,
    #[doc = "5: For S32K148: QSPI_SFIF_CLK_HYP_PREMUX: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    _0101 = 5,
    #[doc = "6: FIRC DIV2 CLK"]
    _0110 = 6,
    #[doc = "7: HCLK"]
    _0111 = 7,
    #[doc = "8: For S32K14x: SPLL DIV2 CLK For S32K11x: Reserved"]
    _1000 = 8,
    #[doc = "9: BUS_CLK"]
    _1001 = 9,
    #[doc = "10: LPO128K_CLK"]
    _1010 = 10,
    #[doc = "11: For S32K148: QSPI_Module clock; For others: Reserved"]
    _1011 = 11,
    #[doc = "12: LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    _1100 = 12,
    #[doc = "13: For S32K148: QSPI_SFIF_CLK; For others: Reserved"]
    _1101 = 13,
    #[doc = "14: RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    _1110 = 14,
    #[doc = "15: For S32K148: QSPI_2xSFIF_CLK; For others: Reserved"]
    _1111 = 15,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT Select"]
pub struct CLKOUTSEL_R(crate::FieldReader<u8, CLKOUTSEL_A>);
impl CLKOUTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            0 => Some(CLKOUTSEL_A::_0000),
            2 => Some(CLKOUTSEL_A::_0010),
            4 => Some(CLKOUTSEL_A::_0100),
            5 => Some(CLKOUTSEL_A::_0101),
            6 => Some(CLKOUTSEL_A::_0110),
            7 => Some(CLKOUTSEL_A::_0111),
            8 => Some(CLKOUTSEL_A::_1000),
            9 => Some(CLKOUTSEL_A::_1001),
            10 => Some(CLKOUTSEL_A::_1010),
            11 => Some(CLKOUTSEL_A::_1011),
            12 => Some(CLKOUTSEL_A::_1100),
            13 => Some(CLKOUTSEL_A::_1101),
            14 => Some(CLKOUTSEL_A::_1110),
            15 => Some(CLKOUTSEL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == CLKOUTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == CLKOUTSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == CLKOUTSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == CLKOUTSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == CLKOUTSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == CLKOUTSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == CLKOUTSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == CLKOUTSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == CLKOUTSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == CLKOUTSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == CLKOUTSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == CLKOUTSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == CLKOUTSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == CLKOUTSEL_A::_1111
    }
}
impl core::ops::Deref for CLKOUTSEL_R {
    type Target = crate::FieldReader<u8, CLKOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT Select"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCG CLKOUT"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0000)
    }
    #[doc = "SOSC DIV2 CLK"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0010)
    }
    #[doc = "SIRC DIV2 CLK"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0100)
    }
    #[doc = "For S32K148: QSPI_SFIF_CLK_HYP_PREMUX: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0101)
    }
    #[doc = "FIRC DIV2 CLK"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0110)
    }
    #[doc = "HCLK"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0111)
    }
    #[doc = "For S32K14x: SPLL DIV2 CLK For S32K11x: Reserved"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1000)
    }
    #[doc = "BUS_CLK"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1001)
    }
    #[doc = "LPO128K_CLK"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1010)
    }
    #[doc = "For S32K148: QSPI_Module clock; For others: Reserved"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1011)
    }
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1100)
    }
    #[doc = "For S32K148: QSPI_SFIF_CLK; For others: Reserved"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1101)
    }
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1110)
    }
    #[doc = "For S32K148: QSPI_2xSFIF_CLK; For others: Reserved"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "CLKOUT Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTDIV_A {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 3"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 5"]
    _100 = 4,
    #[doc = "5: Divide by 6"]
    _101 = 5,
    #[doc = "6: Divide by 7"]
    _110 = 6,
    #[doc = "7: Divide by 8"]
    _111 = 7,
}
impl From<CLKOUTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTDIV` reader - CLKOUT Divide Ratio"]
pub struct CLKOUTDIV_R(crate::FieldReader<u8, CLKOUTDIV_A>);
impl CLKOUTDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUTDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTDIV_A {
        match self.bits {
            0 => CLKOUTDIV_A::_000,
            1 => CLKOUTDIV_A::_001,
            2 => CLKOUTDIV_A::_010,
            3 => CLKOUTDIV_A::_011,
            4 => CLKOUTDIV_A::_100,
            5 => CLKOUTDIV_A::_101,
            6 => CLKOUTDIV_A::_110,
            7 => CLKOUTDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == CLKOUTDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == CLKOUTDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == CLKOUTDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == CLKOUTDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == CLKOUTDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == CLKOUTDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == CLKOUTDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == CLKOUTDIV_A::_111
    }
}
impl core::ops::Deref for CLKOUTDIV_R {
    type Target = crate::FieldReader<u8, CLKOUTDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUTDIV` writer - CLKOUT Divide Ratio"]
pub struct CLKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_001)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_011)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_100)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_101)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_110)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "CLKOUT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTEN_A {
    #[doc = "0: Clockout disable"]
    _0 = 0,
    #[doc = "1: Clockout enable"]
    _1 = 1,
}
impl From<CLKOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOUTEN` reader - CLKOUT enable"]
pub struct CLKOUTEN_R(crate::FieldReader<bool, CLKOUTEN_A>);
impl CLKOUTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTEN_A {
        match self.bits {
            false => CLKOUTEN_A::_0,
            true => CLKOUTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKOUTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKOUTEN_A::_1
    }
}
impl core::ops::Deref for CLKOUTEN_R {
    type Target = crate::FieldReader<bool, CLKOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUTEN` writer - CLKOUT enable"]
pub struct CLKOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clockout disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::_0)
    }
    #[doc = "Clockout enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Debug trace clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLK_SEL_A {
    #[doc = "0: Core clock"]
    _0 = 0,
}
impl From<TRACECLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACECLK_SEL` reader - Debug trace clock select"]
pub struct TRACECLK_SEL_R(crate::FieldReader<bool, TRACECLK_SEL_A>);
impl TRACECLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRACECLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRACECLK_SEL_A> {
        match self.bits {
            false => Some(TRACECLK_SEL_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRACECLK_SEL_A::_0
    }
}
impl core::ops::Deref for TRACECLK_SEL_R {
    type Target = crate::FieldReader<bool, TRACECLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACECLK_SEL` writer - Debug trace clock select"]
pub struct TRACECLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECLK_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Core clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLK_SEL_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "PDB back-to-back select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDB_BB_SEL_A {
    #[doc = "0: PDB0 channel 0 back-to-back operation with ADC0 COCO\\[7:0\\]
and PDB1 channel 0 back-to-back operation with ADC1 COCO\\[7:0\\]"]
    _0 = 0,
    #[doc = "1: Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    _1 = 1,
}
impl From<PDB_BB_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_BB_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDB_BB_SEL` reader - PDB back-to-back select"]
pub struct PDB_BB_SEL_R(crate::FieldReader<bool, PDB_BB_SEL_A>);
impl PDB_BB_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDB_BB_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_BB_SEL_A {
        match self.bits {
            false => PDB_BB_SEL_A::_0,
            true => PDB_BB_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDB_BB_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDB_BB_SEL_A::_1
    }
}
impl core::ops::Deref for PDB_BB_SEL_R {
    type Target = crate::FieldReader<bool, PDB_BB_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDB_BB_SEL` writer - PDB back-to-back select"]
pub struct PDB_BB_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_BB_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDB_BB_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDB0 channel 0 back-to-back operation with ADC0 COCO\\[7:0\\]
and PDB1 channel 0 back-to-back operation with ADC1 COCO\\[7:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::_0)
    }
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "ADC_SUPPLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SUPPLY_A {
    #[doc = "0: 5 V input VDD supply (VDD)"]
    _000 = 0,
    #[doc = "1: 5 V input analog supply (VDDA)"]
    _001 = 1,
    #[doc = "2: ADC Reference Supply (VREFH)"]
    _010 = 2,
    #[doc = "3: 3.3 V Oscillator Regulator Output (VDD_3V)"]
    _011 = 3,
    #[doc = "4: 3.3 V flash regulator output (VDD_flash_3V)"]
    _100 = 4,
    #[doc = "5: 1.2 V core regulator output (VDD_LV)"]
    _101 = 5,
}
impl From<ADC_SUPPLY_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SUPPLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_SUPPLY` reader - ADC_SUPPLY"]
pub struct ADC_SUPPLY_R(crate::FieldReader<u8, ADC_SUPPLY_A>);
impl ADC_SUPPLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_SUPPLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_SUPPLY_A> {
        match self.bits {
            0 => Some(ADC_SUPPLY_A::_000),
            1 => Some(ADC_SUPPLY_A::_001),
            2 => Some(ADC_SUPPLY_A::_010),
            3 => Some(ADC_SUPPLY_A::_011),
            4 => Some(ADC_SUPPLY_A::_100),
            5 => Some(ADC_SUPPLY_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == ADC_SUPPLY_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == ADC_SUPPLY_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == ADC_SUPPLY_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == ADC_SUPPLY_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == ADC_SUPPLY_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == ADC_SUPPLY_A::_101
    }
}
impl core::ops::Deref for ADC_SUPPLY_R {
    type Target = crate::FieldReader<u8, ADC_SUPPLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_SUPPLY` writer - ADC_SUPPLY"]
pub struct ADC_SUPPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SUPPLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SUPPLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "5 V input VDD supply (VDD)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_000)
    }
    #[doc = "5 V input analog supply (VDDA)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_001)
    }
    #[doc = "ADC Reference Supply (VREFH)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_010)
    }
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_011)
    }
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_100)
    }
    #[doc = "1.2 V core regulator output (VDD_LV)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "ADC_SUPPLYEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SUPPLYEN_A {
    #[doc = "0: Disable internal supply monitoring"]
    _0 = 0,
    #[doc = "1: Enable internal supply monitoring"]
    _1 = 1,
}
impl From<ADC_SUPPLYEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_SUPPLYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_SUPPLYEN` reader - ADC_SUPPLYEN"]
pub struct ADC_SUPPLYEN_R(crate::FieldReader<bool, ADC_SUPPLYEN_A>);
impl ADC_SUPPLYEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SUPPLYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SUPPLYEN_A {
        match self.bits {
            false => ADC_SUPPLYEN_A::_0,
            true => ADC_SUPPLYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADC_SUPPLYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADC_SUPPLYEN_A::_1
    }
}
impl core::ops::Deref for ADC_SUPPLYEN_R {
    type Target = crate::FieldReader<bool, ADC_SUPPLYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_SUPPLYEN` writer - ADC_SUPPLYEN"]
pub struct ADC_SUPPLYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SUPPLYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SUPPLYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable internal supply monitoring"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::_0)
    }
    #[doc = "Enable internal supply monitoring"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "SRAMU_RETEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMU_RETEN_A {
    #[doc = "0: SRAMU contents are retained across resets"]
    _0 = 0,
    #[doc = "1: No SRAMU retention"]
    _1 = 1,
}
impl From<SRAMU_RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMU_RETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMU_RETEN` reader - SRAMU_RETEN"]
pub struct SRAMU_RETEN_R(crate::FieldReader<bool, SRAMU_RETEN_A>);
impl SRAMU_RETEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAMU_RETEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMU_RETEN_A {
        match self.bits {
            false => SRAMU_RETEN_A::_0,
            true => SRAMU_RETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRAMU_RETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRAMU_RETEN_A::_1
    }
}
impl core::ops::Deref for SRAMU_RETEN_R {
    type Target = crate::FieldReader<bool, SRAMU_RETEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMU_RETEN` writer - SRAMU_RETEN"]
pub struct SRAMU_RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMU_RETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMU_RETEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMU contents are retained across resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::_0)
    }
    #[doc = "No SRAMU retention"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "SRAML_RETEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAML_RETEN_A {
    #[doc = "0: SRAML contents are retained across resets"]
    _0 = 0,
    #[doc = "1: No SRAML retention"]
    _1 = 1,
}
impl From<SRAML_RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAML_RETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAML_RETEN` reader - SRAML_RETEN"]
pub struct SRAML_RETEN_R(crate::FieldReader<bool, SRAML_RETEN_A>);
impl SRAML_RETEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAML_RETEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAML_RETEN_A {
        match self.bits {
            false => SRAML_RETEN_A::_0,
            true => SRAML_RETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRAML_RETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRAML_RETEN_A::_1
    }
}
impl core::ops::Deref for SRAML_RETEN_R {
    type Target = crate::FieldReader<bool, SRAML_RETEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAML_RETEN` writer - SRAML_RETEN"]
pub struct SRAML_RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAML_RETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAML_RETEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAML contents are retained across resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::_0)
    }
    #[doc = "No SRAML retention"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline(always)]
    pub fn adc_interleave_en(&self) -> ADC_INTERLEAVE_EN_R {
        ADC_INTERLEAVE_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline(always)]
    pub fn clkouten(&self) -> CLKOUTEN_R {
        CLKOUTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclk_sel(&self) -> TRACECLK_SEL_R {
        TRACECLK_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline(always)]
    pub fn pdb_bb_sel(&self) -> PDB_BB_SEL_R {
        PDB_BB_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline(always)]
    pub fn adc_supply(&self) -> ADC_SUPPLY_R {
        ADC_SUPPLY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline(always)]
    pub fn adc_supplyen(&self) -> ADC_SUPPLYEN_R {
        ADC_SUPPLYEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline(always)]
    pub fn sramu_reten(&self) -> SRAMU_RETEN_R {
        SRAMU_RETEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline(always)]
    pub fn sraml_reten(&self) -> SRAML_RETEN_R {
        SRAML_RETEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline(always)]
    pub fn adc_interleave_en(&mut self) -> ADC_INTERLEAVE_EN_W {
        ADC_INTERLEAVE_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W {
        CLKOUTDIV_W { w: self }
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline(always)]
    pub fn clkouten(&mut self) -> CLKOUTEN_W {
        CLKOUTEN_W { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclk_sel(&mut self) -> TRACECLK_SEL_W {
        TRACECLK_SEL_W { w: self }
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline(always)]
    pub fn pdb_bb_sel(&mut self) -> PDB_BB_SEL_W {
        PDB_BB_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline(always)]
    pub fn adc_supply(&mut self) -> ADC_SUPPLY_W {
        ADC_SUPPLY_W { w: self }
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline(always)]
    pub fn adc_supplyen(&mut self) -> ADC_SUPPLYEN_W {
        ADC_SUPPLYEN_W { w: self }
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline(always)]
    pub fn sramu_reten(&mut self) -> SRAMU_RETEN_W {
        SRAMU_RETEN_W { w: self }
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline(always)]
    pub fn sraml_reten(&mut self) -> SRAML_RETEN_W {
        SRAML_RETEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipctl](index.html) module"]
pub struct CHIPCTL_SPEC;
impl crate::RegisterSpec for CHIPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chipctl::R](R) reader structure"]
impl crate::Readable for CHIPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chipctl::W](W) writer structure"]
impl crate::Writable for CHIPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIPCTL to value 0x0030_0000"]
impl crate::Resettable for CHIPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0030_0000
    }
}
