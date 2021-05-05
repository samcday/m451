#[doc = "Register `PWM_BNF` reader"]
pub struct R(crate::R<PWM_BNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_BNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_BNF_SPEC>> for R {
    fn from(reader: crate::R<PWM_BNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_BNF` writer"]
pub struct W(crate::W<PWM_BNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_BNF_SPEC>;
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
impl core::convert::From<crate::W<PWM_BNF_SPEC>> for W {
    fn from(writer: crate::W<PWM_BNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Brake 0 Noise Filter Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK0NFEN_A {
    #[doc = "0: Noise filter of PWM Brake 0 Disabled"]
    _0 = 0,
    #[doc = "1: Noise filter of PWM Brake 0 Enabled"]
    _1 = 1,
}
impl From<BRK0NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRK0NFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK0NFEN` reader - PWM Brake 0 Noise Filter Enable Bit"]
pub struct BRK0NFEN_R(crate::FieldReader<bool, BRK0NFEN_A>);
impl BRK0NFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK0NFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK0NFEN_A {
        match self.bits {
            false => BRK0NFEN_A::_0,
            true => BRK0NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK0NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK0NFEN_A::_1
    }
}
impl core::ops::Deref for BRK0NFEN_R {
    type Target = crate::FieldReader<bool, BRK0NFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK0NFEN` writer - PWM Brake 0 Noise Filter Enable Bit"]
pub struct BRK0NFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK0NFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK0NFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Noise filter of PWM Brake 0 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK0NFEN_A::_0)
    }
    #[doc = "Noise filter of PWM Brake 0 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK0NFEN_A::_1)
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
#[doc = "Brake 0 Edge Detector Filter Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRK0NFSEL_A {
    #[doc = "0: Filter clock = HCLK"]
    _0 = 0,
    #[doc = "1: Filter clock = HCLK/2"]
    _1 = 1,
    #[doc = "2: Filter clock = HCLK/4"]
    _2 = 2,
    #[doc = "3: Filter clock = HCLK/8"]
    _3 = 3,
    #[doc = "4: Filter clock = HCLK/16"]
    _4 = 4,
    #[doc = "5: Filter clock = HCLK/32"]
    _5 = 5,
    #[doc = "6: Filter clock = HCLK/64"]
    _6 = 6,
    #[doc = "7: Filter clock = HCLK/128"]
    _7 = 7,
}
impl From<BRK0NFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BRK0NFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRK0NFSEL` reader - Brake 0 Edge Detector Filter Clock Selection"]
pub struct BRK0NFSEL_R(crate::FieldReader<u8, BRK0NFSEL_A>);
impl BRK0NFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRK0NFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK0NFSEL_A {
        match self.bits {
            0 => BRK0NFSEL_A::_0,
            1 => BRK0NFSEL_A::_1,
            2 => BRK0NFSEL_A::_2,
            3 => BRK0NFSEL_A::_3,
            4 => BRK0NFSEL_A::_4,
            5 => BRK0NFSEL_A::_5,
            6 => BRK0NFSEL_A::_6,
            7 => BRK0NFSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK0NFSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK0NFSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BRK0NFSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BRK0NFSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == BRK0NFSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == BRK0NFSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == BRK0NFSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == BRK0NFSEL_A::_7
    }
}
impl core::ops::Deref for BRK0NFSEL_R {
    type Target = crate::FieldReader<u8, BRK0NFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK0NFSEL` writer - Brake 0 Edge Detector Filter Clock Selection"]
pub struct BRK0NFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK0NFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK0NFSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter clock = HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_0)
    }
    #[doc = "Filter clock = HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_1)
    }
    #[doc = "Filter clock = HCLK/4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_2)
    }
    #[doc = "Filter clock = HCLK/8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_3)
    }
    #[doc = "Filter clock = HCLK/16"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_4)
    }
    #[doc = "Filter clock = HCLK/32"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_5)
    }
    #[doc = "Filter clock = HCLK/64"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_6)
    }
    #[doc = "Filter clock = HCLK/128"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(BRK0NFSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `BRK0FCNT` reader - Brake 0 Edge Detector Filter Count\\nThe register bits control the Brake0 filter counter to count from 0 to BRK1FCNT."]
pub struct BRK0FCNT_R(crate::FieldReader<u8, u8>);
impl BRK0FCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRK0FCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK0FCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK0FCNT` writer - Brake 0 Edge Detector Filter Count\\nThe register bits control the Brake0 filter counter to count from 0 to BRK1FCNT."]
pub struct BRK0FCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK0FCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Brake 0 Pin Inverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK0PINV_A {
    #[doc = "0: The state of pin PWMx_BRAKE0 is passed to the negative edge detector"]
    _0 = 0,
    #[doc = "1: The inversed state of pin PWMx_BRAKE10 is passed to the negative edge detector"]
    _1 = 1,
}
impl From<BRK0PINV_A> for bool {
    #[inline(always)]
    fn from(variant: BRK0PINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK0PINV` reader - Brake 0 Pin Inverse"]
pub struct BRK0PINV_R(crate::FieldReader<bool, BRK0PINV_A>);
impl BRK0PINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK0PINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK0PINV_A {
        match self.bits {
            false => BRK0PINV_A::_0,
            true => BRK0PINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK0PINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK0PINV_A::_1
    }
}
impl core::ops::Deref for BRK0PINV_R {
    type Target = crate::FieldReader<bool, BRK0PINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK0PINV` writer - Brake 0 Pin Inverse"]
pub struct BRK0PINV_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK0PINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK0PINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The state of pin PWMx_BRAKE0 is passed to the negative edge detector"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK0PINV_A::_0)
    }
    #[doc = "The inversed state of pin PWMx_BRAKE10 is passed to the negative edge detector"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK0PINV_A::_1)
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
#[doc = "PWM Brake 1 Noise Filter Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK1NFEN_A {
    #[doc = "0: Noise filter of PWM Brake 1 Disabled"]
    _0 = 0,
    #[doc = "1: Noise filter of PWM Brake 1 Enabled"]
    _1 = 1,
}
impl From<BRK1NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRK1NFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK1NFEN` reader - PWM Brake 1 Noise Filter Enable Bit"]
pub struct BRK1NFEN_R(crate::FieldReader<bool, BRK1NFEN_A>);
impl BRK1NFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK1NFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK1NFEN_A {
        match self.bits {
            false => BRK1NFEN_A::_0,
            true => BRK1NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK1NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK1NFEN_A::_1
    }
}
impl core::ops::Deref for BRK1NFEN_R {
    type Target = crate::FieldReader<bool, BRK1NFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK1NFEN` writer - PWM Brake 1 Noise Filter Enable Bit"]
pub struct BRK1NFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK1NFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK1NFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Noise filter of PWM Brake 1 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK1NFEN_A::_0)
    }
    #[doc = "Noise filter of PWM Brake 1 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK1NFEN_A::_1)
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
#[doc = "Brake 1 Edge Detector Filter Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRK1NFSEL_A {
    #[doc = "0: Filter clock = HCLK"]
    _0 = 0,
    #[doc = "1: Filter clock = HCLK/2"]
    _1 = 1,
    #[doc = "2: Filter clock = HCLK/4"]
    _2 = 2,
    #[doc = "3: Filter clock = HCLK/8"]
    _3 = 3,
    #[doc = "4: Filter clock = HCLK/16"]
    _4 = 4,
    #[doc = "5: Filter clock = HCLK/32"]
    _5 = 5,
    #[doc = "6: Filter clock = HCLK/64"]
    _6 = 6,
    #[doc = "7: Filter clock = HCLK/128"]
    _7 = 7,
}
impl From<BRK1NFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BRK1NFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRK1NFSEL` reader - Brake 1 Edge Detector Filter Clock Selection"]
pub struct BRK1NFSEL_R(crate::FieldReader<u8, BRK1NFSEL_A>);
impl BRK1NFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRK1NFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK1NFSEL_A {
        match self.bits {
            0 => BRK1NFSEL_A::_0,
            1 => BRK1NFSEL_A::_1,
            2 => BRK1NFSEL_A::_2,
            3 => BRK1NFSEL_A::_3,
            4 => BRK1NFSEL_A::_4,
            5 => BRK1NFSEL_A::_5,
            6 => BRK1NFSEL_A::_6,
            7 => BRK1NFSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK1NFSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK1NFSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BRK1NFSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BRK1NFSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == BRK1NFSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == BRK1NFSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == BRK1NFSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == BRK1NFSEL_A::_7
    }
}
impl core::ops::Deref for BRK1NFSEL_R {
    type Target = crate::FieldReader<u8, BRK1NFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK1NFSEL` writer - Brake 1 Edge Detector Filter Clock Selection"]
pub struct BRK1NFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK1NFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK1NFSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter clock = HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_0)
    }
    #[doc = "Filter clock = HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_1)
    }
    #[doc = "Filter clock = HCLK/4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_2)
    }
    #[doc = "Filter clock = HCLK/8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_3)
    }
    #[doc = "Filter clock = HCLK/16"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_4)
    }
    #[doc = "Filter clock = HCLK/32"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_5)
    }
    #[doc = "Filter clock = HCLK/64"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_6)
    }
    #[doc = "Filter clock = HCLK/128"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(BRK1NFSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `BRK1FCNT` reader - Brake 1 Edge Detector Filter Count\\nThe register bits control the Brake1 filter counter to count from 0 to BRK1FCNT."]
