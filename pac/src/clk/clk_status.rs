#[doc = "Register `CLK_STATUS` reader"]
pub struct R(crate::R<CLK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_STATUS_SPEC>> for R {
    fn from(reader: crate::R<CLK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HXT Clock Source Stable Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTSTB_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock is not stable or disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock is stable and enabled"]
    _1 = 1,
}
impl From<HXTSTB_A> for bool {
    #[inline(always)]
    fn from(variant: HXTSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTSTB` reader - HXT Clock Source Stable Flag (Read Only)"]
pub struct HXTSTB_R(crate::FieldReader<bool, HXTSTB_A>);
impl HXTSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTSTB_A {
        match self.bits {
            false => HXTSTB_A::_0,
            true => HXTSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTSTB_A::_1
    }
}
impl core::ops::Deref for HXTSTB_R {
    type Target = crate::FieldReader<bool, HXTSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LXT Clock Source Stable Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTSTB_A {
    #[doc = "0: 32.768 kHz external low speed crystal oscillator (LXT) clock is not stable or disabled"]
    _0 = 0,
    #[doc = "1: 32.768 kHz external low speed crystal oscillator (LXT) clock is stabled and enabled"]
    _1 = 1,
}
impl From<LXTSTB_A> for bool {
    #[inline(always)]
    fn from(variant: LXTSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTSTB` reader - LXT Clock Source Stable Flag (Read Only)"]
pub struct LXTSTB_R(crate::FieldReader<bool, LXTSTB_A>);
impl LXTSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTSTB_A {
        match self.bits {
            false => LXTSTB_A::_0,
            true => LXTSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTSTB_A::_1
    }
}
impl core::ops::Deref for LXTSTB_R {
    type Target = crate::FieldReader<bool, LXTSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal PLL Clock Source Stable Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTB_A {
    #[doc = "0: Internal PLL clock is not stable or disabled"]
    _0 = 0,
    #[doc = "1: Internal PLL clock is stable and enabled"]
    _1 = 1,
}
impl From<PLLSTB_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTB` reader - Internal PLL Clock Source Stable Flag (Read Only)"]
pub struct PLLSTB_R(crate::FieldReader<bool, PLLSTB_A>);
impl PLLSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTB_A {
        match self.bits {
            false => PLLSTB_A::_0,
            true => PLLSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLLSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLLSTB_A::_1
    }
}
impl core::ops::Deref for PLLSTB_R {
    type Target = crate::FieldReader<bool, PLLSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIRC Clock Source Stable Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIRCSTB_A {
    #[doc = "0: 10 kHz internal low speed RC oscillator (LIRC) clock is not stable or disabled"]
    _0 = 0,
    #[doc = "1: 10 kHz internal low speed RC oscillator (LIRC) clock is stable and enabled"]
    _1 = 1,
}
impl From<LIRCSTB_A> for bool {
    #[inline(always)]
    fn from(variant: LIRCSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIRCSTB` reader - LIRC Clock Source Stable Flag (Read Only)"]
pub struct LIRCSTB_R(crate::FieldReader<bool, LIRCSTB_A>);
impl LIRCSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIRCSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIRCSTB_A {
        match self.bits {
            false => LIRCSTB_A::_0,
            true => LIRCSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LIRCSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LIRCSTB_A::_1
    }
}
impl core::ops::Deref for LIRCSTB_R {
    type Target = crate::FieldReader<bool, LIRCSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HIRC Clock Source Stable Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRCSTB_A {
    #[doc = "0: 22.1184 MHz internal high speed RC oscillator (HIRC) clock is not stable or disabled"]
    _0 = 0,
    #[doc = "1: 22.1184 MHz internal high speed RC oscillator (HIRC) clock is stabe and enabled"]
    _1 = 1,
}
impl From<HIRCSTB_A> for bool {
    #[inline(always)]
    fn from(variant: HIRCSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRCSTB` reader - HIRC Clock Source Stable Flag (Read Only)"]
pub struct HIRCSTB_R(crate::FieldReader<bool, HIRCSTB_A>);
impl HIRCSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIRCSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRCSTB_A {
        match self.bits {
            false => HIRCSTB_A::_0,
            true => HIRCSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HIRCSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HIRCSTB_A::_1
    }
}
impl core::ops::Deref for HIRCSTB_R {
    type Target = crate::FieldReader<bool, HIRCSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clock Switching Fail Flag (Read Only) \\nThis bit is updated when software switches system clock source. If switch target clock is stable, this bit will be set to 0. If switch target clock is not stable, this bit will be set to 1.\\nNote: Write 1 to clear the bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSFAIL_A {
    #[doc = "0: Clock switching success"]
    _0 = 0,
    #[doc = "1: Clock switching failure"]
    _1 = 1,
}
impl From<CLKSFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSFAIL` reader - Clock Switching Fail Flag (Read Only) \\nThis bit is updated when software switches system clock source. If switch target clock is stable, this bit will be set to 0. If switch target clock is not stable, this bit will be set to 1.\\nNote: Write 1 to clear the bit to 0."]
pub struct CLKSFAIL_R(crate::FieldReader<bool, CLKSFAIL_A>);
impl CLKSFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSFAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSFAIL_A {
        match self.bits {
            false => CLKSFAIL_A::_0,
            true => CLKSFAIL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKSFAIL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKSFAIL_A::_1
    }
}
impl core::ops::Deref for CLKSFAIL_R {
    type Target = crate::FieldReader<bool, CLKSFAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HXT Clock Source Stable Flag (Read Only)"]
    #[inline(always)]
    pub fn hxtstb(&self) -> HXTSTB_R {
        HXTSTB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LXT Clock Source Stable Flag (Read Only)"]
    #[inline(always)]
    pub fn lxtstb(&self) -> LXTSTB_R {
        LXTSTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal PLL Clock Source Stable Flag (Read Only)"]
    #[inline(always)]
    pub fn pllstb(&self) -> PLLSTB_R {
        PLLSTB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LIRC Clock Source Stable Flag (Read Only)"]
    #[inline(always)]
    pub fn lircstb(&self) -> LIRCSTB_R {
        LIRCSTB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HIRC Clock Source Stable Flag (Read Only)"]
    #[inline(always)]
    pub fn hircstb(&self) -> HIRCSTB_R {
        HIRCSTB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock Switching Fail Flag (Read Only) \\nThis bit is updated when software switches system clock source. If switch target clock is stable, this bit will be set to 0. If switch target clock is not stable, this bit will be set to 1.\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn clksfail(&self) -> CLKSFAIL_R {
        CLKSFAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Clock Status Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_status](index.html) module"]
pub struct CLK_STATUS_SPEC;
impl crate::RegisterSpec for CLK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_status::R](R) reader structure"]
impl crate::Readable for CLK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_STATUS to value 0"]
impl crate::Resettable for CLK_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
