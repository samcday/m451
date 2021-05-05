#[doc = "Register `UART_MODEM` reader"]
pub struct R(crate::R<UART_MODEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MODEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_MODEM_SPEC>> for R {
    fn from(reader: crate::R<UART_MODEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MODEM` writer"]
pub struct W(crate::W<UART_MODEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MODEM_SPEC>;
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
impl core::convert::From<crate::W<UART_MODEM_SPEC>> for W {
    fn from(writer: crate::W<UART_MODEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "nRTS (Request-to-send) Signal Control\\nThis bit is direct control internal nRTS signal active or not, and then drive the nRTS pin output with RTSACTLV bit configuration.\\nNote1: This nRTS signal control bit is not effective when nRTS auto-flow control is enabled in UART function mode.\\nNote2: This nRTS signal control bit is not effective when RS-485 auto direction mode (AUD) is enabled in RS-485 function mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_A {
    #[doc = "0: nRTS signal is active"]
    _0 = 0,
    #[doc = "1: nRTS signal is inactive"]
    _1 = 1,
}
impl From<RTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTS` reader - nRTS (Request-to-send) Signal Control\\nThis bit is direct control internal nRTS signal active or not, and then drive the nRTS pin output with RTSACTLV bit configuration.\\nNote1: This nRTS signal control bit is not effective when nRTS auto-flow control is enabled in UART function mode.\\nNote2: This nRTS signal control bit is not effective when RS-485 auto direction mode (AUD) is enabled in RS-485 function mode."]
pub struct RTS_R(crate::FieldReader<bool, RTS_A>);
impl RTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_A {
        match self.bits {
            false => RTS_A::_0,
            true => RTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTS_A::_1
    }
}
impl core::ops::Deref for RTS_R {
    type Target = crate::FieldReader<bool, RTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS` writer - nRTS (Request-to-send) Signal Control\\nThis bit is direct control internal nRTS signal active or not, and then drive the nRTS pin output with RTSACTLV bit configuration.\\nNote1: This nRTS signal control bit is not effective when nRTS auto-flow control is enabled in UART function mode.\\nNote2: This nRTS signal control bit is not effective when RS-485 auto direction mode (AUD) is enabled in RS-485 function mode."]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nRTS signal is active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTS_A::_0)
    }
    #[doc = "nRTS signal is inactive"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTS_A::_1)
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
#[doc = "nRTS Pin Active Level\\nThis bit defines the active level state of nRTS pin output.\\nNote1: Refer to Figure 6.1310 and Figure 6.1311 for UART function mode.\\nNote2: Refer to Figure 6.1321 and Figure 6.1322 for RS-485 function mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSACTLV_A {
    #[doc = "0: n RTS pin output is high level active"]
    _0 = 0,
    #[doc = "1: nRTS pin output is low level active. (Default)"]
    _1 = 1,
}
impl From<RTSACTLV_A> for bool {
    #[inline(always)]
    fn from(variant: RTSACTLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSACTLV` reader - nRTS Pin Active Level\\nThis bit defines the active level state of nRTS pin output.\\nNote1: Refer to Figure 6.1310 and Figure 6.1311 for UART function mode.\\nNote2: Refer to Figure 6.1321 and Figure 6.1322 for RS-485 function mode."]
pub struct RTSACTLV_R(crate::FieldReader<bool, RTSACTLV_A>);
impl RTSACTLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSACTLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSACTLV_A {
        match self.bits {
            false => RTSACTLV_A::_0,
            true => RTSACTLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTSACTLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTSACTLV_A::_1
    }
}
impl core::ops::Deref for RTSACTLV_R {
    type Target = crate::FieldReader<bool, RTSACTLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSACTLV` writer - nRTS Pin Active Level\\nThis bit defines the active level state of nRTS pin output.\\nNote1: Refer to Figure 6.1310 and Figure 6.1311 for UART function mode.\\nNote2: Refer to Figure 6.1321 and Figure 6.1322 for RS-485 function mode."]
pub struct RTSACTLV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSACTLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSACTLV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "n RTS pin output is high level active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSACTLV_A::_0)
    }
    #[doc = "nRTS pin output is low level active. (Default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSACTLV_A::_1)
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
#[doc = "nRTS Pin Status (Read Only)\\nThis bit mirror from nRTS pin output of voltage logic status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSSTS_A {
    #[doc = "0: nRTS pin output is low level voltage logic state"]
    _0 = 0,
    #[doc = "1: nRTS pin output is high level voltage logic state"]
    _1 = 1,
}
impl From<RTSSTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTSSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSSTS` reader - nRTS Pin Status (Read Only)\\nThis bit mirror from nRTS pin output of voltage logic status."]
pub struct RTSSTS_R(crate::FieldReader<bool, RTSSTS_A>);
impl RTSSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSSTS_A {
        match self.bits {
            false => RTSSTS_A::_0,
            true => RTSSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTSSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTSSTS_A::_1
    }
}
impl core::ops::Deref for RTSSTS_R {
    type Target = crate::FieldReader<bool, RTSSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - nRTS (Request-to-send) Signal Control\\nThis bit is direct control internal nRTS signal active or not, and then drive the nRTS pin output with RTSACTLV bit configuration.\\nNote1: This nRTS signal control bit is not effective when nRTS auto-flow control is enabled in UART function mode.\\nNote2: This nRTS signal control bit is not effective when RS-485 auto direction mode (AUD) is enabled in RS-485 function mode."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - nRTS Pin Active Level\\nThis bit defines the active level state of nRTS pin output.\\nNote1: Refer to Figure 6.1310 and Figure 6.1311 for UART function mode.\\nNote2: Refer to Figure 6.1321 and Figure 6.1322 for RS-485 function mode."]
    #[inline(always)]
    pub fn rtsactlv(&self) -> RTSACTLV_R {
        RTSACTLV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - nRTS Pin Status (Read Only)\\nThis bit mirror from nRTS pin output of voltage logic status."]
    #[inline(always)]
    pub fn rtssts(&self) -> RTSSTS_R {
        RTSSTS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - nRTS (Request-to-send) Signal Control\\nThis bit is direct control internal nRTS signal active or not, and then drive the nRTS pin output with RTSACTLV bit configuration.\\nNote1: This nRTS signal control bit is not effective when nRTS auto-flow control is enabled in UART function mode.\\nNote2: This nRTS signal control bit is not effective when RS-485 auto direction mode (AUD) is enabled in RS-485 function mode."]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 9 - nRTS Pin Active Level\\nThis bit defines the active level state of nRTS pin output.\\nNote1: Refer to Figure 6.1310 and Figure 6.1311 for UART function mode.\\nNote2: Refer to Figure 6.1321 and Figure 6.1322 for RS-485 function mode."]
    #[inline(always)]
    pub fn rtsactlv(&mut self) -> RTSACTLV_W {
        RTSACTLV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_modem](index.html) module"]
pub struct UART_MODEM_SPEC;
impl crate::RegisterSpec for UART_MODEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_modem::R](R) reader structure"]
impl crate::Readable for UART_MODEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_modem::W](W) writer structure"]
impl crate::Writable for UART_MODEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MODEM to value 0x0200"]
impl crate::Resettable for UART_MODEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
