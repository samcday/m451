#[doc = "Register `SPI_SSCTL` reader"]
pub struct R(crate::R<SPI_SSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_SSCTL_SPEC>> for R {
    fn from(reader: crate::R<SPI_SSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SSCTL` writer"]
pub struct W(crate::W<SPI_SSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SSCTL_SPEC>;
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
impl core::convert::From<crate::W<SPI_SSCTL_SPEC>> for W {
    fn from(writer: crate::W<SPI_SSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Selection Control (Master Only)\\nIf AUTOSS bit is cleared to 0,\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_A {
    #[doc = "0: set the SPIn_SS line to inactive state.\\nKeep the SPIn_SS line at inactive state"]
    _0 = 0,
    #[doc = "1: set the SPIn_SS line to active state.\\nSPIn_SS line will be automatically driven to active state for the duration of data transfer, and will be driven to inactive state for the rest of the time. The active state of SPIn_SS is specified in SSACTPOL (SPI_SSCTL\\[2\\])"]
    _1 = 1,
}
impl From<SS_A> for bool {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS` reader - Slave Selection Control (Master Only)\\nIf AUTOSS bit is cleared to 0,"]
pub struct SS_R(crate::FieldReader<bool, SS_A>);
impl SS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_A {
        match self.bits {
            false => SS_A::_0,
            true => SS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SS_A::_1
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<bool, SS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS` writer - Slave Selection Control (Master Only)\\nIf AUTOSS bit is cleared to 0,"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "set the SPIn_SS line to inactive state.\\nKeep the SPIn_SS line at inactive state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SS_A::_0)
    }
    #[doc = "set the SPIn_SS line to active state.\\nSPIn_SS line will be automatically driven to active state for the duration of data transfer, and will be driven to inactive state for the rest of the time. The active state of SPIn_SS is specified in SSACTPOL (SPI_SSCTL\\[2\\])"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SS_A::_1)
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
#[doc = "Slave Selection Active Polarity\\nThis bit defines the active polarity of slave selection signal (SPIn_SS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACTPOL_A {
    #[doc = "0: The slave selection signal SPIn_SS is active low"]
    _0 = 0,
    #[doc = "1: The slave selection signal SPIn_SS is active high"]
    _1 = 1,
}
impl From<SSACTPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SSACTPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACTPOL` reader - Slave Selection Active Polarity\\nThis bit defines the active polarity of slave selection signal (SPIn_SS)."]
pub struct SSACTPOL_R(crate::FieldReader<bool, SSACTPOL_A>);
impl SSACTPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACTPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACTPOL_A {
        match self.bits {
            false => SSACTPOL_A::_0,
            true => SSACTPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACTPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACTPOL_A::_1
    }
}
impl core::ops::Deref for SSACTPOL_R {
    type Target = crate::FieldReader<bool, SSACTPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACTPOL` writer - Slave Selection Active Polarity\\nThis bit defines the active polarity of slave selection signal (SPIn_SS)."]
pub struct SSACTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACTPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACTPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave selection signal SPIn_SS is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACTPOL_A::_0)
    }
    #[doc = "The slave selection signal SPIn_SS is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACTPOL_A::_1)
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
#[doc = "Automatic Slave Selection Function Enable Bit (Master Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSS_A {
    #[doc = "0: Automatic slave selection function Disabled. Slave selection signal will be asserted/de-asserted according to SS (SPI_SSCTL\\[0\\])"]
    _0 = 0,
    #[doc = "1: Automatic slave selection function Enabled"]
    _1 = 1,
}
impl From<AUTOSS_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSS` reader - Automatic Slave Selection Function Enable Bit (Master Only)"]
pub struct AUTOSS_R(crate::FieldReader<bool, AUTOSS_A>);
impl AUTOSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSS_A {
        match self.bits {
            false => AUTOSS_A::_0,
            true => AUTOSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AUTOSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AUTOSS_A::_1
    }
}
impl core::ops::Deref for AUTOSS_R {
    type Target = crate::FieldReader<bool, AUTOSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSS` writer - Automatic Slave Selection Function Enable Bit (Master Only)"]
pub struct AUTOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOSS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic slave selection function Disabled. Slave selection signal will be asserted/de-asserted according to SS (SPI_SSCTL\\[0\\])"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOSS_A::_0)
    }
    #[doc = "Automatic slave selection function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOSS_A::_1)
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
#[doc = "Slave 3-wire Mode Enable Bit (Only Supported in SPI0)\\nSlave 3-wire mode is only available in SPI0. In Slave 3-wire mode, the SPI controller can work with 3-wire interface including SPI0_CLK, SPI0_MISO and SPI0_MOSI pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLV3WIRE_A {
    #[doc = "0: 4-wire bi-direction interface"]
    _0 = 0,
    #[doc = "1: 3-wire bi-direction interface"]
    _1 = 1,
}
impl From<SLV3WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: SLV3WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV3WIRE` reader - Slave 3-wire Mode Enable Bit (Only Supported in SPI0)\\nSlave 3-wire mode is only available in SPI0. In Slave 3-wire mode, the SPI controller can work with 3-wire interface including SPI0_CLK, SPI0_MISO and SPI0_MOSI pins."]
pub struct SLV3WIRE_R(crate::FieldReader<bool, SLV3WIRE_A>);
impl SLV3WIRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV3WIRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLV3WIRE_A {
        match self.bits {
            false => SLV3WIRE_A::_0,
            true => SLV3WIRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLV3WIRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLV3WIRE_A::_1
    }
}
impl core::ops::Deref for SLV3WIRE_R {
    type Target = crate::FieldReader<bool, SLV3WIRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV3WIRE` writer - Slave 3-wire Mode Enable Bit (Only Supported in SPI0)\\nSlave 3-wire mode is only available in SPI0. In Slave 3-wire mode, the SPI controller can work with 3-wire interface including SPI0_CLK, SPI0_MISO and SPI0_MOSI pins."]
pub struct SLV3WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV3WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLV3WIRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4-wire bi-direction interface"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLV3WIRE_A::_0)
    }
    #[doc = "3-wire bi-direction interface"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLV3WIRE_A::_1)
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
#[doc = "Slave Mode Time-out Interrupt Enable Bit (Only Supported in SPI0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVTOIEN_A {
    #[doc = "0: Slave mode time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave mode time-out interrupt Enabled"]
    _1 = 1,
}
impl From<SLVTOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVTOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVTOIEN` reader - Slave Mode Time-out Interrupt Enable Bit (Only Supported in SPI0)"]
pub struct SLVTOIEN_R(crate::FieldReader<bool, SLVTOIEN_A>);
impl SLVTOIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVTOIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVTOIEN_A {
        match self.bits {
            false => SLVTOIEN_A::_0,
            true => SLVTOIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVTOIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVTOIEN_A::_1
    }
}
impl core::ops::Deref for SLVTOIEN_R {
    type Target = crate::FieldReader<bool, SLVTOIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOIEN` writer - Slave Mode Time-out Interrupt Enable Bit (Only Supported in SPI0)"]
pub struct SLVTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVTOIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVTOIEN_A::_0)
    }
    #[doc = "Slave mode time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVTOIEN_A::_1)
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
#[doc = "Slave Mode Time-out Reset Control (Only Supported in SPI0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVTORST_A {
    #[doc = "0: When Slave mode time-out event occurs, the TX and RX control circuit will not be reset"]
    _0 = 0,
    #[doc = "1: When Slave mode time-out event occurs, the TX and RX control circuit will be reset by hardware"]
    _1 = 1,
}
impl From<SLVTORST_A> for bool {
    #[inline(always)]
    fn from(variant: SLVTORST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVTORST` reader - Slave Mode Time-out Reset Control (Only Supported in SPI0)"]
pub struct SLVTORST_R(crate::FieldReader<bool, SLVTORST_A>);
impl SLVTORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVTORST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVTORST_A {
        match self.bits {
            false => SLVTORST_A::_0,
            true => SLVTORST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVTORST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVTORST_A::_1
    }
}
impl core::ops::Deref for SLVTORST_R {
    type Target = crate::FieldReader<bool, SLVTORST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTORST` writer - Slave Mode Time-out Reset Control (Only Supported in SPI0)"]
pub struct SLVTORST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVTORST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When Slave mode time-out event occurs, the TX and RX control circuit will not be reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVTORST_A::_0)
    }
    #[doc = "When Slave mode time-out event occurs, the TX and RX control circuit will be reset by hardware"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVTORST_A::_1)
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
#[doc = "Slave Mode Bit Count Error Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVBEIEN_A {
    #[doc = "0: Slave mode bit count error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave mode bit count error interrupt Enabled"]
    _1 = 1,
}
impl From<SLVBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVBEIEN` reader - Slave Mode Bit Count Error Interrupt Enable Bit"]
pub struct SLVBEIEN_R(crate::FieldReader<bool, SLVBEIEN_A>);
impl SLVBEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVBEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVBEIEN_A {
        match self.bits {
            false => SLVBEIEN_A::_0,
            true => SLVBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVBEIEN_A::_1
    }
}
impl core::ops::Deref for SLVBEIEN_R {
    type Target = crate::FieldReader<bool, SLVBEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVBEIEN` writer - Slave Mode Bit Count Error Interrupt Enable Bit"]
pub struct SLVBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVBEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode bit count error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVBEIEN_A::_0)
    }
    #[doc = "Slave mode bit count error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVBEIEN_A::_1)
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
#[doc = "Slave Mode TX Under Run Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVURIEN_A {
    #[doc = "0: Slave mode TX under run interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave mode TX under run interrupt Enabled"]
    _1 = 1,
}
impl From<SLVURIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVURIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVURIEN` reader - Slave Mode TX Under Run Interrupt Enable Bit"]
pub struct SLVURIEN_R(crate::FieldReader<bool, SLVURIEN_A>);
impl SLVURIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVURIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVURIEN_A {
        match self.bits {
            false => SLVURIEN_A::_0,
            true => SLVURIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVURIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVURIEN_A::_1
    }
}
impl core::ops::Deref for SLVURIEN_R {
    type Target = crate::FieldReader<bool, SLVURIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVURIEN` writer - Slave Mode TX Under Run Interrupt Enable Bit"]
pub struct SLVURIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVURIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVURIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode TX under run interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVURIEN_A::_0)
    }
    #[doc = "Slave mode TX under run interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVURIEN_A::_1)
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
#[doc = "Slave Select Active Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACTIEN_A {
    #[doc = "0: Slave select active interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave select active interrupt Enabled"]
    _1 = 1,
}
impl From<SSACTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSACTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACTIEN` reader - Slave Select Active Interrupt Enable Bit"]
pub struct SSACTIEN_R(crate::FieldReader<bool, SSACTIEN_A>);
impl SSACTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACTIEN_A {
        match self.bits {
            false => SSACTIEN_A::_0,
            true => SSACTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACTIEN_A::_1
    }
}
impl core::ops::Deref for SSACTIEN_R {
    type Target = crate::FieldReader<bool, SSACTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACTIEN` writer - Slave Select Active Interrupt Enable Bit"]
pub struct SSACTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select active interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACTIEN_A::_0)
    }
    #[doc = "Slave select active interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACTIEN_A::_1)
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
#[doc = "Slave Select Inactive Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSINAIEN_A {
    #[doc = "0: Slave select inactive interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Slave select inactive interrupt Enabled"]
    _1 = 1,
}
impl From<SSINAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SSINAIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSINAIEN` reader - Slave Select Inactive Interrupt Enable Bit"]
pub struct SSINAIEN_R(crate::FieldReader<bool, SSINAIEN_A>);
impl SSINAIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSINAIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSINAIEN_A {
        match self.bits {
            false => SSINAIEN_A::_0,
            true => SSINAIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSINAIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSINAIEN_A::_1
    }
}
impl core::ops::Deref for SSINAIEN_R {
    type Target = crate::FieldReader<bool, SSINAIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINAIEN` writer - Slave Select Inactive Interrupt Enable Bit"]
pub struct SSINAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINAIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSINAIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave select inactive interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSINAIEN_A::_0)
    }
    #[doc = "Slave select inactive interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSINAIEN_A::_1)
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
#[doc = "Field `SLVTOCNT` reader - Slave Mode Time-out Period (Only Supported in SPI0)\\nIn Slave mode, these bits indicate the time-out period when there is bus clock input during slave select active. The clock source of the time-out counter is Slave peripheral clock. If the value is 0, it indicates the slave mode time-out function is disabled."]
pub struct SLVTOCNT_R(crate::FieldReader<u16, u16>);
impl SLVTOCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        SLVTOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVTOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVTOCNT` writer - Slave Mode Time-out Period (Only Supported in SPI0)\\nIn Slave mode, these bits indicate the time-out period when there is bus clock input during slave select active. The clock source of the time-out counter is Slave peripheral clock. If the value is 0, it indicates the slave mode time-out function is disabled."]
pub struct SLVTOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVTOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Selection Control (Master Only)\\nIf AUTOSS bit is cleared to 0,"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Selection Active Polarity\\nThis bit defines the active polarity of slave selection signal (SPIn_SS)."]
    #[inline(always)]
    pub fn ssactpol(&self) -> SSACTPOL_R {
        SSACTPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Automatic Slave Selection Function Enable Bit (Master Only)"]
    #[inline(always)]
    pub fn autoss(&self) -> AUTOSS_R {
        AUTOSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave 3-wire Mode Enable Bit (Only Supported in SPI0)\\nSlave 3-wire mode is only available in SPI0. In Slave 3-wire mode, the SPI controller can work with 3-wire interface including SPI0_CLK, SPI0_MISO and SPI0_MOSI pins."]
    #[inline(always)]
    pub fn slv3wire(&self) -> SLV3WIRE_R {
        SLV3WIRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Mode Time-out Interrupt Enable Bit (Only Supported in SPI0)"]
    #[inline(always)]
    pub fn slvtoien(&self) -> SLVTOIEN_R {
        SLVTOIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slave Mode Time-out Reset Control (Only Supported in SPI0)"]
    #[inline(always)]
    pub fn slvtorst(&self) -> SLVTORST_R {
        SLVTORST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Mode Bit Count Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn slvbeien(&self) -> SLVBEIEN_R {
        SLVBEIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Mode TX Under Run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn slvurien(&self) -> SLVURIEN_R {
        SLVURIEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slave Select Active Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ssactien(&self) -> SSACTIEN_R {
        SSACTIEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Slave Select Inactive Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ssinaien(&self) -> SSINAIEN_R {
        SSINAIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Slave Mode Time-out Period (Only Supported in SPI0)\\nIn Slave mode, these bits indicate the time-out period when there is bus clock input during slave select active. The clock source of the time-out counter is Slave peripheral clock. If the value is 0, it indicates the slave mode time-out function is disabled."]
    #[inline(always)]
    pub fn slvtocnt(&self) -> SLVTOCNT_R {
        SLVTOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Selection Control (Master Only)\\nIf AUTOSS bit is cleared to 0,"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Bit 2 - Slave Selection Active Polarity\\nThis bit defines the active polarity of slave selection signal (SPIn_SS)."]
    #[inline(always)]
    pub fn ssactpol(&mut self) -> SSACTPOL_W {
        SSACTPOL_W { w: self }
    }
    #[doc = "Bit 3 - Automatic Slave Selection Function Enable Bit (Master Only)"]
    #[inline(always)]
    pub fn autoss(&mut self) -> AUTOSS_W {
        AUTOSS_W { w: self }
    }
    #[doc = "Bit 4 - Slave 3-wire Mode Enable Bit (Only Supported in SPI0)\\nSlave 3-wire mode is only available in SPI0. In Slave 3-wire mode, the SPI controller can work with 3-wire interface including SPI0_CLK, SPI0_MISO and SPI0_MOSI pins."]
    #[inline(always)]
    pub fn slv3wire(&mut self) -> SLV3WIRE_W {
        SLV3WIRE_W { w: self }
    }
    #[doc = "Bit 5 - Slave Mode Time-out Interrupt Enable Bit (Only Supported in SPI0)"]
    #[inline(always)]
    pub fn slvtoien(&mut self) -> SLVTOIEN_W {
        SLVTOIEN_W { w: self }
    }
    #[doc = "Bit 6 - Slave Mode Time-out Reset Control (Only Supported in SPI0)"]
    #[inline(always)]
    pub fn slvtorst(&mut self) -> SLVTORST_W {
        SLVTORST_W { w: self }
    }
    #[doc = "Bit 8 - Slave Mode Bit Count Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn slvbeien(&mut self) -> SLVBEIEN_W {
        SLVBEIEN_W { w: self }
    }
    #[doc = "Bit 9 - Slave Mode TX Under Run Interrupt Enable Bit"]
    #[inline(always)]
    pub fn slvurien(&mut self) -> SLVURIEN_W {
        SLVURIEN_W { w: self }
    }
    #[doc = "Bit 12 - Slave Select Active Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ssactien(&mut self) -> SSACTIEN_W {
        SSACTIEN_W { w: self }
    }
    #[doc = "Bit 13 - Slave Select Inactive Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ssinaien(&mut self) -> SSINAIEN_W {
        SSINAIEN_W { w: self }
    }
    #[doc = "Bits 16:31 - Slave Mode Time-out Period (Only Supported in SPI0)\\nIn Slave mode, these bits indicate the time-out period when there is bus clock input during slave select active. The clock source of the time-out counter is Slave peripheral clock. If the value is 0, it indicates the slave mode time-out function is disabled."]
    #[inline(always)]
    pub fn slvtocnt(&mut self) -> SLVTOCNT_W {
        SLVTOCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ssctl](index.html) module"]
pub struct SPI_SSCTL_SPEC;
impl crate::RegisterSpec for SPI_SSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ssctl::R](R) reader structure"]
impl crate::Readable for SPI_SSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ssctl::W](W) writer structure"]
impl crate::Writable for SPI_SSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SSCTL to value 0"]
impl crate::Resettable for SPI_SSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
