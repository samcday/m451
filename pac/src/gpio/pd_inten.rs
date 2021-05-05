#[doc = "Register `PD_INTEN` reader"]
pub struct R(crate::R<PD_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PD_INTEN_SPEC>> for R {
    fn from(reader: crate::R<PD_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_INTEN` writer"]
pub struct W(crate::W<PD_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_INTEN_SPEC>;
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
impl core::convert::From<crate::W<PD_INTEN_SPEC>> for W {
    fn from(writer: crate::W<PD_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN0_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN0` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN0_R(crate::FieldReader<bool, FLIEN0_A>);
impl FLIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN0_A {
        match self.bits {
            false => FLIEN0_A::_0,
            true => FLIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN0_A::_1
    }
}
impl core::ops::Deref for FLIEN0_R {
    type Target = crate::FieldReader<bool, FLIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN0` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN0_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN0_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN1_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN1` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN1_R(crate::FieldReader<bool, FLIEN1_A>);
impl FLIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN1_A {
        match self.bits {
            false => FLIEN1_A::_0,
            true => FLIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN1_A::_1
    }
}
impl core::ops::Deref for FLIEN1_R {
    type Target = crate::FieldReader<bool, FLIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN1` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN1_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN1_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN2_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN2` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN2_R(crate::FieldReader<bool, FLIEN2_A>);
impl FLIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN2_A {
        match self.bits {
            false => FLIEN2_A::_0,
            true => FLIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN2_A::_1
    }
}
impl core::ops::Deref for FLIEN2_R {
    type Target = crate::FieldReader<bool, FLIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN2` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN2_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN2_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN3_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN3` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN3_R(crate::FieldReader<bool, FLIEN3_A>);
impl FLIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN3_A {
        match self.bits {
            false => FLIEN3_A::_0,
            true => FLIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN3_A::_1
    }
}
impl core::ops::Deref for FLIEN3_R {
    type Target = crate::FieldReader<bool, FLIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN3` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN3_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN3_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN4_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN4` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN4_R(crate::FieldReader<bool, FLIEN4_A>);
impl FLIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN4_A {
        match self.bits {
            false => FLIEN4_A::_0,
            true => FLIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN4_A::_1
    }
}
impl core::ops::Deref for FLIEN4_R {
    type Target = crate::FieldReader<bool, FLIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN4` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN4_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN4_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN5_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN5` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN5_R(crate::FieldReader<bool, FLIEN5_A>);
impl FLIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN5_A {
        match self.bits {
            false => FLIEN5_A::_0,
            true => FLIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN5_A::_1
    }
}
impl core::ops::Deref for FLIEN5_R {
    type Target = crate::FieldReader<bool, FLIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN5` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN5_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN5_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN6_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN6_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN6` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN6_R(crate::FieldReader<bool, FLIEN6_A>);
impl FLIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN6_A {
        match self.bits {
            false => FLIEN6_A::_0,
            true => FLIEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN6_A::_1
    }
}
impl core::ops::Deref for FLIEN6_R {
    type Target = crate::FieldReader<bool, FLIEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN6` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN6_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN6_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN7_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN7_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN7` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN7_R(crate::FieldReader<bool, FLIEN7_A>);
impl FLIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN7_A {
        match self.bits {
            false => FLIEN7_A::_0,
            true => FLIEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN7_A::_1
    }
}
impl core::ops::Deref for FLIEN7_R {
    type Target = crate::FieldReader<bool, FLIEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN7` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN7_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN7_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN8_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN8_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN8` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN8_R(crate::FieldReader<bool, FLIEN8_A>);
impl FLIEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN8_A {
        match self.bits {
            false => FLIEN8_A::_0,
            true => FLIEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN8_A::_1
    }
}
impl core::ops::Deref for FLIEN8_R {
    type Target = crate::FieldReader<bool, FLIEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN8` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN8_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN8_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN9_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN9_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN9` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN9_R(crate::FieldReader<bool, FLIEN9_A>);
impl FLIEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN9_A {
        match self.bits {
            false => FLIEN9_A::_0,
            true => FLIEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN9_A::_1
    }
}
impl core::ops::Deref for FLIEN9_R {
    type Target = crate::FieldReader<bool, FLIEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN9` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN9_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN9_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN10_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN10_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN10` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN10_R(crate::FieldReader<bool, FLIEN10_A>);
impl FLIEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN10_A {
        match self.bits {
            false => FLIEN10_A::_0,
            true => FLIEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN10_A::_1
    }
}
impl core::ops::Deref for FLIEN10_R {
    type Target = crate::FieldReader<bool, FLIEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN10` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN10_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN10_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN11_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN11_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN11` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN11_R(crate::FieldReader<bool, FLIEN11_A>);
impl FLIEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN11_A {
        match self.bits {
            false => FLIEN11_A::_0,
            true => FLIEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN11_A::_1
    }
}
impl core::ops::Deref for FLIEN11_R {
    type Target = crate::FieldReader<bool, FLIEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN11` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN11_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN11_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN12_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN12_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN12` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN12_R(crate::FieldReader<bool, FLIEN12_A>);
impl FLIEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN12_A {
        match self.bits {
            false => FLIEN12_A::_0,
            true => FLIEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN12_A::_1
    }
}
impl core::ops::Deref for FLIEN12_R {
    type Target = crate::FieldReader<bool, FLIEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN12` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN12_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN12_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN13_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN13_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN13` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN13_R(crate::FieldReader<bool, FLIEN13_A>);
impl FLIEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN13_A {
        match self.bits {
            false => FLIEN13_A::_0,
            true => FLIEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN13_A::_1
    }
}
impl core::ops::Deref for FLIEN13_R {
    type Target = crate::FieldReader<bool, FLIEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN13` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN13_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN13_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN14_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN14_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN14` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN14_R(crate::FieldReader<bool, FLIEN14_A>);
impl FLIEN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN14_A {
        match self.bits {
            false => FLIEN14_A::_0,
            true => FLIEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN14_A::_1
    }
}
impl core::ops::Deref for FLIEN14_R {
    type Target = crate::FieldReader<bool, FLIEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN14` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN14_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN14_A::_1)
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
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIEN15_A {
    #[doc = "0: Px.n level low or high to low interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level low or high to low interrupt Enabled"]
    _1 = 1,
}
impl From<FLIEN15_A> for bool {
    #[inline(always)]
    fn from(variant: FLIEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIEN15` reader - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN15_R(crate::FieldReader<bool, FLIEN15_A>);
impl FLIEN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIEN15_A {
        match self.bits {
            false => FLIEN15_A::_0,
            true => FLIEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLIEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLIEN15_A::_1
    }
}
impl core::ops::Deref for FLIEN15_R {
    type Target = crate::FieldReader<bool, FLIEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIEN15` writer - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct FLIEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level low or high to low interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLIEN15_A::_0)
    }
    #[doc = "Px.n level low or high to low interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLIEN15_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN0_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN0` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN0_R(crate::FieldReader<bool, RHIEN0_A>);
impl RHIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN0_A {
        match self.bits {
            false => RHIEN0_A::_0,
            true => RHIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN0_A::_1
    }
}
impl core::ops::Deref for RHIEN0_R {
    type Target = crate::FieldReader<bool, RHIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN0` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN0_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN0_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN1_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN1` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN1_R(crate::FieldReader<bool, RHIEN1_A>);
impl RHIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN1_A {
        match self.bits {
            false => RHIEN1_A::_0,
            true => RHIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN1_A::_1
    }
}
impl core::ops::Deref for RHIEN1_R {
    type Target = crate::FieldReader<bool, RHIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN1` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN1_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN1_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN2_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN2` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN2_R(crate::FieldReader<bool, RHIEN2_A>);
impl RHIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN2_A {
        match self.bits {
            false => RHIEN2_A::_0,
            true => RHIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN2_A::_1
    }
}
impl core::ops::Deref for RHIEN2_R {
    type Target = crate::FieldReader<bool, RHIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN2` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN2_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN2_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN3_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN3` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN3_R(crate::FieldReader<bool, RHIEN3_A>);
impl RHIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN3_A {
        match self.bits {
            false => RHIEN3_A::_0,
            true => RHIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN3_A::_1
    }
}
impl core::ops::Deref for RHIEN3_R {
    type Target = crate::FieldReader<bool, RHIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN3` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN3_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN3_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN4_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN4_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN4` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN4_R(crate::FieldReader<bool, RHIEN4_A>);
impl RHIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN4_A {
        match self.bits {
            false => RHIEN4_A::_0,
            true => RHIEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN4_A::_1
    }
}
impl core::ops::Deref for RHIEN4_R {
    type Target = crate::FieldReader<bool, RHIEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN4` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN4_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN4_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN5_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN5_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN5` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN5_R(crate::FieldReader<bool, RHIEN5_A>);
impl RHIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN5_A {
        match self.bits {
            false => RHIEN5_A::_0,
            true => RHIEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN5_A::_1
    }
}
impl core::ops::Deref for RHIEN5_R {
    type Target = crate::FieldReader<bool, RHIEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN5` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN5_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN5_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN6_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN6_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN6` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN6_R(crate::FieldReader<bool, RHIEN6_A>);
impl RHIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN6_A {
        match self.bits {
            false => RHIEN6_A::_0,
            true => RHIEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN6_A::_1
    }
}
impl core::ops::Deref for RHIEN6_R {
    type Target = crate::FieldReader<bool, RHIEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN6` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN6_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN7_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN7_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN7` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN7_R(crate::FieldReader<bool, RHIEN7_A>);
impl RHIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN7_A {
        match self.bits {
            false => RHIEN7_A::_0,
            true => RHIEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN7_A::_1
    }
}
impl core::ops::Deref for RHIEN7_R {
    type Target = crate::FieldReader<bool, RHIEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN7` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN7_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN7_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN8_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN8_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN8` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN8_R(crate::FieldReader<bool, RHIEN8_A>);
impl RHIEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN8_A {
        match self.bits {
            false => RHIEN8_A::_0,
            true => RHIEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN8_A::_1
    }
}
impl core::ops::Deref for RHIEN8_R {
    type Target = crate::FieldReader<bool, RHIEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN8` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN8_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN8_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN9_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN9_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN9` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN9_R(crate::FieldReader<bool, RHIEN9_A>);
impl RHIEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN9_A {
        match self.bits {
            false => RHIEN9_A::_0,
            true => RHIEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN9_A::_1
    }
}
impl core::ops::Deref for RHIEN9_R {
    type Target = crate::FieldReader<bool, RHIEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN9` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN9_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN9_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN10_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN10_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN10` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN10_R(crate::FieldReader<bool, RHIEN10_A>);
impl RHIEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN10_A {
        match self.bits {
            false => RHIEN10_A::_0,
            true => RHIEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN10_A::_1
    }
}
impl core::ops::Deref for RHIEN10_R {
    type Target = crate::FieldReader<bool, RHIEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN10` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN10_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN10_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN11_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN11_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN11` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN11_R(crate::FieldReader<bool, RHIEN11_A>);
impl RHIEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN11_A {
        match self.bits {
            false => RHIEN11_A::_0,
            true => RHIEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN11_A::_1
    }
}
impl core::ops::Deref for RHIEN11_R {
    type Target = crate::FieldReader<bool, RHIEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN11` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN11_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN11_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN12_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN12_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN12` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN12_R(crate::FieldReader<bool, RHIEN12_A>);
impl RHIEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN12_A {
        match self.bits {
            false => RHIEN12_A::_0,
            true => RHIEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN12_A::_1
    }
}
impl core::ops::Deref for RHIEN12_R {
    type Target = crate::FieldReader<bool, RHIEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN12` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN12_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN13_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN13_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN13` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN13_R(crate::FieldReader<bool, RHIEN13_A>);
impl RHIEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN13_A {
        match self.bits {
            false => RHIEN13_A::_0,
            true => RHIEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN13_A::_1
    }
}
impl core::ops::Deref for RHIEN13_R {
    type Target = crate::FieldReader<bool, RHIEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN13` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN13_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN14_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN14_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN14` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN14_R(crate::FieldReader<bool, RHIEN14_A>);
impl RHIEN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN14_A {
        match self.bits {
            false => RHIEN14_A::_0,
            true => RHIEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN14_A::_1
    }
}
impl core::ops::Deref for RHIEN14_R {
    type Target = crate::FieldReader<bool, RHIEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN14` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN14_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHIEN15_A {
    #[doc = "0: Px.n level high or low to high interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Px.n level high or low to high interrupt Enabled"]
    _1 = 1,
}
impl From<RHIEN15_A> for bool {
    #[inline(always)]
    fn from(variant: RHIEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHIEN15` reader - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN15_R(crate::FieldReader<bool, RHIEN15_A>);
impl RHIEN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHIEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHIEN15_A {
        match self.bits {
            false => RHIEN15_A::_0,
            true => RHIEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHIEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHIEN15_A::_1
    }
}
impl core::ops::Deref for RHIEN15_R {
    type Target = crate::FieldReader<bool, RHIEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHIEN15` writer - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct RHIEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHIEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n level high or low to high interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHIEN15_A::_0)
    }
    #[doc = "Px.n level high or low to high interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHIEN15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien0(&self) -> FLIEN0_R {
        FLIEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien1(&self) -> FLIEN1_R {
        FLIEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien2(&self) -> FLIEN2_R {
        FLIEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien3(&self) -> FLIEN3_R {
        FLIEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien4(&self) -> FLIEN4_R {
        FLIEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien5(&self) -> FLIEN5_R {
        FLIEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien6(&self) -> FLIEN6_R {
        FLIEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien7(&self) -> FLIEN7_R {
        FLIEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien8(&self) -> FLIEN8_R {
        FLIEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien9(&self) -> FLIEN9_R {
        FLIEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien10(&self) -> FLIEN10_R {
        FLIEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien11(&self) -> FLIEN11_R {
        FLIEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien12(&self) -> FLIEN12_R {
        FLIEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien13(&self) -> FLIEN13_R {
        FLIEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien14(&self) -> FLIEN14_R {
        FLIEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien15(&self) -> FLIEN15_R {
        FLIEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien0(&self) -> RHIEN0_R {
        RHIEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien1(&self) -> RHIEN1_R {
        RHIEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien2(&self) -> RHIEN2_R {
        RHIEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien3(&self) -> RHIEN3_R {
        RHIEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien4(&self) -> RHIEN4_R {
        RHIEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien5(&self) -> RHIEN5_R {
        RHIEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien6(&self) -> RHIEN6_R {
        RHIEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien7(&self) -> RHIEN7_R {
        RHIEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien8(&self) -> RHIEN8_R {
        RHIEN8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien9(&self) -> RHIEN9_R {
        RHIEN9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien10(&self) -> RHIEN10_R {
        RHIEN10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien11(&self) -> RHIEN11_R {
        RHIEN11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien12(&self) -> RHIEN12_R {
        RHIEN12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien13(&self) -> RHIEN13_R {
        RHIEN13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien14(&self) -> RHIEN14_R {
        RHIEN14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien15(&self) -> RHIEN15_R {
        RHIEN15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien0(&mut self) -> FLIEN0_W {
        FLIEN0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien1(&mut self) -> FLIEN1_W {
        FLIEN1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien2(&mut self) -> FLIEN2_W {
        FLIEN2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien3(&mut self) -> FLIEN3_W {
        FLIEN3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien4(&mut self) -> FLIEN4_W {
        FLIEN4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien5(&mut self) -> FLIEN5_W {
        FLIEN5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien6(&mut self) -> FLIEN6_W {
        FLIEN6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien7(&mut self) -> FLIEN7_W {
        FLIEN7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien8(&mut self) -> FLIEN8_W {
        FLIEN8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien9(&mut self) -> FLIEN9_W {
        FLIEN9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien10(&mut self) -> FLIEN10_W {
        FLIEN10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien11(&mut self) -> FLIEN11_W {
        FLIEN11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien12(&mut self) -> FLIEN12_W {
        FLIEN12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien13(&mut self) -> FLIEN13_W {
        FLIEN13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien14(&mut self) -> FLIEN14_W {
        FLIEN14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Falling Edge or Low Level Interrupt Trigger Type Enable Bit\\nThe FLIEN (Px_INTEN\\[n\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function.\\nWhen setting the FLIEN (Px_INTEN\\[n\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at low level.\\nIf the interrupt is edge trigger(TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from high to low.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn flien15(&mut self) -> FLIEN15_W {
        FLIEN15_W { w: self }
    }
    #[doc = "Bit 16 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien0(&mut self) -> RHIEN0_W {
        RHIEN0_W { w: self }
    }
    #[doc = "Bit 17 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien1(&mut self) -> RHIEN1_W {
        RHIEN1_W { w: self }
    }
    #[doc = "Bit 18 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien2(&mut self) -> RHIEN2_W {
        RHIEN2_W { w: self }
    }
    #[doc = "Bit 19 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien3(&mut self) -> RHIEN3_W {
        RHIEN3_W { w: self }
    }
    #[doc = "Bit 20 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien4(&mut self) -> RHIEN4_W {
        RHIEN4_W { w: self }
    }
    #[doc = "Bit 21 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien5(&mut self) -> RHIEN5_W {
        RHIEN5_W { w: self }
    }
    #[doc = "Bit 22 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien6(&mut self) -> RHIEN6_W {
        RHIEN6_W { w: self }
    }
    #[doc = "Bit 23 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien7(&mut self) -> RHIEN7_W {
        RHIEN7_W { w: self }
    }
    #[doc = "Bit 24 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien8(&mut self) -> RHIEN8_W {
        RHIEN8_W { w: self }
    }
    #[doc = "Bit 25 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien9(&mut self) -> RHIEN9_W {
        RHIEN9_W { w: self }
    }
    #[doc = "Bit 26 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien10(&mut self) -> RHIEN10_W {
        RHIEN10_W { w: self }
    }
    #[doc = "Bit 27 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien11(&mut self) -> RHIEN11_W {
        RHIEN11_W { w: self }
    }
    #[doc = "Bit 28 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien12(&mut self) -> RHIEN12_W {
        RHIEN12_W { w: self }
    }
    #[doc = "Bit 29 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien13(&mut self) -> RHIEN13_W {
        RHIEN13_W { w: self }
    }
    #[doc = "Bit 30 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien14(&mut self) -> RHIEN14_W {
        RHIEN14_W { w: self }
    }
    #[doc = "Bit 31 - Port A-f Pin\\[N\\]
Rising Edge or High Level Interrupt Trigger Type Enable Bit\\nThe RHIEN (Px_INTEN\\[n+16\\]) bit is used to enable the interrupt for each of the corresponding input Px.n pin. Set bit to 1 also enable the pin wake-up function. \\nWhen setting the RHIEN (Px_INTEN\\[n+16\\]) bit to 1 :\\nIf the interrupt is level trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 1), the input Px.n pin will generate the interrupt while this pin state is at high level.\\nIf the interrupt is edge trigger (TYPE (Px_INTTYPE\\[n\\]) bit is set to 0), the input Px.n pin will generate the interrupt while this pin state changed from low to high.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn rhien15(&mut self) -> RHIEN15_W {
        RHIEN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Interrupt Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_inten](index.html) module"]
pub struct PD_INTEN_SPEC;
impl crate::RegisterSpec for PD_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_inten::R](R) reader structure"]
impl crate::Readable for PD_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_inten::W](W) writer structure"]
impl crate::Writable for PD_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_INTEN to value 0"]
impl crate::Resettable for PD_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
