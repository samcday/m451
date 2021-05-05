#[doc = "Register `UART_ALTCTL` reader"]
pub struct R(crate::R<UART_ALTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ALTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_ALTCTL_SPEC>> for R {
    fn from(reader: crate::R<UART_ALTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_ALTCTL` writer"]
pub struct W(crate::W<UART_ALTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ALTCTL_SPEC>;
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
impl core::convert::From<crate::W<UART_ALTCTL_SPEC>> for W {
    fn from(writer: crate::W<UART_ALTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRKFL` reader - UART LIN Break Field Length (Only Available in UART0/UART1 Channel)\\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: This break field length is BRKFL + 1"]
pub struct BRKFL_R(crate::FieldReader<u8, u8>);
impl BRKFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRKFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRKFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKFL` writer - UART LIN Break Field Length (Only Available in UART0/UART1 Channel)\\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: This break field length is BRKFL + 1"]
pub struct BRKFL_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "LIN RX Enable Bit (Only Available in UART0/UART1 Channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINRXEN_A {
    #[doc = "0: LIN RX mode Disabled"]
    _0 = 0,
    #[doc = "1: LIN RX mode Enabled"]
    _1 = 1,
}
impl From<LINRXEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINRXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINRXEN` reader - LIN RX Enable Bit (Only Available in UART0/UART1 Channel)"]
pub struct LINRXEN_R(crate::FieldReader<bool, LINRXEN_A>);
impl LINRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINRXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINRXEN_A {
        match self.bits {
            false => LINRXEN_A::_0,
            true => LINRXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINRXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINRXEN_A::_1
    }
}
impl core::ops::Deref for LINRXEN_R {
    type Target = crate::FieldReader<bool, LINRXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINRXEN` writer - LIN RX Enable Bit (Only Available in UART0/UART1 Channel)"]
pub struct LINRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINRXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINRXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN RX mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINRXEN_A::_0)
    }
    #[doc = "LIN RX mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINRXEN_A::_1)
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
#[doc = "LIN TX Break Mode Enable Bit (Only Available in UART0/UART1 Channel)\\nNote: When TX break field transfer operation finished, this bit will be cleared automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINTXEN_A {
    #[doc = "0: LIN TX Break mode Disabled"]
    _0 = 0,
    #[doc = "1: LIN TX Break mode Enabled"]
    _1 = 1,
}
impl From<LINTXEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINTXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINTXEN` reader - LIN TX Break Mode Enable Bit (Only Available in UART0/UART1 Channel)\\nNote: When TX break field transfer operation finished, this bit will be cleared automatically."]
pub struct LINTXEN_R(crate::FieldReader<bool, LINTXEN_A>);
impl LINTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINTXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINTXEN_A {
        match self.bits {
            false => LINTXEN_A::_0,
            true => LINTXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINTXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINTXEN_A::_1
    }
}
impl core::ops::Deref for LINTXEN_R {
    type Target = crate::FieldReader<bool, LINTXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINTXEN` writer - LIN TX Break Mode Enable Bit (Only Available in UART0/UART1 Channel)\\nNote: When TX break field transfer operation finished, this bit will be cleared automatically."]
pub struct LINTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINTXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINTXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN TX Break mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINTXEN_A::_0)
    }
    #[doc = "LIN TX Break mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINTXEN_A::_1)
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
#[doc = "RS-485 Normal Multi-drop Operation Mode (NMM) \\nNote: It cannot be active with RS-485_AAD operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS485NMM_A {
    #[doc = "0: RS-485 Normal Multi-drop Operation mode (NMM) Disabled"]
    _0 = 0,
    #[doc = "1: RS-485 Normal Multi-drop Operation mode (NMM) Enabled"]
    _1 = 1,
}
impl From<RS485NMM_A> for bool {
    #[inline(always)]
    fn from(variant: RS485NMM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS485NMM` reader - RS-485 Normal Multi-drop Operation Mode (NMM) \\nNote: It cannot be active with RS-485_AAD operation mode."]
pub struct RS485NMM_R(crate::FieldReader<bool, RS485NMM_A>);
impl RS485NMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485NMM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS485NMM_A {
        match self.bits {
            false => RS485NMM_A::_0,
            true => RS485NMM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RS485NMM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RS485NMM_A::_1
    }
}
impl core::ops::Deref for RS485NMM_R {
    type Target = crate::FieldReader<bool, RS485NMM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485NMM` writer - RS-485 Normal Multi-drop Operation Mode (NMM) \\nNote: It cannot be active with RS-485_AAD operation mode."]
pub struct RS485NMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485NMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS485NMM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RS-485 Normal Multi-drop Operation mode (NMM) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RS485NMM_A::_0)
    }
    #[doc = "RS-485 Normal Multi-drop Operation mode (NMM) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RS485NMM_A::_1)
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
#[doc = "RS-485 Auto Address Detection Operation Mode (AAD)\\nNote: It cannot be active with RS-485_NMM operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS485AAD_A {
    #[doc = "0: RS-485 Auto Address Detection Operation mode (AAD) Disabled"]
    _0 = 0,
    #[doc = "1: RS-485 Auto Address Detection Operation mode (AAD) Enabled"]
    _1 = 1,
}
impl From<RS485AAD_A> for bool {
    #[inline(always)]
    fn from(variant: RS485AAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS485AAD` reader - RS-485 Auto Address Detection Operation Mode (AAD)\\nNote: It cannot be active with RS-485_NMM operation mode."]
pub struct RS485AAD_R(crate::FieldReader<bool, RS485AAD_A>);
impl RS485AAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485AAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS485AAD_A {
        match self.bits {
            false => RS485AAD_A::_0,
            true => RS485AAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RS485AAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RS485AAD_A::_1
    }
}
impl core::ops::Deref for RS485AAD_R {
    type Target = crate::FieldReader<bool, RS485AAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485AAD` writer - RS-485 Auto Address Detection Operation Mode (AAD)\\nNote: It cannot be active with RS-485_NMM operation mode."]
pub struct RS485AAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485AAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS485AAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RS-485 Auto Address Detection Operation mode (AAD) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RS485AAD_A::_0)
    }
    #[doc = "RS-485 Auto Address Detection Operation mode (AAD) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RS485AAD_A::_1)
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
#[doc = "RS-485 Auto Direction Function (AUD) \\nNote: It can be active with RS-485_AAD or RS-485_NMM operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS485AUD_A {
    #[doc = "0: RS-485 Auto Direction Operation function (AUD) Disabled"]
    _0 = 0,
    #[doc = "1: RS-485 Auto Direction Operation function (AUD) Enabled"]
    _1 = 1,
}
impl From<RS485AUD_A> for bool {
    #[inline(always)]
    fn from(variant: RS485AUD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS485AUD` reader - RS-485 Auto Direction Function (AUD) \\nNote: It can be active with RS-485_AAD or RS-485_NMM operation mode."]
pub struct RS485AUD_R(crate::FieldReader<bool, RS485AUD_A>);
impl RS485AUD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485AUD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS485AUD_A {
        match self.bits {
            false => RS485AUD_A::_0,
            true => RS485AUD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RS485AUD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RS485AUD_A::_1
    }
}
impl core::ops::Deref for RS485AUD_R {
    type Target = crate::FieldReader<bool, RS485AUD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485AUD` writer - RS-485 Auto Direction Function (AUD) \\nNote: It can be active with RS-485_AAD or RS-485_NMM operation mode."]
pub struct RS485AUD_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485AUD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS485AUD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RS-485 Auto Direction Operation function (AUD) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RS485AUD_A::_0)
    }
    #[doc = "RS-485 Auto Direction Operation function (AUD) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RS485AUD_A::_1)
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
#[doc = "RS-485 Address Detection Enable Bit\\nThis bit is used to enable RS-485 Address Detection mode. \\nNote: This bit is used for RS-485 any operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDEN_A {
    #[doc = "0: Address detection mode Disabled"]
    _0 = 0,
    #[doc = "1: Address detection mode Enabled"]
    _1 = 1,
}
impl From<ADDRDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRDEN` reader - RS-485 Address Detection Enable Bit\\nThis bit is used to enable RS-485 Address Detection mode. \\nNote: This bit is used for RS-485 any operation mode."]
pub struct ADDRDEN_R(crate::FieldReader<bool, ADDRDEN_A>);
impl ADDRDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRDEN_A {
        match self.bits {
            false => ADDRDEN_A::_0,
            true => ADDRDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDRDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDRDEN_A::_1
    }
}
impl core::ops::Deref for ADDRDEN_R {
    type Target = crate::FieldReader<bool, ADDRDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRDEN` writer - RS-485 Address Detection Enable Bit\\nThis bit is used to enable RS-485 Address Detection mode. \\nNote: This bit is used for RS-485 any operation mode."]
pub struct ADDRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address detection mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDRDEN_A::_0)
    }
    #[doc = "Address detection mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDRDEN_A::_1)
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
#[doc = "Field `ABRIF` reader - Auto-baud Rate Interrupt Flag (Read Only) \\nThis bit is set when auto-baud rate detection function finished or the auto-baud rate counter was overflow and if ABRIEN(UART_INTEN \\[18\\]) is set then the auto-baud rate interrupt will be generated. \\nNote: This bit is read only, but it can be cleared by writing '1' to ABRDTOIF (UART_FIFOSTS\\[2\\]) and ABRDIF(UART_FIFOSTS\\[1\\])."]
pub struct ABRIF_R(crate::FieldReader<bool, bool>);
impl ABRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Auto-baud Rate Detect Enable Bit\\nThis bit is cleared automatically after auto-baud detection is finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRDEN_A {
    #[doc = "0: Auto-baud rate detect function Disabled"]
    _0 = 0,
    #[doc = "1: Auto-baud rate detect function Enabled"]
    _1 = 1,
}
impl From<ABRDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABRDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRDEN` reader - Auto-baud Rate Detect Enable Bit\\nThis bit is cleared automatically after auto-baud detection is finished."]
pub struct ABRDEN_R(crate::FieldReader<bool, ABRDEN_A>);
impl ABRDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRDEN_A {
        match self.bits {
            false => ABRDEN_A::_0,
            true => ABRDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRDEN_A::_1
    }
}
impl core::ops::Deref for ABRDEN_R {
    type Target = crate::FieldReader<bool, ABRDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRDEN` writer - Auto-baud Rate Detect Enable Bit\\nThis bit is cleared automatically after auto-baud detection is finished."]
pub struct ABRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-baud rate detect function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRDEN_A::_0)
    }
    #[doc = "Auto-baud rate detect function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRDEN_A::_1)
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
#[doc = "Auto-baud Rate Detect Bit Length \\nNote : The calculation of bit number includes the START bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ABRDBITS_A {
    #[doc = "0: 1-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x01"]
    _0 = 0,
    #[doc = "1: 2-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x02"]
    _1 = 1,
    #[doc = "2: 4-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x08"]
    _2 = 2,
    #[doc = "3: 8-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x80"]
    _3 = 3,
}
impl From<ABRDBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: ABRDBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ABRDBITS` reader - Auto-baud Rate Detect Bit Length \\nNote : The calculation of bit number includes the START bit."]
pub struct ABRDBITS_R(crate::FieldReader<u8, ABRDBITS_A>);
impl ABRDBITS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ABRDBITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRDBITS_A {
        match self.bits {
            0 => ABRDBITS_A::_0,
            1 => ABRDBITS_A::_1,
            2 => ABRDBITS_A::_2,
            3 => ABRDBITS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABRDBITS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABRDBITS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ABRDBITS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ABRDBITS_A::_3
    }
}
impl core::ops::Deref for ABRDBITS_R {
    type Target = crate::FieldReader<u8, ABRDBITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRDBITS` writer - Auto-baud Rate Detect Bit Length \\nNote : The calculation of bit number includes the START bit."]
pub struct ABRDBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRDBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRDBITS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x01"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABRDBITS_A::_0)
    }
    #[doc = "2-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x02"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABRDBITS_A::_1)
    }
    #[doc = "4-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x08"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ABRDBITS_A::_2)
    }
    #[doc = "8-bit time from Start bit to the 1st rising edge. The input pattern shall be 0x80"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ABRDBITS_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `ADDRMV` reader - Address Match Value \\nThis field contains the RS-485 address match values.\\nNote: This field is used for RS-485 auto address detection mode."]
pub struct ADDRMV_R(crate::FieldReader<u8, u8>);
impl ADDRMV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMV` writer - Address Match Value \\nThis field contains the RS-485 address match values.\\nNote: This field is used for RS-485 auto address detection mode."]
pub struct ADDRMV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - UART LIN Break Field Length (Only Available in UART0/UART1 Channel)\\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: This break field length is BRKFL + 1"]
    #[inline(always)]
    pub fn brkfl(&self) -> BRKFL_R {
        BRKFL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - LIN RX Enable Bit (Only Available in UART0/UART1 Channel)"]
    #[inline(always)]
    pub fn linrxen(&self) -> LINRXEN_R {
        LINRXEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN TX Break Mode Enable Bit (Only Available in UART0/UART1 Channel)\\nNote: When TX break field transfer operation finished, this bit will be cleared automatically."]
    #[inline(always)]
    pub fn lintxen(&self) -> LINTXEN_R {
        LINTXEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RS-485 Normal Multi-drop Operation Mode (NMM) \\nNote: It cannot be active with RS-485_AAD operation mode."]
    #[inline(always)]
    pub fn rs485nmm(&self) -> RS485NMM_R {
        RS485NMM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RS-485 Auto Address Detection Operation Mode (AAD)\\nNote: It cannot be active with RS-485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485aad(&self) -> RS485AAD_R {
        RS485AAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RS-485 Auto Direction Function (AUD) \\nNote: It can be active with RS-485_AAD or RS-485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485aud(&self) -> RS485AUD_R {
        RS485AUD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RS-485 Address Detection Enable Bit\\nThis bit is used to enable RS-485 Address Detection mode. \\nNote: This bit is used for RS-485 any operation mode."]
    #[inline(always)]
    pub fn addrden(&self) -> ADDRDEN_R {
        ADDRDEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Auto-baud Rate Interrupt Flag (Read Only) \\nThis bit is set when auto-baud rate detection function finished or the auto-baud rate counter was overflow and if ABRIEN(UART_INTEN \\[18\\]) is set then the auto-baud rate interrupt will be generated. \\nNote: This bit is read only, but it can be cleared by writing '1' to ABRDTOIF (UART_FIFOSTS\\[2\\]) and ABRDIF(UART_FIFOSTS\\[1\\])."]
    #[inline(always)]
    pub fn abrif(&self) -> ABRIF_R {
        ABRIF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Auto-baud Rate Detect Enable Bit\\nThis bit is cleared automatically after auto-baud detection is finished."]
    #[inline(always)]
    pub fn abrden(&self) -> ABRDEN_R {
        ABRDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Auto-baud Rate Detect Bit Length \\nNote : The calculation of bit number includes the START bit."]
    #[inline(always)]
    pub fn abrdbits(&self) -> ABRDBITS_R {
        ABRDBITS_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - Address Match Value \\nThis field contains the RS-485 address match values.\\nNote: This field is used for RS-485 auto address detection mode."]
    #[inline(always)]
    pub fn addrmv(&self) -> ADDRMV_R {
        ADDRMV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART LIN Break Field Length (Only Available in UART0/UART1 Channel)\\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: This break field length is BRKFL + 1"]
    #[inline(always)]
    pub fn brkfl(&mut self) -> BRKFL_W {
        BRKFL_W { w: self }
    }
    #[doc = "Bit 6 - LIN RX Enable Bit (Only Available in UART0/UART1 Channel)"]
    #[inline(always)]
    pub fn linrxen(&mut self) -> LINRXEN_W {
        LINRXEN_W { w: self }
    }
    #[doc = "Bit 7 - LIN TX Break Mode Enable Bit (Only Available in UART0/UART1 Channel)\\nNote: When TX break field transfer operation finished, this bit will be cleared automatically."]
    #[inline(always)]
    pub fn lintxen(&mut self) -> LINTXEN_W {
        LINTXEN_W { w: self }
    }
    #[doc = "Bit 8 - RS-485 Normal Multi-drop Operation Mode (NMM) \\nNote: It cannot be active with RS-485_AAD operation mode."]
    #[inline(always)]
    pub fn rs485nmm(&mut self) -> RS485NMM_W {
        RS485NMM_W { w: self }
    }
    #[doc = "Bit 9 - RS-485 Auto Address Detection Operation Mode (AAD)\\nNote: It cannot be active with RS-485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485aad(&mut self) -> RS485AAD_W {
        RS485AAD_W { w: self }
    }
    #[doc = "Bit 10 - RS-485 Auto Direction Function (AUD) \\nNote: It can be active with RS-485_AAD or RS-485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485aud(&mut self) -> RS485AUD_W {
        RS485AUD_W { w: self }
    }
    #[doc = "Bit 15 - RS-485 Address Detection Enable Bit\\nThis bit is used to enable RS-485 Address Detection mode. \\nNote: This bit is used for RS-485 any operation mode."]
    #[inline(always)]
    pub fn addrden(&mut self) -> ADDRDEN_W {
        ADDRDEN_W { w: self }
    }
    #[doc = "Bit 18 - Auto-baud Rate Detect Enable Bit\\nThis bit is cleared automatically after auto-baud detection is finished."]
    #[inline(always)]
    pub fn abrden(&mut self) -> ABRDEN_W {
        ABRDEN_W { w: self }
    }
    #[doc = "Bits 19:20 - Auto-baud Rate Detect Bit Length \\nNote : The calculation of bit number includes the START bit."]
    #[inline(always)]
    pub fn abrdbits(&mut self) -> ABRDBITS_W {
        ABRDBITS_W { w: self }
    }
    #[doc = "Bits 24:31 - Address Match Value \\nThis field contains the RS-485 address match values.\\nNote: This field is used for RS-485 auto address detection mode."]
    #[inline(always)]
    pub fn addrmv(&mut self) -> ADDRMV_W {
        ADDRMV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Alternate Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_altctl](index.html) module"]
pub struct UART_ALTCTL_SPEC;
impl crate::RegisterSpec for UART_ALTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_altctl::R](R) reader structure"]
impl crate::Readable for UART_ALTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_altctl::W](W) writer structure"]
impl crate::Writable for UART_ALTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_ALTCTL to value 0x0c"]
impl crate::Resettable for UART_ALTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
