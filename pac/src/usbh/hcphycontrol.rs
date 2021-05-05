#[doc = "Register `HCPHYCONTROL` reader"]
pub struct R(crate::R<HCPHYCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPHYCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCPHYCONTROL_SPEC>> for R {
    fn from(reader: crate::R<HCPHYCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCPHYCONTROL` writer"]
pub struct W(crate::W<HCPHYCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCPHYCONTROL_SPEC>;
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
impl core::convert::From<crate::W<HCPHYCONTROL_SPEC>> for W {
    fn from(writer: crate::W<HCPHYCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Transceiver Standby Enable Bit\\nThis bit controls if USB transceiver could enter the standby mode to reduce power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBYEN_A {
    #[doc = "0: The USB transceiver would never enter the standby mode"]
    _0 = 0,
    #[doc = "1: The USB transceiver will enter standby mode while port is in power off state (port power is inactive)"]
    _1 = 1,
}
impl From<STBYEN_A> for bool {
    #[inline(always)]
    fn from(variant: STBYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBYEN` reader - USB Transceiver Standby Enable Bit\\nThis bit controls if USB transceiver could enter the standby mode to reduce power consumption."]
pub struct STBYEN_R(crate::FieldReader<bool, STBYEN_A>);
impl STBYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBYEN_A {
        match self.bits {
            false => STBYEN_A::_0,
            true => STBYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STBYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STBYEN_A::_1
    }
}
impl core::ops::Deref for STBYEN_R {
    type Target = crate::FieldReader<bool, STBYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBYEN` writer - USB Transceiver Standby Enable Bit\\nThis bit controls if USB transceiver could enter the standby mode to reduce power consumption."]
pub struct STBYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STBYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The USB transceiver would never enter the standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STBYEN_A::_0)
    }
    #[doc = "The USB transceiver will enter standby mode while port is in power off state (port power is inactive)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STBYEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - USB Transceiver Standby Enable Bit\\nThis bit controls if USB transceiver could enter the standby mode to reduce power consumption."]
    #[inline(always)]
    pub fn stbyen(&self) -> STBYEN_R {
        STBYEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - USB Transceiver Standby Enable Bit\\nThis bit controls if USB transceiver could enter the standby mode to reduce power consumption."]
    #[inline(always)]
    pub fn stbyen(&mut self) -> STBYEN_W {
        STBYEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller PHY Control Regsiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcphycontrol](index.html) module"]
pub struct HCPHYCONTROL_SPEC;
impl crate::RegisterSpec for HCPHYCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcphycontrol::R](R) reader structure"]
impl crate::Readable for HCPHYCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcphycontrol::W](W) writer structure"]
impl crate::Writable for HCPHYCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCPHYCONTROL to value 0"]
impl crate::Resettable for HCPHYCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
