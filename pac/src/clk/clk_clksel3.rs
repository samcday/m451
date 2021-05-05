#[doc = "Register `CLK_CLKSEL3` reader"]
pub struct R(crate::R<CLK_CLKSEL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKSEL3_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKSEL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL3` writer"]
pub struct W(crate::W<CLK_CLKSEL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL3_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKSEL3_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKSEL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SC0 Clock Source Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SC0SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK0"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<SC0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SC0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SC0SEL` reader - SC0 Clock Source Selection"]
pub struct SC0SEL_R(crate::FieldReader<u8, SC0SEL_A>);
impl SC0SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SC0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC0SEL_A {
        match self.bits {
            0 => SC0SEL_A::_0,
            1 => SC0SEL_A::_1,
            2 => SC0SEL_A::_2,
            3 => SC0SEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC0SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SC0SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SC0SEL_A::_3
    }
}
impl core::ops::Deref for SC0SEL_R {
    type Target = crate::FieldReader<u8, SC0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0SEL` writer - SC0 Clock Source Selection"]
pub struct SC0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC0SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC0SEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC0SEL_A::_1)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SC0SEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SC0SEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "RTC Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSEL_A {
    #[doc = "0: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _1 = 1,
}
impl From<RTCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCSEL` reader - RTC Clock Source Selection"]
pub struct RTCSEL_R(crate::FieldReader<bool, RTCSEL_A>);
impl RTCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            false => RTCSEL_A::_0,
            true => RTCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTCSEL_A::_1
    }
}
impl core::ops::Deref for RTCSEL_R {
    type Target = crate::FieldReader<bool, RTCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Source Selection"]
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCSEL_A::_0)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCSEL_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - SC0 Clock Source Selection"]
    #[inline(always)]
    pub fn sc0sel(&self) -> SC0SEL_R {
        SC0SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - RTC Clock Source Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SC0 Clock Source Selection"]
    #[inline(always)]
    pub fn sc0sel(&mut self) -> SC0SEL_W {
        SC0SEL_W { w: self }
    }
    #[doc = "Bit 8 - RTC Clock Source Selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel3](index.html) module"]
pub struct CLK_CLKSEL3_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel3::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel3::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL3 to value 0x03"]
impl crate::Resettable for CLK_CLKSEL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
