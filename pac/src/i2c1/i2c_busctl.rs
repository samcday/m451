#[doc = "Register `I2C_BUSCTL` reader"]
pub struct R(crate::R<I2C_BUSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_BUSCTL_SPEC>> for R {
    fn from(reader: crate::R<I2C_BUSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_BUSCTL` writer"]
pub struct W(crate::W<I2C_BUSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUSCTL_SPEC>;
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
impl core::convert::From<crate::W<I2C_BUSCTL_SPEC>> for W {
    fn from(writer: crate::W<I2C_BUSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Acknowledge Control by Manual\\nIn order to allow ACK control in slave reception including the command and data, slave byte control mode must be enabled by setting the ACKMEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKMEN_A {
    #[doc = "0: Slave byte control Disabled"]
    _0 = 0,
    #[doc = "1: Slave byte control Enabled. The 9th bit can response the ACK or NACK according the received data by user. When the byte is received, stretching the SCLK signal low between the 8th and 9th SCLK pulse"]
    _1 = 1,
}
impl From<ACKMEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKMEN` reader - Acknowledge Control by Manual\\nIn order to allow ACK control in slave reception including the command and data, slave byte control mode must be enabled by setting the ACKMEN bit."]
pub struct ACKMEN_R(crate::FieldReader<bool, ACKMEN_A>);
impl ACKMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKMEN_A {
        match self.bits {
            false => ACKMEN_A::_0,
            true => ACKMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACKMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACKMEN_A::_1
    }
}
impl core::ops::Deref for ACKMEN_R {
    type Target = crate::FieldReader<bool, ACKMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKMEN` writer - Acknowledge Control by Manual\\nIn order to allow ACK control in slave reception including the command and data, slave byte control mode must be enabled by setting the ACKMEN bit."]
pub struct ACKMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave byte control Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKMEN_A::_0)
    }
    #[doc = "Slave byte control Enabled. The 9th bit can response the ACK or NACK according the received data by user. When the byte is received, stretching the SCLK signal low between the 8th and 9th SCLK pulse"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKMEN_A::_1)
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
#[doc = "Packet Error Checking Calculation Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECEN_A {
    #[doc = "0: Packet Error Checking Calculation Disabled"]
    _0 = 0,
    #[doc = "1: Packet Error Checking Calculation Enabled"]
    _1 = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - Packet Error Checking Calculation Enable Bit"]
pub struct PECEN_R(crate::FieldReader<bool, PECEN_A>);
impl PECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::_0,
            true => PECEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PECEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PECEN_A::_1
    }
}
impl core::ops::Deref for PECEN_R {
    type Target = crate::FieldReader<bool, PECEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECEN` writer - Packet Error Checking Calculation Enable Bit"]
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Packet Error Checking Calculation Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECEN_A::_0)
    }
    #[doc = "Packet Error Checking Calculation Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECEN_A::_1)
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
#[doc = "Bus Management Device Default Address Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMDEN_A {
    #[doc = "0: Device default address Disable. When the address 0'b1100001x coming and the both of BMDEN and ACKMEN are enabled, the device responses NACKed"]
    _0 = 0,
    #[doc = "1: Device default address Enabled. When the address 0'b1100001x coming and the both of BMDEN and ACKMEN are enabled, the device responses ACKed"]
    _1 = 1,
}
impl From<BMDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BMDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMDEN` reader - Bus Management Device Default Address Enable Bit"]
pub struct BMDEN_R(crate::FieldReader<bool, BMDEN_A>);
impl BMDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMDEN_A {
        match self.bits {
            false => BMDEN_A::_0,
            true => BMDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BMDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BMDEN_A::_1
    }
}
impl core::ops::Deref for BMDEN_R {
    type Target = crate::FieldReader<bool, BMDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMDEN` writer - Bus Management Device Default Address Enable Bit"]
pub struct BMDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Device default address Disable. When the address 0'b1100001x coming and the both of BMDEN and ACKMEN are enabled, the device responses NACKed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMDEN_A::_0)
    }
    #[doc = "Device default address Enabled. When the address 0'b1100001x coming and the both of BMDEN and ACKMEN are enabled, the device responses ACKed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMDEN_A::_1)
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
#[doc = "Bus Management Host Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMHEN_A {
    #[doc = "0: Host function Disabled"]
    _0 = 0,
    #[doc = "1: Host function Enabled and the SUSCON will be used as CONTROL function"]
    _1 = 1,
}
impl From<BMHEN_A> for bool {
    #[inline(always)]
    fn from(variant: BMHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMHEN` reader - Bus Management Host Enable Bit"]
pub struct BMHEN_R(crate::FieldReader<bool, BMHEN_A>);
impl BMHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMHEN_A {
        match self.bits {
            false => BMHEN_A::_0,
            true => BMHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BMHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BMHEN_A::_1
    }
}
impl core::ops::Deref for BMHEN_R {
    type Target = crate::FieldReader<bool, BMHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMHEN` writer - Bus Management Host Enable Bit"]
pub struct BMHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMHEN_A::_0)
    }
    #[doc = "Host function Enabled and the SUSCON will be used as CONTROL function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMHEN_A::_1)
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
#[doc = "Bus Management Alert Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTEN_A {
    #[doc = "0: Release the BM_ALERT pin high and Alert Response Header disabled: 0001100x followed by NACK if both of BMDEN and ACKMEN are enabled.\\nBM_ALERT pin not supported"]
    _0 = 0,
    #[doc = "1: Drive BM_ALERT pin low and Alert Response Address Header enables: 0001100x followed by ACK if both of BMDEN and ACKMEN are enabled.\\nBM_ALERT pin supported"]
    _1 = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - Bus Management Alert Enable Bit"]
