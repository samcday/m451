#[doc = "Register `PWM_FAILBRK` reader"]
pub struct R(crate::R<PWM_FAILBRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FAILBRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_FAILBRK_SPEC>> for R {
    fn from(reader: crate::R<PWM_FAILBRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FAILBRK` writer"]
pub struct W(crate::W<PWM_FAILBRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FAILBRK_SPEC>;
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
impl core::convert::From<crate::W<PWM_FAILBRK_SPEC>> for W {
    fn from(writer: crate::W<PWM_FAILBRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Security System Detection Trigger PWM Brake Function 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSBRKEN_A {
    #[doc = "0: Brake Function triggered by CSS detection Disabled"]
    _0 = 0,
    #[doc = "1: Brake Function triggered by CSS detection Enabled"]
    _1 = 1,
}
impl From<CSSBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSSBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSBRKEN` reader - Clock Security System Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct CSSBRKEN_R(crate::FieldReader<bool, CSSBRKEN_A>);
impl CSSBRKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSBRKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSBRKEN_A {
        match self.bits {
            false => CSSBRKEN_A::_0,
            true => CSSBRKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSSBRKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSSBRKEN_A::_1
    }
}
impl core::ops::Deref for CSSBRKEN_R {
    type Target = crate::FieldReader<bool, CSSBRKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSBRKEN` writer - Clock Security System Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct CSSBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSBRKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSBRKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake Function triggered by CSS detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSSBRKEN_A::_0)
    }
    #[doc = "Brake Function triggered by CSS detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSSBRKEN_A::_1)
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
#[doc = "Brown-out Detection Trigger PWM Brake Function 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODBRKEN_A {
    #[doc = "0: Brake Function triggered by BOD Disabled"]
    _0 = 0,
    #[doc = "1: Brake Function triggered by BOD Enabled"]
    _1 = 1,
}
impl From<BODBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: BODBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODBRKEN` reader - Brown-out Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct BODBRKEN_R(crate::FieldReader<bool, BODBRKEN_A>);
impl BODBRKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODBRKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODBRKEN_A {
        match self.bits {
            false => BODBRKEN_A::_0,
            true => BODBRKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODBRKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODBRKEN_A::_1
    }
}
impl core::ops::Deref for BODBRKEN_R {
    type Target = crate::FieldReader<bool, BODBRKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODBRKEN` writer - Brown-out Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct BODBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODBRKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODBRKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake Function triggered by BOD Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODBRKEN_A::_0)
    }
    #[doc = "Brake Function triggered by BOD Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODBRKEN_A::_1)
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
#[doc = "SRAM Parity Error Detection Trigger PWM Brake Function 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBRKEN_A {
    #[doc = "0: Brake Function triggered by SRAM parity error detection Disabled"]
    _0 = 0,
    #[doc = "1: Brake Function triggered by SRAM parity error detection Enabled"]
    _1 = 1,
}
impl From<RAMBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBRKEN` reader - SRAM Parity Error Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct RAMBRKEN_R(crate::FieldReader<bool, RAMBRKEN_A>);
impl RAMBRKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMBRKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBRKEN_A {
        match self.bits {
            false => RAMBRKEN_A::_0,
            true => RAMBRKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RAMBRKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RAMBRKEN_A::_1
    }
}
impl core::ops::Deref for RAMBRKEN_R {
    type Target = crate::FieldReader<bool, RAMBRKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMBRKEN` writer - SRAM Parity Error Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct RAMBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMBRKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMBRKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake Function triggered by SRAM parity error detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAMBRKEN_A::_0)
    }
    #[doc = "Brake Function triggered by SRAM parity error detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAMBRKEN_A::_1)
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
#[doc = "Core Lockup Detection Trigger PWM Brake Function 0 Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORBRKEN_A {
    #[doc = "0: Brake Function triggered by Core lockup detection Disabled"]
    _0 = 0,
    #[doc = "1: Brake Function triggered by Core lockup detection Enabled"]
    _1 = 1,
}
impl From<CORBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CORBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORBRKEN` reader - Core Lockup Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct CORBRKEN_R(crate::FieldReader<bool, CORBRKEN_A>);
impl CORBRKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORBRKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORBRKEN_A {
        match self.bits {
            false => CORBRKEN_A::_0,
            true => CORBRKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CORBRKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CORBRKEN_A::_1
    }
}
impl core::ops::Deref for CORBRKEN_R {
    type Target = crate::FieldReader<bool, CORBRKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORBRKEN` writer - Core Lockup Detection Trigger PWM Brake Function 0 Enable Bit"]
pub struct CORBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORBRKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORBRKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brake Function triggered by Core lockup detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CORBRKEN_A::_0)
    }
    #[doc = "Brake Function triggered by Core lockup detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CORBRKEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Clock Security System Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn cssbrken(&self) -> CSSBRKEN_R {
        CSSBRKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown-out Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn bodbrken(&self) -> BODBRKEN_R {
        BODBRKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM Parity Error Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn rambrken(&self) -> RAMBRKEN_R {
        RAMBRKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Core Lockup Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn corbrken(&self) -> CORBRKEN_R {
        CORBRKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Security System Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn cssbrken(&mut self) -> CSSBRKEN_W {
        CSSBRKEN_W { w: self }
    }
    #[doc = "Bit 1 - Brown-out Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn bodbrken(&mut self) -> BODBRKEN_W {
        BODBRKEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM Parity Error Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn rambrken(&mut self) -> RAMBRKEN_W {
        RAMBRKEN_W { w: self }
    }
    #[doc = "Bit 3 - Core Lockup Detection Trigger PWM Brake Function 0 Enable Bit"]
    #[inline(always)]
    pub fn corbrken(&mut self) -> CORBRKEN_W {
        CORBRKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM System Fail Brake Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_failbrk](index.html) module"]
pub struct PWM_FAILBRK_SPEC;
impl crate::RegisterSpec for PWM_FAILBRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_failbrk::R](R) reader structure"]
impl crate::Readable for PWM_FAILBRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_failbrk::W](W) writer structure"]
impl crate::Writable for PWM_FAILBRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FAILBRK to value 0"]
impl crate::Resettable for PWM_FAILBRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