pub struct BRK1FCNT_R(crate::FieldReader<u8, u8>);
impl BRK1FCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRK1FCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK1FCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK1FCNT` writer - Brake 1 Edge Detector Filter Count\\nThe register bits control the Brake1 filter counter to count from 0 to BRK1FCNT."]
pub struct BRK1FCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK1FCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Brake 1 Pin Inverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK1PINV_A {
    #[doc = "0: The state of pin PWMx_BRAKE1 is passed to the negative edge detector"]
    _0 = 0,
    #[doc = "1: The inversed state of pin PWMx_BRAKE1 is passed to the negative edge detector"]
    _1 = 1,
}
impl From<BRK1PINV_A> for bool {
    #[inline(always)]
    fn from(variant: BRK1PINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK1PINV` reader - Brake 1 Pin Inverse"]
pub struct BRK1PINV_R(crate::FieldReader<bool, BRK1PINV_A>);
impl BRK1PINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK1PINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK1PINV_A {
        match self.bits {
            false => BRK1PINV_A::_0,
            true => BRK1PINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK1PINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK1PINV_A::_1
    }
}
impl core::ops::Deref for BRK1PINV_R {
    type Target = crate::FieldReader<bool, BRK1PINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK1PINV` writer - Brake 1 Pin Inverse"]
pub struct BRK1PINV_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK1PINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK1PINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The state of pin PWMx_BRAKE1 is passed to the negative edge detector"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK1PINV_A::_0)
    }
    #[doc = "The inversed state of pin PWMx_BRAKE1 is passed to the negative edge detector"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK1PINV_A::_1)
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
#[doc = "Brake 0 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK0SRC_A {
    #[doc = "0: Brake 0 pin source come from PWM0_BRAKE0.\\nBrake 0 pin source come from PWM1_BRAKE0"]
    _0 = 0,
    #[doc = "1: Brake 0 pin source come from PWM1_BRAKE0.\\nBrake 0 pin source come from PWM0_BRAKE0"]
    _1 = 1,
}
impl From<BK0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: BK0SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK0SRC` reader - Brake 0 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
pub struct BK0SRC_R(crate::FieldReader<bool, BK0SRC_A>);
impl BK0SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK0SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK0SRC_A {
        match self.bits {
            false => BK0SRC_A::_0,
            true => BK0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BK0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BK0SRC_A::_1
    }
}
impl core::ops::Deref for BK0SRC_R {
    type Target = crate::FieldReader<bool, BK0SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK0SRC` writer - Brake 0 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
pub struct BK0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BK0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK0SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake 0 pin source come from PWM0_BRAKE0.\\nBrake 0 pin source come from PWM1_BRAKE0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BK0SRC_A::_0)
    }
    #[doc = "Brake 0 pin source come from PWM1_BRAKE0.\\nBrake 0 pin source come from PWM0_BRAKE0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BK0SRC_A::_1)
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
#[doc = "Brake 1 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK1SRC_A {
    #[doc = "0: Brake 1 pin source come from PWM0_BRAKE1.\\nBrake 1 pin source come from PWM1_BRAKE1"]
    _0 = 0,
    #[doc = "1: Brake 1 pin source come from PWM1_BRAKE1.\\nBrake 1 pin source come from PWM0_BRAKE1"]
    _1 = 1,
}
impl From<BK1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: BK1SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK1SRC` reader - Brake 1 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
pub struct BK1SRC_R(crate::FieldReader<bool, BK1SRC_A>);
impl BK1SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK1SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK1SRC_A {
        match self.bits {
            false => BK1SRC_A::_0,
            true => BK1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BK1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BK1SRC_A::_1
    }
}
impl core::ops::Deref for BK1SRC_R {
    type Target = crate::FieldReader<bool, BK1SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK1SRC` writer - Brake 1 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
pub struct BK1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BK1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK1SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake 1 pin source come from PWM0_BRAKE1.\\nBrake 1 pin source come from PWM1_BRAKE1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BK1SRC_A::_0)
    }
    #[doc = "Brake 1 pin source come from PWM1_BRAKE1.\\nBrake 1 pin source come from PWM0_BRAKE1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BK1SRC_A::_1)
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
    #[doc = "Bit 0 - PWM Brake 0 Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn brk0nfen(&self) -> BRK0NFEN_R {
        BRK0NFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Brake 0 Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn brk0nfsel(&self) -> BRK0NFSEL_R {
        BRK0NFSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Brake 0 Edge Detector Filter Count\\nThe register bits control the Brake0 filter counter to count from 0 to BRK1FCNT."]
    #[inline(always)]
    pub fn brk0fcnt(&self) -> BRK0FCNT_R {
        BRK0FCNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Brake 0 Pin Inverse"]
    #[inline(always)]
    pub fn brk0pinv(&self) -> BRK0PINV_R {
        BRK0PINV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Brake 1 Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn brk1nfen(&self) -> BRK1NFEN_R {
        BRK1NFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Brake 1 Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn brk1nfsel(&self) -> BRK1NFSEL_R {
        BRK1NFSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Brake 1 Edge Detector Filter Count\\nThe register bits control the Brake1 filter counter to count from 0 to BRK1FCNT."]
    #[inline(always)]
    pub fn brk1fcnt(&self) -> BRK1FCNT_R {
        BRK1FCNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Brake 1 Pin Inverse"]
    #[inline(always)]
    pub fn brk1pinv(&self) -> BRK1PINV_R {
        BRK1PINV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Brake 0 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
    #[inline(always)]
    pub fn bk0src(&self) -> BK0SRC_R {
        BK0SRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Brake 1 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
    #[inline(always)]
    pub fn bk1src(&self) -> BK1SRC_R {
        BK1SRC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Brake 0 Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn brk0nfen(&mut self) -> BRK0NFEN_W {
        BRK0NFEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Brake 0 Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn brk0nfsel(&mut self) -> BRK0NFSEL_W {
        BRK0NFSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Brake 0 Edge Detector Filter Count\\nThe register bits control the Brake0 filter counter to count from 0 to BRK1FCNT."]
    #[inline(always)]
    pub fn brk0fcnt(&mut self) -> BRK0FCNT_W {
        BRK0FCNT_W { w: self }
    }
    #[doc = "Bit 7 - Brake 0 Pin Inverse"]
    #[inline(always)]
    pub fn brk0pinv(&mut self) -> BRK0PINV_W {
        BRK0PINV_W { w: self }
    }
    #[doc = "Bit 8 - PWM Brake 1 Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn brk1nfen(&mut self) -> BRK1NFEN_W {
        BRK1NFEN_W { w: self }
    }
    #[doc = "Bits 9:11 - Brake 1 Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn brk1nfsel(&mut self) -> BRK1NFSEL_W {
        BRK1NFSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - Brake 1 Edge Detector Filter Count\\nThe register bits control the Brake1 filter counter to count from 0 to BRK1FCNT."]
    #[inline(always)]
    pub fn brk1fcnt(&mut self) -> BRK1FCNT_W {
        BRK1FCNT_W { w: self }
    }
    #[doc = "Bit 15 - Brake 1 Pin Inverse"]
    #[inline(always)]
    pub fn brk1pinv(&mut self) -> BRK1PINV_W {
        BRK1PINV_W { w: self }
    }
    #[doc = "Bit 16 - Brake 0 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
    #[inline(always)]
    pub fn bk0src(&mut self) -> BK0SRC_W {
        BK0SRC_W { w: self }
    }
    #[doc = "Bit 24 - Brake 1 Pin Source Select (M45xD/M45xC Only)\\nFor PWM0 setting:"]
    #[inline(always)]
    pub fn bk1src(&mut self) -> BK1SRC_W {
        BK1SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Brake Noise Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_bnf](index.html) module"]
pub struct PWM_BNF_SPEC;
impl crate::RegisterSpec for PWM_BNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_bnf::R](R) reader structure"]
impl crate::Readable for PWM_BNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_bnf::W](W) writer structure"]
impl crate::Writable for PWM_BNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_BNF to value 0"]
impl crate::Resettable for PWM_BNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
