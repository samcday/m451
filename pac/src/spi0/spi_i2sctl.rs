#[doc = "Register `SPI_I2SCTL` reader"]
pub struct R(crate::R<SPI_I2SCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2SCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_I2SCTL_SPEC>> for R {
    fn from(reader: crate::R<SPI_I2SCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_I2SCTL` writer"]
pub struct W(crate::W<SPI_I2SCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_I2SCTL_SPEC>;
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
impl core::convert::From<crate::W<SPI_I2SCTL_SPEC>> for W {
    fn from(writer: crate::W<SPI_I2SCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2S Controller Enable Bit\\nNote: If set this bit to 1, I2Sn_BCLK will start to output in master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SEN_A {
    #[doc = "0: I2S controller Disabled"]
    _0 = 0,
    #[doc = "1: I2S controller Enabled"]
    _1 = 1,
}
impl From<I2SEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2SEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SEN` reader - I2S Controller Enable Bit\\nNote: If set this bit to 1, I2Sn_BCLK will start to output in master mode."]
pub struct I2SEN_R(crate::FieldReader<bool, I2SEN_A>);
impl I2SEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SEN_A {
        match self.bits {
            false => I2SEN_A::_0,
            true => I2SEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2SEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2SEN_A::_1
    }
}
impl core::ops::Deref for I2SEN_R {
    type Target = crate::FieldReader<bool, I2SEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SEN` writer - I2S Controller Enable Bit\\nNote: If set this bit to 1, I2Sn_BCLK will start to output in master mode."]
pub struct I2SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2S controller Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2SEN_A::_0)
    }
    #[doc = "I2S controller Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2SEN_A::_1)
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
#[doc = "Transmit Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_A {
    #[doc = "0: Data transmit Disabled"]
    _0 = 0,
    #[doc = "1: Data transmit Enabled"]
    _1 = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` reader - Transmit Enable Bit"]
pub struct TXEN_R(crate::FieldReader<bool, TXEN_A>);
impl TXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::_0,
            true => TXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEN_A::_1
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, TXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` writer - Transmit Enable Bit"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data transmit Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEN_A::_0)
    }
    #[doc = "Data transmit Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEN_A::_1)
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
#[doc = "Receive Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_A {
    #[doc = "0: Data receive Disabled"]
    _0 = 0,
    #[doc = "1: Data receive Enabled"]
    _1 = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Receive Enable Bit"]
pub struct RXEN_R(crate::FieldReader<bool, RXEN_A>);
impl RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::_0,
            true => RXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEN_A::_1
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, RXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` writer - Receive Enable Bit"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data receive Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEN_A::_0)
    }
    #[doc = "Data receive Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEN_A::_1)
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
#[doc = "Transmit Mute Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_A {
    #[doc = "0: Transmit data is shifted out from buffer"]
    _0 = 0,
    #[doc = "1: Transmit data is zero"]
    _1 = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTE` reader - Transmit Mute Enable Bit"]
pub struct MUTE_R(crate::FieldReader<bool, MUTE_A>);
impl MUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_A {
        match self.bits {
            false => MUTE_A::_0,
            true => MUTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MUTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MUTE_A::_1
    }
}
impl core::ops::Deref for MUTE_R {
    type Target = crate::FieldReader<bool, MUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTE` writer - Transmit Mute Enable Bit"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit data is shifted out from buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUTE_A::_0)
    }
    #[doc = "Transmit data is zero"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUTE_A::_1)
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
#[doc = "Word Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDWIDTH_A {
    #[doc = "0: data size is 8-bit"]
    _0 = 0,
    #[doc = "1: data size is 16-bit"]
    _1 = 1,
    #[doc = "2: data size is 24-bit"]
    _2 = 2,
    #[doc = "3: data size is 32-bit"]
    _3 = 3,
}
impl From<WDWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WDWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDWIDTH` reader - Word Width"]
pub struct WDWIDTH_R(crate::FieldReader<u8, WDWIDTH_A>);
impl WDWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDWIDTH_A {
        match self.bits {
            0 => WDWIDTH_A::_0,
            1 => WDWIDTH_A::_1,
            2 => WDWIDTH_A::_2,
            3 => WDWIDTH_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDWIDTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDWIDTH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == WDWIDTH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == WDWIDTH_A::_3
    }
}
impl core::ops::Deref for WDWIDTH_R {
    type Target = crate::FieldReader<u8, WDWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDWIDTH` writer - Word Width"]
pub struct WDWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDWIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "data size is 8-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDWIDTH_A::_0)
    }
    #[doc = "data size is 16-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDWIDTH_A::_1)
    }
    #[doc = "data size is 24-bit"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WDWIDTH_A::_2)
    }
    #[doc = "data size is 32-bit"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WDWIDTH_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Monaural Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONO_A {
    #[doc = "0: Data is stereo format"]
    _0 = 0,
    #[doc = "1: Data is monaural format"]
    _1 = 1,
}
impl From<MONO_A> for bool {
    #[inline(always)]
    fn from(variant: MONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONO` reader - Monaural Data"]
pub struct MONO_R(crate::FieldReader<bool, MONO_A>);
impl MONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONO_A {
        match self.bits {
            false => MONO_A::_0,
            true => MONO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MONO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MONO_A::_1
    }
}
impl core::ops::Deref for MONO_R {
    type Target = crate::FieldReader<bool, MONO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO` writer - Monaural Data"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is stereo format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONO_A::_0)
    }
    #[doc = "Data is monaural format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONO_A::_1)
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
#[doc = "Stereo Data Order in FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORDER_A {
    #[doc = "0: Left channel data at high byte"]
    _0 = 0,
    #[doc = "1: Left channel data at low byte"]
    _1 = 1,
}
impl From<ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORDER` reader - Stereo Data Order in FIFO"]
pub struct ORDER_R(crate::FieldReader<bool, ORDER_A>);
impl ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ORDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDER_A {
        match self.bits {
            false => ORDER_A::_0,
            true => ORDER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ORDER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ORDER_A::_1
    }
}
impl core::ops::Deref for ORDER_R {
    type Target = crate::FieldReader<bool, ORDER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORDER` writer - Stereo Data Order in FIFO"]
