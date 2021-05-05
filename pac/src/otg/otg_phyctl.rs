#[doc = "Register `OTG_PHYCTL` reader"]
pub struct R(crate::R<OTG_PHYCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_PHYCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OTG_PHYCTL_SPEC>> for R {
    fn from(reader: crate::R<OTG_PHYCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_PHYCTL` writer"]
pub struct W(crate::W<OTG_PHYCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_PHYCTL_SPEC>;
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
impl core::convert::From<crate::W<OTG_PHYCTL_SPEC>> for W {
    fn from(writer: crate::W<OTG_PHYCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OTG PHY Enable Bit\\nWhen USB frame is configured as OTG-device, user needs to set this bit before using OTG function. If device is not configured as OTG-device, this bit is 'don't care'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGPHYEN_A {
    #[doc = "0: OTG PHY Disabled"]
    _0 = 0,
    #[doc = "1: OTG PHY Enabled"]
    _1 = 1,
}
impl From<OTGPHYEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGPHYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGPHYEN` reader - OTG PHY Enable Bit\\nWhen USB frame is configured as OTG-device, user needs to set this bit before using OTG function. If device is not configured as OTG-device, this bit is 'don't care'."]
pub struct OTGPHYEN_R(crate::FieldReader<bool, OTGPHYEN_A>);
impl OTGPHYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGPHYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGPHYEN_A {
        match self.bits {
            false => OTGPHYEN_A::_0,
            true => OTGPHYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OTGPHYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OTGPHYEN_A::_1
    }
}
impl core::ops::Deref for OTGPHYEN_R {
    type Target = crate::FieldReader<bool, OTGPHYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGPHYEN` writer - OTG PHY Enable Bit\\nWhen USB frame is configured as OTG-device, user needs to set this bit before using OTG function. If device is not configured as OTG-device, this bit is 'don't care'."]
pub struct OTGPHYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGPHYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGPHYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG PHY Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGPHYEN_A::_0)
    }
    #[doc = "OTG PHY Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGPHYEN_A::_1)
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
#[doc = "ID Detection Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDDETEN_A {
    #[doc = "0: Detect ID pin status Disabled"]
    _0 = 0,
    #[doc = "1: Detect ID pin status Enabled"]
    _1 = 1,
}
impl From<IDDETEN_A> for bool {
    #[inline(always)]
    fn from(variant: IDDETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDDETEN` reader - ID Detection Enable Bit"]
pub struct IDDETEN_R(crate::FieldReader<bool, IDDETEN_A>);
impl IDDETEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDDETEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDDETEN_A {
        match self.bits {
            false => IDDETEN_A::_0,
            true => IDDETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDDETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDDETEN_A::_1
    }
}
impl core::ops::Deref for IDDETEN_R {
    type Target = crate::FieldReader<bool, IDDETEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDDETEN` writer - ID Detection Enable Bit"]
pub struct IDDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDDETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDDETEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Detect ID pin status Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDDETEN_A::_0)
    }
    #[doc = "Detect ID pin status Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDDETEN_A::_1)
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
#[doc = "Off-chip USB VBUS Power Switch Enable Polarity\\nThe OTG controller will enable off-chip USB VBUS power switch to provide VBUS power when need. A USB_VBUS_EN pin is used to control the off-chip USB VBUS power switch.\\nThe polarity of enabling off-chip USB VBUS power switch (high active or low active) depends on the selected component. Set this bit as following according to the polarity of off-chip USB VBUS power switch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBENPOL_A {
    #[doc = "0: The off-chip USB VBUS power switch enable is active high"]
    _0 = 0,
    #[doc = "1: The off-chip USB VBUS power switch enable is active low"]
    _1 = 1,
}
impl From<VBENPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VBENPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBENPOL` reader - Off-chip USB VBUS Power Switch Enable Polarity\\nThe OTG controller will enable off-chip USB VBUS power switch to provide VBUS power when need. A USB_VBUS_EN pin is used to control the off-chip USB VBUS power switch.\\nThe polarity of enabling off-chip USB VBUS power switch (high active or low active) depends on the selected component. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
pub struct VBENPOL_R(crate::FieldReader<bool, VBENPOL_A>);
impl VBENPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBENPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBENPOL_A {
        match self.bits {
            false => VBENPOL_A::_0,
            true => VBENPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBENPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBENPOL_A::_1
    }
}
impl core::ops::Deref for VBENPOL_R {
    type Target = crate::FieldReader<bool, VBENPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBENPOL` writer - Off-chip USB VBUS Power Switch Enable Polarity\\nThe OTG controller will enable off-chip USB VBUS power switch to provide VBUS power when need. A USB_VBUS_EN pin is used to control the off-chip USB VBUS power switch.\\nThe polarity of enabling off-chip USB VBUS power switch (high active or low active) depends on the selected component. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
pub struct VBENPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBENPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBENPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The off-chip USB VBUS power switch enable is active high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBENPOL_A::_0)
    }
    #[doc = "The off-chip USB VBUS power switch enable is active low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBENPOL_A::_1)
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
#[doc = "Off-chip USB VBUS Power Switch Status Polarity\\nThe polarity of off-chip USB VBUS power switch valid signal depends on the selected component. A USB_VBUS_ST pin is used to monitor the valid signal of the off-chip USB VBUS power switch. Set this bit as following according to the polarity of off-chip USB VBUS power switch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBSTSPOL_A {
    #[doc = "0: The polarity of off-chip USB VBUS power switch valid status is high"]
    _0 = 0,
    #[doc = "1: The polarity of off-chip USB VBUS power switch valid status is low"]
    _1 = 1,
}
impl From<VBSTSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VBSTSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBSTSPOL` reader - Off-chip USB VBUS Power Switch Status Polarity\\nThe polarity of off-chip USB VBUS power switch valid signal depends on the selected component. A USB_VBUS_ST pin is used to monitor the valid signal of the off-chip USB VBUS power switch. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
pub struct VBSTSPOL_R(crate::FieldReader<bool, VBSTSPOL_A>);
impl VBSTSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBSTSPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBSTSPOL_A {
        match self.bits {
            false => VBSTSPOL_A::_0,
            true => VBSTSPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBSTSPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBSTSPOL_A::_1
    }
}
impl core::ops::Deref for VBSTSPOL_R {
    type Target = crate::FieldReader<bool, VBSTSPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBSTSPOL` writer - Off-chip USB VBUS Power Switch Status Polarity\\nThe polarity of off-chip USB VBUS power switch valid signal depends on the selected component. A USB_VBUS_ST pin is used to monitor the valid signal of the off-chip USB VBUS power switch. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
pub struct VBSTSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBSTSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBSTSPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of off-chip USB VBUS power switch valid status is high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBSTSPOL_A::_0)
    }
    #[doc = "The polarity of off-chip USB VBUS power switch valid status is low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBSTSPOL_A::_1)
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
impl R {
    #[doc = "Bit 0 - OTG PHY Enable Bit\\nWhen USB frame is configured as OTG-device, user needs to set this bit before using OTG function. If device is not configured as OTG-device, this bit is 'don't care'."]
    #[inline(always)]
    pub fn otgphyen(&self) -> OTGPHYEN_R {
        OTGPHYEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ID Detection Enable Bit"]
    #[inline(always)]
    pub fn iddeten(&self) -> IDDETEN_R {
        IDDETEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Off-chip USB VBUS Power Switch Enable Polarity\\nThe OTG controller will enable off-chip USB VBUS power switch to provide VBUS power when need. A USB_VBUS_EN pin is used to control the off-chip USB VBUS power switch.\\nThe polarity of enabling off-chip USB VBUS power switch (high active or low active) depends on the selected component. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
    #[inline(always)]
    pub fn vbenpol(&self) -> VBENPOL_R {
        VBENPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Off-chip USB VBUS Power Switch Status Polarity\\nThe polarity of off-chip USB VBUS power switch valid signal depends on the selected component. A USB_VBUS_ST pin is used to monitor the valid signal of the off-chip USB VBUS power switch. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
    #[inline(always)]
    pub fn vbstspol(&self) -> VBSTSPOL_R {
        VBSTSPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTG PHY Enable Bit\\nWhen USB frame is configured as OTG-device, user needs to set this bit before using OTG function. If device is not configured as OTG-device, this bit is 'don't care'."]
    #[inline(always)]
    pub fn otgphyen(&mut self) -> OTGPHYEN_W {
        OTGPHYEN_W { w: self }
    }
    #[doc = "Bit 1 - ID Detection Enable Bit"]
    #[inline(always)]
    pub fn iddeten(&mut self) -> IDDETEN_W {
        IDDETEN_W { w: self }
    }
    #[doc = "Bit 4 - Off-chip USB VBUS Power Switch Enable Polarity\\nThe OTG controller will enable off-chip USB VBUS power switch to provide VBUS power when need. A USB_VBUS_EN pin is used to control the off-chip USB VBUS power switch.\\nThe polarity of enabling off-chip USB VBUS power switch (high active or low active) depends on the selected component. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
    #[inline(always)]
    pub fn vbenpol(&mut self) -> VBENPOL_W {
        VBENPOL_W { w: self }
    }
    #[doc = "Bit 5 - Off-chip USB VBUS Power Switch Status Polarity\\nThe polarity of off-chip USB VBUS power switch valid signal depends on the selected component. A USB_VBUS_ST pin is used to monitor the valid signal of the off-chip USB VBUS power switch. Set this bit as following according to the polarity of off-chip USB VBUS power switch."]
    #[inline(always)]
    pub fn vbstspol(&mut self) -> VBSTSPOL_W {
        VBSTSPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG PHY Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_phyctl](index.html) module"]
pub struct OTG_PHYCTL_SPEC;
impl crate::RegisterSpec for OTG_PHYCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_phyctl::R](R) reader structure"]
impl crate::Readable for OTG_PHYCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_phyctl::W](W) writer structure"]
impl crate::Writable for OTG_PHYCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_PHYCTL to value 0"]
impl crate::Resettable for OTG_PHYCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
