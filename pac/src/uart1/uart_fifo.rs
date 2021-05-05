#[doc = "Register `UART_FIFO` reader"]
pub struct R(crate::R<UART_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_FIFO_SPEC>> for R {
    fn from(reader: crate::R<UART_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_FIFO` writer"]
pub struct W(crate::W<UART_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_SPEC>;
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
impl core::convert::From<crate::W<UART_FIFO_SPEC>> for W {
    fn from(writer: crate::W<UART_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX Field Software Reset\\nWhen RXRST (UART_FIFO\\[1\\]) is set, all the byte in the receiver FIFO and RX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the RX internal state machine and pointers"]
    _1 = 1,
}
impl From<RXRST_A> for bool {
    #[inline(always)]
    fn from(variant: RXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRST` reader - RX Field Software Reset\\nWhen RXRST (UART_FIFO\\[1\\]) is set, all the byte in the receiver FIFO and RX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
pub struct RXRST_R(crate::FieldReader<bool, RXRST_A>);
impl RXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRST_A {
        match self.bits {
            false => RXRST_A::_0,
            true => RXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXRST_A::_1
    }
}
impl core::ops::Deref for RXRST_R {
    type Target = crate::FieldReader<bool, RXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRST` writer - RX Field Software Reset\\nWhen RXRST (UART_FIFO\\[1\\]) is set, all the byte in the receiver FIFO and RX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
pub struct RXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRST_A::_0)
    }
    #[doc = "Reset the RX internal state machine and pointers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRST_A::_1)
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
#[doc = "TX Field Software Reset\\nWhen TXRST (UART_FIFO\\[2\\]) is set, all the byte in the transmit FIFO and TX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the TX internal state machine and pointers"]
    _1 = 1,
}
impl From<TXRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRST` reader - TX Field Software Reset\\nWhen TXRST (UART_FIFO\\[2\\]) is set, all the byte in the transmit FIFO and TX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
pub struct TXRST_R(crate::FieldReader<bool, TXRST_A>);
impl TXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRST_A {
        match self.bits {
            false => TXRST_A::_0,
            true => TXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRST_A::_1
    }
}
impl core::ops::Deref for TXRST_R {
    type Target = crate::FieldReader<bool, TXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRST` writer - TX Field Software Reset\\nWhen TXRST (UART_FIFO\\[2\\]) is set, all the byte in the transmit FIFO and TX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
pub struct TXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRST_A::_0)
    }
    #[doc = "Reset the TX internal state machine and pointers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRST_A::_1)
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
#[doc = "RX FIFO Interrupt Trigger Level\\nWhen the number of bytes in the receive FIFO equals the RFITL, the RDAIF will be set (if RDAIEN (UART_INTEN \\[0\\]) enabled, and an interrupt will be generated).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RFITL_A {
    #[doc = "0: RX FIFO Interrupt Trigger Level is 1 byte"]
    _0 = 0,
    #[doc = "1: RX FIFO Interrupt Trigger Level is 4 bytes"]
    _1 = 1,
    #[doc = "2: RX FIFO Interrupt Trigger Level is 8 bytes"]
    _2 = 2,
    #[doc = "3: RX FIFO Interrupt Trigger Level is 14 bytes"]
    _3 = 3,
}
impl From<RFITL_A> for u8 {
    #[inline(always)]
    fn from(variant: RFITL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RFITL` reader - RX FIFO Interrupt Trigger Level\\nWhen the number of bytes in the receive FIFO equals the RFITL, the RDAIF will be set (if RDAIEN (UART_INTEN \\[0\\]) enabled, and an interrupt will be generated)."]
pub struct RFITL_R(crate::FieldReader<u8, RFITL_A>);
impl RFITL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFITL_A> {
        match self.bits {
            0 => Some(RFITL_A::_0),
            1 => Some(RFITL_A::_1),
            2 => Some(RFITL_A::_2),
            3 => Some(RFITL_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFITL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFITL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RFITL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RFITL_A::_3
    }
}
impl core::ops::Deref for RFITL_R {
    type Target = crate::FieldReader<u8, RFITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFITL` writer - RX FIFO Interrupt Trigger Level\\nWhen the number of bytes in the receive FIFO equals the RFITL, the RDAIF will be set (if RDAIEN (UART_INTEN \\[0\\]) enabled, and an interrupt will be generated)."]
pub struct RFITL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFITL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RX FIFO Interrupt Trigger Level is 1 byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFITL_A::_0)
    }
    #[doc = "RX FIFO Interrupt Trigger Level is 4 bytes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFITL_A::_1)
    }
    #[doc = "RX FIFO Interrupt Trigger Level is 8 bytes"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RFITL_A::_2)
    }
    #[doc = "RX FIFO Interrupt Trigger Level is 14 bytes"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RFITL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Receiver Disable \\nThe receiver is disabled or not (set 1 to disable receiver)\\nNote: This bit is used for RS-485 Normal Multi-drop mode. It should be programmed before RS485NMM (UART_ALTCTL \\[8\\]) is programmed.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFF_A {
    #[doc = "0: Receiver Enabled"]
    _0 = 0,
    #[doc = "1: Receiver Disabled"]
    _1 = 1,
}
impl From<RXOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOFF` reader - Receiver Disable \\nThe receiver is disabled or not (set 1 to disable receiver)\\nNote: This bit is used for RS-485 Normal Multi-drop mode. It should be programmed before RS485NMM (UART_ALTCTL \\[8\\]) is programmed."]
pub struct RXOFF_R(crate::FieldReader<bool, RXOFF_A>);
impl RXOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFF_A {
        match self.bits {
            false => RXOFF_A::_0,
            true => RXOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOFF_A::_1
    }
}
impl core::ops::Deref for RXOFF_R {
    type Target = crate::FieldReader<bool, RXOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOFF` writer - Receiver Disable \\nThe receiver is disabled or not (set 1 to disable receiver)\\nNote: This bit is used for RS-485 Normal Multi-drop mode. It should be programmed before RS485NMM (UART_ALTCTL \\[8\\]) is programmed."]
pub struct RXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFF_A::_0)
    }
    #[doc = "Receiver Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFF_A::_1)
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
#[doc = "nRTS Trigger Level for Auto-flow Control Use\\nNote: This field is used for auto nRTS flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTSTRGLV_A {
    #[doc = "0: nRTS Trigger Level is 1 byte"]
    _0 = 0,
    #[doc = "1: nRTS Trigger Level is 4 bytes"]
    _1 = 1,
    #[doc = "2: nRTS Trigger Level is 8 bytes"]
    _2 = 2,
    #[doc = "3: nRTS Trigger Level is 14 bytes"]
    _3 = 3,
}
impl From<RTSTRGLV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTSTRGLV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTSTRGLV` reader - nRTS Trigger Level for Auto-flow Control Use\\nNote: This field is used for auto nRTS flow control."]
pub struct RTSTRGLV_R(crate::FieldReader<u8, RTSTRGLV_A>);
impl RTSTRGLV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTSTRGLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTSTRGLV_A> {
        match self.bits {
            0 => Some(RTSTRGLV_A::_0),
            1 => Some(RTSTRGLV_A::_1),
            2 => Some(RTSTRGLV_A::_2),
            3 => Some(RTSTRGLV_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTSTRGLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTSTRGLV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RTSTRGLV_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RTSTRGLV_A::_3
    }
}
impl core::ops::Deref for RTSTRGLV_R {
    type Target = crate::FieldReader<u8, RTSTRGLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSTRGLV` writer - nRTS Trigger Level for Auto-flow Control Use\\nNote: This field is used for auto nRTS flow control."]
pub struct RTSTRGLV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSTRGLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSTRGLV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "nRTS Trigger Level is 1 byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSTRGLV_A::_0)
    }
    #[doc = "nRTS Trigger Level is 4 bytes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSTRGLV_A::_1)
    }
    #[doc = "nRTS Trigger Level is 8 bytes"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RTSTRGLV_A::_2)
    }
    #[doc = "nRTS Trigger Level is 14 bytes"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RTSTRGLV_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - RX Field Software Reset\\nWhen RXRST (UART_FIFO\\[1\\]) is set, all the byte in the receiver FIFO and RX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
    #[inline(always)]
    pub fn rxrst(&self) -> RXRST_R {
        RXRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX Field Software Reset\\nWhen TXRST (UART_FIFO\\[2\\]) is set, all the byte in the transmit FIFO and TX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
    #[inline(always)]
    pub fn txrst(&self) -> TXRST_R {
        TXRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - RX FIFO Interrupt Trigger Level\\nWhen the number of bytes in the receive FIFO equals the RFITL, the RDAIF will be set (if RDAIEN (UART_INTEN \\[0\\]) enabled, and an interrupt will be generated)."]
    #[inline(always)]
    pub fn rfitl(&self) -> RFITL_R {
        RFITL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Receiver Disable \\nThe receiver is disabled or not (set 1 to disable receiver)\\nNote: This bit is used for RS-485 Normal Multi-drop mode. It should be programmed before RS485NMM (UART_ALTCTL \\[8\\]) is programmed."]
    #[inline(always)]
    pub fn rxoff(&self) -> RXOFF_R {
        RXOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - nRTS Trigger Level for Auto-flow Control Use\\nNote: This field is used for auto nRTS flow control."]
    #[inline(always)]
    pub fn rtstrglv(&self) -> RTSTRGLV_R {
        RTSTRGLV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - RX Field Software Reset\\nWhen RXRST (UART_FIFO\\[1\\]) is set, all the byte in the receiver FIFO and RX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
    #[inline(always)]
    pub fn rxrst(&mut self) -> RXRST_W {
        RXRST_W { w: self }
    }
    #[doc = "Bit 2 - TX Field Software Reset\\nWhen TXRST (UART_FIFO\\[2\\]) is set, all the byte in the transmit FIFO and TX internal state machine are cleared.\\nNote: This bit will automatically clear at least 3 UART peripheral clock cycles."]
    #[inline(always)]
    pub fn txrst(&mut self) -> TXRST_W {
        TXRST_W { w: self }
    }
    #[doc = "Bits 4:7 - RX FIFO Interrupt Trigger Level\\nWhen the number of bytes in the receive FIFO equals the RFITL, the RDAIF will be set (if RDAIEN (UART_INTEN \\[0\\]) enabled, and an interrupt will be generated)."]
    #[inline(always)]
    pub fn rfitl(&mut self) -> RFITL_W {
        RFITL_W { w: self }
    }
    #[doc = "Bit 8 - Receiver Disable \\nThe receiver is disabled or not (set 1 to disable receiver)\\nNote: This bit is used for RS-485 Normal Multi-drop mode. It should be programmed before RS485NMM (UART_ALTCTL \\[8\\]) is programmed."]
    #[inline(always)]
    pub fn rxoff(&mut self) -> RXOFF_W {
        RXOFF_W { w: self }
    }
    #[doc = "Bits 16:19 - nRTS Trigger Level for Auto-flow Control Use\\nNote: This field is used for auto nRTS flow control."]
    #[inline(always)]
    pub fn rtstrglv(&mut self) -> RTSTRGLV_W {
        RTSTRGLV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo](index.html) module"]
pub struct UART_FIFO_SPEC;
impl crate::RegisterSpec for UART_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo::R](R) reader structure"]
impl crate::Readable for UART_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo::W](W) writer structure"]
impl crate::Writable for UART_FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_FIFO to value 0x0101"]
impl crate::Resettable for UART_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
