#[doc = "Register `PWM_IFA` reader"]
pub struct R(crate::R<PWM_IFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_IFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_IFA_SPEC>> for R {
    fn from(reader: crate::R<PWM_IFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_IFA` writer"]
pub struct W(crate::W<PWM_IFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_IFA_SPEC>;
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
impl core::convert::From<crate::W<PWM_IFA_SPEC>> for W {
    fn from(writer: crate::W<PWM_IFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFCNT0_1` reader - PWM_CH0 and PWM_CH1 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH0 and PWM_CH1 period occurs to set bit IFAIF0_1 to request the PWM period interrupt. \\nIFAIF0_1 (PWM_INTSTS0\\[7\\]) will be set in every IFCNT0_1+1 times of PWM period."]
pub struct IFCNT0_1_R(crate::FieldReader<u8, u8>);
impl IFCNT0_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFCNT0_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFCNT0_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFCNT0_1` writer - PWM_CH0 and PWM_CH1 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH0 and PWM_CH1 period occurs to set bit IFAIF0_1 to request the PWM period interrupt. \\nIFAIF0_1 (PWM_INTSTS0\\[7\\]) will be set in every IFCNT0_1+1 times of PWM period."]
pub struct IFCNT0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCNT0_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFSEL0_1_A {
    #[doc = "0: CNT equal to Zero in channel 0"]
    _0 = 0,
    #[doc = "1: CNT equal to PERIOD in channel 0"]
    _1 = 1,
    #[doc = "2: CNT equal to CMPU in channel 0"]
    _2 = 2,
    #[doc = "3: CNT equal to CMPD in channel 0"]
    _3 = 3,
    #[doc = "4: CNT equal to Zero in channel 1"]
    _4 = 4,
    #[doc = "5: CNT equal to PERIOD in channel 1"]
    _5 = 5,
    #[doc = "6: CNT equal to CMPU in channel 1"]
    _6 = 6,
    #[doc = "7: CNT equal to CMPD in channel 1"]
    _7 = 7,
}
impl From<IFSEL0_1_A> for u8 {
    #[inline(always)]
    fn from(variant: IFSEL0_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IFSEL0_1` reader - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL0_1_R(crate::FieldReader<u8, IFSEL0_1_A>);
impl IFSEL0_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFSEL0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFSEL0_1_A {
        match self.bits {
            0 => IFSEL0_1_A::_0,
            1 => IFSEL0_1_A::_1,
            2 => IFSEL0_1_A::_2,
            3 => IFSEL0_1_A::_3,
            4 => IFSEL0_1_A::_4,
            5 => IFSEL0_1_A::_5,
            6 => IFSEL0_1_A::_6,
            7 => IFSEL0_1_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFSEL0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFSEL0_1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == IFSEL0_1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == IFSEL0_1_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == IFSEL0_1_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == IFSEL0_1_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == IFSEL0_1_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == IFSEL0_1_A::_7
    }
}
impl core::ops::Deref for IFSEL0_1_R {
    type Target = crate::FieldReader<u8, IFSEL0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFSEL0_1` writer - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IFSEL0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFSEL0_1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CNT equal to Zero in channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_0)
    }
    #[doc = "CNT equal to PERIOD in channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_1)
    }
    #[doc = "CNT equal to CMPU in channel 0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_2)
    }
    #[doc = "CNT equal to CMPD in channel 0"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_3)
    }
    #[doc = "CNT equal to Zero in channel 1"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_4)
    }
    #[doc = "CNT equal to PERIOD in channel 1"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_5)
    }
    #[doc = "CNT equal to CMPU in channel 1"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_6)
    }
    #[doc = "CNT equal to CMPD in channel 1"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(IFSEL0_1_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAEN0_1_A {
    #[doc = "0: PWM_CH0 and PWM_CH1 interrupt flag accumulator Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH0 and PWM_CH1 interrupt flag accumulator Enabled"]
    _1 = 1,
}
impl From<IFAEN0_1_A> for bool {
    #[inline(always)]
    fn from(variant: IFAEN0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAEN0_1` reader - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN0_1_R(crate::FieldReader<bool, IFAEN0_1_A>);
impl IFAEN0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAEN0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAEN0_1_A {
        match self.bits {
            false => IFAEN0_1_A::_0,
            true => IFAEN0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAEN0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAEN0_1_A::_1
    }
}
impl core::ops::Deref for IFAEN0_1_R {
    type Target = crate::FieldReader<bool, IFAEN0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAEN0_1` writer - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAEN0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAEN0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH0 and PWM_CH1 interrupt flag accumulator Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAEN0_1_A::_0)
    }
    #[doc = "PWM_CH0 and PWM_CH1 interrupt flag accumulator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAEN0_1_A::_1)
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
#[doc = "Field `IFCNT2_3` reader - PWM_CH2 and PWM_CH3 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH2 and PWM_CH3 period occurs to set bit IFAIF2_3 to request the PWM period interrupt. \\nIFAIF2_3 (PWM_INTSTS0\\[15\\]) will be set in every IFCNT2_3+1 times of PWM period."]
pub struct IFCNT2_3_R(crate::FieldReader<u8, u8>);
impl IFCNT2_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFCNT2_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFCNT2_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFCNT2_3` writer - PWM_CH2 and PWM_CH3 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH2 and PWM_CH3 period occurs to set bit IFAIF2_3 to request the PWM period interrupt. \\nIFAIF2_3 (PWM_INTSTS0\\[15\\]) will be set in every IFCNT2_3+1 times of PWM period."]
pub struct IFCNT2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCNT2_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFSEL2_3_A {
    #[doc = "0: CNT equal to Zero in channel 2"]
    _0 = 0,
    #[doc = "1: CNT equal to PERIOD in channel 2"]
    _1 = 1,
    #[doc = "2: CNT equal to CMPU in channel 2"]
    _2 = 2,
    #[doc = "3: CNT equal to CMPD in channel 2"]
    _3 = 3,
    #[doc = "4: CNT equal to Zero in channel 3"]
    _4 = 4,
    #[doc = "5: CNT equal to PERIOD in channel 3"]
    _5 = 5,
    #[doc = "6: CNT equal to CMPU in channel 3"]
    _6 = 6,
    #[doc = "7: CNT equal to CMPD in channel 3"]
    _7 = 7,
}
impl From<IFSEL2_3_A> for u8 {
    #[inline(always)]
    fn from(variant: IFSEL2_3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IFSEL2_3` reader - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL2_3_R(crate::FieldReader<u8, IFSEL2_3_A>);
impl IFSEL2_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFSEL2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFSEL2_3_A {
        match self.bits {
            0 => IFSEL2_3_A::_0,
            1 => IFSEL2_3_A::_1,
            2 => IFSEL2_3_A::_2,
            3 => IFSEL2_3_A::_3,
            4 => IFSEL2_3_A::_4,
            5 => IFSEL2_3_A::_5,
            6 => IFSEL2_3_A::_6,
            7 => IFSEL2_3_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFSEL2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFSEL2_3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == IFSEL2_3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == IFSEL2_3_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == IFSEL2_3_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == IFSEL2_3_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == IFSEL2_3_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == IFSEL2_3_A::_7
    }
}
impl core::ops::Deref for IFSEL2_3_R {
    type Target = crate::FieldReader<u8, IFSEL2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFSEL2_3` writer - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IFSEL2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFSEL2_3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CNT equal to Zero in channel 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_0)
    }
    #[doc = "CNT equal to PERIOD in channel 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_1)
    }
    #[doc = "CNT equal to CMPU in channel 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_2)
    }
    #[doc = "CNT equal to CMPD in channel 2"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_3)
    }
    #[doc = "CNT equal to Zero in channel 3"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_4)
    }
    #[doc = "CNT equal to PERIOD in channel 3"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_5)
    }
    #[doc = "CNT equal to CMPU in channel 3"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_6)
    }
    #[doc = "CNT equal to CMPD in channel 3"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(IFSEL2_3_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAEN2_3_A {
    #[doc = "0: PWM_CH2 and PWM_CH3 interrupt flag accumulator Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH2 and PWM_CH3 interrupt flag accumulator Enabled"]
    _1 = 1,
}
impl From<IFAEN2_3_A> for bool {
    #[inline(always)]
    fn from(variant: IFAEN2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAEN2_3` reader - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN2_3_R(crate::FieldReader<bool, IFAEN2_3_A>);
impl IFAEN2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAEN2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAEN2_3_A {
        match self.bits {
            false => IFAEN2_3_A::_0,
            true => IFAEN2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAEN2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAEN2_3_A::_1
    }
}
impl core::ops::Deref for IFAEN2_3_R {
    type Target = crate::FieldReader<bool, IFAEN2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAEN2_3` writer - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAEN2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAEN2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH2 and PWM_CH3 interrupt flag accumulator Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAEN2_3_A::_0)
    }
    #[doc = "PWM_CH2 and PWM_CH3 interrupt flag accumulator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAEN2_3_A::_1)
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
#[doc = "Field `IFCNT4_5` reader - PWM_CH4 and PWM_CH5 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH4 and PWM_CH5 period occurs to set bit IFAIF4_5 to request the PWM period interrupt. \\nIFAIF4_5 (PWM_INTSTS0\\[23\\]) will be set in every IFCNT4_5+1 times of PWM period."]
pub struct IFCNT4_5_R(crate::FieldReader<u8, u8>);
impl IFCNT4_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFCNT4_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFCNT4_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFCNT4_5` writer - PWM_CH4 and PWM_CH5 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH4 and PWM_CH5 period occurs to set bit IFAIF4_5 to request the PWM period interrupt. \\nIFAIF4_5 (PWM_INTSTS0\\[23\\]) will be set in every IFCNT4_5+1 times of PWM period."]
pub struct IFCNT4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCNT4_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFSEL4_5_A {
    #[doc = "0: CNT equal to Zero in channel 4"]
    _0 = 0,
    #[doc = "1: CNT equal to PERIOD in channel 4"]
    _1 = 1,
    #[doc = "2: CNT equal to CMPU in channel 4"]
    _2 = 2,
    #[doc = "3: CNT equal to CMPD in channel 4"]
    _3 = 3,
    #[doc = "4: CNT equal to Zero in channel 5"]
    _4 = 4,
    #[doc = "5: CNT equal to PERIOD in channel 5"]
    _5 = 5,
    #[doc = "6: CNT equal to CMPU in channel 5"]
    _6 = 6,
    #[doc = "7: CNT equal to CMPD in channel 5"]
    _7 = 7,
}
impl From<IFSEL4_5_A> for u8 {
    #[inline(always)]
    fn from(variant: IFSEL4_5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IFSEL4_5` reader - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL4_5_R(crate::FieldReader<u8, IFSEL4_5_A>);
