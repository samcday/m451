#[doc = "Register `OTG_CTL` reader"]
pub struct R(crate::R<OTG_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OTG_CTL_SPEC>> for R {
    fn from(reader: crate::R<OTG_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_CTL` writer"]
pub struct W(crate::W<OTG_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_CTL_SPEC>;
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
impl core::convert::From<crate::W<OTG_CTL_SPEC>> for W {
    fn from(writer: crate::W<OTG_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Drop VBUS Control\\nIf user application running on this OTG A-device wants to conserve power, set this bit to drop VBUS. BUSREQ (OTG_CTL\\[1\\]) will be also cleared no matter A-device or B-device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSDROP_A {
    #[doc = "0: Not drop the VBUS"]
    _0 = 0,
    #[doc = "1: Drop the VBUS"]
    _1 = 1,
}
impl From<VBUSDROP_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSDROP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSDROP` reader - Drop VBUS Control\\nIf user application running on this OTG A-device wants to conserve power, set this bit to drop VBUS. BUSREQ (OTG_CTL\\[1\\]) will be also cleared no matter A-device or B-device."]
pub struct VBUSDROP_R(crate::FieldReader<bool, VBUSDROP_A>);
impl VBUSDROP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSDROP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSDROP_A {
        match self.bits {
            false => VBUSDROP_A::_0,
            true => VBUSDROP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBUSDROP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBUSDROP_A::_1
    }
}
impl core::ops::Deref for VBUSDROP_R {
    type Target = crate::FieldReader<bool, VBUSDROP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSDROP` writer - Drop VBUS Control\\nIf user application running on this OTG A-device wants to conserve power, set this bit to drop VBUS. BUSREQ (OTG_CTL\\[1\\]) will be also cleared no matter A-device or B-device."]
pub struct VBUSDROP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDROP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSDROP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not drop the VBUS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBUSDROP_A::_0)
    }
    #[doc = "Drop the VBUS"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBUSDROP_A::_1)
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
#[doc = "OTG Bus Request\\nIf OTG A-device wants to do data transfers via USB bus, setting this bit will drive VBUS high to detect USB device connection. If user won't use the bus any more, clearing this bit will drop VBUS to save power. This bit will be cleared when A-device goes to A_wait_vfall state. This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set or IDSTS (OTG_STATUS\\[1\\]) changed.\\nIf user of an OTG-B Device wants to request VBUS, setting this bit will run SRP protocol. This bit will be cleared if SRP failure (OTG A-device does not provide VBUS after B-device issues ARP in specified interval, defined in OTG specification). This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set IDSTS (OTG_STATUS\\[1\\]) changed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSREQ_A {
    #[doc = "0: Not launch VBUS in OTG A-device or not request SRP in OTG B-device"]
    _0 = 0,
    #[doc = "1: Launch VBUS in OTG A-device or request SRP in OTG B-device"]
    _1 = 1,
}
impl From<BUSREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BUSREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSREQ` reader - OTG Bus Request\\nIf OTG A-device wants to do data transfers via USB bus, setting this bit will drive VBUS high to detect USB device connection. If user won't use the bus any more, clearing this bit will drop VBUS to save power. This bit will be cleared when A-device goes to A_wait_vfall state. This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set or IDSTS (OTG_STATUS\\[1\\]) changed.\\nIf user of an OTG-B Device wants to request VBUS, setting this bit will run SRP protocol. This bit will be cleared if SRP failure (OTG A-device does not provide VBUS after B-device issues ARP in specified interval, defined in OTG specification). This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set IDSTS (OTG_STATUS\\[1\\]) changed."]
pub struct BUSREQ_R(crate::FieldReader<bool, BUSREQ_A>);
impl BUSREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSREQ_A {
        match self.bits {
            false => BUSREQ_A::_0,
            true => BUSREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSREQ_A::_1
    }
}
impl core::ops::Deref for BUSREQ_R {
    type Target = crate::FieldReader<bool, BUSREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSREQ` writer - OTG Bus Request\\nIf OTG A-device wants to do data transfers via USB bus, setting this bit will drive VBUS high to detect USB device connection. If user won't use the bus any more, clearing this bit will drop VBUS to save power. This bit will be cleared when A-device goes to A_wait_vfall state. This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set or IDSTS (OTG_STATUS\\[1\\]) changed.\\nIf user of an OTG-B Device wants to request VBUS, setting this bit will run SRP protocol. This bit will be cleared if SRP failure (OTG A-device does not provide VBUS after B-device issues ARP in specified interval, defined in OTG specification). This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set IDSTS (OTG_STATUS\\[1\\]) changed."]
pub struct BUSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSREQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not launch VBUS in OTG A-device or not request SRP in OTG B-device"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSREQ_A::_0)
    }
    #[doc = "Launch VBUS in OTG A-device or request SRP in OTG B-device"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSREQ_A::_1)
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
#[doc = "OTG HNP Request Enable Bit\\nWhen USB frame as A-device, set this bit when A-device allows to process HNP protocol A-device changes role from Host to Peripheral. This bit will be cleared when OTG state changes from a_suspend to a_peripheral or goes back to a_idle state. When USB frame as B-device, set this bit after the OTG A-device successfully sends a SetFeature (b_hnp_enable) command to the OTG B-device to start role change B-device changes role from Peripheral to Host. This bit will be cleared when OTG state changes from b_peripheral to b_wait_acon or goes back to b_idle state.\\nNote: Refer to OTG specification to get a_suspend, a_peripheral, a_idle and b_idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNPREQEN_A {
    #[doc = "0: HNP request Disabled"]
    _0 = 0,
    #[doc = "1: HNP request Enabled (A-device can change role from Host to Peripheral or B-device can change role from Peripheral to Host)"]
    _1 = 1,
}
impl From<HNPREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: HNPREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPREQEN` reader - OTG HNP Request Enable Bit\\nWhen USB frame as A-device, set this bit when A-device allows to process HNP protocol A-device changes role from Host to Peripheral. This bit will be cleared when OTG state changes from a_suspend to a_peripheral or goes back to a_idle state. When USB frame as B-device, set this bit after the OTG A-device successfully sends a SetFeature (b_hnp_enable) command to the OTG B-device to start role change B-device changes role from Peripheral to Host. This bit will be cleared when OTG state changes from b_peripheral to b_wait_acon or goes back to b_idle state.\\nNote: Refer to OTG specification to get a_suspend, a_peripheral, a_idle and b_idle state."]
pub struct HNPREQEN_R(crate::FieldReader<bool, HNPREQEN_A>);
impl HNPREQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPREQEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPREQEN_A {
        match self.bits {
            false => HNPREQEN_A::_0,
            true => HNPREQEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HNPREQEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HNPREQEN_A::_1
    }
}
impl core::ops::Deref for HNPREQEN_R {
    type Target = crate::FieldReader<bool, HNPREQEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPREQEN` writer - OTG HNP Request Enable Bit\\nWhen USB frame as A-device, set this bit when A-device allows to process HNP protocol A-device changes role from Host to Peripheral. This bit will be cleared when OTG state changes from a_suspend to a_peripheral or goes back to a_idle state. When USB frame as B-device, set this bit after the OTG A-device successfully sends a SetFeature (b_hnp_enable) command to the OTG B-device to start role change B-device changes role from Peripheral to Host. This bit will be cleared when OTG state changes from b_peripheral to b_wait_acon or goes back to b_idle state.\\nNote: Refer to OTG specification to get a_suspend, a_peripheral, a_idle and b_idle state."]
pub struct HNPREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNPREQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HNP request Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HNPREQEN_A::_0)
    }
    #[doc = "HNP request Enabled (A-device can change role from Host to Peripheral or B-device can change role from Peripheral to Host)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HNPREQEN_A::_1)
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
#[doc = "OTG Function Enable Bit\\nUser needs to set this bit to enable OTG function while USB frame configured as OTG device. When USB frame not configured as OTG device, this bit is must be low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGEN_A {
    #[doc = "0: OTG function Disabled"]
    _0 = 0,
    #[doc = "1: OTG function Enabled"]
    _1 = 1,
}
impl From<OTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGEN` reader - OTG Function Enable Bit\\nUser needs to set this bit to enable OTG function while USB frame configured as OTG device. When USB frame not configured as OTG device, this bit is must be low."]
pub struct OTGEN_R(crate::FieldReader<bool, OTGEN_A>);
impl OTGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGEN_A {
        match self.bits {
            false => OTGEN_A::_0,
            true => OTGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OTGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OTGEN_A::_1
    }
}
impl core::ops::Deref for OTGEN_R {
    type Target = crate::FieldReader<bool, OTGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGEN` writer - OTG Function Enable Bit\\nUser needs to set this bit to enable OTG function while USB frame configured as OTG device. When USB frame not configured as OTG device, this bit is must be low."]
pub struct OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGEN_A::_0)
    }
    #[doc = "OTG function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGEN_A::_1)
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
#[doc = "OTG ID Pin Wake-up Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: OTG ID pin status change wake-up function Disabled"]
    _0 = 0,
    #[doc = "1: OTG ID pin status change wake-up function Enabled"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - OTG ID Pin Wake-up Enable Bit"]
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
#[doc = "Field `WKEN` writer - OTG ID Pin Wake-up Enable Bit"]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OTG ID pin status change wake-up function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "OTG ID pin status change wake-up function Enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Drop VBUS Control\\nIf user application running on this OTG A-device wants to conserve power, set this bit to drop VBUS. BUSREQ (OTG_CTL\\[1\\]) will be also cleared no matter A-device or B-device."]
    #[inline(always)]
    pub fn vbusdrop(&self) -> VBUSDROP_R {
        VBUSDROP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OTG Bus Request\\nIf OTG A-device wants to do data transfers via USB bus, setting this bit will drive VBUS high to detect USB device connection. If user won't use the bus any more, clearing this bit will drop VBUS to save power. This bit will be cleared when A-device goes to A_wait_vfall state. This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set or IDSTS (OTG_STATUS\\[1\\]) changed.\\nIf user of an OTG-B Device wants to request VBUS, setting this bit will run SRP protocol. This bit will be cleared if SRP failure (OTG A-device does not provide VBUS after B-device issues ARP in specified interval, defined in OTG specification). This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set IDSTS (OTG_STATUS\\[1\\]) changed."]
    #[inline(always)]
    pub fn busreq(&self) -> BUSREQ_R {
        BUSREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG HNP Request Enable Bit\\nWhen USB frame as A-device, set this bit when A-device allows to process HNP protocol A-device changes role from Host to Peripheral. This bit will be cleared when OTG state changes from a_suspend to a_peripheral or goes back to a_idle state. When USB frame as B-device, set this bit after the OTG A-device successfully sends a SetFeature (b_hnp_enable) command to the OTG B-device to start role change B-device changes role from Peripheral to Host. This bit will be cleared when OTG state changes from b_peripheral to b_wait_acon or goes back to b_idle state.\\nNote: Refer to OTG specification to get a_suspend, a_peripheral, a_idle and b_idle state."]
    #[inline(always)]
    pub fn hnpreqen(&self) -> HNPREQEN_R {
        HNPREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OTG Function Enable Bit\\nUser needs to set this bit to enable OTG function while USB frame configured as OTG device. When USB frame not configured as OTG device, this bit is must be low."]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OTG ID Pin Wake-up Enable Bit"]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drop VBUS Control\\nIf user application running on this OTG A-device wants to conserve power, set this bit to drop VBUS. BUSREQ (OTG_CTL\\[1\\]) will be also cleared no matter A-device or B-device."]
    #[inline(always)]
    pub fn vbusdrop(&mut self) -> VBUSDROP_W {
        VBUSDROP_W { w: self }
    }
    #[doc = "Bit 1 - OTG Bus Request\\nIf OTG A-device wants to do data transfers via USB bus, setting this bit will drive VBUS high to detect USB device connection. If user won't use the bus any more, clearing this bit will drop VBUS to save power. This bit will be cleared when A-device goes to A_wait_vfall state. This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set or IDSTS (OTG_STATUS\\[1\\]) changed.\\nIf user of an OTG-B Device wants to request VBUS, setting this bit will run SRP protocol. This bit will be cleared if SRP failure (OTG A-device does not provide VBUS after B-device issues ARP in specified interval, defined in OTG specification). This bit will be also cleared if VBUSDROP (OTG_CTL\\[0\\]) bit is set IDSTS (OTG_STATUS\\[1\\]) changed."]
    #[inline(always)]
    pub fn busreq(&mut self) -> BUSREQ_W {
        BUSREQ_W { w: self }
    }
    #[doc = "Bit 2 - OTG HNP Request Enable Bit\\nWhen USB frame as A-device, set this bit when A-device allows to process HNP protocol A-device changes role from Host to Peripheral. This bit will be cleared when OTG state changes from a_suspend to a_peripheral or goes back to a_idle state. When USB frame as B-device, set this bit after the OTG A-device successfully sends a SetFeature (b_hnp_enable) command to the OTG B-device to start role change B-device changes role from Peripheral to Host. This bit will be cleared when OTG state changes from b_peripheral to b_wait_acon or goes back to b_idle state.\\nNote: Refer to OTG specification to get a_suspend, a_peripheral, a_idle and b_idle state."]
    #[inline(always)]
    pub fn hnpreqen(&mut self) -> HNPREQEN_W {
        HNPREQEN_W { w: self }
    }
    #[doc = "Bit 4 - OTG Function Enable Bit\\nUser needs to set this bit to enable OTG function while USB frame configured as OTG device. When USB frame not configured as OTG device, this bit is must be low."]
    #[inline(always)]
    pub fn otgen(&mut self) -> OTGEN_W {
        OTGEN_W { w: self }
    }
    #[doc = "Bit 5 - OTG ID Pin Wake-up Enable Bit"]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_ctl](index.html) module"]
pub struct OTG_CTL_SPEC;
impl crate::RegisterSpec for OTG_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_ctl::R](R) reader structure"]
impl crate::Readable for OTG_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_ctl::W](W) writer structure"]
impl crate::Writable for OTG_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_CTL to value 0"]
impl crate::Resettable for OTG_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
