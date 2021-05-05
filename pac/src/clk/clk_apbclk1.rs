#[doc = "Register `CLK_APBCLK1` reader"]
pub struct R(crate::R<CLK_APBCLK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_APBCLK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_APBCLK1_SPEC>> for R {
    fn from(reader: crate::R<CLK_APBCLK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_APBCLK1` writer"]
pub struct W(crate::W<CLK_APBCLK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_APBCLK1_SPEC>;
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
impl core::convert::From<crate::W<CLK_APBCLK1_SPEC>> for W {
    fn from(writer: crate::W<CLK_APBCLK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SC0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC0CKEN_A {
    #[doc = "0: SC0 clock Disabled"]
    _0 = 0,
    #[doc = "1: SC0 clock Enabled"]
    _1 = 1,
}
impl From<SC0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SC0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC0CKEN` reader - SC0 Clock Enable Bit"]
pub struct SC0CKEN_R(crate::FieldReader<bool, SC0CKEN_A>);
impl SC0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC0CKEN_A {
        match self.bits {
            false => SC0CKEN_A::_0,
            true => SC0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC0CKEN_A::_1
    }
}
impl core::ops::Deref for SC0CKEN_R {
    type Target = crate::FieldReader<bool, SC0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0CKEN` writer - SC0 Clock Enable Bit"]
pub struct SC0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC0CKEN_A::_0)
    }
    #[doc = "SC0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC0CKEN_A::_1)
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
#[doc = "DAC Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCKEN_A {
    #[doc = "0: DAC clock Disabled"]
    _0 = 0,
    #[doc = "1: DAC clock Enabled"]
    _1 = 1,
}
impl From<DACCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCKEN` reader - DAC Clock Enable Bit"]
pub struct DACCKEN_R(crate::FieldReader<bool, DACCKEN_A>);
impl DACCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCKEN_A {
        match self.bits {
            false => DACCKEN_A::_0,
            true => DACCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACCKEN_A::_1
    }
}
impl core::ops::Deref for DACCKEN_R {
    type Target = crate::FieldReader<bool, DACCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACCKEN` writer - DAC Clock Enable Bit"]
pub struct DACCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACCKEN_A::_0)
    }
    #[doc = "DAC clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACCKEN_A::_1)
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
#[doc = "PWM0 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0CKEN_A {
    #[doc = "0: PWM0 clock Disabled"]
    _0 = 0,
    #[doc = "1: PWM0 clock Enabled"]
    _1 = 1,
}
impl From<PWM0CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM0CKEN` reader - PWM0 Clock Enable Bit"]
pub struct PWM0CKEN_R(crate::FieldReader<bool, PWM0CKEN_A>);
impl PWM0CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM0CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0CKEN_A {
        match self.bits {
            false => PWM0CKEN_A::_0,
            true => PWM0CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM0CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM0CKEN_A::_1
    }
}
impl core::ops::Deref for PWM0CKEN_R {
    type Target = crate::FieldReader<bool, PWM0CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0CKEN` writer - PWM0 Clock Enable Bit"]
pub struct PWM0CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM0CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM0 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM0CKEN_A::_0)
    }
    #[doc = "PWM0 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM0CKEN_A::_1)
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
#[doc = "PWM1 Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1CKEN_A {
    #[doc = "0: PWM1 clock Disabled"]
    _0 = 0,
    #[doc = "1: PWM1 clock Enabled"]
    _1 = 1,
}
impl From<PWM1CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM1CKEN` reader - PWM1 Clock Enable Bit"]
pub struct PWM1CKEN_R(crate::FieldReader<bool, PWM1CKEN_A>);
impl PWM1CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM1CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1CKEN_A {
        match self.bits {
            false => PWM1CKEN_A::_0,
            true => PWM1CKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM1CKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM1CKEN_A::_1
    }
}
impl core::ops::Deref for PWM1CKEN_R {
    type Target = crate::FieldReader<bool, PWM1CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1CKEN` writer - PWM1 Clock Enable Bit"]
pub struct PWM1CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM1CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM1 clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM1CKEN_A::_0)
    }
    #[doc = "PWM1 clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM1CKEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - SC0 Clock Enable Bit"]
    #[inline(always)]
    pub fn sc0cken(&self) -> SC0CKEN_R {
        SC0CKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 12 - DAC Clock Enable Bit"]
    #[inline(always)]
    pub fn daccken(&self) -> DACCKEN_R {
        DACCKEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM0 Clock Enable Bit"]
    #[inline(always)]
    pub fn pwm0cken(&self) -> PWM0CKEN_R {
        PWM0CKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWM1 Clock Enable Bit"]
    #[inline(always)]
    pub fn pwm1cken(&self) -> PWM1CKEN_R {
        PWM1CKEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SC0 Clock Enable Bit"]
    #[inline(always)]
    pub fn sc0cken(&mut self) -> SC0CKEN_W {
        SC0CKEN_W { w: self }
    }
    #[doc = "Bit 12 - DAC Clock Enable Bit"]
    #[inline(always)]
    pub fn daccken(&mut self) -> DACCKEN_W {
        DACCKEN_W { w: self }
    }
    #[doc = "Bit 16 - PWM0 Clock Enable Bit"]
    #[inline(always)]
    pub fn pwm0cken(&mut self) -> PWM0CKEN_W {
        PWM0CKEN_W { w: self }
    }
    #[doc = "Bit 17 - PWM1 Clock Enable Bit"]
    #[inline(always)]
    pub fn pwm1cken(&mut self) -> PWM1CKEN_W {
        PWM1CKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Devices Clock Enable Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_apbclk1](index.html) module"]
pub struct CLK_APBCLK1_SPEC;
impl crate::RegisterSpec for CLK_APBCLK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_apbclk1::R](R) reader structure"]
impl crate::Readable for CLK_APBCLK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_apbclk1::W](W) writer structure"]
impl crate::Writable for CLK_APBCLK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_APBCLK1 to value 0"]
impl crate::Resettable for CLK_APBCLK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
