#[doc = "Register `EADC_STATUS2` reader"]
pub struct R(crate::R<EADC_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_STATUS2_SPEC>> for R {
    fn from(reader: crate::R<EADC_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_STATUS2` writer"]
pub struct W(crate::W<EADC_STATUS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_STATUS2_SPEC>;
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
impl core::convert::From<crate::W<EADC_STATUS2_SPEC>> for W {
    fn from(writer: crate::W<EADC_STATUS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D ADINT0 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIF0_A {
    #[doc = "0: No ADINT0 interrupt pulse received"]
    _0 = 0,
    #[doc = "1: ADINT0 interrupt pulse has been received"]
    _1 = 1,
}
impl From<ADIF0_A> for bool {
    #[inline(always)]
    fn from(variant: ADIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIF0` reader - A/D ADINT0 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF0_R(crate::FieldReader<bool, ADIF0_A>);
impl ADIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIF0_A {
        match self.bits {
            false => ADIF0_A::_0,
            true => ADIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADIF0_A::_1
    }
}
impl core::ops::Deref for ADIF0_R {
    type Target = crate::FieldReader<bool, ADIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIF0` writer - A/D ADINT0 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADINT0 interrupt pulse received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIF0_A::_0)
    }
    #[doc = "ADINT0 interrupt pulse has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIF0_A::_1)
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
#[doc = "A/D ADINT1 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIF1_A {
    #[doc = "0: No ADINT1 interrupt pulse received"]
    _0 = 0,
    #[doc = "1: ADINT1 interrupt pulse has been received"]
    _1 = 1,
}
impl From<ADIF1_A> for bool {
    #[inline(always)]
    fn from(variant: ADIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIF1` reader - A/D ADINT1 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF1_R(crate::FieldReader<bool, ADIF1_A>);
impl ADIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIF1_A {
        match self.bits {
            false => ADIF1_A::_0,
            true => ADIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADIF1_A::_1
    }
}
impl core::ops::Deref for ADIF1_R {
    type Target = crate::FieldReader<bool, ADIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIF1` writer - A/D ADINT1 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADINT1 interrupt pulse received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIF1_A::_0)
    }
    #[doc = "ADINT1 interrupt pulse has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIF1_A::_1)
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
#[doc = "A/D ADINT2 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it. \\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIF2_A {
    #[doc = "0: No ADINT2 interrupt pulse received"]
    _0 = 0,
    #[doc = "1: ADINT2 interrupt pulse has been received"]
    _1 = 1,
}
impl From<ADIF2_A> for bool {
    #[inline(always)]
    fn from(variant: ADIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIF2` reader - A/D ADINT2 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it. \\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF2_R(crate::FieldReader<bool, ADIF2_A>);
impl ADIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIF2_A {
        match self.bits {
            false => ADIF2_A::_0,
            true => ADIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADIF2_A::_1
    }
}
impl core::ops::Deref for ADIF2_R {
    type Target = crate::FieldReader<bool, ADIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIF2` writer - A/D ADINT2 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it. \\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADINT2 interrupt pulse received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIF2_A::_0)
    }
    #[doc = "ADINT2 interrupt pulse has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIF2_A::_1)
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
#[doc = "A/D ADINT3 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIF3_A {
    #[doc = "0: No ADINT3 interrupt pulse received"]
    _0 = 0,
    #[doc = "1: ADINT3 interrupt pulse has been received"]
    _1 = 1,
}
impl From<ADIF3_A> for bool {
    #[inline(always)]
    fn from(variant: ADIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIF3` reader - A/D ADINT3 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF3_R(crate::FieldReader<bool, ADIF3_A>);
impl ADIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIF3_A {
        match self.bits {
            false => ADIF3_A::_0,
            true => ADIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADIF3_A::_1
    }
}
impl core::ops::Deref for ADIF3_R {
    type Target = crate::FieldReader<bool, ADIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIF3` writer - A/D ADINT3 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
pub struct ADIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADINT3 interrupt pulse received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADIF3_A::_0)
    }
    #[doc = "ADINT3 interrupt pulse has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADIF3_A::_1)
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
#[doc = "ADC Compare 0 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP0 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPF0_A {
    #[doc = "0: Conversion result in EADC_DAT does not meet EADC_CMP0 register setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT meets EADC_CMP0 register setting"]
    _1 = 1,
}
impl From<ADCMPF0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPF0` reader - ADC Compare 0 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP0 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF0_R(crate::FieldReader<bool, ADCMPF0_A>);
impl ADCMPF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPF0_A {
        match self.bits {
            false => ADCMPF0_A::_0,
            true => ADCMPF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPF0_A::_1
    }
}
impl core::ops::Deref for ADCMPF0_R {
    type Target = crate::FieldReader<bool, ADCMPF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPF0` writer - ADC Compare 0 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP0 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT does not meet EADC_CMP0 register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPF0_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT meets EADC_CMP0 register setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPF0_A::_1)
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
#[doc = "ADC Compare 1 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP1 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPF1_A {
    #[doc = "0: Conversion result in EADC_DAT does not meet EADC_CMP1 register setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT meets EADC_CMP1 register setting"]
    _1 = 1,
}
impl From<ADCMPF1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPF1` reader - ADC Compare 1 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP1 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF1_R(crate::FieldReader<bool, ADCMPF1_A>);
impl ADCMPF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPF1_A {
        match self.bits {
            false => ADCMPF1_A::_0,
            true => ADCMPF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPF1_A::_1
    }
}
impl core::ops::Deref for ADCMPF1_R {
    type Target = crate::FieldReader<bool, ADCMPF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPF1` writer - ADC Compare 1 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP1 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT does not meet EADC_CMP1 register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPF1_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT meets EADC_CMP1 register setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPF1_A::_1)
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
#[doc = "ADC Compare 2 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP2 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPF2_A {
    #[doc = "0: Conversion result in EADC_DAT does not meet EADC_CMP2 register setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT meets EADC_CMP2 register setting"]
    _1 = 1,
}
impl From<ADCMPF2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPF2` reader - ADC Compare 2 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP2 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF2_R(crate::FieldReader<bool, ADCMPF2_A>);
impl ADCMPF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPF2_A {
        match self.bits {
            false => ADCMPF2_A::_0,
            true => ADCMPF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPF2_A::_1
    }
}
impl core::ops::Deref for ADCMPF2_R {
    type Target = crate::FieldReader<bool, ADCMPF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPF2` writer - ADC Compare 2 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP2 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT does not meet EADC_CMP2 register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPF2_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT meets EADC_CMP2 register setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPF2_A::_1)
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
#[doc = "ADC Compare 3 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP3 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPF3_A {
    #[doc = "0: Conversion result in EADC_DAT does not meet EADC_CMP3 register setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT meets EADC_CMP3 register setting"]
    _1 = 1,
}
impl From<ADCMPF3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPF3` reader - ADC Compare 3 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP3 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF3_R(crate::FieldReader<bool, ADCMPF3_A>);
impl ADCMPF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPF3_A {
        match self.bits {
            false => ADCMPF3_A::_0,
            true => ADCMPF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPF3_A::_1
    }
}
impl core::ops::Deref for ADCMPF3_R {
    type Target = crate::FieldReader<bool, ADCMPF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPF3` writer - ADC Compare 3 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP3 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADCMPF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT does not meet EADC_CMP3 register setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPF3_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT meets EADC_CMP3 register setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPF3_A::_1)
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
#[doc = "A/D ADINT0 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADOVIF0_A {
    #[doc = "0: ADINT0 interrupt flag is not overwritten to 1"]
    _0 = 0,
    #[doc = "1: ADINT0 interrupt flag is overwritten to 1"]
    _1 = 1,
}
impl From<ADOVIF0_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVIF0` reader - A/D ADINT0 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF0_R(crate::FieldReader<bool, ADOVIF0_A>);
impl ADOVIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADOVIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVIF0_A {
        match self.bits {
            false => ADOVIF0_A::_0,
            true => ADOVIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADOVIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADOVIF0_A::_1
    }
}
impl core::ops::Deref for ADOVIF0_R {
    type Target = crate::FieldReader<bool, ADOVIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADOVIF0` writer - A/D ADINT0 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOVIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADOVIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADINT0 interrupt flag is not overwritten to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVIF0_A::_0)
    }
    #[doc = "ADINT0 interrupt flag is overwritten to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVIF0_A::_1)
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
#[doc = "A/D ADINT1 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADOVIF1_A {
    #[doc = "0: ADINT1 interrupt flag is not overwritten to 1"]
    _0 = 0,
    #[doc = "1: ADINT1 interrupt flag is overwritten to 1"]
    _1 = 1,
}
impl From<ADOVIF1_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVIF1` reader - A/D ADINT1 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF1_R(crate::FieldReader<bool, ADOVIF1_A>);
impl ADOVIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADOVIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVIF1_A {
        match self.bits {
            false => ADOVIF1_A::_0,
            true => ADOVIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADOVIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADOVIF1_A::_1
    }
}
impl core::ops::Deref for ADOVIF1_R {
    type Target = crate::FieldReader<bool, ADOVIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADOVIF1` writer - A/D ADINT1 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOVIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADOVIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADINT1 interrupt flag is not overwritten to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVIF1_A::_0)
    }
    #[doc = "ADINT1 interrupt flag is overwritten to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVIF1_A::_1)
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
#[doc = "A/D ADINT2 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADOVIF2_A {
    #[doc = "0: ADINT2 interrupt flag is not overwritten to 1"]
    _0 = 0,
    #[doc = "1: ADINT2 interrupt flag is overwritten to 1"]
    _1 = 1,
}
impl From<ADOVIF2_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVIF2` reader - A/D ADINT2 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF2_R(crate::FieldReader<bool, ADOVIF2_A>);
impl ADOVIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADOVIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVIF2_A {
        match self.bits {
            false => ADOVIF2_A::_0,
            true => ADOVIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADOVIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADOVIF2_A::_1
    }
}
impl core::ops::Deref for ADOVIF2_R {
    type Target = crate::FieldReader<bool, ADOVIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADOVIF2` writer - A/D ADINT2 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOVIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADOVIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADINT2 interrupt flag is not overwritten to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVIF2_A::_0)
    }
    #[doc = "ADINT2 interrupt flag is overwritten to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVIF2_A::_1)
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
#[doc = "A/D ADINT3 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADOVIF3_A {
    #[doc = "0: ADINT3 interrupt flag is not overwritten to 1"]
    _0 = 0,
    #[doc = "1: ADINT3 interrupt flag is overwritten to 1"]
    _1 = 1,
}
impl From<ADOVIF3_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVIF3` reader - A/D ADINT3 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF3_R(crate::FieldReader<bool, ADOVIF3_A>);
impl ADOVIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADOVIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVIF3_A {
        match self.bits {
            false => ADOVIF3_A::_0,
            true => ADOVIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADOVIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADOVIF3_A::_1
    }
}
impl core::ops::Deref for ADOVIF3_R {
    type Target = crate::FieldReader<bool, ADOVIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADOVIF3` writer - A/D ADINT3 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
pub struct ADOVIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOVIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADOVIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADINT3 interrupt flag is not overwritten to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVIF3_A::_0)
    }
    #[doc = "ADINT3 interrupt flag is overwritten to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVIF3_A::_1)
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
#[doc = "ADC Compare 0 Output Status\\nThe 12 bits compare0 data CMPDAT0 (EADC_CMP0\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPO0_A {
    #[doc = "0: Conversion result in EADC_DAT less than CMPDAT0 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT great than or equal CMPDAT0 setting"]
    _1 = 1,
}
impl From<ADCMPO0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPO0` reader - ADC Compare 0 Output Status\\nThe 12 bits compare0 data CMPDAT0 (EADC_CMP0\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO0_R(crate::FieldReader<bool, ADCMPO0_A>);
impl ADCMPO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPO0_A {
        match self.bits {
            false => ADCMPO0_A::_0,
            true => ADCMPO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPO0_A::_1
    }
}
impl core::ops::Deref for ADCMPO0_R {
    type Target = crate::FieldReader<bool, ADCMPO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPO0` writer - ADC Compare 0 Output Status\\nThe 12 bits compare0 data CMPDAT0 (EADC_CMP0\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT less than CMPDAT0 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPO0_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT great than or equal CMPDAT0 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPO0_A::_1)
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
#[doc = "ADC Compare 1 Output Status\\nThe 12 bits compare1 data CMPDAT1 (EADC_CMP1\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPO1_A {
    #[doc = "0: Conversion result in EADC_DAT less than CMPDAT1 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT great than or equal CMPDAT1 setting"]
    _1 = 1,
}
impl From<ADCMPO1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPO1` reader - ADC Compare 1 Output Status\\nThe 12 bits compare1 data CMPDAT1 (EADC_CMP1\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO1_R(crate::FieldReader<bool, ADCMPO1_A>);
impl ADCMPO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPO1_A {
        match self.bits {
            false => ADCMPO1_A::_0,
            true => ADCMPO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPO1_A::_1
    }
}
impl core::ops::Deref for ADCMPO1_R {
    type Target = crate::FieldReader<bool, ADCMPO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPO1` writer - ADC Compare 1 Output Status\\nThe 12 bits compare1 data CMPDAT1 (EADC_CMP1\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPO1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT less than CMPDAT1 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPO1_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT great than or equal CMPDAT1 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPO1_A::_1)
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
#[doc = "ADC Compare 2 Output Status\\nThe 12 bits compare2 data CMPDAT2 (EADC_CMP2\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPO2_A {
    #[doc = "0: Conversion result in EADC_DAT less than CMPDAT2 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT great than or equal CMPDAT2 setting"]
    _1 = 1,
}
impl From<ADCMPO2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPO2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPO2` reader - ADC Compare 2 Output Status\\nThe 12 bits compare2 data CMPDAT2 (EADC_CMP2\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO2_R(crate::FieldReader<bool, ADCMPO2_A>);
impl ADCMPO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPO2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPO2_A {
        match self.bits {
            false => ADCMPO2_A::_0,
            true => ADCMPO2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPO2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPO2_A::_1
    }
}
impl core::ops::Deref for ADCMPO2_R {
    type Target = crate::FieldReader<bool, ADCMPO2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPO2` writer - ADC Compare 2 Output Status\\nThe 12 bits compare2 data CMPDAT2 (EADC_CMP2\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPO2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT less than CMPDAT2 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPO2_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT great than or equal CMPDAT2 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPO2_A::_1)
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
#[doc = "ADC Compare 3 Output Status\\nThe 12 bits compare3 data CMPDAT3 (EADC_CMP3\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPO3_A {
    #[doc = "0: Conversion result in EADC_DAT less than CMPDAT3 setting"]
    _0 = 0,
    #[doc = "1: Conversion result in EADC_DAT great than or equal CMPDAT3 setting"]
    _1 = 1,
}
impl From<ADCMPO3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMPO3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMPO3` reader - ADC Compare 3 Output Status\\nThe 12 bits compare3 data CMPDAT3 (EADC_CMP3\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO3_R(crate::FieldReader<bool, ADCMPO3_A>);
impl ADCMPO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCMPO3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMPO3_A {
        match self.bits {
            false => ADCMPO3_A::_0,
            true => ADCMPO3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCMPO3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCMPO3_A::_1
    }
}
impl core::ops::Deref for ADCMPO3_R {
    type Target = crate::FieldReader<bool, ADCMPO3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPO3` writer - ADC Compare 3 Output Status\\nThe 12 bits compare3 data CMPDAT3 (EADC_CMP3\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
pub struct ADCMPO3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPO3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion result in EADC_DAT less than CMPDAT3 setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCMPO3_A::_0)
    }
    #[doc = "Conversion result in EADC_DAT great than or equal CMPDAT3 setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCMPO3_A::_1)
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
#[doc = "Field `CHANNEL` reader - Current Conversion Channel"]
pub struct CHANNEL_R(crate::FieldReader<u8, u8>);
impl CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL` writer - Current Conversion Channel"]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Busy/Idle\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: EADC is in idle state"]
    _0 = 0,
    #[doc = "1: EADC is busy at conversion"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy/Idle\\nNote: This bit is read only."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - Busy/Idle\\nNote: This bit is read only."]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EADC is in idle state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSY_A::_0)
    }
    #[doc = "EADC is busy at conversion"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSY_A::_1)
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
#[doc = "All A/D Interrupt Flag Overrun Bits Check \\nNote: This bit will keep 1 when any ADOVIFn Flag is equal to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADOVIF_A {
    #[doc = "0: None of ADINT interrupt flag ADOVIFn (EADC_STATUS2\\[11:8\\]) is overwritten to 1"]
    _0 = 0,
    #[doc = "1: Any one of ADINT interrupt flag ADOVIFn (EADC_STATUS2\\[11:8\\]) is overwritten to 1"]
    _1 = 1,
}
impl From<ADOVIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADOVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADOVIF` reader - All A/D Interrupt Flag Overrun Bits Check \\nNote: This bit will keep 1 when any ADOVIFn Flag is equal to 1."]
pub struct ADOVIF_R(crate::FieldReader<bool, ADOVIF_A>);
impl ADOVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADOVIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADOVIF_A {
        match self.bits {
            false => ADOVIF_A::_0,
            true => ADOVIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADOVIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADOVIF_A::_1
    }
}
impl core::ops::Deref for ADOVIF_R {
    type Target = crate::FieldReader<bool, ADOVIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADOVIF` writer - All A/D Interrupt Flag Overrun Bits Check \\nNote: This bit will keep 1 when any ADOVIFn Flag is equal to 1."]
pub struct ADOVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOVIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADOVIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None of ADINT interrupt flag ADOVIFn (EADC_STATUS2\\[11:8\\]) is overwritten to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADOVIF_A::_0)
    }
    #[doc = "Any one of ADINT interrupt flag ADOVIFn (EADC_STATUS2\\[11:8\\]) is overwritten to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADOVIF_A::_1)
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
#[doc = "for All A/D Sample Module Start of Conversion Overrun Flags Check\\nNote: This bit will keep 1 when any SPOVFn Flag is equal to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOVF_A {
    #[doc = "0: None of sample module event overrun flag SPOVFn (EADC_OVSTS\\[n\\]) is set to 1"]
    _0 = 0,
    #[doc = "1: Any one of sample module event overrun flag SPOVFn (EADC_OVSTS\\[n\\])  is set to 1"]
    _1 = 1,
}
impl From<STOVF_A> for bool {
    #[inline(always)]
    fn from(variant: STOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOVF` reader - for All A/D Sample Module Start of Conversion Overrun Flags Check\\nNote: This bit will keep 1 when any SPOVFn Flag is equal to 1."]
pub struct STOVF_R(crate::FieldReader<bool, STOVF_A>);
impl STOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOVF_A {
        match self.bits {
            false => STOVF_A::_0,
            true => STOVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOVF_A::_1
    }
}
impl core::ops::Deref for STOVF_R {
    type Target = crate::FieldReader<bool, STOVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOVF` writer - for All A/D Sample Module Start of Conversion Overrun Flags Check\\nNote: This bit will keep 1 when any SPOVFn Flag is equal to 1."]
pub struct STOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> STOVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None of sample module event overrun flag SPOVFn (EADC_OVSTS\\[n\\]) is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOVF_A::_0)
    }
    #[doc = "Any one of sample module event overrun flag SPOVFn (EADC_OVSTS\\[n\\]) is set to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOVF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "for All Sample Module A/D Result Data Register EADC_DAT Data Valid Flag Check\\nNote: This bit will keep 1 when any VALIDn Flag is equal to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALID_A {
    #[doc = "0: None of sample module data register valid flag VALIDn (EADC_DATn\\[17\\]) is set to 1"]
    _0 = 0,
    #[doc = "1: Any one of sample module data register valid flag VALIDn (EADC_DATn\\[17\\]) is set to 1"]
    _1 = 1,
}
impl From<AVALID_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVALID` reader - for All Sample Module A/D Result Data Register EADC_DAT Data Valid Flag Check\\nNote: This bit will keep 1 when any VALIDn Flag is equal to 1."]
pub struct AVALID_R(crate::FieldReader<bool, AVALID_A>);
impl AVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_A {
        match self.bits {
            false => AVALID_A::_0,
            true => AVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVALID_A::_1
    }
}
impl core::ops::Deref for AVALID_R {
    type Target = crate::FieldReader<bool, AVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVALID` writer - for All Sample Module A/D Result Data Register EADC_DAT Data Valid Flag Check\\nNote: This bit will keep 1 when any VALIDn Flag is equal to 1."]
pub struct AVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None of sample module data register valid flag VALIDn (EADC_DATn\\[17\\]) is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVALID_A::_0)
    }
    #[doc = "Any one of sample module data register valid flag VALIDn (EADC_DATn\\[17\\]) is set to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVALID_A::_1)
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
#[doc = "for All Sample Module A/D Result Data Register Overrun Flags Check \\nNote: This bit will keep 1 when any OVn Flag is equal to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOV_A {
    #[doc = "0: None of sample module data register overrun flag OVn (EADC_DATn\\[16\\]) is set to 1"]
    _0 = 0,
    #[doc = "1: Any one of sample module data register overrun flag OVn (EADC_DATn\\[16\\]) is set to 1"]
    _1 = 1,
}
impl From<AOV_A> for bool {
    #[inline(always)]
    fn from(variant: AOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOV` reader - for All Sample Module A/D Result Data Register Overrun Flags Check \\nNote: This bit will keep 1 when any OVn Flag is equal to 1."]
pub struct AOV_R(crate::FieldReader<bool, AOV_A>);
impl AOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        AOV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOV_A {
        match self.bits {
            false => AOV_A::_0,
            true => AOV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AOV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AOV_A::_1
    }
}
impl core::ops::Deref for AOV_R {
    type Target = crate::FieldReader<bool, AOV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AOV` writer - for All Sample Module A/D Result Data Register Overrun Flags Check \\nNote: This bit will keep 1 when any OVn Flag is equal to 1."]
pub struct AOV_W<'a> {
    w: &'a mut W,
}
impl<'a> AOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AOV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None of sample module data register overrun flag OVn (EADC_DATn\\[16\\]) is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AOV_A::_0)
    }
    #[doc = "Any one of sample module data register overrun flag OVn (EADC_DATn\\[16\\]) is set to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AOV_A::_1)
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
impl R {
    #[doc = "Bit 0 - A/D ADINT0 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif0(&self) -> ADIF0_R {
        ADIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - A/D ADINT1 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif1(&self) -> ADIF1_R {
        ADIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A/D ADINT2 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it. \\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif2(&self) -> ADIF2_R {
        ADIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A/D ADINT3 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif3(&self) -> ADIF3_R {
        ADIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC Compare 0 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP0 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf0(&self) -> ADCMPF0_R {
        ADCMPF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC Compare 1 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP1 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf1(&self) -> ADCMPF1_R {
        ADCMPF1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC Compare 2 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP2 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf2(&self) -> ADCMPF2_R {
        ADCMPF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC Compare 3 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP3 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf3(&self) -> ADCMPF3_R {
        ADCMPF3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A/D ADINT0 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif0(&self) -> ADOVIF0_R {
        ADOVIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A/D ADINT1 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif1(&self) -> ADOVIF1_R {
        ADOVIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A/D ADINT2 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif2(&self) -> ADOVIF2_R {
        ADOVIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - A/D ADINT3 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif3(&self) -> ADOVIF3_R {
        ADOVIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC Compare 0 Output Status\\nThe 12 bits compare0 data CMPDAT0 (EADC_CMP0\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo0(&self) -> ADCMPO0_R {
        ADCMPO0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC Compare 1 Output Status\\nThe 12 bits compare1 data CMPDAT1 (EADC_CMP1\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo1(&self) -> ADCMPO1_R {
        ADCMPO1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC Compare 2 Output Status\\nThe 12 bits compare2 data CMPDAT2 (EADC_CMP2\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo2(&self) -> ADCMPO2_R {
        ADCMPO2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC Compare 3 Output Status\\nThe 12 bits compare3 data CMPDAT3 (EADC_CMP3\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo3(&self) -> ADCMPO3_R {
        ADCMPO3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Current Conversion Channel"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Busy/Idle\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - All A/D Interrupt Flag Overrun Bits Check \\nNote: This bit will keep 1 when any ADOVIFn Flag is equal to 1."]
    #[inline(always)]
    pub fn adovif(&self) -> ADOVIF_R {
        ADOVIF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - for All A/D Sample Module Start of Conversion Overrun Flags Check\\nNote: This bit will keep 1 when any SPOVFn Flag is equal to 1."]
    #[inline(always)]
    pub fn stovf(&self) -> STOVF_R {
        STOVF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - for All Sample Module A/D Result Data Register EADC_DAT Data Valid Flag Check\\nNote: This bit will keep 1 when any VALIDn Flag is equal to 1."]
    #[inline(always)]
    pub fn avalid(&self) -> AVALID_R {
        AVALID_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - for All Sample Module A/D Result Data Register Overrun Flags Check \\nNote: This bit will keep 1 when any OVn Flag is equal to 1."]
    #[inline(always)]
    pub fn aov(&self) -> AOV_R {
        AOV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D ADINT0 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif0(&mut self) -> ADIF0_W {
        ADIF0_W { w: self }
    }
    #[doc = "Bit 1 - A/D ADINT1 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif1(&mut self) -> ADIF1_W {
        ADIF1_W { w: self }
    }
    #[doc = "Bit 2 - A/D ADINT2 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it. \\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif2(&mut self) -> ADIF2_W {
        ADIF2_W { w: self }
    }
    #[doc = "Bit 3 - A/D ADINT3 Interrupt Flag\\nNote1: This bit is cleared by writing 1 to it.\\nNote2:This bit indicates whether an A/D conversion of specific sample module has been completed"]
    #[inline(always)]
    pub fn adif3(&mut self) -> ADIF3_W {
        ADIF3_W { w: self }
    }
    #[doc = "Bit 4 - ADC Compare 0 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP0 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf0(&mut self) -> ADCMPF0_W {
        ADCMPF0_W { w: self }
    }
    #[doc = "Bit 5 - ADC Compare 1 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP1 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf1(&mut self) -> ADCMPF1_W {
        ADCMPF1_W { w: self }
    }
    #[doc = "Bit 6 - ADC Compare 2 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP2 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf2(&mut self) -> ADCMPF2_W {
        ADCMPF2_W { w: self }
    }
    #[doc = "Bit 7 - ADC Compare 3 Flag\\nWhen the specific sample module A/D conversion result meets setting condition in EADC_CMP3 then this bit is set to 1.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adcmpf3(&mut self) -> ADCMPF3_W {
        ADCMPF3_W { w: self }
    }
    #[doc = "Bit 8 - A/D ADINT0 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif0(&mut self) -> ADOVIF0_W {
        ADOVIF0_W { w: self }
    }
    #[doc = "Bit 9 - A/D ADINT1 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif1(&mut self) -> ADOVIF1_W {
        ADOVIF1_W { w: self }
    }
    #[doc = "Bit 10 - A/D ADINT2 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif2(&mut self) -> ADOVIF2_W {
        ADOVIF2_W { w: self }
    }
    #[doc = "Bit 11 - A/D ADINT3 Interrupt Flag Overrun\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn adovif3(&mut self) -> ADOVIF3_W {
        ADOVIF3_W { w: self }
    }
    #[doc = "Bit 12 - ADC Compare 0 Output Status\\nThe 12 bits compare0 data CMPDAT0 (EADC_CMP0\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo0(&mut self) -> ADCMPO0_W {
        ADCMPO0_W { w: self }
    }
    #[doc = "Bit 13 - ADC Compare 1 Output Status\\nThe 12 bits compare1 data CMPDAT1 (EADC_CMP1\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo1(&mut self) -> ADCMPO1_W {
        ADCMPO1_W { w: self }
    }
    #[doc = "Bit 14 - ADC Compare 2 Output Status\\nThe 12 bits compare2 data CMPDAT2 (EADC_CMP2\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo2(&mut self) -> ADCMPO2_W {
        ADCMPO2_W { w: self }
    }
    #[doc = "Bit 15 - ADC Compare 3 Output Status\\nThe 12 bits compare3 data CMPDAT3 (EADC_CMP3\\[27:16\\]) is used to compare with conversion result of specified sample module. User can use it to monitor the external analog input pin voltage status."]
    #[inline(always)]
    pub fn adcmpo3(&mut self) -> ADCMPO3_W {
        ADCMPO3_W { w: self }
    }
    #[doc = "Bits 16:20 - Current Conversion Channel"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
    #[doc = "Bit 23 - Busy/Idle\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 24 - All A/D Interrupt Flag Overrun Bits Check \\nNote: This bit will keep 1 when any ADOVIFn Flag is equal to 1."]
    #[inline(always)]
    pub fn adovif(&mut self) -> ADOVIF_W {
        ADOVIF_W { w: self }
    }
    #[doc = "Bit 25 - for All A/D Sample Module Start of Conversion Overrun Flags Check\\nNote: This bit will keep 1 when any SPOVFn Flag is equal to 1."]
    #[inline(always)]
    pub fn stovf(&mut self) -> STOVF_W {
        STOVF_W { w: self }
    }
    #[doc = "Bit 26 - for All Sample Module A/D Result Data Register EADC_DAT Data Valid Flag Check\\nNote: This bit will keep 1 when any VALIDn Flag is equal to 1."]
    #[inline(always)]
    pub fn avalid(&mut self) -> AVALID_W {
        AVALID_W { w: self }
    }
    #[doc = "Bit 27 - for All Sample Module A/D Result Data Register Overrun Flags Check \\nNote: This bit will keep 1 when any OVn Flag is equal to 1."]
    #[inline(always)]
    pub fn aov(&mut self) -> AOV_W {
        AOV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_status2](index.html) module"]
pub struct EADC_STATUS2_SPEC;
impl crate::RegisterSpec for EADC_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_status2::R](R) reader structure"]
impl crate::Readable for EADC_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_status2::W](W) writer structure"]
impl crate::Writable for EADC_STATUS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_STATUS2 to value 0"]
impl crate::Resettable for EADC_STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
