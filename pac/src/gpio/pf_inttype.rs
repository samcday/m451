#[doc = "Register `PF_INTTYPE` reader"]
pub struct R(crate::R<PF_INTTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_INTTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PF_INTTYPE_SPEC>> for R {
    fn from(reader: crate::R<PF_INTTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_INTTYPE` writer"]
pub struct W(crate::W<PF_INTTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_INTTYPE_SPEC>;
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
impl core::convert::From<crate::W<PF_INTTYPE_SPEC>> for W {
    fn from(writer: crate::W<PF_INTTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE0_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE0_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE0` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE0_R(crate::FieldReader<bool, TYPE0_A>);
impl TYPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE0_A {
        match self.bits {
            false => TYPE0_A::_0,
            true => TYPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE0_A::_1
    }
}
impl core::ops::Deref for TYPE0_R {
    type Target = crate::FieldReader<bool, TYPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE0` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE0_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE0_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE1_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE1_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE1` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE1_R(crate::FieldReader<bool, TYPE1_A>);
impl TYPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE1_A {
        match self.bits {
            false => TYPE1_A::_0,
            true => TYPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE1_A::_1
    }
}
impl core::ops::Deref for TYPE1_R {
    type Target = crate::FieldReader<bool, TYPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE1` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE1_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE1_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE2_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE2_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE2` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE2_R(crate::FieldReader<bool, TYPE2_A>);
impl TYPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE2_A {
        match self.bits {
            false => TYPE2_A::_0,
            true => TYPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE2_A::_1
    }
}
impl core::ops::Deref for TYPE2_R {
    type Target = crate::FieldReader<bool, TYPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE2` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE2_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE2_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE3_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE3_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE3` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE3_R(crate::FieldReader<bool, TYPE3_A>);
impl TYPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE3_A {
        match self.bits {
            false => TYPE3_A::_0,
            true => TYPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE3_A::_1
    }
}
impl core::ops::Deref for TYPE3_R {
    type Target = crate::FieldReader<bool, TYPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE3` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE3_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE3_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE4_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE4_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE4` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE4_R(crate::FieldReader<bool, TYPE4_A>);
impl TYPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE4_A {
        match self.bits {
            false => TYPE4_A::_0,
            true => TYPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE4_A::_1
    }
}
impl core::ops::Deref for TYPE4_R {
    type Target = crate::FieldReader<bool, TYPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE4` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE4_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE4_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE5_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE5_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE5` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE5_R(crate::FieldReader<bool, TYPE5_A>);
impl TYPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE5_A {
        match self.bits {
            false => TYPE5_A::_0,
            true => TYPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE5_A::_1
    }
}
impl core::ops::Deref for TYPE5_R {
    type Target = crate::FieldReader<bool, TYPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE5` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE5_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE5_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE6_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE6_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE6` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE6_R(crate::FieldReader<bool, TYPE6_A>);
impl TYPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE6_A {
        match self.bits {
            false => TYPE6_A::_0,
            true => TYPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE6_A::_1
    }
}
impl core::ops::Deref for TYPE6_R {
    type Target = crate::FieldReader<bool, TYPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE6` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE6_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE6_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE7_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE7_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE7` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE7_R(crate::FieldReader<bool, TYPE7_A>);
impl TYPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE7_A {
        match self.bits {
            false => TYPE7_A::_0,
            true => TYPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE7_A::_1
    }
}
impl core::ops::Deref for TYPE7_R {
    type Target = crate::FieldReader<bool, TYPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE7` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE7_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE7_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE8_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE8_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE8` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE8_R(crate::FieldReader<bool, TYPE8_A>);
impl TYPE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE8_A {
        match self.bits {
            false => TYPE8_A::_0,
            true => TYPE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE8_A::_1
    }
}
impl core::ops::Deref for TYPE8_R {
    type Target = crate::FieldReader<bool, TYPE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE8` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE8_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE8_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE8_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE9_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE9_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE9` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE9_R(crate::FieldReader<bool, TYPE9_A>);
impl TYPE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE9_A {
        match self.bits {
            false => TYPE9_A::_0,
            true => TYPE9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE9_A::_1
    }
}
impl core::ops::Deref for TYPE9_R {
    type Target = crate::FieldReader<bool, TYPE9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE9` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE9_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE9_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE9_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE10_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE10_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE10` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE10_R(crate::FieldReader<bool, TYPE10_A>);
impl TYPE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE10_A {
        match self.bits {
            false => TYPE10_A::_0,
            true => TYPE10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE10_A::_1
    }
}
impl core::ops::Deref for TYPE10_R {
    type Target = crate::FieldReader<bool, TYPE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE10` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE10_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE10_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE10_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE11_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE11_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE11` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE11_R(crate::FieldReader<bool, TYPE11_A>);
impl TYPE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE11_A {
        match self.bits {
            false => TYPE11_A::_0,
            true => TYPE11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE11_A::_1
    }
}
impl core::ops::Deref for TYPE11_R {
    type Target = crate::FieldReader<bool, TYPE11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE11` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE11_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE11_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE11_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE12_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE12_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE12` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE12_R(crate::FieldReader<bool, TYPE12_A>);
impl TYPE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE12_A {
        match self.bits {
            false => TYPE12_A::_0,
            true => TYPE12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE12_A::_1
    }
}
impl core::ops::Deref for TYPE12_R {
    type Target = crate::FieldReader<bool, TYPE12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE12` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE12_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE12_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE12_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE13_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE13_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE13` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE13_R(crate::FieldReader<bool, TYPE13_A>);
impl TYPE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE13_A {
        match self.bits {
            false => TYPE13_A::_0,
            true => TYPE13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE13_A::_1
    }
}
impl core::ops::Deref for TYPE13_R {
    type Target = crate::FieldReader<bool, TYPE13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE13` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE13_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE13_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE13_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE14_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE14_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE14` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE14_R(crate::FieldReader<bool, TYPE14_A>);
impl TYPE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE14_A {
        match self.bits {
            false => TYPE14_A::_0,
            true => TYPE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE14_A::_1
    }
}
impl core::ops::Deref for TYPE14_R {
    type Target = crate::FieldReader<bool, TYPE14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE14` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE14_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE14_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE14_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE15_A {
    #[doc = "0: Edge trigger interrupt"]
    _0 = 0,
    #[doc = "1: Level trigger interrupt"]
    _1 = 1,
}
impl From<TYPE15_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE15` reader - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE15_R(crate::FieldReader<bool, TYPE15_A>);
impl TYPE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPE15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE15_A {
        match self.bits {
            false => TYPE15_A::_0,
            true => TYPE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TYPE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TYPE15_A::_1
    }
}
impl core::ops::Deref for TYPE15_R {
    type Target = crate::FieldReader<bool, TYPE15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE15` writer - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct TYPE15_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge trigger interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TYPE15_A::_0)
    }
    #[doc = "Level trigger interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TYPE15_A::_1)
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
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type0(&self) -> TYPE0_R {
        TYPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type1(&self) -> TYPE1_R {
        TYPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type2(&self) -> TYPE2_R {
        TYPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type3(&self) -> TYPE3_R {
        TYPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type4(&self) -> TYPE4_R {
        TYPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type5(&self) -> TYPE5_R {
        TYPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type6(&self) -> TYPE6_R {
        TYPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type7(&self) -> TYPE7_R {
        TYPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type8(&self) -> TYPE8_R {
        TYPE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type9(&self) -> TYPE9_R {
        TYPE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type10(&self) -> TYPE10_R {
        TYPE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type11(&self) -> TYPE11_R {
        TYPE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type12(&self) -> TYPE12_R {
        TYPE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type13(&self) -> TYPE13_R {
        TYPE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type14(&self) -> TYPE14_R {
        TYPE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type15(&self) -> TYPE15_R {
        TYPE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type0(&mut self) -> TYPE0_W {
        TYPE0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type1(&mut self) -> TYPE1_W {
        TYPE1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type2(&mut self) -> TYPE2_W {
        TYPE2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type3(&mut self) -> TYPE3_W {
        TYPE3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type4(&mut self) -> TYPE4_W {
        TYPE4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type5(&mut self) -> TYPE5_W {
        TYPE5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type6(&mut self) -> TYPE6_W {
        TYPE6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type7(&mut self) -> TYPE7_W {
        TYPE7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type8(&mut self) -> TYPE8_W {
        TYPE8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type9(&mut self) -> TYPE9_W {
        TYPE9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type10(&mut self) -> TYPE10_W {
        TYPE10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type11(&mut self) -> TYPE11_W {
        TYPE11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type12(&mut self) -> TYPE12_W {
        TYPE12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type13(&mut self) -> TYPE13_W {
        TYPE13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type14(&mut self) -> TYPE14_W {
        TYPE14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Edge or Level Detection Interrupt Trigger Type Control\\nTYPE (Px_INTTYPE\\[n\\]) bit is used to control the triggered interrupt is by level trigger or by edge trigger. If the interrupt is by edge trigger, the trigger source can be controlled by de-bounce. If the interrupt is by level trigger, the input source is sampled by one HCLK clock and generates the interrupt.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn type15(&mut self) -> TYPE15_W {
        TYPE15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Interrupt Trigger Type Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_inttype](index.html) module"]
pub struct PF_INTTYPE_SPEC;
impl crate::RegisterSpec for PF_INTTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_inttype::R](R) reader structure"]
impl crate::Readable for PF_INTTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_inttype::W](W) writer structure"]
impl crate::Writable for PF_INTTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_INTTYPE to value 0"]
impl crate::Resettable for PF_INTTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
