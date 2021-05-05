#[doc = "Register `EBI_CTL0` reader"]
pub struct R(crate::R<EBI_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EBI_CTL0_SPEC>> for R {
    fn from(reader: crate::R<EBI_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_CTL0` writer"]
pub struct W(crate::W<EBI_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_CTL0_SPEC>;
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
impl core::convert::From<crate::W<EBI_CTL0_SPEC>> for W {
    fn from(writer: crate::W<EBI_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EBI Enable Bit\\nThis bit is the functional enable bit for EBI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: EBI function Disabled"]
    _0 = 0,
    #[doc = "1: EBI function Enabled"]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - EBI Enable Bit\\nThis bit is the functional enable bit for EBI."]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EBI Enable Bit\\nThis bit is the functional enable bit for EBI."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EBI function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "EBI function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
#[doc = "EBI Data Width 16-bit Select\\nThis bit defines if the EBI data width is 8-bit or 16-bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DW16_A {
    #[doc = "0: EBI data width is 8-bit"]
    _0 = 0,
    #[doc = "1: EBI data width is 16-bit"]
    _1 = 1,
}
impl From<DW16_A> for bool {
    #[inline(always)]
    fn from(variant: DW16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DW16` reader - EBI Data Width 16-bit Select\\nThis bit defines if the EBI data width is 8-bit or 16-bit."]
pub struct DW16_R(crate::FieldReader<bool, DW16_A>);
impl DW16_R {
    pub(crate) fn new(bits: bool) -> Self {
        DW16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DW16_A {
        match self.bits {
            false => DW16_A::_0,
            true => DW16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DW16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DW16_A::_1
    }
}
impl core::ops::Deref for DW16_R {
    type Target = crate::FieldReader<bool, DW16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DW16` writer - EBI Data Width 16-bit Select\\nThis bit defines if the EBI data width is 8-bit or 16-bit."]
pub struct DW16_W<'a> {
    w: &'a mut W,
}
impl<'a> DW16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DW16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EBI data width is 8-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DW16_A::_0)
    }
    #[doc = "EBI data width is 16-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DW16_A::_1)
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
#[doc = "Chip Select Pin Polar Inverse\\nThis bit defines the active level of EBI chip select pin (EBI_nCS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSPOLINV_A {
    #[doc = "0: Chip select pin (EBI_nCS) is active low"]
    _0 = 0,
    #[doc = "1: Chip select pin (EBI_nCS) is active high"]
    _1 = 1,
}
impl From<CSPOLINV_A> for bool {
    #[inline(always)]
    fn from(variant: CSPOLINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSPOLINV` reader - Chip Select Pin Polar Inverse\\nThis bit defines the active level of EBI chip select pin (EBI_nCS)."]
pub struct CSPOLINV_R(crate::FieldReader<bool, CSPOLINV_A>);
impl CSPOLINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSPOLINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSPOLINV_A {
        match self.bits {
            false => CSPOLINV_A::_0,
            true => CSPOLINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSPOLINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSPOLINV_A::_1
    }
}
impl core::ops::Deref for CSPOLINV_R {
    type Target = crate::FieldReader<bool, CSPOLINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPOLINV` writer - Chip Select Pin Polar Inverse\\nThis bit defines the active level of EBI chip select pin (EBI_nCS)."]
pub struct CSPOLINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPOLINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSPOLINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip select pin (EBI_nCS) is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSPOLINV_A::_0)
    }
    #[doc = "Chip select pin (EBI_nCS) is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSPOLINV_A::_1)
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
#[doc = "External Output Clock Divider\\nThe frequency of EBI output clock (MCLK) is controlled by MCLKDIV as follow:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCLKDIV_A {
    #[doc = "0: HCLK/1"]
    _0 = 0,
    #[doc = "1: HCLK/2"]
    _1 = 1,
    #[doc = "2: HCLK/4"]
    _2 = 2,
    #[doc = "3: HCLK/8"]
    _3 = 3,
    #[doc = "4: HCLK/16"]
    _4 = 4,
    #[doc = "5: HCLK/32"]
    _5 = 5,
    #[doc = "6: Reserved."]
    _6 = 6,
    #[doc = "7: Reserved."]
    _7 = 7,
}
impl From<MCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCLKDIV` reader - External Output Clock Divider\\nThe frequency of EBI output clock (MCLK) is controlled by MCLKDIV as follow:"]
pub struct MCLKDIV_R(crate::FieldReader<u8, MCLKDIV_A>);
impl MCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCLKDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKDIV_A {
        match self.bits {
            0 => MCLKDIV_A::_0,
            1 => MCLKDIV_A::_1,
            2 => MCLKDIV_A::_2,
            3 => MCLKDIV_A::_3,
            4 => MCLKDIV_A::_4,
            5 => MCLKDIV_A::_5,
            6 => MCLKDIV_A::_6,
            7 => MCLKDIV_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MCLKDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MCLKDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MCLKDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MCLKDIV_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == MCLKDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == MCLKDIV_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == MCLKDIV_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == MCLKDIV_A::_7
    }
}
impl core::ops::Deref for MCLKDIV_R {
    type Target = crate::FieldReader<u8, MCLKDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKDIV` writer - External Output Clock Divider\\nThe frequency of EBI output clock (MCLK) is controlled by MCLKDIV as follow:"]
pub struct MCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HCLK/1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_0)
    }
    #[doc = "HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_1)
    }
    #[doc = "HCLK/4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_2)
    }
    #[doc = "HCLK/8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_3)
    }
    #[doc = "HCLK/16"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_4)
    }
    #[doc = "HCLK/32"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_5)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_6)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(MCLKDIV_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `TALE` reader - Extend Time of ALE\\nThe EBI_ALE high pulse period (tALE) to latch the address can be controlled by TALE.\\nNote: This field only available in EBI_CTL0 register"]
pub struct TALE_R(crate::FieldReader<u8, u8>);
impl TALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TALE` writer - Extend Time of ALE\\nThe EBI_ALE high pulse period (tALE) to latch the address can be controlled by TALE.\\nNote: This field only available in EBI_CTL0 register"]
pub struct TALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "EBI Write Buffer Enable Bit\\nNote: This bit only available in EBI_CTL0 register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WBUFEN_A {
    #[doc = "0: EBI write buffer Disabled"]
    _0 = 0,
    #[doc = "1: EBI write buffer Enabled"]
    _1 = 1,
}
impl From<WBUFEN_A> for bool {
    #[inline(always)]
    fn from(variant: WBUFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WBUFEN` reader - EBI Write Buffer Enable Bit\\nNote: This bit only available in EBI_CTL0 register"]
pub struct WBUFEN_R(crate::FieldReader<bool, WBUFEN_A>);
impl WBUFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WBUFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WBUFEN_A {
        match self.bits {
            false => WBUFEN_A::_0,
            true => WBUFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WBUFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WBUFEN_A::_1
    }
}
impl core::ops::Deref for WBUFEN_R {
    type Target = crate::FieldReader<bool, WBUFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBUFEN` writer - EBI Write Buffer Enable Bit\\nNote: This bit only available in EBI_CTL0 register"]
pub struct WBUFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WBUFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WBUFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EBI write buffer Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WBUFEN_A::_0)
    }
    #[doc = "EBI write buffer Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WBUFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EBI Enable Bit\\nThis bit is the functional enable bit for EBI."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EBI Data Width 16-bit Select\\nThis bit defines if the EBI data width is 8-bit or 16-bit."]
    #[inline(always)]
    pub fn dw16(&self) -> DW16_R {
        DW16_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Pin Polar Inverse\\nThis bit defines the active level of EBI chip select pin (EBI_nCS)."]
    #[inline(always)]
    pub fn cspolinv(&self) -> CSPOLINV_R {
        CSPOLINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - External Output Clock Divider\\nThe frequency of EBI output clock (MCLK) is controlled by MCLKDIV as follow:"]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Extend Time of ALE\\nThe EBI_ALE high pulse period (tALE) to latch the address can be controlled by TALE.\\nNote: This field only available in EBI_CTL0 register"]
    #[inline(always)]
    pub fn tale(&self) -> TALE_R {
        TALE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - EBI Write Buffer Enable Bit\\nNote: This bit only available in EBI_CTL0 register"]
    #[inline(always)]
    pub fn wbufen(&self) -> WBUFEN_R {
        WBUFEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Enable Bit\\nThis bit is the functional enable bit for EBI."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - EBI Data Width 16-bit Select\\nThis bit defines if the EBI data width is 8-bit or 16-bit."]
    #[inline(always)]
    pub fn dw16(&mut self) -> DW16_W {
        DW16_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Pin Polar Inverse\\nThis bit defines the active level of EBI chip select pin (EBI_nCS)."]
    #[inline(always)]
    pub fn cspolinv(&mut self) -> CSPOLINV_W {
        CSPOLINV_W { w: self }
    }
    #[doc = "Bits 8:10 - External Output Clock Divider\\nThe frequency of EBI output clock (MCLK) is controlled by MCLKDIV as follow:"]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W {
        MCLKDIV_W { w: self }
    }
    #[doc = "Bits 16:18 - Extend Time of ALE\\nThe EBI_ALE high pulse period (tALE) to latch the address can be controlled by TALE.\\nNote: This field only available in EBI_CTL0 register"]
    #[inline(always)]
    pub fn tale(&mut self) -> TALE_W {
        TALE_W { w: self }
    }
    #[doc = "Bit 24 - EBI Write Buffer Enable Bit\\nNote: This bit only available in EBI_CTL0 register"]
    #[inline(always)]
    pub fn wbufen(&mut self) -> WBUFEN_W {
        WBUFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Bus Interface Bank0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ctl0](index.html) module"]
pub struct EBI_CTL0_SPEC;
impl crate::RegisterSpec for EBI_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_ctl0::R](R) reader structure"]
impl crate::Readable for EBI_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_ctl0::W](W) writer structure"]
impl crate::Writable for EBI_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EBI_CTL0 to value 0"]
impl crate::Resettable for EBI_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
