#[doc = "Register `ACMP_CTL1` reader"]
pub struct R(crate::R<ACMP_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMP_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACMP_CTL1_SPEC>> for R {
    fn from(reader: crate::R<ACMP_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMP_CTL1` writer"]
pub struct W(crate::W<ACMP_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMP_CTL1_SPEC>;
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
impl core::convert::From<crate::W<ACMP_CTL1_SPEC>> for W {
    fn from(writer: crate::W<ACMP_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMPEN_A {
    #[doc = "0: Comparator 1 Disabled"]
    _0 = 0,
    #[doc = "1: Comparator 1 Enabled"]
    _1 = 1,
}
impl From<ACMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMPEN` reader - Comparator Enable Bit"]
pub struct ACMPEN_R(crate::FieldReader<bool, ACMPEN_A>);
impl ACMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMPEN_A {
        match self.bits {
            false => ACMPEN_A::_0,
            true => ACMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMPEN_A::_1
    }
}
impl core::ops::Deref for ACMPEN_R {
    type Target = crate::FieldReader<bool, ACMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPEN` writer - Comparator Enable Bit"]
pub struct ACMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMPEN_A::_0)
    }
    #[doc = "Comparator 1 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMPEN_A::_1)
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
#[doc = "Comparator Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMPIE_A {
    #[doc = "0: Comparator 1 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Comparator 1 interrupt Enabled. If WKEN (ACMP_CTL1\\[16\\]) is set to 1, the wake-up interrupt function will be enabled as well"]
    _1 = 1,
}
impl From<ACMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMPIE` reader - Comparator Interrupt Enable Bit"]
pub struct ACMPIE_R(crate::FieldReader<bool, ACMPIE_A>);
impl ACMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMPIE_A {
        match self.bits {
            false => ACMPIE_A::_0,
            true => ACMPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMPIE_A::_1
    }
}
impl core::ops::Deref for ACMPIE_R {
    type Target = crate::FieldReader<bool, ACMPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPIE` writer - Comparator Interrupt Enable Bit"]
pub struct ACMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMPIE_A::_0)
    }
    #[doc = "Comparator 1 interrupt Enabled. If WKEN (ACMP_CTL1\\[16\\]) is set to 1, the wake-up interrupt function will be enabled as well"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMPIE_A::_1)
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
#[doc = "Comparator Hysteresis Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSEN_A {
    #[doc = "0: Comparator 1 hysteresis Disabled"]
    _0 = 0,
    #[doc = "1: Comparator 1 hysteresis Enabled"]
    _1 = 1,
}
impl From<HYSEN_A> for bool {
    #[inline(always)]
    fn from(variant: HYSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYSEN` reader - Comparator Hysteresis Enable Bit"]
pub struct HYSEN_R(crate::FieldReader<bool, HYSEN_A>);
impl HYSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HYSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSEN_A {
        match self.bits {
            false => HYSEN_A::_0,
            true => HYSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HYSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HYSEN_A::_1
    }
}
impl core::ops::Deref for HYSEN_R {
    type Target = crate::FieldReader<bool, HYSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYSEN` writer - Comparator Hysteresis Enable Bit"]
pub struct HYSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 hysteresis Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HYSEN_A::_0)
    }
    #[doc = "Comparator 1 hysteresis Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HYSEN_A::_1)
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
#[doc = "Comparator Output Inverse Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMPOINV_A {
    #[doc = "0: Comparator 1 output inverse Disabled"]
    _0 = 0,
    #[doc = "1: Comparator 1 output inverse Enabled"]
    _1 = 1,
}
impl From<ACMPOINV_A> for bool {
    #[inline(always)]
    fn from(variant: ACMPOINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMPOINV` reader - Comparator Output Inverse Control"]
pub struct ACMPOINV_R(crate::FieldReader<bool, ACMPOINV_A>);
impl ACMPOINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMPOINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMPOINV_A {
        match self.bits {
            false => ACMPOINV_A::_0,
            true => ACMPOINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMPOINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMPOINV_A::_1
    }
}
impl core::ops::Deref for ACMPOINV_R {
    type Target = crate::FieldReader<bool, ACMPOINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMPOINV` writer - Comparator Output Inverse Control"]
pub struct ACMPOINV_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMPOINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMPOINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 output inverse Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMPOINV_A::_0)
    }
    #[doc = "Comparator 1 output inverse Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMPOINV_A::_1)
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
#[doc = "Comparator Negative Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEGSEL_A {
    #[doc = "0: ACMP1_N pin"]
    _0 = 0,
    #[doc = "1: Internal comparator reference voltage (CRV)"]
    _1 = 1,
    #[doc = "2: Band-gap voltage"]
    _2 = 2,
    #[doc = "3: DAC output"]
    _3 = 3,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEGSEL` reader - Comparator Negative Input Selection"]
pub struct NEGSEL_R(crate::FieldReader<u8, NEGSEL_A>);
impl NEGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEGSEL_A {
        match self.bits {
            0 => NEGSEL_A::_0,
            1 => NEGSEL_A::_1,
            2 => NEGSEL_A::_2,
            3 => NEGSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEGSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == NEGSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == NEGSEL_A::_3
    }
}
impl core::ops::Deref for NEGSEL_R {
    type Target = crate::FieldReader<u8, NEGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEGSEL` writer - Comparator Negative Input Selection"]
pub struct NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ACMP1_N pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEGSEL_A::_0)
    }
    #[doc = "Internal comparator reference voltage (CRV)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEGSEL_A::_1)
    }
    #[doc = "Band-gap voltage"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(NEGSEL_A::_2)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(NEGSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Comparator Positive Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POSSEL_A {
    #[doc = "0: Input from ACMP1_P0"]
    _0 = 0,
    #[doc = "1: Input from ACMP1_P1"]
    _1 = 1,
    #[doc = "2: Input from ACMP1_P2"]
    _2 = 2,
    #[doc = "3: Input from ACMP1_P3"]
    _3 = 3,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POSSEL` reader - Comparator Positive Input Selection"]
pub struct POSSEL_R(crate::FieldReader<u8, POSSEL_A>);
impl POSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        POSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSSEL_A {
        match self.bits {
            0 => POSSEL_A::_0,
            1 => POSSEL_A::_1,
            2 => POSSEL_A::_2,
            3 => POSSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POSSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POSSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == POSSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == POSSEL_A::_3
    }
}
impl core::ops::Deref for POSSEL_R {
    type Target = crate::FieldReader<u8, POSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSSEL` writer - Comparator Positive Input Selection"]
pub struct POSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input from ACMP1_P0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSSEL_A::_0)
    }
    #[doc = "Input from ACMP1_P1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSSEL_A::_1)
    }
    #[doc = "Input from ACMP1_P2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(POSSEL_A::_2)
    }
    #[doc = "Input from ACMP1_P3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(POSSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Interrupt Condition Polarity Selection\\nACMPIF1 will be set to 1 when comparator output edge condition is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTPOL_A {
    #[doc = "0: Rising edge or falling edge"]
    _0 = 0,
    #[doc = "1: Rising edge"]
    _1 = 1,
    #[doc = "2: Falling edge"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<INTPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: INTPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTPOL` reader - Interrupt Condition Polarity Selection\\nACMPIF1 will be set to 1 when comparator output edge condition is detected."]
pub struct INTPOL_R(crate::FieldReader<u8, INTPOL_A>);
impl INTPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTPOL_A {
        match self.bits {
            0 => INTPOL_A::_0,
            1 => INTPOL_A::_1,
            2 => INTPOL_A::_2,
            3 => INTPOL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTPOL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == INTPOL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == INTPOL_A::_3
    }
}
impl core::ops::Deref for INTPOL_R {
    type Target = crate::FieldReader<u8, INTPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTPOL` writer - Interrupt Condition Polarity Selection\\nACMPIF1 will be set to 1 when comparator output edge condition is detected."]
pub struct INTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Rising edge or falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTPOL_A::_0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTPOL_A::_1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(INTPOL_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(INTPOL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSEL_A {
    #[doc = "0: Comparator 1 output to ACMP1_O pin is unfiltered comparator output"]
    _0 = 0,
    #[doc = "1: Comparator 1 output to ACMP1_O pin is from filter output"]
    _1 = 1,
}
impl From<OUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` reader - Comparator Output Select"]
pub struct OUTSEL_R(crate::FieldReader<bool, OUTSEL_A>);
impl OUTSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSEL_A {
        match self.bits {
            false => OUTSEL_A::_0,
            true => OUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OUTSEL_A::_1
    }
}
impl core::ops::Deref for OUTSEL_R {
    type Target = crate::FieldReader<bool, OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTSEL` writer - Comparator Output Select"]
pub struct OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 output to ACMP1_O pin is unfiltered comparator output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTSEL_A::_0)
    }
    #[doc = "Comparator 1 output to ACMP1_O pin is from filter output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTSEL_A::_1)
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
#[doc = "Comparator Output Filter Count Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Filter function is Disabled"]
    _0 = 0,
    #[doc = "1: ACMP1 output is sampled 1 consecutive PCLK"]
    _1 = 1,
    #[doc = "2: ACMP1 output is sampled 2 consecutive PCLKs"]
    _2 = 2,
    #[doc = "3: ACMP1 output is sampled 4 consecutive PCLKs"]
    _3 = 3,
    #[doc = "4: ACMP1 output is sampled 8 consecutive PCLKs"]
    _4 = 4,
    #[doc = "5: ACMP1 output is sampled 16 consecutive PCLKs"]
    _5 = 5,
    #[doc = "6: ACMP1 output is sampled 32 consecutive PCLKs"]
    _6 = 6,
    #[doc = "7: ACMP1 output is sampled 64 consecutive PCLKs"]
    _7 = 7,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTSEL` reader - Comparator Output Filter Count Selection"]
pub struct FILTSEL_R(crate::FieldReader<u8, FILTSEL_A>);
impl FILTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTSEL_A {
        match self.bits {
            0 => FILTSEL_A::_0,
            1 => FILTSEL_A::_1,
            2 => FILTSEL_A::_2,
            3 => FILTSEL_A::_3,
            4 => FILTSEL_A::_4,
            5 => FILTSEL_A::_5,
            6 => FILTSEL_A::_6,
            7 => FILTSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FILTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FILTSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FILTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FILTSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == FILTSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == FILTSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == FILTSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == FILTSEL_A::_7
    }
}
impl core::ops::Deref for FILTSEL_R {
    type Target = crate::FieldReader<u8, FILTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTSEL` writer - Comparator Output Filter Count Selection"]
pub struct FILTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter function is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTSEL_A::_0)
    }
    #[doc = "ACMP1 output is sampled 1 consecutive PCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTSEL_A::_1)
    }
    #[doc = "ACMP1 output is sampled 2 consecutive PCLKs"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FILTSEL_A::_2)
    }
    #[doc = "ACMP1 output is sampled 4 consecutive PCLKs"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FILTSEL_A::_3)
    }
    #[doc = "ACMP1 output is sampled 8 consecutive PCLKs"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FILTSEL_A::_4)
    }
    #[doc = "ACMP1 output is sampled 16 consecutive PCLKs"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(FILTSEL_A::_5)
    }
    #[doc = "ACMP1 output is sampled 32 consecutive PCLKs"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(FILTSEL_A::_6)
    }
    #[doc = "ACMP1 output is sampled 64 consecutive PCLKs"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(FILTSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Power-down Wakeup Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: Wake-up function Disabled"]
    _0 = 0,
    #[doc = "1: Wake-up function Enabled"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - Power-down Wakeup Enable Bit"]
pub struct WKEN_R(crate::FieldReader<bool, WKEN_A>);
impl WKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKEN_A {
        match self.bits {
            false => WKEN_A::_0,
            true => WKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKEN_A::_1
    }
}
impl core::ops::Deref for WKEN_R {
    type Target = crate::FieldReader<bool, WKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKEN` writer - Power-down Wakeup Enable Bit"]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake-up function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "Wake-up function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator Enable Bit"]
    #[inline(always)]
    pub fn acmpen(&self) -> ACMPEN_R {
        ACMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn acmpie(&self) -> ACMPIE_R {
        ACMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator Hysteresis Enable Bit"]
    #[inline(always)]
    pub fn hysen(&self) -> HYSEN_R {
        HYSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator Output Inverse Control"]
    #[inline(always)]
    pub fn acmpoinv(&self) -> ACMPOINV_R {
        ACMPOINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comparator Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Comparator Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Interrupt Condition Polarity Selection\\nACMPIF1 will be set to 1 when comparator output edge condition is detected."]
    #[inline(always)]
    pub fn intpol(&self) -> INTPOL_R {
        INTPOL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Comparator Output Select"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Comparator Output Filter Count Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Power-down Wakeup Enable Bit"]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Enable Bit"]
    #[inline(always)]
    pub fn acmpen(&mut self) -> ACMPEN_W {
        ACMPEN_W { w: self }
    }
    #[doc = "Bit 1 - Comparator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn acmpie(&mut self) -> ACMPIE_W {
        ACMPIE_W { w: self }
    }
    #[doc = "Bit 2 - Comparator Hysteresis Enable Bit"]
    #[inline(always)]
    pub fn hysen(&mut self) -> HYSEN_W {
        HYSEN_W { w: self }
    }
    #[doc = "Bit 3 - Comparator Output Inverse Control"]
    #[inline(always)]
    pub fn acmpoinv(&mut self) -> ACMPOINV_W {
        ACMPOINV_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Comparator Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Interrupt Condition Polarity Selection\\nACMPIF1 will be set to 1 when comparator output edge condition is detected."]
    #[inline(always)]
    pub fn intpol(&mut self) -> INTPOL_W {
        INTPOL_W { w: self }
    }
    #[doc = "Bit 12 - Comparator Output Select"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W { w: self }
    }
    #[doc = "Bits 13:15 - Comparator Output Filter Count Selection"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FILTSEL_W {
        FILTSEL_W { w: self }
    }
    #[doc = "Bit 16 - Power-down Wakeup Enable Bit"]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmp_ctl1](index.html) module"]
pub struct ACMP_CTL1_SPEC;
impl crate::RegisterSpec for ACMP_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmp_ctl1::R](R) reader structure"]
impl crate::Readable for ACMP_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmp_ctl1::W](W) writer structure"]
impl crate::Writable for ACMP_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMP_CTL1 to value 0"]
impl crate::Resettable for ACMP_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
