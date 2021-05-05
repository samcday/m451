#[doc = "Register `HCRHDESCRIPTORA` reader"]
pub struct R(crate::R<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCRHDESCRIPTORA_SPEC>> for R {
    fn from(reader: crate::R<HCRHDESCRIPTORA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORA` writer"]
pub struct W(crate::W<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORA_SPEC>;
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
impl core::convert::From<crate::W<HCRHDESCRIPTORA_SPEC>> for W {
    fn from(writer: crate::W<HCRHDESCRIPTORA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDP` reader - Number Downstream Ports\\nUSB host control supports two downstream ports and only one port is available in this series of chip."]
pub struct NDP_R(crate::FieldReader<u8, u8>);
impl NDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDP` writer - Number Downstream Ports\\nUSB host control supports two downstream ports and only one port is available in this series of chip."]
pub struct NDP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Power Switching Mode\\nThis bit is used to specify how the power switching of the Root Hub ports is controlled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSM_A {
    #[doc = "0: Global Switching"]
    _0 = 0,
    #[doc = "1: Individual Switching"]
    _1 = 1,
}
impl From<PSM_A> for bool {
    #[inline(always)]
    fn from(variant: PSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSM` reader - Power Switching Mode\\nThis bit is used to specify how the power switching of the Root Hub ports is controlled."]
pub struct PSM_R(crate::FieldReader<bool, PSM_A>);
impl PSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSM_A {
        match self.bits {
            false => PSM_A::_0,
            true => PSM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PSM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PSM_A::_1
    }
}
impl core::ops::Deref for PSM_R {
    type Target = crate::FieldReader<bool, PSM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSM` writer - Power Switching Mode\\nThis bit is used to specify how the power switching of the Root Hub ports is controlled."]
pub struct PSM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Global Switching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSM_A::_0)
    }
    #[doc = "Individual Switching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSM_A::_1)
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
#[doc = "over Current Protection Mode\\nThis bit describes how the over current status for the Root Hub ports reported. This bit is only valid when NOCP (HcRhDescriptorA\\[12\\]) is cleared.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCPM_A {
    #[doc = "0: Global Over current"]
    _0 = 0,
    #[doc = "1: Individual Over current"]
    _1 = 1,
}
impl From<OCPM_A> for bool {
    #[inline(always)]
    fn from(variant: OCPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCPM` reader - over Current Protection Mode\\nThis bit describes how the over current status for the Root Hub ports reported. This bit is only valid when NOCP (HcRhDescriptorA\\[12\\]) is cleared."]
pub struct OCPM_R(crate::FieldReader<bool, OCPM_A>);
impl OCPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCPM_A {
        match self.bits {
            false => OCPM_A::_0,
            true => OCPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCPM_A::_1
    }
}
impl core::ops::Deref for OCPM_R {
    type Target = crate::FieldReader<bool, OCPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCPM` writer - over Current Protection Mode\\nThis bit describes how the over current status for the Root Hub ports reported. This bit is only valid when NOCP (HcRhDescriptorA\\[12\\]) is cleared."]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Global Over current"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCPM_A::_0)
    }
    #[doc = "Individual Over current"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCPM_A::_1)
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
#[doc = "No over Current Protection\\nThis bit describes how the over current status for the Root Hub ports reported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCP_A {
    #[doc = "0: Over current status is reported"]
    _0 = 0,
    #[doc = "1: Over current status is not reported"]
    _1 = 1,
}
impl From<NOCP_A> for bool {
    #[inline(always)]
    fn from(variant: NOCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOCP` reader - No over Current Protection\\nThis bit describes how the over current status for the Root Hub ports reported."]
pub struct NOCP_R(crate::FieldReader<bool, NOCP_A>);
impl NOCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOCP_A {
        match self.bits {
            false => NOCP_A::_0,
            true => NOCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NOCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NOCP_A::_1
    }
}
impl core::ops::Deref for NOCP_R {
    type Target = crate::FieldReader<bool, NOCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCP` writer - No over Current Protection\\nThis bit describes how the over current status for the Root Hub ports reported."]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Over current status is reported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOCP_A::_0)
    }
    #[doc = "Over current status is not reported"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOCP_A::_1)
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
impl R {
    #[doc = "Bits 0:7 - Number Downstream Ports\\nUSB host control supports two downstream ports and only one port is available in this series of chip."]
    #[inline(always)]
    pub fn ndp(&self) -> NDP_R {
        NDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Power Switching Mode\\nThis bit is used to specify how the power switching of the Root Hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&self) -> PSM_R {
        PSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - over Current Protection Mode\\nThis bit describes how the over current status for the Root Hub ports reported. This bit is only valid when NOCP (HcRhDescriptorA\\[12\\]) is cleared."]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - No over Current Protection\\nThis bit describes how the over current status for the Root Hub ports reported."]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number Downstream Ports\\nUSB host control supports two downstream ports and only one port is available in this series of chip."]
    #[inline(always)]
    pub fn ndp(&mut self) -> NDP_W {
        NDP_W { w: self }
    }
    #[doc = "Bit 8 - Power Switching Mode\\nThis bit is used to specify how the power switching of the Root Hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&mut self) -> PSM_W {
        PSM_W { w: self }
    }
    #[doc = "Bit 11 - over Current Protection Mode\\nThis bit describes how the over current status for the Root Hub ports reported. This bit is only valid when NOCP (HcRhDescriptorA\\[12\\]) is cleared."]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bit 12 - No over Current Protection\\nThis bit describes how the over current status for the Root Hub ports reported."]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Root Hub Descriptor A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptora](index.html) module"]
pub struct HCRHDESCRIPTORA_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptora::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptora::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORA to value 0x0902"]
impl crate::Resettable for HCRHDESCRIPTORA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0902
    }
}
