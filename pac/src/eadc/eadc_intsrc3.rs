#[doc = "Register `EADC_INTSRC3` reader"]
pub struct R(crate::R<EADC_INTSRC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_INTSRC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_INTSRC3_SPEC>> for R {
    fn from(reader: crate::R<EADC_INTSRC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_INTSRC3` writer"]
pub struct W(crate::W<EADC_INTSRC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_INTSRC3_SPEC>;
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
impl core::convert::From<crate::W<EADC_INTSRC3_SPEC>> for W {
    fn from(writer: crate::W<EADC_INTSRC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sample Module 0 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE0_A {
    #[doc = "0: Sample Module 0 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 0 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE0_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE0` reader - Sample Module 0 Interrupt Enable Bit"]
pub struct SPLIE0_R(crate::FieldReader<bool, SPLIE0_A>);
impl SPLIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE0_A {
        match self.bits {
            false => SPLIE0_A::_0,
            true => SPLIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE0_A::_1
    }
}
impl core::ops::Deref for SPLIE0_R {
    type Target = crate::FieldReader<bool, SPLIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE0` writer - Sample Module 0 Interrupt Enable Bit"]
pub struct SPLIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 0 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE0_A::_0)
    }
    #[doc = "Sample Module 0 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE0_A::_1)
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
#[doc = "Sample Module 1 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE1_A {
    #[doc = "0: Sample Module 1 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 1 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE1_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE1` reader - Sample Module 1 Interrupt Enable Bit"]
pub struct SPLIE1_R(crate::FieldReader<bool, SPLIE1_A>);
impl SPLIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE1_A {
        match self.bits {
            false => SPLIE1_A::_0,
            true => SPLIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE1_A::_1
    }
}
impl core::ops::Deref for SPLIE1_R {
    type Target = crate::FieldReader<bool, SPLIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE1` writer - Sample Module 1 Interrupt Enable Bit"]
pub struct SPLIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 1 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE1_A::_0)
    }
    #[doc = "Sample Module 1 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE1_A::_1)
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
#[doc = "Sample Module 2 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE2_A {
    #[doc = "0: Sample Module 2 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 2 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE2_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE2` reader - Sample Module 2 Interrupt Enable Bit"]
pub struct SPLIE2_R(crate::FieldReader<bool, SPLIE2_A>);
impl SPLIE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE2_A {
        match self.bits {
            false => SPLIE2_A::_0,
            true => SPLIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE2_A::_1
    }
}
impl core::ops::Deref for SPLIE2_R {
    type Target = crate::FieldReader<bool, SPLIE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE2` writer - Sample Module 2 Interrupt Enable Bit"]
pub struct SPLIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 2 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE2_A::_0)
    }
    #[doc = "Sample Module 2 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE2_A::_1)
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
#[doc = "Sample Module 3 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE3_A {
    #[doc = "0: Sample Module 3 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 3 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE3_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE3` reader - Sample Module 3 Interrupt Enable Bit"]
pub struct SPLIE3_R(crate::FieldReader<bool, SPLIE3_A>);
impl SPLIE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE3_A {
        match self.bits {
            false => SPLIE3_A::_0,
            true => SPLIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE3_A::_1
    }
}
impl core::ops::Deref for SPLIE3_R {
    type Target = crate::FieldReader<bool, SPLIE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE3` writer - Sample Module 3 Interrupt Enable Bit"]
pub struct SPLIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 3 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE3_A::_0)
    }
    #[doc = "Sample Module 3 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE3_A::_1)
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
#[doc = "Sample Module 4 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE4_A {
    #[doc = "0: Sample Module 4 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 4 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE4_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE4` reader - Sample Module 4 Interrupt Enable Bit"]
pub struct SPLIE4_R(crate::FieldReader<bool, SPLIE4_A>);
impl SPLIE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE4_A {
        match self.bits {
            false => SPLIE4_A::_0,
            true => SPLIE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE4_A::_1
    }
}
impl core::ops::Deref for SPLIE4_R {
    type Target = crate::FieldReader<bool, SPLIE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE4` writer - Sample Module 4 Interrupt Enable Bit"]
pub struct SPLIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 4 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE4_A::_0)
    }
    #[doc = "Sample Module 4 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE4_A::_1)
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
#[doc = "Sample Module 5 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE5_A {
    #[doc = "0: Sample Module 5 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 5 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE5_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE5` reader - Sample Module 5 Interrupt Enable Bit"]
pub struct SPLIE5_R(crate::FieldReader<bool, SPLIE5_A>);
impl SPLIE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE5_A {
        match self.bits {
            false => SPLIE5_A::_0,
            true => SPLIE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE5_A::_1
    }
}
impl core::ops::Deref for SPLIE5_R {
    type Target = crate::FieldReader<bool, SPLIE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE5` writer - Sample Module 5 Interrupt Enable Bit"]
pub struct SPLIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 5 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE5_A::_0)
    }
    #[doc = "Sample Module 5 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE5_A::_1)
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
#[doc = "Sample Module 6 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE6_A {
    #[doc = "0: Sample Module 6 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 6 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE6_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE6` reader - Sample Module 6 Interrupt Enable Bit"]
pub struct SPLIE6_R(crate::FieldReader<bool, SPLIE6_A>);
impl SPLIE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE6_A {
        match self.bits {
            false => SPLIE6_A::_0,
            true => SPLIE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE6_A::_1
    }
}
impl core::ops::Deref for SPLIE6_R {
    type Target = crate::FieldReader<bool, SPLIE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE6` writer - Sample Module 6 Interrupt Enable Bit"]
pub struct SPLIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 6 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE6_A::_0)
    }
    #[doc = "Sample Module 6 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Sample Module 7 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE7_A {
    #[doc = "0: Sample Module 7 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 7 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE7_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE7` reader - Sample Module 7 Interrupt Enable Bit"]
pub struct SPLIE7_R(crate::FieldReader<bool, SPLIE7_A>);
impl SPLIE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE7_A {
        match self.bits {
            false => SPLIE7_A::_0,
            true => SPLIE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE7_A::_1
    }
}
impl core::ops::Deref for SPLIE7_R {
    type Target = crate::FieldReader<bool, SPLIE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE7` writer - Sample Module 7 Interrupt Enable Bit"]
pub struct SPLIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 7 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE7_A::_0)
    }
    #[doc = "Sample Module 7 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Sample Module 8 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE8_A {
    #[doc = "0: Sample Module 8 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 8 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE8_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE8` reader - Sample Module 8 Interrupt Enable Bit"]
pub struct SPLIE8_R(crate::FieldReader<bool, SPLIE8_A>);
impl SPLIE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE8_A {
        match self.bits {
            false => SPLIE8_A::_0,
            true => SPLIE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE8_A::_1
    }
}
impl core::ops::Deref for SPLIE8_R {
    type Target = crate::FieldReader<bool, SPLIE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE8` writer - Sample Module 8 Interrupt Enable Bit"]
pub struct SPLIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 8 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE8_A::_0)
    }
    #[doc = "Sample Module 8 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE8_A::_1)
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
#[doc = "Sample Module 9 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE9_A {
    #[doc = "0: Sample Module 9 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 9 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE9_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE9` reader - Sample Module 9 Interrupt Enable Bit"]
pub struct SPLIE9_R(crate::FieldReader<bool, SPLIE9_A>);
impl SPLIE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE9_A {
        match self.bits {
            false => SPLIE9_A::_0,
            true => SPLIE9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE9_A::_1
    }
}
impl core::ops::Deref for SPLIE9_R {
    type Target = crate::FieldReader<bool, SPLIE9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE9` writer - Sample Module 9 Interrupt Enable Bit"]
pub struct SPLIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 9 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE9_A::_0)
    }
    #[doc = "Sample Module 9 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE9_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Sample Module 10 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE10_A {
    #[doc = "0: Sample Module 10 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 10 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE10_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE10` reader - Sample Module 10 Interrupt Enable Bit"]
pub struct SPLIE10_R(crate::FieldReader<bool, SPLIE10_A>);
impl SPLIE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE10_A {
        match self.bits {
            false => SPLIE10_A::_0,
            true => SPLIE10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE10_A::_1
    }
}
impl core::ops::Deref for SPLIE10_R {
    type Target = crate::FieldReader<bool, SPLIE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE10` writer - Sample Module 10 Interrupt Enable Bit"]
pub struct SPLIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 10 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE10_A::_0)
    }
    #[doc = "Sample Module 10 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE10_A::_1)
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
#[doc = "Sample Module 11 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE11_A {
    #[doc = "0: Sample Module 11 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 11 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE11_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE11` reader - Sample Module 11 Interrupt Enable Bit"]
pub struct SPLIE11_R(crate::FieldReader<bool, SPLIE11_A>);
impl SPLIE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE11_A {
        match self.bits {
            false => SPLIE11_A::_0,
            true => SPLIE11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE11_A::_1
    }
}
impl core::ops::Deref for SPLIE11_R {
    type Target = crate::FieldReader<bool, SPLIE11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE11` writer - Sample Module 11 Interrupt Enable Bit"]
pub struct SPLIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 11 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE11_A::_0)
    }
    #[doc = "Sample Module 11 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE11_A::_1)
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
#[doc = "Sample Module 12 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE12_A {
    #[doc = "0: Sample Module 12 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 12 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE12_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE12` reader - Sample Module 12 Interrupt Enable Bit"]
pub struct SPLIE12_R(crate::FieldReader<bool, SPLIE12_A>);
impl SPLIE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE12_A {
        match self.bits {
            false => SPLIE12_A::_0,
            true => SPLIE12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE12_A::_1
    }
}
impl core::ops::Deref for SPLIE12_R {
    type Target = crate::FieldReader<bool, SPLIE12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE12` writer - Sample Module 12 Interrupt Enable Bit"]
pub struct SPLIE12_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 12 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE12_A::_0)
    }
    #[doc = "Sample Module 12 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE12_A::_1)
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
#[doc = "Sample Module 13 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE13_A {
    #[doc = "0: Sample Module 13 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 13 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE13_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE13` reader - Sample Module 13 Interrupt Enable Bit"]
pub struct SPLIE13_R(crate::FieldReader<bool, SPLIE13_A>);
impl SPLIE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE13_A {
        match self.bits {
            false => SPLIE13_A::_0,
            true => SPLIE13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE13_A::_1
    }
}
impl core::ops::Deref for SPLIE13_R {
    type Target = crate::FieldReader<bool, SPLIE13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE13` writer - Sample Module 13 Interrupt Enable Bit"]
pub struct SPLIE13_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 13 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE13_A::_0)
    }
    #[doc = "Sample Module 13 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE13_A::_1)
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
#[doc = "Sample Module 14 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE14_A {
    #[doc = "0: Sample Module 14 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 14 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE14_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE14` reader - Sample Module 14 Interrupt Enable Bit"]
pub struct SPLIE14_R(crate::FieldReader<bool, SPLIE14_A>);
impl SPLIE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE14_A {
        match self.bits {
            false => SPLIE14_A::_0,
            true => SPLIE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE14_A::_1
    }
}
impl core::ops::Deref for SPLIE14_R {
    type Target = crate::FieldReader<bool, SPLIE14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE14` writer - Sample Module 14 Interrupt Enable Bit"]
pub struct SPLIE14_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 14 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE14_A::_0)
    }
    #[doc = "Sample Module 14 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Sample Module 15 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE15_A {
    #[doc = "0: Sample Module 15 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 15 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE15_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE15` reader - Sample Module 15 Interrupt Enable Bit"]
pub struct SPLIE15_R(crate::FieldReader<bool, SPLIE15_A>);
impl SPLIE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE15_A {
        match self.bits {
            false => SPLIE15_A::_0,
            true => SPLIE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE15_A::_1
    }
}
impl core::ops::Deref for SPLIE15_R {
    type Target = crate::FieldReader<bool, SPLIE15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE15` writer - Sample Module 15 Interrupt Enable Bit"]
pub struct SPLIE15_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 15 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE15_A::_0)
    }
    #[doc = "Sample Module 15 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Sample Module 16 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE16_A {
    #[doc = "0: Sample Module 16 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 16 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE16_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE16` reader - Sample Module 16 Interrupt Enable Bit"]
pub struct SPLIE16_R(crate::FieldReader<bool, SPLIE16_A>);
impl SPLIE16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE16_A {
        match self.bits {
            false => SPLIE16_A::_0,
            true => SPLIE16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE16_A::_1
    }
}
impl core::ops::Deref for SPLIE16_R {
    type Target = crate::FieldReader<bool, SPLIE16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE16` writer - Sample Module 16 Interrupt Enable Bit"]
pub struct SPLIE16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 16 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE16_A::_0)
    }
    #[doc = "Sample Module 16 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE16_A::_1)
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
#[doc = "Sample Module 17 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE17_A {
    #[doc = "0: Sample Module 17 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 17 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE17_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE17` reader - Sample Module 17 Interrupt Enable Bit"]
pub struct SPLIE17_R(crate::FieldReader<bool, SPLIE17_A>);
impl SPLIE17_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE17_A {
        match self.bits {
            false => SPLIE17_A::_0,
            true => SPLIE17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE17_A::_1
    }
}
impl core::ops::Deref for SPLIE17_R {
    type Target = crate::FieldReader<bool, SPLIE17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE17` writer - Sample Module 17 Interrupt Enable Bit"]
pub struct SPLIE17_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 17 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE17_A::_0)
    }
    #[doc = "Sample Module 17 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE17_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Sample Module 18 Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIE18_A {
    #[doc = "0: Sample Module 18 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Sample Module 18 interrupt Enabled"]
    _1 = 1,
}
impl From<SPLIE18_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIE18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLIE18` reader - Sample Module 18 Interrupt Enable Bit"]
pub struct SPLIE18_R(crate::FieldReader<bool, SPLIE18_A>);
impl SPLIE18_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLIE18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIE18_A {
        match self.bits {
            false => SPLIE18_A::_0,
            true => SPLIE18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPLIE18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPLIE18_A::_1
    }
}
impl core::ops::Deref for SPLIE18_R {
    type Target = crate::FieldReader<bool, SPLIE18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIE18` writer - Sample Module 18 Interrupt Enable Bit"]
pub struct SPLIE18_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIE18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample Module 18 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLIE18_A::_0)
    }
    #[doc = "Sample Module 18 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLIE18_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sample Module 0 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie0(&self) -> SPLIE0_R {
        SPLIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sample Module 1 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie1(&self) -> SPLIE1_R {
        SPLIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sample Module 2 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie2(&self) -> SPLIE2_R {
        SPLIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sample Module 3 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie3(&self) -> SPLIE3_R {
        SPLIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sample Module 4 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie4(&self) -> SPLIE4_R {
        SPLIE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sample Module 5 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie5(&self) -> SPLIE5_R {
        SPLIE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sample Module 6 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie6(&self) -> SPLIE6_R {
        SPLIE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sample Module 7 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie7(&self) -> SPLIE7_R {
        SPLIE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sample Module 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie8(&self) -> SPLIE8_R {
        SPLIE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sample Module 9 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie9(&self) -> SPLIE9_R {
        SPLIE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Sample Module 10 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie10(&self) -> SPLIE10_R {
        SPLIE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sample Module 11 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie11(&self) -> SPLIE11_R {
        SPLIE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Sample Module 12 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie12(&self) -> SPLIE12_R {
        SPLIE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Sample Module 13 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie13(&self) -> SPLIE13_R {
        SPLIE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Sample Module 14 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie14(&self) -> SPLIE14_R {
        SPLIE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Sample Module 15 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie15(&self) -> SPLIE15_R {
        SPLIE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Sample Module 16 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie16(&self) -> SPLIE16_R {
        SPLIE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Sample Module 17 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie17(&self) -> SPLIE17_R {
        SPLIE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Sample Module 18 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie18(&self) -> SPLIE18_R {
        SPLIE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample Module 0 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie0(&mut self) -> SPLIE0_W {
        SPLIE0_W { w: self }
    }
    #[doc = "Bit 1 - Sample Module 1 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie1(&mut self) -> SPLIE1_W {
        SPLIE1_W { w: self }
    }
    #[doc = "Bit 2 - Sample Module 2 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie2(&mut self) -> SPLIE2_W {
        SPLIE2_W { w: self }
    }
    #[doc = "Bit 3 - Sample Module 3 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie3(&mut self) -> SPLIE3_W {
        SPLIE3_W { w: self }
    }
    #[doc = "Bit 4 - Sample Module 4 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie4(&mut self) -> SPLIE4_W {
        SPLIE4_W { w: self }
    }
    #[doc = "Bit 5 - Sample Module 5 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie5(&mut self) -> SPLIE5_W {
        SPLIE5_W { w: self }
    }
    #[doc = "Bit 6 - Sample Module 6 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie6(&mut self) -> SPLIE6_W {
        SPLIE6_W { w: self }
    }
    #[doc = "Bit 7 - Sample Module 7 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie7(&mut self) -> SPLIE7_W {
        SPLIE7_W { w: self }
    }
    #[doc = "Bit 8 - Sample Module 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie8(&mut self) -> SPLIE8_W {
        SPLIE8_W { w: self }
    }
    #[doc = "Bit 9 - Sample Module 9 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie9(&mut self) -> SPLIE9_W {
        SPLIE9_W { w: self }
    }
    #[doc = "Bit 10 - Sample Module 10 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie10(&mut self) -> SPLIE10_W {
        SPLIE10_W { w: self }
    }
    #[doc = "Bit 11 - Sample Module 11 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie11(&mut self) -> SPLIE11_W {
        SPLIE11_W { w: self }
    }
    #[doc = "Bit 12 - Sample Module 12 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie12(&mut self) -> SPLIE12_W {
        SPLIE12_W { w: self }
    }
    #[doc = "Bit 13 - Sample Module 13 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie13(&mut self) -> SPLIE13_W {
        SPLIE13_W { w: self }
    }
    #[doc = "Bit 14 - Sample Module 14 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie14(&mut self) -> SPLIE14_W {
        SPLIE14_W { w: self }
    }
    #[doc = "Bit 15 - Sample Module 15 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie15(&mut self) -> SPLIE15_W {
        SPLIE15_W { w: self }
    }
    #[doc = "Bit 16 - Sample Module 16 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie16(&mut self) -> SPLIE16_W {
        SPLIE16_W { w: self }
    }
    #[doc = "Bit 17 - Sample Module 17 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie17(&mut self) -> SPLIE17_W {
        SPLIE17_W { w: self }
    }
    #[doc = "Bit 18 - Sample Module 18 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn splie18(&mut self) -> SPLIE18_W {
        SPLIE18_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt 3 Source Enable Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_intsrc3](index.html) module"]
pub struct EADC_INTSRC3_SPEC;
impl crate::RegisterSpec for EADC_INTSRC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_intsrc3::R](R) reader structure"]
impl crate::Readable for EADC_INTSRC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_intsrc3::W](W) writer structure"]
impl crate::Writable for EADC_INTSRC3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_INTSRC3 to value 0"]
impl crate::Resettable for EADC_INTSRC3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