pub struct ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORDER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left channel data at high byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORDER_A::_0)
    }
    #[doc = "Left channel data at low byte"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORDER_A::_1)
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
#[doc = "Slave Mode\\nI2S can operate as master or slave. For Master mode, I2Sn_BCLK and I2Sn_LRCLK pins are output mode and send bit clock from this chip to Audio CODEC chip. In Slave mode, I2Sn_BCLK and I2Sn_LRCLK pins are input mode and I2Sn_BCLK and I2Sn_LRCLK signals are received from outer Audio CODEC chip.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_A {
    #[doc = "0: Master mode"]
    _0 = 0,
    #[doc = "1: Slave mode"]
    _1 = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE` reader - Slave Mode\\nI2S can operate as master or slave. For Master mode, I2Sn_BCLK and I2Sn_LRCLK pins are output mode and send bit clock from this chip to Audio CODEC chip. In Slave mode, I2Sn_BCLK and I2Sn_LRCLK pins are input mode and I2Sn_BCLK and I2Sn_LRCLK signals are received from outer Audio CODEC chip."]
pub struct SLAVE_R(crate::FieldReader<bool, SLAVE_A>);
impl SLAVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::_0,
            true => SLAVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLAVE_A::_1
    }
}
impl core::ops::Deref for SLAVE_R {
    type Target = crate::FieldReader<bool, SLAVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE` writer - Slave Mode\\nI2S can operate as master or slave. For Master mode, I2Sn_BCLK and I2Sn_LRCLK pins are output mode and send bit clock from this chip to Audio CODEC chip. In Slave mode, I2Sn_BCLK and I2Sn_LRCLK pins are input mode and I2Sn_BCLK and I2Sn_LRCLK signals are received from outer Audio CODEC chip."]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLAVE_A::_0)
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLAVE_A::_1)
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
#[doc = "Master Clock Enable Bit\\nIf MCLKEN is set to 1, I2S controller will generate master clock on SPIn_I2SMCLK pin for external audio devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKEN_A {
    #[doc = "0: Master clock Disabled"]
    _0 = 0,
    #[doc = "1: Master clock Enabled"]
    _1 = 1,
}
impl From<MCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKEN` reader - Master Clock Enable Bit\\nIf MCLKEN is set to 1, I2S controller will generate master clock on SPIn_I2SMCLK pin for external audio devices."]
pub struct MCLKEN_R(crate::FieldReader<bool, MCLKEN_A>);
impl MCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKEN_A {
        match self.bits {
            false => MCLKEN_A::_0,
            true => MCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MCLKEN_A::_1
    }
}
impl core::ops::Deref for MCLKEN_R {
    type Target = crate::FieldReader<bool, MCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKEN` writer - Master Clock Enable Bit\\nIf MCLKEN is set to 1, I2S controller will generate master clock on SPIn_I2SMCLK pin for external audio devices."]