pub struct ALERTEN_R(crate::FieldReader<bool, ALERTEN_A>);
impl ALERTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALERTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::_0,
            true => ALERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALERTEN_A::_1
    }
}
impl core::ops::Deref for ALERTEN_R {
    type Target = crate::FieldReader<bool, ALERTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALERTEN` writer - Bus Management Alert Enable Bit"]
pub struct ALERTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Release the BM_ALERT pin high and Alert Response Header disabled: 0001100x followed by NACK if both of BMDEN and ACKMEN are enabled.\\nBM_ALERT pin not supported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERTEN_A::_0)
    }
    #[doc = "Drive BM_ALERT pin low and Alert Response Address Header enables: 0001100x followed by ACK if both of BMDEN and ACKMEN are enabled.\\nBM_ALERT pin supported"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERTEN_A::_1)
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
#[doc = "Suspend/Control Data Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTLOSTS_A {
    #[doc = "0: The output of SUSCON pin is low"]
    _0 = 0,
    #[doc = "1: The output of SUSCON pin is high"]
    _1 = 1,
}
impl From<SCTLOSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SCTLOSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTLOSTS` reader - Suspend/Control Data Output Status"]
pub struct SCTLOSTS_R(crate::FieldReader<bool, SCTLOSTS_A>);
impl SCTLOSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTLOSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTLOSTS_A {
        match self.bits {
            false => SCTLOSTS_A::_0,
            true => SCTLOSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCTLOSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCTLOSTS_A::_1
    }
}
impl core::ops::Deref for SCTLOSTS_R {
    type Target = crate::FieldReader<bool, SCTLOSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTLOSTS` writer - Suspend/Control Data Output Status"]
pub struct SCTLOSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTLOSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTLOSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output of SUSCON pin is low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCTLOSTS_A::_0)
    }
    #[doc = "The output of SUSCON pin is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCTLOSTS_A::_1)
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
#[doc = "Suspend or Control Pin Output Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTLOEN_A {
    #[doc = "0: The SUSCON pin in input"]
    _0 = 0,
    #[doc = "1: The output enable is active on the SUSCON pin"]
    _1 = 1,
}
impl From<SCTLOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCTLOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTLOEN` reader - Suspend or Control Pin Output Enable Bit"]
pub struct SCTLOEN_R(crate::FieldReader<bool, SCTLOEN_A>);
impl SCTLOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTLOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTLOEN_A {
        match self.bits {
            false => SCTLOEN_A::_0,
            true => SCTLOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCTLOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCTLOEN_A::_1
    }
}
impl core::ops::Deref for SCTLOEN_R {
    type Target = crate::FieldReader<bool, SCTLOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTLOEN` writer - Suspend or Control Pin Output Enable Bit"]
pub struct SCTLOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTLOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTLOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SUSCON pin in input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCTLOEN_A::_0)
    }
    #[doc = "The output enable is active on the SUSCON pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCTLOEN_A::_1)
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
#[doc = "BUS Enable Bit\\nNote: When the bit is enabled, the internal 14-bit counter is used to calculate the time out event of clock low condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSEN_A {
    #[doc = "0: The system management function is Disabled"]
    _0 = 0,
    #[doc = "1: The system management function is Enable"]
    _1 = 1,
}
impl From<BUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSEN` reader - BUS Enable Bit\\nNote: When the bit is enabled, the internal 14-bit counter is used to calculate the time out event of clock low condition."]
pub struct BUSEN_R(crate::FieldReader<bool, BUSEN_A>);
impl BUSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSEN_A {
        match self.bits {
            false => BUSEN_A::_0,
            true => BUSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSEN_A::_1
    }
}
impl core::ops::Deref for BUSEN_R {
    type Target = crate::FieldReader<bool, BUSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSEN` writer - BUS Enable Bit\\nNote: When the bit is enabled, the internal 14-bit counter is used to calculate the time out event of clock low condition."]
pub struct BUSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The system management function is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSEN_A::_0)
    }
    #[doc = "The system management function is Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSEN_A::_1)
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
#[doc = "Packet Error Checking Byte Transmission/Reception\\nThis bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address Matched is received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECTXEN_A {
    #[doc = "0: No PEC transfer"]
    _0 = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    _1 = 1,
}
impl From<PECTXEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECTXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTXEN` reader - Packet Error Checking Byte Transmission/Reception\\nThis bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address Matched is received"]
pub struct PECTXEN_R(crate::FieldReader<bool, PECTXEN_A>);
impl PECTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECTXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECTXEN_A {
        match self.bits {
            false => PECTXEN_A::_0,
            true => PECTXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PECTXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PECTXEN_A::_1
    }
}
impl core::ops::Deref for PECTXEN_R {
    type Target = crate::FieldReader<bool, PECTXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECTXEN` writer - Packet Error Checking Byte Transmission/Reception\\nThis bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address Matched is received"]
pub struct PECTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECTXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECTXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECTXEN_A::_0)
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECTXEN_A::_1)
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
#[doc = "Timer Check in Idle State\\nThe BUSTOUT is used to calculate the time-out of clock low in bus active and the idle period in bus Idle. This bit is used to define which condition is enabled.\\nNote: The BUSY (I2C_BUSSTS\\[0\\]) indicate the current bus state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIDLE_A {
    #[doc = "0: The BUSTOUT is used to calculate the clock low period in bus active"]
    _0 = 0,
    #[doc = "1: The BUSTOUT is used to calculate the IDLE period in bus Idle"]
    _1 = 1,
}
impl From<TIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIDLE` reader - Timer Check in Idle State\\nThe BUSTOUT is used to calculate the time-out of clock low in bus active and the idle period in bus Idle. This bit is used to define which condition is enabled.\\nNote: The BUSY (I2C_BUSSTS\\[0\\]) indicate the current bus state."]
pub struct TIDLE_R(crate::FieldReader<bool, TIDLE_A>);
impl TIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIDLE_A {
        match self.bits {
            false => TIDLE_A::_0,
            true => TIDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIDLE_A::_1
    }
}
impl core::ops::Deref for TIDLE_R {
    type Target = crate::FieldReader<bool, TIDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIDLE` writer - Timer Check in Idle State\\nThe BUSTOUT is used to calculate the time-out of clock low in bus active and the idle period in bus Idle. This bit is used to define which condition is enabled.\\nNote: The BUSY (I2C_BUSSTS\\[0\\]) indicate the current bus state."]
pub struct TIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIDLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The BUSTOUT is used to calculate the clock low period in bus active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIDLE_A::_0)
    }
    #[doc = "The BUSTOUT is used to calculate the IDLE period in bus Idle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIDLE_A::_1)
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
#[doc = "PEC Clear at Repeat Start\\nThe calculation of PEC starts when PECEN is set to 1 and it is clear when the STA or STO bit is detected. This PECCLR bit is used to enable the condition of REPEAT START can clear the PEC calculation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECCLR_A {
    #[doc = "0: The PEC calculation is cleared by 'Repeat Start' function is Disabled"]
    _0 = 0,
    #[doc = "1: The PEC calculation is cleared by 'Repeat Start' function is Enabled"]
    _1 = 1,
}
impl From<PECCLR_A> for bool {
    #[inline(always)]
    fn from(variant: PECCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECCLR` reader - PEC Clear at Repeat Start\\nThe calculation of PEC starts when PECEN is set to 1 and it is clear when the STA or STO bit is detected. This PECCLR bit is used to enable the condition of REPEAT START can clear the PEC calculation."]
pub struct PECCLR_R(crate::FieldReader<bool, PECCLR_A>);
impl PECCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECCLR_A {
        match self.bits {
            false => PECCLR_A::_0,
            true => PECCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PECCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PECCLR_A::_1
    }
}
impl core::ops::Deref for PECCLR_R {
    type Target = crate::FieldReader<bool, PECCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECCLR` writer - PEC Clear at Repeat Start\\nThe calculation of PEC starts when PECEN is set to 1 and it is clear when the STA or STO bit is detected. This PECCLR bit is used to enable the condition of REPEAT START can clear the PEC calculation."]
pub struct PECCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PECCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PEC calculation is cleared by 'Repeat Start' function is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECCLR_A::_0)
    }
    #[doc = "The PEC calculation is cleared by 'Repeat Start' function is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECCLR_A::_1)
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
#[doc = "Acknowledge Manual Enable Extra SI Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKM9SI_A {
    #[doc = "0: There is no SI interrupt in the 9th clock cycle when the BUSEN =1 and ACKMEN =1"]
    _0 = 0,
    #[doc = "1: There is SI interrupt in the 9th clock cycle when the BUSEN =1 and ACKMEN =1"]
    _1 = 1,
}
impl From<ACKM9SI_A> for bool {
    #[inline(always)]
    fn from(variant: ACKM9SI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKM9SI` reader - Acknowledge Manual Enable Extra SI Interrupt"]
pub struct ACKM9SI_R(crate::FieldReader<bool, ACKM9SI_A>);
impl ACKM9SI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKM9SI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKM9SI_A {
        match self.bits {
            false => ACKM9SI_A::_0,
            true => ACKM9SI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACKM9SI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACKM9SI_A::_1
    }
}
impl core::ops::Deref for ACKM9SI_R {
    type Target = crate::FieldReader<bool, ACKM9SI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKM9SI` writer - Acknowledge Manual Enable Extra SI Interrupt"]
pub struct ACKM9SI_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKM9SI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKM9SI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "There is no SI interrupt in the 9th clock cycle when the BUSEN =1 and ACKMEN =1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKM9SI_A::_0)
    }
    #[doc = "There is SI interrupt in the 9th clock cycle when the BUSEN =1 and ACKMEN =1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKM9SI_A::_1)
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
impl R {
    #[doc = "Bit 0 - Acknowledge Control by Manual\\nIn order to allow ACK control in slave reception including the command and data, slave byte control mode must be enabled by setting the ACKMEN bit."]
    #[inline(always)]
    pub fn ackmen(&self) -> ACKMEN_R {
        ACKMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Packet Error Checking Calculation Enable Bit"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Management Device Default Address Enable Bit"]
    #[inline(always)]
    pub fn bmden(&self) -> BMDEN_R {
        BMDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Management Host Enable Bit"]
    #[inline(always)]
    pub fn bmhen(&self) -> BMHEN_R {
        BMHEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bus Management Alert Enable Bit"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Suspend/Control Data Output Status"]
    #[inline(always)]
    pub fn sctlosts(&self) -> SCTLOSTS_R {
        SCTLOSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Suspend or Control Pin Output Enable Bit"]
    #[inline(always)]
    pub fn sctloen(&self) -> SCTLOEN_R {
        SCTLOEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BUS Enable Bit\\nNote: When the bit is enabled, the internal 14-bit counter is used to calculate the time out event of clock low condition."]
    #[inline(always)]
    pub fn busen(&self) -> BUSEN_R {
        BUSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Packet Error Checking Byte Transmission/Reception\\nThis bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address Matched is received"]
    #[inline(always)]
    pub fn pectxen(&self) -> PECTXEN_R {
        PECTXEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer Check in Idle State\\nThe BUSTOUT is used to calculate the time-out of clock low in bus active and the idle period in bus Idle. This bit is used to define which condition is enabled.\\nNote: The BUSY (I2C_BUSSTS\\[0\\]) indicate the current bus state."]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PEC Clear at Repeat Start\\nThe calculation of PEC starts when PECEN is set to 1 and it is clear when the STA or STO bit is detected. This PECCLR bit is used to enable the condition of REPEAT START can clear the PEC calculation."]
    #[inline(always)]
    pub fn pecclr(&self) -> PECCLR_R {
        PECCLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge Manual Enable Extra SI Interrupt"]
    #[inline(always)]
    pub fn ackm9si(&self) -> ACKM9SI_R {
        ACKM9SI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge Control by Manual\\nIn order to allow ACK control in slave reception including the command and data, slave byte control mode must be enabled by setting the ACKMEN bit."]
    #[inline(always)]
    pub fn ackmen(&mut self) -> ACKMEN_W {
        ACKMEN_W { w: self }
    }
    #[doc = "Bit 1 - Packet Error Checking Calculation Enable Bit"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 2 - Bus Management Device Default Address Enable Bit"]
    #[inline(always)]
    pub fn bmden(&mut self) -> BMDEN_W {
        BMDEN_W { w: self }
    }
    #[doc = "Bit 3 - Bus Management Host Enable Bit"]
    #[inline(always)]
    pub fn bmhen(&mut self) -> BMHEN_W {
        BMHEN_W { w: self }
    }
    #[doc = "Bit 4 - Bus Management Alert Enable Bit"]
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W {
        ALERTEN_W { w: self }
    }
    #[doc = "Bit 5 - Suspend/Control Data Output Status"]
    #[inline(always)]
    pub fn sctlosts(&mut self) -> SCTLOSTS_W {
        SCTLOSTS_W { w: self }
    }
    #[doc = "Bit 6 - Suspend or Control Pin Output Enable Bit"]
    #[inline(always)]
    pub fn sctloen(&mut self) -> SCTLOEN_W {
        SCTLOEN_W { w: self }
    }
    #[doc = "Bit 7 - BUS Enable Bit\\nNote: When the bit is enabled, the internal 14-bit counter is used to calculate the time out event of clock low condition."]
    #[inline(always)]
    pub fn busen(&mut self) -> BUSEN_W {
        BUSEN_W { w: self }
    }
    #[doc = "Bit 8 - Packet Error Checking Byte Transmission/Reception\\nThis bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address Matched is received"]
    #[inline(always)]
    pub fn pectxen(&mut self) -> PECTXEN_W {
        PECTXEN_W { w: self }
    }
    #[doc = "Bit 9 - Timer Check in Idle State\\nThe BUSTOUT is used to calculate the time-out of clock low in bus active and the idle period in bus Idle. This bit is used to define which condition is enabled.\\nNote: The BUSY (I2C_BUSSTS\\[0\\]) indicate the current bus state."]
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W {
        TIDLE_W { w: self }
    }
    #[doc = "Bit 10 - PEC Clear at Repeat Start\\nThe calculation of PEC starts when PECEN is set to 1 and it is clear when the STA or STO bit is detected. This PECCLR bit is used to enable the condition of REPEAT START can clear the PEC calculation."]
    #[inline(always)]
    pub fn pecclr(&mut self) -> PECCLR_W {
        PECCLR_W { w: self }
    }
    #[doc = "Bit 11 - Acknowledge Manual Enable Extra SI Interrupt"]
    #[inline(always)]
    pub fn ackm9si(&mut self) -> ACKM9SI_W {
        ACKM9SI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Management Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_busctl](index.html) module"]
pub struct I2C_BUSCTL_SPEC;
impl crate::RegisterSpec for I2C_BUSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_busctl::R](R) reader structure"]
impl crate::Readable for I2C_BUSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_busctl::W](W) writer structure"]
impl crate::Writable for I2C_BUSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_BUSCTL to value 0"]
impl crate::Resettable for I2C_BUSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
