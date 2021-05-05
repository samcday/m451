#[doc = "Register `CAN_IF2_MCON` reader"]
pub struct R(crate::R<CAN_IF2_MCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_MCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_MCON_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_MCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_MCON` writer"]
pub struct W(crate::W<CAN_IF2_MCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_MCON_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_MCON_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_MCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data Length Code\\n0-8: Data Frame has 0-8 data bytes.\\n9-15: Data Frame has 8 data bytes\\nNote: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message.\\nData(0): 1st data byte of a CAN Data Frame\\nData(1): 2nd data byte of a CAN Data Frame\\nData(2): 3rd data byte of a CAN Data Frame\\nData(3): 4th data byte of a CAN Data Frame\\nData(4): 5th data byte of a CAN Data Frame\\nData(5): 6th data byte of a CAN Data Frame\\nData(6): 7th data byte of a CAN Data Frame\\nData(7): 8th data byte of a CAN Data Frame\\nNote: The Data(0) byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data(7) byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - Data Length Code\\n0-8: Data Frame has 0-8 data bytes.\\n9-15: Data Frame has 8 data bytes\\nNote: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message.\\nData(0): 1st data byte of a CAN Data Frame\\nData(1): 2nd data byte of a CAN Data Frame\\nData(2): 3rd data byte of a CAN Data Frame\\nData(3): 4th data byte of a CAN Data Frame\\nData(4): 5th data byte of a CAN Data Frame\\nData(5): 6th data byte of a CAN Data Frame\\nData(6): 7th data byte of a CAN Data Frame\\nData(7): 8th data byte of a CAN Data Frame\\nNote: The Data(0) byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data(7) byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "End of Buffer\\nNote: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOB_A {
    #[doc = "0: Message Object belongs to a FIFO Buffer and is not the last Message Object of that FIFO Buffer"]
    _0 = 0,
    #[doc = "1: Single Message Object or last Message Object of a FIFO Buffer"]
    _1 = 1,
}
impl From<EOB_A> for bool {
    #[inline(always)]
    fn from(variant: EOB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EoB` reader - End of Buffer\\nNote: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
pub struct EOB_R(crate::FieldReader<bool, EOB_A>);
impl EOB_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOB_A {
        match self.bits {
            false => EOB_A::_0,
            true => EOB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EOB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EOB_A::_1
    }
}
impl core::ops::Deref for EOB_R {
    type Target = crate::FieldReader<bool, EOB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EoB` writer - End of Buffer\\nNote: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
pub struct EOB_W<'a> {
    w: &'a mut W,
}
impl<'a> EOB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Message Object belongs to a FIFO Buffer and is not the last Message Object of that FIFO Buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOB_A::_0)
    }
    #[doc = "Single Message Object or last Message Object of a FIFO Buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOB_A::_1)
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
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQST_A {
    #[doc = "0: This Message Object is not waiting for transmission"]
    _0 = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done"]
    _1 = 1,
}
impl From<TXRQST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRQST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxRqst` reader - Transmit Request"]
pub struct TXRQST_R(crate::FieldReader<bool, TXRQST_A>);
impl TXRQST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRQST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRQST_A {
        match self.bits {
            false => TXRQST_A::_0,
            true => TXRQST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRQST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRQST_A::_1
    }
}
impl core::ops::Deref for TXRQST_R {
    type Target = crate::FieldReader<bool, TXRQST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxRqst` writer - Transmit Request"]
pub struct TXRQST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRQST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This Message Object is not waiting for transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRQST_A::_0)
    }
    #[doc = "The transmission of this Message Object is requested and is not yet done"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRQST_A::_1)
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
#[doc = "Remote Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMTEN_A {
    #[doc = "0: At the reception of a Remote Frame, TxRqst (CAN_IFn_MCON\\[8\\]) is left unchanged"]
    _0 = 0,
    #[doc = "1: At the reception of a Remote Frame, TxRqst is set"]
    _1 = 1,
}
impl From<RMTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RMTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RmtEn` reader - Remote Enable Bit"]
pub struct RMTEN_R(crate::FieldReader<bool, RMTEN_A>);
impl RMTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMTEN_A {
        match self.bits {
            false => RMTEN_A::_0,
            true => RMTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RMTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RMTEN_A::_1
    }
}
impl core::ops::Deref for RMTEN_R {
    type Target = crate::FieldReader<bool, RMTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RmtEn` writer - Remote Enable Bit"]
pub struct RMTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "At the reception of a Remote Frame, TxRqst (CAN_IFn_MCON\\[8\\]) is left unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMTEN_A::_0)
    }
    #[doc = "At the reception of a Remote Frame, TxRqst is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMTEN_A::_1)
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
#[doc = "Receive Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    #[doc = "0: IntPnd (CAN_IFn_MCON\\[13\\]) will be left unchanged after a successful reception of a frame"]
    _0 = 0,
    #[doc = "1: IntPnd will be set after a successful reception of a frame"]
    _1 = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RxIE` reader - Receive Interrupt Enable Bit"]
pub struct RXIE_R(crate::FieldReader<bool, RXIE_A>);
impl RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::_0,
            true => RXIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXIE_A::_1
    }
}
impl core::ops::Deref for RXIE_R {
    type Target = crate::FieldReader<bool, RXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxIE` writer - Receive Interrupt Enable Bit"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IntPnd (CAN_IFn_MCON\\[13\\]) will be left unchanged after a successful reception of a frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXIE_A::_0)
    }
    #[doc = "IntPnd will be set after a successful reception of a frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXIE_A::_1)
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
#[doc = "Transmit Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    #[doc = "0: IntPnd (CAN_IFn_MCON\\[13\\]) will be left unchanged after the successful transmission of a frame"]
    _0 = 0,
    #[doc = "1: IntPnd will be set after a successful transmission of a frame"]
    _1 = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxIE` reader - Transmit Interrupt Enable Bit"]
pub struct TXIE_R(crate::FieldReader<bool, TXIE_A>);
impl TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::_0,
            true => TXIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXIE_A::_1
    }
}
impl core::ops::Deref for TXIE_R {
    type Target = crate::FieldReader<bool, TXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxIE` writer - Transmit Interrupt Enable Bit"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IntPnd (CAN_IFn_MCON\\[13\\]) will be left unchanged after the successful transmission of a frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXIE_A::_0)
    }
    #[doc = "IntPnd will be set after a successful transmission of a frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Use Acceptance Mask\\nNote: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal bit (CAN_IFn_ARB2\\[15\\]) is set to one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMASK_A {
    #[doc = "0: Mask ignored"]
    _0 = 0,
    #[doc = "1: Use Mask (Msk28-0, MXtd, and MDir) for acceptance filtering"]
    _1 = 1,
}
impl From<UMASK_A> for bool {
    #[inline(always)]
    fn from(variant: UMASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UMask` reader - Use Acceptance Mask\\nNote: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal bit (CAN_IFn_ARB2\\[15\\]) is set to one."]
pub struct UMASK_R(crate::FieldReader<bool, UMASK_A>);
impl UMASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UMASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UMASK_A {
        match self.bits {
            false => UMASK_A::_0,
            true => UMASK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UMASK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UMASK_A::_1
    }
}
impl core::ops::Deref for UMASK_R {
    type Target = crate::FieldReader<bool, UMASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UMask` writer - Use Acceptance Mask\\nNote: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal bit (CAN_IFn_ARB2\\[15\\]) is set to one."]
pub struct UMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UMASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UMASK_A::_0)
    }
    #[doc = "Use Mask (Msk28-0, MXtd, and MDir) for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UMASK_A::_1)
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
#[doc = "Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPND_A {
    #[doc = "0: This message object is not the source of an interrupt"]
    _0 = 0,
    #[doc = "1: This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    _1 = 1,
}
impl From<INTPND_A> for bool {
    #[inline(always)]
    fn from(variant: INTPND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IntPnd` reader - Interrupt Pending"]
pub struct INTPND_R(crate::FieldReader<bool, INTPND_A>);
impl INTPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTPND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTPND_A {
        match self.bits {
            false => INTPND_A::_0,
            true => INTPND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTPND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTPND_A::_1
    }
}
impl core::ops::Deref for INTPND_R {
    type Target = crate::FieldReader<bool, INTPND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IntPnd` writer - Interrupt Pending"]
pub struct INTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTPND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This message object is not the source of an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTPND_A::_0)
    }
    #[doc = "This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTPND_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLST_A {
    #[doc = "0: No message lost since last time this bit was reset by the CPU"]
    _0 = 0,
    #[doc = "1: The Message Handler stored a new message into this object when NewDat was still set, the CPU has lost a message"]
    _1 = 1,
}
impl From<MSGLST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGLST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MsgLst` reader - "]
pub struct MSGLST_R(crate::FieldReader<bool, MSGLST_A>);
impl MSGLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSGLST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGLST_A {
        match self.bits {
            false => MSGLST_A::_0,
            true => MSGLST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSGLST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSGLST_A::_1
    }
}
impl core::ops::Deref for MSGLST_R {
    type Target = crate::FieldReader<bool, MSGLST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MsgLst` writer - "]
pub struct MSGLST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGLST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGLST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No message lost since last time this bit was reset by the CPU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSGLST_A::_0)
    }
    #[doc = "The Message Handler stored a new message into this object when NewDat was still set, the CPU has lost a message"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSGLST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "New Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEWDAT_A {
    #[doc = "0: No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the application software"]
    _0 = 0,
    #[doc = "1: The Message Handler or the application software has written new data into the data portion of this Message Object"]
    _1 = 1,
}
impl From<NEWDAT_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NewDat` reader - New Data"]
pub struct NEWDAT_R(crate::FieldReader<bool, NEWDAT_A>);
impl NEWDAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEWDAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEWDAT_A {
        match self.bits {
            false => NEWDAT_A::_0,
            true => NEWDAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEWDAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEWDAT_A::_1
    }
}
impl core::ops::Deref for NEWDAT_R {
    type Target = crate::FieldReader<bool, NEWDAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NewDat` writer - New Data"]
pub struct NEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWDAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEWDAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the application software"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEWDAT_A::_0)
    }
    #[doc = "The Message Handler or the application software has written new data into the data portion of this Message Object"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEWDAT_A::_1)
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
impl R {
    #[doc = "Bits 0:3 - Data Length Code\\n0-8: Data Frame has 0-8 data bytes.\\n9-15: Data Frame has 8 data bytes\\nNote: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message.\\nData(0): 1st data byte of a CAN Data Frame\\nData(1): 2nd data byte of a CAN Data Frame\\nData(2): 3rd data byte of a CAN Data Frame\\nData(3): 4th data byte of a CAN Data Frame\\nData(4): 5th data byte of a CAN Data Frame\\nData(5): 6th data byte of a CAN Data Frame\\nData(6): 7th data byte of a CAN Data Frame\\nData(7): 8th data byte of a CAN Data Frame\\nNote: The Data(0) byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data(7) byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer\\nNote: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
    #[inline(always)]
    pub fn eo_b(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn tx_rqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remote Enable Bit"]
    #[inline(always)]
    pub fn rmt_en(&self) -> RMTEN_R {
        RMTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rx_ie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable Bit"]
    #[inline(always)]
    pub fn tx_ie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask\\nNote: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal bit (CAN_IFn_ARB2\\[15\\]) is set to one."]
    #[inline(always)]
    pub fn umask(&self) -> UMASK_R {
        UMASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn int_pnd(&self) -> INTPND_R {
        INTPND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn msg_lst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn new_dat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code\\n0-8: Data Frame has 0-8 data bytes.\\n9-15: Data Frame has 8 data bytes\\nNote: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message.\\nData(0): 1st data byte of a CAN Data Frame\\nData(1): 2nd data byte of a CAN Data Frame\\nData(2): 3rd data byte of a CAN Data Frame\\nData(3): 4th data byte of a CAN Data Frame\\nData(4): 5th data byte of a CAN Data Frame\\nData(5): 6th data byte of a CAN Data Frame\\nData(6): 7th data byte of a CAN Data Frame\\nData(7): 8th data byte of a CAN Data Frame\\nNote: The Data(0) byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data(7) byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bit 7 - End of Buffer\\nNote: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
    #[inline(always)]
    pub fn eo_b(&mut self) -> EOB_W {
        EOB_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn tx_rqst(&mut self) -> TXRQST_W {
        TXRQST_W { w: self }
    }
    #[doc = "Bit 9 - Remote Enable Bit"]
    #[inline(always)]
    pub fn rmt_en(&mut self) -> RMTEN_W {
        RMTEN_W { w: self }
    }
    #[doc = "Bit 10 - Receive Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rx_ie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable Bit"]
    #[inline(always)]
    pub fn tx_ie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    #[doc = "Bit 12 - Use Acceptance Mask\\nNote: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal bit (CAN_IFn_ARB2\\[15\\]) is set to one."]
    #[inline(always)]
    pub fn umask(&mut self) -> UMASK_W {
        UMASK_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn int_pnd(&mut self) -> INTPND_W {
        INTPND_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn msg_lst(&mut self) -> MSGLST_W {
        MSGLST_W { w: self }
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn new_dat(&mut self) -> NEWDAT_W {
        NEWDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Message Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mcon](index.html) module"]
pub struct CAN_IF2_MCON_SPEC;
impl crate::RegisterSpec for CAN_IF2_MCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_mcon::R](R) reader structure"]
impl crate::Readable for CAN_IF2_MCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_mcon::W](W) writer structure"]
impl crate::Writable for CAN_IF2_MCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_MCON to value 0"]
impl crate::Resettable for CAN_IF2_MCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
