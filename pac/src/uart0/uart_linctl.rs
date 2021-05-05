#[doc = "Register `UART_LINCTL` reader"]
pub struct R(crate::R<UART_LINCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LINCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_LINCTL_SPEC>> for R {
    fn from(reader: crate::R<UART_LINCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LINCTL` writer"]
pub struct W(crate::W<UART_LINCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LINCTL_SPEC>;
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
impl core::convert::From<crate::W<UART_LINCTL_SPEC>> for W {
    fn from(writer: crate::W<UART_LINCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LIN Slave Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVEN_A {
    #[doc = "0: LIN slave mode Disabled"]
    _0 = 0,
    #[doc = "1: LIN slave mode Enabled"]
    _1 = 1,
}
impl From<SLVEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - LIN Slave Mode Enable Bit"]
pub struct SLVEN_R(crate::FieldReader<bool, SLVEN_A>);
impl SLVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVEN_A {
        match self.bits {
            false => SLVEN_A::_0,
            true => SLVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVEN_A::_1
    }
}
impl core::ops::Deref for SLVEN_R {
    type Target = crate::FieldReader<bool, SLVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVEN` writer - LIN Slave Mode Enable Bit"]
pub struct SLVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN slave mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVEN_A::_0)
    }
    #[doc = "LIN slave mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVEN_A::_1)
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
#[doc = "LIN Slave Header Detection Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVHDEN_A {
    #[doc = "0: LIN slave header detection Disabled"]
    _0 = 0,
    #[doc = "1: LIN slave header detection Enabled"]
    _1 = 1,
}
impl From<SLVHDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVHDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVHDEN` reader - LIN Slave Header Detection Enable Bit"]
pub struct SLVHDEN_R(crate::FieldReader<bool, SLVHDEN_A>);
impl SLVHDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVHDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVHDEN_A {
        match self.bits {
            false => SLVHDEN_A::_0,
            true => SLVHDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVHDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVHDEN_A::_1
    }
}
impl core::ops::Deref for SLVHDEN_R {
    type Target = crate::FieldReader<bool, SLVHDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVHDEN` writer - LIN Slave Header Detection Enable Bit"]
pub struct SLVHDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVHDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVHDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN slave header detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVHDEN_A::_0)
    }
    #[doc = "LIN slave header detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVHDEN_A::_1)
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
#[doc = "LIN Slave Automatic Resynchronization Mode Enable Bit\\nNote2: When operation in Automatic Resynchronization mode, the baud rate setting must be mode2 (BAUDM1 (UART_BAUD \\[29\\]) and BAUDM0 (UART_BAUD \\[28\\]) must be 1).\\nNote3: The control and interactions of this field are explained in 6.13.5.9(Slave mode with automatic resynchronization).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVAREN_A {
    #[doc = "0: LIN automatic resynchronization Disabled"]
    _0 = 0,
    #[doc = "1: LIN automatic resynchronization Enabled"]
    _1 = 1,
}
impl From<SLVAREN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVAREN` reader - LIN Slave Automatic Resynchronization Mode Enable Bit\\nNote2: When operation in Automatic Resynchronization mode, the baud rate setting must be mode2 (BAUDM1 (UART_BAUD \\[29\\]) and BAUDM0 (UART_BAUD \\[28\\]) must be 1).\\nNote3: The control and interactions of this field are explained in 6.13.5.9(Slave mode with automatic resynchronization)."]
pub struct SLVAREN_R(crate::FieldReader<bool, SLVAREN_A>);
impl SLVAREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVAREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVAREN_A {
        match self.bits {
            false => SLVAREN_A::_0,
            true => SLVAREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVAREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVAREN_A::_1
    }
}
impl core::ops::Deref for SLVAREN_R {
    type Target = crate::FieldReader<bool, SLVAREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVAREN` writer - LIN Slave Automatic Resynchronization Mode Enable Bit\\nNote2: When operation in Automatic Resynchronization mode, the baud rate setting must be mode2 (BAUDM1 (UART_BAUD \\[29\\]) and BAUDM0 (UART_BAUD \\[28\\]) must be 1).\\nNote3: The control and interactions of this field are explained in 6.13.5.9(Slave mode with automatic resynchronization)."]
