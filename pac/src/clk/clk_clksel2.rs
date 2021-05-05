#[doc = "Register `CLK_CLKSEL2` reader"]
pub struct R(crate::R<CLK_CLKSEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKSEL2_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKSEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL2` writer"]
pub struct W(crate::W<CLK_CLKSEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL2_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKSEL2_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKSEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM0 Clock Source Selection\\nThe peripheral clock source of PWM0 is defined by PWM0SEL.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0SEL_A {
    #[doc = "0: Clock source from PLL"]
    _0 = 0,
    #[doc = "1: Clock source from PCLK0"]
    _1 = 1,
}
impl From<PWM0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM0SEL` reader - PWM0 Clock Source Selection\\nThe peripheral clock source of PWM0 is defined by PWM0SEL."]
pub struct PWM0SEL_R(crate::FieldReader<bool, PWM0SEL_A>);
impl PWM0SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0SEL_A {
        match self.bits {
            false => PWM0SEL_A::_0,
            true => PWM0SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM0SEL_A::_1
    }
}
impl core::ops::Deref for PWM0SEL_R {
    type Target = crate::FieldReader<bool, PWM0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0SEL` writer - PWM0 Clock Source Selection\\nThe peripheral clock source of PWM0 is defined by PWM0SEL."]
pub struct PWM0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM0SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM0SEL_A::_0)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM0SEL_A::_1)
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
#[doc = "PWM1 Clock Source Selection\\nThe peripheral clock source of PWM1 is defined by PWM1SEL.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1SEL_A {
    #[doc = "0: Clock source from PLL"]
    _0 = 0,
    #[doc = "1: Clock source from PCLK1"]
    _1 = 1,
}
impl From<PWM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM1SEL` reader - PWM1 Clock Source Selection\\nThe peripheral clock source of PWM1 is defined by PWM1SEL."]
pub struct PWM1SEL_R(crate::FieldReader<bool, PWM1SEL_A>);
impl PWM1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1SEL_A {
        match self.bits {
            false => PWM1SEL_A::_0,
            true => PWM1SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWM1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWM1SEL_A::_1
    }
}
impl core::ops::Deref for PWM1SEL_R {
    type Target = crate::FieldReader<bool, PWM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1SEL` writer - PWM1 Clock Source Selection\\nThe peripheral clock source of PWM1 is defined by PWM1SEL."]
pub struct PWM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWM1SEL_A::_0)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWM1SEL_A::_1)
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
#[doc = "SPI0 Clock Source Selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI0SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK0"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<SPI0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI0SEL` reader - SPI0 Clock Source Selection"]
pub struct SPI0SEL_R(crate::FieldReader<u8, SPI0SEL_A>);
impl SPI0SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0SEL_A {
        match self.bits {
            0 => SPI0SEL_A::_0,
            1 => SPI0SEL_A::_1,
            2 => SPI0SEL_A::_2,
            3 => SPI0SEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SPI0SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SPI0SEL_A::_3
    }
}
impl core::ops::Deref for SPI0SEL_R {
    type Target = crate::FieldReader<u8, SPI0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0SEL` writer - SPI0 Clock Source Selection"]
pub struct SPI0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0SEL_A::_1)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SPI0SEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SPI0SEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "SPI1 Clock Source Selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI1SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK1"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<SPI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI1SEL` reader - SPI1 Clock Source Selection"]
pub struct SPI1SEL_R(crate::FieldReader<u8, SPI1SEL_A>);
impl SPI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1SEL_A {
        match self.bits {
            0 => SPI1SEL_A::_0,
            1 => SPI1SEL_A::_1,
            2 => SPI1SEL_A::_2,
            3 => SPI1SEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI1SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SPI1SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SPI1SEL_A::_3
    }
}
impl core::ops::Deref for SPI1SEL_R {
    type Target = crate::FieldReader<u8, SPI1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1SEL` writer - SPI1 Clock Source Selection"]
pub struct SPI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1SEL_A::_1)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SPI1SEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SPI1SEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "SPI2 Clock Source Selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI2SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK0"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<SPI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI2SEL` reader - SPI2 Clock Source Selection"]
pub struct SPI2SEL_R(crate::FieldReader<u8, SPI2SEL_A>);
impl SPI2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2SEL_A {
        match self.bits {
            0 => SPI2SEL_A::_0,
            1 => SPI2SEL_A::_1,
            2 => SPI2SEL_A::_2,
            3 => SPI2SEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI2SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI2SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SPI2SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SPI2SEL_A::_3
    }
}
impl core::ops::Deref for SPI2SEL_R {
    type Target = crate::FieldReader<u8, SPI2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2SEL` writer - SPI2 Clock Source Selection"]
pub struct SPI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI2SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI2SEL_A::_1)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SPI2SEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SPI2SEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM0 Clock Source Selection\\nThe peripheral clock source of PWM0 is defined by PWM0SEL."]
    #[inline(always)]
    pub fn pwm0sel(&self) -> PWM0SEL_R {
        PWM0SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM1 Clock Source Selection\\nThe peripheral clock source of PWM1 is defined by PWM1SEL."]
    #[inline(always)]
    pub fn pwm1sel(&self) -> PWM1SEL_R {
        PWM1SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - SPI0 Clock Source Selection"]
    #[inline(always)]
    pub fn spi0sel(&self) -> SPI0SEL_R {
        SPI0SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SPI1 Clock Source Selection"]
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - SPI2 Clock Source Selection"]
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Clock Source Selection\\nThe peripheral clock source of PWM0 is defined by PWM0SEL."]
    #[inline(always)]
    pub fn pwm0sel(&mut self) -> PWM0SEL_W {
        PWM0SEL_W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Clock Source Selection\\nThe peripheral clock source of PWM1 is defined by PWM1SEL."]
    #[inline(always)]
    pub fn pwm1sel(&mut self) -> PWM1SEL_W {
        PWM1SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI0 Clock Source Selection"]
    #[inline(always)]
    pub fn spi0sel(&mut self) -> SPI0SEL_W {
        SPI0SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI1 Clock Source Selection"]
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W {
        SPI1SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - SPI2 Clock Source Selection"]
    #[inline(always)]
    pub fn spi2sel(&mut self) -> SPI2SEL_W {
        SPI2SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel2](index.html) module"]
pub struct CLK_CLKSEL2_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel2::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel2::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL2 to value 0xab"]
impl crate::Resettable for CLK_CLKSEL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xab
    }
}