pub struct MCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCLKEN_A::_0)
    }
    #[doc = "Master clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCLKEN_A::_1)
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
#[doc = "Right Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when right channel data sign bit change or next shift data bits are all 0 then RZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RZCEN_A {
    #[doc = "0: Right channel zero cross detection Disabled"]
    _0 = 0,
    #[doc = "1: Right channel zero cross detection Enabled"]
    _1 = 1,
}
impl From<RZCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RZCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RZCEN` reader - Right Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when right channel data sign bit change or next shift data bits are all 0 then RZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
pub struct RZCEN_R(crate::FieldReader<bool, RZCEN_A>);
impl RZCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RZCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RZCEN_A {
        match self.bits {
            false => RZCEN_A::_0,
            true => RZCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RZCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RZCEN_A::_1
    }
}
impl core::ops::Deref for RZCEN_R {
    type Target = crate::FieldReader<bool, RZCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RZCEN` writer - Right Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when right channel data sign bit change or next shift data bits are all 0 then RZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
pub struct RZCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RZCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RZCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right channel zero cross detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RZCEN_A::_0)
    }
    #[doc = "Right channel zero cross detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RZCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Left Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when left channel data sign bit changes or next shift data bits are all 0 then LZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LZCEN_A {
    #[doc = "0: Left channel zero cross detection Disabled"]
    _0 = 0,
    #[doc = "1: Left channel zero cross detection Enabled"]
    _1 = 1,
}
impl From<LZCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LZCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LZCEN` reader - Left Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when left channel data sign bit changes or next shift data bits are all 0 then LZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
pub struct LZCEN_R(crate::FieldReader<bool, LZCEN_A>);
impl LZCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LZCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LZCEN_A {
        match self.bits {
            false => LZCEN_A::_0,
            true => LZCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LZCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LZCEN_A::_1
    }
}
impl core::ops::Deref for LZCEN_R {
    type Target = crate::FieldReader<bool, LZCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LZCEN` writer - Left Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when left channel data sign bit changes or next shift data bits are all 0 then LZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
pub struct LZCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LZCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LZCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left channel zero cross detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LZCEN_A::_0)
    }
    #[doc = "Left channel zero cross detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LZCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Receive Left Channel Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLCH_A {
    #[doc = "0: Receive right channel data in Mono mode"]
    _0 = 0,
    #[doc = "1: Receive left channel data in Mono mode"]
    _1 = 1,
}
impl From<RXLCH_A> for bool {
    #[inline(always)]
    fn from(variant: RXLCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLCH` reader - Receive Left Channel Enable Bit"]
pub struct RXLCH_R(crate::FieldReader<bool, RXLCH_A>);
impl RXLCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLCH_A {
        match self.bits {
            false => RXLCH_A::_0,
            true => RXLCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXLCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXLCH_A::_1
    }
}
impl core::ops::Deref for RXLCH_R {
    type Target = crate::FieldReader<bool, RXLCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLCH` writer - Receive Left Channel Enable Bit"]
pub struct RXLCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive right channel data in Mono mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXLCH_A::_0)
    }
    #[doc = "Receive left channel data in Mono mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXLCH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Right Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and right channel zero-cross event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RZCIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<RZCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RZCIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RZCIEN` reader - Right Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and right channel zero-cross event occurs."]
