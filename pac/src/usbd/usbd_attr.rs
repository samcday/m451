#[doc = "Register `USBD_ATTR` reader"]
pub struct R(crate::R<USBD_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBD_ATTR_SPEC>> for R {
    fn from(reader: crate::R<USBD_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_ATTR` writer"]
pub struct W(crate::W<USBD_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_ATTR_SPEC>;
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
impl core::convert::From<crate::W<USBD_ATTR_SPEC>> for W {
    fn from(writer: crate::W<USBD_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Reset Status\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRST_A {
    #[doc = "0: Bus no reset"]
    _0 = 0,
    #[doc = "1: Bus reset when SE0 (single-ended 0) more than 2.5us"]
    _1 = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB Reset Status\\nNote: This bit is read only."]
pub struct USBRST_R(crate::FieldReader<bool, USBRST_A>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::_0,
            true => USBRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBRST_A::_1
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, USBRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRST` writer - USB Reset Status\\nNote: This bit is read only."]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus no reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRST_A::_0)
    }
    #[doc = "Bus reset when SE0 (single-ended 0) more than 2.5us"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRST_A::_1)
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
#[doc = "Suspend Status\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPEND_A {
    #[doc = "0: Bus no suspend"]
    _0 = 0,
    #[doc = "1: Bus idle more than 3 ms, either cable is plugged off or host is sleeping"]
    _1 = 1,
}
impl From<SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND` reader - Suspend Status\\nNote: This bit is read only."]
pub struct SUSPEND_R(crate::FieldReader<bool, SUSPEND_A>);
impl SUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPEND_A {
        match self.bits {
            false => SUSPEND_A::_0,
            true => SUSPEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SUSPEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SUSPEND_A::_1
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, SUSPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` writer - Suspend Status\\nNote: This bit is read only."]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus no suspend"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSPEND_A::_0)
    }
    #[doc = "Bus idle more than 3 ms, either cable is plugged off or host is sleeping"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSPEND_A::_1)
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
#[doc = "Resume Status\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUME_A {
    #[doc = "0: No bus resume"]
    _0 = 0,
    #[doc = "1: Resume from suspend"]
    _1 = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Resume Status\\nNote: This bit is read only."]
pub struct RESUME_R(crate::FieldReader<bool, RESUME_A>);
impl RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUME_A {
        match self.bits {
            false => RESUME_A::_0,
            true => RESUME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RESUME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RESUME_A::_1
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, RESUME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Resume Status\\nNote: This bit is read only."]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESUME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No bus resume"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESUME_A::_0)
    }
    #[doc = "Resume from suspend"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESUME_A::_1)
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
#[doc = "Time-out Status\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUT_A {
    #[doc = "0: No time-out"]
    _0 = 0,
    #[doc = "1: No Bus response more than 18 bits time"]
    _1 = 1,
}
impl From<TOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUT` reader - Time-out Status\\nNote: This bit is read only."]
pub struct TOUT_R(crate::FieldReader<bool, TOUT_A>);
impl TOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUT_A {
        match self.bits {
            false => TOUT_A::_0,
            true => TOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUT_A::_1
    }
}
impl core::ops::Deref for TOUT_R {
    type Target = crate::FieldReader<bool, TOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT` writer - Time-out Status\\nNote: This bit is read only."]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUT_A::_0)
    }
    #[doc = "No Bus response more than 18 bits time"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUT_A::_1)
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
#[doc = "PHY Transceiver Function Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEN_A {
    #[doc = "0: PHY transceiver function Disabled"]
    _0 = 0,
    #[doc = "1: PHY transceiver function Enabled"]
    _1 = 1,
}
impl From<PHYEN_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEN` reader - PHY Transceiver Function Enable Bit"]
pub struct PHYEN_R(crate::FieldReader<bool, PHYEN_A>);
impl PHYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEN_A {
        match self.bits {
            false => PHYEN_A::_0,
            true => PHYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PHYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PHYEN_A::_1
    }
}
impl core::ops::Deref for PHYEN_R {
    type Target = crate::FieldReader<bool, PHYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYEN` writer - PHY Transceiver Function Enable Bit"]
