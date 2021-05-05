#[doc = "Register `SYS_IPRST1` reader"]
pub struct R(crate::R<SYS_IPRST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IPRST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_IPRST1_SPEC>> for R {
    fn from(reader: crate::R<SYS_IPRST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IPRST1` writer"]
pub struct W(crate::W<SYS_IPRST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IPRST1_SPEC>;
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
impl core::convert::From<crate::W<SYS_IPRST1_SPEC>> for W {
    fn from(writer: crate::W<SYS_IPRST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIORST_A {
    #[doc = "0: GPIO controller normal operation"]
    _0 = 0,
    #[doc = "1: GPIO controller reset"]
    _1 = 1,
}
impl From<GPIORST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIORST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIORST` reader - GPIO Controller Reset"]
pub struct GPIORST_R(crate::FieldReader<bool, GPIORST_A>);
impl GPIORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIORST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIORST_A {
        match self.bits {
            false => GPIORST_A::_0,
            true => GPIORST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GPIORST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GPIORST_A::_1
    }
}
impl core::ops::Deref for GPIORST_R {
    type Target = crate::FieldReader<bool, GPIORST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIORST` writer - GPIO Controller Reset"]
pub struct GPIORST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIORST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GPIO controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPIORST_A::_0)
    }
    #[doc = "GPIO controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPIORST_A::_1)
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
#[doc = "Timer0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0RST_A {
    #[doc = "0: Timer0 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer0 controller reset"]
    _1 = 1,
}
impl From<TMR0RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0RST` reader - Timer0 Controller Reset"]
pub struct TMR0RST_R(crate::FieldReader<bool, TMR0RST_A>);
impl TMR0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0RST_A {
        match self.bits {
            false => TMR0RST_A::_0,
            true => TMR0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR0RST_A::_1
    }
}
impl core::ops::Deref for TMR0RST_R {
    type Target = crate::FieldReader<bool, TMR0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0RST` writer - Timer0 Controller Reset"]
pub struct TMR0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0RST_A::_0)
    }
    #[doc = "Timer0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0RST_A::_1)
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
#[doc = "Timer1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1RST_A {
    #[doc = "0: Timer1 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer1 controller reset"]
    _1 = 1,
}
impl From<TMR1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1RST` reader - Timer1 Controller Reset"]
pub struct TMR1RST_R(crate::FieldReader<bool, TMR1RST_A>);
impl TMR1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1RST_A {
        match self.bits {
            false => TMR1RST_A::_0,
            true => TMR1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR1RST_A::_1
    }
}
impl core::ops::Deref for TMR1RST_R {
    type Target = crate::FieldReader<bool, TMR1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1RST` writer - Timer1 Controller Reset"]
pub struct TMR1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1RST_A::_0)
    }
    #[doc = "Timer1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1RST_A::_1)
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
#[doc = "Timer2 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2RST_A {
    #[doc = "0: Timer2 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer2 controller reset"]
    _1 = 1,
}
impl From<TMR2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2RST` reader - Timer2 Controller Reset"]
pub struct TMR2RST_R(crate::FieldReader<bool, TMR2RST_A>);
impl TMR2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2RST_A {
        match self.bits {
            false => TMR2RST_A::_0,
            true => TMR2RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR2RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR2RST_A::_1
    }
}
impl core::ops::Deref for TMR2RST_R {
    type Target = crate::FieldReader<bool, TMR2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2RST` writer - Timer2 Controller Reset"]
pub struct TMR2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer2 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2RST_A::_0)
    }
    #[doc = "Timer2 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2RST_A::_1)
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
#[doc = "Timer3 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3RST_A {
    #[doc = "0: Timer3 controller normal operation"]
    _0 = 0,
    #[doc = "1: Timer3 controller reset"]
    _1 = 1,
}
impl From<TMR3RST_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR3RST` reader - Timer3 Controller Reset"]
pub struct TMR3RST_R(crate::FieldReader<bool, TMR3RST_A>);
impl TMR3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR3RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3RST_A {
        match self.bits {
            false => TMR3RST_A::_0,
            true => TMR3RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR3RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR3RST_A::_1
    }
}
impl core::ops::Deref for TMR3RST_R {
    type Target = crate::FieldReader<bool, TMR3RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR3RST` writer - Timer3 Controller Reset"]
pub struct TMR3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer3 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3RST_A::_0)
    }
    #[doc = "Timer3 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3RST_A::_1)
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
#[doc = "Analog Comparator 0/1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP01RST_A {
    #[doc = "0: Analog Comparator 0/1 controller normal operation"]
    _0 = 0,
    #[doc = "1: Analog Comparator 0/1 controller reset"]
    _1 = 1,
}
impl From<ACMP01RST_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP01RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP01RST` reader - Analog Comparator 0/1 Controller Reset"]
pub struct ACMP01RST_R(crate::FieldReader<bool, ACMP01RST_A>);
impl ACMP01RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP01RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP01RST_A {
        match self.bits {
            false => ACMP01RST_A::_0,
            true => ACMP01RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMP01RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMP01RST_A::_1
    }
}
impl core::ops::Deref for ACMP01RST_R {
    type Target = crate::FieldReader<bool, ACMP01RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP01RST` writer - Analog Comparator 0/1 Controller Reset"]
pub struct ACMP01RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP01RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP01RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog Comparator 0/1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP01RST_A::_0)
    }
    #[doc = "Analog Comparator 0/1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP01RST_A::_1)
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
#[doc = "I2C0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0RST_A {
    #[doc = "0: I2C0 controller normal operation"]
    _0 = 0,
    #[doc = "1: I2C0 controller reset"]
    _1 = 1,
}
impl From<I2C0RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0RST` reader - I2C0 Controller Reset"]
pub struct I2C0RST_R(crate::FieldReader<bool, I2C0RST_A>);
impl I2C0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0RST_A {
        match self.bits {
            false => I2C0RST_A::_0,
            true => I2C0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C0RST_A::_1
    }
}
impl core::ops::Deref for I2C0RST_R {
    type Target = crate::FieldReader<bool, I2C0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0RST` writer - I2C0 Controller Reset"]
pub struct I2C0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0RST_A::_0)
    }
    #[doc = "I2C0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0RST_A::_1)
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
#[doc = "I2C1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1RST_A {
    #[doc = "0: I2C1 controller normal operation"]
    _0 = 0,
    #[doc = "1: I2C1 controller reset"]
    _1 = 1,
}
impl From<I2C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 Controller Reset"]
pub struct I2C1RST_R(crate::FieldReader<bool, I2C1RST_A>);
impl I2C1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1RST_A {
        match self.bits {
            false => I2C1RST_A::_0,
            true => I2C1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C1RST_A::_1
    }
}
impl core::ops::Deref for I2C1RST_R {
    type Target = crate::FieldReader<bool, I2C1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 Controller Reset"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1RST_A::_0)
    }
    #[doc = "I2C1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1RST_A::_1)
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
#[doc = "SPI0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0RST_A {
    #[doc = "0: SPI0 controller normal operation"]
    _0 = 0,
    #[doc = "1: SPI0 controller reset"]
    _1 = 1,
}
impl From<SPI0RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0RST` reader - SPI0 Controller Reset"]
pub struct SPI0RST_R(crate::FieldReader<bool, SPI0RST_A>);
impl SPI0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0RST_A {
        match self.bits {
            false => SPI0RST_A::_0,
            true => SPI0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0RST_A::_1
    }
}
impl core::ops::Deref for SPI0RST_R {
    type Target = crate::FieldReader<bool, SPI0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0RST` writer - SPI0 Controller Reset"]
pub struct SPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0RST_A::_0)
    }
    #[doc = "SPI0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0RST_A::_1)
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
#[doc = "SPI1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1RST_A {
    #[doc = "0: SPI1 controller normal operation"]
    _0 = 0,
    #[doc = "1: SPI1 controller reset"]
    _1 = 1,
}
impl From<SPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 Controller Reset"]
pub struct SPI1RST_R(crate::FieldReader<bool, SPI1RST_A>);
impl SPI1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1RST_A {
        match self.bits {
            false => SPI1RST_A::_0,
            true => SPI1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI1RST_A::_1
    }
}
impl core::ops::Deref for SPI1RST_R {
    type Target = crate::FieldReader<bool, SPI1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 Controller Reset"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1RST_A::_0)
    }
    #[doc = "SPI1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1RST_A::_1)
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
#[doc = "SPI2 Controller Reset (M45xG/M45xE Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2RST_A {
    #[doc = "0: SPI2 controller normal operation"]
    _0 = 0,
    #[doc = "1: SPI2 controller reset"]
    _1 = 1,
}
impl From<SPI2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2RST` reader - SPI2 Controller Reset (M45xG/M45xE Only)"]
pub struct SPI2RST_R(crate::FieldReader<bool, SPI2RST_A>);
impl SPI2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2RST_A {
        match self.bits {
            false => SPI2RST_A::_0,
            true => SPI2RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI2RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI2RST_A::_1
    }
}
impl core::ops::Deref for SPI2RST_R {
    type Target = crate::FieldReader<bool, SPI2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2RST` writer - SPI2 Controller Reset (M45xG/M45xE Only)"]
pub struct SPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI2 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI2RST_A::_0)
    }
    #[doc = "SPI2 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI2RST_A::_1)
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
#[doc = "UART0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RST_A {
    #[doc = "0: UART0 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART0 controller reset"]
    _1 = 1,
}
impl From<UART0RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0RST` reader - UART0 Controller Reset"]
pub struct UART0RST_R(crate::FieldReader<bool, UART0RST_A>);
impl UART0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0RST_A {
        match self.bits {
            false => UART0RST_A::_0,
            true => UART0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0RST_A::_1
    }
}
impl core::ops::Deref for UART0RST_R {
    type Target = crate::FieldReader<bool, UART0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0RST` writer - UART0 Controller Reset"]
pub struct UART0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0RST_A::_0)
    }
    #[doc = "UART0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0RST_A::_1)
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
#[doc = "UART1 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RST_A {
    #[doc = "0: UART1 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART1 controller reset"]
    _1 = 1,
}
impl From<UART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1RST` reader - UART1 Controller Reset"]
pub struct UART1RST_R(crate::FieldReader<bool, UART1RST_A>);
impl UART1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1RST_A {
        match self.bits {
            false => UART1RST_A::_0,
            true => UART1RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART1RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART1RST_A::_1
    }
}
impl core::ops::Deref for UART1RST_R {
    type Target = crate::FieldReader<bool, UART1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1RST` writer - UART1 Controller Reset"]
pub struct UART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART1 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1RST_A::_0)
    }
    #[doc = "UART1 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1RST_A::_1)
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
#[doc = "UART2 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2RST_A {
    #[doc = "0: UART2 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART2 controller reset"]
    _1 = 1,
}
impl From<UART2RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2RST` reader - UART2 Controller Reset"]
pub struct UART2RST_R(crate::FieldReader<bool, UART2RST_A>);
impl UART2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2RST_A {
        match self.bits {
            false => UART2RST_A::_0,
            true => UART2RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART2RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART2RST_A::_1
    }
}
impl core::ops::Deref for UART2RST_R {
    type Target = crate::FieldReader<bool, UART2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2RST` writer - UART2 Controller Reset"]
pub struct UART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART2 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2RST_A::_0)
    }
    #[doc = "UART2 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2RST_A::_1)
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
#[doc = "UART3 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3RST_A {
    #[doc = "0: UART3 controller normal operation"]
    _0 = 0,
    #[doc = "1: UART3 controller reset"]
    _1 = 1,
}
impl From<UART3RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART3RST` reader - UART3 Controller Reset"]
pub struct UART3RST_R(crate::FieldReader<bool, UART3RST_A>);
impl UART3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART3RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3RST_A {
        match self.bits {
            false => UART3RST_A::_0,
            true => UART3RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART3RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART3RST_A::_1
    }
}
impl core::ops::Deref for UART3RST_R {
    type Target = crate::FieldReader<bool, UART3RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3RST` writer - UART3 Controller Reset"]
pub struct UART3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART3 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3RST_A::_0)
    }
    #[doc = "UART3 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3RST_A::_1)
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
#[doc = "CAN0 Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN0RST_A {
    #[doc = "0: CAN0 controller normal operation"]
    _0 = 0,
    #[doc = "1: CAN0 controller reset"]
    _1 = 1,
}
impl From<CAN0RST_A> for bool {
    #[inline(always)]
    fn from(variant: CAN0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN0RST` reader - CAN0 Controller Reset"]
pub struct CAN0RST_R(crate::FieldReader<bool, CAN0RST_A>);
impl CAN0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN0RST_A {
        match self.bits {
            false => CAN0RST_A::_0,
            true => CAN0RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAN0RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAN0RST_A::_1
    }
}
impl core::ops::Deref for CAN0RST_R {
    type Target = crate::FieldReader<bool, CAN0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN0RST` writer - CAN0 Controller Reset"]
pub struct CAN0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN0RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAN0 controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAN0RST_A::_0)
    }
    #[doc = "CAN0 controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAN0RST_A::_1)
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
#[doc = "OTG Controller Reset (M45xG/M45xE Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGRST_A {
    #[doc = "0: OTG controller normal operation"]
    _0 = 0,
    #[doc = "1: OTG controller reset"]
    _1 = 1,
}
impl From<OTGRST_A> for bool {
    #[inline(always)]
    fn from(variant: OTGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGRST` reader - OTG Controller Reset (M45xG/M45xE Only)"]
pub struct OTGRST_R(crate::FieldReader<bool, OTGRST_A>);
impl OTGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGRST_A {
        match self.bits {
            false => OTGRST_A::_0,
            true => OTGRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OTGRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OTGRST_A::_1
    }
}
impl core::ops::Deref for OTGRST_R {
    type Target = crate::FieldReader<bool, OTGRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGRST` writer - OTG Controller Reset (M45xG/M45xE Only)"]
pub struct OTGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGRST_A::_0)
    }
    #[doc = "OTG controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "USB Device Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDRST_A {
    #[doc = "0: USB device controller normal operation"]
    _0 = 0,
    #[doc = "1: USB device controller reset"]
    _1 = 1,
}
impl From<USBDRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBDRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDRST` reader - USB Device Controller Reset"]
pub struct USBDRST_R(crate::FieldReader<bool, USBDRST_A>);
impl USBDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBDRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDRST_A {
        match self.bits {
            false => USBDRST_A::_0,
            true => USBDRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBDRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBDRST_A::_1
    }
}
impl core::ops::Deref for USBDRST_R {
    type Target = crate::FieldReader<bool, USBDRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDRST` writer - USB Device Controller Reset"]
pub struct USBDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB device controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDRST_A::_0)
    }
    #[doc = "USB device controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "EADC Controller Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EADCRST_A {
    #[doc = "0: EADC controller normal operation"]
    _0 = 0,
    #[doc = "1: EADC controller reset"]
    _1 = 1,
}
impl From<EADCRST_A> for bool {
    #[inline(always)]
    fn from(variant: EADCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EADCRST` reader - EADC Controller Reset"]
pub struct EADCRST_R(crate::FieldReader<bool, EADCRST_A>);
impl EADCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EADCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EADCRST_A {
        match self.bits {
            false => EADCRST_A::_0,
            true => EADCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EADCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EADCRST_A::_1
    }
}
impl core::ops::Deref for EADCRST_R {
    type Target = crate::FieldReader<bool, EADCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EADCRST` writer - EADC Controller Reset"]
pub struct EADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EADCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EADCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EADC controller normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EADCRST_A::_0)
    }
    #[doc = "EADC controller reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EADCRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - GPIO Controller Reset"]
    #[inline(always)]
    pub fn gpiorst(&self) -> GPIORST_R {
        GPIORST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0 Controller Reset"]
    #[inline(always)]
    pub fn tmr0rst(&self) -> TMR0RST_R {
        TMR0RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer1 Controller Reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> TMR1RST_R {
        TMR1RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer2 Controller Reset"]
    #[inline(always)]
    pub fn tmr2rst(&self) -> TMR2RST_R {
        TMR2RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer3 Controller Reset"]
    #[inline(always)]
    pub fn tmr3rst(&self) -> TMR3RST_R {
        TMR3RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Controller Reset"]
    #[inline(always)]
    pub fn acmp01rst(&self) -> ACMP01RST_R {
        ACMP01RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 Controller Reset"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C1 Controller Reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 Controller Reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI1 Controller Reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 Controller Reset (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Controller Reset"]
    #[inline(always)]
    pub fn uart0rst(&self) -> UART0RST_R {
        UART0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART1 Controller Reset"]
    #[inline(always)]
    pub fn uart1rst(&self) -> UART1RST_R {
        UART1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART2 Controller Reset"]
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART3 Controller Reset"]
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CAN0 Controller Reset"]
    #[inline(always)]
    pub fn can0rst(&self) -> CAN0RST_R {
        CAN0RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - OTG Controller Reset (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn otgrst(&self) -> OTGRST_R {
        OTGRST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB Device Controller Reset"]
    #[inline(always)]
    pub fn usbdrst(&self) -> USBDRST_R {
        USBDRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EADC Controller Reset"]
    #[inline(always)]
    pub fn eadcrst(&self) -> EADCRST_R {
        EADCRST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - GPIO Controller Reset"]
    #[inline(always)]
    pub fn gpiorst(&mut self) -> GPIORST_W {
        GPIORST_W { w: self }
    }
    #[doc = "Bit 2 - Timer0 Controller Reset"]
    #[inline(always)]
    pub fn tmr0rst(&mut self) -> TMR0RST_W {
        TMR0RST_W { w: self }
    }
    #[doc = "Bit 3 - Timer1 Controller Reset"]
    #[inline(always)]
    pub fn tmr1rst(&mut self) -> TMR1RST_W {
        TMR1RST_W { w: self }
    }
    #[doc = "Bit 4 - Timer2 Controller Reset"]
    #[inline(always)]
    pub fn tmr2rst(&mut self) -> TMR2RST_W {
        TMR2RST_W { w: self }
    }
    #[doc = "Bit 5 - Timer3 Controller Reset"]
    #[inline(always)]
    pub fn tmr3rst(&mut self) -> TMR3RST_W {
        TMR3RST_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator 0/1 Controller Reset"]
    #[inline(always)]
    pub fn acmp01rst(&mut self) -> ACMP01RST_W {
        ACMP01RST_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 Controller Reset"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2C0RST_W {
        I2C0RST_W { w: self }
    }
    #[doc = "Bit 9 - I2C1 Controller Reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 Controller Reset"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W { w: self }
    }
    #[doc = "Bit 13 - SPI1 Controller Reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 Controller Reset (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Controller Reset"]
    #[inline(always)]
    pub fn uart0rst(&mut self) -> UART0RST_W {
        UART0RST_W { w: self }
    }
    #[doc = "Bit 17 - UART1 Controller Reset"]
    #[inline(always)]
    pub fn uart1rst(&mut self) -> UART1RST_W {
        UART1RST_W { w: self }
    }
    #[doc = "Bit 18 - UART2 Controller Reset"]
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W {
        UART2RST_W { w: self }
    }
    #[doc = "Bit 19 - UART3 Controller Reset"]
    #[inline(always)]
    pub fn uart3rst(&mut self) -> UART3RST_W {
        UART3RST_W { w: self }
    }
    #[doc = "Bit 24 - CAN0 Controller Reset"]
    #[inline(always)]
    pub fn can0rst(&mut self) -> CAN0RST_W {
        CAN0RST_W { w: self }
    }
    #[doc = "Bit 26 - OTG Controller Reset (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn otgrst(&mut self) -> OTGRST_W {
        OTGRST_W { w: self }
    }
    #[doc = "Bit 27 - USB Device Controller Reset"]
    #[inline(always)]
    pub fn usbdrst(&mut self) -> USBDRST_W {
        USBDRST_W { w: self }
    }
    #[doc = "Bit 28 - EADC Controller Reset"]
    #[inline(always)]
    pub fn eadcrst(&mut self) -> EADCRST_W {
        EADCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_iprst1](index.html) module"]
pub struct SYS_IPRST1_SPEC;
impl crate::RegisterSpec for SYS_IPRST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_iprst1::R](R) reader structure"]
impl crate::Readable for SYS_IPRST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_iprst1::W](W) writer structure"]
impl crate::Writable for SYS_IPRST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IPRST1 to value 0"]
impl crate::Resettable for SYS_IPRST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
