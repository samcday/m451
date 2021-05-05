#[doc = "Register `PE_SLEWCTL` reader"]
pub struct R(crate::R<PE_SLEWCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_SLEWCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PE_SLEWCTL_SPEC>> for R {
    fn from(reader: crate::R<PE_SLEWCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE_SLEWCTL` writer"]
pub struct W(crate::W<PE_SLEWCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_SLEWCTL_SPEC>;
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
impl core::convert::From<crate::W<PE_SLEWCTL_SPEC>> for W {
    fn from(writer: crate::W<PE_SLEWCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN0_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN0_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN0` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN0_R(crate::FieldReader<bool, HSREN0_A>);
impl HSREN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN0_A {
        match self.bits {
            false => HSREN0_A::_0,
            true => HSREN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN0_A::_1
    }
}
impl core::ops::Deref for HSREN0_R {
    type Target = crate::FieldReader<bool, HSREN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN0` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN0_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN0_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN1_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN1_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN1` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN1_R(crate::FieldReader<bool, HSREN1_A>);
impl HSREN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN1_A {
        match self.bits {
            false => HSREN1_A::_0,
            true => HSREN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN1_A::_1
    }
}
impl core::ops::Deref for HSREN1_R {
    type Target = crate::FieldReader<bool, HSREN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN1` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN1_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN1_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN2_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN2_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN2` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN2_R(crate::FieldReader<bool, HSREN2_A>);
impl HSREN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN2_A {
        match self.bits {
            false => HSREN2_A::_0,
            true => HSREN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN2_A::_1
    }
}
impl core::ops::Deref for HSREN2_R {
    type Target = crate::FieldReader<bool, HSREN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN2` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN2_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN2_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN2_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN3_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN3_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN3` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN3_R(crate::FieldReader<bool, HSREN3_A>);
impl HSREN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN3_A {
        match self.bits {
            false => HSREN3_A::_0,
            true => HSREN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN3_A::_1
    }
}
impl core::ops::Deref for HSREN3_R {
    type Target = crate::FieldReader<bool, HSREN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN3` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN3_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN3_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN3_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN4_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN4_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN4` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN4_R(crate::FieldReader<bool, HSREN4_A>);
impl HSREN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN4_A {
        match self.bits {
            false => HSREN4_A::_0,
            true => HSREN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN4_A::_1
    }
}
impl core::ops::Deref for HSREN4_R {
    type Target = crate::FieldReader<bool, HSREN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN4` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN4_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN4_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN4_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN5_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN5_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN5` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN5_R(crate::FieldReader<bool, HSREN5_A>);
impl HSREN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN5_A {
        match self.bits {
            false => HSREN5_A::_0,
            true => HSREN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN5_A::_1
    }
}
impl core::ops::Deref for HSREN5_R {
    type Target = crate::FieldReader<bool, HSREN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN5` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN5_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN5_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN5_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN6_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN6_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN6` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN6_R(crate::FieldReader<bool, HSREN6_A>);
impl HSREN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN6_A {
        match self.bits {
            false => HSREN6_A::_0,
            true => HSREN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN6_A::_1
    }
}
impl core::ops::Deref for HSREN6_R {
    type Target = crate::FieldReader<bool, HSREN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN6` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN6_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN6_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN6_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN7_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN7_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN7` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN7_R(crate::FieldReader<bool, HSREN7_A>);
impl HSREN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN7_A {
        match self.bits {
            false => HSREN7_A::_0,
            true => HSREN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN7_A::_1
    }
}
impl core::ops::Deref for HSREN7_R {
    type Target = crate::FieldReader<bool, HSREN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN7` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN7_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN7_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN7_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN8_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN8_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN8` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN8_R(crate::FieldReader<bool, HSREN8_A>);
impl HSREN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN8_A {
        match self.bits {
            false => HSREN8_A::_0,
            true => HSREN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN8_A::_1
    }
}
impl core::ops::Deref for HSREN8_R {
    type Target = crate::FieldReader<bool, HSREN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN8` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN8_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN8_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN8_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN9_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN9_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN9` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN9_R(crate::FieldReader<bool, HSREN9_A>);
impl HSREN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN9_A {
        match self.bits {
            false => HSREN9_A::_0,
            true => HSREN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN9_A::_1
    }
}
impl core::ops::Deref for HSREN9_R {
    type Target = crate::FieldReader<bool, HSREN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN9` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN9_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN9_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN9_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN10_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN10_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN10` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN10_R(crate::FieldReader<bool, HSREN10_A>);
impl HSREN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN10_A {
        match self.bits {
            false => HSREN10_A::_0,
            true => HSREN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN10_A::_1
    }
}
impl core::ops::Deref for HSREN10_R {
    type Target = crate::FieldReader<bool, HSREN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN10` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN10_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN10_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN10_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN11_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN11_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN11` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN11_R(crate::FieldReader<bool, HSREN11_A>);
impl HSREN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN11_A {
        match self.bits {
            false => HSREN11_A::_0,
            true => HSREN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN11_A::_1
    }
}
impl core::ops::Deref for HSREN11_R {
    type Target = crate::FieldReader<bool, HSREN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN11` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN11_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN11_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN11_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN12_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN12_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN12` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN12_R(crate::FieldReader<bool, HSREN12_A>);
impl HSREN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN12_A {
        match self.bits {
            false => HSREN12_A::_0,
            true => HSREN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN12_A::_1
    }
}
impl core::ops::Deref for HSREN12_R {
    type Target = crate::FieldReader<bool, HSREN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN12` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN12_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN12_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN12_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN13_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN13_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN13` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN13_R(crate::FieldReader<bool, HSREN13_A>);
impl HSREN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN13_A {
        match self.bits {
            false => HSREN13_A::_0,
            true => HSREN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN13_A::_1
    }
}
impl core::ops::Deref for HSREN13_R {
    type Target = crate::FieldReader<bool, HSREN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN13` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN13_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN13_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN13_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN14_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN14_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN14` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN14_R(crate::FieldReader<bool, HSREN14_A>);
impl HSREN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN14_A {
        match self.bits {
            false => HSREN14_A::_0,
            true => HSREN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN14_A::_1
    }
}
impl core::ops::Deref for HSREN14_R {
    type Target = crate::FieldReader<bool, HSREN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN14` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN14_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN14_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN14_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSREN15_A {
    #[doc = "0: Px.n output with basic slew rate"]
    _0 = 0,
    #[doc = "1: Px.n output with higher slew rate"]
    _1 = 1,
}
impl From<HSREN15_A> for bool {
    #[inline(always)]
    fn from(variant: HSREN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSREN15` reader - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN15_R(crate::FieldReader<bool, HSREN15_A>);
impl HSREN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSREN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSREN15_A {
        match self.bits {
            false => HSREN15_A::_0,
            true => HSREN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSREN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSREN15_A::_1
    }
}
impl core::ops::Deref for HSREN15_R {
    type Target = crate::FieldReader<bool, HSREN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREN15` writer - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct HSREN15_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSREN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic slew rate"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSREN15_A::_0)
    }
    #[doc = "Px.n output with higher slew rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSREN15_A::_1)
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
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren0(&self) -> HSREN0_R {
        HSREN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren1(&self) -> HSREN1_R {
        HSREN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren2(&self) -> HSREN2_R {
        HSREN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren3(&self) -> HSREN3_R {
        HSREN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren4(&self) -> HSREN4_R {
        HSREN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren5(&self) -> HSREN5_R {
        HSREN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren6(&self) -> HSREN6_R {
        HSREN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren7(&self) -> HSREN7_R {
        HSREN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren8(&self) -> HSREN8_R {
        HSREN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren9(&self) -> HSREN9_R {
        HSREN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren10(&self) -> HSREN10_R {
        HSREN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren11(&self) -> HSREN11_R {
        HSREN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren12(&self) -> HSREN12_R {
        HSREN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren13(&self) -> HSREN13_R {
        HSREN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren14(&self) -> HSREN14_R {
        HSREN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren15(&self) -> HSREN15_R {
        HSREN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren0(&mut self) -> HSREN0_W {
        HSREN0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren1(&mut self) -> HSREN1_W {
        HSREN1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren2(&mut self) -> HSREN2_W {
        HSREN2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren3(&mut self) -> HSREN3_W {
        HSREN3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren4(&mut self) -> HSREN4_W {
        HSREN4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren5(&mut self) -> HSREN5_W {
        HSREN5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren6(&mut self) -> HSREN6_W {
        HSREN6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren7(&mut self) -> HSREN7_W {
        HSREN7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren8(&mut self) -> HSREN8_W {
        HSREN8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren9(&mut self) -> HSREN9_W {
        HSREN9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren10(&mut self) -> HSREN10_W {
        HSREN10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren11(&mut self) -> HSREN11_W {
        HSREN11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren12(&mut self) -> HSREN12_W {
        HSREN12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren13(&mut self) -> HSREN13_W {
        HSREN13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren14(&mut self) -> HSREN14_W {
        HSREN14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
High Slew Rate Control\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn hsren15(&mut self) -> HSREN15_W {
        HSREN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE High Slew Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_slewctl](index.html) module"]
pub struct PE_SLEWCTL_SPEC;
impl crate::RegisterSpec for PE_SLEWCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_slewctl::R](R) reader structure"]
impl crate::Readable for PE_SLEWCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_slewctl::W](W) writer structure"]
impl crate::Writable for PE_SLEWCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE_SLEWCTL to value 0"]
impl crate::Resettable for PE_SLEWCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
