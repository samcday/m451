#[doc = "Register `OTG_INTSTS` reader"]
pub struct R(crate::R<OTG_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OTG_INTSTS_SPEC>> for R {
    fn from(reader: crate::R<OTG_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_INTSTS` writer"]
pub struct W(crate::W<OTG_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_INTSTS_SPEC>;
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
impl core::convert::From<crate::W<OTG_INTSTS_SPEC>> for W {
    fn from(writer: crate::W<OTG_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OTG Role Change Interrupt Status\\nThis flag is set when the role of an OTG device changed from a host to a peripheral, or changed from a peripheral to a host while USB_ID pin status does not change.\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROLECHGIF_A {
    #[doc = "0: OTG device role not changed"]
    _0 = 0,
    #[doc = "1: OTG device role changed"]
    _1 = 1,
}
impl From<ROLECHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: ROLECHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROLECHGIF` reader - OTG Role Change Interrupt Status\\nThis flag is set when the role of an OTG device changed from a host to a peripheral, or changed from a peripheral to a host while USB_ID pin status does not change.\\nNote: Write 1 to clear this flag."]
pub struct ROLECHGIF_R(crate::FieldReader<bool, ROLECHGIF_A>);
impl ROLECHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROLECHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROLECHGIF_A {
        match self.bits {
            false => ROLECHGIF_A::_0,
            true => ROLECHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ROLECHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ROLECHGIF_A::_1
    }
}
impl core::ops::Deref for ROLECHGIF_R {
    type Target = crate::FieldReader<bool, ROLECHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROLECHGIF` writer - OTG Role Change Interrupt Status\\nThis flag is set when the role of an OTG device changed from a host to a peripheral, or changed from a peripheral to a host while USB_ID pin status does not change.\\nNote: Write 1 to clear this flag."]
pub struct ROLECHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLECHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROLECHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG device role not changed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROLECHGIF_A::_0)
    }
    #[doc = "OTG device role changed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROLECHGIF_A::_1)
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
#[doc = "VBUS Error Interrupt Status\\nThis bit will be set when voltage on VBUS cannot reach a minimum valid threshold 4.4V within a maximum time of 100ms after OTG A-device starting to drive VBUS high. \\nNote: Write 1 to clear this flag and recover from the VBUS error state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBEIF_A {
    #[doc = "0: OTG A-device drives VBUS over threshold voltage before this interval expires"]
    _0 = 0,
    #[doc = "1: OTG A-device cannot drive VBUS over threshold voltage before this interval expires"]
    _1 = 1,
}
impl From<VBEIF_A> for bool {
    #[inline(always)]
    fn from(variant: VBEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBEIF` reader - VBUS Error Interrupt Status\\nThis bit will be set when voltage on VBUS cannot reach a minimum valid threshold 4.4V within a maximum time of 100ms after OTG A-device starting to drive VBUS high. \\nNote: Write 1 to clear this flag and recover from the VBUS error state."]
pub struct VBEIF_R(crate::FieldReader<bool, VBEIF_A>);
impl VBEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBEIF_A {
        match self.bits {
            false => VBEIF_A::_0,
            true => VBEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBEIF_A::_1
    }
}
impl core::ops::Deref for VBEIF_R {
    type Target = crate::FieldReader<bool, VBEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBEIF` writer - VBUS Error Interrupt Status\\nThis bit will be set when voltage on VBUS cannot reach a minimum valid threshold 4.4V within a maximum time of 100ms after OTG A-device starting to drive VBUS high. \\nNote: Write 1 to clear this flag and recover from the VBUS error state."]
pub struct VBEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VBEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG A-device drives VBUS over threshold voltage before this interval expires"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBEIF_A::_0)
    }
    #[doc = "OTG A-device cannot drive VBUS over threshold voltage before this interval expires"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBEIF_A::_1)
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
#[doc = "SRP Fail Interrupt Status\\nAfter initiating SRP, an OTG B-device will wait for the OTG A-device to drive VBUS high at least TB_SRP_FAIL minimum, defined in OTG specification. This flag is set when the OTG B-device does not get VBUS high after this interval.\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRPFIF_A {
    #[doc = "0: OTG B-device gets VBUS high before this interval"]
    _0 = 0,
    #[doc = "1: OTG B-device does not get VBUS high before this interval"]
    _1 = 1,
}
impl From<SRPFIF_A> for bool {
    #[inline(always)]
    fn from(variant: SRPFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPFIF` reader - SRP Fail Interrupt Status\\nAfter initiating SRP, an OTG B-device will wait for the OTG A-device to drive VBUS high at least TB_SRP_FAIL minimum, defined in OTG specification. This flag is set when the OTG B-device does not get VBUS high after this interval.\\nNote: Write 1 to clear this flag."]
pub struct SRPFIF_R(crate::FieldReader<bool, SRPFIF_A>);
impl SRPFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRPFIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPFIF_A {
        match self.bits {
            false => SRPFIF_A::_0,
            true => SRPFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRPFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRPFIF_A::_1
    }
}
impl core::ops::Deref for SRPFIF_R {
    type Target = crate::FieldReader<bool, SRPFIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPFIF` writer - SRP Fail Interrupt Status\\nAfter initiating SRP, an OTG B-device will wait for the OTG A-device to drive VBUS high at least TB_SRP_FAIL minimum, defined in OTG specification. This flag is set when the OTG B-device does not get VBUS high after this interval.\\nNote: Write 1 to clear this flag."]
pub struct SRPFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPFIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPFIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG B-device gets VBUS high before this interval"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRPFIF_A::_0)
    }
    #[doc = "OTG B-device does not get VBUS high before this interval"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRPFIF_A::_1)
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
#[doc = "HNP Fail Interrupt Status\\nWhen A-device has granted B-device to be host and USB bus is in SE0 (both USB_D+ and USB_D- low) state, this bit will be set when A-device does not connect after specified interval expires. \\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNPFIF_A {
    #[doc = "0: A-device connects to B-device before specified interval expires"]
    _0 = 0,
    #[doc = "1: A-device does not connect to B-device before specified interval expires"]
    _1 = 1,
}
impl From<HNPFIF_A> for bool {
    #[inline(always)]
    fn from(variant: HNPFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPFIF` reader - HNP Fail Interrupt Status\\nWhen A-device has granted B-device to be host and USB bus is in SE0 (both USB_D+ and USB_D- low) state, this bit will be set when A-device does not connect after specified interval expires. \\nNote: Write 1 to clear this flag."]
pub struct HNPFIF_R(crate::FieldReader<bool, HNPFIF_A>);
impl HNPFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPFIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPFIF_A {
        match self.bits {
            false => HNPFIF_A::_0,
            true => HNPFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HNPFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HNPFIF_A::_1
    }
}
impl core::ops::Deref for HNPFIF_R {
    type Target = crate::FieldReader<bool, HNPFIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPFIF` writer - HNP Fail Interrupt Status\\nWhen A-device has granted B-device to be host and USB bus is in SE0 (both USB_D+ and USB_D- low) state, this bit will be set when A-device does not connect after specified interval expires. \\nNote: Write 1 to clear this flag."]
pub struct HNPFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPFIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNPFIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A-device connects to B-device before specified interval expires"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HNPFIF_A::_0)
    }
    #[doc = "A-device does not connect to B-device before specified interval expires"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HNPFIF_A::_1)
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
#[doc = "OTG Device Goes to IDLE Interrupt Status\\nFlag is set if the OTG device transfers from non-idle state to idle state. The OTG device will be neither a host nor a peripheral.\\nNote 1: Going to idle state means going to a_idle or b_idle state. Please refer to OTG specification.\\nNote 2: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOIDLEIF_A {
    #[doc = "0: OTG device does not go back to idle state (a_idle or b_idle)"]
    _0 = 0,
    #[doc = "1: OTG device goes back to idle state(a_idle or b_idle)"]
    _1 = 1,
}
impl From<GOIDLEIF_A> for bool {
    #[inline(always)]
    fn from(variant: GOIDLEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GOIDLEIF` reader - OTG Device Goes to IDLE Interrupt Status\\nFlag is set if the OTG device transfers from non-idle state to idle state. The OTG device will be neither a host nor a peripheral.\\nNote 1: Going to idle state means going to a_idle or b_idle state. Please refer to OTG specification.\\nNote 2: Write 1 to clear this flag."]
pub struct GOIDLEIF_R(crate::FieldReader<bool, GOIDLEIF_A>);
impl GOIDLEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOIDLEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GOIDLEIF_A {
        match self.bits {
            false => GOIDLEIF_A::_0,
            true => GOIDLEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GOIDLEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GOIDLEIF_A::_1
    }
}
impl core::ops::Deref for GOIDLEIF_R {
    type Target = crate::FieldReader<bool, GOIDLEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOIDLEIF` writer - OTG Device Goes to IDLE Interrupt Status\\nFlag is set if the OTG device transfers from non-idle state to idle state. The OTG device will be neither a host nor a peripheral.\\nNote 1: Going to idle state means going to a_idle or b_idle state. Please refer to OTG specification.\\nNote 2: Write 1 to clear this flag."]
pub struct GOIDLEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> GOIDLEIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GOIDLEIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG device does not go back to idle state (a_idle or b_idle)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GOIDLEIF_A::_0)
    }
    #[doc = "OTG device goes back to idle state(a_idle or b_idle)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GOIDLEIF_A::_1)
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
#[doc = "ID State Change Interrupt Status\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDCHGIF_A {
    #[doc = "0: IDSTS (OTG_STATUS\\[1\\]) not toggled"]
    _0 = 0,
    #[doc = "1: IDSTS (OTG_STATUS\\[1\\]) from high to low or from low to high"]
    _1 = 1,
}
impl From<IDCHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: IDCHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDCHGIF` reader - ID State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct IDCHGIF_R(crate::FieldReader<bool, IDCHGIF_A>);
impl IDCHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDCHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDCHGIF_A {
        match self.bits {
            false => IDCHGIF_A::_0,
            true => IDCHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDCHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDCHGIF_A::_1
    }
}
impl core::ops::Deref for IDCHGIF_R {
    type Target = crate::FieldReader<bool, IDCHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDCHGIF` writer - ID State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct IDCHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDCHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IDSTS (OTG_STATUS\\[1\\]) not toggled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDCHGIF_A::_0)
    }
    #[doc = "IDSTS (OTG_STATUS\\[1\\]) from high to low or from low to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDCHGIF_A::_1)
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
#[doc = "Act As Peripheral Interrupt Status\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEVIF_A {
    #[doc = "0: This device does not act as a peripheral"]
    _0 = 0,
    #[doc = "1: This device acts as a peripheral"]
    _1 = 1,
}
impl From<PDEVIF_A> for bool {
    #[inline(always)]
    fn from(variant: PDEVIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEVIF` reader - Act As Peripheral Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct PDEVIF_R(crate::FieldReader<bool, PDEVIF_A>);
impl PDEVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEVIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEVIF_A {
        match self.bits {
            false => PDEVIF_A::_0,
            true => PDEVIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDEVIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDEVIF_A::_1
    }
}
impl core::ops::Deref for PDEVIF_R {
    type Target = crate::FieldReader<bool, PDEVIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEVIF` writer - Act As Peripheral Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct PDEVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEVIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEVIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This device does not act as a peripheral"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDEVIF_A::_0)
    }
    #[doc = "This device acts as a peripheral"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDEVIF_A::_1)
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
#[doc = "Act As Host Interrupt Status\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTIF_A {
    #[doc = "0: This device does not act as a host"]
    _0 = 0,
    #[doc = "1: This device acts as a host"]
    _1 = 1,
}
impl From<HOSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTIF` reader - Act As Host Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct HOSTIF_R(crate::FieldReader<bool, HOSTIF_A>);
impl HOSTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTIF_A {
        match self.bits {
            false => HOSTIF_A::_0,
            true => HOSTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HOSTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HOSTIF_A::_1
    }
}
impl core::ops::Deref for HOSTIF_R {
    type Target = crate::FieldReader<bool, HOSTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTIF` writer - Act As Host Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct HOSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOSTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This device does not act as a host"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOSTIF_A::_0)
    }
    #[doc = "This device acts as a host"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOSTIF_A::_1)
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
#[doc = "B-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVLDCHGIF_A {
    #[doc = "0: BVLD (OTG_STATUS\\[3\\]) is not toggled"]
    _0 = 0,
    #[doc = "1: BVLD (OTG_STATUS\\[3\\]) from high to low or low to high"]
    _1 = 1,
}
impl From<BVLDCHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: BVLDCHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BVLDCHGIF` reader - B-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct BVLDCHGIF_R(crate::FieldReader<bool, BVLDCHGIF_A>);
impl BVLDCHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVLDCHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVLDCHGIF_A {
        match self.bits {
            false => BVLDCHGIF_A::_0,
            true => BVLDCHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BVLDCHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BVLDCHGIF_A::_1
    }
}
impl core::ops::Deref for BVLDCHGIF_R {
    type Target = crate::FieldReader<bool, BVLDCHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVLDCHGIF` writer - B-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct BVLDCHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BVLDCHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BVLDCHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BVLD (OTG_STATUS\\[3\\]) is not toggled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BVLDCHGIF_A::_0)
    }
    #[doc = "BVLD (OTG_STATUS\\[3\\]) from high to low or low to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BVLDCHGIF_A::_1)
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
#[doc = "A-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLDCHGIF_A {
    #[doc = "0: AVLD (OTG_STATUS\\[4\\]) not toggled"]
    _0 = 0,
    #[doc = "1: AVLD (OTG_STATUS\\[4\\]) from high to low or low to high"]
    _1 = 1,
}
impl From<AVLDCHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: AVLDCHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLDCHGIF` reader - A-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct AVLDCHGIF_R(crate::FieldReader<bool, AVLDCHGIF_A>);
impl AVLDCHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVLDCHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLDCHGIF_A {
        match self.bits {
            false => AVLDCHGIF_A::_0,
            true => AVLDCHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVLDCHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVLDCHGIF_A::_1
    }
}
impl core::ops::Deref for AVLDCHGIF_R {
    type Target = crate::FieldReader<bool, AVLDCHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVLDCHGIF` writer - A-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct AVLDCHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLDCHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLDCHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AVLD (OTG_STATUS\\[4\\]) not toggled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLDCHGIF_A::_0)
    }
    #[doc = "AVLD (OTG_STATUS\\[4\\]) from high to low or low to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLDCHGIF_A::_1)
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
#[doc = "VBUSVLD State Change Interrupt Status\\nNote: Write 1 to clear this status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBCHGIF_A {
    #[doc = "0: VBUSVLD (OTG_STATUS\\[5\\]) not toggled"]
    _0 = 0,
    #[doc = "1: VBUSVLD (OTG_STATUS\\[5\\]) from high to low or from low to high"]
    _1 = 1,
}
impl From<VBCHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: VBCHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBCHGIF` reader - VBUSVLD State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct VBCHGIF_R(crate::FieldReader<bool, VBCHGIF_A>);
impl VBCHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBCHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBCHGIF_A {
        match self.bits {
            false => VBCHGIF_A::_0,
            true => VBCHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBCHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBCHGIF_A::_1
    }
}
impl core::ops::Deref for VBCHGIF_R {
    type Target = crate::FieldReader<bool, VBCHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBCHGIF` writer - VBUSVLD State Change Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct VBCHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VBCHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBCHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBUSVLD (OTG_STATUS\\[5\\]) not toggled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBCHGIF_A::_0)
    }
    #[doc = "VBUSVLD (OTG_STATUS\\[5\\]) from high to low or from low to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBCHGIF_A::_1)
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
#[doc = "SESSEND State Change Interrupt Status\\nNote: Write 1 to clear this flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECHGIF_A {
    #[doc = "0: SESSEND (OTG_STATUS\\[2\\]) not toggled"]
    _0 = 0,
    #[doc = "1: SESSEND (OTG_STATUS\\[2\\]) from high to low or from low to high"]
    _1 = 1,
}
impl From<SECHGIF_A> for bool {
    #[inline(always)]
    fn from(variant: SECHGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECHGIF` reader - SESSEND State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct SECHGIF_R(crate::FieldReader<bool, SECHGIF_A>);
impl SECHGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECHGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECHGIF_A {
        match self.bits {
            false => SECHGIF_A::_0,
            true => SECHGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SECHGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SECHGIF_A::_1
    }
}
impl core::ops::Deref for SECHGIF_R {
    type Target = crate::FieldReader<bool, SECHGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECHGIF` writer - SESSEND State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
pub struct SECHGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SECHGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECHGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SESSEND (OTG_STATUS\\[2\\]) not toggled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECHGIF_A::_0)
    }
    #[doc = "SESSEND (OTG_STATUS\\[2\\]) from high to low or from low to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECHGIF_A::_1)
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
#[doc = "SRP Detected Interrupt Status\\nNote: Write 1 to clear this status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRPDETIF_A {
    #[doc = "0: SRP not detected"]
    _0 = 0,
    #[doc = "1: SRP detected"]
    _1 = 1,
}
impl From<SRPDETIF_A> for bool {
    #[inline(always)]
    fn from(variant: SRPDETIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPDETIF` reader - SRP Detected Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct SRPDETIF_R(crate::FieldReader<bool, SRPDETIF_A>);
impl SRPDETIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRPDETIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPDETIF_A {
        match self.bits {
            false => SRPDETIF_A::_0,
            true => SRPDETIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRPDETIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRPDETIF_A::_1
    }
}
impl core::ops::Deref for SRPDETIF_R {
    type Target = crate::FieldReader<bool, SRPDETIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPDETIF` writer - SRP Detected Interrupt Status\\nNote: Write 1 to clear this status."]
pub struct SRPDETIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPDETIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPDETIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRP not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRPDETIF_A::_0)
    }
    #[doc = "SRP detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRPDETIF_A::_1)
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
impl R {
    #[doc = "Bit 0 - OTG Role Change Interrupt Status\\nThis flag is set when the role of an OTG device changed from a host to a peripheral, or changed from a peripheral to a host while USB_ID pin status does not change.\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn rolechgif(&self) -> ROLECHGIF_R {
        ROLECHGIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Error Interrupt Status\\nThis bit will be set when voltage on VBUS cannot reach a minimum valid threshold 4.4V within a maximum time of 100ms after OTG A-device starting to drive VBUS high. \\nNote: Write 1 to clear this flag and recover from the VBUS error state."]
    #[inline(always)]
    pub fn vbeif(&self) -> VBEIF_R {
        VBEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Fail Interrupt Status\\nAfter initiating SRP, an OTG B-device will wait for the OTG A-device to drive VBUS high at least TB_SRP_FAIL minimum, defined in OTG specification. This flag is set when the OTG B-device does not get VBUS high after this interval.\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn srpfif(&self) -> SRPFIF_R {
        SRPFIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HNP Fail Interrupt Status\\nWhen A-device has granted B-device to be host and USB bus is in SE0 (both USB_D+ and USB_D- low) state, this bit will be set when A-device does not connect after specified interval expires. \\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn hnpfif(&self) -> HNPFIF_R {
        HNPFIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OTG Device Goes to IDLE Interrupt Status\\nFlag is set if the OTG device transfers from non-idle state to idle state. The OTG device will be neither a host nor a peripheral.\\nNote 1: Going to idle state means going to a_idle or b_idle state. Please refer to OTG specification.\\nNote 2: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn goidleif(&self) -> GOIDLEIF_R {
        GOIDLEIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ID State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn idchgif(&self) -> IDCHGIF_R {
        IDCHGIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Act As Peripheral Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn pdevif(&self) -> PDEVIF_R {
        PDEVIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Act As Host Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn hostif(&self) -> HOSTIF_R {
        HOSTIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn bvldchgif(&self) -> BVLDCHGIF_R {
        BVLDCHGIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn avldchgif(&self) -> AVLDCHGIF_R {
        AVLDCHGIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBUSVLD State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn vbchgif(&self) -> VBCHGIF_R {
        VBCHGIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SESSEND State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn sechgif(&self) -> SECHGIF_R {
        SECHGIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SRP Detected Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn srpdetif(&self) -> SRPDETIF_R {
        SRPDETIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTG Role Change Interrupt Status\\nThis flag is set when the role of an OTG device changed from a host to a peripheral, or changed from a peripheral to a host while USB_ID pin status does not change.\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn rolechgif(&mut self) -> ROLECHGIF_W {
        ROLECHGIF_W { w: self }
    }
    #[doc = "Bit 1 - VBUS Error Interrupt Status\\nThis bit will be set when voltage on VBUS cannot reach a minimum valid threshold 4.4V within a maximum time of 100ms after OTG A-device starting to drive VBUS high. \\nNote: Write 1 to clear this flag and recover from the VBUS error state."]
    #[inline(always)]
    pub fn vbeif(&mut self) -> VBEIF_W {
        VBEIF_W { w: self }
    }
    #[doc = "Bit 2 - SRP Fail Interrupt Status\\nAfter initiating SRP, an OTG B-device will wait for the OTG A-device to drive VBUS high at least TB_SRP_FAIL minimum, defined in OTG specification. This flag is set when the OTG B-device does not get VBUS high after this interval.\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn srpfif(&mut self) -> SRPFIF_W {
        SRPFIF_W { w: self }
    }
    #[doc = "Bit 3 - HNP Fail Interrupt Status\\nWhen A-device has granted B-device to be host and USB bus is in SE0 (both USB_D+ and USB_D- low) state, this bit will be set when A-device does not connect after specified interval expires. \\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn hnpfif(&mut self) -> HNPFIF_W {
        HNPFIF_W { w: self }
    }
    #[doc = "Bit 4 - OTG Device Goes to IDLE Interrupt Status\\nFlag is set if the OTG device transfers from non-idle state to idle state. The OTG device will be neither a host nor a peripheral.\\nNote 1: Going to idle state means going to a_idle or b_idle state. Please refer to OTG specification.\\nNote 2: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn goidleif(&mut self) -> GOIDLEIF_W {
        GOIDLEIF_W { w: self }
    }
    #[doc = "Bit 5 - ID State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn idchgif(&mut self) -> IDCHGIF_W {
        IDCHGIF_W { w: self }
    }
    #[doc = "Bit 6 - Act As Peripheral Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn pdevif(&mut self) -> PDEVIF_W {
        PDEVIF_W { w: self }
    }
    #[doc = "Bit 7 - Act As Host Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn hostif(&mut self) -> HOSTIF_W {
        HOSTIF_W { w: self }
    }
    #[doc = "Bit 8 - B-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn bvldchgif(&mut self) -> BVLDCHGIF_W {
        BVLDCHGIF_W { w: self }
    }
    #[doc = "Bit 9 - A-device Session Valid State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn avldchgif(&mut self) -> AVLDCHGIF_W {
        AVLDCHGIF_W { w: self }
    }
    #[doc = "Bit 10 - VBUSVLD State Change Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn vbchgif(&mut self) -> VBCHGIF_W {
        VBCHGIF_W { w: self }
    }
    #[doc = "Bit 11 - SESSEND State Change Interrupt Status\\nNote: Write 1 to clear this flag."]
    #[inline(always)]
    pub fn sechgif(&mut self) -> SECHGIF_W {
        SECHGIF_W { w: self }
    }
    #[doc = "Bit 13 - SRP Detected Interrupt Status\\nNote: Write 1 to clear this status."]
    #[inline(always)]
    pub fn srpdetif(&mut self) -> SRPDETIF_W {
        SRPDETIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_intsts](index.html) module"]
pub struct OTG_INTSTS_SPEC;
impl crate::RegisterSpec for OTG_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_intsts::R](R) reader structure"]
impl crate::Readable for OTG_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_intsts::W](W) writer structure"]
impl crate::Writable for OTG_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_INTSTS to value 0"]
impl crate::Resettable for OTG_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
