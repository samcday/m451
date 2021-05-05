#[doc = "Register `CLK_CLKSEL1` reader"]
pub struct R(crate::R<CLK_CLKSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKSEL1_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL1` writer"]
pub struct W(crate::W<CLK_CLKSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL1_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKSEL1_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Timer Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: Reserved."]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from HCLK/2048"]
    _2 = 2,
    #[doc = "3: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _3 = 3,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTSEL` reader - Watchdog Timer Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct WDTSEL_R(crate::FieldReader<u8, WDTSEL_A>);
impl WDTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSEL_A {
        match self.bits {
            0 => WDTSEL_A::_0,
            1 => WDTSEL_A::_1,
            2 => WDTSEL_A::_2,
            3 => WDTSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDTSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == WDTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == WDTSEL_A::_3
    }
}
impl core::ops::Deref for WDTSEL_R {
    type Target = crate::FieldReader<u8, WDTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTSEL` writer - Watchdog Timer Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct WDTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTSEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTSEL_A::_1)
    }
    #[doc = "Clock source from HCLK/2048"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WDTSEL_A::_2)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WDTSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "TIMER0 Clock Source Selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR0SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK0"]
    _2 = 2,
    #[doc = "3: Clock source from external clock T0 pin"]
    _3 = 3,
    #[doc = "5: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _5 = 5,
    #[doc = "7: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _7 = 7,
}
impl From<TMR0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR0SEL` reader - TIMER0 Clock Source Selection"]
pub struct TMR0SEL_R(crate::FieldReader<u8, TMR0SEL_A>);
impl TMR0SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMR0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR0SEL_A> {
        match self.bits {
            0 => Some(TMR0SEL_A::_0),
            1 => Some(TMR0SEL_A::_1),
            2 => Some(TMR0SEL_A::_2),
            3 => Some(TMR0SEL_A::_3),
            5 => Some(TMR0SEL_A::_5),
            7 => Some(TMR0SEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR0SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TMR0SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TMR0SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TMR0SEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TMR0SEL_A::_7
    }
}
impl core::ops::Deref for TMR0SEL_R {
    type Target = crate::FieldReader<u8, TMR0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0SEL` writer - TIMER0 Clock Source Selection"]
pub struct TMR0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_1)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_2)
    }
    #[doc = "Clock source from external clock T0 pin"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_3)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_5)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TMR0SEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "TIMER1 Clock Source Selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR1SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK0"]
    _2 = 2,
    #[doc = "3: Clock source from external clock T1 pin"]
    _3 = 3,
    #[doc = "5: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _5 = 5,
    #[doc = "7: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _7 = 7,
}
impl From<TMR1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR1SEL` reader - TIMER1 Clock Source Selection"]
pub struct TMR1SEL_R(crate::FieldReader<u8, TMR1SEL_A>);
impl TMR1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMR1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR1SEL_A> {
        match self.bits {
            0 => Some(TMR1SEL_A::_0),
            1 => Some(TMR1SEL_A::_1),
            2 => Some(TMR1SEL_A::_2),
            3 => Some(TMR1SEL_A::_3),
            5 => Some(TMR1SEL_A::_5),
            7 => Some(TMR1SEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR1SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TMR1SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TMR1SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TMR1SEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TMR1SEL_A::_7
    }
}
impl core::ops::Deref for TMR1SEL_R {
    type Target = crate::FieldReader<u8, TMR1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1SEL` writer - TIMER1 Clock Source Selection"]
pub struct TMR1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_1)
    }
    #[doc = "Clock source from PCLK0"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_2)
    }
    #[doc = "Clock source from external clock T1 pin"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_3)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_5)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TMR1SEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "TIMER2 Clock Source Selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR2SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK1"]
    _2 = 2,
    #[doc = "3: Clock source from external clock T2 pin"]
    _3 = 3,
    #[doc = "5: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _5 = 5,
    #[doc = "7: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _7 = 7,
}
impl From<TMR2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR2SEL` reader - TIMER2 Clock Source Selection"]
pub struct TMR2SEL_R(crate::FieldReader<u8, TMR2SEL_A>);
impl TMR2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMR2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR2SEL_A> {
        match self.bits {
            0 => Some(TMR2SEL_A::_0),
            1 => Some(TMR2SEL_A::_1),
            2 => Some(TMR2SEL_A::_2),
            3 => Some(TMR2SEL_A::_3),
            5 => Some(TMR2SEL_A::_5),
            7 => Some(TMR2SEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR2SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR2SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TMR2SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TMR2SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TMR2SEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TMR2SEL_A::_7
    }
}
impl core::ops::Deref for TMR2SEL_R {
    type Target = crate::FieldReader<u8, TMR2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2SEL` writer - TIMER2 Clock Source Selection"]
pub struct TMR2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_1)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_2)
    }
    #[doc = "Clock source from external clock T2 pin"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_3)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_5)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TMR2SEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "TIMER3 Clock Source Selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR3SEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from PCLK1"]
    _2 = 2,
    #[doc = "3: Clock source from external clock T3 pin"]
    _3 = 3,
    #[doc = "5: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _5 = 5,
    #[doc = "7: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _7 = 7,
}
impl From<TMR3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR3SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMR3SEL` reader - TIMER3 Clock Source Selection"]
pub struct TMR3SEL_R(crate::FieldReader<u8, TMR3SEL_A>);
impl TMR3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMR3SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR3SEL_A> {
        match self.bits {
            0 => Some(TMR3SEL_A::_0),
            1 => Some(TMR3SEL_A::_1),
            2 => Some(TMR3SEL_A::_2),
            3 => Some(TMR3SEL_A::_3),
            5 => Some(TMR3SEL_A::_5),
            7 => Some(TMR3SEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR3SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR3SEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TMR3SEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TMR3SEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TMR3SEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TMR3SEL_A::_7
    }
}
impl core::ops::Deref for TMR3SEL_R {
    type Target = crate::FieldReader<u8, TMR3SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR3SEL` writer - TIMER3 Clock Source Selection"]
pub struct TMR3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_1)
    }
    #[doc = "Clock source from PCLK1"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_2)
    }
    #[doc = "Clock source from external clock T3 pin"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_3)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_5)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TMR3SEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "UART Clock Source Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UARTSEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from PLL"]
    _1 = 1,
    #[doc = "2: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<UARTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UARTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UARTSEL` reader - UART Clock Source Selection"]
pub struct UARTSEL_R(crate::FieldReader<u8, UARTSEL_A>);
impl UARTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UARTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTSEL_A {
        match self.bits {
            0 => UARTSEL_A::_0,
            1 => UARTSEL_A::_1,
            2 => UARTSEL_A::_2,
            3 => UARTSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UARTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UARTSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == UARTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == UARTSEL_A::_3
    }
}
impl core::ops::Deref for UARTSEL_R {
    type Target = crate::FieldReader<u8, UARTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTSEL` writer - UART Clock Source Selection"]
pub struct UARTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSEL_A::_0)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSEL_A::_1)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(UARTSEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(UARTSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Clock Divider Clock Source Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOSEL_A {
    #[doc = "0: Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    _1 = 1,
    #[doc = "2: Clock source from HCLK"]
    _2 = 2,
    #[doc = "3: Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    _3 = 3,
}
impl From<CLKOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOSEL` reader - Clock Divider Clock Source Selection"]
pub struct CLKOSEL_R(crate::FieldReader<u8, CLKOSEL_A>);
impl CLKOSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOSEL_A {
        match self.bits {
            0 => CLKOSEL_A::_0,
            1 => CLKOSEL_A::_1,
            2 => CLKOSEL_A::_2,
            3 => CLKOSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKOSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKOSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CLKOSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CLKOSEL_A::_3
    }
}
impl core::ops::Deref for CLKOSEL_R {
    type Target = crate::FieldReader<u8, CLKOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOSEL` writer - Clock Divider Clock Source Selection"]
pub struct CLKOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock source from 4~20 MHz external high speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOSEL_A::_0)
    }
    #[doc = "Clock source from 32.768 kHz external low speed crystal oscillator (LXT)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOSEL_A::_1)
    }
    #[doc = "Clock source from HCLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CLKOSEL_A::_2)
    }
    #[doc = "Clock source from 22.1184 MHz internal high speed RC oscillator (HIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CLKOSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Window Watchdog Timer Clock Source Selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WWDTSEL_A {
    #[doc = "2: Clock source from HCLK/2048"]
    _2 = 2,
    #[doc = "3: Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    _3 = 3,
}
impl From<WWDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WWDTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WWDTSEL` reader - Window Watchdog Timer Clock Source Selection"]
pub struct WWDTSEL_R(crate::FieldReader<u8, WWDTSEL_A>);
impl WWDTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WWDTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WWDTSEL_A> {
        match self.bits {
            2 => Some(WWDTSEL_A::_2),
            3 => Some(WWDTSEL_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == WWDTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == WWDTSEL_A::_3
    }
}
impl core::ops::Deref for WWDTSEL_R {
    type Target = crate::FieldReader<u8, WWDTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDTSEL` writer - Window Watchdog Timer Clock Source Selection"]
pub struct WWDTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from HCLK/2048"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WWDTSEL_A::_2)
    }
    #[doc = "Clock source from 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WWDTSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - TIMER0 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr0sel(&self) -> TMR0SEL_R {
        TMR0SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - TIMER1 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr1sel(&self) -> TMR1SEL_R {
        TMR1SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - TIMER2 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr2sel(&self) -> TMR2SEL_R {
        TMR2SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - TIMER3 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr3sel(&self) -> TMR3SEL_R {
        TMR3SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - UART Clock Source Selection"]
    #[inline(always)]
    pub fn uartsel(&self) -> UARTSEL_R {
        UARTSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Clock Divider Clock Source Selection"]
    #[inline(always)]
    pub fn clkosel(&self) -> CLKOSEL_R {
        CLKOSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Window Watchdog Timer Clock Source Selection"]
    #[inline(always)]
    pub fn wwdtsel(&self) -> WWDTSEL_R {
        WWDTSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Timer Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W {
        WDTSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - TIMER0 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr0sel(&mut self) -> TMR0SEL_W {
        TMR0SEL_W { w: self }
    }
    #[doc = "Bits 12:14 - TIMER1 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr1sel(&mut self) -> TMR1SEL_W {
        TMR1SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - TIMER2 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr2sel(&mut self) -> TMR2SEL_W {
        TMR2SEL_W { w: self }
    }
    #[doc = "Bits 20:22 - TIMER3 Clock Source Selection"]
    #[inline(always)]
    pub fn tmr3sel(&mut self) -> TMR3SEL_W {
        TMR3SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - UART Clock Source Selection"]
    #[inline(always)]
    pub fn uartsel(&mut self) -> UARTSEL_W {
        UARTSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - Clock Divider Clock Source Selection"]
    #[inline(always)]
    pub fn clkosel(&mut self) -> CLKOSEL_W {
        CLKOSEL_W { w: self }
    }
    #[doc = "Bits 30:31 - Window Watchdog Timer Clock Source Selection"]
    #[inline(always)]
    pub fn wwdtsel(&mut self) -> WWDTSEL_W {
        WWDTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel1](index.html) module"]
pub struct CLK_CLKSEL1_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel1::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel1::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL1 to value 0xb377_770f"]
impl crate::Resettable for CLK_CLKSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb377_770f
    }
}
