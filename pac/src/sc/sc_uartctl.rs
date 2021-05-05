#[doc = "Register `SC_UARTCTL` reader"]
pub struct R(crate::R<SC_UARTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_UARTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_UARTCTL_SPEC>> for R {
    fn from(reader: crate::R<SC_UARTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_UARTCTL` writer"]
pub struct W(crate::W<SC_UARTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_UARTCTL_SPEC>;
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
impl core::convert::From<crate::W<SC_UARTCTL_SPEC>> for W {
    fn from(writer: crate::W<SC_UARTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UART Mode Enable Bit\\nNote3: When UART is enabled, hardware will generate a reset to reset FIFO and internal state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTEN_A {
    #[doc = "0: Smart Card mode"]
    _0 = 0,
    #[doc = "1: UART mode"]
    _1 = 1,
}
impl From<UARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UARTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTEN` reader - UART Mode Enable Bit\\nNote3: When UART is enabled, hardware will generate a reset to reset FIFO and internal state machine."]
pub struct UARTEN_R(crate::FieldReader<bool, UARTEN_A>);
impl UARTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UARTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTEN_A {
        match self.bits {
            false => UARTEN_A::_0,
            true => UARTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UARTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UARTEN_A::_1
    }
}
impl core::ops::Deref for UARTEN_R {
    type Target = crate::FieldReader<bool, UARTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTEN` writer - UART Mode Enable Bit\\nNote3: When UART is enabled, hardware will generate a reset to reset FIFO and internal state machine."]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smart Card mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTEN_A::_0)
    }
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTEN_A::_1)
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
#[doc = "Word Length Selection\\nNote: In smart card mode, this WLS must be '00'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLS_A {
    #[doc = "0: Word length is 8 bits"]
    _0 = 0,
    #[doc = "1: Word length is 7 bits"]
    _1 = 1,
    #[doc = "2: Word length is 6 bits"]
    _2 = 2,
    #[doc = "3: Word length is 5 bits"]
    _3 = 3,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WLS` reader - Word Length Selection\\nNote: In smart card mode, this WLS must be '00'"]
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
#[doc = "Field `WLS` writer - Word Length Selection\\nNote: In smart card mode, this WLS must be '00'"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Word length is 8 bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WLS_A::_0)
    }
    #[doc = "Word length is 7 bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WLS_A::_1)
    }
    #[doc = "Word length is 6 bits"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WLS_A::_2)
    }
    #[doc = "Word length is 5 bits"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WLS_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Parity Bit Disable Control\\nNote: In smart card mode, this field must be '0' (default setting is with parity bit)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBOFF_A {
    #[doc = "0: Parity bit is generated or checked between the 'last data word bit' and 'stop bit' of the serial data"]
    _0 = 0,
    #[doc = "1: Parity bit is not generated (transmitting data) or checked (receiving data) during transfer"]
    _1 = 1,
}
impl From<PBOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PBOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBOFF` reader - Parity Bit Disable Control\\nNote: In smart card mode, this field must be '0' (default setting is with parity bit)"]
pub struct PBOFF_R(crate::FieldReader<bool, PBOFF_A>);
impl PBOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBOFF_A {
        match self.bits {
            false => PBOFF_A::_0,
            true => PBOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PBOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PBOFF_A::_1
    }
}
impl core::ops::Deref for PBOFF_R {
    type Target = crate::FieldReader<bool, PBOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBOFF` writer - Parity Bit Disable Control\\nNote: In smart card mode, this field must be '0' (default setting is with parity bit)"]
pub struct PBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PBOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Parity bit is generated or checked between the 'last data word bit' and 'stop bit' of the serial data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PBOFF_A::_0)
    }
    #[doc = "Parity bit is not generated (transmitting data) or checked (receiving data) during transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PBOFF_A::_1)
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
#[doc = "Odd Parity Enable Bit\\nNote: This bit has effect only when PBOFF bit is '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPE_A {
    #[doc = "0: Even number of logic 1's are transmitted or check the data word and parity bits in receiving mode"]
    _0 = 0,
    #[doc = "1: Odd number of logic 1's are transmitted or check the data word and parity bits in receiving mode"]
    _1 = 1,
}
impl From<OPE_A> for bool {
    #[inline(always)]
    fn from(variant: OPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPE` reader - Odd Parity Enable Bit\\nNote: This bit has effect only when PBOFF bit is '0'."]
pub struct OPE_R(crate::FieldReader<bool, OPE_A>);
impl OPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPE_A {
        match self.bits {
            false => OPE_A::_0,
            true => OPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OPE_A::_1
    }
}
impl core::ops::Deref for OPE_R {
    type Target = crate::FieldReader<bool, OPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPE` writer - Odd Parity Enable Bit\\nNote: This bit has effect only when PBOFF bit is '0'."]
pub struct OPE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even number of logic 1's are transmitted or check the data word and parity bits in receiving mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPE_A::_0)
    }
    #[doc = "Odd number of logic 1's are transmitted or check the data word and parity bits in receiving mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPE_A::_1)
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
impl R {
    #[doc = "Bit 0 - UART Mode Enable Bit\\nNote3: When UART is enabled, hardware will generate a reset to reset FIFO and internal state machine."]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Word Length Selection\\nNote: In smart card mode, this WLS must be '00'"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Parity Bit Disable Control\\nNote: In smart card mode, this field must be '0' (default setting is with parity bit)"]
    #[inline(always)]
    pub fn pboff(&self) -> PBOFF_R {
        PBOFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Odd Parity Enable Bit\\nNote: This bit has effect only when PBOFF bit is '0'."]
    #[inline(always)]
    pub fn ope(&self) -> OPE_R {
        OPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Mode Enable Bit\\nNote3: When UART is enabled, hardware will generate a reset to reset FIFO and internal state machine."]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Word Length Selection\\nNote: In smart card mode, this WLS must be '00'"]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 6 - Parity Bit Disable Control\\nNote: In smart card mode, this field must be '0' (default setting is with parity bit)"]
    #[inline(always)]
    pub fn pboff(&mut self) -> PBOFF_W {
        PBOFF_W { w: self }
    }
    #[doc = "Bit 7 - Odd Parity Enable Bit\\nNote: This bit has effect only when PBOFF bit is '0'."]
    #[inline(always)]
    pub fn ope(&mut self) -> OPE_W {
        OPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC UART Mode Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_uartctl](index.html) module"]
pub struct SC_UARTCTL_SPEC;
impl crate::RegisterSpec for SC_UARTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_uartctl::R](R) reader structure"]
impl crate::Readable for SC_UARTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_uartctl::W](W) writer structure"]
impl crate::Writable for SC_UARTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_UARTCTL to value 0"]
impl crate::Resettable for SC_UARTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