pub struct PHYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PHY transceiver function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHYEN_A::_0)
    }
    #[doc = "PHY transceiver function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHYEN_A::_1)
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
#[doc = "Remote Wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWAKEUP_A {
    #[doc = "0: Release the USB bus from K state"]
    _0 = 0,
    #[doc = "1: Force USB bus to K (USB_D+ low and USB_D- high) state, used for remote wake-up"]
    _1 = 1,
}
impl From<RWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: RWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWAKEUP` reader - Remote Wake-up"]
pub struct RWAKEUP_R(crate::FieldReader<bool, RWAKEUP_A>);
impl RWAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWAKEUP_A {
        match self.bits {
            false => RWAKEUP_A::_0,
            true => RWAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RWAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RWAKEUP_A::_1
    }
}
impl core::ops::Deref for RWAKEUP_R {
    type Target = crate::FieldReader<bool, RWAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWAKEUP` writer - Remote Wake-up"]
pub struct RWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWAKEUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Release the USB bus from K state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWAKEUP_A::_0)
    }
    #[doc = "Force USB bus to K (USB_D+ low and USB_D- high) state, used for remote wake-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWAKEUP_A::_1)
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
#[doc = "USB Controller Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEN_A {
    #[doc = "0: USB Controller Disabled"]
    _0 = 0,
    #[doc = "1: USB Controller Enabled"]
    _1 = 1,
}
impl From<USBEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEN` reader - USB Controller Enable Bit"]
pub struct USBEN_R(crate::FieldReader<bool, USBEN_A>);
impl USBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEN_A {
        match self.bits {
            false => USBEN_A::_0,
            true => USBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBEN_A::_1
    }
}
impl core::ops::Deref for USBEN_R {
    type Target = crate::FieldReader<bool, USBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBEN` writer - USB Controller Enable Bit"]
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB Controller Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBEN_A::_0)
    }
    #[doc = "USB Controller Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBEN_A::_1)
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
#[doc = "Pull-up Resistor on USB_DP Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPUEN_A {
    #[doc = "0: Pull-up resistor in USB_D+ bus Disabled"]
    _0 = 0,
    #[doc = "1: Pull-up resistor in USB_D+ bus Active"]
    _1 = 1,
}
impl From<DPPUEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPPUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPUEN` reader - Pull-up Resistor on USB_DP Enable Bit"]
pub struct DPPUEN_R(crate::FieldReader<bool, DPPUEN_A>);
impl DPPUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPUEN_A {
        match self.bits {
            false => DPPUEN_A::_0,
            true => DPPUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPPUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPPUEN_A::_1
    }
}
impl core::ops::Deref for DPPUEN_R {
    type Target = crate::FieldReader<bool, DPPUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPPUEN` writer - Pull-up Resistor on USB_DP Enable Bit"]
