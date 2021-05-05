#[doc = "Register `SPI_STATUS` reader"]
pub struct R(crate::R<SPI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SPI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_STATUS` writer"]
pub struct W(crate::W<SPI_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_STATUS_SPEC>;
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
impl core::convert::From<crate::W<SPI_STATUS_SPEC>> for W {
    fn from(writer: crate::W<SPI_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Busy Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SPI controller is in idle state"]
    _0 = 0,
    #[doc = "1: SPI controller is in busy state"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy Status (Read Only)"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Unit Transfer Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNITIF_A {
    #[doc = "0: No transaction has been finished since this bit was cleared to 0"]
    _0 = 0,
    #[doc = "1: SPI controller has finished one unit transfer"]
    _1 = 1,
}
impl From<UNITIF_A> for bool {
    #[inline(always)]
    fn from(variant: UNITIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNITIF` reader - Unit Transfer Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
pub struct UNITIF_R(crate::FieldReader<bool, UNITIF_A>);
impl UNITIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNITIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNITIF_A {
        match self.bits {
            false => UNITIF_A::_0,
            true => UNITIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UNITIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UNITIF_A::_1
    }
}
impl core::ops::Deref for UNITIF_R {
    type Target = crate::FieldReader<bool, UNITIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNITIF` writer - Unit Transfer Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
pub struct UNITIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UNITIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNITIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No transaction has been finished since this bit was cleared to 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNITIF_A::_0)
    }
    #[doc = "SPI controller has finished one unit transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNITIF_A::_1)
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
#[doc = "Slave Select Active Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACTIF_A {
    #[doc = "0: Slave select active interrupt be cleared or not occurrs"]
    _0 = 0,
    #[doc = "1: Slave select active interrupt event occurrs"]
    _1 = 1,
}
impl From<SSACTIF_A> for bool {
    #[inline(always)]
    fn from(variant: SSACTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACTIF` reader - Slave Select Active Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
pub struct SSACTIF_R(crate::FieldReader<bool, SSACTIF_A>);
impl SSACTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACTIF_A {
        match self.bits {
            false => SSACTIF_A::_0,
            true => SSACTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACTIF_A::_1
    }
}
impl core::ops::Deref for SSACTIF_R {
    type Target = crate::FieldReader<bool, SSACTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACTIF` writer - Slave Select Active Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
pub struct SSACTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select active interrupt be cleared or not occurrs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACTIF_A::_0)
    }
    #[doc = "Slave select active interrupt event occurrs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACTIF_A::_1)
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
#[doc = "Slave Select Inactive Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSINAIF_A {
    #[doc = "0: Slave select inactive interrupt be cleared or not occurrs"]
    _0 = 0,
    #[doc = "1: Slave select inactive interrupt event occurrs"]
    _1 = 1,
}
impl From<SSINAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SSINAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSINAIF` reader - Slave Select Inactive Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
pub struct SSINAIF_R(crate::FieldReader<bool, SSINAIF_A>);
impl SSINAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSINAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSINAIF_A {
        match self.bits {
            false => SSINAIF_A::_0,
            true => SSINAIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSINAIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSINAIF_A::_1
    }
}
impl core::ops::Deref for SSINAIF_R {
    type Target = crate::FieldReader<bool, SSINAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINAIF` writer - Slave Select Inactive Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
pub struct SSINAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSINAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select inactive interrupt be cleared or not occurrs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSINAIF_A::_0)
    }
    #[doc = "Slave select inactive interrupt event occurrs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSINAIF_A::_1)
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
#[doc = "Slave Select Line Bus Status (Read Only)\\nNote: This bit is only available in Slave mode. If SSACTPOL (SPI_SSCTL\\[2\\]) is set 0, and the SSLINE is 1, the SPI slave select is in inactive status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSLINE_A {
    #[doc = "0: The slave select line status is 0"]
    _0 = 0,
    #[doc = "1: The slave select line status is 1"]
    _1 = 1,
}
impl From<SSLINE_A> for bool {
    #[inline(always)]
    fn from(variant: SSLINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSLINE` reader - Slave Select Line Bus Status (Read Only)\\nNote: This bit is only available in Slave mode. If SSACTPOL (SPI_SSCTL\\[2\\]) is set 0, and the SSLINE is 1, the SPI slave select is in inactive status."]
pub struct SSLINE_R(crate::FieldReader<bool, SSLINE_A>);
impl SSLINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSLINE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSLINE_A {
        match self.bits {
            false => SSLINE_A::_0,
            true => SSLINE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSLINE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSLINE_A::_1
    }
}
impl core::ops::Deref for SSLINE_R {
    type Target = crate::FieldReader<bool, SSLINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Slave Time-out Interrupt Flag (Only Supported in SPI0)\\nWhen the slave select is active and the value of SLVTOCNT is not 0, as the bus clock is detected, the slave time-out counter in SPI controller logic will be started. When the value of time-out counter is greater than or equal to the value of SLVTOCNT (SPI_SSCTL\\[31:16\\]) before one transaction is done, the slave time-out interrupt event will be asserted.\\nNote: This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVTOIF_A {
    #[doc = "0: Slave time-out is not active"]
    _0 = 0,
    #[doc = "1: Slave time-out is active"]
    _1 = 1,
}
impl From<SLVTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVTOIF` reader - Slave Time-out Interrupt Flag (Only Supported in SPI0)\\nWhen the slave select is active and the value of SLVTOCNT is not 0, as the bus clock is detected, the slave time-out counter in SPI controller logic will be started. When the value of time-out counter is greater than or equal to the value of SLVTOCNT (SPI_SSCTL\\[31:16\\]) before one transaction is done, the slave time-out interrupt event will be asserted.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct SLVTOIF_R(crate::FieldReader<bool, SLVTOIF_A>);
impl SLVTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVTOIF_A {
        match self.bits {
            false => SLVTOIF_A::_0,
            true => SLVTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVTOIF_A::_1
    }
}
impl core::ops::Deref for SLVTOIF_R {
    type Target = crate::FieldReader<bool, SLVTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOIF` writer - Slave Time-out Interrupt Flag (Only Supported in SPI0)\\nWhen the slave select is active and the value of SLVTOCNT is not 0, as the bus clock is detected, the slave time-out counter in SPI controller logic will be started. When the value of time-out counter is greater than or equal to the value of SLVTOCNT (SPI_SSCTL\\[31:16\\]) before one transaction is done, the slave time-out interrupt event will be asserted.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct SLVTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVTOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave time-out is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVTOIF_A::_0)
    }
    #[doc = "Slave time-out is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVTOIF_A::_1)
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
#[doc = "Slave Mode Bit Count Error Interrupt Flag\\nIn Slave mode, when the slave select line goes to inactive state, if bit counter is mismatch with DWIDTH, this interrupt flag will be set to 1.\\nNote: If the slave select active but there is no any bus clock input, the SLVBCEIF also be set when the slave select goes to inactive state. This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVBEIF_A {
    #[doc = "0: No Slave mode bit count error event"]
    _0 = 0,
    #[doc = "1: Slave mode bit count error event occurs"]
    _1 = 1,
}
impl From<SLVBEIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVBEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVBEIF` reader - Slave Mode Bit Count Error Interrupt Flag\\nIn Slave mode, when the slave select line goes to inactive state, if bit counter is mismatch with DWIDTH, this interrupt flag will be set to 1.\\nNote: If the slave select active but there is no any bus clock input, the SLVBCEIF also be set when the slave select goes to inactive state. This bit will be cleared by writing 1 to it."]
pub struct SLVBEIF_R(crate::FieldReader<bool, SLVBEIF_A>);
impl SLVBEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVBEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVBEIF_A {
        match self.bits {
            false => SLVBEIF_A::_0,
            true => SLVBEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVBEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVBEIF_A::_1
    }
}
impl core::ops::Deref for SLVBEIF_R {
    type Target = crate::FieldReader<bool, SLVBEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVBEIF` writer - Slave Mode Bit Count Error Interrupt Flag\\nIn Slave mode, when the slave select line goes to inactive state, if bit counter is mismatch with DWIDTH, this interrupt flag will be set to 1.\\nNote: If the slave select active but there is no any bus clock input, the SLVBCEIF also be set when the slave select goes to inactive state. This bit will be cleared by writing 1 to it."]
pub struct SLVBEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVBEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVBEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Slave mode bit count error event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVBEIF_A::_0)
    }
    #[doc = "Slave mode bit count error event occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVBEIF_A::_1)
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
#[doc = "Slave Mode TX Under Run Interrupt Flag\\nIn Slave mode, if TX underflow event occurs and the slave select line goes to inactive state, this interrupt flag will be set to 1.\\nNote: This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVURIF_A {
    #[doc = "0: No Slave TX under run event"]
    _0 = 0,
    #[doc = "1: Slave TX under run event occurs"]
    _1 = 1,
}
impl From<SLVURIF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVURIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVURIF` reader - Slave Mode TX Under Run Interrupt Flag\\nIn Slave mode, if TX underflow event occurs and the slave select line goes to inactive state, this interrupt flag will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct SLVURIF_R(crate::FieldReader<bool, SLVURIF_A>);
impl SLVURIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVURIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVURIF_A {
        match self.bits {
            false => SLVURIF_A::_0,
            true => SLVURIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVURIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVURIF_A::_1
    }
}
impl core::ops::Deref for SLVURIF_R {
    type Target = crate::FieldReader<bool, SLVURIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVURIF` writer - Slave Mode TX Under Run Interrupt Flag\\nIn Slave mode, if TX underflow event occurs and the slave select line goes to inactive state, this interrupt flag will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct SLVURIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVURIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVURIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Slave TX under run event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVURIF_A::_0)
    }
    #[doc = "Slave TX under run event occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVURIF_A::_1)
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
    #[doc = "0: The valid data count within the RX FIFO buffer is smaller than or equal to the setting value of RXTH"]
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
#[doc = "Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVIF_A {
    #[doc = "0: Receive FIFO does not over run"]
    _0 = 0,
    #[doc = "1: Receive FIFO over run"]
    _1 = 1,
}
impl From<RXOVIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVIF` reader - Receive FIFO Overrun Interrupt Flag\\nWhen the receive FIFO buffer is full, the follow-up data will be dropped and this bit will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
pub struct RXOVIF_R(crate::FieldReader<bool, RXOVIF_A>);
impl RXOVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVIF_A {
        match self.bits {
            false => RXOVIF_A::_0,
            true => RXOVIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOVIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOVIF_A::_1
    }
}
impl core::ops::Deref for RXOVIF_R {
    type Target = crate::FieldReader<bool, RXOVIF_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO does not over run"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOVIF_A::_0)
    }
    #[doc = "Receive FIFO over run"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOVIF_A::_1)
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
#[doc = "SPI Enable Status (Read Only)\\nNote: The SPI peripheral clock is asynchronous with the system clock. In order to make sure the SPI control logic is disabled, this bit indicates the real status of SPI controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIENSTS_A {
    #[doc = "0: The SPI controller is disabled"]
    _0 = 0,
    #[doc = "1: The SPI controller is enabled"]
    _1 = 1,
}
impl From<SPIENSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SPIENSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIENSTS` reader - SPI Enable Status (Read Only)\\nNote: The SPI peripheral clock is asynchronous with the system clock. In order to make sure the SPI control logic is disabled, this bit indicates the real status of SPI controller."]
pub struct SPIENSTS_R(crate::FieldReader<bool, SPIENSTS_A>);
impl SPIENSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIENSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIENSTS_A {
        match self.bits {
            false => SPIENSTS_A::_0,
            true => SPIENSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPIENSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPIENSTS_A::_1
    }
}
impl core::ops::Deref for SPIENSTS_R {
    type Target = crate::FieldReader<bool, SPIENSTS_A>;
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
#[doc = "TX Underflow Interrupt Flag\\nWhen the TX underflow event occurs, this bit will be set to 1, the state of data output pin depends on the setting of TXUFPOL.\\nNote 1: This bit will be cleared by writing 1 to it.\\nNote 2: If reset slave's transmission circuit when slave selection signal is active, this flag will be set to 1 after 3 system clock cycles + 2 peripheral clock cycles since the reset operation is done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUFIF_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: No data in Transmit FIFO and TX shift register when the slave selection signal is active"]
    _1 = 1,
}
impl From<TXUFIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXUFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUFIF` reader - TX Underflow Interrupt Flag\\nWhen the TX underflow event occurs, this bit will be set to 1, the state of data output pin depends on the setting of TXUFPOL.\\nNote 1: This bit will be cleared by writing 1 to it.\\nNote 2: If reset slave's transmission circuit when slave selection signal is active, this flag will be set to 1 after 3 system clock cycles + 2 peripheral clock cycles since the reset operation is done."]
pub struct TXUFIF_R(crate::FieldReader<bool, TXUFIF_A>);
impl TXUFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUFIF_A {
        match self.bits {
            false => TXUFIF_A::_0,
            true => TXUFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXUFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXUFIF_A::_1
    }
}
impl core::ops::Deref for TXUFIF_R {
    type Target = crate::FieldReader<bool, TXUFIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFIF` writer - TX Underflow Interrupt Flag\\nWhen the TX underflow event occurs, this bit will be set to 1, the state of data output pin depends on the setting of TXUFPOL.\\nNote 1: This bit will be cleared by writing 1 to it.\\nNote 2: If reset slave's transmission circuit when slave selection signal is active, this flag will be set to 1 after 3 system clock cycles + 2 peripheral clock cycles since the reset operation is done."]