pub struct SLVAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVAREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVAREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN automatic resynchronization Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVAREN_A::_0)
    }
    #[doc = "LIN automatic resynchronization Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVAREN_A::_1)
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
#[doc = "LIN Slave Divider Update Method Enable Bit\\nNote2: This bit used for LIN Slave Automatic Resynchronization mode. (for Non-Automatic Resynchronization mode, this bit should be kept cleared) \\nNote3: The control and interactions of this field are explained in 6.13.5.9 (Slave mode with automatic resynchronization).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDUEN_A {
    #[doc = "0: UART_BAUD updated is written by software (if no automatic resynchronization update occurs at the same time)"]
    _0 = 0,
    #[doc = "1: UART_BAUD is updated at the next received character. User must set the bit before checksum reception"]
    _1 = 1,
}
impl From<SLVDUEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVDUEN` reader - LIN Slave Divider Update Method Enable Bit\\nNote2: This bit used for LIN Slave Automatic Resynchronization mode. (for Non-Automatic Resynchronization mode, this bit should be kept cleared) \\nNote3: The control and interactions of this field are explained in 6.13.5.9 (Slave mode with automatic resynchronization)."]
pub struct SLVDUEN_R(crate::FieldReader<bool, SLVDUEN_A>);
impl SLVDUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVDUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDUEN_A {
        match self.bits {
            false => SLVDUEN_A::_0,
            true => SLVDUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVDUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVDUEN_A::_1
    }
}
impl core::ops::Deref for SLVDUEN_R {
    type Target = crate::FieldReader<bool, SLVDUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVDUEN` writer - LIN Slave Divider Update Method Enable Bit\\nNote2: This bit used for LIN Slave Automatic Resynchronization mode. (for Non-Automatic Resynchronization mode, this bit should be kept cleared) \\nNote3: The control and interactions of this field are explained in 6.13.5.9 (Slave mode with automatic resynchronization)."]
pub struct SLVDUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART_BAUD updated is written by software (if no automatic resynchronization update occurs at the same time)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVDUEN_A::_0)
    }
    #[doc = "UART_BAUD is updated at the next received character. User must set the bit before checksum reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVDUEN_A::_1)
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
#[doc = "LIN Mute Mode Enable Bit\\nNote: The exit from mute mode condition and each control and interactions of this field are explained in 6.13.5.9 (LIN slave mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_A {
    #[doc = "0: LIN mute mode Disabled"]
    _0 = 0,
    #[doc = "1: LIN mute mode Enabled"]
    _1 = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTE` reader - LIN Mute Mode Enable Bit\\nNote: The exit from mute mode condition and each control and interactions of this field are explained in 6.13.5.9 (LIN slave mode)."]
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
#[doc = "Field `MUTE` writer - LIN Mute Mode Enable Bit\\nNote: The exit from mute mode condition and each control and interactions of this field are explained in 6.13.5.9 (LIN slave mode)."]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN mute mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUTE_A::_0)
    }
    #[doc = "LIN mute mode Enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "LIN TX Send Header Enable Bit\\nThe LIN TX header can be 'break field' or 'break and sync field' or 'break, sync and frame ID field', it is depend on setting HSEL (UART_LINCTL\\[23:22\\]).\\nNote1: These registers are shadow registers of SENDH (UART_ALTCTL \\[7\\]); user can read/write it by setting SENDH (UART_ALTCTL \\[7\\]) or SENDH (UART_LINCTL \\[8\\]).\\nNote2: When transmitter header field (it may be 'break' or 'break + sync' or 'break + sync + frame ID' selected by HSEL (UART_LINCTL\\[23:22\\]) field) transfer operation finished, this bit will be cleared automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENDH_A {
    #[doc = "0: Send LIN TX header Disabled"]
    _0 = 0,
    #[doc = "1: Send LIN TX header Enabled"]
    _1 = 1,
}
impl From<SENDH_A> for bool {
    #[inline(always)]
    fn from(variant: SENDH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENDH` reader - LIN TX Send Header Enable Bit\\nThe LIN TX header can be 'break field' or 'break and sync field' or 'break, sync and frame ID field', it is depend on setting HSEL (UART_LINCTL\\[23:22\\]).\\nNote1: These registers are shadow registers of SENDH (UART_ALTCTL \\[7\\]); user can read/write it by setting SENDH (UART_ALTCTL \\[7\\]) or SENDH (UART_LINCTL \\[8\\]).\\nNote2: When transmitter header field (it may be 'break' or 'break + sync' or 'break + sync + frame ID' selected by HSEL (UART_LINCTL\\[23:22\\]) field) transfer operation finished, this bit will be cleared automatically."]
