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
    ADC_INTERLEAVE_EN_0 = 0,
    #[doc = "8: PTB14 to ADC1_SE9 and ADC0_SE9"]
    ADC_INTERLEAVE_EN_8 = 8,
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
            0 => Some(ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_0),
            8 => Some(ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_INTERLEAVE_EN_0`"]
    #[inline(always)]
    pub fn is_adc_interleave_en_0(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_0
    }
    #[doc = "Checks if the value of the field is `ADC_INTERLEAVE_EN_8`"]
    #[inline(always)]
    pub fn is_adc_interleave_en_8(&self) -> bool {
        **self == ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_8
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
    pub fn adc_interleave_en_0(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_0)
    }
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"]
    #[inline(always)]
    pub fn adc_interleave_en_8(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::ADC_INTERLEAVE_EN_8)
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
    CLKOUTSEL_0 = 0,
    #[doc = "2: SOSC DIV2 CLK"]
    CLKOUTSEL_2 = 2,
    #[doc = "4: SIRC DIV2 CLK"]
    CLKOUTSEL_4 = 4,
    #[doc = "5: For S32K148: QSPI_SFIF_CLK_HYP_PREMUX: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    CLKOUTSEL_5 = 5,
    #[doc = "6: FIRC DIV2 CLK"]
    CLKOUTSEL_6 = 6,
    #[doc = "7: HCLK"]
    CLKOUTSEL_7 = 7,
    #[doc = "8: For S32K14x: SPLL DIV2 CLK For S32K11x: Reserved"]
    CLKOUTSEL_8 = 8,
    #[doc = "9: BUS_CLK"]
    CLKOUTSEL_9 = 9,
    #[doc = "10: LPO128K_CLK"]
    CLKOUTSEL_10 = 10,
    #[doc = "11: For S32K148: QSPI_Module clock; For others: Reserved"]
    CLKOUTSEL_11 = 11,
    #[doc = "12: LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    CLKOUTSEL_12 = 12,
    #[doc = "13: For S32K148: QSPI_SFIF_CLK; For others: Reserved"]
    CLKOUTSEL_13 = 13,
    #[doc = "14: RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    CLKOUTSEL_14 = 14,
    #[doc = "15: For S32K148: QSPI_2xSFIF_CLK; For others: Reserved"]
    CLKOUTSEL_15 = 15,
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
            0 => Some(CLKOUTSEL_A::CLKOUTSEL_0),
            2 => Some(CLKOUTSEL_A::CLKOUTSEL_2),
            4 => Some(CLKOUTSEL_A::CLKOUTSEL_4),
            5 => Some(CLKOUTSEL_A::CLKOUTSEL_5),
            6 => Some(CLKOUTSEL_A::CLKOUTSEL_6),
            7 => Some(CLKOUTSEL_A::CLKOUTSEL_7),
            8 => Some(CLKOUTSEL_A::CLKOUTSEL_8),
            9 => Some(CLKOUTSEL_A::CLKOUTSEL_9),
            10 => Some(CLKOUTSEL_A::CLKOUTSEL_10),
            11 => Some(CLKOUTSEL_A::CLKOUTSEL_11),
            12 => Some(CLKOUTSEL_A::CLKOUTSEL_12),
            13 => Some(CLKOUTSEL_A::CLKOUTSEL_13),
            14 => Some(CLKOUTSEL_A::CLKOUTSEL_14),
            15 => Some(CLKOUTSEL_A::CLKOUTSEL_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_0`"]
    #[inline(always)]
    pub fn is_clkoutsel_0(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_0
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_2`"]
    #[inline(always)]
    pub fn is_clkoutsel_2(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_2
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_4`"]
    #[inline(always)]
    pub fn is_clkoutsel_4(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_4
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_5`"]
    #[inline(always)]
    pub fn is_clkoutsel_5(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_5
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_6`"]
    #[inline(always)]
    pub fn is_clkoutsel_6(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_6
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_7`"]
    #[inline(always)]
    pub fn is_clkoutsel_7(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_7
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_8`"]
    #[inline(always)]
    pub fn is_clkoutsel_8(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_8
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_9`"]
    #[inline(always)]
    pub fn is_clkoutsel_9(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_9
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_10`"]
    #[inline(always)]
    pub fn is_clkoutsel_10(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_10
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_11`"]
    #[inline(always)]
    pub fn is_clkoutsel_11(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_11
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_12`"]
    #[inline(always)]
    pub fn is_clkoutsel_12(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_12
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_13`"]
    #[inline(always)]
    pub fn is_clkoutsel_13(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_13
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_14`"]
    #[inline(always)]
    pub fn is_clkoutsel_14(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_14
    }
    #[doc = "Checks if the value of the field is `CLKOUTSEL_15`"]
    #[inline(always)]
    pub fn is_clkoutsel_15(&self) -> bool {
        **self == CLKOUTSEL_A::CLKOUTSEL_15
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
    pub fn clkoutsel_0(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_0)
    }
    #[doc = "SOSC DIV2 CLK"]
    #[inline(always)]
    pub fn clkoutsel_2(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_2)
    }
    #[doc = "SIRC DIV2 CLK"]
    #[inline(always)]
    pub fn clkoutsel_4(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_4)
    }
    #[doc = "For S32K148: QSPI_SFIF_CLK_HYP_PREMUX: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    #[inline(always)]
    pub fn clkoutsel_5(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_5)
    }
    #[doc = "FIRC DIV2 CLK"]
    #[inline(always)]
    pub fn clkoutsel_6(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_6)
    }
    #[doc = "HCLK"]
    #[inline(always)]
    pub fn clkoutsel_7(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_7)
    }
    #[doc = "For S32K14x: SPLL DIV2 CLK For S32K11x: Reserved"]
    #[inline(always)]
    pub fn clkoutsel_8(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_8)
    }
    #[doc = "BUS_CLK"]
    #[inline(always)]
    pub fn clkoutsel_9(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_9)
    }
    #[doc = "LPO128K_CLK"]
    #[inline(always)]
    pub fn clkoutsel_10(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_10)
    }
    #[doc = "For S32K148: QSPI_Module clock; For others: Reserved"]
    #[inline(always)]
    pub fn clkoutsel_11(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_11)
    }
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    #[inline(always)]
    pub fn clkoutsel_12(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_12)
    }
    #[doc = "For S32K148: QSPI_SFIF_CLK; For others: Reserved"]
    #[inline(always)]
    pub fn clkoutsel_13(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_13)
    }
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    #[inline(always)]
    pub fn clkoutsel_14(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_14)
    }
    #[doc = "For S32K148: QSPI_2xSFIF_CLK; For others: Reserved"]
    #[inline(always)]
    pub fn clkoutsel_15(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::CLKOUTSEL_15)
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
    CLKOUTDIV_0 = 0,
    #[doc = "1: Divide by 2"]
    CLKOUTDIV_1 = 1,
    #[doc = "2: Divide by 3"]
    CLKOUTDIV_2 = 2,
    #[doc = "3: Divide by 4"]
    CLKOUTDIV_3 = 3,
    #[doc = "4: Divide by 5"]
    CLKOUTDIV_4 = 4,
    #[doc = "5: Divide by 6"]
    CLKOUTDIV_5 = 5,
    #[doc = "6: Divide by 7"]
    CLKOUTDIV_6 = 6,
    #[doc = "7: Divide by 8"]
    CLKOUTDIV_7 = 7,
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
            0 => CLKOUTDIV_A::CLKOUTDIV_0,
            1 => CLKOUTDIV_A::CLKOUTDIV_1,
            2 => CLKOUTDIV_A::CLKOUTDIV_2,
            3 => CLKOUTDIV_A::CLKOUTDIV_3,
            4 => CLKOUTDIV_A::CLKOUTDIV_4,
            5 => CLKOUTDIV_A::CLKOUTDIV_5,
            6 => CLKOUTDIV_A::CLKOUTDIV_6,
            7 => CLKOUTDIV_A::CLKOUTDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_0`"]
    #[inline(always)]
    pub fn is_clkoutdiv_0(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_0
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_1`"]
    #[inline(always)]
    pub fn is_clkoutdiv_1(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_1
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_2`"]
    #[inline(always)]
    pub fn is_clkoutdiv_2(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_2
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_3`"]
    #[inline(always)]
    pub fn is_clkoutdiv_3(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_3
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_4`"]
    #[inline(always)]
    pub fn is_clkoutdiv_4(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_4
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_5`"]
    #[inline(always)]
    pub fn is_clkoutdiv_5(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_5
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_6`"]
    #[inline(always)]
    pub fn is_clkoutdiv_6(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_6
    }
    #[doc = "Checks if the value of the field is `CLKOUTDIV_7`"]
    #[inline(always)]
    pub fn is_clkoutdiv_7(&self) -> bool {
        **self == CLKOUTDIV_A::CLKOUTDIV_7
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
    pub fn clkoutdiv_0(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn clkoutdiv_1(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn clkoutdiv_2(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn clkoutdiv_3(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn clkoutdiv_4(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn clkoutdiv_5(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn clkoutdiv_6(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn clkoutdiv_7(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::CLKOUTDIV_7)
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
    CLKOUTEN_0 = 0,
    #[doc = "1: Clockout enable"]
    CLKOUTEN_1 = 1,
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
            false => CLKOUTEN_A::CLKOUTEN_0,
            true => CLKOUTEN_A::CLKOUTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUTEN_0`"]
    #[inline(always)]
    pub fn is_clkouten_0(&self) -> bool {
        **self == CLKOUTEN_A::CLKOUTEN_0
    }
    #[doc = "Checks if the value of the field is `CLKOUTEN_1`"]
    #[inline(always)]
    pub fn is_clkouten_1(&self) -> bool {
        **self == CLKOUTEN_A::CLKOUTEN_1
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
    pub fn clkouten_0(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::CLKOUTEN_0)
    }
    #[doc = "Clockout enable"]
    #[inline(always)]
    pub fn clkouten_1(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::CLKOUTEN_1)
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
    TRACECLK_SEL_0 = 0,
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
            false => Some(TRACECLK_SEL_A::TRACECLK_SEL_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRACECLK_SEL_0`"]
    #[inline(always)]
    pub fn is_traceclk_sel_0(&self) -> bool {
        **self == TRACECLK_SEL_A::TRACECLK_SEL_0
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
    pub fn traceclk_sel_0(self) -> &'a mut W {
        self.variant(TRACECLK_SEL_A::TRACECLK_SEL_0)
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
    PDB_BB_SEL_0 = 0,
    #[doc = "1: Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    PDB_BB_SEL_1 = 1,
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
            false => PDB_BB_SEL_A::PDB_BB_SEL_0,
            true => PDB_BB_SEL_A::PDB_BB_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDB_BB_SEL_0`"]
    #[inline(always)]
    pub fn is_pdb_bb_sel_0(&self) -> bool {
        **self == PDB_BB_SEL_A::PDB_BB_SEL_0
    }
    #[doc = "Checks if the value of the field is `PDB_BB_SEL_1`"]
    #[inline(always)]
    pub fn is_pdb_bb_sel_1(&self) -> bool {
        **self == PDB_BB_SEL_A::PDB_BB_SEL_1
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
    pub fn pdb_bb_sel_0(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::PDB_BB_SEL_0)
    }
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    #[inline(always)]
    pub fn pdb_bb_sel_1(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::PDB_BB_SEL_1)
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
    ADC_SUPPLY_0 = 0,
    #[doc = "1: 5 V input analog supply (VDDA)"]
    ADC_SUPPLY_1 = 1,
    #[doc = "2: ADC Reference Supply (VREFH)"]
    ADC_SUPPLY_2 = 2,
    #[doc = "3: 3.3 V Oscillator Regulator Output (VDD_3V)"]
    ADC_SUPPLY_3 = 3,
    #[doc = "4: 3.3 V flash regulator output (VDD_flash_3V)"]
    ADC_SUPPLY_4 = 4,
    #[doc = "5: 1.2 V core regulator output (VDD_LV)"]
    ADC_SUPPLY_5 = 5,
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
            0 => Some(ADC_SUPPLY_A::ADC_SUPPLY_0),
            1 => Some(ADC_SUPPLY_A::ADC_SUPPLY_1),
            2 => Some(ADC_SUPPLY_A::ADC_SUPPLY_2),
            3 => Some(ADC_SUPPLY_A::ADC_SUPPLY_3),
            4 => Some(ADC_SUPPLY_A::ADC_SUPPLY_4),
            5 => Some(ADC_SUPPLY_A::ADC_SUPPLY_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_0`"]
    #[inline(always)]
    pub fn is_adc_supply_0(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_0
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_1`"]
    #[inline(always)]
    pub fn is_adc_supply_1(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_1
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_2`"]
    #[inline(always)]
    pub fn is_adc_supply_2(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_2
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_3`"]
    #[inline(always)]
    pub fn is_adc_supply_3(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_3
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_4`"]
    #[inline(always)]
    pub fn is_adc_supply_4(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_4
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLY_5`"]
    #[inline(always)]
    pub fn is_adc_supply_5(&self) -> bool {
        **self == ADC_SUPPLY_A::ADC_SUPPLY_5
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
    pub fn adc_supply_0(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_0)
    }
    #[doc = "5 V input analog supply (VDDA)"]
    #[inline(always)]
    pub fn adc_supply_1(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_1)
    }
    #[doc = "ADC Reference Supply (VREFH)"]
    #[inline(always)]
    pub fn adc_supply_2(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_2)
    }
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"]
    #[inline(always)]
    pub fn adc_supply_3(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_3)
    }
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"]
    #[inline(always)]
    pub fn adc_supply_4(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_4)
    }
    #[doc = "1.2 V core regulator output (VDD_LV)"]
    #[inline(always)]
    pub fn adc_supply_5(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::ADC_SUPPLY_5)
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
    ADC_SUPPLYEN_0 = 0,
    #[doc = "1: Enable internal supply monitoring"]
    ADC_SUPPLYEN_1 = 1,
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
            false => ADC_SUPPLYEN_A::ADC_SUPPLYEN_0,
            true => ADC_SUPPLYEN_A::ADC_SUPPLYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLYEN_0`"]
    #[inline(always)]
    pub fn is_adc_supplyen_0(&self) -> bool {
        **self == ADC_SUPPLYEN_A::ADC_SUPPLYEN_0
    }
    #[doc = "Checks if the value of the field is `ADC_SUPPLYEN_1`"]
    #[inline(always)]
    pub fn is_adc_supplyen_1(&self) -> bool {
        **self == ADC_SUPPLYEN_A::ADC_SUPPLYEN_1
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
    pub fn adc_supplyen_0(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::ADC_SUPPLYEN_0)
    }
    #[doc = "Enable internal supply monitoring"]
    #[inline(always)]
    pub fn adc_supplyen_1(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::ADC_SUPPLYEN_1)
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
    SRAMU_RETEN_0 = 0,
    #[doc = "1: No SRAMU retention"]
    SRAMU_RETEN_1 = 1,
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
            false => SRAMU_RETEN_A::SRAMU_RETEN_0,
            true => SRAMU_RETEN_A::SRAMU_RETEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAMU_RETEN_0`"]
    #[inline(always)]
    pub fn is_sramu_reten_0(&self) -> bool {
        **self == SRAMU_RETEN_A::SRAMU_RETEN_0
    }
    #[doc = "Checks if the value of the field is `SRAMU_RETEN_1`"]
    #[inline(always)]
    pub fn is_sramu_reten_1(&self) -> bool {
        **self == SRAMU_RETEN_A::SRAMU_RETEN_1
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
    pub fn sramu_reten_0(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::SRAMU_RETEN_0)
    }
    #[doc = "No SRAMU retention"]
    #[inline(always)]
    pub fn sramu_reten_1(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::SRAMU_RETEN_1)
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
    SRAML_RETEN_0 = 0,
    #[doc = "1: No SRAML retention"]
    SRAML_RETEN_1 = 1,
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
            false => SRAML_RETEN_A::SRAML_RETEN_0,
            true => SRAML_RETEN_A::SRAML_RETEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAML_RETEN_0`"]
    #[inline(always)]
    pub fn is_sraml_reten_0(&self) -> bool {
        **self == SRAML_RETEN_A::SRAML_RETEN_0
    }
    #[doc = "Checks if the value of the field is `SRAML_RETEN_1`"]
    #[inline(always)]
    pub fn is_sraml_reten_1(&self) -> bool {
        **self == SRAML_RETEN_A::SRAML_RETEN_1
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
    pub fn sraml_reten_0(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::SRAML_RETEN_0)
    }
    #[doc = "No SRAML retention"]
    #[inline(always)]
    pub fn sraml_reten_1(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::SRAML_RETEN_1)
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
#[doc = "Field `PDB_BB_SEL_1` reader - PDB back-to-back select 1"]
pub struct PDB_BB_SEL_1_R(crate::FieldReader<bool, bool>);
impl PDB_BB_SEL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDB_BB_SEL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDB_BB_SEL_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDB_BB_SEL_1` writer - PDB back-to-back select 1"]
pub struct PDB_BB_SEL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_BB_SEL_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PDB_BB_SEL_2` reader - PDB back-to-back select 2"]
pub struct PDB_BB_SEL_2_R(crate::FieldReader<bool, bool>);
impl PDB_BB_SEL_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDB_BB_SEL_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDB_BB_SEL_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDB_BB_SEL_2` writer - PDB back-to-back select 2"]
pub struct PDB_BB_SEL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_BB_SEL_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
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
    #[doc = "Bit 22 - PDB back-to-back select 1"]
    #[inline(always)]
    pub fn pdb_bb_sel_1(&self) -> PDB_BB_SEL_1_R {
        PDB_BB_SEL_1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PDB back-to-back select 2"]
    #[inline(always)]
    pub fn pdb_bb_sel_2(&self) -> PDB_BB_SEL_2_R {
        PDB_BB_SEL_2_R::new(((self.bits >> 23) & 0x01) != 0)
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
    #[doc = "Bit 22 - PDB back-to-back select 1"]
    #[inline(always)]
    pub fn pdb_bb_sel_1(&mut self) -> PDB_BB_SEL_1_W {
        PDB_BB_SEL_1_W { w: self }
    }
    #[doc = "Bit 23 - PDB back-to-back select 2"]
    #[inline(always)]
    pub fn pdb_bb_sel_2(&mut self) -> PDB_BB_SEL_2_W {
        PDB_BB_SEL_2_W { w: self }
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
