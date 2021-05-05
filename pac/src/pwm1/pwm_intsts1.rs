#[doc = "Register `PWM_INTSTS1` reader"]
pub struct R(crate::R<PWM_INTSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INTSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_INTSTS1_SPEC>> for R {
    fn from(reader: crate::R<PWM_INTSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_INTSTS1` writer"]
pub struct W(crate::W<PWM_INTSTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INTSTS1_SPEC>;
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
impl core::convert::From<crate::W<PWM_INTSTS1_SPEC>> for W {
    fn from(writer: crate::W<PWM_INTSTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Channel0 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF0_A {
    #[doc = "0: PWM channel0 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel0 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF0` reader - PWM Channel0 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF0_R(crate::FieldReader<bool, BRKEIF0_A>);
impl BRKEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF0_A {
        match self.bits {
            false => BRKEIF0_A::_0,
            true => BRKEIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF0_A::_1
    }
}
impl core::ops::Deref for BRKEIF0_R {
    type Target = crate::FieldReader<bool, BRKEIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF0` writer - PWM Channel0 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel0 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF0_A::_0)
    }
    #[doc = "When PWM channel0 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF0_A::_1)
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
#[doc = "PWM Channel1 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF1_A {
    #[doc = "0: PWM channel1 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel1 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF1` reader - PWM Channel1 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF1_R(crate::FieldReader<bool, BRKEIF1_A>);
impl BRKEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF1_A {
        match self.bits {
            false => BRKEIF1_A::_0,
            true => BRKEIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF1_A::_1
    }
}
impl core::ops::Deref for BRKEIF1_R {
    type Target = crate::FieldReader<bool, BRKEIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF1` writer - PWM Channel1 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel1 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF1_A::_0)
    }
    #[doc = "When PWM channel1 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF1_A::_1)
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
#[doc = "PWM Channel2 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF2_A {
    #[doc = "0: PWM channel2 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel2 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF2_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF2` reader - PWM Channel2 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF2_R(crate::FieldReader<bool, BRKEIF2_A>);
impl BRKEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF2_A {
        match self.bits {
            false => BRKEIF2_A::_0,
            true => BRKEIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF2_A::_1
    }
}
impl core::ops::Deref for BRKEIF2_R {
    type Target = crate::FieldReader<bool, BRKEIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF2` writer - PWM Channel2 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel2 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF2_A::_0)
    }
    #[doc = "When PWM channel2 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF2_A::_1)
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
#[doc = "PWM Channel3 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF3_A {
    #[doc = "0: PWM channel3 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel3 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF3` reader - PWM Channel3 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF3_R(crate::FieldReader<bool, BRKEIF3_A>);
impl BRKEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF3_A {
        match self.bits {
            false => BRKEIF3_A::_0,
            true => BRKEIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF3_A::_1
    }
}
impl core::ops::Deref for BRKEIF3_R {
    type Target = crate::FieldReader<bool, BRKEIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF3` writer - PWM Channel3 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel3 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF3_A::_0)
    }
    #[doc = "When PWM channel3 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF3_A::_1)
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
#[doc = "PWM Channel4 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF4_A {
    #[doc = "0: PWM channel4 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel4 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF4` reader - PWM Channel4 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF4_R(crate::FieldReader<bool, BRKEIF4_A>);
impl BRKEIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF4_A {
        match self.bits {
            false => BRKEIF4_A::_0,
            true => BRKEIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF4_A::_1
    }
}
impl core::ops::Deref for BRKEIF4_R {
    type Target = crate::FieldReader<bool, BRKEIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF4` writer - PWM Channel4 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel4 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF4_A::_0)
    }
    #[doc = "When PWM channel4 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF4_A::_1)
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
#[doc = "PWM Channel5 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIF5_A {
    #[doc = "0: PWM channel5 edge-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel5 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKEIF5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIF5` reader - PWM Channel5 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF5_R(crate::FieldReader<bool, BRKEIF5_A>);
impl BRKEIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIF5_A {
        match self.bits {
            false => BRKEIF5_A::_0,
            true => BRKEIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIF5_A::_1
    }
}
impl core::ops::Deref for BRKEIF5_R {
    type Target = crate::FieldReader<bool, BRKEIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIF5` writer - PWM Channel5 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel5 edge-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIF5_A::_0)
    }
    #[doc = "When PWM channel5 edge-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIF5_A::_1)
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
#[doc = "PWM Channel0 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF0_A {
    #[doc = "0: PWM channel0 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel0 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF0_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF0` reader - PWM Channel0 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF0_R(crate::FieldReader<bool, BRKLIF0_A>);
impl BRKLIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF0_A {
        match self.bits {
            false => BRKLIF0_A::_0,
            true => BRKLIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF0_A::_1
    }
}
impl core::ops::Deref for BRKLIF0_R {
    type Target = crate::FieldReader<bool, BRKLIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF0` writer - PWM Channel0 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel0 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF0_A::_0)
    }
    #[doc = "When PWM channel0 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF0_A::_1)
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
#[doc = "PWM Channel1 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF1_A {
    #[doc = "0: PWM channel1 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel1 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF1` reader - PWM Channel1 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF1_R(crate::FieldReader<bool, BRKLIF1_A>);
impl BRKLIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF1_A {
        match self.bits {
            false => BRKLIF1_A::_0,
            true => BRKLIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF1_A::_1
    }
}
impl core::ops::Deref for BRKLIF1_R {
    type Target = crate::FieldReader<bool, BRKLIF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF1` writer - PWM Channel1 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel1 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF1_A::_0)
    }
    #[doc = "When PWM channel1 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF1_A::_1)
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
#[doc = "PWM Channel2 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF2_A {
    #[doc = "0: PWM channel2 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel2 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF2_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF2` reader - PWM Channel2 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF2_R(crate::FieldReader<bool, BRKLIF2_A>);
impl BRKLIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF2_A {
        match self.bits {
            false => BRKLIF2_A::_0,
            true => BRKLIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF2_A::_1
    }
}
impl core::ops::Deref for BRKLIF2_R {
    type Target = crate::FieldReader<bool, BRKLIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF2` writer - PWM Channel2 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel2 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF2_A::_0)
    }
    #[doc = "When PWM channel2 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF2_A::_1)
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
#[doc = "PWM Channel3 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF3_A {
    #[doc = "0: PWM channel3 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel3 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF3` reader - PWM Channel3 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF3_R(crate::FieldReader<bool, BRKLIF3_A>);
impl BRKLIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF3_A {
        match self.bits {
            false => BRKLIF3_A::_0,
            true => BRKLIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF3_A::_1
    }
}
impl core::ops::Deref for BRKLIF3_R {
    type Target = crate::FieldReader<bool, BRKLIF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF3` writer - PWM Channel3 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel3 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF3_A::_0)
    }
    #[doc = "When PWM channel3 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF3_A::_1)
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
#[doc = "PWM Channel4 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF4_A {
    #[doc = "0: PWM channel4 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel4 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF4_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF4` reader - PWM Channel4 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF4_R(crate::FieldReader<bool, BRKLIF4_A>);
impl BRKLIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF4_A {
        match self.bits {
            false => BRKLIF4_A::_0,
            true => BRKLIF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF4_A::_1
    }
}
impl core::ops::Deref for BRKLIF4_R {
    type Target = crate::FieldReader<bool, BRKLIF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF4` writer - PWM Channel4 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel4 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF4_A::_0)
    }
    #[doc = "When PWM channel4 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF4_A::_1)
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
#[doc = "PWM Channel5 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIF5_A {
    #[doc = "0: PWM channel5 level-detect brake event do not happened"]
    _0 = 0,
    #[doc = "1: When PWM channel5 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKLIF5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIF5` reader - PWM Channel5 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF5_R(crate::FieldReader<bool, BRKLIF5_A>);
impl BRKLIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIF5_A {
        match self.bits {
            false => BRKLIF5_A::_0,
            true => BRKLIF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIF5_A::_1
    }
}
impl core::ops::Deref for BRKLIF5_R {
    type Target = crate::FieldReader<bool, BRKLIF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIF5` writer - PWM Channel5 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel5 level-detect brake event do not happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIF5_A::_0)
    }
    #[doc = "When PWM channel5 level-detect brake event happened, this bit is set to 1, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIF5_A::_1)
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
#[doc = "PWM Channel0 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS0_A {
    #[doc = "0: PWM channel0 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel0 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel0 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS0_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS0` reader - PWM Channel0 Edge-detect Brake Status"]
pub struct BRKESTS0_R(crate::FieldReader<bool, BRKESTS0_A>);
impl BRKESTS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS0_A {
        match self.bits {
            false => BRKESTS0_A::_0,
            true => BRKESTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS0_A::_1
    }
}
impl core::ops::Deref for BRKESTS0_R {
    type Target = crate::FieldReader<bool, BRKESTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS0` writer - PWM Channel0 Edge-detect Brake Status"]
pub struct BRKESTS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel0 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS0_A::_0)
    }
    #[doc = "When PWM channel0 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel0 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS0_A::_1)
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
#[doc = "PWM Channel1 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS1_A {
    #[doc = "0: PWM channel1 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel1 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel1 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS1` reader - PWM Channel1 Edge-detect Brake Status"]
pub struct BRKESTS1_R(crate::FieldReader<bool, BRKESTS1_A>);
impl BRKESTS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS1_A {
        match self.bits {
            false => BRKESTS1_A::_0,
            true => BRKESTS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS1_A::_1
    }
}
impl core::ops::Deref for BRKESTS1_R {
    type Target = crate::FieldReader<bool, BRKESTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS1` writer - PWM Channel1 Edge-detect Brake Status"]
pub struct BRKESTS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel1 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS1_A::_0)
    }
    #[doc = "When PWM channel1 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel1 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS1_A::_1)
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
#[doc = "PWM Channel2 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS2_A {
    #[doc = "0: PWM channel2 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel2 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel2 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS2_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS2` reader - PWM Channel2 Edge-detect Brake Status"]
pub struct BRKESTS2_R(crate::FieldReader<bool, BRKESTS2_A>);
impl BRKESTS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS2_A {
        match self.bits {
            false => BRKESTS2_A::_0,
            true => BRKESTS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS2_A::_1
    }
}
impl core::ops::Deref for BRKESTS2_R {
    type Target = crate::FieldReader<bool, BRKESTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS2` writer - PWM Channel2 Edge-detect Brake Status"]
pub struct BRKESTS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel2 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS2_A::_0)
    }
    #[doc = "When PWM channel2 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel2 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS2_A::_1)
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
#[doc = "PWM Channel3 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS3_A {
    #[doc = "0: PWM channel3 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel3 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel3 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS3` reader - PWM Channel3 Edge-detect Brake Status"]
pub struct BRKESTS3_R(crate::FieldReader<bool, BRKESTS3_A>);
impl BRKESTS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS3_A {
        match self.bits {
            false => BRKESTS3_A::_0,
            true => BRKESTS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS3_A::_1
    }
}
impl core::ops::Deref for BRKESTS3_R {
    type Target = crate::FieldReader<bool, BRKESTS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS3` writer - PWM Channel3 Edge-detect Brake Status"]
pub struct BRKESTS3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel3 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS3_A::_0)
    }
    #[doc = "When PWM channel3 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel3 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS3_A::_1)
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
#[doc = "PWM Channel4 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS4_A {
    #[doc = "0: PWM channel4 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel4 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel4 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS4_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS4` reader - PWM Channel4 Edge-detect Brake Status"]
pub struct BRKESTS4_R(crate::FieldReader<bool, BRKESTS4_A>);
impl BRKESTS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS4_A {
        match self.bits {
            false => BRKESTS4_A::_0,
            true => BRKESTS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS4_A::_1
    }
}
impl core::ops::Deref for BRKESTS4_R {
    type Target = crate::FieldReader<bool, BRKESTS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS4` writer - PWM Channel4 Edge-detect Brake Status"]
pub struct BRKESTS4_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel4 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS4_A::_0)
    }
    #[doc = "When PWM channel4 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel4 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "PWM Channel5 Edge-detect Brake Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKESTS5_A {
    #[doc = "0: PWM channel5 edge-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel5 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel5 at brake state, writing 1 to clear"]
    _1 = 1,
}
impl From<BRKESTS5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKESTS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKESTS5` reader - PWM Channel5 Edge-detect Brake Status"]
pub struct BRKESTS5_R(crate::FieldReader<bool, BRKESTS5_A>);
impl BRKESTS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKESTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKESTS5_A {
        match self.bits {
            false => BRKESTS5_A::_0,
            true => BRKESTS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKESTS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKESTS5_A::_1
    }
}
impl core::ops::Deref for BRKESTS5_R {
    type Target = crate::FieldReader<bool, BRKESTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKESTS5` writer - PWM Channel5 Edge-detect Brake Status"]
pub struct BRKESTS5_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKESTS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKESTS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM channel5 edge-detect brake state is released"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKESTS5_A::_0)
    }
    #[doc = "When PWM channel5 edge-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel5 at brake state, writing 1 to clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKESTS5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "PWM Channel0 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS0_A {
    #[doc = "0: PWM channel0 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel0 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel0 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS0` reader - PWM Channel0 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS0_R(crate::FieldReader<bool, BRKLSTS0_A>);
impl BRKLSTS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS0_A {
        match self.bits {
            false => BRKLSTS0_A::_0,
            true => BRKLSTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS0_A::_1
    }
}
impl core::ops::Deref for BRKLSTS0_R {
    type Target = crate::FieldReader<bool, BRKLSTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Channel1 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS1_A {
    #[doc = "0: PWM channel1 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel1 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel1 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS1` reader - PWM Channel1 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS1_R(crate::FieldReader<bool, BRKLSTS1_A>);
impl BRKLSTS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS1_A {
        match self.bits {
            false => BRKLSTS1_A::_0,
            true => BRKLSTS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS1_A::_1
    }
}
impl core::ops::Deref for BRKLSTS1_R {
    type Target = crate::FieldReader<bool, BRKLSTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Channel2 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS2_A {
    #[doc = "0: PWM channel2 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel2 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel2 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS2_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS2` reader - PWM Channel2 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS2_R(crate::FieldReader<bool, BRKLSTS2_A>);
impl BRKLSTS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS2_A {
        match self.bits {
            false => BRKLSTS2_A::_0,
            true => BRKLSTS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS2_A::_1
    }
}
impl core::ops::Deref for BRKLSTS2_R {
    type Target = crate::FieldReader<bool, BRKLSTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Channel3 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS3_A {
    #[doc = "0: PWM channel3 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel3 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel3 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS3` reader - PWM Channel3 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS3_R(crate::FieldReader<bool, BRKLSTS3_A>);
impl BRKLSTS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS3_A {
        match self.bits {
            false => BRKLSTS3_A::_0,
            true => BRKLSTS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS3_A::_1
    }
}
impl core::ops::Deref for BRKLSTS3_R {
    type Target = crate::FieldReader<bool, BRKLSTS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Channel4 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS4_A {
    #[doc = "0: PWM channel4 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel4 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel4 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS4_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS4` reader - PWM Channel4 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS4_R(crate::FieldReader<bool, BRKLSTS4_A>);
impl BRKLSTS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS4_A {
        match self.bits {
            false => BRKLSTS4_A::_0,
            true => BRKLSTS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS4_A::_1
    }
}
impl core::ops::Deref for BRKLSTS4_R {
    type Target = crate::FieldReader<bool, BRKLSTS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PWM Channel5 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLSTS5_A {
    #[doc = "0: PWM channel5 level-detect brake state is released"]
    _0 = 0,
    #[doc = "1: When PWM channel5 level-detect brake detects a falling edge of any enabled brake source; this flag will be set to indicate the PWM channel5 at brake state"]
    _1 = 1,
}
impl From<BRKLSTS5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLSTS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLSTS5` reader - PWM Channel5 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
pub struct BRKLSTS5_R(crate::FieldReader<bool, BRKLSTS5_A>);
impl BRKLSTS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLSTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLSTS5_A {
        match self.bits {
            false => BRKLSTS5_A::_0,
            true => BRKLSTS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLSTS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLSTS5_A::_1
    }
}
impl core::ops::Deref for BRKLSTS5_R {
    type Target = crate::FieldReader<bool, BRKLSTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PWM Channel0 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif0(&self) -> BRKEIF0_R {
        BRKEIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Channel1 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif1(&self) -> BRKEIF1_R {
        BRKEIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Channel2 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif2(&self) -> BRKEIF2_R {
        BRKEIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Channel3 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif3(&self) -> BRKEIF3_R {
        BRKEIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Channel4 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif4(&self) -> BRKEIF4_R {
        BRKEIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Channel5 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif5(&self) -> BRKEIF5_R {
        BRKEIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Channel0 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif0(&self) -> BRKLIF0_R {
        BRKLIF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM Channel1 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif1(&self) -> BRKLIF1_R {
        BRKLIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Channel2 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif2(&self) -> BRKLIF2_R {
        BRKLIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM Channel3 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif3(&self) -> BRKLIF3_R {
        BRKLIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM Channel4 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif4(&self) -> BRKLIF4_R {
        BRKLIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM Channel5 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif5(&self) -> BRKLIF5_R {
        BRKLIF5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM Channel0 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests0(&self) -> BRKESTS0_R {
        BRKESTS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWM Channel1 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests1(&self) -> BRKESTS1_R {
        BRKESTS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PWM Channel2 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests2(&self) -> BRKESTS2_R {
        BRKESTS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PWM Channel3 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests3(&self) -> BRKESTS3_R {
        BRKESTS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM Channel4 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests4(&self) -> BRKESTS4_R {
        BRKESTS4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM Channel5 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests5(&self) -> BRKESTS5_R {
        BRKESTS5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PWM Channel0 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts0(&self) -> BRKLSTS0_R {
        BRKLSTS0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PWM Channel1 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts1(&self) -> BRKLSTS1_R {
        BRKLSTS1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PWM Channel2 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts2(&self) -> BRKLSTS2_R {
        BRKLSTS2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PWM Channel3 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts3(&self) -> BRKLSTS3_R {
        BRKLSTS3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PWM Channel4 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts4(&self) -> BRKLSTS4_R {
        BRKLSTS4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PWM Channel5 Level-detect Brake Status (Read Only)\\nNote: This bit is read only and auto cleared by hardware. When enabled brake source return to high level, PWM will release brake state until current PWM period finished. The PWM waveform will start output from next full PWM period."]
    #[inline(always)]
    pub fn brklsts5(&self) -> BRKLSTS5_R {
        BRKLSTS5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Channel0 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif0(&mut self) -> BRKEIF0_W {
        BRKEIF0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Channel1 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif1(&mut self) -> BRKEIF1_W {
        BRKEIF1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Channel2 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif2(&mut self) -> BRKEIF2_W {
        BRKEIF2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Channel3 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif3(&mut self) -> BRKEIF3_W {
        BRKEIF3_W { w: self }
    }
    #[doc = "Bit 4 - PWM Channel4 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif4(&mut self) -> BRKEIF4_W {
        BRKEIF4_W { w: self }
    }
    #[doc = "Bit 5 - PWM Channel5 Edge-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeif5(&mut self) -> BRKEIF5_W {
        BRKEIF5_W { w: self }
    }
    #[doc = "Bit 8 - PWM Channel0 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif0(&mut self) -> BRKLIF0_W {
        BRKLIF0_W { w: self }
    }
    #[doc = "Bit 9 - PWM Channel1 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif1(&mut self) -> BRKLIF1_W {
        BRKLIF1_W { w: self }
    }
    #[doc = "Bit 10 - PWM Channel2 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif2(&mut self) -> BRKLIF2_W {
        BRKLIF2_W { w: self }
    }
    #[doc = "Bit 11 - PWM Channel3 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif3(&mut self) -> BRKLIF3_W {
        BRKLIF3_W { w: self }
    }
    #[doc = "Bit 12 - PWM Channel4 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif4(&mut self) -> BRKLIF4_W {
        BRKLIF4_W { w: self }
    }
    #[doc = "Bit 13 - PWM Channel5 Level-detect Brake Interrupt Flag (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklif5(&mut self) -> BRKLIF5_W {
        BRKLIF5_W { w: self }
    }
    #[doc = "Bit 16 - PWM Channel0 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests0(&mut self) -> BRKESTS0_W {
        BRKESTS0_W { w: self }
    }
    #[doc = "Bit 17 - PWM Channel1 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests1(&mut self) -> BRKESTS1_W {
        BRKESTS1_W { w: self }
    }
    #[doc = "Bit 18 - PWM Channel2 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests2(&mut self) -> BRKESTS2_W {
        BRKESTS2_W { w: self }
    }
    #[doc = "Bit 19 - PWM Channel3 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests3(&mut self) -> BRKESTS3_W {
        BRKESTS3_W { w: self }
    }
    #[doc = "Bit 20 - PWM Channel4 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests4(&mut self) -> BRKESTS4_W {
        BRKESTS4_W { w: self }
    }
    #[doc = "Bit 21 - PWM Channel5 Edge-detect Brake Status"]
    #[inline(always)]
    pub fn brkests5(&mut self) -> BRKESTS5_W {
        BRKESTS5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Flag Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_intsts1](index.html) module"]
pub struct PWM_INTSTS1_SPEC;
impl crate::RegisterSpec for PWM_INTSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_intsts1::R](R) reader structure"]
impl crate::Readable for PWM_INTSTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_intsts1::W](W) writer structure"]
impl crate::Writable for PWM_INTSTS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_INTSTS1 to value 0"]
impl crate::Resettable for PWM_INTSTS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