pub struct SENDH_R(crate::FieldReader<bool, SENDH_A>);
impl SENDH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SENDH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENDH_A {
        match self.bits {
            false => SENDH_A::_0,
            true => SENDH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SENDH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SENDH_A::_1
    }
}
impl core::ops::Deref for SENDH_R {
    type Target = crate::FieldReader<bool, SENDH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENDH` writer - LIN TX Send Header Enable Bit\\nThe LIN TX header can be 'break field' or 'break and sync field' or 'break, sync and frame ID field', it is depend on setting HSEL (UART_LINCTL\\[23:22\\]).\\nNote1: These registers are shadow registers of SENDH (UART_ALTCTL \\[7\\]); user can read/write it by setting SENDH (UART_ALTCTL \\[7\\]) or SENDH (UART_LINCTL \\[8\\]).\\nNote2: When transmitter header field (it may be 'break' or 'break + sync' or 'break + sync + frame ID' selected by HSEL (UART_LINCTL\\[23:22\\]) field) transfer operation finished, this bit will be cleared automatically."]
pub struct SENDH_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENDH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Send LIN TX header Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENDH_A::_0)
    }
    #[doc = "Send LIN TX header Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENDH_A::_1)
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
#[doc = "LIN ID Parity Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDPEN_A {
    #[doc = "0: LIN frame ID parity Disabled"]
    _0 = 0,
    #[doc = "1: LIN frame ID parity Enabled"]
    _1 = 1,
}
impl From<IDPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IDPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDPEN` reader - LIN ID Parity Enable Bit"]
pub struct IDPEN_R(crate::FieldReader<bool, IDPEN_A>);
impl IDPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPEN_A {
        match self.bits {
            false => IDPEN_A::_0,
            true => IDPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDPEN_A::_1
    }
}
impl core::ops::Deref for IDPEN_R {
    type Target = crate::FieldReader<bool, IDPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDPEN` writer - LIN ID Parity Enable Bit"]
pub struct IDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN frame ID parity Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPEN_A::_0)
    }
    #[doc = "LIN frame ID parity Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPEN_A::_1)
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
#[doc = "LIN Break Detection Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKDETEN_A {
    #[doc = "0: LIN break detection Disabled "]
    _0 = 0,
    #[doc = "1: LIN break detection Enabled"]
    _1 = 1,
}
impl From<BRKDETEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRKDETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKDETEN` reader - LIN Break Detection Enable Bit"]
pub struct BRKDETEN_R(crate::FieldReader<bool, BRKDETEN_A>);
impl BRKDETEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKDETEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKDETEN_A {
        match self.bits {
            false => BRKDETEN_A::_0,
            true => BRKDETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKDETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKDETEN_A::_1
    }
}
impl core::ops::Deref for BRKDETEN_R {
    type Target = crate::FieldReader<bool, BRKDETEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKDETEN` writer - LIN Break Detection Enable Bit"]
pub struct BRKDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKDETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRKDETEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN break detection Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRKDETEN_A::_0)
    }
    #[doc = "LIN break detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRKDETEN_A::_1)
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
#[doc = "LIN Receiver Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFF_A {
    #[doc = "0: LIN receiver Enabled"]
    _0 = 0,
    #[doc = "1: LIN receiver Disabled"]
    _1 = 1,
}
impl From<RXOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOFF` reader - LIN Receiver Disable Bit"]
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
#[doc = "Field `RXOFF` writer - LIN Receiver Disable Bit"]
pub struct RXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LIN receiver Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFF_A::_0)
    }
    #[doc = "LIN receiver Disabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Bit Error Detect Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITERREN_A {
    #[doc = "0: Bit error detection function Disabled"]
    _0 = 0,
    #[doc = "1: Bit error detection Enabled"]
    _1 = 1,
}
impl From<BITERREN_A> for bool {
    #[inline(always)]
    fn from(variant: BITERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITERREN` reader - Bit Error Detect Enable Bit"]
pub struct BITERREN_R(crate::FieldReader<bool, BITERREN_A>);
impl BITERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BITERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITERREN_A {
        match self.bits {
            false => BITERREN_A::_0,
            true => BITERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BITERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BITERREN_A::_1
    }
}
impl core::ops::Deref for BITERREN_R {
    type Target = crate::FieldReader<bool, BITERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITERREN` writer - Bit Error Detect Enable Bit"]
pub struct BITERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BITERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit error detection function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BITERREN_A::_0)
    }
    #[doc = "Bit error detection Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BITERREN_A::_1)
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
#[doc = "Field `BRKFL` reader - LIN Break Field Length \\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: These registers are shadow registers of BRKFL, User can read/write it by setting BRKFL (UART_ALTCTL\\[3:0\\]) or BRKFL (UART_LINCTL\\[19:16\\]).\\nNote2: This break field length is BRKFL + 1."]
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
#[doc = "Field `BRKFL` writer - LIN Break Field Length \\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: These registers are shadow registers of BRKFL, User can read/write it by setting BRKFL (UART_ALTCTL\\[3:0\\]) or BRKFL (UART_LINCTL\\[19:16\\]).\\nNote2: This break field length is BRKFL + 1."]
pub struct BRKFL_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "LIN Break/Sync Delimiter Length \\nNote: This bit used for LIN master to sending header field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BSL_A {
    #[doc = "0: The LIN break/sync delimiter length is 1-bit time"]
    _0 = 0,
    #[doc = "1: The LIN break/sync delimiter length is 2-bit time"]
    _1 = 1,
    #[doc = "2: The LIN break/sync delimiter length is 3-bit time"]
    _2 = 2,
    #[doc = "3: The LIN break/sync delimiter length is 4-bit time"]
    _3 = 3,
}
impl From<BSL_A> for u8 {
    #[inline(always)]
    fn from(variant: BSL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BSL` reader - LIN Break/Sync Delimiter Length \\nNote: This bit used for LIN master to sending header field."]
pub struct BSL_R(crate::FieldReader<u8, BSL_A>);
impl BSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BSL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSL_A {
        match self.bits {
            0 => BSL_A::_0,
            1 => BSL_A::_1,
            2 => BSL_A::_2,
            3 => BSL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BSL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BSL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BSL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BSL_A::_3
    }
}
impl core::ops::Deref for BSL_R {
    type Target = crate::FieldReader<u8, BSL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSL` writer - LIN Break/Sync Delimiter Length \\nNote: This bit used for LIN master to sending header field."]
pub struct BSL_W<'a> {
    w: &'a mut W,
}
impl<'a> BSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The LIN break/sync delimiter length is 1-bit time"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSL_A::_0)
    }
    #[doc = "The LIN break/sync delimiter length is 2-bit time"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSL_A::_1)
    }
    #[doc = "The LIN break/sync delimiter length is 3-bit time"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BSL_A::_2)
    }
    #[doc = "The LIN break/sync delimiter length is 4-bit time"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BSL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "LIN Header Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSEL_A {
    #[doc = "0: The LIN header includes 'break field'"]
    _0 = 0,
    #[doc = "1: The LIN header includes 'break field' and 'sync field'"]
    _1 = 1,
    #[doc = "2: The LIN header includes 'break field', 'sync field' and 'frame ID field'"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<HSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HSEL` reader - LIN Header Select"]
