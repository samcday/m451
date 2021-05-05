#[doc = "Register `PD_DBEN` reader"]
pub struct R(crate::R<PD_DBEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_DBEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PD_DBEN_SPEC>> for R {
    fn from(reader: crate::R<PD_DBEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_DBEN` writer"]
pub struct W(crate::W<PD_DBEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_DBEN_SPEC>;
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
impl core::convert::From<crate::W<PD_DBEN_SPEC>> for W {
    fn from(writer: crate::W<PD_DBEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN0_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN0` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN0_R(crate::FieldReader<bool, DBEN0_A>);
impl DBEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN0_A {
        match self.bits {
            false => DBEN0_A::_0,
            true => DBEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN0_A::_1
    }
}
impl core::ops::Deref for DBEN0_R {
    type Target = crate::FieldReader<bool, DBEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN0` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN0_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN0_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN1_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN1` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN1_R(crate::FieldReader<bool, DBEN1_A>);
impl DBEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN1_A {
        match self.bits {
            false => DBEN1_A::_0,
            true => DBEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN1_A::_1
    }
}
impl core::ops::Deref for DBEN1_R {
    type Target = crate::FieldReader<bool, DBEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN1` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN1_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN1_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN2_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN2` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN2_R(crate::FieldReader<bool, DBEN2_A>);
impl DBEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN2_A {
        match self.bits {
            false => DBEN2_A::_0,
            true => DBEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN2_A::_1
    }
}
impl core::ops::Deref for DBEN2_R {
    type Target = crate::FieldReader<bool, DBEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN2` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN2_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN2_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN3_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN3` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN3_R(crate::FieldReader<bool, DBEN3_A>);
impl DBEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN3_A {
        match self.bits {
            false => DBEN3_A::_0,
            true => DBEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN3_A::_1
    }
}
impl core::ops::Deref for DBEN3_R {
    type Target = crate::FieldReader<bool, DBEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN3` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN3_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN3_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN4_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN4_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN4` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN4_R(crate::FieldReader<bool, DBEN4_A>);
impl DBEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN4_A {
        match self.bits {
            false => DBEN4_A::_0,
            true => DBEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN4_A::_1
    }
}
impl core::ops::Deref for DBEN4_R {
    type Target = crate::FieldReader<bool, DBEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN4` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN4_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN4_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN5_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN5_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN5` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN5_R(crate::FieldReader<bool, DBEN5_A>);
impl DBEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN5_A {
        match self.bits {
            false => DBEN5_A::_0,
            true => DBEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN5_A::_1
    }
}
impl core::ops::Deref for DBEN5_R {
    type Target = crate::FieldReader<bool, DBEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN5` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN5_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN5_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN6_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN6_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN6` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN6_R(crate::FieldReader<bool, DBEN6_A>);
impl DBEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN6_A {
        match self.bits {
            false => DBEN6_A::_0,
            true => DBEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN6_A::_1
    }
}
impl core::ops::Deref for DBEN6_R {
    type Target = crate::FieldReader<bool, DBEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN6` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN6_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN6_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN7_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN7_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN7` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN7_R(crate::FieldReader<bool, DBEN7_A>);
impl DBEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN7_A {
        match self.bits {
            false => DBEN7_A::_0,
            true => DBEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN7_A::_1
    }
}
impl core::ops::Deref for DBEN7_R {
    type Target = crate::FieldReader<bool, DBEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN7` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN7_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN7_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN8_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN8_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN8` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN8_R(crate::FieldReader<bool, DBEN8_A>);
impl DBEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN8_A {
        match self.bits {
            false => DBEN8_A::_0,
            true => DBEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN8_A::_1
    }
}
impl core::ops::Deref for DBEN8_R {
    type Target = crate::FieldReader<bool, DBEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN8` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN8_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN8_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN9_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN9_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN9` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN9_R(crate::FieldReader<bool, DBEN9_A>);
impl DBEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN9_A {
        match self.bits {
            false => DBEN9_A::_0,
            true => DBEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN9_A::_1
    }
}
impl core::ops::Deref for DBEN9_R {
    type Target = crate::FieldReader<bool, DBEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN9` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN9_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN9_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN10_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN10_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN10` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN10_R(crate::FieldReader<bool, DBEN10_A>);
impl DBEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN10_A {
        match self.bits {
            false => DBEN10_A::_0,
            true => DBEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN10_A::_1
    }
}
impl core::ops::Deref for DBEN10_R {
    type Target = crate::FieldReader<bool, DBEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN10` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN10_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN10_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN11_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN11_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN11` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN11_R(crate::FieldReader<bool, DBEN11_A>);
impl DBEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN11_A {
        match self.bits {
            false => DBEN11_A::_0,
            true => DBEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN11_A::_1
    }
}
impl core::ops::Deref for DBEN11_R {
    type Target = crate::FieldReader<bool, DBEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN11` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN11_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN11_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN12_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN12_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN12` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN12_R(crate::FieldReader<bool, DBEN12_A>);
impl DBEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN12_A {
        match self.bits {
            false => DBEN12_A::_0,
            true => DBEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN12_A::_1
    }
}
impl core::ops::Deref for DBEN12_R {
    type Target = crate::FieldReader<bool, DBEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN12` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN12_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN12_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN13_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN13_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN13` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN13_R(crate::FieldReader<bool, DBEN13_A>);
impl DBEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN13_A {
        match self.bits {
            false => DBEN13_A::_0,
            true => DBEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN13_A::_1
    }
}
impl core::ops::Deref for DBEN13_R {
    type Target = crate::FieldReader<bool, DBEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN13` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN13_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN13_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN14_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN14_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN14` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN14_R(crate::FieldReader<bool, DBEN14_A>);
impl DBEN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN14_A {
        match self.bits {
            false => DBEN14_A::_0,
            true => DBEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN14_A::_1
    }
}
impl core::ops::Deref for DBEN14_R {
    type Target = crate::FieldReader<bool, DBEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN14` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN14_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN14_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBEN15_A {
    #[doc = "0: Px.n de-bounce function Disabled"]
    _0 = 0,
    #[doc = "1: Px.n de-bounce function Enabled"]
    _1 = 1,
}
impl From<DBEN15_A> for bool {
    #[inline(always)]
    fn from(variant: DBEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBEN15` reader - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN15_R(crate::FieldReader<bool, DBEN15_A>);
impl DBEN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBEN15_A {
        match self.bits {
            false => DBEN15_A::_0,
            true => DBEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBEN15_A::_1
    }
}
impl core::ops::Deref for DBEN15_R {
    type Target = crate::FieldReader<bool, DBEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBEN15` writer - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DBEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> DBEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n de-bounce function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBEN15_A::_0)
    }
    #[doc = "Px.n de-bounce function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBEN15_A::_1)
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
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben0(&self) -> DBEN0_R {
        DBEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben1(&self) -> DBEN1_R {
        DBEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben2(&self) -> DBEN2_R {
        DBEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben3(&self) -> DBEN3_R {
        DBEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben4(&self) -> DBEN4_R {
        DBEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben5(&self) -> DBEN5_R {
        DBEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben6(&self) -> DBEN6_R {
        DBEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben7(&self) -> DBEN7_R {
        DBEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben8(&self) -> DBEN8_R {
        DBEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben9(&self) -> DBEN9_R {
        DBEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben10(&self) -> DBEN10_R {
        DBEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben11(&self) -> DBEN11_R {
        DBEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben12(&self) -> DBEN12_R {
        DBEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben13(&self) -> DBEN13_R {
        DBEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben14(&self) -> DBEN14_R {
        DBEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben15(&self) -> DBEN15_R {
        DBEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben0(&mut self) -> DBEN0_W {
        DBEN0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben1(&mut self) -> DBEN1_W {
        DBEN1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben2(&mut self) -> DBEN2_W {
        DBEN2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben3(&mut self) -> DBEN3_W {
        DBEN3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben4(&mut self) -> DBEN4_W {
        DBEN4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben5(&mut self) -> DBEN5_W {
        DBEN5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben6(&mut self) -> DBEN6_W {
        DBEN6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben7(&mut self) -> DBEN7_W {
        DBEN7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben8(&mut self) -> DBEN8_W {
        DBEN8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben9(&mut self) -> DBEN9_W {
        DBEN9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben10(&mut self) -> DBEN10_W {
        DBEN10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben11(&mut self) -> DBEN11_W {
        DBEN11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben12(&mut self) -> DBEN12_W {
        DBEN12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben13(&mut self) -> DBEN13_W {
        DBEN13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben14(&mut self) -> DBEN14_W {
        DBEN14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Input Signal De-bounce Enable Bit\\nThe DBEN\\[n\\]
bit is used to enable the de-bounce function for each corresponding bit. If the input signal pulse width cannot be sampled by continuous two de-bounce sample cycle, the input signal transition is seen as the signal bounce and will not trigger the interrupt. The de-bounce clock source is controlled by DBCLKSRC (GPIO_DBCTL \\[4\\]), one de-bounce sample cycle period is controlled by DBCLKSEL (GPIO_DBCTL \\[3:0\\]).\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dben15(&mut self) -> DBEN15_W {
        DBEN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD De-bounce Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_dben](index.html) module"]
pub struct PD_DBEN_SPEC;
impl crate::RegisterSpec for PD_DBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_dben::R](R) reader structure"]
impl crate::Readable for PD_DBEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_dben::W](W) writer structure"]
impl crate::Writable for PD_DBEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_DBEN to value 0"]
impl crate::Resettable for PD_DBEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