pub struct TXUFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUFIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUFIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXUFIF_A::_0)
    }
    #[doc = "No data in Transmit FIFO and TX shift register when the slave selection signal is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXUFIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 2 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done.\n\nValue on reset: 0"]
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
#[doc = "Field `TXRXRST` reader - TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 2 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done."]
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
    #[doc = "Bit 0 - Busy Status (Read Only)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Unit Transfer Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn unitif(&self) -> UNITIF_R {
        UNITIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Select Active Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn ssactif(&self) -> SSACTIF_R {
        SSACTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Select Inactive Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn ssinaif(&self) -> SSINAIF_R {
        SSINAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select Line Bus Status (Read Only)\\nNote: This bit is only available in Slave mode. If SSACTPOL (SPI_SSCTL\\[2\\]) is set 0, and the SSLINE is 1, the SPI slave select is in inactive status."]
    #[inline(always)]
    pub fn ssline(&self) -> SSLINE_R {
        SSLINE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Time-out Interrupt Flag (Only Supported in SPI0)\\nWhen the slave select is active and the value of SLVTOCNT is not 0, as the bus clock is detected, the slave time-out counter in SPI controller logic will be started. When the value of time-out counter is greater than or equal to the value of SLVTOCNT (SPI_SSCTL\\[31:16\\]) before one transaction is done, the slave time-out interrupt event will be asserted.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvtoif(&self) -> SLVTOIF_R {
        SLVTOIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slave Mode Bit Count Error Interrupt Flag\\nIn Slave mode, when the slave select line goes to inactive state, if bit counter is mismatch with DWIDTH, this interrupt flag will be set to 1.\\nNote: If the slave select active but there is no any bus clock input, the SLVBCEIF also be set when the slave select goes to inactive state. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvbeif(&self) -> SLVBEIF_R {
        SLVBEIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Slave Mode TX Under Run Interrupt Flag\\nIn Slave mode, if TX underflow event occurs and the slave select line goes to inactive state, this interrupt flag will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvurif(&self) -> SLVURIF_R {
        SLVURIF_R::new(((self.bits >> 7) & 0x01) != 0)
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
    #[doc = "Bit 15 - SPI Enable Status (Read Only)\\nNote: The SPI peripheral clock is asynchronous with the system clock. In order to make sure the SPI control logic is disabled, this bit indicates the real status of SPI controller."]
    #[inline(always)]
    pub fn spiensts(&self) -> SPIENSTS_R {
        SPIENSTS_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[doc = "Bit 19 - TX Underflow Interrupt Flag\\nWhen the TX underflow event occurs, this bit will be set to 1, the state of data output pin depends on the setting of TXUFPOL.\\nNote 1: This bit will be cleared by writing 1 to it.\\nNote 2: If reset slave's transmission circuit when slave selection signal is active, this flag will be set to 1 after 3 system clock cycles + 2 peripheral clock cycles since the reset operation is done."]
    #[inline(always)]
    pub fn txufif(&self) -> TXUFIF_R {
        TXUFIF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TX or RX Reset Status (Read Only)\\nNote: Both the reset operations of TXRST and RXRST need 3 system clock cycles + 2 peripheral clock cycles. User can check the status of this bit to monitor the reset function is doing or done."]
    #[inline(always)]
    pub fn txrxrst(&self) -> TXRXRST_R {
        TXRXRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Receive FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of receive FIFO buffer."]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Transmit FIFO Data Count (Read Only)\\nThis bit field indicates the valid data count of transmit FIFO buffer."]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Unit Transfer Interrupt Flag\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn unitif(&mut self) -> UNITIF_W {
        UNITIF_W { w: self }
    }
    #[doc = "Bit 2 - Slave Select Active Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn ssactif(&mut self) -> SSACTIF_W {
        SSACTIF_W { w: self }
    }
    #[doc = "Bit 3 - Slave Select Inactive Interrupt Flag\\nNote: Only available in Slave mode. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn ssinaif(&mut self) -> SSINAIF_W {
        SSINAIF_W { w: self }
    }
    #[doc = "Bit 5 - Slave Time-out Interrupt Flag (Only Supported in SPI0)\\nWhen the slave select is active and the value of SLVTOCNT is not 0, as the bus clock is detected, the slave time-out counter in SPI controller logic will be started. When the value of time-out counter is greater than or equal to the value of SLVTOCNT (SPI_SSCTL\\[31:16\\]) before one transaction is done, the slave time-out interrupt event will be asserted.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvtoif(&mut self) -> SLVTOIF_W {
        SLVTOIF_W { w: self }
    }
    #[doc = "Bit 6 - Slave Mode Bit Count Error Interrupt Flag\\nIn Slave mode, when the slave select line goes to inactive state, if bit counter is mismatch with DWIDTH, this interrupt flag will be set to 1.\\nNote: If the slave select active but there is no any bus clock input, the SLVBCEIF also be set when the slave select goes to inactive state. This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvbeif(&mut self) -> SLVBEIF_W {
        SLVBEIF_W { w: self }
    }
    #[doc = "Bit 7 - Slave Mode TX Under Run Interrupt Flag\\nIn Slave mode, if TX underflow event occurs and the slave select line goes to inactive state, this interrupt flag will be set to 1.\\nNote: This bit will be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn slvurif(&mut self) -> SLVURIF_W {
        SLVURIF_W { w: self }
    }
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
    #[doc = "Bit 19 - TX Underflow Interrupt Flag\\nWhen the TX underflow event occurs, this bit will be set to 1, the state of data output pin depends on the setting of TXUFPOL.\\nNote 1: This bit will be cleared by writing 1 to it.\\nNote 2: If reset slave's transmission circuit when slave selection signal is active, this flag will be set to 1 after 3 system clock cycles + 2 peripheral clock cycles since the reset operation is done."]
    #[inline(always)]
    pub fn txufif(&mut self) -> TXUFIF_W {
        TXUFIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_status](index.html) module"]
pub struct SPI_STATUS_SPEC;
impl crate::RegisterSpec for SPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_status::R](R) reader structure"]
impl crate::Readable for SPI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_status::W](W) writer structure"]
impl crate::Writable for SPI_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_STATUS to value 0x0005_0110"]
impl crate::Resettable for SPI_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0110
    }
}
