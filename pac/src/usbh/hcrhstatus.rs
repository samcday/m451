#[doc = "Register `HCRHSTATUS` reader"]
pub struct R(crate::R<HCRHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCRHSTATUS_SPEC>> for R {
    fn from(reader: crate::R<HCRHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHSTATUS` writer"]
pub struct W(crate::W<HCRHSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHSTATUS_SPEC>;
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
impl core::convert::From<crate::W<HCRHSTATUS_SPEC>> for W {
    fn from(writer: crate::W<HCRHSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Global Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPS_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear global power"]
    _1 = 1,
}
impl From<LPS_A> for bool {
    #[inline(always)]
    fn from(variant: LPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPS` reader - Clear Global Power"]
pub struct LPS_R(crate::FieldReader<bool, LPS_A>);
impl LPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPS_A {
        match self.bits {
            false => LPS_A::_0,
            true => LPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPS_A::_1
    }
}
impl core::ops::Deref for LPS_R {
    type Target = crate::FieldReader<bool, LPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPS` writer - Clear Global Power"]
pub struct LPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPS_A::_0)
    }
    #[doc = "Clear global power"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPS_A::_1)
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
#[doc = "over Current Indicator\\nThis bit reflects the state of the over current status pin. This field is only valid if NOCP (HcRhDesA\\[12\\]) and OCPM (HcRhDesA\\[11\\]) are cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCI_A {
    #[doc = "0: No over current condition"]
    _0 = 0,
    #[doc = "1: Over current condition"]
    _1 = 1,
}
impl From<OCI_A> for bool {
    #[inline(always)]
    fn from(variant: OCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCI` reader - over Current Indicator\\nThis bit reflects the state of the over current status pin. This field is only valid if NOCP (HcRhDesA\\[12\\]) and OCPM (HcRhDesA\\[11\\]) are cleared."]
pub struct OCI_R(crate::FieldReader<bool, OCI_A>);
impl OCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCI_A {
        match self.bits {
            false => OCI_A::_0,
            true => OCI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCI_A::_1
    }
}
impl core::ops::Deref for OCI_R {
    type Target = crate::FieldReader<bool, OCI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCI` writer - over Current Indicator\\nThis bit reflects the state of the over current status pin. This field is only valid if NOCP (HcRhDesA\\[12\\]) and OCPM (HcRhDesA\\[11\\]) are cleared."]
pub struct OCI_W<'a> {
    w: &'a mut W,
}
impl<'a> OCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No over current condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCI_A::_0)
    }
    #[doc = "Over current condition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCI_A::_1)
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
#[doc = "Device Remote Wakeup Enable Bit\\nThis bit controls if port's Connect Status Change as a remote wake-up event.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRWE_A {
    #[doc = "0: No effect.\\nConnect Status Change as a remote wake-up event Disabled"]
    _0 = 0,
    #[doc = "1: Connect Status Change as a remote wake-up event Enabled"]
    _1 = 1,
}
impl From<DRWE_A> for bool {
    #[inline(always)]
    fn from(variant: DRWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRWE` reader - Device Remote Wakeup Enable Bit\\nThis bit controls if port's Connect Status Change as a remote wake-up event.\\nWrite Operation:"]
pub struct DRWE_R(crate::FieldReader<bool, DRWE_A>);
impl DRWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRWE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRWE_A {
        match self.bits {
            false => DRWE_A::_0,
            true => DRWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DRWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DRWE_A::_1
    }
}
impl core::ops::Deref for DRWE_R {
    type Target = crate::FieldReader<bool, DRWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRWE` writer - Device Remote Wakeup Enable Bit\\nThis bit controls if port's Connect Status Change as a remote wake-up event.\\nWrite Operation:"]
pub struct DRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRWE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nConnect Status Change as a remote wake-up event Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRWE_A::_0)
    }
    #[doc = "Connect Status Change as a remote wake-up event Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRWE_A::_1)
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
#[doc = "Set Global Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSC_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Set global power"]
    _1 = 1,
}
impl From<LPSC_A> for bool {
    #[inline(always)]
    fn from(variant: LPSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSC` reader - Set Global Power"]
pub struct LPSC_R(crate::FieldReader<bool, LPSC_A>);
impl LPSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSC_A {
        match self.bits {
            false => LPSC_A::_0,
            true => LPSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPSC_A::_1
    }
}
impl core::ops::Deref for LPSC_R {
    type Target = crate::FieldReader<bool, LPSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSC` writer - Set Global Power"]
pub struct LPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPSC_A::_0)
    }
    #[doc = "Set global power"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPSC_A::_1)
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
#[doc = "over Current Indicator Change\\nThis bit is set by hardware when a change has occurred in OCI (HcRhStatus\\[1\\]).\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCIC_A {
    #[doc = "0: OCI (HcRhStatus\\[1\\]) didn't change"]
    _0 = 0,
    #[doc = "1: OCI (HcRhStatus\\[1\\]) change"]
    _1 = 1,
}
impl From<OCIC_A> for bool {
    #[inline(always)]
    fn from(variant: OCIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCIC` reader - over Current Indicator Change\\nThis bit is set by hardware when a change has occurred in OCI (HcRhStatus\\[1\\]).\\nWrite 1 to clear this bit to zero."]
pub struct OCIC_R(crate::FieldReader<bool, OCIC_A>);
impl OCIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCIC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCIC_A {
        match self.bits {
            false => OCIC_A::_0,
            true => OCIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCIC_A::_1
    }
}
impl core::ops::Deref for OCIC_R {
    type Target = crate::FieldReader<bool, OCIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCIC` writer - over Current Indicator Change\\nThis bit is set by hardware when a change has occurred in OCI (HcRhStatus\\[1\\]).\\nWrite 1 to clear this bit to zero."]
pub struct OCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCIC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OCI (HcRhStatus\\[1\\]) didn't change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCIC_A::_0)
    }
    #[doc = "OCI (HcRhStatus\\[1\\]) change"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCIC_A::_1)
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
#[doc = "Clear Remote Wake-up Enable Bit\\nThis bit is use to clear DRWE (HcRhStatus\\[15\\]).\\nThis bit always read as zero.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRWE_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear DRWE (HcRhStatus\\[15\\])"]
    _1 = 1,
}
impl From<CRWE_A> for bool {
    #[inline(always)]
    fn from(variant: CRWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRWE` reader - Clear Remote Wake-up Enable Bit\\nThis bit is use to clear DRWE (HcRhStatus\\[15\\]).\\nThis bit always read as zero.\\nWrite Operation:"]
pub struct CRWE_R(crate::FieldReader<bool, CRWE_A>);
impl CRWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRWE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRWE_A {
        match self.bits {
            false => CRWE_A::_0,
            true => CRWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRWE_A::_1
    }
}
impl core::ops::Deref for CRWE_R {
    type Target = crate::FieldReader<bool, CRWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRWE` writer - Clear Remote Wake-up Enable Bit\\nThis bit is use to clear DRWE (HcRhStatus\\[15\\]).\\nThis bit always read as zero.\\nWrite Operation:"]
pub struct CRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRWE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRWE_A::_0)
    }
    #[doc = "Clear DRWE (HcRhStatus\\[15\\])"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRWE_A::_1)
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
    #[doc = "Bit 0 - Clear Global Power"]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - over Current Indicator\\nThis bit reflects the state of the over current status pin. This field is only valid if NOCP (HcRhDesA\\[12\\]) and OCPM (HcRhDesA\\[11\\]) are cleared."]
    #[inline(always)]
    pub fn oci(&self) -> OCI_R {
        OCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Device Remote Wakeup Enable Bit\\nThis bit controls if port's Connect Status Change as a remote wake-up event.\\nWrite Operation:"]
    #[inline(always)]
    pub fn drwe(&self) -> DRWE_R {
        DRWE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set Global Power"]
    #[inline(always)]
    pub fn lpsc(&self) -> LPSC_R {
        LPSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - over Current Indicator Change\\nThis bit is set by hardware when a change has occurred in OCI (HcRhStatus\\[1\\]).\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clear Remote Wake-up Enable Bit\\nThis bit is use to clear DRWE (HcRhStatus\\[15\\]).\\nThis bit always read as zero.\\nWrite Operation:"]
    #[inline(always)]
    pub fn crwe(&self) -> CRWE_R {
        CRWE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Global Power"]
    #[inline(always)]
    pub fn lps(&mut self) -> LPS_W {
        LPS_W { w: self }
    }
    #[doc = "Bit 1 - over Current Indicator\\nThis bit reflects the state of the over current status pin. This field is only valid if NOCP (HcRhDesA\\[12\\]) and OCPM (HcRhDesA\\[11\\]) are cleared."]
    #[inline(always)]
    pub fn oci(&mut self) -> OCI_W {
        OCI_W { w: self }
    }
    #[doc = "Bit 15 - Device Remote Wakeup Enable Bit\\nThis bit controls if port's Connect Status Change as a remote wake-up event.\\nWrite Operation:"]
    #[inline(always)]
    pub fn drwe(&mut self) -> DRWE_W {
        DRWE_W { w: self }
    }
    #[doc = "Bit 16 - Set Global Power"]
    #[inline(always)]
    pub fn lpsc(&mut self) -> LPSC_W {
        LPSC_W { w: self }
    }
    #[doc = "Bit 17 - over Current Indicator Change\\nThis bit is set by hardware when a change has occurred in OCI (HcRhStatus\\[1\\]).\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 31 - Clear Remote Wake-up Enable Bit\\nThis bit is use to clear DRWE (HcRhStatus\\[15\\]).\\nThis bit always read as zero.\\nWrite Operation:"]
    #[inline(always)]
    pub fn crwe(&mut self) -> CRWE_W {
        CRWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Root Hub Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhstatus](index.html) module"]
pub struct HCRHSTATUS_SPEC;
impl crate::RegisterSpec for HCRHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhstatus::R](R) reader structure"]
impl crate::Readable for HCRHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhstatus::W](W) writer structure"]
impl crate::Writable for HCRHSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHSTATUS to value 0"]
impl crate::Resettable for HCRHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