pub struct RZCIEN_R(crate::FieldReader<bool, RZCIEN_A>);
impl RZCIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RZCIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RZCIEN_A {
        match self.bits {
            false => RZCIEN_A::_0,
            true => RZCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RZCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RZCIEN_A::_1
    }
}
impl core::ops::Deref for RZCIEN_R {
    type Target = crate::FieldReader<bool, RZCIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RZCIEN` writer - Right Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and right channel zero-cross event occurs."]
pub struct RZCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RZCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RZCIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RZCIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RZCIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Left Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and left channel zero-cross event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LZCIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<LZCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: LZCIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LZCIEN` reader - Left Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and left channel zero-cross event occurs."]
pub struct LZCIEN_R(crate::FieldReader<bool, LZCIEN_A>);
impl LZCIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LZCIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LZCIEN_A {
        match self.bits {
            false => LZCIEN_A::_0,
            true => LZCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LZCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LZCIEN_A::_1
    }
}
impl core::ops::Deref for LZCIEN_R {
    type Target = crate::FieldReader<bool, LZCIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LZCIEN` writer - Left Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and left channel zero-cross event occurs."]
pub struct LZCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LZCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LZCIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LZCIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LZCIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Data Format Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: I2S data format"]
    _0 = 0,
    #[doc = "1: MSB justified data format"]
    _1 = 1,
    #[doc = "2: PCM mode A"]
    _2 = 2,
    #[doc = "3: PCM mode B"]
    _3 = 3,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORMAT` reader - Data Format Selection"]
pub struct FORMAT_R(crate::FieldReader<u8, FORMAT_A>);
impl FORMAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            0 => FORMAT_A::_0,
            1 => FORMAT_A::_1,
            2 => FORMAT_A::_2,
            3 => FORMAT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FORMAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FORMAT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == FORMAT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == FORMAT_A::_3
    }
}
impl core::ops::Deref for FORMAT_R {
    type Target = crate::FieldReader<u8, FORMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORMAT` writer - Data Format Selection"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I2S data format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORMAT_A::_0)
    }
    #[doc = "MSB justified data format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORMAT_A::_1)
    }
    #[doc = "PCM mode A"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FORMAT_A::_2)
    }
    #[doc = "PCM mode B"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(FORMAT_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2S Controller Enable Bit\\nNote: If set this bit to 1, I2Sn_BCLK will start to output in master mode."]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable Bit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Enable Bit"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Mute Enable Bit"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Word Width"]
    #[inline(always)]
    pub fn wdwidth(&self) -> WDWIDTH_R {
        WDWIDTH_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Monaural Data"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stereo Data Order in FIFO"]
    #[inline(always)]
    pub fn order(&self) -> ORDER_R {
        ORDER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Mode\\nI2S can operate as master or slave. For Master mode, I2Sn_BCLK and I2Sn_LRCLK pins are output mode and send bit clock from this chip to Audio CODEC chip. In Slave mode, I2Sn_BCLK and I2Sn_LRCLK pins are input mode and I2Sn_BCLK and I2Sn_LRCLK signals are received from outer Audio CODEC chip."]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Master Clock Enable Bit\\nIf MCLKEN is set to 1, I2S controller will generate master clock on SPIn_I2SMCLK pin for external audio devices."]
    #[inline(always)]
    pub fn mclken(&self) -> MCLKEN_R {
        MCLKEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Right Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when right channel data sign bit change or next shift data bits are all 0 then RZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
    #[inline(always)]
    pub fn rzcen(&self) -> RZCEN_R {
        RZCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Left Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when left channel data sign bit changes or next shift data bits are all 0 then LZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
    #[inline(always)]
    pub fn lzcen(&self) -> LZCEN_R {
        LZCEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Receive Left Channel Enable Bit"]
    #[inline(always)]
    pub fn rxlch(&self) -> RXLCH_R {
        RXLCH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Right Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and right channel zero-cross event occurs."]
    #[inline(always)]
    pub fn rzcien(&self) -> RZCIEN_R {
        RZCIEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Left Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and left channel zero-cross event occurs."]
    #[inline(always)]
    pub fn lzcien(&self) -> LZCIEN_R {
        LZCIEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Data Format Selection"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2S Controller Enable Bit\\nNote: If set this bit to 1, I2Sn_BCLK will start to output in master mode."]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Enable Bit"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 2 - Receive Enable Bit"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Mute Enable Bit"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bits 4:5 - Word Width"]
    #[inline(always)]
    pub fn wdwidth(&mut self) -> WDWIDTH_W {
        WDWIDTH_W { w: self }
    }
    #[doc = "Bit 6 - Monaural Data"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 7 - Stereo Data Order in FIFO"]
    #[inline(always)]
    pub fn order(&mut self) -> ORDER_W {
        ORDER_W { w: self }
    }
    #[doc = "Bit 8 - Slave Mode\\nI2S can operate as master or slave. For Master mode, I2Sn_BCLK and I2Sn_LRCLK pins are output mode and send bit clock from this chip to Audio CODEC chip. In Slave mode, I2Sn_BCLK and I2Sn_LRCLK pins are input mode and I2Sn_BCLK and I2Sn_LRCLK signals are received from outer Audio CODEC chip."]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 15 - Master Clock Enable Bit\\nIf MCLKEN is set to 1, I2S controller will generate master clock on SPIn_I2SMCLK pin for external audio devices."]
    #[inline(always)]
    pub fn mclken(&mut self) -> MCLKEN_W {
        MCLKEN_W { w: self }
    }
    #[doc = "Bit 16 - Right Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when right channel data sign bit change or next shift data bits are all 0 then RZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
    #[inline(always)]
    pub fn rzcen(&mut self) -> RZCEN_W {
        RZCEN_W { w: self }
    }
    #[doc = "Bit 17 - Left Channel Zero Cross Detection Enable Bit\\nIf this bit is set to 1, when left channel data sign bit changes or next shift data bits are all 0 then LZCIF flag in SPI_I2SSTS register will be set to 1. This function is only available in transmit operation."]
    #[inline(always)]
    pub fn lzcen(&mut self) -> LZCEN_W {
        LZCEN_W { w: self }
    }
    #[doc = "Bit 23 - Receive Left Channel Enable Bit"]
    #[inline(always)]
    pub fn rxlch(&mut self) -> RXLCH_W {
        RXLCH_W { w: self }
    }
    #[doc = "Bit 24 - Right Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and right channel zero-cross event occurs."]
    #[inline(always)]
    pub fn rzcien(&mut self) -> RZCIEN_W {
        RZCIEN_W { w: self }
    }
    #[doc = "Bit 25 - Left Channel Zero-cross Interrupt Enable Bit\\nInterrupt occurs if this bit is set to 1 and left channel zero-cross event occurs."]
    #[inline(always)]
    pub fn lzcien(&mut self) -> LZCIEN_W {
        LZCIEN_W { w: self }
    }
    #[doc = "Bits 28:29 - Data Format Selection"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2sctl](index.html) module"]
pub struct SPI_I2SCTL_SPEC;
impl crate::RegisterSpec for SPI_I2SCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2sctl::R](R) reader structure"]
impl crate::Readable for SPI_I2SCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_i2sctl::W](W) writer structure"]
impl crate::Writable for SPI_I2SCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_I2SCTL to value 0"]
impl crate::Resettable for SPI_I2SCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
