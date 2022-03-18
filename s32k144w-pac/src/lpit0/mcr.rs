#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Module Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_CEN_A {
    #[doc = "0: Disable peripheral clock to timers"]
    M_CEN_0 = 0,
    #[doc = "1: Enable peripheral clock to timers"]
    M_CEN_1 = 1,
}
impl From<M_CEN_A> for bool {
    #[inline(always)]
    fn from(variant: M_CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M_CEN` reader - Module Clock Enable"]
pub struct M_CEN_R(crate::FieldReader<bool, M_CEN_A>);
impl M_CEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_CEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_CEN_A {
        match self.bits {
            false => M_CEN_A::M_CEN_0,
            true => M_CEN_A::M_CEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `M_CEN_0`"]
    #[inline(always)]
    pub fn is_m_cen_0(&self) -> bool {
        **self == M_CEN_A::M_CEN_0
    }
    #[doc = "Checks if the value of the field is `M_CEN_1`"]
    #[inline(always)]
    pub fn is_m_cen_1(&self) -> bool {
        **self == M_CEN_A::M_CEN_1
    }
}
impl core::ops::Deref for M_CEN_R {
    type Target = crate::FieldReader<bool, M_CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_CEN` writer - Module Clock Enable"]
pub struct M_CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> M_CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable peripheral clock to timers"]
    #[inline(always)]
    pub fn m_cen_0(self) -> &'a mut W {
        self.variant(M_CEN_A::M_CEN_0)
    }
    #[doc = "Enable peripheral clock to timers"]
    #[inline(always)]
    pub fn m_cen_1(self) -> &'a mut W {
        self.variant(M_CEN_A::M_CEN_1)
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
#[doc = "Software Reset Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_A {
    #[doc = "0: Timer channels and registers are not reset"]
    SW_RST_0 = 0,
    #[doc = "1: Reset timer channels and registers"]
    SW_RST_1 = 1,
}
impl From<SW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST` reader - Software Reset Bit"]
pub struct SW_RST_R(crate::FieldReader<bool, SW_RST_A>);
impl SW_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_A {
        match self.bits {
            false => SW_RST_A::SW_RST_0,
            true => SW_RST_A::SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_RST_0`"]
    #[inline(always)]
    pub fn is_sw_rst_0(&self) -> bool {
        **self == SW_RST_A::SW_RST_0
    }
    #[doc = "Checks if the value of the field is `SW_RST_1`"]
    #[inline(always)]
    pub fn is_sw_rst_1(&self) -> bool {
        **self == SW_RST_A::SW_RST_1
    }
}
impl core::ops::Deref for SW_RST_R {
    type Target = crate::FieldReader<bool, SW_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RST` writer - Software Reset Bit"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer channels and registers are not reset"]
    #[inline(always)]
    pub fn sw_rst_0(self) -> &'a mut W {
        self.variant(SW_RST_A::SW_RST_0)
    }
    #[doc = "Reset timer channels and registers"]
    #[inline(always)]
    pub fn sw_rst_1(self) -> &'a mut W {
        self.variant(SW_RST_A::SW_RST_1)
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
#[doc = "DOZE Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_EN_A {
    #[doc = "0: Stop timer channels in DOZE mode"]
    DOZE_EN_0 = 0,
    #[doc = "1: Allow timer channels to continue to run in DOZE mode"]
    DOZE_EN_1 = 1,
}
impl From<DOZE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZE_EN` reader - DOZE Mode Enable Bit"]
pub struct DOZE_EN_R(crate::FieldReader<bool, DOZE_EN_A>);
impl DOZE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_EN_A {
        match self.bits {
            false => DOZE_EN_A::DOZE_EN_0,
            true => DOZE_EN_A::DOZE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZE_EN_0`"]
    #[inline(always)]
    pub fn is_doze_en_0(&self) -> bool {
        **self == DOZE_EN_A::DOZE_EN_0
    }
    #[doc = "Checks if the value of the field is `DOZE_EN_1`"]
    #[inline(always)]
    pub fn is_doze_en_1(&self) -> bool {
        **self == DOZE_EN_A::DOZE_EN_1
    }
}
impl core::ops::Deref for DOZE_EN_R {
    type Target = crate::FieldReader<bool, DOZE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZE_EN` writer - DOZE Mode Enable Bit"]
pub struct DOZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop timer channels in DOZE mode"]
    #[inline(always)]
    pub fn doze_en_0(self) -> &'a mut W {
        self.variant(DOZE_EN_A::DOZE_EN_0)
    }
    #[doc = "Allow timer channels to continue to run in DOZE mode"]
    #[inline(always)]
    pub fn doze_en_1(self) -> &'a mut W {
        self.variant(DOZE_EN_A::DOZE_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Debug Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_EN_A {
    #[doc = "0: Stop timer channels in Debug mode"]
    DBG_EN_0 = 0,
    #[doc = "1: Allow timer channels to continue to run in Debug mode"]
    DBG_EN_1 = 1,
}
impl From<DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_EN` reader - Debug Enable Bit"]
pub struct DBG_EN_R(crate::FieldReader<bool, DBG_EN_A>);
impl DBG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            false => DBG_EN_A::DBG_EN_0,
            true => DBG_EN_A::DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline(always)]
    pub fn is_dbg_en_0(&self) -> bool {
        **self == DBG_EN_A::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline(always)]
    pub fn is_dbg_en_1(&self) -> bool {
        **self == DBG_EN_A::DBG_EN_1
    }
}
impl core::ops::Deref for DBG_EN_R {
    type Target = crate::FieldReader<bool, DBG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_EN` writer - Debug Enable Bit"]
pub struct DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop timer channels in Debug mode"]
    #[inline(always)]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_0)
    }
    #[doc = "Allow timer channels to continue to run in Debug mode"]
    #[inline(always)]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline(always)]
    pub fn m_cen(&self) -> M_CEN_R {
        M_CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline(always)]
    pub fn doze_en(&self) -> DOZE_EN_R {
        DOZE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline(always)]
    pub fn m_cen(&mut self) -> M_CEN_W {
        M_CEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline(always)]
    pub fn doze_en(&mut self) -> DOZE_EN_W {
        DOZE_EN_W { w: self }
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline(always)]
    pub fn dbg_en(&mut self) -> DBG_EN_W {
        DBG_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
