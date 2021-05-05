#[doc = "Register `PD_INTSRC` reader"]
pub struct R(crate::R<PD_INTSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_INTSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PD_INTSRC_SPEC>> for R {
    fn from(reader: crate::R<PD_INTSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_INTSRC` writer"]
pub struct W(crate::W<PD_INTSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_INTSRC_SPEC>;
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
impl core::convert::From<crate::W<PD_INTSRC_SPEC>> for W {
    fn from(writer: crate::W<PD_INTSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC0_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC0_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC0` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC0_R(crate::FieldReader<bool, INTSRC0_A>);
impl INTSRC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC0_A {
        match self.bits {
            false => INTSRC0_A::_0,
            true => INTSRC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC0_A::_1
    }
}
impl core::ops::Deref for INTSRC0_R {
    type Target = crate::FieldReader<bool, INTSRC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC0` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC0_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC0_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC1_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC1_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC1` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC1_R(crate::FieldReader<bool, INTSRC1_A>);
impl INTSRC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC1_A {
        match self.bits {
            false => INTSRC1_A::_0,
            true => INTSRC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC1_A::_1
    }
}
impl core::ops::Deref for INTSRC1_R {
    type Target = crate::FieldReader<bool, INTSRC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC1` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC1_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC1_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC2_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC2_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC2` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC2_R(crate::FieldReader<bool, INTSRC2_A>);
impl INTSRC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC2_A {
        match self.bits {
            false => INTSRC2_A::_0,
            true => INTSRC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC2_A::_1
    }
}
impl core::ops::Deref for INTSRC2_R {
    type Target = crate::FieldReader<bool, INTSRC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC2` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC2_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC2_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC3_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC3_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC3` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC3_R(crate::FieldReader<bool, INTSRC3_A>);
impl INTSRC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC3_A {
        match self.bits {
            false => INTSRC3_A::_0,
            true => INTSRC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC3_A::_1
    }
}
impl core::ops::Deref for INTSRC3_R {
    type Target = crate::FieldReader<bool, INTSRC3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC3` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC3_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC3_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC4_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC4_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC4` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC4_R(crate::FieldReader<bool, INTSRC4_A>);
impl INTSRC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC4_A {
        match self.bits {
            false => INTSRC4_A::_0,
            true => INTSRC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC4_A::_1
    }
}
impl core::ops::Deref for INTSRC4_R {
    type Target = crate::FieldReader<bool, INTSRC4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC4` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC4_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC4_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC5_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC5_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC5` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC5_R(crate::FieldReader<bool, INTSRC5_A>);
impl INTSRC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC5_A {
        match self.bits {
            false => INTSRC5_A::_0,
            true => INTSRC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC5_A::_1
    }
}
impl core::ops::Deref for INTSRC5_R {
    type Target = crate::FieldReader<bool, INTSRC5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC5` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC5_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC5_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC6_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC6_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC6` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC6_R(crate::FieldReader<bool, INTSRC6_A>);
impl INTSRC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC6_A {
        match self.bits {
            false => INTSRC6_A::_0,
            true => INTSRC6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC6_A::_1
    }
}
impl core::ops::Deref for INTSRC6_R {
    type Target = crate::FieldReader<bool, INTSRC6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC6` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC6_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC6_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC7_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC7_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC7` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC7_R(crate::FieldReader<bool, INTSRC7_A>);
impl INTSRC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC7_A {
        match self.bits {
            false => INTSRC7_A::_0,
            true => INTSRC7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC7_A::_1
    }
}
impl core::ops::Deref for INTSRC7_R {
    type Target = crate::FieldReader<bool, INTSRC7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC7` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC7_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC7_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC8_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC8_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC8` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC8_R(crate::FieldReader<bool, INTSRC8_A>);
impl INTSRC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC8_A {
        match self.bits {
            false => INTSRC8_A::_0,
            true => INTSRC8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC8_A::_1
    }
}
impl core::ops::Deref for INTSRC8_R {
    type Target = crate::FieldReader<bool, INTSRC8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC8` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC8_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC8_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC9_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC9_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC9` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC9_R(crate::FieldReader<bool, INTSRC9_A>);
impl INTSRC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC9_A {
        match self.bits {
            false => INTSRC9_A::_0,
            true => INTSRC9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC9_A::_1
    }
}
impl core::ops::Deref for INTSRC9_R {
    type Target = crate::FieldReader<bool, INTSRC9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC9` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC9_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC9_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC10_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC10_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC10` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC10_R(crate::FieldReader<bool, INTSRC10_A>);
impl INTSRC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC10_A {
        match self.bits {
            false => INTSRC10_A::_0,
            true => INTSRC10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC10_A::_1
    }
}
impl core::ops::Deref for INTSRC10_R {
    type Target = crate::FieldReader<bool, INTSRC10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC10` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC10_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC10_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC10_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC11_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC11_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC11` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC11_R(crate::FieldReader<bool, INTSRC11_A>);
impl INTSRC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC11_A {
        match self.bits {
            false => INTSRC11_A::_0,
            true => INTSRC11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC11_A::_1
    }
}
impl core::ops::Deref for INTSRC11_R {
    type Target = crate::FieldReader<bool, INTSRC11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC11` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC11_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC11_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC11_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC12_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC12_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC12` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC12_R(crate::FieldReader<bool, INTSRC12_A>);
impl INTSRC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC12_A {
        match self.bits {
            false => INTSRC12_A::_0,
            true => INTSRC12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC12_A::_1
    }
}
impl core::ops::Deref for INTSRC12_R {
    type Target = crate::FieldReader<bool, INTSRC12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC12` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC12_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC12_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC12_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC13_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC13_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC13` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC13_R(crate::FieldReader<bool, INTSRC13_A>);
impl INTSRC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC13_A {
        match self.bits {
            false => INTSRC13_A::_0,
            true => INTSRC13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC13_A::_1
    }
}
impl core::ops::Deref for INTSRC13_R {
    type Target = crate::FieldReader<bool, INTSRC13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC13` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC13_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC13_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC13_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC14_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC14_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC14` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC14_R(crate::FieldReader<bool, INTSRC14_A>);
impl INTSRC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC14_A {
        match self.bits {
            false => INTSRC14_A::_0,
            true => INTSRC14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC14_A::_1
    }
}
impl core::ops::Deref for INTSRC14_R {
    type Target = crate::FieldReader<bool, INTSRC14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC14` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC14_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC14_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC14_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRC15_A {
    #[doc = "0: No action.\\nNo interrupt at Px.n"]
    _0 = 0,
    #[doc = "1: Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    _1 = 1,
}
impl From<INTSRC15_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRC15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRC15` reader - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC15_R(crate::FieldReader<bool, INTSRC15_A>);
impl INTSRC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSRC15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRC15_A {
        match self.bits {
            false => INTSRC15_A::_0,
            true => INTSRC15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTSRC15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTSRC15_A::_1
    }
}
impl core::ops::Deref for INTSRC15_R {
    type Target = crate::FieldReader<bool, INTSRC15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSRC15` writer - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct INTSRC15_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSRC15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTSRC15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action.\\nNo interrupt at Px.n"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTSRC15_A::_0)
    }
    #[doc = "Clear the corresponding pending interrupt.\\nPx.n generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTSRC15_A::_1)
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
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc0(&self) -> INTSRC0_R {
        INTSRC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc1(&self) -> INTSRC1_R {
        INTSRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc2(&self) -> INTSRC2_R {
        INTSRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc3(&self) -> INTSRC3_R {
        INTSRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc4(&self) -> INTSRC4_R {
        INTSRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc5(&self) -> INTSRC5_R {
        INTSRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc6(&self) -> INTSRC6_R {
        INTSRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc7(&self) -> INTSRC7_R {
        INTSRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc8(&self) -> INTSRC8_R {
        INTSRC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc9(&self) -> INTSRC9_R {
        INTSRC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc10(&self) -> INTSRC10_R {
        INTSRC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc11(&self) -> INTSRC11_R {
        INTSRC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc12(&self) -> INTSRC12_R {
        INTSRC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc13(&self) -> INTSRC13_R {
        INTSRC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc14(&self) -> INTSRC14_R {
        INTSRC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc15(&self) -> INTSRC15_R {
        INTSRC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc0(&mut self) -> INTSRC0_W {
        INTSRC0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc1(&mut self) -> INTSRC1_W {
        INTSRC1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc2(&mut self) -> INTSRC2_W {
        INTSRC2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc3(&mut self) -> INTSRC3_W {
        INTSRC3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc4(&mut self) -> INTSRC4_W {
        INTSRC4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc5(&mut self) -> INTSRC5_W {
        INTSRC5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc6(&mut self) -> INTSRC6_W {
        INTSRC6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc7(&mut self) -> INTSRC7_W {
        INTSRC7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc8(&mut self) -> INTSRC8_W {
        INTSRC8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc9(&mut self) -> INTSRC9_W {
        INTSRC9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc10(&mut self) -> INTSRC10_W {
        INTSRC10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc11(&mut self) -> INTSRC11_W {
        INTSRC11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc12(&mut self) -> INTSRC12_W {
        INTSRC12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc13(&mut self) -> INTSRC13_W {
        INTSRC13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc14(&mut self) -> INTSRC14_W {
        INTSRC14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Interrupt Source Flag\\nWrite Operation :\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn intsrc15(&mut self) -> INTSRC15_W {
        INTSRC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Interrupt Source Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_intsrc](index.html) module"]
pub struct PD_INTSRC_SPEC;
impl crate::RegisterSpec for PD_INTSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_intsrc::R](R) reader structure"]
impl crate::Readable for PD_INTSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_intsrc::W](W) writer structure"]
impl crate::Writable for PD_INTSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_INTSRC to value 0"]
impl crate::Resettable for PD_INTSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
