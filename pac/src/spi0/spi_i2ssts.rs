#[doc = "Register `SPI_I2SSTS` reader"]
pub struct R(crate::R<SPI_I2SSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2SSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_I2SSTS_SPEC>> for R {
    fn from(reader: crate::R<SPI_I2SSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_I2SSTS` writer"]
pub struct W(crate::W<SPI_I2SSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_I2SSTS_SPEC>;
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
impl core::convert::From<crate::W<SPI_I2SSTS_SPEC>> for W {
    fn from(writer: crate::W<SPI_I2SSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Right Channel (Read Only)\\nThis bit indicates the current transmit data is belong to which channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIGHT_A {
    #[doc = "0: Left channel"]
    _0 = 0,
    #[doc = "1: Right channel"]
    _1 = 1,
}
impl From<RIGHT_A> for bool {
    #[inline(always)]
    fn from(variant: RIGHT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIGHT` reader - Right Channel (Read Only)\\nThis bit indicates the current transmit data is belong to which channel."]
pub struct RIGHT_R(crate::FieldReader<bool, RIGHT_A>);
impl RIGHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIGHT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIGHT_A {
        match self.bits {
            false => RIGHT_A::_0,
            true => RIGHT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RIGHT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RIGHT_A::_1
    }
}
impl core::ops::Deref for RIGHT_R {
    type Target = crate::FieldReader<bool, RIGHT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Buffer Empty Indicator (Read Only)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: Receive FIFO buffer is not empty"]
    _0 = 0,
    #[doc = "1: Receive FIFO buffer is empty"]
    _1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - Receive FIFO Buffer Empty Indicator (Read Only)"]
pub struct RXEMPTY_R(crate::FieldReader<bool, RXEMPTY_A>);
impl RXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::_0,
            true => RXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEMPTY_A::_1
    }
}
impl core::ops::Deref for RXEMPTY_R {
    type Target = crate::FieldReader<bool, RXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Buffer Full Indicator (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFULL_A {
    #[doc = "0: Receive FIFO buffer is not full"]
    _0 = 0,
    #[doc = "1: Receive FIFO buffer is full"]
    _1 = 1,
}
impl From<RXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFULL` reader - Receive FIFO Buffer Full Indicator (Read Only)"]
pub struct RXFULL_R(crate::FieldReader<bool, RXFULL_A>);
impl RXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFULL_A {
        match self.bits {
            false => RXFULL_A::_0,
            true => RXFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXFULL_A::_1
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, RXFULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Threshold Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTHIF_A {
    #[doc = "0: The valid data count within the Rx FIFO buffer is smaller than or equal to the setting value of RXTH"]
    _0 = 0,
    #[doc = "1: The valid data count within the receive FIFO buffer is larger than the setting value of RXTH"]
    _1 = 1,
}
impl From<RXTHIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXTHIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTHIF` reader - Receive FIFO Threshold Interrupt Flag (Read Only)"]
pub struct RXTHIF_R(crate::FieldReader<bool, RXTHIF_A>);
impl RXTHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTHIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTHIF_A {
        match self.bits {
            false => RXTHIF_A::_0,
            true => RXTHIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTHIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTHIF_A::_1
    }
}
impl core::ops::Deref for RXTHIF_R {
    type Target = crate::FieldReader<bool, RXTHIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVIF` reader - Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct RXOVIF_R(crate::FieldReader<bool, bool>);
impl RXOVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVIF` writer - Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct RXOVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Receive Time-out Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIF_A {
    #[doc = "0: No receive FIFO time-out event"]
    _0 = 0,
    #[doc = "1: Receive FIFO buffer is not empty and no read operation on receive FIFO buffer over 64 SPI clock period in Master mode or over 576 peripheral clock period in Slave mode. When the received FIFO buffer is read by software, the time-out status will be cleared automatically"]
    _1 = 1,
}
impl From<RXTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIF` reader - Receive Time-out Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
pub struct RXTOIF_R(crate::FieldReader<bool, RXTOIF_A>);
impl RXTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIF_A {
        match self.bits {
            false => RXTOIF_A::_0,
            true => RXTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOIF_A::_1
    }
}
impl core::ops::Deref for RXTOIF_R {
    type Target = crate::FieldReader<bool, RXTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOIF` writer - Receive Time-out Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
pub struct RXTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No receive FIFO time-out event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTOIF_A::_0)
    }
    #[doc = "Receive FIFO buffer is not empty and no read operation on receive FIFO buffer over 64 SPI clock period in Master mode or over 576 peripheral clock period in Slave mode. When the received FIFO buffer is read by software, the time-out status will be cleared automatically"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTOIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "I2S Enable Status (Read Only)\\nNote: The SPI/I2S peripheral clock is asynchronous with the system clock. In order to make sure the SPI/I2S controller logic is disabled, this bit indicates the real status of SPI/I2S controller logic for user.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SENSTS_A {
    #[doc = "0: The SPI/I2S control logic is disabled"]
    _0 = 0,
    #[doc = "1: The SPI/I2S control logic is enabled"]
    _1 = 1,
}
impl From<I2SENSTS_A> for bool {
    #[inline(always)]
    fn from(variant: I2SENSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SENSTS` reader - I2S Enable Status (Read Only)\\nNote: The SPI/I2S peripheral clock is asynchronous with the system clock. In order to make sure the SPI/I2S controller logic is disabled, this bit indicates the real status of SPI/I2S controller logic for user."]
pub struct I2SENSTS_R(crate::FieldReader<bool, I2SENSTS_A>);
impl I2SENSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SENSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SENSTS_A {
        match self.bits {
            false => I2SENSTS_A::_0,
            true => I2SENSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2SENSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2SENSTS_A::_1
    }
}
impl core::ops::Deref for I2SENSTS_R {
    type Target = crate::FieldReader<bool, I2SENSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Buffer Empty Indicator (Read Only)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: Transmit FIFO buffer is not empty"]
    _0 = 0,
    #[doc = "1: Transmit FIFO buffer is empty"]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Buffer Empty Indicator (Read Only)"]
pub struct TXEMPTY_R(crate::FieldReader<bool, TXEMPTY_A>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            false => TXEMPTY_A::_0,
            true => TXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPTY_A::_1
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, TXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Buffer Full Indicator (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFULL_A {
    #[doc = "0: Transmit FIFO buffer is not full"]
    _0 = 0,
    #[doc = "1: Transmit FIFO buffer is full"]
    _1 = 1,
}
impl From<TXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TXFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFULL` reader - Transmit FIFO Buffer Full Indicator (Read Only)"]
pub struct TXFULL_R(crate::FieldReader<bool, TXFULL_A>);
impl TXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFULL_A {
        match self.bits {
            false => TXFULL_A::_0,
            true => TXFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXFULL_A::_1
    }
}
impl core::ops::Deref for TXFULL_R {
    type Target = crate::FieldReader<bool, TXFULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Threshold Interrupt Flag (Read Only)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTHIF_A {
    #[doc = "0: The valid data count within the transmit FIFO buffer is larger than the setting value of TXTH"]
    _0 = 0,
    #[doc = "1: The valid data count within the transmit FIFO buffer is less than or equal to the setting value of TXTH"]
    _1 = 1,
}
impl From<TXTHIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXTHIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTHIF` reader - Transmit FIFO Threshold Interrupt Flag (Read Only)"]
pub struct TXTHIF_R(crate::FieldReader<bool, TXTHIF_A>);
impl TXTHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTHIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTHIF_A {
        match self.bits {
            false => TXTHIF_A::_0,
            true => TXTHIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXTHIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXTHIF_A::_1
    }
}
impl core::ops::Deref for TXTHIF_R {
    type Target = crate::FieldReader<bool, TXTHIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFIF` reader - Transmit FIFO Underflow Interrupt Flag\\nWhen the transmit FIFO buffer is empty and there is no datum written into the FIFO buffer, if there is more bus clock input, this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct TXUFIF_R(crate::FieldReader<bool, bool>);
impl TXUFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUFIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFIF` writer - Transmit FIFO Underflow Interrupt Flag\\nWhen the transmit FIFO buffer is empty and there is no datum written into the FIFO buffer, if there is more bus clock input, this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct TXUFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Right Channel Zero Cross Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RZCIF_A {
    #[doc = "0: No zero cross event occurred on right channel"]
    _0 = 0,
    #[doc = "1: Zero cross event occurred on right channel"]
    _1 = 1,
}
impl From<RZCIF_A> for bool {
    #[inline(always)]
    fn from(variant: RZCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RZCIF` reader - Right Channel Zero Cross Interrupt Flag"]
pub struct RZCIF_R(crate::FieldReader<bool, RZCIF_A>);
impl RZCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RZCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RZCIF_A {
        match self.bits {
            false => RZCIF_A::_0,
            true => RZCIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RZCIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RZCIF_A::_1
    }
}
impl core::ops::Deref for RZCIF_R {
    type Target = crate::FieldReader<bool, RZCIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RZCIF` writer - Right Channel Zero Cross Interrupt Flag"]
pub struct RZCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RZCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RZCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No zero cross event occurred on right channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RZCIF_A::_0)
    }
    #[doc = "Zero cross event occurred on right channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RZCIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Left Channel Zero Cross Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LZCIF_A {
    #[doc = "0: No zero cross event occurred on left channel"]
    _0 = 0,
    #[doc = "1: Zero cross event occurred on left channel"]
    _1 = 1,
}
impl From<LZCIF_A> for bool {
    #[inline(always)]
    fn from(variant: LZCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LZCIF` reader - Left Channel Zero Cross Interrupt Flag"]
pub struct LZCIF_R(crate::FieldReader<bool, LZCIF_A>);
impl LZCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LZCIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LZCIF_A {
        match self.bits {
            false => LZCIF_A::_0,
            true => LZCIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LZCIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LZCIF_A::_1
    }
}
impl core::ops::Deref for LZCIF_R {
    type Target = crate::FieldReader<bool, LZCIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LZCIF` writer - Left Channel Zero Cross Interrupt Flag"]
pub struct LZCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LZCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LZCIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No zero cross event occurred on left channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LZCIF_A::_0)
    }
    #[doc = "Zero cross event occurred on left channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LZCIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 3 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXRST_A {
    #[doc = "0: The reset function of TXRST or RXRST is done"]
    _0 = 0,
    #[doc = "1: Doing the reset function of TXRST or RXRST"]
    _1 = 1,
}
impl From<TXRXRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRXRST` reader - TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 3 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done."]
pub struct TXRXRST_R(crate::FieldReader<bool, TXRXRST_A>);
impl TXRXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXRST_A {
        match self.bits {
            false => TXRXRST_A::_0,
            true => TXRXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRXRST_A::_1
    }
}
impl core::ops::Deref for TXRXRST_R {
    type Target = crate::FieldReader<bool, TXRXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCNT` reader - Receive FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of receive FIFO buffer."]
pub struct RXCNT_R(crate::FieldReader<u8, u8>);
impl RXCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCNT` reader - Transmit FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of transmit FIFO buffer."]
pub struct TXCNT_R(crate::FieldReader<u8, u8>);
impl TXCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Right Channel (Read Only)\\nThis bit indicates the current transmit data is belong to which channel."]
    #[inline(always)]
    pub fn right(&self) -> RIGHT_R {
        RIGHT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Buffer Empty Indicator (Read Only)"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Buffer Full Indicator (Read Only)"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO Threshold Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn rxthif(&self) -> RXTHIF_R {
        RXTHIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rxovif(&self) -> RXOVIF_R {
        RXOVIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Time-out Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rxtoif(&self) -> RXTOIF_R {
        RXTOIF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2S Enable Status (Read Only)\\nNote: The SPI/I2S peripheral clock is asynchronous with the system clock. In order to make sure the SPI/I2S controller logic is disabled, this bit indicates the real status of SPI/I2S controller logic for user."]
    #[inline(always)]
    pub fn i2sensts(&self) -> I2SENSTS_R {
        I2SENSTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO Buffer Empty Indicator (Read Only)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit FIFO Buffer Full Indicator (Read Only)"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO Threshold Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn txthif(&self) -> TXTHIF_R {
        TXTHIF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Flag\\nWhen the transmit FIFO buffer is empty and there is no datum written into the FIFO buffer, if there is more bus clock input, this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn txufif(&self) -> TXUFIF_R {
        TXUFIF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Right Channel Zero Cross Interrupt Flag"]
    #[inline(always)]
    pub fn rzcif(&self) -> RZCIF_R {
        RZCIF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Left Channel Zero Cross Interrupt Flag"]
    #[inline(always)]
    pub fn lzcif(&self) -> LZCIF_R {
        LZCIF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 3 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done."]
    #[inline(always)]
    pub fn txrxrst(&self) -> TXRXRST_R {
        TXRXRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Receive FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of receive FIFO buffer."]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Transmit FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of transmit FIFO buffer."]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rxovif(&mut self) -> RXOVIF_W {
        RXOVIF_W { w: self }
    }
    #[doc = "Bit 12 - Receive Time-out Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rxtoif(&mut self) -> RXTOIF_W {
        RXTOIF_W { w: self }
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Interrupt Flag\\nWhen the transmit FIFO buffer is empty and there is no datum written into the FIFO buffer, if there is more bus clock input, this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn txufif(&mut self) -> TXUFIF_W {
        TXUFIF_W { w: self }
    }
    #[doc = "Bit 20 - Right Channel Zero Cross Interrupt Flag"]
    #[inline(always)]
    pub fn rzcif(&mut self) -> RZCIF_W {
        RZCIF_W { w: self }
    }
    #[doc = "Bit 21 - Left Channel Zero Cross Interrupt Flag"]
    #[inline(always)]
    pub fn lzcif(&mut self) -> LZCIF_W {
        LZCIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2ssts](index.html) module"]
pub struct SPI_I2SSTS_SPEC;
impl crate::RegisterSpec for SPI_I2SSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2ssts::R](R) reader structure"]
impl crate::Readable for SPI_I2SSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_i2ssts::W](W) writer structure"]
impl crate::Writable for SPI_I2SSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_I2SSTS to value 0x0005_0100"]
impl crate::Resettable for SPI_I2SSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0100
    }
}
