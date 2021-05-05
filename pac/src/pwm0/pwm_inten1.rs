#[doc = "Register `PWM_INTEN1` reader"]
pub struct R(crate::R<PWM_INTEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INTEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_INTEN1_SPEC>> for R {
    fn from(reader: crate::R<PWM_INTEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_INTEN1` writer"]
pub struct W(crate::W<PWM_INTEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INTEN1_SPEC>;
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
impl core::convert::From<crate::W<PWM_INTEN1_SPEC>> for W {
    fn from(writer: crate::W<PWM_INTEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Edge-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIEN0_1_A {
    #[doc = "0: Edge-detect Brake interrupt for channel0/1 Disabled"]
    _0 = 0,
    #[doc = "1: Edge-detect Brake interrupt for channel0/1 Enabled"]
    _1 = 1,
}
impl From<BRKEIEN0_1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIEN0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIEN0_1` reader - PWM Edge-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN0_1_R(crate::FieldReader<bool, BRKEIEN0_1_A>);
impl BRKEIEN0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIEN0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIEN0_1_A {
        match self.bits {
            false => BRKEIEN0_1_A::_0,
            true => BRKEIEN0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIEN0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIEN0_1_A::_1
    }
}
impl core::ops::Deref for BRKEIEN0_1_R {
    type Target = crate::FieldReader<bool, BRKEIEN0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIEN0_1` writer - PWM Edge-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIEN0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIEN0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge-detect Brake interrupt for channel0/1 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIEN0_1_A::_0)
    }
    #[doc = "Edge-detect Brake interrupt for channel0/1 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIEN0_1_A::_1)
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
#[doc = "PWM Edge-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIEN2_3_A {
    #[doc = "0: Edge-detect Brake interrupt for channel2/3 Disabled"]
    _0 = 0,
    #[doc = "1: Edge-detect Brake interrupt for channel2/3 Enabled"]
    _1 = 1,
}
impl From<BRKEIEN2_3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIEN2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIEN2_3` reader - PWM Edge-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN2_3_R(crate::FieldReader<bool, BRKEIEN2_3_A>);
impl BRKEIEN2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIEN2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIEN2_3_A {
        match self.bits {
            false => BRKEIEN2_3_A::_0,
            true => BRKEIEN2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIEN2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIEN2_3_A::_1
    }
}
impl core::ops::Deref for BRKEIEN2_3_R {
    type Target = crate::FieldReader<bool, BRKEIEN2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIEN2_3` writer - PWM Edge-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIEN2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIEN2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge-detect Brake interrupt for channel2/3 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIEN2_3_A::_0)
    }
    #[doc = "Edge-detect Brake interrupt for channel2/3 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIEN2_3_A::_1)
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
#[doc = "PWM Edge-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKEIEN4_5_A {
    #[doc = "0: Edge-detect Brake interrupt for channel4/5 Disabled"]
    _0 = 0,
    #[doc = "1: Edge-detect Brake interrupt for channel4/5 Enabled"]
    _1 = 1,
}
impl From<BRKEIEN4_5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKEIEN4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEIEN4_5` reader - PWM Edge-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN4_5_R(crate::FieldReader<bool, BRKEIEN4_5_A>);
impl BRKEIEN4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEIEN4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKEIEN4_5_A {
        match self.bits {
            false => BRKEIEN4_5_A::_0,
            true => BRKEIEN4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKEIEN4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKEIEN4_5_A::_1
    }
}
impl core::ops::Deref for BRKEIEN4_5_R {
    type Target = crate::FieldReader<bool, BRKEIEN4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEIEN4_5` writer - PWM Edge-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKEIEN4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEIEN4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKEIEN4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge-detect Brake interrupt for channel4/5 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKEIEN4_5_A::_0)
    }
    #[doc = "Edge-detect Brake interrupt for channel4/5 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKEIEN4_5_A::_1)
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
#[doc = "PWM Level-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIEN0_1_A {
    #[doc = "0: Level-detect Brake interrupt for channel0/1 Disabled"]
    _0 = 0,
    #[doc = "1: Level-detect Brake interrupt for channel0/1 Enabled"]
    _1 = 1,
}
impl From<BRKLIEN0_1_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIEN0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIEN0_1` reader - PWM Level-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN0_1_R(crate::FieldReader<bool, BRKLIEN0_1_A>);
impl BRKLIEN0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIEN0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIEN0_1_A {
        match self.bits {
            false => BRKLIEN0_1_A::_0,
            true => BRKLIEN0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIEN0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIEN0_1_A::_1
    }
}
impl core::ops::Deref for BRKLIEN0_1_R {
    type Target = crate::FieldReader<bool, BRKLIEN0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIEN0_1` writer - PWM Level-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIEN0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIEN0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level-detect Brake interrupt for channel0/1 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIEN0_1_A::_0)
    }
    #[doc = "Level-detect Brake interrupt for channel0/1 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIEN0_1_A::_1)
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
#[doc = "PWM Level-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIEN2_3_A {
    #[doc = "0: Level-detect Brake interrupt for channel2/3 Disabled"]
    _0 = 0,
    #[doc = "1: Level-detect Brake interrupt for channel2/3 Enabled"]
    _1 = 1,
}
impl From<BRKLIEN2_3_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIEN2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIEN2_3` reader - PWM Level-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN2_3_R(crate::FieldReader<bool, BRKLIEN2_3_A>);
impl BRKLIEN2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIEN2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIEN2_3_A {
        match self.bits {
            false => BRKLIEN2_3_A::_0,
            true => BRKLIEN2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIEN2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIEN2_3_A::_1
    }
}
impl core::ops::Deref for BRKLIEN2_3_R {
    type Target = crate::FieldReader<bool, BRKLIEN2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIEN2_3` writer - PWM Level-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIEN2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIEN2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level-detect Brake interrupt for channel2/3 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIEN2_3_A::_0)
    }
    #[doc = "Level-detect Brake interrupt for channel2/3 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIEN2_3_A::_1)
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
#[doc = "PWM Level-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKLIEN4_5_A {
    #[doc = "0: Level-detect Brake interrupt for channel4/5 Disabled"]
    _0 = 0,
    #[doc = "1: Level-detect Brake interrupt for channel4/5 Enabled"]
    _1 = 1,
}
impl From<BRKLIEN4_5_A> for bool {
    #[inline(always)]
    fn from(variant: BRKLIEN4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKLIEN4_5` reader - PWM Level-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN4_5_R(crate::FieldReader<bool, BRKLIEN4_5_A>);
impl BRKLIEN4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKLIEN4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKLIEN4_5_A {
        match self.bits {
            false => BRKLIEN4_5_A::_0,
            true => BRKLIEN4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKLIEN4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKLIEN4_5_A::_1
    }
}
impl core::ops::Deref for BRKLIEN4_5_R {
    type Target = crate::FieldReader<bool, BRKLIEN4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKLIEN4_5` writer - PWM Level-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLIEN4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLIEN4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKLIEN4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level-detect Brake interrupt for channel4/5 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKLIEN4_5_A::_0)
    }
    #[doc = "Level-detect Brake interrupt for channel4/5 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKLIEN4_5_A::_1)
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
impl R {
    #[doc = "Bit 0 - PWM Edge-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien0_1(&self) -> BRKEIEN0_1_R {
        BRKEIEN0_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Edge-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien2_3(&self) -> BRKEIEN2_3_R {
        BRKEIEN2_3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Edge-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien4_5(&self) -> BRKEIEN4_5_R {
        BRKEIEN4_5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM Level-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien0_1(&self) -> BRKLIEN0_1_R {
        BRKLIEN0_1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM Level-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien2_3(&self) -> BRKLIEN2_3_R {
        BRKLIEN2_3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Level-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien4_5(&self) -> BRKLIEN4_5_R {
        BRKLIEN4_5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Edge-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien0_1(&mut self) -> BRKEIEN0_1_W {
        BRKEIEN0_1_W { w: self }
    }
    #[doc = "Bit 1 - PWM Edge-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien2_3(&mut self) -> BRKEIEN2_3_W {
        BRKEIEN2_3_W { w: self }
    }
    #[doc = "Bit 2 - PWM Edge-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkeien4_5(&mut self) -> BRKEIEN4_5_W {
        BRKEIEN4_5_W { w: self }
    }
    #[doc = "Bit 8 - PWM Level-detect Brake Interrupt Enable Bit for Channel0/1 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien0_1(&mut self) -> BRKLIEN0_1_W {
        BRKLIEN0_1_W { w: self }
    }
    #[doc = "Bit 9 - PWM Level-detect Brake Interrupt Enable Bit for Channel2/3 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien2_3(&mut self) -> BRKLIEN2_3_W {
        BRKLIEN2_3_W { w: self }
    }
    #[doc = "Bit 10 - PWM Level-detect Brake Interrupt Enable Bit for Channel4/5 (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brklien4_5(&mut self) -> BRKLIEN4_5_W {
        BRKLIEN4_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_inten1](index.html) module"]
pub struct PWM_INTEN1_SPEC;
impl crate::RegisterSpec for PWM_INTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_inten1::R](R) reader structure"]
impl crate::Readable for PWM_INTEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_inten1::W](W) writer structure"]
impl crate::Writable for PWM_INTEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_INTEN1 to value 0"]
impl crate::Resettable for PWM_INTEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
