#[doc = "Register `UART_LINE` reader"]
pub struct R(crate::R<UART_LINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_LINE_SPEC>> for R {
    fn from(reader: crate::R<UART_LINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LINE` writer"]
pub struct W(crate::W<UART_LINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LINE_SPEC>;
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
impl core::convert::From<crate::W<UART_LINE_SPEC>> for W {
    fn from(writer: crate::W<UART_LINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Word Length Selection\\nThis field sets UART word length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLS_A {
    #[doc = "0: 5 bits"]
    _0 = 0,
    #[doc = "1: 6 bits"]
    _1 = 1,
    #[doc = "2: 7 bits"]
    _2 = 2,
    #[doc = "3: 8 bits"]
    _3 = 3,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WLS` reader - Word Length Selection\\nThis field sets UART word length."]
pub struct WLS_R(crate::FieldReader<u8, WLS_A>);
impl WLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLS_A {
        match self.bits {
            0 => WLS_A::_0,
            1 => WLS_A::_1,
            2 => WLS_A::_2,
            3 => WLS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WLS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == WLS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == WLS_A::_3
    }
}
impl core::ops::Deref for WLS_R {
    type Target = crate::FieldReader<u8, WLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLS` writer - Word Length Selection\\nThis field sets UART word length."]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WLS_A::_0)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WLS_A::_1)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WLS_A::_2)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WLS_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Number of 'STOP Bit'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSB_A {
    #[doc = "0: One 'STOP bit' is generated in the transmitted data"]
    _0 = 0,
    #[doc = "1: When select 5-bit word length, 1.5 'STOP bit' is generated in the transmitted data. When select 6-, 7- and 8-bit word length, 2 'STOP bit' is generated in the transmitted data"]
    _1 = 1,
}
impl From<NSB_A> for bool {
    #[inline(always)]
    fn from(variant: NSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSB` reader - Number of 'STOP Bit'"]
pub struct NSB_R(crate::FieldReader<bool, NSB_A>);
impl NSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSB_A {
        match self.bits {
            false => NSB_A::_0,
            true => NSB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NSB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NSB_A::_1
    }
}
impl core::ops::Deref for NSB_R {
    type Target = crate::FieldReader<bool, NSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSB` writer - Number of 'STOP Bit'"]
pub struct NSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One 'STOP bit' is generated in the transmitted data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NSB_A::_0)
    }
    #[doc = "When select 5-bit word length, 1.5 'STOP bit' is generated in the transmitted data. When select 6-, 7- and 8-bit word length, 2 'STOP bit' is generated in the transmitted data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSB_A::_1)
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
#[doc = "Parity Bit Enable Bit\\nNote : Parity bit is generated on each outgoing character and is checked on each incoming data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBE_A {
    #[doc = "0: No parity bit generated Disabled"]
    _0 = 0,
    #[doc = "1: Parity bit generated Enabled"]
    _1 = 1,
}
impl From<PBE_A> for bool {
    #[inline(always)]
    fn from(variant: PBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE` reader - Parity Bit Enable Bit\\nNote : Parity bit is generated on each outgoing character and is checked on each incoming data."]
pub struct PBE_R(crate::FieldReader<bool, PBE_A>);
impl PBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBE_A {
        match self.bits {
            false => PBE_A::_0,
            true => PBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PBE_A::_1
    }
}
impl core::ops::Deref for PBE_R {
    type Target = crate::FieldReader<bool, PBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBE` writer - Parity Bit Enable Bit\\nNote : Parity bit is generated on each outgoing character and is checked on each incoming data."]
pub struct PBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity bit generated Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PBE_A::_0)
    }
    #[doc = "Parity bit generated Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PBE_A::_1)
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
#[doc = "Even Parity Enable Bit\\nNote:This bit has effect only when PBE (UART_LINE\\[3\\]) is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPE_A {
    #[doc = "0: Odd number of logic 1's is transmitted and checked in each word"]
    _0 = 0,
    #[doc = "1: Even number of logic 1's is transmitted and checked in each word"]
    _1 = 1,
}
impl From<EPE_A> for bool {
    #[inline(always)]
    fn from(variant: EPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPE` reader - Even Parity Enable Bit\\nNote:This bit has effect only when PBE (UART_LINE\\[3\\]) is set."]
pub struct EPE_R(crate::FieldReader<bool, EPE_A>);
impl EPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPE_A {
        match self.bits {
            false => EPE_A::_0,
            true => EPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPE_A::_1
    }
}
impl core::ops::Deref for EPE_R {
    type Target = crate::FieldReader<bool, EPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPE` writer - Even Parity Enable Bit\\nNote:This bit has effect only when PBE (UART_LINE\\[3\\]) is set."]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Odd number of logic 1's is transmitted and checked in each word"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPE_A::_0)
    }
    #[doc = "Even number of logic 1's is transmitted and checked in each word"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPE_A::_1)
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
#[doc = "Stick Parity Enable Bit\\nNote: If PBE (UART_LINE\\[3\\]) and EPE (UART_LINE\\[4\\]) are logic 1, the parity bit is transmitted and checked as logic 0. If PBE (UART_LINE\\[3\\]) is 1 and EPE (UART_LINE\\[4\\]) is 0 then the parity bit is transmitted and checked as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    #[doc = "0: Stick parity Disabled"]
    _0 = 0,
    #[doc = "1: Stick parity Enabled"]
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - Stick Parity Enable Bit\\nNote: If PBE (UART_LINE\\[3\\]) and EPE (UART_LINE\\[4\\]) are logic 1, the parity bit is transmitted and checked as logic 0. If PBE (UART_LINE\\[3\\]) is 1 and EPE (UART_LINE\\[4\\]) is 0 then the parity bit is transmitted and checked as 1."]
pub struct SPE_R(crate::FieldReader<bool, SPE_A>);
impl SPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPE_A::_1
    }
}
impl core::ops::Deref for SPE_R {
    type Target = crate::FieldReader<bool, SPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPE` writer - Stick Parity Enable Bit\\nNote: If PBE (UART_LINE\\[3\\]) and EPE (UART_LINE\\[4\\]) are logic 1, the parity bit is transmitted and checked as logic 0. If PBE (UART_LINE\\[3\\]) is 1 and EPE (UART_LINE\\[4\\]) is 0 then the parity bit is transmitted and checked as 1."]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stick parity Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPE_A::_0)
    }
    #[doc = "Stick parity Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPE_A::_1)
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
#[doc = "Break Control Bit\\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCB_A {
    #[doc = "0: Break Control Disabled"]
    _0 = 0,
    #[doc = "1: Break Control Enabled"]
    _1 = 1,
}
impl From<BCB_A> for bool {
    #[inline(always)]
    fn from(variant: BCB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCB` reader - Break Control Bit\\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
pub struct BCB_R(crate::FieldReader<bool, BCB_A>);
impl BCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCB_A {
        match self.bits {
            false => BCB_A::_0,
            true => BCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCB_A::_1
    }
}
impl core::ops::Deref for BCB_R {
    type Target = crate::FieldReader<bool, BCB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCB` writer - Break Control Bit\\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
pub struct BCB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Break Control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCB_A::_0)
    }
    #[doc = "Break Control Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCB_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Word Length Selection\\nThis field sets UART word length."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Number of 'STOP Bit'"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Bit Enable Bit\\nNote : Parity bit is generated on each outgoing character and is checked on each incoming data."]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Even Parity Enable Bit\\nNote:This bit has effect only when PBE (UART_LINE\\[3\\]) is set."]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stick Parity Enable Bit\\nNote: If PBE (UART_LINE\\[3\\]) and EPE (UART_LINE\\[4\\]) are logic 1, the parity bit is transmitted and checked as logic 0. If PBE (UART_LINE\\[3\\]) is 1 and EPE (UART_LINE\\[4\\]) is 0 then the parity bit is transmitted and checked as 1."]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit\\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Selection\\nThis field sets UART word length."]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - Number of 'STOP Bit'"]
    #[inline(always)]
    pub fn nsb(&mut self) -> NSB_W {
        NSB_W { w: self }
    }
    #[doc = "Bit 3 - Parity Bit Enable Bit\\nNote : Parity bit is generated on each outgoing character and is checked on each incoming data."]
    #[inline(always)]
    pub fn pbe(&mut self) -> PBE_W {
        PBE_W { w: self }
    }
    #[doc = "Bit 4 - Even Parity Enable Bit\\nNote:This bit has effect only when PBE (UART_LINE\\[3\\]) is set."]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 5 - Stick Parity Enable Bit\\nNote: If PBE (UART_LINE\\[3\\]) and EPE (UART_LINE\\[4\\]) are logic 1, the parity bit is transmitted and checked as logic 0. If PBE (UART_LINE\\[3\\]) is 1 and EPE (UART_LINE\\[4\\]) is 0 then the parity bit is transmitted and checked as 1."]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 6 - Break Control Bit\\nNote: When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX line and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcb(&mut self) -> BCB_W {
        BCB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_line](index.html) module"]
pub struct UART_LINE_SPEC;
impl crate::RegisterSpec for UART_LINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_line::R](R) reader structure"]
impl crate::Readable for UART_LINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_line::W](W) writer structure"]
impl crate::Writable for UART_LINE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LINE to value 0"]
impl crate::Resettable for UART_LINE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
