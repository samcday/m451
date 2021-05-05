#[doc = "Register `PWM_EADCTS0` reader"]
pub struct R(crate::R<PWM_EADCTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_EADCTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_EADCTS0_SPEC>> for R {
    fn from(reader: crate::R<PWM_EADCTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_EADCTS0` writer"]
pub struct W(crate::W<PWM_EADCTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_EADCTS0_SPEC>;
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
impl core::convert::From<crate::W<PWM_EADCTS0_SPEC>> for W {
    fn from(writer: crate::W<PWM_EADCTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM_CH0 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL0_A {
    #[doc = "0: PWM_CH0 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH0 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH0 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH0 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH0 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH1 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH1 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH1 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH1 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH1 down-count CMPDAT point"]
    _9 = 9,
    #[doc = "10: PWM_CH0 up-count free CMPDAT point"]
    _10 = 10,
    #[doc = "11: PWM_CH0 down-count free CMPDAT point"]
    _11 = 11,
    #[doc = "12: PWM_CH2 up-count free CMPDAT point"]
    _12 = 12,
    #[doc = "13: PWM_CH2 down-count free CMPDAT point"]
    _13 = 13,
    #[doc = "14: PWM_CH4 up-count free CMPDAT point"]
    _14 = 14,
    #[doc = "15: PWM_CH4 down-count free CMPDAT point"]
    _15 = 15,
}
impl From<TRGSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL0` reader - PWM_CH0 Trigger EADC Source Select"]
pub struct TRGSEL0_R(crate::FieldReader<u8, TRGSEL0_A>);
impl TRGSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL0_A {
        match self.bits {
            0 => TRGSEL0_A::_0,
            1 => TRGSEL0_A::_1,
            2 => TRGSEL0_A::_2,
            3 => TRGSEL0_A::_3,
            4 => TRGSEL0_A::_4,
            5 => TRGSEL0_A::_5,
            6 => TRGSEL0_A::_6,
            7 => TRGSEL0_A::_7,
            8 => TRGSEL0_A::_8,
            9 => TRGSEL0_A::_9,
            10 => TRGSEL0_A::_10,
            11 => TRGSEL0_A::_11,
            12 => TRGSEL0_A::_12,
            13 => TRGSEL0_A::_13,
            14 => TRGSEL0_A::_14,
            15 => TRGSEL0_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL0_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL0_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL0_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL0_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL0_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL0_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL0_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL0_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL0_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL0_A::_15
    }
}
impl core::ops::Deref for TRGSEL0_R {
    type Target = crate::FieldReader<u8, TRGSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL0` writer - PWM_CH0 Trigger EADC Source Select"]
pub struct TRGSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH0 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_0)
    }
    #[doc = "PWM_CH0 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_1)
    }
    #[doc = "PWM_CH0 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_2)
    }
    #[doc = "PWM_CH0 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_3)
    }
    #[doc = "PWM_CH0 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_4)
    }
    #[doc = "PWM_CH1 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_5)
    }
    #[doc = "PWM_CH1 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_6)
    }
    #[doc = "PWM_CH1 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_7)
    }
    #[doc = "PWM_CH1 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_8)
    }
    #[doc = "PWM_CH1 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL0_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "PWM_CH0 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN0_A {
    #[doc = "0: PWM_CH0 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH0 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN0` reader - PWM_CH0 Trigger EADC Enable Bit"]
pub struct TRGEN0_R(crate::FieldReader<bool, TRGEN0_A>);
impl TRGEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN0_A {
        match self.bits {
            false => TRGEN0_A::_0,
            true => TRGEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN0_A::_1
    }
}
impl core::ops::Deref for TRGEN0_R {
    type Target = crate::FieldReader<bool, TRGEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN0` writer - PWM_CH0 Trigger EADC Enable Bit"]
pub struct TRGEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH0 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN0_A::_0)
    }
    #[doc = "PWM_CH0 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN0_A::_1)
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
#[doc = "PWM_CH1 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL1_A {
    #[doc = "0: PWM_CH0 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH0 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH0 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH0 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH0 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH1 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH1 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH1 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH1 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH1 down-count CMPDAT point"]
    _9 = 9,
    #[doc = "10: PWM_CH0 up-count free CMPDAT point"]
    _10 = 10,
    #[doc = "11: PWM_CH0 down-count free CMPDAT point"]
    _11 = 11,
    #[doc = "12: PWM_CH2 up-count free CMPDAT point"]
    _12 = 12,
    #[doc = "13: PWM_CH2 down-count free CMPDAT point"]
    _13 = 13,
    #[doc = "14: PWM_CH4 up-count free CMPDAT point"]
    _14 = 14,
    #[doc = "15: PWM_CH4 down-count free CMPDAT point"]
    _15 = 15,
}
impl From<TRGSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL1` reader - PWM_CH1 Trigger EADC Source Select"]
pub struct TRGSEL1_R(crate::FieldReader<u8, TRGSEL1_A>);
impl TRGSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL1_A {
        match self.bits {
            0 => TRGSEL1_A::_0,
            1 => TRGSEL1_A::_1,
            2 => TRGSEL1_A::_2,
            3 => TRGSEL1_A::_3,
            4 => TRGSEL1_A::_4,
            5 => TRGSEL1_A::_5,
            6 => TRGSEL1_A::_6,
            7 => TRGSEL1_A::_7,
            8 => TRGSEL1_A::_8,
            9 => TRGSEL1_A::_9,
            10 => TRGSEL1_A::_10,
            11 => TRGSEL1_A::_11,
            12 => TRGSEL1_A::_12,
            13 => TRGSEL1_A::_13,
            14 => TRGSEL1_A::_14,
            15 => TRGSEL1_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL1_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL1_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL1_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL1_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL1_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL1_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL1_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL1_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL1_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL1_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL1_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL1_A::_15
    }
}
impl core::ops::Deref for TRGSEL1_R {
    type Target = crate::FieldReader<u8, TRGSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL1` writer - PWM_CH1 Trigger EADC Source Select"]
pub struct TRGSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH0 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_0)
    }
    #[doc = "PWM_CH0 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_1)
    }
    #[doc = "PWM_CH0 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_2)
    }
    #[doc = "PWM_CH0 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_3)
    }
    #[doc = "PWM_CH0 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_4)
    }
    #[doc = "PWM_CH1 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_5)
    }
    #[doc = "PWM_CH1 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_6)
    }
    #[doc = "PWM_CH1 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_7)
    }
    #[doc = "PWM_CH1 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_8)
    }
    #[doc = "PWM_CH1 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL1_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PWM_CH1 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN1_A {
    #[doc = "0: PWM_CH1 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH1 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN1` reader - PWM_CH1 Trigger EADC Enable Bit"]
pub struct TRGEN1_R(crate::FieldReader<bool, TRGEN1_A>);
impl TRGEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN1_A {
        match self.bits {
            false => TRGEN1_A::_0,
            true => TRGEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN1_A::_1
    }
}
impl core::ops::Deref for TRGEN1_R {
    type Target = crate::FieldReader<bool, TRGEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN1` writer - PWM_CH1 Trigger EADC Enable Bit"]
pub struct TRGEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH1 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN1_A::_0)
    }
    #[doc = "PWM_CH1 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN1_A::_1)
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
#[doc = "PWM_CH2 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL2_A {
    #[doc = "0: PWM_CH2 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH2 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH2 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH2 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH2 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH3 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH3 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH3 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH3 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH3 down-count CMPDAT point"]
    _9 = 9,
    #[doc = "10: PWM_CH0 up-count free CMPDAT point"]
    _10 = 10,
    #[doc = "11: PWM_CH0 down-count free CMPDAT point"]
    _11 = 11,
    #[doc = "12: PWM_CH2 up-count free CMPDAT point"]
    _12 = 12,
    #[doc = "13: PWM_CH2 down-count free CMPDAT point"]
    _13 = 13,
    #[doc = "14: PWM_CH4 up-count free CMPDAT point"]
    _14 = 14,
    #[doc = "15: PWM_CH4 down-count free CMPDAT point"]
    _15 = 15,
}
impl From<TRGSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL2` reader - PWM_CH2 Trigger EADC Source Select"]
pub struct TRGSEL2_R(crate::FieldReader<u8, TRGSEL2_A>);
impl TRGSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL2_A {
        match self.bits {
            0 => TRGSEL2_A::_0,
            1 => TRGSEL2_A::_1,
            2 => TRGSEL2_A::_2,
            3 => TRGSEL2_A::_3,
            4 => TRGSEL2_A::_4,
            5 => TRGSEL2_A::_5,
            6 => TRGSEL2_A::_6,
            7 => TRGSEL2_A::_7,
            8 => TRGSEL2_A::_8,
            9 => TRGSEL2_A::_9,
            10 => TRGSEL2_A::_10,
            11 => TRGSEL2_A::_11,
            12 => TRGSEL2_A::_12,
            13 => TRGSEL2_A::_13,
            14 => TRGSEL2_A::_14,
            15 => TRGSEL2_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL2_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL2_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL2_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL2_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL2_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL2_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL2_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL2_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL2_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL2_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL2_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL2_A::_15
    }
}
impl core::ops::Deref for TRGSEL2_R {
    type Target = crate::FieldReader<u8, TRGSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL2` writer - PWM_CH2 Trigger EADC Source Select"]
pub struct TRGSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH2 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_0)
    }
    #[doc = "PWM_CH2 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_1)
    }
    #[doc = "PWM_CH2 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_2)
    }
    #[doc = "PWM_CH2 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_3)
    }
    #[doc = "PWM_CH2 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_4)
    }
    #[doc = "PWM_CH3 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_5)
    }
    #[doc = "PWM_CH3 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_6)
    }
    #[doc = "PWM_CH3 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_7)
    }
    #[doc = "PWM_CH3 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_8)
    }
    #[doc = "PWM_CH3 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL2_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "PWM_CH2 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN2_A {
    #[doc = "0: PWM_CH2 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH2 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN2_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN2` reader - PWM_CH2 Trigger EADC Enable Bit"]
pub struct TRGEN2_R(crate::FieldReader<bool, TRGEN2_A>);
impl TRGEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN2_A {
        match self.bits {
            false => TRGEN2_A::_0,
            true => TRGEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN2_A::_1
    }
}
impl core::ops::Deref for TRGEN2_R {
    type Target = crate::FieldReader<bool, TRGEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN2` writer - PWM_CH2 Trigger EADC Enable Bit"]
pub struct TRGEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH2 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN2_A::_0)
    }
    #[doc = "PWM_CH2 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN2_A::_1)
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
#[doc = "PWM_CH3 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL3_A {
    #[doc = "0: PWM_CH2 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH2 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH2 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH2 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH2 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH3 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH3 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH3 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH3 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH3 down-count CMPDAT point"]
    _9 = 9,
    #[doc = "10: PWM_CH0 up-count free CMPDAT point"]
    _10 = 10,
    #[doc = "11: PWM_CH0 down-count free CMPDAT point"]
    _11 = 11,
    #[doc = "12: PWM_CH2 up-count free CMPDAT point"]
    _12 = 12,
    #[doc = "13: PWM_CH2 down-count free CMPDAT point"]
    _13 = 13,
    #[doc = "14: PWM_CH4 up-count free CMPDAT point"]
    _14 = 14,
    #[doc = "15: PWM_CH4 down-count free CMPDAT point"]
    _15 = 15,
}
impl From<TRGSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL3` reader - PWM_CH3 Trigger EADC Source Select"]
pub struct TRGSEL3_R(crate::FieldReader<u8, TRGSEL3_A>);
impl TRGSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL3_A {
        match self.bits {
            0 => TRGSEL3_A::_0,
            1 => TRGSEL3_A::_1,
            2 => TRGSEL3_A::_2,
            3 => TRGSEL3_A::_3,
            4 => TRGSEL3_A::_4,
            5 => TRGSEL3_A::_5,
            6 => TRGSEL3_A::_6,
            7 => TRGSEL3_A::_7,
            8 => TRGSEL3_A::_8,
            9 => TRGSEL3_A::_9,
            10 => TRGSEL3_A::_10,
            11 => TRGSEL3_A::_11,
            12 => TRGSEL3_A::_12,
            13 => TRGSEL3_A::_13,
            14 => TRGSEL3_A::_14,
            15 => TRGSEL3_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL3_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL3_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL3_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL3_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL3_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL3_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL3_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL3_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL3_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL3_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL3_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL3_A::_15
    }
}
impl core::ops::Deref for TRGSEL3_R {
    type Target = crate::FieldReader<u8, TRGSEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL3` writer - PWM_CH3 Trigger EADC Source Select"]
pub struct TRGSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH2 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_0)
    }
    #[doc = "PWM_CH2 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_1)
    }
    #[doc = "PWM_CH2 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_2)
    }
    #[doc = "PWM_CH2 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_3)
    }
    #[doc = "PWM_CH2 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_4)
    }
    #[doc = "PWM_CH3 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_5)
    }
    #[doc = "PWM_CH3 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_6)
    }
    #[doc = "PWM_CH3 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_7)
    }
    #[doc = "PWM_CH3 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_8)
    }
    #[doc = "PWM_CH3 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL3_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "PWM_CH3 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN3_A {
    #[doc = "0: PWM_CH3 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH3 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN3_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN3` reader - PWM_CH3 Trigger EADC Enable Bit"]
pub struct TRGEN3_R(crate::FieldReader<bool, TRGEN3_A>);
impl TRGEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN3_A {
        match self.bits {
            false => TRGEN3_A::_0,
            true => TRGEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN3_A::_1
    }
}
impl core::ops::Deref for TRGEN3_R {
    type Target = crate::FieldReader<bool, TRGEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN3` writer - PWM_CH3 Trigger EADC Enable Bit"]
pub struct TRGEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH3 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN3_A::_0)
    }
    #[doc = "PWM_CH3 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_CH0 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel0(&self) -> TRGSEL0_R {
        TRGSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - PWM_CH0 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen0(&self) -> TRGEN0_R {
        TRGEN0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - PWM_CH1 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel1(&self) -> TRGSEL1_R {
        TRGSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PWM_CH1 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen1(&self) -> TRGEN1_R {
        TRGEN1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - PWM_CH2 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel2(&self) -> TRGSEL2_R {
        TRGSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - PWM_CH2 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen2(&self) -> TRGEN2_R {
        TRGEN2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - PWM_CH3 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel3(&self) -> TRGSEL3_R {
        TRGSEL3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - PWM_CH3 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen3(&self) -> TRGEN3_R {
        TRGEN3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_CH0 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel0(&mut self) -> TRGSEL0_W {
        TRGSEL0_W { w: self }
    }
    #[doc = "Bit 7 - PWM_CH0 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen0(&mut self) -> TRGEN0_W {
        TRGEN0_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_CH1 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel1(&mut self) -> TRGSEL1_W {
        TRGSEL1_W { w: self }
    }
    #[doc = "Bit 15 - PWM_CH1 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen1(&mut self) -> TRGEN1_W {
        TRGEN1_W { w: self }
    }
    #[doc = "Bits 16:19 - PWM_CH2 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel2(&mut self) -> TRGSEL2_W {
        TRGSEL2_W { w: self }
    }
    #[doc = "Bit 23 - PWM_CH2 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen2(&mut self) -> TRGEN2_W {
        TRGEN2_W { w: self }
    }
    #[doc = "Bits 24:27 - PWM_CH3 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel3(&mut self) -> TRGSEL3_W {
        TRGSEL3_W { w: self }
    }
    #[doc = "Bit 31 - PWM_CH3 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen3(&mut self) -> TRGEN3_W {
        TRGEN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Trigger EADC Source Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_eadcts0](index.html) module"]
pub struct PWM_EADCTS0_SPEC;
impl crate::RegisterSpec for PWM_EADCTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_eadcts0::R](R) reader structure"]
impl crate::Readable for PWM_EADCTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_eadcts0::W](W) writer structure"]
impl crate::Writable for PWM_EADCTS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_EADCTS0 to value 0"]
impl crate::Resettable for PWM_EADCTS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
