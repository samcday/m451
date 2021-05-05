#[doc = "Register `SYST_CTRL` reader"]
pub struct R(crate::R<SYST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYST_CTRL_SPEC>> for R {
    fn from(reader: crate::R<SYST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CTRL` writer"]
pub struct W(crate::W<SYST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CTRL_SPEC>;
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
impl core::convert::From<crate::W<SYST_CTRL_SPEC>> for W {
    fn from(writer: crate::W<SYST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System Tick Counter Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Counter Disabled"]
    _0 = 0,
    #[doc = "1: Counter will operate in a multi-shot manner"]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - System Tick Counter Enabled"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENABLE_A::_1
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - System Tick Counter Enabled"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Counter will operate in a multi-shot manner"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
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
#[doc = "System Tick Interrupt Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINT_A {
    #[doc = "0: Counting down to 0 does not cause the SysTick exception to be pended. Software can use COUNTFLAG to determine if a count to zero has occurred"]
    _0 = 0,
    #[doc = "1: Counting down to 0 will cause the SysTick exception to be pended. Clearing the SysTick current value register by a register write in software will not cause SysTick to be pended"]
    _1 = 1,
}
impl From<TICKINT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - System Tick Interrupt Enabled"]
pub struct TICKINT_R(crate::FieldReader<bool, TICKINT_A>);
impl TICKINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKINT_A {
        match self.bits {
            false => TICKINT_A::_0,
            true => TICKINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TICKINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TICKINT_A::_1
    }
}
impl core::ops::Deref for TICKINT_R {
    type Target = crate::FieldReader<bool, TICKINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKINT` writer - System Tick Interrupt Enabled"]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counting down to 0 does not cause the SysTick exception to be pended. Software can use COUNTFLAG to determine if a count to zero has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TICKINT_A::_0)
    }
    #[doc = "Counting down to 0 will cause the SysTick exception to be pended. Clearing the SysTick current value register by a register write in software will not cause SysTick to be pended"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TICKINT_A::_1)
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
#[doc = "System Tick Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: Clock source is the (optional) external reference clock"]
    _0 = 0,
    #[doc = "1: Core clock used for SysTick"]
    _1 = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - System Tick Clock Source Selection"]
pub struct CLKSRC_R(crate::FieldReader<bool, CLKSRC_A>);
impl CLKSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::_0,
            true => CLKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKSRC_A::_1
    }
}
impl core::ops::Deref for CLKSRC_R {
    type Target = crate::FieldReader<bool, CLKSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSRC` writer - System Tick Clock Source Selection"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source is the (optional) external reference clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRC_A::_0)
    }
    #[doc = "Core clock used for SysTick"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRC_A::_1)
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
#[doc = "Field `COUNTFLAG` reader - System Tick Counter Flag\\nReturns 1 if timer counted to 0 since last time this register was read.\\nCOUNTFLAG is set by a count transition from 1 to 0.\\nCOUNTFLAG is cleared on read or by a write to the Current Value register."]
pub struct COUNTFLAG_R(crate::FieldReader<bool, bool>);
impl COUNTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTFLAG` writer - System Tick Counter Flag\\nReturns 1 if timer counted to 0 since last time this register was read.\\nCOUNTFLAG is set by a count transition from 1 to 0.\\nCOUNTFLAG is cleared on read or by a write to the Current Value register."]
pub struct COUNTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTFLAG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - System Tick Counter Enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System Tick Interrupt Enabled"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Tick Clock Source Selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - System Tick Counter Flag\\nReturns 1 if timer counted to 0 since last time this register was read.\\nCOUNTFLAG is set by a count transition from 1 to 0.\\nCOUNTFLAG is cleared on read or by a write to the Current Value register."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Tick Counter Enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - System Tick Interrupt Enabled"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 2 - System Tick Clock Source Selection"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 16 - System Tick Counter Flag\\nReturns 1 if timer counted to 0 since last time this register was read.\\nCOUNTFLAG is set by a count transition from 1 to 0.\\nCOUNTFLAG is cleared on read or by a write to the Current Value register."]
    #[inline(always)]
    pub fn countflag(&mut self) -> COUNTFLAG_W {
        COUNTFLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_ctrl](index.html) module"]
pub struct SYST_CTRL_SPEC;
impl crate::RegisterSpec for SYST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_ctrl::R](R) reader structure"]
impl crate::Readable for SYST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_ctrl::W](W) writer structure"]
impl crate::Writable for SYST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYST_CTRL to value 0"]
impl crate::Resettable for SYST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
