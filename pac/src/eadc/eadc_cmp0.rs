#[doc = "Register `EADC_CMP0` reader"]
pub struct R(crate::R<EADC_CMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_CMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_CMP0_SPEC>> for R {
    fn from(reader: crate::R<EADC_CMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_CMP0` writer"]
pub struct W(crate::W<EADC_CMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_CMP0_SPEC>;
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
impl core::convert::From<crate::W<EADC_CMP0_SPEC>> for W {
    fn from(writer: crate::W<EADC_CMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Result Compare Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPEN_A {
    #[doc = "0: Compare Disabled"]
    _0 = 0,
    #[doc = "1: Compare Enabled"]
    _1 = 1,
}
impl From<ADCMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPEN` reader - A/D Result Compare Enable Bit"]
pub struct ADCMPEN_R(crate::FieldReader<bool, ADCMPEN_A>);
impl ADCMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPEN_A {
        match self.bits {
            false => ADCMPEN_A::_0,
            true => ADCMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPEN_A::_1
    }
}
impl core::ops::Deref for ADCMPEN_R {
    type Target = crate::FieldReader<bool, ADCMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPEN` writer - A/D Result Compare Enable Bit"]
pub struct ADCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPEN_A::_0)
    }
    #[doc = "Compare Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPEN_A::_1)
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
#[doc = "A/D Result Compare Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPIE_A {
    #[doc = "0: Compare function interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare function interrupt Enabled"]
    _1 = 1,
}
impl From<ADCMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPIE` reader - A/D Result Compare Interrupt Enable Bit"]
pub struct ADCMPIE_R(crate::FieldReader<bool, ADCMPIE_A>);
impl ADCMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPIE_A {
        match self.bits {
            false => ADCMPIE_A::_0,
            true => ADCMPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPIE_A::_1
    }
}
impl core::ops::Deref for ADCMPIE_R {
    type Target = crate::FieldReader<bool, ADCMPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPIE` writer - A/D Result Compare Interrupt Enable Bit"]
pub struct ADCMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare function interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPIE_A::_0)
    }
    #[doc = "Compare function interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPIE_A::_1)
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
#[doc = "Compare Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPCOND_A {
    #[doc = "0: Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPDAT (EADC_CMPn \\[27:16\\]), the internal match counter will increase one"]
    _0 = 0,
    #[doc = "1: Set the compare condition as that when a 12-bit A/D conversion result is greater or equal to the 12-bit CMPDAT (EADC_CMPn \\[27:16\\]), the internal match counter will increase one"]
    _1 = 1,
}
impl From<CMPCOND_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCOND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPCOND` reader - Compare Condition"]
pub struct CMPCOND_R(crate::FieldReader<bool, CMPCOND_A>);
impl CMPCOND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPCOND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPCOND_A {
        match self.bits {
            false => CMPCOND_A::_0,
            true => CMPCOND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPCOND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPCOND_A::_1
    }
}
impl core::ops::Deref for CMPCOND_R {
    type Target = crate::FieldReader<bool, CMPCOND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPCOND` writer - Compare Condition"]
pub struct CMPCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPCOND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPCOND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set the compare condition as that when a 12-bit A/D conversion result is less than the 12-bit CMPDAT (EADC_CMPn \\[27:16\\]), the internal match counter will increase one"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCOND_A::_0)
    }
    #[doc = "Set the compare condition as that when a 12-bit A/D conversion result is greater or equal to the 12-bit CMPDAT (EADC_CMPn \\[27:16\\]), the internal match counter will increase one"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCOND_A::_1)
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
#[doc = "Compare Sample Module Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPSPL_A {
    #[doc = "0: Sample Module 0 conversion result EADC_DAT0 is selected to be compared"]
    _0 = 0,
    #[doc = "1: Sample Module 1 conversion result EADC_DAT1 is selected to be compared"]
    _1 = 1,
    #[doc = "2: Sample Module 2 conversion result EADC_DAT2 is selected to be compared"]
    _2 = 2,
    #[doc = "3: Sample Module 3 conversion result EADC_DAT3 is selected to be compared"]
    _3 = 3,
    #[doc = "4: Sample Module 4 conversion result EADC_DAT4 is selected to be compared"]
    _4 = 4,
    #[doc = "5: Sample Module 5 conversion result EADC_DAT5 is selected to be compared"]
    _5 = 5,
    #[doc = "6: Sample Module 6 conversion result EADC_DAT6 is selected to be compared"]
    _6 = 6,
    #[doc = "7: Sample Module 7 conversion result EADC_DAT7 is selected to be compared"]
    _7 = 7,
    #[doc = "8: Sample Module 8 conversion result EADC_DAT8 is selected to be compared"]
    _8 = 8,
    #[doc = "9: Sample Module 9 conversion result EADC_DAT9 is selected to be compared"]
    _9 = 9,
    #[doc = "10: Sample Module 10 conversion result EADC_DAT10 is selected to be compared"]
    _10 = 10,
    #[doc = "11: Sample Module 11 conversion result EADC_DAT11 is selected to be compared"]
    _11 = 11,
    #[doc = "12: Sample Module 12 conversion result EADC_DAT12 is selected to be compared"]
    _12 = 12,
    #[doc = "13: Sample Module 13 conversion result EADC_DAT13 is selected to be compared"]
    _13 = 13,
    #[doc = "14: Sample Module 14 conversion result EADC_DAT14 is selected to be compared"]
    _14 = 14,
    #[doc = "15: Sample Module 15 conversion result EADC_DAT15 is selected to be compared"]
    _15 = 15,
    #[doc = "16: Sample Module 16 conversion result EADC_DAT16 is selected to be compared"]
    _16 = 16,
    #[doc = "17: Sample Module 17 conversion result EADC_DAT17 is selected to be compared"]
    _17 = 17,
    #[doc = "18: Sample Module 18 conversion result EADC_DAT18 is selected to be compared"]
    _18 = 18,
}
impl From<CMPSPL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSPL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPSPL` reader - Compare Sample Module Selection"]
pub struct CMPSPL_R(crate::FieldReader<u8, CMPSPL_A>);
impl CMPSPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPSPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSPL_A> {
        match self.bits {
            0 => Some(CMPSPL_A::_0),
            1 => Some(CMPSPL_A::_1),
            2 => Some(CMPSPL_A::_2),
            3 => Some(CMPSPL_A::_3),
            4 => Some(CMPSPL_A::_4),
            5 => Some(CMPSPL_A::_5),
            6 => Some(CMPSPL_A::_6),
            7 => Some(CMPSPL_A::_7),
            8 => Some(CMPSPL_A::_8),
            9 => Some(CMPSPL_A::_9),
            10 => Some(CMPSPL_A::_10),
            11 => Some(CMPSPL_A::_11),
            12 => Some(CMPSPL_A::_12),
            13 => Some(CMPSPL_A::_13),
            14 => Some(CMPSPL_A::_14),
            15 => Some(CMPSPL_A::_15),
            16 => Some(CMPSPL_A::_16),
            17 => Some(CMPSPL_A::_17),
            18 => Some(CMPSPL_A::_18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPSPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPSPL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CMPSPL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CMPSPL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == CMPSPL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == CMPSPL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == CMPSPL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == CMPSPL_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == CMPSPL_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == CMPSPL_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CMPSPL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMPSPL_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == CMPSPL_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == CMPSPL_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == CMPSPL_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == CMPSPL_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == CMPSPL_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        **self == CMPSPL_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        **self == CMPSPL_A::_18
    }
}
impl core::ops::Deref for CMPSPL_R {
    type Target = crate::FieldReader<u8, CMPSPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPSPL` writer - Compare Sample Module Selection"]
pub struct CMPSPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPSPL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sample Module 0 conversion result EADC_DAT0 is selected to be compared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSPL_A::_0)
    }
    #[doc = "Sample Module 1 conversion result EADC_DAT1 is selected to be compared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSPL_A::_1)
    }
    #[doc = "Sample Module 2 conversion result EADC_DAT2 is selected to be compared"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CMPSPL_A::_2)
    }
    #[doc = "Sample Module 3 conversion result EADC_DAT3 is selected to be compared"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CMPSPL_A::_3)
    }
    #[doc = "Sample Module 4 conversion result EADC_DAT4 is selected to be compared"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CMPSPL_A::_4)
    }
    #[doc = "Sample Module 5 conversion result EADC_DAT5 is selected to be compared"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(CMPSPL_A::_5)
    }
    #[doc = "Sample Module 6 conversion result EADC_DAT6 is selected to be compared"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(CMPSPL_A::_6)
    }
    #[doc = "Sample Module 7 conversion result EADC_DAT7 is selected to be compared"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(CMPSPL_A::_7)
    }
    #[doc = "Sample Module 8 conversion result EADC_DAT8 is selected to be compared"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CMPSPL_A::_8)
    }
    #[doc = "Sample Module 9 conversion result EADC_DAT9 is selected to be compared"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(CMPSPL_A::_9)
    }
    #[doc = "Sample Module 10 conversion result EADC_DAT10 is selected to be compared"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPSPL_A::_10)
    }
    #[doc = "Sample Module 11 conversion result EADC_DAT11 is selected to be compared"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPSPL_A::_11)
    }
    #[doc = "Sample Module 12 conversion result EADC_DAT12 is selected to be compared"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(CMPSPL_A::_12)
    }
    #[doc = "Sample Module 13 conversion result EADC_DAT13 is selected to be compared"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(CMPSPL_A::_13)
    }
    #[doc = "Sample Module 14 conversion result EADC_DAT14 is selected to be compared"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(CMPSPL_A::_14)
    }
    #[doc = "Sample Module 15 conversion result EADC_DAT15 is selected to be compared"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(CMPSPL_A::_15)
    }
    #[doc = "Sample Module 16 conversion result EADC_DAT16 is selected to be compared"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CMPSPL_A::_16)
    }
    #[doc = "Sample Module 17 conversion result EADC_DAT17 is selected to be compared"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(CMPSPL_A::_17)
    }
    #[doc = "Sample Module 18 conversion result EADC_DAT18 is selected to be compared"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(CMPSPL_A::_18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `CMPMCNT` reader - Compare Match Count"]
pub struct CMPMCNT_R(crate::FieldReader<u8, u8>);
impl CMPMCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPMCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMCNT` writer - Compare Match Count"]
pub struct CMPMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Compare Window Mode Enable Bit\\nNote: This bit is only present in EADC_CMP0 and EADC_CMP2 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPWEN_A {
    #[doc = "0: ADCMPF0 (EADC_STATUS2\\[4\\]) will be set when EADC_CMP0 compared condition matched. ADCMPF2 (EADC_STATUS2\\[6\\]) will be set when EADC_CMP2 compared condition matched"]
    _0 = 0,
    #[doc = "1: ADCMPF0 (EADC_STATUS2\\[4\\]) will be set when both EADC_CMP0 and EADC_CMP1 compared condition matched. ADCMPF2 (EADC_STATUS2\\[6\\]) will be set when both EADC_CMP2 and EADC_CMP3 compared condition matched"]
    _1 = 1,
}
impl From<CMPWEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMPWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPWEN` reader - Compare Window Mode Enable Bit\\nNote: This bit is only present in EADC_CMP0 and EADC_CMP2 register."]
pub struct CMPWEN_R(crate::FieldReader<bool, CMPWEN_A>);
impl CMPWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPWEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPWEN_A {
        match self.bits {
            false => CMPWEN_A::_0,
            true => CMPWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPWEN_A::_1
    }
}
impl core::ops::Deref for CMPWEN_R {
    type Target = crate::FieldReader<bool, CMPWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPWEN` writer - Compare Window Mode Enable Bit\\nNote: This bit is only present in EADC_CMP0 and EADC_CMP2 register."]
pub struct CMPWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPWEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADCMPF0 (EADC_STATUS2\\[4\\]) will be set when EADC_CMP0 compared condition matched. ADCMPF2 (EADC_STATUS2\\[6\\]) will be set when EADC_CMP2 compared condition matched"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPWEN_A::_0)
    }
    #[doc = "ADCMPF0 (EADC_STATUS2\\[4\\]) will be set when both EADC_CMP0 and EADC_CMP1 compared condition matched. ADCMPF2 (EADC_STATUS2\\[6\\]) will be set when both EADC_CMP2 and EADC_CMP3 compared condition matched"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPWEN_A::_1)
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
#[doc = "Field `CMPDAT` reader - Comparison Data\\nThe 12 bits data is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage transition without imposing a load on software."]
pub struct CMPDAT_R(crate::FieldReader<u16, u16>);
impl CMPDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDAT` writer - Comparison Data\\nThe 12 bits data is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage transition without imposing a load on software."]
pub struct CMPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A/D Result Compare Enable Bit"]
    #[inline(always)]
    pub fn adcmpen(&self) -> ADCMPEN_R {
        ADCMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - A/D Result Compare Interrupt Enable Bit"]
    #[inline(always)]
    pub fn adcmpie(&self) -> ADCMPIE_R {
        ADCMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Condition"]
    #[inline(always)]
    pub fn cmpcond(&self) -> CMPCOND_R {
        CMPCOND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Compare Sample Module Selection"]
    #[inline(always)]
    pub fn cmpspl(&self) -> CMPSPL_R {
        CMPSPL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Compare Match Count"]
    #[inline(always)]
    pub fn cmpmcnt(&self) -> CMPMCNT_R {
        CMPMCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Compare Window Mode Enable Bit\\nNote: This bit is only present in EADC_CMP0 and EADC_CMP2 register."]
    #[inline(always)]
    pub fn cmpwen(&self) -> CMPWEN_R {
        CMPWEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Comparison Data\\nThe 12 bits data is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage transition without imposing a load on software."]
    #[inline(always)]
    pub fn cmpdat(&self) -> CMPDAT_R {
        CMPDAT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Result Compare Enable Bit"]
    #[inline(always)]
    pub fn adcmpen(&mut self) -> ADCMPEN_W {
        ADCMPEN_W { w: self }
    }
    #[doc = "Bit 1 - A/D Result Compare Interrupt Enable Bit"]
    #[inline(always)]
    pub fn adcmpie(&mut self) -> ADCMPIE_W {
        ADCMPIE_W { w: self }
    }
    #[doc = "Bit 2 - Compare Condition"]
    #[inline(always)]
    pub fn cmpcond(&mut self) -> CMPCOND_W {
        CMPCOND_W { w: self }
    }
    #[doc = "Bits 3:7 - Compare Sample Module Selection"]
    #[inline(always)]
    pub fn cmpspl(&mut self) -> CMPSPL_W {
        CMPSPL_W { w: self }
    }
    #[doc = "Bits 8:11 - Compare Match Count"]
    #[inline(always)]
    pub fn cmpmcnt(&mut self) -> CMPMCNT_W {
        CMPMCNT_W { w: self }
    }
    #[doc = "Bit 15 - Compare Window Mode Enable Bit\\nNote: This bit is only present in EADC_CMP0 and EADC_CMP2 register."]
    #[inline(always)]
    pub fn cmpwen(&mut self) -> CMPWEN_W {
        CMPWEN_W { w: self }
    }
    #[doc = "Bits 16:27 - Comparison Data\\nThe 12 bits data is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage transition without imposing a load on software."]
    #[inline(always)]
    pub fn cmpdat(&mut self) -> CMPDAT_W {
        CMPDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Result Compare Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_cmp0](index.html) module"]
pub struct EADC_CMP0_SPEC;
impl crate::RegisterSpec for EADC_CMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_cmp0::R](R) reader structure"]
impl crate::Readable for EADC_CMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_cmp0::W](W) writer structure"]
impl crate::Writable for EADC_CMP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_CMP0 to value 0"]
impl crate::Resettable for EADC_CMP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
