#[doc = "Register `CLK_PWRCTL` reader"]
pub struct R(crate::R<CLK_PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_PWRCTL_SPEC>> for R {
    fn from(reader: crate::R<CLK_PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PWRCTL` writer"]
pub struct W(crate::W<CLK_PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PWRCTL_SPEC>;
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
impl core::convert::From<crate::W<CLK_PWRCTL_SPEC>> for W {
    fn from(writer: crate::W<CLK_PWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HXT Enable Bit (Write Protect)\\nThe bit default value is set by flash controller user configuration register CONFIG0 \\[26:24\\]. When the default clock source is from HXT, this bit is set to 1 automatically.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTEN_A {
    #[doc = "0: 4~20 MHz xxternal high speed crystal (HXT) Disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal (HXT) Enabled"]
    _1 = 1,
}
impl From<HXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTEN` reader - HXT Enable Bit (Write Protect)\\nThe bit default value is set by flash controller user configuration register CONFIG0 \\[26:24\\]. When the default clock source is from HXT, this bit is set to 1 automatically.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTEN_R(crate::FieldReader<bool, HXTEN_A>);
impl HXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTEN_A {
        match self.bits {
            false => HXTEN_A::_0,
            true => HXTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTEN_A::_1
    }
}
impl core::ops::Deref for HXTEN_R {
    type Target = crate::FieldReader<bool, HXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTEN` writer - HXT Enable Bit (Write Protect)\\nThe bit default value is set by flash controller user configuration register CONFIG0 \\[26:24\\]. When the default clock source is from HXT, this bit is set to 1 automatically.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz xxternal high speed crystal (HXT) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTEN_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal (HXT) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTEN_A::_1)
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
#[doc = "LXT Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTEN_A {
    #[doc = "0: 32.768 kHz external low speed crystal (LXT) Disabled"]
    _0 = 0,
    #[doc = "1: 32.768 kHz external low speed crystal (LXT) Enabled"]
    _1 = 1,
}
impl From<LXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTEN` reader - LXT Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTEN_R(crate::FieldReader<bool, LXTEN_A>);
impl LXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTEN_A {
        match self.bits {
            false => LXTEN_A::_0,
            true => LXTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTEN_A::_1
    }
}
impl core::ops::Deref for LXTEN_R {
    type Target = crate::FieldReader<bool, LXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTEN` writer - LXT Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32.768 kHz external low speed crystal (LXT) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTEN_A::_0)
    }
    #[doc = "32.768 kHz external low speed crystal (LXT) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTEN_A::_1)
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
#[doc = "HIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRCEN_A {
    #[doc = "0: 22.1184 MHz internal high speed RC oscillator (HIRC) Disabled"]
    _0 = 0,
    #[doc = "1: 22.1184 MHz internal high speed RC oscillator (HIRC) Enabled"]
    _1 = 1,
}
impl From<HIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRCEN` reader - HIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HIRCEN_R(crate::FieldReader<bool, HIRCEN_A>);
impl HIRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRCEN_A {
        match self.bits {
            false => HIRCEN_A::_0,
            true => HIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HIRCEN_A::_1
    }
}
impl core::ops::Deref for HIRCEN_R {
    type Target = crate::FieldReader<bool, HIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRCEN` writer - HIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "22.1184 MHz internal high speed RC oscillator (HIRC) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HIRCEN_A::_0)
    }
    #[doc = "22.1184 MHz internal high speed RC oscillator (HIRC) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HIRCEN_A::_1)
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
#[doc = "LIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIRCEN_A {
    #[doc = "0: 10 kHz internal low speed RC oscillator (LIRC) Disabled"]
    _0 = 0,
    #[doc = "1: 10 kHz internal low speed RC oscillator (LIRC) Enabled"]
    _1 = 1,
}
impl From<LIRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LIRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIRCEN` reader - LIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LIRCEN_R(crate::FieldReader<bool, LIRCEN_A>);
impl LIRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIRCEN_A {
        match self.bits {
            false => LIRCEN_A::_0,
            true => LIRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LIRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LIRCEN_A::_1
    }
}
impl core::ops::Deref for LIRCEN_R {
    type Target = crate::FieldReader<bool, LIRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIRCEN` writer - LIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LIRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "10 kHz internal low speed RC oscillator (LIRC) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LIRCEN_A::_0)
    }
    #[doc = "10 kHz internal low speed RC oscillator (LIRC) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LIRCEN_A::_1)
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
#[doc = "Enable the Wake-up Delay Counter (Write Protect)\\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\\nThe delayed clock cycle is 4096 clock cycles when chip works at 4~20 MHz external high speed crystal oscillator (HXT), and 256 clock cycles when chip works at 22.1184 MHz internal high speed RC oscillator (HIRC).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWKDLY_A {
    #[doc = "0: Clock cycles delay Disabled"]
    _0 = 0,
    #[doc = "1: Clock cycles delay Enabled"]
    _1 = 1,
}
impl From<PDWKDLY_A> for bool {
    #[inline(always)]
    fn from(variant: PDWKDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWKDLY` reader - Enable the Wake-up Delay Counter (Write Protect)\\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\\nThe delayed clock cycle is 4096 clock cycles when chip works at 4~20 MHz external high speed crystal oscillator (HXT), and 256 clock cycles when chip works at 22.1184 MHz internal high speed RC oscillator (HIRC).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKDLY_R(crate::FieldReader<bool, PDWKDLY_A>);
impl PDWKDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWKDLY_A {
        match self.bits {
            false => PDWKDLY_A::_0,
            true => PDWKDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDWKDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDWKDLY_A::_1
    }
}
impl core::ops::Deref for PDWKDLY_R {
    type Target = crate::FieldReader<bool, PDWKDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKDLY` writer - Enable the Wake-up Delay Counter (Write Protect)\\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\\nThe delayed clock cycle is 4096 clock cycles when chip works at 4~20 MHz external high speed crystal oscillator (HXT), and 256 clock cycles when chip works at 22.1184 MHz internal high speed RC oscillator (HIRC).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWKDLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock cycles delay Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDWKDLY_A::_0)
    }
    #[doc = "Clock cycles delay Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDWKDLY_A::_1)
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
#[doc = "Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\\nNote1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWKIEN_A {
    #[doc = "0: Power-down mode wake-up interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Power-down mode wake-up interrupt Enabled"]
    _1 = 1,
}
impl From<PDWKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWKIEN` reader - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\\nNote1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKIEN_R(crate::FieldReader<bool, PDWKIEN_A>);
impl PDWKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWKIEN_A {
        match self.bits {
            false => PDWKIEN_A::_0,
            true => PDWKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDWKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDWKIEN_A::_1
    }
}
impl core::ops::Deref for PDWKIEN_R {
    type Target = crate::FieldReader<bool, PDWKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKIEN` writer - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\\nNote1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power-down mode wake-up interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDWKIEN_A::_0)
    }
    #[doc = "Power-down mode wake-up interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDWKIEN_A::_1)
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
#[doc = "Field `PDWKIF` reader - Power-down Mode Wake-up Interrupt Status\\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \\nThe flag is set if the EINT0~5, GPIO, USBH, USBD, OTG, UART0~3, WDT, CAN0, ACMP01, BOD, RTC, TMR0~3, I2C0~1.\\nNote1: Write 1 to clear the bit to 0.\\nNote2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
pub struct PDWKIF_R(crate::FieldReader<bool, bool>);
impl PDWKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWKIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDWKIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWKIF` writer - Power-down Mode Wake-up Interrupt Status\\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \\nThe flag is set if the EINT0~5, GPIO, USBH, USBD, OTG, UART0~3, WDT, CAN0, ACMP01, BOD, RTC, TMR0~3, I2C0~1.\\nNote1: Write 1 to clear the bit to 0.\\nNote2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
pub struct PDWKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWKIF_W<'a> {
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
#[doc = "System Power-down Enable (Write Protect)\\nWhen this bit is set to 1, Power-down mode is enabled and chip Power-down behavior will depend on the PDWTCPU bit.\\n(a) If the PDWTCPU is 0, then the chip enters Power-down mode immediately after the PDEN bit set. (default)\\n(b) if the PDWTCPU is 1, then the chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode.\\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_A {
    #[doc = "0: Chip operating normally or chip in idle mode because of WFI command"]
    _0 = 0,
    #[doc = "1: Chip enters Power-down mode instant or wait CPU sleep command WFI"]
    _1 = 1,
}
impl From<PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN` reader - System Power-down Enable (Write Protect)\\nWhen this bit is set to 1, Power-down mode is enabled and chip Power-down behavior will depend on the PDWTCPU bit.\\n(a) If the PDWTCPU is 0, then the chip enters Power-down mode immediately after the PDEN bit set. (default)\\n(b) if the PDWTCPU is 1, then the chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode.\\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDEN_R(crate::FieldReader<bool, PDEN_A>);
impl PDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_A {
        match self.bits {
            false => PDEN_A::_0,
            true => PDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDEN_A::_1
    }
}
impl core::ops::Deref for PDEN_R {
    type Target = crate::FieldReader<bool, PDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN` writer - System Power-down Enable (Write Protect)\\nWhen this bit is set to 1, Power-down mode is enabled and chip Power-down behavior will depend on the PDWTCPU bit.\\n(a) If the PDWTCPU is 0, then the chip enters Power-down mode immediately after the PDEN bit set. (default)\\n(b) if the PDWTCPU is 1, then the chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode.\\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip operating normally or chip in idle mode because of WFI command"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDEN_A::_0)
    }
    #[doc = "Chip enters Power-down mode instant or wait CPU sleep command WFI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDEN_A::_1)
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
#[doc = "this Bit Control the Power-down Entry Condition (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWTCPU_A {
    #[doc = "0: Chip enters Power-down mode when the PDEN bit is set to 1"]
    _0 = 0,
    #[doc = "1: Chip enters Power-down mode when the both PDWTCPU and PDEN bits are set to 1 and CPU runs WFI instruction"]
    _1 = 1,
}
impl From<PDWTCPU_A> for bool {
    #[inline(always)]
    fn from(variant: PDWTCPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWTCPU` reader - this Bit Control the Power-down Entry Condition (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWTCPU_R(crate::FieldReader<bool, PDWTCPU_A>);
impl PDWTCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWTCPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWTCPU_A {
        match self.bits {
            false => PDWTCPU_A::_0,
            true => PDWTCPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDWTCPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDWTCPU_A::_1
    }
}
impl core::ops::Deref for PDWTCPU_R {
    type Target = crate::FieldReader<bool, PDWTCPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWTCPU` writer - this Bit Control the Power-down Entry Condition (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PDWTCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWTCPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWTCPU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chip enters Power-down mode when the PDEN bit is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDWTCPU_A::_0)
    }
    #[doc = "Chip enters Power-down mode when the both PDWTCPU and PDEN bits are set to 1 and CPU runs WFI instruction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDWTCPU_A::_1)
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
#[doc = "HXT Gain Control Bit (Write Protect)\\nThis is a protected register. Please refer to open lock sequence to program it.\\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HXTGAIN_A {
    #[doc = "0: HXT frequency is lower than from 8 MHz"]
    _0 = 0,
    #[doc = "1: HXT frequency is from 8 MHz to 12 MHz"]
    _1 = 1,
    #[doc = "2: HXT frequency is from 12 MHz to 16 MHz"]
    _2 = 2,
    #[doc = "3: HXT frequency is higher than 16 MHz"]
    _3 = 3,
}
impl From<HXTGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: HXTGAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HXTGAIN` reader - HXT Gain Control Bit (Write Protect)\\nThis is a protected register. Please refer to open lock sequence to program it.\\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTGAIN_R(crate::FieldReader<u8, HXTGAIN_A>);
impl HXTGAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HXTGAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTGAIN_A {
        match self.bits {
            0 => HXTGAIN_A::_0,
            1 => HXTGAIN_A::_1,
            2 => HXTGAIN_A::_2,
            3 => HXTGAIN_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTGAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTGAIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HXTGAIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HXTGAIN_A::_3
    }
}
impl core::ops::Deref for HXTGAIN_R {
    type Target = crate::FieldReader<u8, HXTGAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTGAIN` writer - HXT Gain Control Bit (Write Protect)\\nThis is a protected register. Please refer to open lock sequence to program it.\\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HXTGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTGAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTGAIN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HXT frequency is lower than from 8 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_0)
    }
    #[doc = "HXT frequency is from 8 MHz to 12 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_1)
    }
    #[doc = "HXT frequency is from 12 MHz to 16 MHz"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_2)
    }
    #[doc = "HXT frequency is higher than 16 MHz"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HXTGAIN_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HXT Enable Bit (Write Protect)\\nThe bit default value is set by flash controller user configuration register CONFIG0 \\[26:24\\]. When the default clock source is from HXT, this bit is set to 1 automatically.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxten(&self) -> HXTEN_R {
        HXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LXT Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxten(&self) -> LXTEN_R {
        LXTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hircen(&self) -> HIRCEN_R {
        HIRCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lircen(&self) -> LIRCEN_R {
        LIRCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable the Wake-up Delay Counter (Write Protect)\\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\\nThe delayed clock cycle is 4096 clock cycles when chip works at 4~20 MHz external high speed crystal oscillator (HXT), and 256 clock cycles when chip works at 22.1184 MHz internal high speed RC oscillator (HIRC).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkdly(&self) -> PDWKDLY_R {
        PDWKDLY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\\nNote1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkien(&self) -> PDWKIEN_R {
        PDWKIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power-down Mode Wake-up Interrupt Status\\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \\nThe flag is set if the EINT0~5, GPIO, USBH, USBD, OTG, UART0~3, WDT, CAN0, ACMP01, BOD, RTC, TMR0~3, I2C0~1.\\nNote1: Write 1 to clear the bit to 0.\\nNote2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pdwkif(&self) -> PDWKIF_R {
        PDWKIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System Power-down Enable (Write Protect)\\nWhen this bit is set to 1, Power-down mode is enabled and chip Power-down behavior will depend on the PDWTCPU bit.\\n(a) If the PDWTCPU is 0, then the chip enters Power-down mode immediately after the PDEN bit set. (default)\\n(b) if the PDWTCPU is 1, then the chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode.\\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - this Bit Control the Power-down Entry Condition (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwtcpu(&self) -> PDWTCPU_R {
        PDWTCPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - HXT Gain Control Bit (Write Protect)\\nThis is a protected register. Please refer to open lock sequence to program it.\\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxtgain(&self) -> HXTGAIN_R {
        HXTGAIN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HXT Enable Bit (Write Protect)\\nThe bit default value is set by flash controller user configuration register CONFIG0 \\[26:24\\]. When the default clock source is from HXT, this bit is set to 1 automatically.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxten(&mut self) -> HXTEN_W {
        HXTEN_W { w: self }
    }
    #[doc = "Bit 1 - LXT Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lxten(&mut self) -> LXTEN_W {
        LXTEN_W { w: self }
    }
    #[doc = "Bit 2 - HIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hircen(&mut self) -> HIRCEN_W {
        HIRCEN_W { w: self }
    }
    #[doc = "Bit 3 - LIRC Enable Bit (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lircen(&mut self) -> LIRCEN_W {
        LIRCEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the Wake-up Delay Counter (Write Protect)\\nWhen the chip wakes up from Power-down mode, the clock control will delay certain clock cycles to wait system clock stable.\\nThe delayed clock cycle is 4096 clock cycles when chip works at 4~20 MHz external high speed crystal oscillator (HXT), and 256 clock cycles when chip works at 22.1184 MHz internal high speed RC oscillator (HIRC).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkdly(&mut self) -> PDWKDLY_W {
        PDWKDLY_W { w: self }
    }
    #[doc = "Bit 5 - Power-down Mode Wake-up Interrupt Enable Bit (Write Protect)\\nNote1: The interrupt will occur when both PDWKIF and PDWKIEN are high.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwkien(&mut self) -> PDWKIEN_W {
        PDWKIEN_W { w: self }
    }
    #[doc = "Bit 6 - Power-down Mode Wake-up Interrupt Status\\nSet by 'Power-down wake-up event', it indicates that resume from Power-down mode' \\nThe flag is set if the EINT0~5, GPIO, USBH, USBD, OTG, UART0~3, WDT, CAN0, ACMP01, BOD, RTC, TMR0~3, I2C0~1.\\nNote1: Write 1 to clear the bit to 0.\\nNote2: This bit works only if PDWKIEN (CLK_PWRCTL\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pdwkif(&mut self) -> PDWKIF_W {
        PDWKIF_W { w: self }
    }
    #[doc = "Bit 7 - System Power-down Enable (Write Protect)\\nWhen this bit is set to 1, Power-down mode is enabled and chip Power-down behavior will depend on the PDWTCPU bit.\\n(a) If the PDWTCPU is 0, then the chip enters Power-down mode immediately after the PDEN bit set. (default)\\n(b) if the PDWTCPU is 1, then the chip keeps active till the CPU sleep mode is also active and then the chip enters Power-down mode.\\nWhen chip wakes up from Power-down mode, this bit is auto cleared. Users need to set this bit again for next Power-down.\\nIn Power-down mode, HXT and the HIRC will be disabled in this mode, but LXT and LIRC are not controlled by Power-down mode.\\nIn Power-down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by Power-down mode, if the peripheral clock source is from LXT or LIRC.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 8 - this Bit Control the Power-down Entry Condition (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pdwtcpu(&mut self) -> PDWTCPU_W {
        PDWTCPU_W { w: self }
    }
    #[doc = "Bits 10:11 - HXT Gain Control Bit (Write Protect)\\nThis is a protected register. Please refer to open lock sequence to program it.\\nGain control is used to enlarge the gain of crystal to make sure crystal work normally. If gain control is enabled, crystal will consume more power than gain control off. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hxtgain(&mut self) -> HXTGAIN_W {
        HXTGAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Power-down Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pwrctl](index.html) module"]
pub struct CLK_PWRCTL_SPEC;
impl crate::RegisterSpec for CLK_PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pwrctl::R](R) reader structure"]
impl crate::Readable for CLK_PWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pwrctl::W](W) writer structure"]
impl crate::Writable for CLK_PWRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PWRCTL to value 0x10"]
impl crate::Resettable for CLK_PWRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
