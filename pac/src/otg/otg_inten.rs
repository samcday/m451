#[doc = "Register `OTG_INTEN` reader"]
pub struct R(crate::R<OTG_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OTG_INTEN_SPEC>> for R {
    fn from(reader: crate::R<OTG_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_INTEN` writer"]
pub struct W(crate::W<OTG_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_INTEN_SPEC>;
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
impl core::convert::From<crate::W<OTG_INTEN_SPEC>> for W {
    fn from(writer: crate::W<OTG_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Role (Host or Peripheral) Changed Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROLECHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<ROLECHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ROLECHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROLECHGIEN` reader - Role (Host or Peripheral) Changed Interrupt Enable Bit"]
pub struct ROLECHGIEN_R(crate::FieldReader<bool, ROLECHGIEN_A>);
impl ROLECHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROLECHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROLECHGIEN_A {
        match self.bits {
            false => ROLECHGIEN_A::_0,
            true => ROLECHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ROLECHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ROLECHGIEN_A::_1
    }
}
impl core::ops::Deref for ROLECHGIEN_R {
    type Target = crate::FieldReader<bool, ROLECHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROLECHGIEN` writer - Role (Host or Peripheral) Changed Interrupt Enable Bit"]
pub struct ROLECHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLECHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROLECHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROLECHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROLECHGIEN_A::_1)
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
#[doc = "VBUS Error Interrupt Enable Bit\\nNote: VBUS error means going to a_vbus_err state. Please refer to A-device state diagram in OTG spec.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBEIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<VBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBEIEN` reader - VBUS Error Interrupt Enable Bit\\nNote: VBUS error means going to a_vbus_err state. Please refer to A-device state diagram in OTG spec."]
pub struct VBEIEN_R(crate::FieldReader<bool, VBEIEN_A>);
impl VBEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBEIEN_A {
        match self.bits {
            false => VBEIEN_A::_0,
            true => VBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBEIEN_A::_1
    }
}
impl core::ops::Deref for VBEIEN_R {
    type Target = crate::FieldReader<bool, VBEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBEIEN` writer - VBUS Error Interrupt Enable Bit\\nNote: VBUS error means going to a_vbus_err state. Please refer to A-device state diagram in OTG spec."]
pub struct VBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBEIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBEIEN_A::_1)
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
#[doc = "SRP Fail Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRPFIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<SRPFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRPFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPFIEN` reader - SRP Fail Interrupt Enable Bit"]
pub struct SRPFIEN_R(crate::FieldReader<bool, SRPFIEN_A>);
impl SRPFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRPFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPFIEN_A {
        match self.bits {
            false => SRPFIEN_A::_0,
            true => SRPFIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRPFIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRPFIEN_A::_1
    }
}
impl core::ops::Deref for SRPFIEN_R {
    type Target = crate::FieldReader<bool, SRPFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPFIEN` writer - SRP Fail Interrupt Enable Bit"]
pub struct SRPFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRPFIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRPFIEN_A::_1)
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
#[doc = "HNP Fail Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNPFIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<HNPFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: HNPFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPFIEN` reader - HNP Fail Interrupt Enable Bit"]
pub struct HNPFIEN_R(crate::FieldReader<bool, HNPFIEN_A>);
impl HNPFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPFIEN_A {
        match self.bits {
            false => HNPFIEN_A::_0,
            true => HNPFIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HNPFIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HNPFIEN_A::_1
    }
}
impl core::ops::Deref for HNPFIEN_R {
    type Target = crate::FieldReader<bool, HNPFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPFIEN` writer - HNP Fail Interrupt Enable Bit"]
pub struct HNPFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNPFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HNPFIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HNPFIEN_A::_1)
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
#[doc = "OTG Device Goes to IDLE State Interrupt Enable Bit\\nNote: Going to idle state means going to a_idle or b_idle state. Please refer to A-device state diagram and B-device state diagram in OTG spec.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOIDLEIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<GOIDLEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: GOIDLEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GOIDLEIEN` reader - OTG Device Goes to IDLE State Interrupt Enable Bit\\nNote: Going to idle state means going to a_idle or b_idle state. Please refer to A-device state diagram and B-device state diagram in OTG spec."]
pub struct GOIDLEIEN_R(crate::FieldReader<bool, GOIDLEIEN_A>);
impl GOIDLEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOIDLEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GOIDLEIEN_A {
        match self.bits {
            false => GOIDLEIEN_A::_0,
            true => GOIDLEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GOIDLEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GOIDLEIEN_A::_1
    }
}
impl core::ops::Deref for GOIDLEIEN_R {
    type Target = crate::FieldReader<bool, GOIDLEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOIDLEIEN` writer - OTG Device Goes to IDLE State Interrupt Enable Bit\\nNote: Going to idle state means going to a_idle or b_idle state. Please refer to A-device state diagram and B-device state diagram in OTG spec."]
pub struct GOIDLEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GOIDLEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GOIDLEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GOIDLEIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GOIDLEIEN_A::_1)
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
#[doc = "IDSTS Changed Interrupt Enable Bit\\nIf this bit is set to 1 and IDSTS (OTG_STATUS\\[1\\]) status is changed from high to low or from low to high, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDCHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<IDCHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: IDCHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDCHGIEN` reader - IDSTS Changed Interrupt Enable Bit\\nIf this bit is set to 1 and IDSTS (OTG_STATUS\\[1\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct IDCHGIEN_R(crate::FieldReader<bool, IDCHGIEN_A>);
impl IDCHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDCHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDCHGIEN_A {
        match self.bits {
            false => IDCHGIEN_A::_0,
            true => IDCHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDCHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDCHGIEN_A::_1
    }
}
impl core::ops::Deref for IDCHGIEN_R {
    type Target = crate::FieldReader<bool, IDCHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDCHGIEN` writer - IDSTS Changed Interrupt Enable Bit\\nIf this bit is set to 1 and IDSTS (OTG_STATUS\\[1\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct IDCHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDCHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDCHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDCHGIEN_A::_1)
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
#[doc = "Act As Peripheral Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a peripheral, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEVIEN_A {
    #[doc = "0: This device as a peripheral interrupt Disabled"]
    _0 = 0,
    #[doc = "1: This device as a peripheral interrupt Enabled"]
    _1 = 1,
}
impl From<PDEVIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEVIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEVIEN` reader - Act As Peripheral Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a peripheral, a interrupt will be asserted."]
pub struct PDEVIEN_R(crate::FieldReader<bool, PDEVIEN_A>);
impl PDEVIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEVIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEVIEN_A {
        match self.bits {
            false => PDEVIEN_A::_0,
            true => PDEVIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDEVIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDEVIEN_A::_1
    }
}
impl core::ops::Deref for PDEVIEN_R {
    type Target = crate::FieldReader<bool, PDEVIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEVIEN` writer - Act As Peripheral Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a peripheral, a interrupt will be asserted."]
pub struct PDEVIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEVIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEVIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This device as a peripheral interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDEVIEN_A::_0)
    }
    #[doc = "This device as a peripheral interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDEVIEN_A::_1)
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
#[doc = "Act As Host Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a host, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTIEN_A {
    #[doc = "0: This device as a host interrupt Disabled"]
    _0 = 0,
    #[doc = "1: This device as a host interrupt Enabled"]
    _1 = 1,
}
impl From<HOSTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTIEN` reader - Act As Host Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a host, a interrupt will be asserted."]
pub struct HOSTIEN_R(crate::FieldReader<bool, HOSTIEN_A>);
impl HOSTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTIEN_A {
        match self.bits {
            false => HOSTIEN_A::_0,
            true => HOSTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HOSTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HOSTIEN_A::_1
    }
}
impl core::ops::Deref for HOSTIEN_R {
    type Target = crate::FieldReader<bool, HOSTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTIEN` writer - Act As Host Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a host, a interrupt will be asserted."]
pub struct HOSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOSTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This device as a host interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOSTIEN_A::_0)
    }
    #[doc = "This device as a host interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOSTIEN_A::_1)
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
#[doc = "B-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and BVLD (OTG_STATUS\\[3\\]) status is changed from high to low or from low to high, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVLDCHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<BVLDCHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BVLDCHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BVLDCHGIEN` reader - B-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and BVLD (OTG_STATUS\\[3\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct BVLDCHGIEN_R(crate::FieldReader<bool, BVLDCHGIEN_A>);
impl BVLDCHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVLDCHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVLDCHGIEN_A {
        match self.bits {
            false => BVLDCHGIEN_A::_0,
            true => BVLDCHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BVLDCHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BVLDCHGIEN_A::_1
    }
}
impl core::ops::Deref for BVLDCHGIEN_R {
    type Target = crate::FieldReader<bool, BVLDCHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVLDCHGIEN` writer - B-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and BVLD (OTG_STATUS\\[3\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct BVLDCHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BVLDCHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BVLDCHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BVLDCHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BVLDCHGIEN_A::_1)
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
#[doc = "A-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and AVLD (OTG_STATUS\\[4\\]) status is changed from high to low or from low to high, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLDCHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<AVLDCHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AVLDCHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLDCHGIEN` reader - A-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and AVLD (OTG_STATUS\\[4\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct AVLDCHGIEN_R(crate::FieldReader<bool, AVLDCHGIEN_A>);
impl AVLDCHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVLDCHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLDCHGIEN_A {
        match self.bits {
            false => AVLDCHGIEN_A::_0,
            true => AVLDCHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVLDCHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVLDCHGIEN_A::_1
    }
}
impl core::ops::Deref for AVLDCHGIEN_R {
    type Target = crate::FieldReader<bool, AVLDCHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVLDCHGIEN` writer - A-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and AVLD (OTG_STATUS\\[4\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct AVLDCHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLDCHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLDCHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLDCHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLDCHGIEN_A::_1)
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
#[doc = "VBUSVLD Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and VBUSVLD (OTG_STATUS\\[5\\]) status is changed from high to low or from low to high, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBCHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<VBCHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBCHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBCHGIEN` reader - VBUSVLD Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and VBUSVLD (OTG_STATUS\\[5\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct VBCHGIEN_R(crate::FieldReader<bool, VBCHGIEN_A>);
impl VBCHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBCHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBCHGIEN_A {
        match self.bits {
            false => VBCHGIEN_A::_0,
            true => VBCHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBCHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBCHGIEN_A::_1
    }
}
impl core::ops::Deref for VBCHGIEN_R {
    type Target = crate::FieldReader<bool, VBCHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBCHGIEN` writer - VBUSVLD Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and VBUSVLD (OTG_STATUS\\[5\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct VBCHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBCHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBCHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBCHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBCHGIEN_A::_1)
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
#[doc = "SESSEND Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and SESSEND (OTG_STATUS\\[2\\]) status is changed from high to low or from low to high, a interrupt will be asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECHGIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<SECHGIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SECHGIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECHGIEN` reader - SESSEND Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and SESSEND (OTG_STATUS\\[2\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct SECHGIEN_R(crate::FieldReader<bool, SECHGIEN_A>);
impl SECHGIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECHGIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECHGIEN_A {
        match self.bits {
            false => SECHGIEN_A::_0,
            true => SECHGIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SECHGIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SECHGIEN_A::_1
    }
}
impl core::ops::Deref for SECHGIEN_R {
    type Target = crate::FieldReader<bool, SECHGIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECHGIEN` writer - SESSEND Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and SESSEND (OTG_STATUS\\[2\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
pub struct SECHGIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SECHGIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECHGIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECHGIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECHGIEN_A::_1)
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
#[doc = "SRP Detected Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRPDETIEN_A {
    #[doc = "0: Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Enabled"]
    _1 = 1,
}
impl From<SRPDETIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRPDETIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPDETIEN` reader - SRP Detected Interrupt Enable Bit"]
pub struct SRPDETIEN_R(crate::FieldReader<bool, SRPDETIEN_A>);
impl SRPDETIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRPDETIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPDETIEN_A {
        match self.bits {
            false => SRPDETIEN_A::_0,
            true => SRPDETIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRPDETIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRPDETIEN_A::_1
    }
}
impl core::ops::Deref for SRPDETIEN_R {
    type Target = crate::FieldReader<bool, SRPDETIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPDETIEN` writer - SRP Detected Interrupt Enable Bit"]
pub struct SRPDETIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPDETIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPDETIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRPDETIEN_A::_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRPDETIEN_A::_1)
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
    #[doc = "Bit 0 - Role (Host or Peripheral) Changed Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rolechgien(&self) -> ROLECHGIEN_R {
        ROLECHGIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Error Interrupt Enable Bit\\nNote: VBUS error means going to a_vbus_err state. Please refer to A-device state diagram in OTG spec."]
    #[inline(always)]
    pub fn vbeien(&self) -> VBEIEN_R {
        VBEIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn srpfien(&self) -> SRPFIEN_R {
        SRPFIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HNP Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hnpfien(&self) -> HNPFIEN_R {
        HNPFIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OTG Device Goes to IDLE State Interrupt Enable Bit\\nNote: Going to idle state means going to a_idle or b_idle state. Please refer to A-device state diagram and B-device state diagram in OTG spec."]
    #[inline(always)]
    pub fn goidleien(&self) -> GOIDLEIEN_R {
        GOIDLEIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IDSTS Changed Interrupt Enable Bit\\nIf this bit is set to 1 and IDSTS (OTG_STATUS\\[1\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn idchgien(&self) -> IDCHGIEN_R {
        IDCHGIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Act As Peripheral Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a peripheral, a interrupt will be asserted."]
    #[inline(always)]
    pub fn pdevien(&self) -> PDEVIEN_R {
        PDEVIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Act As Host Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a host, a interrupt will be asserted."]
    #[inline(always)]
    pub fn hostien(&self) -> HOSTIEN_R {
        HOSTIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and BVLD (OTG_STATUS\\[3\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn bvldchgien(&self) -> BVLDCHGIEN_R {
        BVLDCHGIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and AVLD (OTG_STATUS\\[4\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn avldchgien(&self) -> AVLDCHGIEN_R {
        AVLDCHGIEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBUSVLD Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and VBUSVLD (OTG_STATUS\\[5\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn vbchgien(&self) -> VBCHGIEN_R {
        VBCHGIEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SESSEND Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and SESSEND (OTG_STATUS\\[2\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn sechgien(&self) -> SECHGIEN_R {
        SECHGIEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SRP Detected Interrupt Enable Bit"]
    #[inline(always)]
    pub fn srpdetien(&self) -> SRPDETIEN_R {
        SRPDETIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Role (Host or Peripheral) Changed Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rolechgien(&mut self) -> ROLECHGIEN_W {
        ROLECHGIEN_W { w: self }
    }
    #[doc = "Bit 1 - VBUS Error Interrupt Enable Bit\\nNote: VBUS error means going to a_vbus_err state. Please refer to A-device state diagram in OTG spec."]
    #[inline(always)]
    pub fn vbeien(&mut self) -> VBEIEN_W {
        VBEIEN_W { w: self }
    }
    #[doc = "Bit 2 - SRP Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn srpfien(&mut self) -> SRPFIEN_W {
        SRPFIEN_W { w: self }
    }
    #[doc = "Bit 3 - HNP Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hnpfien(&mut self) -> HNPFIEN_W {
        HNPFIEN_W { w: self }
    }
    #[doc = "Bit 4 - OTG Device Goes to IDLE State Interrupt Enable Bit\\nNote: Going to idle state means going to a_idle or b_idle state. Please refer to A-device state diagram and B-device state diagram in OTG spec."]
    #[inline(always)]
    pub fn goidleien(&mut self) -> GOIDLEIEN_W {
        GOIDLEIEN_W { w: self }
    }
    #[doc = "Bit 5 - IDSTS Changed Interrupt Enable Bit\\nIf this bit is set to 1 and IDSTS (OTG_STATUS\\[1\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn idchgien(&mut self) -> IDCHGIEN_W {
        IDCHGIEN_W { w: self }
    }
    #[doc = "Bit 6 - Act As Peripheral Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a peripheral, a interrupt will be asserted."]
    #[inline(always)]
    pub fn pdevien(&mut self) -> PDEVIEN_W {
        PDEVIEN_W { w: self }
    }
    #[doc = "Bit 7 - Act As Host Interrupt Enable Bit\\nIf this bit is set to 1 and the device is changed as a host, a interrupt will be asserted."]
    #[inline(always)]
    pub fn hostien(&mut self) -> HOSTIEN_W {
        HOSTIEN_W { w: self }
    }
    #[doc = "Bit 8 - B-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and BVLD (OTG_STATUS\\[3\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn bvldchgien(&mut self) -> BVLDCHGIEN_W {
        BVLDCHGIEN_W { w: self }
    }
    #[doc = "Bit 9 - A-device Session Valid Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and AVLD (OTG_STATUS\\[4\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn avldchgien(&mut self) -> AVLDCHGIEN_W {
        AVLDCHGIEN_W { w: self }
    }
    #[doc = "Bit 10 - VBUSVLD Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and VBUSVLD (OTG_STATUS\\[5\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn vbchgien(&mut self) -> VBCHGIEN_W {
        VBCHGIEN_W { w: self }
    }
    #[doc = "Bit 11 - SESSEND Status Changed Interrupt Enable Bit\\nIf this bit is set to 1 and SESSEND (OTG_STATUS\\[2\\]) status is changed from high to low or from low to high, a interrupt will be asserted."]
    #[inline(always)]
    pub fn sechgien(&mut self) -> SECHGIEN_W {
        SECHGIEN_W { w: self }
    }
    #[doc = "Bit 13 - SRP Detected Interrupt Enable Bit"]
    #[inline(always)]
    pub fn srpdetien(&mut self) -> SRPDETIEN_W {
        SRPDETIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_inten](index.html) module"]
pub struct OTG_INTEN_SPEC;
impl crate::RegisterSpec for OTG_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_inten::R](R) reader structure"]
impl crate::Readable for OTG_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_inten::W](W) writer structure"]
impl crate::Writable for OTG_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_INTEN to value 0"]
impl crate::Resettable for OTG_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
