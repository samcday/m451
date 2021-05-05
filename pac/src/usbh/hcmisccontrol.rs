#[doc = "Register `HCMISCCONTROL` reader"]
pub struct R(crate::R<HCMISCCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCMISCCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCMISCCONTROL_SPEC>> for R {
    fn from(reader: crate::R<HCMISCCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCMISCCONTROL` writer"]
pub struct W(crate::W<HCMISCCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCMISCCONTROL_SPEC>;
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
impl core::convert::From<crate::W<HCMISCCONTROL_SPEC>> for W {
    fn from(writer: crate::W<HCMISCCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AHB Bus ERROR Response\\nThis bit indicates there is an ERROR response received in AHB bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "0: No ERROR response received"]
    _0 = 0,
    #[doc = "1: ERROR response received"]
    _1 = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - AHB Bus ERROR Response\\nThis bit indicates there is an ERROR response received in AHB bus."]
pub struct ABORT_R(crate::FieldReader<bool, ABORT_A>);
impl ABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::_0,
            true => ABORT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABORT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABORT_A::_1
    }
}
impl core::ops::Deref for ABORT_R {
    type Target = crate::FieldReader<bool, ABORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT` writer - AHB Bus ERROR Response\\nThis bit indicates there is an ERROR response received in AHB bus."]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABORT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ERROR response received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABORT_A::_0)
    }
    #[doc = "ERROR response received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABORT_A::_1)
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
#[doc = "over Current Active Low\\nThis bit controls the polarity of over current flag from external power IC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCAL_A {
    #[doc = "0: Over current flag is high active"]
    _0 = 0,
    #[doc = "1: Over current flag is low active"]
    _1 = 1,
}
impl From<OCAL_A> for bool {
    #[inline(always)]
    fn from(variant: OCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCAL` reader - over Current Active Low\\nThis bit controls the polarity of over current flag from external power IC."]
pub struct OCAL_R(crate::FieldReader<bool, OCAL_A>);
impl OCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCAL_A {
        match self.bits {
            false => OCAL_A::_0,
            true => OCAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCAL_A::_1
    }
}
impl core::ops::Deref for OCAL_R {
    type Target = crate::FieldReader<bool, OCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCAL` writer - over Current Active Low\\nThis bit controls the polarity of over current flag from external power IC."]
pub struct OCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Over current flag is high active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCAL_A::_0)
    }
    #[doc = "Over current flag is low active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCAL_A::_1)
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
#[doc = "Disable Port 1\\nThis bit controls if the connection between USB host controller and transceiver of port 1 is disabled. If the connection is disabled, the USB host controller will not recognize any event of USB bus.\\nSet this bit high, the transceiver of port 1 will also be forced into the standby mode no matter what USB host controller operation is.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPRT1_A {
    #[doc = "0: The connection between USB host controller and transceiver of port 1 Enabled"]
    _0 = 0,
    #[doc = "1: The connection between USB host controller and transceiver of port 1 Disabled and the transceiver of port 1 will also be forced into the standby mode"]
    _1 = 1,
}
impl From<DPRT1_A> for bool {
    #[inline(always)]
    fn from(variant: DPRT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPRT1` reader - Disable Port 1\\nThis bit controls if the connection between USB host controller and transceiver of port 1 is disabled. If the connection is disabled, the USB host controller will not recognize any event of USB bus.\\nSet this bit high, the transceiver of port 1 will also be forced into the standby mode no matter what USB host controller operation is."]
pub struct DPRT1_R(crate::FieldReader<bool, DPRT1_A>);
impl DPRT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPRT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPRT1_A {
        match self.bits {
            false => DPRT1_A::_0,
            true => DPRT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPRT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPRT1_A::_1
    }
}
impl core::ops::Deref for DPRT1_R {
    type Target = crate::FieldReader<bool, DPRT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPRT1` writer - Disable Port 1\\nThis bit controls if the connection between USB host controller and transceiver of port 1 is disabled. If the connection is disabled, the USB host controller will not recognize any event of USB bus.\\nSet this bit high, the transceiver of port 1 will also be forced into the standby mode no matter what USB host controller operation is."]
pub struct DPRT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPRT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPRT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The connection between USB host controller and transceiver of port 1 Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPRT1_A::_0)
    }
    #[doc = "The connection between USB host controller and transceiver of port 1 Disabled and the transceiver of port 1 will also be forced into the standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPRT1_A::_1)
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
impl R {
    #[doc = "Bit 1 - AHB Bus ERROR Response\\nThis bit indicates there is an ERROR response received in AHB bus."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - over Current Active Low\\nThis bit controls the polarity of over current flag from external power IC."]
    #[inline(always)]
    pub fn ocal(&self) -> OCAL_R {
        OCAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable Port 1\\nThis bit controls if the connection between USB host controller and transceiver of port 1 is disabled. If the connection is disabled, the USB host controller will not recognize any event of USB bus.\\nSet this bit high, the transceiver of port 1 will also be forced into the standby mode no matter what USB host controller operation is."]
    #[inline(always)]
    pub fn dprt1(&self) -> DPRT1_R {
        DPRT1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - AHB Bus ERROR Response\\nThis bit indicates there is an ERROR response received in AHB bus."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 3 - over Current Active Low\\nThis bit controls the polarity of over current flag from external power IC."]
    #[inline(always)]
    pub fn ocal(&mut self) -> OCAL_W {
        OCAL_W { w: self }
    }
    #[doc = "Bit 16 - Disable Port 1\\nThis bit controls if the connection between USB host controller and transceiver of port 1 is disabled. If the connection is disabled, the USB host controller will not recognize any event of USB bus.\\nSet this bit high, the transceiver of port 1 will also be forced into the standby mode no matter what USB host controller operation is."]
    #[inline(always)]
    pub fn dprt1(&mut self) -> DPRT1_W {
        DPRT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcmisccontrol](index.html) module"]
pub struct HCMISCCONTROL_SPEC;
impl crate::RegisterSpec for HCMISCCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcmisccontrol::R](R) reader structure"]
impl crate::Readable for HCMISCCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcmisccontrol::W](W) writer structure"]
impl crate::Writable for HCMISCCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCMISCCONTROL to value 0"]
impl crate::Resettable for HCMISCCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
