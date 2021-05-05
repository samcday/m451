#[doc = "Register `EADC_SCTL2` reader"]
pub struct R(crate::R<EADC_SCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_SCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_SCTL2_SPEC>> for R {
    fn from(reader: crate::R<EADC_SCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_SCTL2` writer"]
pub struct W(crate::W<EADC_SCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_SCTL2_SPEC>;
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
impl core::convert::From<crate::W<EADC_SCTL2_SPEC>> for W {
    fn from(writer: crate::W<EADC_SCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - A/D Sample Module Channel Selection"]
pub struct CHSEL_R(crate::FieldReader<u8, u8>);
impl CHSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - A/D Sample Module Channel Selection"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "A/D External Trigger Rising Edge Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREN_A {
    #[doc = "0: Rising edge Disabled when A/D selects STADC as trigger source"]
    _0 = 0,
    #[doc = "1: Rising edge Enabled when A/D selects STADC as trigger source"]
    _1 = 1,
}
impl From<EXTREN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTREN` reader - A/D External Trigger Rising Edge Enable Bit"]
pub struct EXTREN_R(crate::FieldReader<bool, EXTREN_A>);
impl EXTREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTREN_A {
        match self.bits {
            false => EXTREN_A::_0,
            true => EXTREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EXTREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EXTREN_A::_1
    }
}
impl core::ops::Deref for EXTREN_R {
    type Target = crate::FieldReader<bool, EXTREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTREN` writer - A/D External Trigger Rising Edge Enable Bit"]
pub struct EXTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising edge Disabled when A/D selects STADC as trigger source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTREN_A::_0)
    }
    #[doc = "Rising edge Enabled when A/D selects STADC as trigger source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "A/D External Trigger Falling Edge Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTFEN_A {
    #[doc = "0: Falling edge Disabled when A/D selects STADC as trigger source"]
    _0 = 0,
    #[doc = "1: Falling edge Enabled when A/D selects STADC as trigger source"]
    _1 = 1,
}
impl From<EXTFEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTFEN` reader - A/D External Trigger Falling Edge Enable Bit"]
pub struct EXTFEN_R(crate::FieldReader<bool, EXTFEN_A>);
impl EXTFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTFEN_A {
        match self.bits {
            false => EXTFEN_A::_0,
            true => EXTFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EXTFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EXTFEN_A::_1
    }
}
impl core::ops::Deref for EXTFEN_R {
    type Target = crate::FieldReader<bool, EXTFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTFEN` writer - A/D External Trigger Falling Edge Enable Bit"]
pub struct EXTFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge Disabled when A/D selects STADC as trigger source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTFEN_A::_0)
    }
    #[doc = "Falling edge Enabled when A/D selects STADC as trigger source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "A/D Sample Module Start of Conversion Trigger Delay Clock Divider Selection\\nTrigger delay clock frequency:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGDLYDIV_A {
    #[doc = "0: ADC_CLK/1"]
    _0 = 0,
    #[doc = "1: ADC_CLK/2"]
    _1 = 1,
    #[doc = "2: ADC_CLK/4"]
    _2 = 2,
    #[doc = "3: ADC_CLK/16"]
    _3 = 3,
}
impl From<TRGDLYDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGDLYDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGDLYDIV` reader - A/D Sample Module Start of Conversion Trigger Delay Clock Divider Selection\\nTrigger delay clock frequency:"]
pub struct TRGDLYDIV_R(crate::FieldReader<u8, TRGDLYDIV_A>);
impl TRGDLYDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGDLYDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGDLYDIV_A {
        match self.bits {
            0 => TRGDLYDIV_A::_0,
            1 => TRGDLYDIV_A::_1,
            2 => TRGDLYDIV_A::_2,
            3 => TRGDLYDIV_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGDLYDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGDLYDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGDLYDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGDLYDIV_A::_3
    }
}
impl core::ops::Deref for TRGDLYDIV_R {
    type Target = crate::FieldReader<u8, TRGDLYDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGDLYDIV` writer - A/D Sample Module Start of Conversion Trigger Delay Clock Divider Selection\\nTrigger delay clock frequency:"]
pub struct TRGDLYDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGDLYDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGDLYDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC_CLK/1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGDLYDIV_A::_0)
    }
    #[doc = "ADC_CLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGDLYDIV_A::_1)
    }
    #[doc = "ADC_CLK/4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGDLYDIV_A::_2)
    }
    #[doc = "ADC_CLK/16"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGDLYDIV_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TRGDLYCNT` reader - A/D Sample Module Start of Conversion Trigger Delay Time"]
pub struct TRGDLYCNT_R(crate::FieldReader<u8, u8>);
impl TRGDLYCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGDLYCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGDLYCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGDLYCNT` writer - A/D Sample Module Start of Conversion Trigger Delay Time"]
pub struct TRGDLYCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGDLYCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TRGSEL` reader - A/D Sample Module Start of Conversion Trigger Source Selection"]
pub struct TRGSEL_R(crate::FieldReader<u8, u8>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - A/D Sample Module Start of Conversion Trigger Source Selection"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Interrupt Flag Position Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPOS_A {
    #[doc = "0: Set ADIFn (EADC_STATUS2\\[n\\], n=0~3) at A/D end of conversion"]
    _0 = 0,
    #[doc = "1: Set ADIFn (EADC_STATUS2\\[n\\], n=0~3)  at A/D start of conversion"]
    _1 = 1,
}
impl From<INTPOS_A> for bool {
    #[inline(always)]
    fn from(variant: INTPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTPOS` reader - Interrupt Flag Position Select"]
pub struct INTPOS_R(crate::FieldReader<bool, INTPOS_A>);
impl INTPOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTPOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTPOS_A {
        match self.bits {
            false => INTPOS_A::_0,
            true => INTPOS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTPOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTPOS_A::_1
    }
}
impl core::ops::Deref for INTPOS_R {
    type Target = crate::FieldReader<bool, INTPOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTPOS` writer - Interrupt Flag Position Select"]
pub struct INTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTPOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set ADIFn (EADC_STATUS2\\[n\\], n=0~3) at A/D end of conversion"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTPOS_A::_0)
    }
    #[doc = "Set ADIFn (EADC_STATUS2\\[n\\], n=0~3) at A/D start of conversion"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTPOS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Double Buffer Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBMEN_A {
    #[doc = "0: Sample has one sample result register. (default)"]
    _0 = 0,
    #[doc = "1: Sample has two sample result registers"]
    _1 = 1,
}
impl From<DBMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBMEN` reader - Double Buffer Mode Enable Bit"]
pub struct DBMEN_R(crate::FieldReader<bool, DBMEN_A>);
impl DBMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBMEN_A {
        match self.bits {
            false => DBMEN_A::_0,
            true => DBMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBMEN_A::_1
    }
}
impl core::ops::Deref for DBMEN_R {
    type Target = crate::FieldReader<bool, DBMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBMEN` writer - Double Buffer Mode Enable Bit"]
pub struct DBMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample has one sample result register. (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBMEN_A::_0)
    }
    #[doc = "Sample has two sample result registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBMEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `EXTSMPT` reader - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, user can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
pub struct EXTSMPT_R(crate::FieldReader<u8, u8>);
impl EXTSMPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSMPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSMPT` writer - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, user can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
pub struct EXTSMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSMPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - A/D Sample Module Channel Selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - A/D External Trigger Rising Edge Enable Bit"]
    #[inline(always)]
    pub fn extren(&self) -> EXTREN_R {
        EXTREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A/D External Trigger Falling Edge Enable Bit"]
    #[inline(always)]
    pub fn extfen(&self) -> EXTFEN_R {
        EXTFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - A/D Sample Module Start of Conversion Trigger Delay Clock Divider Selection\\nTrigger delay clock frequency:"]
    #[inline(always)]
    pub fn trgdlydiv(&self) -> TRGDLYDIV_R {
        TRGDLYDIV_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - A/D Sample Module Start of Conversion Trigger Delay Time"]
    #[inline(always)]
    pub fn trgdlycnt(&self) -> TRGDLYCNT_R {
        TRGDLYCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - A/D Sample Module Start of Conversion Trigger Source Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Interrupt Flag Position Select"]
    #[inline(always)]
    pub fn intpos(&self) -> INTPOS_R {
        INTPOS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Double Buffer Mode Enable Bit"]
    #[inline(always)]
    pub fn dbmen(&self) -> DBMEN_R {
        DBMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, user can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
    #[inline(always)]
    pub fn extsmpt(&self) -> EXTSMPT_R {
        EXTSMPT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A/D Sample Module Channel Selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bit 4 - A/D External Trigger Rising Edge Enable Bit"]
    #[inline(always)]
    pub fn extren(&mut self) -> EXTREN_W {
        EXTREN_W { w: self }
    }
    #[doc = "Bit 5 - A/D External Trigger Falling Edge Enable Bit"]
    #[inline(always)]
    pub fn extfen(&mut self) -> EXTFEN_W {
        EXTFEN_W { w: self }
    }
    #[doc = "Bits 6:7 - A/D Sample Module Start of Conversion Trigger Delay Clock Divider Selection\\nTrigger delay clock frequency:"]
    #[inline(always)]
    pub fn trgdlydiv(&mut self) -> TRGDLYDIV_W {
        TRGDLYDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - A/D Sample Module Start of Conversion Trigger Delay Time"]
    #[inline(always)]
    pub fn trgdlycnt(&mut self) -> TRGDLYCNT_W {
        TRGDLYCNT_W { w: self }
    }
    #[doc = "Bits 16:20 - A/D Sample Module Start of Conversion Trigger Source Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt Flag Position Select"]
    #[inline(always)]
    pub fn intpos(&mut self) -> INTPOS_W {
        INTPOS_W { w: self }
    }
    #[doc = "Bit 23 - Double Buffer Mode Enable Bit"]
    #[inline(always)]
    pub fn dbmen(&mut self) -> DBMEN_W {
        DBMEN_W { w: self }
    }
    #[doc = "Bits 24:31 - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, user can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
    #[inline(always)]
    pub fn extsmpt(&mut self) -> EXTSMPT_W {
        EXTSMPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample Module 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_sctl2](index.html) module"]
pub struct EADC_SCTL2_SPEC;
impl crate::RegisterSpec for EADC_SCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_sctl2::R](R) reader structure"]
impl crate::Readable for EADC_SCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_sctl2::W](W) writer structure"]
impl crate::Writable for EADC_SCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_SCTL2 to value 0"]
impl crate::Resettable for EADC_SCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