pub struct HSEL_R(crate::FieldReader<u8, HSEL_A>);
impl HSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEL_A {
        match self.bits {
            0 => HSEL_A::_0,
            1 => HSEL_A::_1,
            2 => HSEL_A::_2,
            3 => HSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HSEL_A::_3
    }
}
impl core::ops::Deref for HSEL_R {
    type Target = crate::FieldReader<u8, HSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEL` writer - LIN Header Select"]
pub struct HSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The LIN header includes 'break field'"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSEL_A::_0)
    }
    #[doc = "The LIN header includes 'break field' and 'sync field'"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSEL_A::_1)
    }
    #[doc = "The LIN header includes 'break field', 'sync field' and 'frame ID field'"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HSEL_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PID` reader - LIN PID Bits\\nIf the parity generated by hardware, user fill ID0~ID5, (PID \\[29:24\\]
)hardware will calculate P0 (PID\\[30\\]) and P1 (PID\\[31\\]), otherwise user must filled frame ID and parity in this field.\\nNote1: User can fill any 8-bit value to this field and the bit 24 indicates ID0 (LSB first).\\nNote2: This field can be used for LIN master mode or slave mode."]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - LIN PID Bits\\nIf the parity generated by hardware, user fill ID0~ID5, (PID \\[29:24\\]
)hardware will calculate P0 (PID\\[30\\]) and P1 (PID\\[31\\]), otherwise user must filled frame ID and parity in this field.\\nNote1: User can fill any 8-bit value to this field and the bit 24 indicates ID0 (LSB first).\\nNote2: This field can be used for LIN master mode or slave mode."]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LIN Slave Mode Enable Bit"]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LIN Slave Header Detection Enable Bit"]
    #[inline(always)]
    pub fn slvhden(&self) -> SLVHDEN_R {
        SLVHDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LIN Slave Automatic Resynchronization Mode Enable Bit\\nNote2: When operation in Automatic Resynchronization mode, the baud rate setting must be mode2 (BAUDM1 (UART_BAUD \\[29\\]) and BAUDM0 (UART_BAUD \\[28\\]) must be 1).\\nNote3: The control and interactions of this field are explained in 6.13.5.9(Slave mode with automatic resynchronization)."]
    #[inline(always)]
    pub fn slvaren(&self) -> SLVAREN_R {
        SLVAREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LIN Slave Divider Update Method Enable Bit\\nNote2: This bit used for LIN Slave Automatic Resynchronization mode. (for Non-Automatic Resynchronization mode, this bit should be kept cleared) \\nNote3: The control and interactions of this field are explained in 6.13.5.9 (Slave mode with automatic resynchronization)."]
    #[inline(always)]
    pub fn slvduen(&self) -> SLVDUEN_R {
        SLVDUEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LIN Mute Mode Enable Bit\\nNote: The exit from mute mode condition and each control and interactions of this field are explained in 6.13.5.9 (LIN slave mode)."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN TX Send Header Enable Bit\\nThe LIN TX header can be 'break field' or 'break and sync field' or 'break, sync and frame ID field', it is depend on setting HSEL (UART_LINCTL\\[23:22\\]).\\nNote1: These registers are shadow registers of SENDH (UART_ALTCTL \\[7\\]); user can read/write it by setting SENDH (UART_ALTCTL \\[7\\]) or SENDH (UART_LINCTL \\[8\\]).\\nNote2: When transmitter header field (it may be 'break' or 'break + sync' or 'break + sync + frame ID' selected by HSEL (UART_LINCTL\\[23:22\\]) field) transfer operation finished, this bit will be cleared automatically."]
    #[inline(always)]
    pub fn sendh(&self) -> SENDH_R {
        SENDH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LIN ID Parity Enable Bit"]
    #[inline(always)]
    pub fn idpen(&self) -> IDPEN_R {
        IDPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LIN Break Detection Enable Bit"]
    #[inline(always)]
    pub fn brkdeten(&self) -> BRKDETEN_R {
        BRKDETEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LIN Receiver Disable Bit"]
    #[inline(always)]
    pub fn rxoff(&self) -> RXOFF_R {
        RXOFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bit Error Detect Enable Bit"]
    #[inline(always)]
    pub fn biterren(&self) -> BITERREN_R {
        BITERREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - LIN Break Field Length \\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: These registers are shadow registers of BRKFL, User can read/write it by setting BRKFL (UART_ALTCTL\\[3:0\\]) or BRKFL (UART_LINCTL\\[19:16\\]).\\nNote2: This break field length is BRKFL + 1."]
    #[inline(always)]
    pub fn brkfl(&self) -> BRKFL_R {
        BRKFL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - LIN Break/Sync Delimiter Length \\nNote: This bit used for LIN master to sending header field."]
    #[inline(always)]
    pub fn bsl(&self) -> BSL_R {
        BSL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - LIN Header Select"]
    #[inline(always)]
    pub fn hsel(&self) -> HSEL_R {
        HSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - LIN PID Bits\\nIf the parity generated by hardware, user fill ID0~ID5, (PID \\[29:24\\]
)hardware will calculate P0 (PID\\[30\\]) and P1 (PID\\[31\\]), otherwise user must filled frame ID and parity in this field.\\nNote1: User can fill any 8-bit value to this field and the bit 24 indicates ID0 (LSB first).\\nNote2: This field can be used for LIN master mode or slave mode."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LIN Slave Mode Enable Bit"]
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W {
        SLVEN_W { w: self }
    }
    #[doc = "Bit 1 - LIN Slave Header Detection Enable Bit"]
    #[inline(always)]
    pub fn slvhden(&mut self) -> SLVHDEN_W {
        SLVHDEN_W { w: self }
    }
    #[doc = "Bit 2 - LIN Slave Automatic Resynchronization Mode Enable Bit\\nNote2: When operation in Automatic Resynchronization mode, the baud rate setting must be mode2 (BAUDM1 (UART_BAUD \\[29\\]) and BAUDM0 (UART_BAUD \\[28\\]) must be 1).\\nNote3: The control and interactions of this field are explained in 6.13.5.9(Slave mode with automatic resynchronization)."]
    #[inline(always)]
    pub fn slvaren(&mut self) -> SLVAREN_W {
        SLVAREN_W { w: self }
    }
    #[doc = "Bit 3 - LIN Slave Divider Update Method Enable Bit\\nNote2: This bit used for LIN Slave Automatic Resynchronization mode. (for Non-Automatic Resynchronization mode, this bit should be kept cleared) \\nNote3: The control and interactions of this field are explained in 6.13.5.9 (Slave mode with automatic resynchronization)."]
    #[inline(always)]
    pub fn slvduen(&mut self) -> SLVDUEN_W {
        SLVDUEN_W { w: self }
    }
    #[doc = "Bit 4 - LIN Mute Mode Enable Bit\\nNote: The exit from mute mode condition and each control and interactions of this field are explained in 6.13.5.9 (LIN slave mode)."]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bit 8 - LIN TX Send Header Enable Bit\\nThe LIN TX header can be 'break field' or 'break and sync field' or 'break, sync and frame ID field', it is depend on setting HSEL (UART_LINCTL\\[23:22\\]).\\nNote1: These registers are shadow registers of SENDH (UART_ALTCTL \\[7\\]); user can read/write it by setting SENDH (UART_ALTCTL \\[7\\]) or SENDH (UART_LINCTL \\[8\\]).\\nNote2: When transmitter header field (it may be 'break' or 'break + sync' or 'break + sync + frame ID' selected by HSEL (UART_LINCTL\\[23:22\\]) field) transfer operation finished, this bit will be cleared automatically."]
    #[inline(always)]
    pub fn sendh(&mut self) -> SENDH_W {
        SENDH_W { w: self }
    }
    #[doc = "Bit 9 - LIN ID Parity Enable Bit"]
    #[inline(always)]
    pub fn idpen(&mut self) -> IDPEN_W {
        IDPEN_W { w: self }
    }
    #[doc = "Bit 10 - LIN Break Detection Enable Bit"]
    #[inline(always)]
    pub fn brkdeten(&mut self) -> BRKDETEN_W {
        BRKDETEN_W { w: self }
    }
    #[doc = "Bit 11 - LIN Receiver Disable Bit"]
    #[inline(always)]
    pub fn rxoff(&mut self) -> RXOFF_W {
        RXOFF_W { w: self }
    }
    #[doc = "Bit 12 - Bit Error Detect Enable Bit"]
    #[inline(always)]
    pub fn biterren(&mut self) -> BITERREN_W {
        BITERREN_W { w: self }
    }
    #[doc = "Bits 16:19 - LIN Break Field Length \\nThis field indicates a 4-bit LIN TX break field count.\\nNote1: These registers are shadow registers of BRKFL, User can read/write it by setting BRKFL (UART_ALTCTL\\[3:0\\]) or BRKFL (UART_LINCTL\\[19:16\\]).\\nNote2: This break field length is BRKFL + 1."]
    #[inline(always)]
    pub fn brkfl(&mut self) -> BRKFL_W {
        BRKFL_W { w: self }
    }
    #[doc = "Bits 20:21 - LIN Break/Sync Delimiter Length \\nNote: This bit used for LIN master to sending header field."]
    #[inline(always)]
    pub fn bsl(&mut self) -> BSL_W {
        BSL_W { w: self }
    }
    #[doc = "Bits 22:23 - LIN Header Select"]
    #[inline(always)]
    pub fn hsel(&mut self) -> HSEL_W {
        HSEL_W { w: self }
    }
    #[doc = "Bits 24:31 - LIN PID Bits\\nIf the parity generated by hardware, user fill ID0~ID5, (PID \\[29:24\\]
)hardware will calculate P0 (PID\\[30\\]) and P1 (PID\\[31\\]), otherwise user must filled frame ID and parity in this field.\\nNote1: User can fill any 8-bit value to this field and the bit 24 indicates ID0 (LSB first).\\nNote2: This field can be used for LIN master mode or slave mode."]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART LIN Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_linctl](index.html) module"]
pub struct UART_LINCTL_SPEC;
impl crate::RegisterSpec for UART_LINCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_linctl::R](R) reader structure"]
impl crate::Readable for UART_LINCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_linctl::W](W) writer structure"]
impl crate::Writable for UART_LINCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LINCTL to value 0x000c_0000"]
impl crate::Resettable for UART_LINCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0000
    }
}