pub struct DPPUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPPUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pull-up resistor in USB_D+ bus Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPPUEN_A::_0)
    }
    #[doc = "Pull-up resistor in USB_D+ bus Active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPPUEN_A::_1)
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
#[doc = "Power-down PHY Transceiver, Low Active (M45xD/M45xC Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRDN_A {
    #[doc = "0: Power-down related circuits of PHY transceiver"]
    _0 = 0,
    #[doc = "1: Turn-on related circuits of PHY transceiver"]
    _1 = 1,
}
impl From<PWRDN_A> for bool {
    #[inline(always)]
    fn from(variant: PWRDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDN` reader - Power-down PHY Transceiver, Low Active (M45xD/M45xC Only)"]
pub struct PWRDN_R(crate::FieldReader<bool, PWRDN_A>);
impl PWRDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRDN_A {
        match self.bits {
            false => PWRDN_A::_0,
            true => PWRDN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWRDN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWRDN_A::_1
    }
}
impl core::ops::Deref for PWRDN_R {
    type Target = crate::FieldReader<bool, PWRDN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRDN` writer - Power-down PHY Transceiver, Low Active (M45xD/M45xC Only)"]
pub struct PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRDN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power-down related circuits of PHY transceiver"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWRDN_A::_0)
    }
    #[doc = "Turn-on related circuits of PHY transceiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWRDN_A::_1)
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
#[doc = "CPU Access USB SRAM Size Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTEM_A {
    #[doc = "0: Word mode: The size of the transfer from CPU to USB SRAM can be Word only"]
    _0 = 0,
    #[doc = "1: Byte mode: The size of the transfer from CPU to USB SRAM can be Byte only"]
    _1 = 1,
}
impl From<BYTEM_A> for bool {
    #[inline(always)]
    fn from(variant: BYTEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYTEM` reader - CPU Access USB SRAM Size Mode Selection"]
pub struct BYTEM_R(crate::FieldReader<bool, BYTEM_A>);
impl BYTEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYTEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTEM_A {
        match self.bits {
            false => BYTEM_A::_0,
            true => BYTEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BYTEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BYTEM_A::_1
    }
}
impl core::ops::Deref for BYTEM_R {
    type Target = crate::FieldReader<bool, BYTEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTEM` writer - CPU Access USB SRAM Size Mode Selection"]
pub struct BYTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYTEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Word mode: The size of the transfer from CPU to USB SRAM can be Word only"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYTEM_A::_0)
    }
    #[doc = "Byte mode: The size of the transfer from CPU to USB SRAM can be Byte only"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYTEM_A::_1)
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
impl R {
    #[doc = "Bit 0 - USB Reset Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Suspend Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Resume Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time-out Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY Transceiver Function Enable Bit"]
    #[inline(always)]
    pub fn phyen(&self) -> PHYEN_R {
        PHYEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Remote Wake-up"]
    #[inline(always)]
    pub fn rwakeup(&self) -> RWAKEUP_R {
        RWAKEUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Controller Enable Bit"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull-up Resistor on USB_DP Enable Bit"]
    #[inline(always)]
    pub fn dppuen(&self) -> DPPUEN_R {
        DPPUEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Power-down PHY Transceiver, Low Active (M45xD/M45xC Only)"]
    #[inline(always)]
    pub fn pwrdn(&self) -> PWRDN_R {
        PWRDN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU Access USB SRAM Size Mode Selection"]
    #[inline(always)]
    pub fn bytem(&self) -> BYTEM_R {
        BYTEM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 1 - Suspend Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 2 - Resume Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3 - Time-out Status\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bit 4 - PHY Transceiver Function Enable Bit"]
    #[inline(always)]
    pub fn phyen(&mut self) -> PHYEN_W {
        PHYEN_W { w: self }
    }
    #[doc = "Bit 5 - Remote Wake-up"]
    #[inline(always)]
    pub fn rwakeup(&mut self) -> RWAKEUP_W {
        RWAKEUP_W { w: self }
    }
    #[doc = "Bit 7 - USB Controller Enable Bit"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
    #[doc = "Bit 8 - Pull-up Resistor on USB_DP Enable Bit"]
    #[inline(always)]
    pub fn dppuen(&mut self) -> DPPUEN_W {
        DPPUEN_W { w: self }
    }
    #[doc = "Bit 9 - Power-down PHY Transceiver, Low Active (M45xD/M45xC Only)"]
    #[inline(always)]
    pub fn pwrdn(&mut self) -> PWRDN_W {
        PWRDN_W { w: self }
    }
    #[doc = "Bit 10 - CPU Access USB SRAM Size Mode Selection"]
    #[inline(always)]
    pub fn bytem(&mut self) -> BYTEM_W {
        BYTEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Bus Status and Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_attr](index.html) module"]
pub struct USBD_ATTR_SPEC;
impl crate::RegisterSpec for USBD_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_attr::R](R) reader structure"]
impl crate::Readable for USBD_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_attr::W](W) writer structure"]
impl crate::Writable for USBD_ATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_ATTR to value 0x40"]
impl crate::Resettable for USBD_ATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
