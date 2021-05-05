#[doc = "Register `PWM_BRKCTL0_1` reader"]
pub struct R(crate::R<PWM_BRKCTL0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_BRKCTL0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_BRKCTL0_1_SPEC>> for R {
    fn from(reader: crate::R<PWM_BRKCTL0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_BRKCTL0_1` writer"]
pub struct W(crate::W<PWM_BRKCTL0_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_BRKCTL0_1_SPEC>;
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
impl core::convert::From<crate::W<PWM_BRKCTL0_1_SPEC>> for W {
    fn from(writer: crate::W<PWM_BRKCTL0_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACMP0_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPO0EBEN_A {
    #[doc = "0: ACMP0_O as edge-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: ACMP0_O as edge-detect brake source Enabled"]
    _1 = 1,
}
impl From<CPO0EBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPO0EBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPO0EBEN` reader - ACMP0_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO0EBEN_R(crate::FieldReader<bool, CPO0EBEN_A>);
impl CPO0EBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPO0EBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPO0EBEN_A {
        match self.bits {
            false => CPO0EBEN_A::_0,
            true => CPO0EBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPO0EBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPO0EBEN_A::_1
    }
}
impl core::ops::Deref for CPO0EBEN_R {
    type Target = crate::FieldReader<bool, CPO0EBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPO0EBEN` writer - ACMP0_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO0EBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPO0EBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPO0EBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP0_O as edge-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPO0EBEN_A::_0)
    }
    #[doc = "ACMP0_O as edge-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPO0EBEN_A::_1)
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
#[doc = "ACMP1_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPO1EBEN_A {
    #[doc = "0: ACMP1_O as edge-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: ACMP1_O as edge-detect brake source Enabled"]
    _1 = 1,
}
impl From<CPO1EBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPO1EBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPO1EBEN` reader - ACMP1_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO1EBEN_R(crate::FieldReader<bool, CPO1EBEN_A>);
impl CPO1EBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPO1EBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPO1EBEN_A {
        match self.bits {
            false => CPO1EBEN_A::_0,
            true => CPO1EBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPO1EBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPO1EBEN_A::_1
    }
}
impl core::ops::Deref for CPO1EBEN_R {
    type Target = crate::FieldReader<bool, CPO1EBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPO1EBEN` writer - ACMP1_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO1EBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPO1EBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPO1EBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP1_O as edge-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPO1EBEN_A::_0)
    }
    #[doc = "ACMP1_O as edge-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPO1EBEN_A::_1)
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
#[doc = "PWMx_BRAKE0 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKP0EEN_A {
    #[doc = "0: BKP0 pin as edge-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: BKP0 pin as edge-detect brake source Enabled"]
    _1 = 1,
}
impl From<BRKP0EEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKP0EEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKP0EEN` reader - PWMx_BRAKE0 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP0EEN_R(crate::FieldReader<bool, BRKP0EEN_A>);
impl BRKP0EEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKP0EEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKP0EEN_A {
        match self.bits {
            false => BRKP0EEN_A::_0,
            true => BRKP0EEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKP0EEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKP0EEN_A::_1
    }
}
impl core::ops::Deref for BRKP0EEN_R {
    type Target = crate::FieldReader<bool, BRKP0EEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKP0EEN` writer - PWMx_BRAKE0 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP0EEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKP0EEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKP0EEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BKP0 pin as edge-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKP0EEN_A::_0)
    }
    #[doc = "BKP0 pin as edge-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKP0EEN_A::_1)
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
#[doc = "PWMx_BRAKE1 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKP1EEN_A {
    #[doc = "0: BKP1 pin as edge-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: BKP1 pin as edge-detect brake source Enabled"]
    _1 = 1,
}
impl From<BRKP1EEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKP1EEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKP1EEN` reader - PWMx_BRAKE1 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP1EEN_R(crate::FieldReader<bool, BRKP1EEN_A>);
impl BRKP1EEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKP1EEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKP1EEN_A {
        match self.bits {
            false => BRKP1EEN_A::_0,
            true => BRKP1EEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKP1EEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKP1EEN_A::_1
    }
}
impl core::ops::Deref for BRKP1EEN_R {
    type Target = crate::FieldReader<bool, BRKP1EEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKP1EEN` writer - PWMx_BRAKE1 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP1EEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKP1EEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKP1EEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BKP1 pin as edge-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKP1EEN_A::_0)
    }
    #[doc = "BKP1 pin as edge-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKP1EEN_A::_1)
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
#[doc = "System Fail As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSEBEN_A {
    #[doc = "0: System Fail condition as edge-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: System Fail condition as edge-detect brake source Enabled"]
    _1 = 1,
}
impl From<SYSEBEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSEBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSEBEN` reader - System Fail As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct SYSEBEN_R(crate::FieldReader<bool, SYSEBEN_A>);
impl SYSEBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSEBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSEBEN_A {
        match self.bits {
            false => SYSEBEN_A::_0,
            true => SYSEBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYSEBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYSEBEN_A::_1
    }
}
impl core::ops::Deref for SYSEBEN_R {
    type Target = crate::FieldReader<bool, SYSEBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSEBEN` writer - System Fail As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct SYSEBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSEBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSEBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System Fail condition as edge-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSEBEN_A::_0)
    }
    #[doc = "System Fail condition as edge-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSEBEN_A::_1)
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
#[doc = "ACMP0_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPO0LBEN_A {
    #[doc = "0: ACMP0_O as level-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: ACMP0_O as level-detect brake source Enabled"]
    _1 = 1,
}
impl From<CPO0LBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPO0LBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPO0LBEN` reader - ACMP0_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO0LBEN_R(crate::FieldReader<bool, CPO0LBEN_A>);
impl CPO0LBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPO0LBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPO0LBEN_A {
        match self.bits {
            false => CPO0LBEN_A::_0,
            true => CPO0LBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPO0LBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPO0LBEN_A::_1
    }
}
impl core::ops::Deref for CPO0LBEN_R {
    type Target = crate::FieldReader<bool, CPO0LBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPO0LBEN` writer - ACMP0_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO0LBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPO0LBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPO0LBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP0_O as level-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPO0LBEN_A::_0)
    }
    #[doc = "ACMP0_O as level-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPO0LBEN_A::_1)
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
#[doc = "ACMP1_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPO1LBEN_A {
    #[doc = "0: ACMP1_O as level-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: ACMP1_O as level-detect brake source Enabled"]
    _1 = 1,
}
impl From<CPO1LBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPO1LBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPO1LBEN` reader - ACMP1_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO1LBEN_R(crate::FieldReader<bool, CPO1LBEN_A>);
impl CPO1LBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPO1LBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPO1LBEN_A {
        match self.bits {
            false => CPO1LBEN_A::_0,
            true => CPO1LBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPO1LBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPO1LBEN_A::_1
    }
}
impl core::ops::Deref for CPO1LBEN_R {
    type Target = crate::FieldReader<bool, CPO1LBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPO1LBEN` writer - ACMP1_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct CPO1LBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPO1LBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPO1LBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP1_O as level-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPO1LBEN_A::_0)
    }
    #[doc = "ACMP1_O as level-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPO1LBEN_A::_1)
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
#[doc = "BKP0 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKP0LEN_A {
    #[doc = "0: PWMx_BRAKE0 pin as level-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: PWMx_BRAKE0 pin as level-detect brake source Enabled"]
    _1 = 1,
}
impl From<BRKP0LEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKP0LEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKP0LEN` reader - BKP0 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP0LEN_R(crate::FieldReader<bool, BRKP0LEN_A>);
impl BRKP0LEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKP0LEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKP0LEN_A {
        match self.bits {
            false => BRKP0LEN_A::_0,
            true => BRKP0LEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKP0LEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKP0LEN_A::_1
    }
}
impl core::ops::Deref for BRKP0LEN_R {
    type Target = crate::FieldReader<bool, BRKP0LEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKP0LEN` writer - BKP0 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP0LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKP0LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKP0LEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWMx_BRAKE0 pin as level-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKP0LEN_A::_0)
    }
    #[doc = "PWMx_BRAKE0 pin as level-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKP0LEN_A::_1)
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
#[doc = "BKP1 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKP1LEN_A {
    #[doc = "0: PWMx_BRAKE1 pin as level-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: PWMx_BRAKE1 pin as level-detect brake source Enabled"]
    _1 = 1,
}
impl From<BRKP1LEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKP1LEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKP1LEN` reader - BKP1 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP1LEN_R(crate::FieldReader<bool, BRKP1LEN_A>);
impl BRKP1LEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKP1LEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKP1LEN_A {
        match self.bits {
            false => BRKP1LEN_A::_0,
            true => BRKP1LEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKP1LEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKP1LEN_A::_1
    }
}
impl core::ops::Deref for BRKP1LEN_R {
    type Target = crate::FieldReader<bool, BRKP1LEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKP1LEN` writer - BKP1 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKP1LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKP1LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKP1LEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWMx_BRAKE1 pin as level-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKP1LEN_A::_0)
    }
    #[doc = "PWMx_BRAKE1 pin as level-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKP1LEN_A::_1)
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
#[doc = "System Fail As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSLBEN_A {
    #[doc = "0: System Fail condition as level-detect brake source Disabled"]
    _0 = 0,
    #[doc = "1: System Fail condition as level-detect brake source Enabled"]
    _1 = 1,
}
impl From<SYSLBEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSLBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSLBEN` reader - System Fail As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct SYSLBEN_R(crate::FieldReader<bool, SYSLBEN_A>);
impl SYSLBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSLBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSLBEN_A {
        match self.bits {
            false => SYSLBEN_A::_0,
            true => SYSLBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYSLBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYSLBEN_A::_1
    }
}
impl core::ops::Deref for SYSLBEN_R {
    type Target = crate::FieldReader<bool, SYSLBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSLBEN` writer - System Fail As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct SYSLBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSLBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSLBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System Fail condition as level-detect brake source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSLBEN_A::_0)
    }
    #[doc = "System Fail condition as level-detect brake source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSLBEN_A::_1)
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
#[doc = "PWM Brake Action Select for Even Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRKAEVEN_A {
    #[doc = "0: PWM even channel level-detect brake function not affect channel output"]
    _0 = 0,
    #[doc = "1: PWM even channel output tri-state when level-detect brake happened"]
    _1 = 1,
    #[doc = "2: PWM even channel output low level when level-detect brake happened"]
    _2 = 2,
    #[doc = "3: PWM even channel output high level when level-detect brake happened"]
    _3 = 3,
}
impl From<BRKAEVEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BRKAEVEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRKAEVEN` reader - PWM Brake Action Select for Even Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKAEVEN_R(crate::FieldReader<u8, BRKAEVEN_A>);
impl BRKAEVEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRKAEVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKAEVEN_A {
        match self.bits {
            0 => BRKAEVEN_A::_0,
            1 => BRKAEVEN_A::_1,
            2 => BRKAEVEN_A::_2,
            3 => BRKAEVEN_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKAEVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKAEVEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BRKAEVEN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BRKAEVEN_A::_3
    }
}
impl core::ops::Deref for BRKAEVEN_R {
    type Target = crate::FieldReader<u8, BRKAEVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKAEVEN` writer - PWM Brake Action Select for Even Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKAEVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKAEVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKAEVEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM even channel level-detect brake function not affect channel output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKAEVEN_A::_0)
    }
    #[doc = "PWM even channel output tri-state when level-detect brake happened"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKAEVEN_A::_1)
    }
    #[doc = "PWM even channel output low level when level-detect brake happened"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BRKAEVEN_A::_2)
    }
    #[doc = "PWM even channel output high level when level-detect brake happened"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BRKAEVEN_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "PWM Brake Action Select for Odd Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRKAODD_A {
    #[doc = "0: PWM odd channel level-detect brake function not affect channel output"]
    _0 = 0,
    #[doc = "1: PWM odd channel output tri-state when level-detect brake happened"]
    _1 = 1,
    #[doc = "2: PWM odd channel output low level when level-detect brake happened"]
    _2 = 2,
    #[doc = "3: PWM odd channel output high level when level-detect brake happened"]
    _3 = 3,
}
impl From<BRKAODD_A> for u8 {
    #[inline(always)]
    fn from(variant: BRKAODD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRKAODD` reader - PWM Brake Action Select for Odd Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKAODD_R(crate::FieldReader<u8, BRKAODD_A>);
impl BRKAODD_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRKAODD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKAODD_A {
        match self.bits {
            0 => BRKAODD_A::_0,
            1 => BRKAODD_A::_1,
            2 => BRKAODD_A::_2,
            3 => BRKAODD_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKAODD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKAODD_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BRKAODD_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BRKAODD_A::_3
    }
}
impl core::ops::Deref for BRKAODD_R {
    type Target = crate::FieldReader<u8, BRKAODD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKAODD` writer - PWM Brake Action Select for Odd Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKAODD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKAODD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKAODD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PWM odd channel level-detect brake function not affect channel output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKAODD_A::_0)
    }
    #[doc = "PWM odd channel output tri-state when level-detect brake happened"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKAODD_A::_1)
    }
    #[doc = "PWM odd channel output low level when level-detect brake happened"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BRKAODD_A::_2)
    }
    #[doc = "PWM odd channel output high level when level-detect brake happened"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BRKAODD_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ACMP0_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo0eben(&self) -> CPO0EBEN_R {
        CPO0EBEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACMP1_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo1eben(&self) -> CPO1EBEN_R {
        CPO1EBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWMx_BRAKE0 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp0een(&self) -> BRKP0EEN_R {
        BRKP0EEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWMx_BRAKE1 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp1een(&self) -> BRKP1EEN_R {
        BRKP1EEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System Fail As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn syseben(&self) -> SYSEBEN_R {
        SYSEBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ACMP0_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo0lben(&self) -> CPO0LBEN_R {
        CPO0LBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ACMP1_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo1lben(&self) -> CPO1LBEN_R {
        CPO1LBEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BKP0 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp0len(&self) -> BRKP0LEN_R {
        BRKP0LEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BKP1 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp1len(&self) -> BRKP1LEN_R {
        BRKP1LEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - System Fail As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn syslben(&self) -> SYSLBEN_R {
        SYSLBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - PWM Brake Action Select for Even Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkaeven(&self) -> BRKAEVEN_R {
        BRKAEVEN_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PWM Brake Action Select for Odd Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkaodd(&self) -> BRKAODD_R {
        BRKAODD_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ACMP0_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo0eben(&mut self) -> CPO0EBEN_W {
        CPO0EBEN_W { w: self }
    }
    #[doc = "Bit 1 - ACMP1_O Digital Output As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo1eben(&mut self) -> CPO1EBEN_W {
        CPO1EBEN_W { w: self }
    }
    #[doc = "Bit 4 - PWMx_BRAKE0 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp0een(&mut self) -> BRKP0EEN_W {
        BRKP0EEN_W { w: self }
    }
    #[doc = "Bit 5 - PWMx_BRAKE1 Pin As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp1een(&mut self) -> BRKP1EEN_W {
        BRKP1EEN_W { w: self }
    }
    #[doc = "Bit 7 - System Fail As Edge-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn syseben(&mut self) -> SYSEBEN_W {
        SYSEBEN_W { w: self }
    }
    #[doc = "Bit 8 - ACMP0_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo0lben(&mut self) -> CPO0LBEN_W {
        CPO0LBEN_W { w: self }
    }
    #[doc = "Bit 9 - ACMP1_O Digital Output As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn cpo1lben(&mut self) -> CPO1LBEN_W {
        CPO1LBEN_W { w: self }
    }
    #[doc = "Bit 12 - BKP0 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp0len(&mut self) -> BRKP0LEN_W {
        BRKP0LEN_W { w: self }
    }
    #[doc = "Bit 13 - BKP1 Pin As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkp1len(&mut self) -> BRKP1LEN_W {
        BRKP1LEN_W { w: self }
    }
    #[doc = "Bit 15 - System Fail As Level-detect Brake Source Enable Bit (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn syslben(&mut self) -> SYSLBEN_W {
        SYSLBEN_W { w: self }
    }
    #[doc = "Bits 16:17 - PWM Brake Action Select for Even Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkaeven(&mut self) -> BRKAEVEN_W {
        BRKAEVEN_W { w: self }
    }
    #[doc = "Bits 18:19 - PWM Brake Action Select for Odd Channel (Write Protect)\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkaodd(&mut self) -> BRKAODD_W {
        BRKAODD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Brake Edge Detect Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_brkctl0_1](index.html) module"]
pub struct PWM_BRKCTL0_1_SPEC;
impl crate::RegisterSpec for PWM_BRKCTL0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_brkctl0_1::R](R) reader structure"]
impl crate::Readable for PWM_BRKCTL0_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_brkctl0_1::W](W) writer structure"]
impl crate::Writable for PWM_BRKCTL0_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_BRKCTL0_1 to value 0"]
impl crate::Resettable for PWM_BRKCTL0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
