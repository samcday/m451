#[doc = "Register `DAC_CTL` reader"]
pub struct R(crate::R<DAC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_CTL_SPEC>> for R {
    fn from(reader: crate::R<DAC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CTL` writer"]
pub struct W(crate::W<DAC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CTL_SPEC>;
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
impl core::convert::From<crate::W<DAC_CTL_SPEC>> for W {
    fn from(writer: crate::W<DAC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC is Disabled"]
    _0 = 0,
    #[doc = "1: DAC is Enabled"]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable Bit"]
pub struct DACEN_R(crate::FieldReader<bool, DACEN_A>);
impl DACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACEN_A::_1
    }
}
impl core::ops::Deref for DACEN_R {
    type Target = crate::FieldReader<bool, DACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACEN` writer - DAC Enable Bit"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "DAC is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
#[doc = "DAC Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACIEN_A {
    #[doc = "0: Interrupt is Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is Enabled"]
    _1 = 1,
}
impl From<DACIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACIEN` reader - DAC Interrupt Enable Bit"]
pub struct DACIEN_R(crate::FieldReader<bool, DACIEN_A>);
impl DACIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACIEN_A {
        match self.bits {
            false => DACIEN_A::_0,
            true => DACIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACIEN_A::_1
    }
}
impl core::ops::Deref for DACIEN_R {
    type Target = crate::FieldReader<bool, DACIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACIEN` writer - DAC Interrupt Enable Bit"]
pub struct DACIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACIEN_A::_0)
    }
    #[doc = "Interrupt is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACIEN_A::_1)
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
#[doc = "DMA Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA mode Disabled"]
    _0 = 0,
    #[doc = "1: DMA mode Enabled"]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Mode Enable Bit"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAEN_A::_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Mode Enable Bit"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
#[doc = "DMA Under-run Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAURIEN_A {
    #[doc = "0: DMA underrun interrupt Disabled"]
    _0 = 0,
    #[doc = "1: DMA underrun interrupt Enabled"]
    _1 = 1,
}
impl From<DMAURIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAURIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAURIEN` reader - DMA Under-run Interrupt Enable Bit"]
pub struct DMAURIEN_R(crate::FieldReader<bool, DMAURIEN_A>);
impl DMAURIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAURIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAURIEN_A {
        match self.bits {
            false => DMAURIEN_A::_0,
            true => DMAURIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAURIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAURIEN_A::_1
    }
}
impl core::ops::Deref for DMAURIEN_R {
    type Target = crate::FieldReader<bool, DMAURIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAURIEN` writer - DMA Under-run Interrupt Enable Bit"]
pub struct DMAURIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAURIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAURIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA underrun interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAURIEN_A::_0)
    }
    #[doc = "DMA underrun interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAURIEN_A::_1)
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
#[doc = "Trigger Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: DAC event trigger mode Disabled"]
    _0 = 0,
    #[doc = "1: DAC event trigger mode Enabled"]
    _1 = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN` reader - Trigger Mode Enable Bit"]
pub struct TRGEN_R(crate::FieldReader<bool, TRGEN_A>);
impl TRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::_0,
            true => TRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN_A::_1
    }
}
impl core::ops::Deref for TRGEN_R {
    type Target = crate::FieldReader<bool, TRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN` writer - Trigger Mode Enable Bit"]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC event trigger mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN_A::_0)
    }
    #[doc = "DAC event trigger mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN_A::_1)
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
#[doc = "Trigger Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Software trigger"]
    _0 = 0,
    #[doc = "1: External pin STDAC trigger"]
    _1 = 1,
    #[doc = "2: Timer 0 trigger"]
    _2 = 2,
    #[doc = "3: Timer 1 trigger"]
    _3 = 3,
    #[doc = "4: Timer 2 trigger"]
    _4 = 4,
    #[doc = "5: Timer 3 trigger"]
    _5 = 5,
    #[doc = "6: PWM0 trigger"]
    _6 = 6,
    #[doc = "7: PWM1 trigger"]
    _7 = 7,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Source Selection"]
