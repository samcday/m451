#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTEN_SPEC>> for R {
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl core::convert::From<crate::W<INTEN_SPEC>> for W {
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bus Event Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSIEN_A {
    #[doc = "0: BUS event interrupt Disabled"]
    _0 = 0,
    #[doc = "1: BUS event interrupt Enabled"]
    _1 = 1,
}
impl From<BUSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSIEN` reader - Bus Event Interrupt Enable Bit"]
pub struct BUSIEN_R(crate::FieldReader<bool, BUSIEN_A>);
impl BUSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSIEN_A {
        match self.bits {
            false => BUSIEN_A::_0,
            true => BUSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSIEN_A::_1
    }
}
impl core::ops::Deref for BUSIEN_R {
    type Target = crate::FieldReader<bool, BUSIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSIEN` writer - Bus Event Interrupt Enable Bit"]
pub struct BUSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BUS event interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSIEN_A::_0)
    }
    #[doc = "BUS event interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSIEN_A::_1)
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
#[doc = "USB Event Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBIEN_A {
    #[doc = "0: USB event interrupt Disabled"]
    _0 = 0,
    #[doc = "1: USB event interrupt Enabled"]
    _1 = 1,
}
impl From<USBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBIEN` reader - USB Event Interrupt Enable Bit"]
pub struct USBIEN_R(crate::FieldReader<bool, USBIEN_A>);
impl USBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBIEN_A {
        match self.bits {
            false => USBIEN_A::_0,
            true => USBIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBIEN_A::_1
    }
}
impl core::ops::Deref for USBIEN_R {
    type Target = crate::FieldReader<bool, USBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBIEN` writer - USB Event Interrupt Enable Bit"]
pub struct USBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB event interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBIEN_A::_0)
    }
    #[doc = "USB event interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBIEN_A::_1)
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
#[doc = "VBUS Detection Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBDETIEN_A {
    #[doc = "0: VBUS detection Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: VBUS detection Interrupt Enabled"]
    _1 = 1,
}
impl From<VBDETIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBDETIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBDETIEN` reader - VBUS Detection Interrupt Enable Bit"]
pub struct VBDETIEN_R(crate::FieldReader<bool, VBDETIEN_A>);
impl VBDETIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBDETIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBDETIEN_A {
        match self.bits {
            false => VBDETIEN_A::_0,
            true => VBDETIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBDETIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBDETIEN_A::_1
    }
}
impl core::ops::Deref for VBDETIEN_R {
    type Target = crate::FieldReader<bool, VBDETIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBDETIEN` writer - VBUS Detection Interrupt Enable Bit"]
pub struct VBDETIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBDETIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBDETIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBUS detection Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBDETIEN_A::_0)
    }
    #[doc = "VBUS detection Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBDETIEN_A::_1)
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
#[doc = "USB No-event-wake-up Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEVWKIEN_A {
    #[doc = "0: No-event-wake-up Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: No-event-wake-up Interrupt Enabled"]
    _1 = 1,
}
impl From<NEVWKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NEVWKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEVWKIEN` reader - USB No-event-wake-up Interrupt Enable Bit"]
pub struct NEVWKIEN_R(crate::FieldReader<bool, NEVWKIEN_A>);
impl NEVWKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEVWKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEVWKIEN_A {
        match self.bits {
            false => NEVWKIEN_A::_0,
            true => NEVWKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEVWKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEVWKIEN_A::_1
    }
}
impl core::ops::Deref for NEVWKIEN_R {
    type Target = crate::FieldReader<bool, NEVWKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEVWKIEN` writer - USB No-event-wake-up Interrupt Enable Bit"]
pub struct NEVWKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NEVWKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEVWKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No-event-wake-up Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEVWKIEN_A::_0)
    }
    #[doc = "No-event-wake-up Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEVWKIEN_A::_1)
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
#[doc = "Wake-up Function Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: USB wake-up function Disabled"]
    _0 = 0,
    #[doc = "1: USB wake-up function Enabled"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - Wake-up Function Enable Bit"]
pub struct WKEN_R(crate::FieldReader<bool, WKEN_A>);
impl WKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKEN_A {
        match self.bits {
            false => WKEN_A::_0,
            true => WKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKEN_A::_1
    }
}
impl core::ops::Deref for WKEN_R {
    type Target = crate::FieldReader<bool, WKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKEN` writer - Wake-up Function Enable Bit"]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB wake-up function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "USB wake-up function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKEN_A::_1)
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
#[doc = "Active NAK Function and Its Status in IN Token\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INNAKEN_A {
    #[doc = "0: When device responds NAK after receiving IN token, IN NAK status will not be updated to USBD_EPSTS register, so that the USB interrupt event will not be asserted"]
    _0 = 0,
    #[doc = "1: IN NAK status will be updated to USBD_EPSTS register and the USB interrupt event will be asserted, when the device responds NAK after receiving IN token"]
    _1 = 1,
}
impl From<INNAKEN_A> for bool {
    #[inline(always)]
    fn from(variant: INNAKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INNAKEN` reader - Active NAK Function and Its Status in IN Token"]
pub struct INNAKEN_R(crate::FieldReader<bool, INNAKEN_A>);
impl INNAKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INNAKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INNAKEN_A {
        match self.bits {
            false => INNAKEN_A::_0,
            true => INNAKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INNAKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INNAKEN_A::_1
    }
}
impl core::ops::Deref for INNAKEN_R {
    type Target = crate::FieldReader<bool, INNAKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INNAKEN` writer - Active NAK Function and Its Status in IN Token"]
pub struct INNAKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INNAKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INNAKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When device responds NAK after receiving IN token, IN NAK status will not be updated to USBD_EPSTS register, so that the USB interrupt event will not be asserted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INNAKEN_A::_0)
    }
    #[doc = "IN NAK status will be updated to USBD_EPSTS register and the USB interrupt event will be asserted, when the device responds NAK after receiving IN token"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INNAKEN_A::_1)
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
    #[doc = "Bit 0 - Bus Event Interrupt Enable Bit"]
    #[inline(always)]
    pub fn busien(&self) -> BUSIEN_R {
        BUSIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Event Interrupt Enable Bit"]
    #[inline(always)]
    pub fn usbien(&self) -> USBIEN_R {
        USBIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn vbdetien(&self) -> VBDETIEN_R {
        VBDETIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB No-event-wake-up Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nevwkien(&self) -> NEVWKIEN_R {
        NEVWKIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-up Function Enable Bit"]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Active NAK Function and Its Status in IN Token"]
    #[inline(always)]
    pub fn innaken(&self) -> INNAKEN_R {
        INNAKEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Event Interrupt Enable Bit"]
    #[inline(always)]
    pub fn busien(&mut self) -> BUSIEN_W {
        BUSIEN_W { w: self }
    }
    #[doc = "Bit 1 - USB Event Interrupt Enable Bit"]
    #[inline(always)]
    pub fn usbien(&mut self) -> USBIEN_W {
        USBIEN_W { w: self }
    }
    #[doc = "Bit 2 - VBUS Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn vbdetien(&mut self) -> VBDETIEN_W {
        VBDETIEN_W { w: self }
    }
    #[doc = "Bit 3 - USB No-event-wake-up Interrupt Enable Bit"]
    #[inline(always)]
    pub fn nevwkien(&mut self) -> NEVWKIEN_W {
        NEVWKIEN_W { w: self }
    }
    #[doc = "Bit 8 - Wake-up Function Enable Bit"]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Bit 15 - Active NAK Function and Its Status in IN Token"]
    #[inline(always)]
    pub fn innaken(&mut self) -> INNAKEN_W {
        INNAKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
