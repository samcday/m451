#[doc = "Register `PF_SMTEN` reader"]
pub struct R(crate::R<PF_SMTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_SMTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PF_SMTEN_SPEC>> for R {
    fn from(reader: crate::R<PF_SMTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_SMTEN` writer"]
pub struct W(crate::W<PF_SMTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_SMTEN_SPEC>;
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
impl core::convert::From<crate::W<PF_SMTEN_SPEC>> for W {
    fn from(writer: crate::W<PF_SMTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN0_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN0` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN0_R(crate::FieldReader<bool, SMTEN0_A>);
impl SMTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN0_A {
        match self.bits {
            false => SMTEN0_A::_0,
            true => SMTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN0_A::_1
    }
}
impl core::ops::Deref for SMTEN0_R {
    type Target = crate::FieldReader<bool, SMTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN0` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN0_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN0_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN1_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN1` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN1_R(crate::FieldReader<bool, SMTEN1_A>);
impl SMTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN1_A {
        match self.bits {
            false => SMTEN1_A::_0,
            true => SMTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN1_A::_1
    }
}
impl core::ops::Deref for SMTEN1_R {
    type Target = crate::FieldReader<bool, SMTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN1` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN1_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN1_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN2_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN2` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN2_R(crate::FieldReader<bool, SMTEN2_A>);
impl SMTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN2_A {
        match self.bits {
            false => SMTEN2_A::_0,
            true => SMTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN2_A::_1
    }
}
impl core::ops::Deref for SMTEN2_R {
    type Target = crate::FieldReader<bool, SMTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN2` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN2_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN2_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN3_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN3` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN3_R(crate::FieldReader<bool, SMTEN3_A>);
impl SMTEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN3_A {
        match self.bits {
            false => SMTEN3_A::_0,
            true => SMTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN3_A::_1
    }
}
impl core::ops::Deref for SMTEN3_R {
    type Target = crate::FieldReader<bool, SMTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN3` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN3_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN3_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN4_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN4` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN4_R(crate::FieldReader<bool, SMTEN4_A>);
impl SMTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN4_A {
        match self.bits {
            false => SMTEN4_A::_0,
            true => SMTEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN4_A::_1
    }
}
impl core::ops::Deref for SMTEN4_R {
    type Target = crate::FieldReader<bool, SMTEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN4` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN4_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN4_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN5_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN5` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN5_R(crate::FieldReader<bool, SMTEN5_A>);
impl SMTEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN5_A {
        match self.bits {
            false => SMTEN5_A::_0,
            true => SMTEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN5_A::_1
    }
}
impl core::ops::Deref for SMTEN5_R {
    type Target = crate::FieldReader<bool, SMTEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN5` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN5_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN5_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN6_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN6` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN6_R(crate::FieldReader<bool, SMTEN6_A>);
impl SMTEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN6_A {
        match self.bits {
            false => SMTEN6_A::_0,
            true => SMTEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN6_A::_1
    }
}
impl core::ops::Deref for SMTEN6_R {
    type Target = crate::FieldReader<bool, SMTEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN6` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN6_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN6_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN7_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN7` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN7_R(crate::FieldReader<bool, SMTEN7_A>);
impl SMTEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN7_A {
        match self.bits {
            false => SMTEN7_A::_0,
            true => SMTEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN7_A::_1
    }
}
impl core::ops::Deref for SMTEN7_R {
    type Target = crate::FieldReader<bool, SMTEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN7` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN7_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN7_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN8_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN8_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN8` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN8_R(crate::FieldReader<bool, SMTEN8_A>);
impl SMTEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN8_A {
        match self.bits {
            false => SMTEN8_A::_0,
            true => SMTEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN8_A::_1
    }
}
impl core::ops::Deref for SMTEN8_R {
    type Target = crate::FieldReader<bool, SMTEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN8` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN8_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN8_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN9_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN9_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN9` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN9_R(crate::FieldReader<bool, SMTEN9_A>);
impl SMTEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN9_A {
        match self.bits {
            false => SMTEN9_A::_0,
            true => SMTEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN9_A::_1
    }
}
impl core::ops::Deref for SMTEN9_R {
    type Target = crate::FieldReader<bool, SMTEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN9` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN9_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN9_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN10_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN10_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN10` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN10_R(crate::FieldReader<bool, SMTEN10_A>);
impl SMTEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN10_A {
        match self.bits {
            false => SMTEN10_A::_0,
            true => SMTEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN10_A::_1
    }
}
impl core::ops::Deref for SMTEN10_R {
    type Target = crate::FieldReader<bool, SMTEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN10` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN10_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN10_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN11_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN11_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN11` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN11_R(crate::FieldReader<bool, SMTEN11_A>);
impl SMTEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN11_A {
        match self.bits {
            false => SMTEN11_A::_0,
            true => SMTEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN11_A::_1
    }
}
impl core::ops::Deref for SMTEN11_R {
    type Target = crate::FieldReader<bool, SMTEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN11` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN11_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN11_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN12_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN12_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN12` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN12_R(crate::FieldReader<bool, SMTEN12_A>);
impl SMTEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN12_A {
        match self.bits {
            false => SMTEN12_A::_0,
            true => SMTEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN12_A::_1
    }
}
impl core::ops::Deref for SMTEN12_R {
    type Target = crate::FieldReader<bool, SMTEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN12` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN12_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN12_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN13_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN13_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN13` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN13_R(crate::FieldReader<bool, SMTEN13_A>);
impl SMTEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN13_A {
        match self.bits {
            false => SMTEN13_A::_0,
            true => SMTEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN13_A::_1
    }
}
impl core::ops::Deref for SMTEN13_R {
    type Target = crate::FieldReader<bool, SMTEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN13` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN13_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN13_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN14_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN14_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN14` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN14_R(crate::FieldReader<bool, SMTEN14_A>);
impl SMTEN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN14_A {
        match self.bits {
            false => SMTEN14_A::_0,
            true => SMTEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN14_A::_1
    }
}
impl core::ops::Deref for SMTEN14_R {
    type Target = crate::FieldReader<bool, SMTEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN14` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN14_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN14_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTEN15_A {
    #[doc = "0: Px.n input schmitt trigger function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n input schmitt trigger function Enabled"]
    _1 = 1,
}
impl From<SMTEN15_A> for bool {
    #[inline(always)]
    fn from(variant: SMTEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMTEN15` reader - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN15_R(crate::FieldReader<bool, SMTEN15_A>);
impl SMTEN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMTEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTEN15_A {
        match self.bits {
            false => SMTEN15_A::_0,
            true => SMTEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMTEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMTEN15_A::_1
    }
}
impl core::ops::Deref for SMTEN15_R {
    type Target = crate::FieldReader<bool, SMTEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTEN15` writer - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct SMTEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMTEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n input schmitt trigger function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMTEN15_A::_0)
    }
    #[doc = "Px.n input schmitt trigger function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMTEN15_A::_1)
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
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten0(&self) -> SMTEN0_R {
        SMTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten1(&self) -> SMTEN1_R {
        SMTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten2(&self) -> SMTEN2_R {
        SMTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten3(&self) -> SMTEN3_R {
        SMTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten4(&self) -> SMTEN4_R {
        SMTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten5(&self) -> SMTEN5_R {
        SMTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten6(&self) -> SMTEN6_R {
        SMTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten7(&self) -> SMTEN7_R {
        SMTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten8(&self) -> SMTEN8_R {
        SMTEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten9(&self) -> SMTEN9_R {
        SMTEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten10(&self) -> SMTEN10_R {
        SMTEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten11(&self) -> SMTEN11_R {
        SMTEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten12(&self) -> SMTEN12_R {
        SMTEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten13(&self) -> SMTEN13_R {
        SMTEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten14(&self) -> SMTEN14_R {
        SMTEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten15(&self) -> SMTEN15_R {
        SMTEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten0(&mut self) -> SMTEN0_W {
        SMTEN0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten1(&mut self) -> SMTEN1_W {
        SMTEN1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten2(&mut self) -> SMTEN2_W {
        SMTEN2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten3(&mut self) -> SMTEN3_W {
        SMTEN3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten4(&mut self) -> SMTEN4_W {
        SMTEN4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten5(&mut self) -> SMTEN5_W {
        SMTEN5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten6(&mut self) -> SMTEN6_W {
        SMTEN6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten7(&mut self) -> SMTEN7_W {
        SMTEN7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten8(&mut self) -> SMTEN8_W {
        SMTEN8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten9(&mut self) -> SMTEN9_W {
        SMTEN9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten10(&mut self) -> SMTEN10_W {
        SMTEN10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten11(&mut self) -> SMTEN11_W {
        SMTEN11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten12(&mut self) -> SMTEN12_W {
        SMTEN12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten13(&mut self) -> SMTEN13_W {
        SMTEN13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten14(&mut self) -> SMTEN14_W {
        SMTEN14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Input Schmitt Trigger Enable Bit\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn smten15(&mut self) -> SMTEN15_W {
        SMTEN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Input Schmitt Trigger Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_smten](index.html) module"]
pub struct PF_SMTEN_SPEC;
impl crate::RegisterSpec for PF_SMTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_smten::R](R) reader structure"]
impl crate::Readable for PF_SMTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_smten::W](W) writer structure"]
impl crate::Writable for PF_SMTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_SMTEN to value 0"]
impl crate::Resettable for PF_SMTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
