#[doc = "Register `PB_DOUT` reader"]
pub struct R(crate::R<PB_DOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_DOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PB_DOUT_SPEC>> for R {
    fn from(reader: crate::R<PB_DOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB_DOUT` writer"]
pub struct W(crate::W<PB_DOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_DOUT_SPEC>;
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
impl core::convert::From<crate::W<PB_DOUT_SPEC>> for W {
    fn from(writer: crate::W<PB_DOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT0_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT0` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT0_R(crate::FieldReader<bool, DOUT0_A>);
impl DOUT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT0_A {
        match self.bits {
            false => DOUT0_A::_0,
            true => DOUT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT0_A::_1
    }
}
impl core::ops::Deref for DOUT0_R {
    type Target = crate::FieldReader<bool, DOUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT0` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT0_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT0_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT1_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT1` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT1_R(crate::FieldReader<bool, DOUT1_A>);
impl DOUT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT1_A {
        match self.bits {
            false => DOUT1_A::_0,
            true => DOUT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT1_A::_1
    }
}
impl core::ops::Deref for DOUT1_R {
    type Target = crate::FieldReader<bool, DOUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT1` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT1_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT1_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT2_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT2` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT2_R(crate::FieldReader<bool, DOUT2_A>);
impl DOUT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT2_A {
        match self.bits {
            false => DOUT2_A::_0,
            true => DOUT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT2_A::_1
    }
}
impl core::ops::Deref for DOUT2_R {
    type Target = crate::FieldReader<bool, DOUT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT2` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT2_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT2_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT3_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT3_R(crate::FieldReader<bool, DOUT3_A>);
impl DOUT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT3_A {
        match self.bits {
            false => DOUT3_A::_0,
            true => DOUT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT3_A::_1
    }
}
impl core::ops::Deref for DOUT3_R {
    type Target = crate::FieldReader<bool, DOUT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT3` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT3_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT3_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT4_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT4` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT4_R(crate::FieldReader<bool, DOUT4_A>);
impl DOUT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT4_A {
        match self.bits {
            false => DOUT4_A::_0,
            true => DOUT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT4_A::_1
    }
}
impl core::ops::Deref for DOUT4_R {
    type Target = crate::FieldReader<bool, DOUT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT4` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT4_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT4_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT5_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT5` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT5_R(crate::FieldReader<bool, DOUT5_A>);
impl DOUT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT5_A {
        match self.bits {
            false => DOUT5_A::_0,
            true => DOUT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT5_A::_1
    }
}
impl core::ops::Deref for DOUT5_R {
    type Target = crate::FieldReader<bool, DOUT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT5` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT5_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT5_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT6_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT6` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT6_R(crate::FieldReader<bool, DOUT6_A>);
impl DOUT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT6_A {
        match self.bits {
            false => DOUT6_A::_0,
            true => DOUT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT6_A::_1
    }
}
impl core::ops::Deref for DOUT6_R {
    type Target = crate::FieldReader<bool, DOUT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT6` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT6_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT6_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT7_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT7_R(crate::FieldReader<bool, DOUT7_A>);
impl DOUT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT7_A {
        match self.bits {
            false => DOUT7_A::_0,
            true => DOUT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT7_A::_1
    }
}
impl core::ops::Deref for DOUT7_R {
    type Target = crate::FieldReader<bool, DOUT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT7` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT7_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT7_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT8_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT8_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT8` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT8_R(crate::FieldReader<bool, DOUT8_A>);
impl DOUT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT8_A {
        match self.bits {
            false => DOUT8_A::_0,
            true => DOUT8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT8_A::_1
    }
}
impl core::ops::Deref for DOUT8_R {
    type Target = crate::FieldReader<bool, DOUT8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT8` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT8_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT8_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT9_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT9_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT9` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT9_R(crate::FieldReader<bool, DOUT9_A>);
impl DOUT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT9_A {
        match self.bits {
            false => DOUT9_A::_0,
            true => DOUT9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT9_A::_1
    }
}
impl core::ops::Deref for DOUT9_R {
    type Target = crate::FieldReader<bool, DOUT9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT9` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT9_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT9_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT10_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT10_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT10` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT10_R(crate::FieldReader<bool, DOUT10_A>);
impl DOUT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT10_A {
        match self.bits {
            false => DOUT10_A::_0,
            true => DOUT10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT10_A::_1
    }
}
impl core::ops::Deref for DOUT10_R {
    type Target = crate::FieldReader<bool, DOUT10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT10` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT10_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT10_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT11_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT11_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT11_R(crate::FieldReader<bool, DOUT11_A>);
impl DOUT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT11_A {
        match self.bits {
            false => DOUT11_A::_0,
            true => DOUT11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT11_A::_1
    }
}
impl core::ops::Deref for DOUT11_R {
    type Target = crate::FieldReader<bool, DOUT11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT11` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT11_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT11_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT12_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT12_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT12` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT12_R(crate::FieldReader<bool, DOUT12_A>);
impl DOUT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT12_A {
        match self.bits {
            false => DOUT12_A::_0,
            true => DOUT12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT12_A::_1
    }
}
impl core::ops::Deref for DOUT12_R {
    type Target = crate::FieldReader<bool, DOUT12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT12` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT12_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT12_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT12_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT13_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT13_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT13` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT13_R(crate::FieldReader<bool, DOUT13_A>);
impl DOUT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT13_A {
        match self.bits {
            false => DOUT13_A::_0,
            true => DOUT13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT13_A::_1
    }
}
impl core::ops::Deref for DOUT13_R {
    type Target = crate::FieldReader<bool, DOUT13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT13` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT13_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT13_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT14_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT14_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT14` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT14_R(crate::FieldReader<bool, DOUT14_A>);
impl DOUT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT14_A {
        match self.bits {
            false => DOUT14_A::_0,
            true => DOUT14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT14_A::_1
    }
}
impl core::ops::Deref for DOUT14_R {
    type Target = crate::FieldReader<bool, DOUT14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT14` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT14_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT14_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT14_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT15_A {
    #[doc = "0: Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    _0 = 0,
    #[doc = "1: Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    _1 = 1,
}
impl From<DOUT15_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15` reader - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT15_R(crate::FieldReader<bool, DOUT15_A>);
impl DOUT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT15_A {
        match self.bits {
            false => DOUT15_A::_0,
            true => DOUT15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT15_A::_1
    }
}
impl core::ops::Deref for DOUT15_R {
    type Target = crate::FieldReader<bool, DOUT15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT15` writer - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DOUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n will drive Low if the Px.n pin is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT15_A::_0)
    }
    #[doc = "Px.n will drive High if the Px.n pin is configured as Push-pull output or Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT15_A::_1)
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
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout6(&self) -> DOUT6_R {
        DOUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout12(&self) -> DOUT12_R {
        DOUT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout0(&mut self) -> DOUT0_W {
        DOUT0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout1(&mut self) -> DOUT1_W {
        DOUT1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout2(&mut self) -> DOUT2_W {
        DOUT2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout3(&mut self) -> DOUT3_W {
        DOUT3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout4(&mut self) -> DOUT4_W {
        DOUT4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT5_W {
        DOUT5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout6(&mut self) -> DOUT6_W {
        DOUT6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout7(&mut self) -> DOUT7_W {
        DOUT7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout8(&mut self) -> DOUT8_W {
        DOUT8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout9(&mut self) -> DOUT9_W {
        DOUT9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout10(&mut self) -> DOUT10_W {
        DOUT10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout11(&mut self) -> DOUT11_W {
        DOUT11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout12(&mut self) -> DOUT12_W {
        DOUT12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout13(&mut self) -> DOUT13_W {
        DOUT13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout14(&mut self) -> DOUT14_W {
        DOUT14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Output Value\\nEach of these bits controls the status of a Px.n pin when the Px.n is configured as Push-pull output, Open-drain output or Quasi-bidirectional mode.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dout15(&mut self) -> DOUT15_W {
        DOUT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Data Output Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_dout](index.html) module"]
pub struct PB_DOUT_SPEC;
impl crate::RegisterSpec for PB_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_dout::R](R) reader structure"]
impl crate::Readable for PB_DOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_dout::W](W) writer structure"]
impl crate::Writable for PB_DOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB_DOUT to value 0xffff"]
impl crate::Resettable for PB_DOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
