#[doc = "Register `PWM_EADCTS1` reader"]
pub struct R(crate::R<PWM_EADCTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_EADCTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_EADCTS1_SPEC>> for R {
    fn from(reader: crate::R<PWM_EADCTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_EADCTS1` writer"]
pub struct W(crate::W<PWM_EADCTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_EADCTS1_SPEC>;
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
impl core::convert::From<crate::W<PWM_EADCTS1_SPEC>> for W {
    fn from(writer: crate::W<PWM_EADCTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM_CH4 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL4_A {
    #[doc = "0: PWM_CH4 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH4 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH4 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH4 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH4 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH5 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH5 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH5 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH5 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH5 down-count CMPDAT point"]
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
impl From<TRGSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL4` reader - PWM_CH4 Trigger EADC Source Select"]
pub struct TRGSEL4_R(crate::FieldReader<u8, TRGSEL4_A>);
impl TRGSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL4_A {
        match self.bits {
            0 => TRGSEL4_A::_0,
            1 => TRGSEL4_A::_1,
            2 => TRGSEL4_A::_2,
            3 => TRGSEL4_A::_3,
            4 => TRGSEL4_A::_4,
            5 => TRGSEL4_A::_5,
            6 => TRGSEL4_A::_6,
            7 => TRGSEL4_A::_7,
            8 => TRGSEL4_A::_8,
            9 => TRGSEL4_A::_9,
            10 => TRGSEL4_A::_10,
            11 => TRGSEL4_A::_11,
            12 => TRGSEL4_A::_12,
            13 => TRGSEL4_A::_13,
            14 => TRGSEL4_A::_14,
            15 => TRGSEL4_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL4_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL4_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL4_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL4_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL4_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL4_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL4_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL4_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL4_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL4_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL4_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL4_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL4_A::_15
    }
}
impl core::ops::Deref for TRGSEL4_R {
    type Target = crate::FieldReader<u8, TRGSEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL4` writer - PWM_CH4 Trigger EADC Source Select"]
pub struct TRGSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH4 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_0)
    }
    #[doc = "PWM_CH4 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_1)
    }
    #[doc = "PWM_CH4 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_2)
    }
    #[doc = "PWM_CH4 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_3)
    }
    #[doc = "PWM_CH4 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_4)
    }
    #[doc = "PWM_CH5 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_5)
    }
    #[doc = "PWM_CH5 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_6)
    }
    #[doc = "PWM_CH5 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_7)
    }
    #[doc = "PWM_CH5 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_8)
    }
    #[doc = "PWM_CH5 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL4_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "PWM_CH4 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN4_A {
    #[doc = "0: PWM_CH4 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH4 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN4_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN4` reader - PWM_CH4 Trigger EADC Enable Bit"]
pub struct TRGEN4_R(crate::FieldReader<bool, TRGEN4_A>);
impl TRGEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN4_A {
        match self.bits {
            false => TRGEN4_A::_0,
            true => TRGEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN4_A::_1
    }
}
impl core::ops::Deref for TRGEN4_R {
    type Target = crate::FieldReader<bool, TRGEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN4` writer - PWM_CH4 Trigger EADC Enable Bit"]
pub struct TRGEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH4 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN4_A::_0)
    }
    #[doc = "PWM_CH4 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN4_A::_1)
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
#[doc = "PWM_CH5 Trigger EADC Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL5_A {
    #[doc = "0: PWM_CH4 zero point"]
    _0 = 0,
    #[doc = "1: PWM_CH4 period point"]
    _1 = 1,
    #[doc = "2: PWM_CH4 zero or period point"]
    _2 = 2,
    #[doc = "3: PWM_CH4 up-count CMPDAT point"]
    _3 = 3,
    #[doc = "4: PWM_CH4 down-count CMPDAT point"]
    _4 = 4,
    #[doc = "5: PWM_CH5 zero point"]
    _5 = 5,
    #[doc = "6: PWM_CH5 period point"]
    _6 = 6,
    #[doc = "7: PWM_CH5 zero or period point"]
    _7 = 7,
    #[doc = "8: PWM_CH5 up-count CMPDAT point"]
    _8 = 8,
    #[doc = "9: PWM_CH5 down-count CMPDAT point"]
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
impl From<TRGSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL5` reader - PWM_CH5 Trigger EADC Source Select"]
pub struct TRGSEL5_R(crate::FieldReader<u8, TRGSEL5_A>);
impl TRGSEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL5_A {
        match self.bits {
            0 => TRGSEL5_A::_0,
            1 => TRGSEL5_A::_1,
            2 => TRGSEL5_A::_2,
            3 => TRGSEL5_A::_3,
            4 => TRGSEL5_A::_4,
            5 => TRGSEL5_A::_5,
            6 => TRGSEL5_A::_6,
            7 => TRGSEL5_A::_7,
            8 => TRGSEL5_A::_8,
            9 => TRGSEL5_A::_9,
            10 => TRGSEL5_A::_10,
            11 => TRGSEL5_A::_11,
            12 => TRGSEL5_A::_12,
            13 => TRGSEL5_A::_13,
            14 => TRGSEL5_A::_14,
            15 => TRGSEL5_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSEL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSEL5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TRGSEL5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TRGSEL5_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TRGSEL5_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TRGSEL5_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TRGSEL5_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TRGSEL5_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == TRGSEL5_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == TRGSEL5_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TRGSEL5_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TRGSEL5_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == TRGSEL5_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == TRGSEL5_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == TRGSEL5_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == TRGSEL5_A::_15
    }
}
impl core::ops::Deref for TRGSEL5_R {
    type Target = crate::FieldReader<u8, TRGSEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL5` writer - PWM_CH5 Trigger EADC Source Select"]
pub struct TRGSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM_CH4 zero point"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_0)
    }
    #[doc = "PWM_CH4 period point"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_1)
    }
    #[doc = "PWM_CH4 zero or period point"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_2)
    }
    #[doc = "PWM_CH4 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_3)
    }
    #[doc = "PWM_CH4 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_4)
    }
    #[doc = "PWM_CH5 zero point"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_5)
    }
    #[doc = "PWM_CH5 period point"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_6)
    }
    #[doc = "PWM_CH5 zero or period point"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_7)
    }
    #[doc = "PWM_CH5 up-count CMPDAT point"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_8)
    }
    #[doc = "PWM_CH5 down-count CMPDAT point"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_9)
    }
    #[doc = "PWM_CH0 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_10)
    }
    #[doc = "PWM_CH0 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_11)
    }
    #[doc = "PWM_CH2 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_12)
    }
    #[doc = "PWM_CH2 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_13)
    }
    #[doc = "PWM_CH4 up-count free CMPDAT point"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_14)
    }
    #[doc = "PWM_CH4 down-count free CMPDAT point"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(TRGSEL5_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PWM_CH5 Trigger EADC Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN5_A {
    #[doc = "0: PWM_CH5 Trigger EADC Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH5 Trigger EADC Enabled"]
    _1 = 1,
}
impl From<TRGEN5_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN5` reader - PWM_CH5 Trigger EADC Enable Bit"]
pub struct TRGEN5_R(crate::FieldReader<bool, TRGEN5_A>);
impl TRGEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN5_A {
        match self.bits {
            false => TRGEN5_A::_0,
            true => TRGEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGEN5_A::_1
    }
}
impl core::ops::Deref for TRGEN5_R {
    type Target = crate::FieldReader<bool, TRGEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN5` writer - PWM_CH5 Trigger EADC Enable Bit"]
pub struct TRGEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH5 Trigger EADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGEN5_A::_0)
    }
    #[doc = "PWM_CH5 Trigger EADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGEN5_A::_1)
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
impl R {
    #[doc = "Bits 0:3 - PWM_CH4 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel4(&self) -> TRGSEL4_R {
        TRGSEL4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - PWM_CH4 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen4(&self) -> TRGEN4_R {
        TRGEN4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - PWM_CH5 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel5(&self) -> TRGSEL5_R {
        TRGSEL5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PWM_CH5 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen5(&self) -> TRGEN5_R {
        TRGEN5_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_CH4 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel4(&mut self) -> TRGSEL4_W {
        TRGSEL4_W { w: self }
    }
    #[doc = "Bit 7 - PWM_CH4 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen4(&mut self) -> TRGEN4_W {
        TRGEN4_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_CH5 Trigger EADC Source Select"]
    #[inline(always)]
    pub fn trgsel5(&mut self) -> TRGSEL5_W {
        TRGSEL5_W { w: self }
    }
    #[doc = "Bit 15 - PWM_CH5 Trigger EADC Enable Bit"]
    #[inline(always)]
    pub fn trgen5(&mut self) -> TRGEN5_W {
        TRGEN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Trigger EADC Source Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_eadcts1](index.html) module"]
pub struct PWM_EADCTS1_SPEC;
impl crate::RegisterSpec for PWM_EADCTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_eadcts1::R](R) reader structure"]
impl crate::Readable for PWM_EADCTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_eadcts1::W](W) writer structure"]
impl crate::Writable for PWM_EADCTS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_EADCTS1 to value 0"]
impl crate::Resettable for PWM_EADCTS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