impl IFSEL4_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFSEL4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFSEL4_5_A {
        match self.bits {
            0 => IFSEL4_5_A::_0,
            1 => IFSEL4_5_A::_1,
            2 => IFSEL4_5_A::_2,
            3 => IFSEL4_5_A::_3,
            4 => IFSEL4_5_A::_4,
            5 => IFSEL4_5_A::_5,
            6 => IFSEL4_5_A::_6,
            7 => IFSEL4_5_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFSEL4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFSEL4_5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == IFSEL4_5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == IFSEL4_5_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == IFSEL4_5_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == IFSEL4_5_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == IFSEL4_5_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == IFSEL4_5_A::_7
    }
}
impl core::ops::Deref for IFSEL4_5_R {
    type Target = crate::FieldReader<u8, IFSEL4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFSEL4_5` writer - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Source Select"]
pub struct IFSEL4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IFSEL4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFSEL4_5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CNT equal to Zero in channel 4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_0)
    }
    #[doc = "CNT equal to PERIOD in channel 4"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_1)
    }
    #[doc = "CNT equal to CMPU in channel 4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_2)
    }
    #[doc = "CNT equal to CMPD in channel 4"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_3)
    }
    #[doc = "CNT equal to Zero in channel 5"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_4)
    }
    #[doc = "CNT equal to PERIOD in channel 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_5)
    }
    #[doc = "CNT equal to CMPU in channel 5"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_6)
    }
    #[doc = "CNT equal to CMPD in channel 5"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(IFSEL4_5_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAEN4_5_A {
    #[doc = "0: PWM_CH4 and PWM_CH5 interrupt flag accumulator Disabled"]
    _0 = 0,
    #[doc = "1: PWM_CH4 and PWM_CH5 interrupt flag accumulator Enabled"]
    _1 = 1,
}
impl From<IFAEN4_5_A> for bool {
    #[inline(always)]
    fn from(variant: IFAEN4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAEN4_5` reader - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN4_5_R(crate::FieldReader<bool, IFAEN4_5_A>);
impl IFAEN4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAEN4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAEN4_5_A {
        match self.bits {
            false => IFAEN4_5_A::_0,
            true => IFAEN4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAEN4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAEN4_5_A::_1
    }
}
impl core::ops::Deref for IFAEN4_5_R {
    type Target = crate::FieldReader<bool, IFAEN4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAEN4_5` writer - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Enable Bit"]
pub struct IFAEN4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAEN4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAEN4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_CH4 and PWM_CH5 interrupt flag accumulator Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAEN4_5_A::_0)
    }
    #[doc = "PWM_CH4 and PWM_CH5 interrupt flag accumulator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAEN4_5_A::_1)
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
impl R {
    #[doc = "Bits 0:3 - PWM_CH0 and PWM_CH1 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH0 and PWM_CH1 period occurs to set bit IFAIF0_1 to request the PWM period interrupt. \\nIFAIF0_1 (PWM_INTSTS0\\[7\\]) will be set in every IFCNT0_1+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt0_1(&self) -> IFCNT0_1_R {
        IFCNT0_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel0_1(&self) -> IFSEL0_1_R {
        IFSEL0_1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen0_1(&self) -> IFAEN0_1_R {
        IFAEN0_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - PWM_CH2 and PWM_CH3 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH2 and PWM_CH3 period occurs to set bit IFAIF2_3 to request the PWM period interrupt. \\nIFAIF2_3 (PWM_INTSTS0\\[15\\]) will be set in every IFCNT2_3+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt2_3(&self) -> IFCNT2_3_R {
        IFCNT2_3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel2_3(&self) -> IFSEL2_3_R {
        IFSEL2_3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen2_3(&self) -> IFAEN2_3_R {
        IFAEN2_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - PWM_CH4 and PWM_CH5 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH4 and PWM_CH5 period occurs to set bit IFAIF4_5 to request the PWM period interrupt. \\nIFAIF4_5 (PWM_INTSTS0\\[23\\]) will be set in every IFCNT4_5+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt4_5(&self) -> IFCNT4_5_R {
        IFCNT4_5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel4_5(&self) -> IFSEL4_5_R {
        IFSEL4_5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen4_5(&self) -> IFAEN4_5_R {
        IFAEN4_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_CH0 and PWM_CH1 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH0 and PWM_CH1 period occurs to set bit IFAIF0_1 to request the PWM period interrupt. \\nIFAIF0_1 (PWM_INTSTS0\\[7\\]) will be set in every IFCNT0_1+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt0_1(&mut self) -> IFCNT0_1_W {
        IFCNT0_1_W { w: self }
    }
    #[doc = "Bits 4:6 - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel0_1(&mut self) -> IFSEL0_1_W {
        IFSEL0_1_W { w: self }
    }
    #[doc = "Bit 7 - PWM_CH0 and PWM_CH1 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen0_1(&mut self) -> IFAEN0_1_W {
        IFAEN0_1_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_CH2 and PWM_CH3 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH2 and PWM_CH3 period occurs to set bit IFAIF2_3 to request the PWM period interrupt. \\nIFAIF2_3 (PWM_INTSTS0\\[15\\]) will be set in every IFCNT2_3+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt2_3(&mut self) -> IFCNT2_3_W {
        IFCNT2_3_W { w: self }
    }
    #[doc = "Bits 12:14 - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel2_3(&mut self) -> IFSEL2_3_W {
        IFSEL2_3_W { w: self }
    }
    #[doc = "Bit 15 - PWM_CH2 and PWM_CH3 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen2_3(&mut self) -> IFAEN2_3_W {
        IFAEN2_3_W { w: self }
    }
    #[doc = "Bits 16:19 - PWM_CH4 and PWM_CH5 Interrupt Flag Counter\\nThe register sets the count number which defines how many times of PWM_CH4 and PWM_CH5 period occurs to set bit IFAIF4_5 to request the PWM period interrupt. \\nIFAIF4_5 (PWM_INTSTS0\\[23\\]) will be set in every IFCNT4_5+1 times of PWM period."]
    #[inline(always)]
    pub fn ifcnt4_5(&mut self) -> IFCNT4_5_W {
        IFCNT4_5_W { w: self }
    }
    #[doc = "Bits 20:22 - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Source Select"]
    #[inline(always)]
    pub fn ifsel4_5(&mut self) -> IFSEL4_5_W {
        IFSEL4_5_W { w: self }
    }
    #[doc = "Bit 23 - PWM_CH4 and PWM_CH5 Interrupt Flag Accumulator Enable Bit"]
    #[inline(always)]
    pub fn ifaen4_5(&mut self) -> IFAEN4_5_W {
        IFAEN4_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Flag Accumulator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ifa](index.html) module"]
pub struct PWM_IFA_SPEC;
impl crate::RegisterSpec for PWM_IFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ifa::R](R) reader structure"]
impl crate::Readable for PWM_IFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ifa::W](W) writer structure"]
impl crate::Writable for PWM_IFA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_IFA to value 0"]
impl crate::Resettable for PWM_IFA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
