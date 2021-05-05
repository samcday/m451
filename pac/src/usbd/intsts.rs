#[doc = "Register `INTSTS` reader"]
pub struct R(crate::R<INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTS_SPEC>> for R {
    fn from(reader: crate::R<INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS` writer"]
pub struct W(crate::W<INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS_SPEC>;
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
impl core::convert::From<crate::W<INTSTS_SPEC>> for W {
    fn from(writer: crate::W<INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BUS Interrupt Status\\nThe BUS event means that there is one of the suspense or the resume function in the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSIF_A {
    #[doc = "0: No BUS event occurred"]
    _0 = 0,
    #[doc = "1: Bus event occurred; check USBD_ATTR\\[3:0\\]
to know which kind of bus event was occurred, cleared by write 1 to USBD_INTSTS\\[0\\]"]
    _1 = 1,
}
impl From<BUSIF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSIF` reader - BUS Interrupt Status\\nThe BUS event means that there is one of the suspense or the resume function in the bus."]
pub struct BUSIF_R(crate::FieldReader<bool, BUSIF_A>);
impl BUSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSIF_A {
        match self.bits {
            false => BUSIF_A::_0,
            true => BUSIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSIF_A::_1
    }
}
impl core::ops::Deref for BUSIF_R {
    type Target = crate::FieldReader<bool, BUSIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSIF` writer - BUS Interrupt Status\\nThe BUS event means that there is one of the suspense or the resume function in the bus."]
pub struct BUSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No BUS event occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSIF_A::_0)
    }
    #[doc = "Bus event occurred; check USBD_ATTR\\[3:0\\]
to know which kind of bus event was occurred, cleared by write 1 to USBD_INTSTS\\[0\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSIF_A::_1)
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
#[doc = "USB Event Interrupt Status\\nThe USB event includes the SETUP Token, IN Token, OUT ACK, ISO IN or ISO OUT events in the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBIF_A {
    #[doc = "0: No USB event occurred"]
    _0 = 0,
    #[doc = "1: USB event occurred, check EPSTS0~5\\[2:0\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[1\\]
or EPSTS0~7 and SETUP (USBD_INTSTS\\[31\\])"]
    _1 = 1,
}
impl From<USBIF_A> for bool {
    #[inline(always)]
    fn from(variant: USBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBIF` reader - USB Event Interrupt Status\\nThe USB event includes the SETUP Token, IN Token, OUT ACK, ISO IN or ISO OUT events in the bus."]
pub struct USBIF_R(crate::FieldReader<bool, USBIF_A>);
impl USBIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBIF_A {
        match self.bits {
            false => USBIF_A::_0,
            true => USBIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBIF_A::_1
    }
}
impl core::ops::Deref for USBIF_R {
    type Target = crate::FieldReader<bool, USBIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBIF` writer - USB Event Interrupt Status\\nThe USB event includes the SETUP Token, IN Token, OUT ACK, ISO IN or ISO OUT events in the bus."]
pub struct USBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No USB event occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBIF_A::_0)
    }
    #[doc = "USB event occurred, check EPSTS0~5\\[2:0\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[1\\]
or EPSTS0~7 and SETUP (USBD_INTSTS\\[31\\])"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBIF_A::_1)
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
#[doc = "VBUS Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBDETIF_A {
    #[doc = "0: There is not attached/detached event in the USB"]
    _0 = 0,
    #[doc = "1: There is attached/detached event in the USB bus and it is cleared by write 1 to USBD_INTSTS\\[2\\]"]
    _1 = 1,
}
impl From<VBDETIF_A> for bool {
    #[inline(always)]
    fn from(variant: VBDETIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBDETIF` reader - VBUS Detection Interrupt Status"]
pub struct VBDETIF_R(crate::FieldReader<bool, VBDETIF_A>);
impl VBDETIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBDETIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBDETIF_A {
        match self.bits {
            false => VBDETIF_A::_0,
            true => VBDETIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBDETIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBDETIF_A::_1
    }
}
impl core::ops::Deref for VBDETIF_R {
    type Target = crate::FieldReader<bool, VBDETIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBDETIF` writer - VBUS Detection Interrupt Status"]
pub struct VBDETIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VBDETIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBDETIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "There is not attached/detached event in the USB"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBDETIF_A::_0)
    }
    #[doc = "There is attached/detached event in the USB bus and it is cleared by write 1 to USBD_INTSTS\\[2\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBDETIF_A::_1)
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
#[doc = "No-event-wake-up Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEVWKIF_A {
    #[doc = "0: NEVWK event does not occur"]
    _0 = 0,
    #[doc = "1: No-event-wake-up event occurred, cleared by write 1 to USBD_INTSTS\\[3\\]"]
    _1 = 1,
}
impl From<NEVWKIF_A> for bool {
    #[inline(always)]
    fn from(variant: NEVWKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEVWKIF` reader - No-event-wake-up Interrupt Status"]
pub struct NEVWKIF_R(crate::FieldReader<bool, NEVWKIF_A>);
impl NEVWKIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEVWKIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEVWKIF_A {
        match self.bits {
            false => NEVWKIF_A::_0,
            true => NEVWKIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEVWKIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEVWKIF_A::_1
    }
}
impl core::ops::Deref for NEVWKIF_R {
    type Target = crate::FieldReader<bool, NEVWKIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEVWKIF` writer - No-event-wake-up Interrupt Status"]
pub struct NEVWKIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NEVWKIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEVWKIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NEVWK event does not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEVWKIF_A::_0)
    }
    #[doc = "No-event-wake-up event occurred, cleared by write 1 to USBD_INTSTS\\[3\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEVWKIF_A::_1)
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
#[doc = "Endpoint 0's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT0_A {
    #[doc = "0: No event occurred in endpoint 0"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 0, check USBD_EPSTS\\[10:8\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[16\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT0_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT0` reader - Endpoint 0's USB Event Status"]
pub struct EPEVT0_R(crate::FieldReader<bool, EPEVT0_A>);
impl EPEVT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT0_A {
        match self.bits {
            false => EPEVT0_A::_0,
            true => EPEVT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT0_A::_1
    }
}
impl core::ops::Deref for EPEVT0_R {
    type Target = crate::FieldReader<bool, EPEVT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT0` writer - Endpoint 0's USB Event Status"]
pub struct EPEVT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT0_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 0, check USBD_EPSTS\\[10:8\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[16\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT0_A::_1)
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
#[doc = "Endpoint 1's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT1_A {
    #[doc = "0: No event occurred in endpoint 1"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 1, check USBD_EPSTS\\[13:11\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[17\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT1_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT1` reader - Endpoint 1's USB Event Status"]
pub struct EPEVT1_R(crate::FieldReader<bool, EPEVT1_A>);
impl EPEVT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT1_A {
        match self.bits {
            false => EPEVT1_A::_0,
            true => EPEVT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT1_A::_1
    }
}
impl core::ops::Deref for EPEVT1_R {
    type Target = crate::FieldReader<bool, EPEVT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT1` writer - Endpoint 1's USB Event Status"]
pub struct EPEVT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT1_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 1, check USBD_EPSTS\\[13:11\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[17\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT1_A::_1)
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
#[doc = "Endpoint 2's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT2_A {
    #[doc = "0: No event occurred in endpoint 2"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 2, check USBD_EPSTS\\[16:14\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[18\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT2_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT2` reader - Endpoint 2's USB Event Status"]
pub struct EPEVT2_R(crate::FieldReader<bool, EPEVT2_A>);
impl EPEVT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT2_A {
        match self.bits {
            false => EPEVT2_A::_0,
            true => EPEVT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT2_A::_1
    }
}
impl core::ops::Deref for EPEVT2_R {
    type Target = crate::FieldReader<bool, EPEVT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT2` writer - Endpoint 2's USB Event Status"]
pub struct EPEVT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT2_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 2, check USBD_EPSTS\\[16:14\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[18\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Endpoint 3's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT3_A {
    #[doc = "0: No event occurred in endpoint 3"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 3, check USBD_EPSTS\\[19:17\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[19\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT3_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT3` reader - Endpoint 3's USB Event Status"]
pub struct EPEVT3_R(crate::FieldReader<bool, EPEVT3_A>);
impl EPEVT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT3_A {
        match self.bits {
            false => EPEVT3_A::_0,
            true => EPEVT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT3_A::_1
    }
}
impl core::ops::Deref for EPEVT3_R {
    type Target = crate::FieldReader<bool, EPEVT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT3` writer - Endpoint 3's USB Event Status"]
pub struct EPEVT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 3"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT3_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 3, check USBD_EPSTS\\[19:17\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[19\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT3_A::_1)
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
#[doc = "Endpoint 4's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT4_A {
    #[doc = "0: No event occurred in endpoint 4"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 4, check USBD_EPSTS\\[22:20\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[20\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT4_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT4` reader - Endpoint 4's USB Event Status"]
pub struct EPEVT4_R(crate::FieldReader<bool, EPEVT4_A>);
impl EPEVT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT4_A {
        match self.bits {
            false => EPEVT4_A::_0,
            true => EPEVT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT4_A::_1
    }
}
impl core::ops::Deref for EPEVT4_R {
    type Target = crate::FieldReader<bool, EPEVT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT4` writer - Endpoint 4's USB Event Status"]
pub struct EPEVT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT4_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 4, check USBD_EPSTS\\[22:20\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[20\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT4_A::_1)
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
#[doc = "Endpoint 5's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT5_A {
    #[doc = "0: No event occurred in endpoint 5"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 5, check USBD_EPSTS\\[25:23\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[21\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT5_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT5` reader - Endpoint 5's USB Event Status"]
pub struct EPEVT5_R(crate::FieldReader<bool, EPEVT5_A>);
impl EPEVT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT5_A {
        match self.bits {
            false => EPEVT5_A::_0,
            true => EPEVT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT5_A::_1
    }
}
impl core::ops::Deref for EPEVT5_R {
    type Target = crate::FieldReader<bool, EPEVT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT5` writer - Endpoint 5's USB Event Status"]
pub struct EPEVT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 5"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT5_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 5, check USBD_EPSTS\\[25:23\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[21\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT5_A::_1)
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
#[doc = "Endpoint 6's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT6_A {
    #[doc = "0: No event occurred in endpoint 6"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 6, check USBD_EPSTS\\[28:26\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[22\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT6_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT6` reader - Endpoint 6's USB Event Status"]
pub struct EPEVT6_R(crate::FieldReader<bool, EPEVT6_A>);
impl EPEVT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT6_A {
        match self.bits {
            false => EPEVT6_A::_0,
            true => EPEVT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT6_A::_1
    }
}
impl core::ops::Deref for EPEVT6_R {
    type Target = crate::FieldReader<bool, EPEVT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT6` writer - Endpoint 6's USB Event Status"]
pub struct EPEVT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 6"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT6_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 6, check USBD_EPSTS\\[28:26\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[22\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Endpoint 7's USB Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEVT7_A {
    #[doc = "0: No event occurred in endpoint 7"]
    _0 = 0,
    #[doc = "1: USB event occurred on Endpoint 7, check USBD_EPSTS\\[31:29\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[23\\]
or USBD_INTSTS\\[1\\]"]
    _1 = 1,
}
impl From<EPEVT7_A> for bool {
    #[inline(always)]
    fn from(variant: EPEVT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEVT7` reader - Endpoint 7's USB Event Status"]
pub struct EPEVT7_R(crate::FieldReader<bool, EPEVT7_A>);
impl EPEVT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEVT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEVT7_A {
        match self.bits {
            false => EPEVT7_A::_0,
            true => EPEVT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPEVT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPEVT7_A::_1
    }
}
impl core::ops::Deref for EPEVT7_R {
    type Target = crate::FieldReader<bool, EPEVT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEVT7` writer - Endpoint 7's USB Event Status"]
pub struct EPEVT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEVT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event occurred in endpoint 7"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPEVT7_A::_0)
    }
    #[doc = "USB event occurred on Endpoint 7, check USBD_EPSTS\\[31:29\\]
to know which kind of USB event was occurred, cleared by write 1 to USBD_INTSTS\\[23\\]
or USBD_INTSTS\\[1\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPEVT7_A::_1)
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
#[doc = "Setup Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETUP_A {
    #[doc = "0: No Setup event"]
    _0 = 0,
    #[doc = "1: Setup event occurred, cleared by write 1 to USBD_INTSTS\\[31\\]"]
    _1 = 1,
}
impl From<SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: SETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETUP` reader - Setup Event Status"]
pub struct SETUP_R(crate::FieldReader<bool, SETUP_A>);
impl SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETUP_A {
        match self.bits {
            false => SETUP_A::_0,
            true => SETUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SETUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SETUP_A::_1
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, SETUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP` writer - Setup Event Status"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Setup event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SETUP_A::_0)
    }
    #[doc = "Setup event occurred, cleared by write 1 to USBD_INTSTS\\[31\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SETUP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BUS Interrupt Status\\nThe BUS event means that there is one of the suspense or the resume function in the bus."]
    #[inline(always)]
    pub fn busif(&self) -> BUSIF_R {
        BUSIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Event Interrupt Status\\nThe USB event includes the SETUP Token, IN Token, OUT ACK, ISO IN or ISO OUT events in the bus."]
    #[inline(always)]
    pub fn usbif(&self) -> USBIF_R {
        USBIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS Detection Interrupt Status"]
    #[inline(always)]
    pub fn vbdetif(&self) -> VBDETIF_R {
        VBDETIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - No-event-wake-up Interrupt Status"]
    #[inline(always)]
    pub fn nevwkif(&self) -> NEVWKIF_R {
        NEVWKIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0's USB Event Status"]
    #[inline(always)]
    pub fn epevt0(&self) -> EPEVT0_R {
        EPEVT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1's USB Event Status"]
    #[inline(always)]
    pub fn epevt1(&self) -> EPEVT1_R {
        EPEVT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2's USB Event Status"]
    #[inline(always)]
    pub fn epevt2(&self) -> EPEVT2_R {
        EPEVT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3's USB Event Status"]
    #[inline(always)]
    pub fn epevt3(&self) -> EPEVT3_R {
        EPEVT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4's USB Event Status"]
    #[inline(always)]
    pub fn epevt4(&self) -> EPEVT4_R {
        EPEVT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5's USB Event Status"]
    #[inline(always)]
    pub fn epevt5(&self) -> EPEVT5_R {
        EPEVT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6's USB Event Status"]
    #[inline(always)]
    pub fn epevt6(&self) -> EPEVT6_R {
        EPEVT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7's USB Event Status"]
    #[inline(always)]
    pub fn epevt7(&self) -> EPEVT7_R {
        EPEVT7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Setup Event Status"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS Interrupt Status\\nThe BUS event means that there is one of the suspense or the resume function in the bus."]
    #[inline(always)]
    pub fn busif(&mut self) -> BUSIF_W {
        BUSIF_W { w: self }
    }
    #[doc = "Bit 1 - USB Event Interrupt Status\\nThe USB event includes the SETUP Token, IN Token, OUT ACK, ISO IN or ISO OUT events in the bus."]
    #[inline(always)]
    pub fn usbif(&mut self) -> USBIF_W {
        USBIF_W { w: self }
    }
    #[doc = "Bit 2 - VBUS Detection Interrupt Status"]
    #[inline(always)]
    pub fn vbdetif(&mut self) -> VBDETIF_W {
        VBDETIF_W { w: self }
    }
    #[doc = "Bit 3 - No-event-wake-up Interrupt Status"]
    #[inline(always)]
    pub fn nevwkif(&mut self) -> NEVWKIF_W {
        NEVWKIF_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint 0's USB Event Status"]
    #[inline(always)]
    pub fn epevt0(&mut self) -> EPEVT0_W {
        EPEVT0_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint 1's USB Event Status"]
    #[inline(always)]
    pub fn epevt1(&mut self) -> EPEVT1_W {
        EPEVT1_W { w: self }
    }
    #[doc = "Bit 18 - Endpoint 2's USB Event Status"]
    #[inline(always)]
    pub fn epevt2(&mut self) -> EPEVT2_W {
        EPEVT2_W { w: self }
    }
    #[doc = "Bit 19 - Endpoint 3's USB Event Status"]
    #[inline(always)]
    pub fn epevt3(&mut self) -> EPEVT3_W {
        EPEVT3_W { w: self }
    }
    #[doc = "Bit 20 - Endpoint 4's USB Event Status"]
    #[inline(always)]
    pub fn epevt4(&mut self) -> EPEVT4_W {
        EPEVT4_W { w: self }
    }
    #[doc = "Bit 21 - Endpoint 5's USB Event Status"]
    #[inline(always)]
    pub fn epevt5(&mut self) -> EPEVT5_W {
        EPEVT5_W { w: self }
    }
    #[doc = "Bit 22 - Endpoint 6's USB Event Status"]
    #[inline(always)]
    pub fn epevt6(&mut self) -> EPEVT6_W {
        EPEVT6_W { w: self }
    }
    #[doc = "Bit 23 - Endpoint 7's USB Event Status"]
    #[inline(always)]
    pub fn epevt7(&mut self) -> EPEVT7_W {
        EPEVT7_W { w: self }
    }
    #[doc = "Bit 31 - Setup Event Status"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Event Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsts::R](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts::W](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
