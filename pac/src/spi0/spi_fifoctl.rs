#[doc = "Register `SPI_FIFOCTL` reader"]
pub struct R(crate::R<SPI_FIFOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_FIFOCTL_SPEC>> for R {
    fn from(reader: crate::R<SPI_FIFOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FIFOCTL` writer"]
pub struct W(crate::W<SPI_FIFOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFOCTL_SPEC>;
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
impl core::convert::From<crate::W<SPI_FIFOCTL_SPEC>> for W {
    fn from(writer: crate::W<SPI_FIFOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Reset\\nNote: If there is slave receive time-out event, the RXRST will be set 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset receive FIFO pointer and receive circuit. The RXFULL bit will be cleared to 0 and the RXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 3 system clock cycles + 2 peripheral clock cycles after it is set to 1. User can read TXRXRST (SPI_STATUS\\[23\\]) to check if reset is accomplished or not"]
    _1 = 1,
}
impl From<RXRST_A> for bool {
    #[inline(always)]
    fn from(variant: RXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRST` reader - Receive Reset\\nNote: If there is slave receive time-out event, the RXRST will be set 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
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
#[doc = "Field `RXRST` writer - Receive Reset\\nNote: If there is slave receive time-out event, the RXRST will be set 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
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
    #[doc = "Reset receive FIFO pointer and receive circuit. The RXFULL bit will be cleared to 0 and the RXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 3 system clock cycles + 2 peripheral clock cycles after it is set to 1. User can read TXRXRST (SPI_STATUS\\[23\\]) to check if reset is accomplished or not"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Transmit Reset\\nNote: If there is slave receive time-out event, the TXRST will be set to 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset transmit FIFO pointer and transmit circuit. The TXFULL bit will be cleared to 0 and the TXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 3 system clock cycles + 2 peripheral clock cycles after it is set to 1. User can read TXRXRST (SPI_STATUS\\[23\\]) to check if reset is accomplished or not"]
    _1 = 1,
}
impl From<TXRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRST` reader - Transmit Reset\\nNote: If there is slave receive time-out event, the TXRST will be set to 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
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
#[doc = "Field `TXRST` writer - Transmit Reset\\nNote: If there is slave receive time-out event, the TXRST will be set to 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
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
    #[doc = "Reset transmit FIFO pointer and transmit circuit. The TXFULL bit will be cleared to 0 and the TXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 3 system clock cycles + 2 peripheral clock cycles after it is set to 1. User can read TXRXRST (SPI_STATUS\\[23\\]) to check if reset is accomplished or not"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Receive FIFO Threshold Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTHIEN_A {
    #[doc = "0: RX FIFO threshold interrupt Disabled"]
    _0 = 0,
    #[doc = "1: RX FIFO threshold interrupt Enabled"]
    _1 = 1,
}
impl From<RXTHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXTHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTHIEN` reader - Receive FIFO Threshold Interrupt Enable Bit"]
pub struct RXTHIEN_R(crate::FieldReader<bool, RXTHIEN_A>);
impl RXTHIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTHIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTHIEN_A {
        match self.bits {
            false => RXTHIEN_A::_0,
            true => RXTHIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTHIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTHIEN_A::_1
    }
}
impl core::ops::Deref for RXTHIEN_R {
    type Target = crate::FieldReader<bool, RXTHIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTHIEN` writer - Receive FIFO Threshold Interrupt Enable Bit"]
pub struct RXTHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTHIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX FIFO threshold interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTHIEN_A::_0)
    }
    #[doc = "RX FIFO threshold interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTHIEN_A::_1)
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
#[doc = "Transmit FIFO Threshold Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTHIEN_A {
    #[doc = "0: TX FIFO threshold interrupt Disabled"]
    _0 = 0,
    #[doc = "1: TX FIFO threshold interrupt Enabled"]
    _1 = 1,
}
impl From<TXTHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXTHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTHIEN` reader - Transmit FIFO Threshold Interrupt Enable Bit"]
pub struct TXTHIEN_R(crate::FieldReader<bool, TXTHIEN_A>);
impl TXTHIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTHIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTHIEN_A {
        match self.bits {
            false => TXTHIEN_A::_0,
            true => TXTHIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXTHIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXTHIEN_A::_1
    }
}
impl core::ops::Deref for TXTHIEN_R {
    type Target = crate::FieldReader<bool, TXTHIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTHIEN` writer - Transmit FIFO Threshold Interrupt Enable Bit"]
pub struct TXTHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTHIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX FIFO threshold interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTHIEN_A::_0)
    }
    #[doc = "TX FIFO threshold interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTHIEN_A::_1)
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
#[doc = "Slave Receive Time-out Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIEN_A {
    #[doc = "0: Receive time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive time-out interrupt Enabled"]
    _1 = 1,
}
impl From<RXTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIEN` reader - Slave Receive Time-out Interrupt Enable Bit"]
pub struct RXTOIEN_R(crate::FieldReader<bool, RXTOIEN_A>);
impl RXTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIEN_A {
        match self.bits {
            false => RXTOIEN_A::_0,
            true => RXTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOIEN_A::_1
    }
}
impl core::ops::Deref for RXTOIEN_R {
    type Target = crate::FieldReader<bool, RXTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOIEN` writer - Slave Receive Time-out Interrupt Enable Bit"]
pub struct RXTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTOIEN_A::_0)
    }
    #[doc = "Receive time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTOIEN_A::_1)
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
#[doc = "Receive FIFO Overrun Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVIEN_A {
    #[doc = "0: Receive FIFO overrun interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive FIFO overrun interrupt Enabled"]
    _1 = 1,
}
impl From<RXOVIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVIEN` reader - Receive FIFO Overrun Interrupt Enable Bit"]
pub struct RXOVIEN_R(crate::FieldReader<bool, RXOVIEN_A>);
impl RXOVIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVIEN_A {
        match self.bits {
            false => RXOVIEN_A::_0,
            true => RXOVIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOVIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOVIEN_A::_1
    }
}
impl core::ops::Deref for RXOVIEN_R {
    type Target = crate::FieldReader<bool, RXOVIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVIEN` writer - Receive FIFO Overrun Interrupt Enable Bit"]
pub struct RXOVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO overrun interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOVIEN_A::_0)
    }
    #[doc = "Receive FIFO overrun interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOVIEN_A::_1)
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
#[doc = "TX Underflow Data Polarity\\nNote 1: The TX underflow event occurs if there is not any data in TX FIFO when the slave selection signal is active.\\nNote 2: This bit should be set as 0 in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUFPOL_A {
    #[doc = "0: The SPI data out is keep 0 if there is TX underflow event in Slave mode"]
    _0 = 0,
    #[doc = "1: The SPI data out is keep 1 if there is TX underflow event in Slave mode"]
    _1 = 1,
}
impl From<TXUFPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXUFPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUFPOL` reader - TX Underflow Data Polarity\\nNote 1: The TX underflow event occurs if there is not any data in TX FIFO when the slave selection signal is active.\\nNote 2: This bit should be set as 0 in I2S mode."]
pub struct TXUFPOL_R(crate::FieldReader<bool, TXUFPOL_A>);
impl TXUFPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUFPOL_A {
        match self.bits {
            false => TXUFPOL_A::_0,
            true => TXUFPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXUFPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXUFPOL_A::_1
    }
}
impl core::ops::Deref for TXUFPOL_R {
    type Target = crate::FieldReader<bool, TXUFPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFPOL` writer - TX Underflow Data Polarity\\nNote 1: The TX underflow event occurs if there is not any data in TX FIFO when the slave selection signal is active.\\nNote 2: This bit should be set as 0 in I2S mode."]
pub struct TXUFPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUFPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SPI data out is keep 0 if there is TX underflow event in Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUFPOL_A::_0)
    }
    #[doc = "The SPI data out is keep 1 if there is TX underflow event in Slave mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUFPOL_A::_1)
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
#[doc = "TX Underflow Interrupt Enable Bit\\nIn Slave mode, when TX underflow event occurs, this interrupt flag will be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUFIEN_A {
    #[doc = "0: Slave TX underflow interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave TX underflow interrupt Enabled"]
    _1 = 1,
}
impl From<TXUFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXUFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUFIEN` reader - TX Underflow Interrupt Enable Bit\\nIn Slave mode, when TX underflow event occurs, this interrupt flag will be set to 1."]
pub struct TXUFIEN_R(crate::FieldReader<bool, TXUFIEN_A>);
impl TXUFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUFIEN_A {
        match self.bits {
            false => TXUFIEN_A::_0,
            true => TXUFIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXUFIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXUFIEN_A::_1
    }
}
impl core::ops::Deref for TXUFIEN_R {
    type Target = crate::FieldReader<bool, TXUFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFIEN` writer - TX Underflow Interrupt Enable Bit\\nIn Slave mode, when TX underflow event occurs, this interrupt flag will be set to 1."]
pub struct TXUFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave TX underflow interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUFIEN_A::_0)
    }
    #[doc = "Slave TX underflow interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUFIEN_A::_1)
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
#[doc = "Receive FIFO Buffer Clear\\nNote: The RX shift register will not be cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFBCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear receive FIFO pointer. The RXFULL bit will be cleared to 0 and the RXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 1 system clock after it is set to 1"]
    _1 = 1,
}
impl From<RXFBCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RXFBCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFBCLR` reader - Receive FIFO Buffer Clear\\nNote: The RX shift register will not be cleared."]
pub struct RXFBCLR_R(crate::FieldReader<bool, RXFBCLR_A>);
impl RXFBCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFBCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFBCLR_A {
        match self.bits {
            false => RXFBCLR_A::_0,
            true => RXFBCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXFBCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXFBCLR_A::_1
    }
}
impl core::ops::Deref for RXFBCLR_R {
    type Target = crate::FieldReader<bool, RXFBCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFBCLR` writer - Receive FIFO Buffer Clear\\nNote: The RX shift register will not be cleared."]
pub struct RXFBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFBCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFBCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFBCLR_A::_0)
    }
    #[doc = "Clear receive FIFO pointer. The RXFULL bit will be cleared to 0 and the RXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 1 system clock after it is set to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFBCLR_A::_1)
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
#[doc = "Transmit FIFO Buffer Clear\\nNote: The TX shift register will not be cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFBCLR_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear transmit FIFO pointer. The TXFULL bit will be cleared to 0 and the TXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 1 system clock after it is set to 1"]
    _1 = 1,
}
impl From<TXFBCLR_A> for bool {
    #[inline(always)]
    fn from(variant: TXFBCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFBCLR` reader - Transmit FIFO Buffer Clear\\nNote: The TX shift register will not be cleared."]
pub struct TXFBCLR_R(crate::FieldReader<bool, TXFBCLR_A>);
impl TXFBCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFBCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFBCLR_A {
        match self.bits {
            false => TXFBCLR_A::_0,
            true => TXFBCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXFBCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXFBCLR_A::_1
    }
}
impl core::ops::Deref for TXFBCLR_R {
    type Target = crate::FieldReader<bool, TXFBCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFBCLR` writer - Transmit FIFO Buffer Clear\\nNote: The TX shift register will not be cleared."]
pub struct TXFBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFBCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFBCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFBCLR_A::_0)
    }
    #[doc = "Clear transmit FIFO pointer. The TXFULL bit will be cleared to 0 and the TXEMPTY bit will be set to 1. This bit will be cleared to 0 by hardware about 1 system clock after it is set to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFBCLR_A::_1)
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
#[doc = "Field `RXTH` reader - Receive FIFO Threshold\\nIf the valid data count of the receive FIFO buffer is larger than the RXTH setting, the RXTHIF bit will be set to 1, else the RXTHIF bit will be cleared to 0. In SPI0, RXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[25:24\\])."]
pub struct RXTH_R(crate::FieldReader<u8, u8>);
impl RXTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTH` writer - Receive FIFO Threshold\\nIf the valid data count of the receive FIFO buffer is larger than the RXTH setting, the RXTHIF bit will be set to 1, else the RXTHIF bit will be cleared to 0. In SPI0, RXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[25:24\\])."]
pub struct RXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `TXTH` reader - Transmit FIFO Threshold\\nIf the valid data count of the transmit FIFO buffer is less than or equal to the TXTH setting, the TXTHIF bit will be set to 1, else the TXTHIF bit will be cleared to 0. In SPI0, TXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[29:28\\])."]
pub struct TXTH_R(crate::FieldReader<u8, u8>);
impl TXTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTH` writer - Transmit FIFO Threshold\\nIf the valid data count of the transmit FIFO buffer is less than or equal to the TXTH setting, the TXTHIF bit will be set to 1, else the TXTHIF bit will be cleared to 0. In SPI0, TXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[29:28\\])."]
pub struct TXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Reset\\nNote: If there is slave receive time-out event, the RXRST will be set 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
    #[inline(always)]
    pub fn rxrst(&self) -> RXRST_R {
        RXRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Reset\\nNote: If there is slave receive time-out event, the TXRST will be set to 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
    #[inline(always)]
    pub fn txrst(&self) -> TXRST_R {
        TXRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Threshold Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxthien(&self) -> RXTHIEN_R {
        RXTHIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO Threshold Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txthien(&self) -> TXTHIEN_R {
        TXTHIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Receive Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxtoien(&self) -> RXTOIEN_R {
        RXTOIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Overrun Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxovien(&self) -> RXOVIEN_R {
        RXOVIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Underflow Data Polarity\\nNote 1: The TX underflow event occurs if there is not any data in TX FIFO when the slave selection signal is active.\\nNote 2: This bit should be set as 0 in I2S mode."]
    #[inline(always)]
    pub fn txufpol(&self) -> TXUFPOL_R {
        TXUFPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Enable Bit\\nIn Slave mode, when TX underflow event occurs, this interrupt flag will be set to 1."]
    #[inline(always)]
    pub fn txufien(&self) -> TXUFIEN_R {
        TXUFIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Buffer Clear\\nNote: The RX shift register will not be cleared."]
    #[inline(always)]
    pub fn rxfbclr(&self) -> RXFBCLR_R {
        RXFBCLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO Buffer Clear\\nNote: The TX shift register will not be cleared."]
    #[inline(always)]
    pub fn txfbclr(&self) -> TXFBCLR_R {
        TXFBCLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Receive FIFO Threshold\\nIf the valid data count of the receive FIFO buffer is larger than the RXTH setting, the RXTHIF bit will be set to 1, else the RXTHIF bit will be cleared to 0. In SPI0, RXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[25:24\\])."]
    #[inline(always)]
    pub fn rxth(&self) -> RXTH_R {
        RXTH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Transmit FIFO Threshold\\nIf the valid data count of the transmit FIFO buffer is less than or equal to the TXTH setting, the TXTHIF bit will be set to 1, else the TXTHIF bit will be cleared to 0. In SPI0, TXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[29:28\\])."]
    #[inline(always)]
    pub fn txth(&self) -> TXTH_R {
        TXTH_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Reset\\nNote: If there is slave receive time-out event, the RXRST will be set 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
    #[inline(always)]
    pub fn rxrst(&mut self) -> RXRST_W {
        RXRST_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Reset\\nNote: If there is slave receive time-out event, the TXRST will be set to 1 when the SLVTORST (SPI_SSCTL\\[6\\]) is enabled."]
    #[inline(always)]
    pub fn txrst(&mut self) -> TXRST_W {
        TXRST_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Threshold Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxthien(&mut self) -> RXTHIEN_W {
        RXTHIEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit FIFO Threshold Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txthien(&mut self) -> TXTHIEN_W {
        TXTHIEN_W { w: self }
    }
    #[doc = "Bit 4 - Slave Receive Time-out Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxtoien(&mut self) -> RXTOIEN_W {
        RXTOIEN_W { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO Overrun Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxovien(&mut self) -> RXOVIEN_W {
        RXOVIEN_W { w: self }
    }
    #[doc = "Bit 6 - TX Underflow Data Polarity\\nNote 1: The TX underflow event occurs if there is not any data in TX FIFO when the slave selection signal is active.\\nNote 2: This bit should be set as 0 in I2S mode."]
    #[inline(always)]
    pub fn txufpol(&mut self) -> TXUFPOL_W {
        TXUFPOL_W { w: self }
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Enable Bit\\nIn Slave mode, when TX underflow event occurs, this interrupt flag will be set to 1."]
    #[inline(always)]
    pub fn txufien(&mut self) -> TXUFIEN_W {
        TXUFIEN_W { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Buffer Clear\\nNote: The RX shift register will not be cleared."]
    #[inline(always)]
    pub fn rxfbclr(&mut self) -> RXFBCLR_W {
        RXFBCLR_W { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO Buffer Clear\\nNote: The TX shift register will not be cleared."]
    #[inline(always)]
    pub fn txfbclr(&mut self) -> TXFBCLR_W {
        TXFBCLR_W { w: self }
    }
    #[doc = "Bits 24:26 - Receive FIFO Threshold\\nIf the valid data count of the receive FIFO buffer is larger than the RXTH setting, the RXTHIF bit will be set to 1, else the RXTHIF bit will be cleared to 0. In SPI0, RXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[25:24\\])."]
    #[inline(always)]
    pub fn rxth(&mut self) -> RXTH_W {
        RXTH_W { w: self }
    }
    #[doc = "Bits 28:30 - Transmit FIFO Threshold\\nIf the valid data count of the transmit FIFO buffer is less than or equal to the TXTH setting, the TXTHIF bit will be set to 1, else the TXTHIF bit will be cleared to 0. In SPI0, TXTH is a 3-bit wide configuration; in SPI1 and SPI2, 2-bit wide only (SPI_FIFOCTL\\[29:28\\])."]
    #[inline(always)]
    pub fn txth(&mut self) -> TXTH_W {
        TXTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifoctl](index.html) module"]
pub struct SPI_FIFOCTL_SPEC;
impl crate::RegisterSpec for SPI_FIFOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fifoctl::R](R) reader structure"]
impl crate::Readable for SPI_FIFOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifoctl::W](W) writer structure"]
impl crate::Writable for SPI_FIFOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FIFOCTL to value 0x4400_0000"]
impl crate::Resettable for SPI_FIFOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4400_0000
    }
}