pub struct TRGSEL_R(crate::FieldReader<u8, TRGSEL_A>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::_0,
            1 => TRGSEL_A::_1,
            2 => TRGSEL_A::_2,
            3 => TRGSEL_A::_3,
            4 => TRGSEL_A::_4,
            5 => TRGSEL_A::_5,
            6 => TRGSEL_A::_6,
            7 => TRGSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL_A::_7
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Source Selection"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0)
    }
    #[doc = "External pin STDAC trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1)
    }
    #[doc = "Timer 0 trigger"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL_A::_2)
    }
    #[doc = "Timer 1 trigger"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL_A::_3)
    }
    #[doc = "Timer 2 trigger"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL_A::_4)
    }
    #[doc = "Timer 3 trigger"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL_A::_5)
    }
    #[doc = "PWM0 trigger"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL_A::_6)
    }
    #[doc = "PWM1 trigger"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Bypass Buffer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Output voltage buffer Enabled"]
    _0 = 0,
    #[doc = "1: Output voltage buffer Disabled"]
    _1 = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Bypass Buffer Mode"]
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::_0,
            true => BYPASS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BYPASS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BYPASS_A::_1
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Bypass Buffer Mode"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output voltage buffer Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYPASS_A::_0)
    }
    #[doc = "Output voltage buffer Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYPASS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "DAC Data Left-aligned Enabled Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LALIGN_A {
    #[doc = "0: Right alignment"]
    _0 = 0,
    #[doc = "1: Left alignment"]
    _1 = 1,
}
impl From<LALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: LALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LALIGN` reader - DAC Data Left-aligned Enabled Control"]
pub struct LALIGN_R(crate::FieldReader<bool, LALIGN_A>);
impl LALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LALIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LALIGN_A {
        match self.bits {
            false => LALIGN_A::_0,
            true => LALIGN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LALIGN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LALIGN_A::_1
    }
}
impl core::ops::Deref for LALIGN_R {
    type Target = crate::FieldReader<bool, LALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LALIGN` writer - DAC Data Left-aligned Enabled Control"]
pub struct LALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> LALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LALIGN_A::_0)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LALIGN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "External Pin Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRGSEL_A {
    #[doc = "0: Low level trigger"]
    _0 = 0,
    #[doc = "1: High level trigger"]
    _1 = 1,
    #[doc = "2: Falling edge trigger"]
    _2 = 2,
    #[doc = "3: Rising edge trigger"]
    _3 = 3,
}
impl From<ETRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETRGSEL` reader - External Pin Trigger Selection"]
pub struct ETRGSEL_R(crate::FieldReader<u8, ETRGSEL_A>);
impl ETRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGSEL_A {
        match self.bits {
            0 => ETRGSEL_A::_0,
            1 => ETRGSEL_A::_1,
            2 => ETRGSEL_A::_2,
            3 => ETRGSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ETRGSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ETRGSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ETRGSEL_A::_3
    }
}
impl core::ops::Deref for ETRGSEL_R {
    type Target = crate::FieldReader<u8, ETRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRGSEL` writer - External Pin Trigger Selection"]
pub struct ETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low level trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETRGSEL_A::_0)
    }
    #[doc = "High level trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETRGSEL_A::_1)
    }
    #[doc = "Falling edge trigger"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ETRGSEL_A::_2)
    }
    #[doc = "Rising edge trigger"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ETRGSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DAC Enable Bit"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Interrupt Enable Bit"]
    #[inline(always)]
    pub fn dacien(&self) -> DACIEN_R {
        DACIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Mode Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Under-run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn dmaurien(&self) -> DMAURIEN_R {
        DMAURIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trigger Mode Enable Bit"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Trigger Source Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Bypass Buffer Mode"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DAC Data Left-aligned Enabled Control"]
    #[inline(always)]
    pub fn lalign(&self) -> LALIGN_R {
        LALIGN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External Pin Trigger Selection"]
    #[inline(always)]
    pub fn etrgsel(&self) -> ETRGSEL_R {
        ETRGSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Enable Bit"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Bit 1 - DAC Interrupt Enable Bit"]
    #[inline(always)]
    pub fn dacien(&mut self) -> DACIEN_W {
        DACIEN_W { w: self }
    }
    #[doc = "Bit 2 - DMA Mode Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Under-run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn dmaurien(&mut self) -> DMAURIEN_W {
        DMAURIEN_W { w: self }
    }
    #[doc = "Bit 4 - Trigger Mode Enable Bit"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TRGEN_W {
        TRGEN_W { w: self }
    }
    #[doc = "Bits 5:7 - Trigger Source Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bit 8 - Bypass Buffer Mode"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 10 - DAC Data Left-aligned Enabled Control"]
    #[inline(always)]
    pub fn lalign(&mut self) -> LALIGN_W {
        LALIGN_W { w: self }
    }
    #[doc = "Bits 12:13 - External Pin Trigger Selection"]
    #[inline(always)]
    pub fn etrgsel(&mut self) -> ETRGSEL_W {
        ETRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_ctl](index.html) module"]
pub struct DAC_CTL_SPEC;
impl crate::RegisterSpec for DAC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_ctl::R](R) reader structure"]
impl crate::Readable for DAC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_ctl::W](W) writer structure"]
impl crate::Writable for DAC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_CTL to value 0"]
impl crate::Resettable for DAC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
