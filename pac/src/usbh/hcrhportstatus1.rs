#[doc = "Register `HCRHPORTSTATUS1` reader"]
pub struct R(crate::R<HCRHPORTSTATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHPORTSTATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCRHPORTSTATUS1_SPEC>> for R {
    fn from(reader: crate::R<HCRHPORTSTATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHPORTSTATUS1` writer"]
pub struct W(crate::W<HCRHPORTSTATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHPORTSTATUS1_SPEC>;
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
impl core::convert::From<crate::W<HCRHPORTSTATUS1_SPEC>> for W {
    fn from(writer: crate::W<HCRHPORTSTATUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CurrentConnectStatus (Read) or ClearPortEnable Bit (Write)\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCS_A {
    #[doc = "0: No effect.\\nNo device connected"]
    _0 = 0,
    #[doc = "1: Clear port enable.\\nDevice connected"]
    _1 = 1,
}
impl From<CCS_A> for bool {
    #[inline(always)]
    fn from(variant: CCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCS` reader - CurrentConnectStatus (Read) or ClearPortEnable Bit (Write)\\nWrite Operation:"]
pub struct CCS_R(crate::FieldReader<bool, CCS_A>);
impl CCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::_0,
            true => CCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CCS_A::_1
    }
}
impl core::ops::Deref for CCS_R {
    type Target = crate::FieldReader<bool, CCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCS` writer - CurrentConnectStatus (Read) or ClearPortEnable Bit (Write)\\nWrite Operation:"]
pub struct CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nNo device connected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCS_A::_0)
    }
    #[doc = "Clear port enable.\\nDevice connected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCS_A::_1)
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
#[doc = "Port Enable Status\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PES_A {
    #[doc = "0: No effect.\\nPort Disabled"]
    _0 = 0,
    #[doc = "1: Set port enable.\\nPort Enabled"]
    _1 = 1,
}
impl From<PES_A> for bool {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - Port Enable Status\\nWrite Operation:"]
pub struct PES_R(crate::FieldReader<bool, PES_A>);
impl PES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PES_A {
        match self.bits {
            false => PES_A::_0,
            true => PES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PES_A::_1
    }
}
impl core::ops::Deref for PES_R {
    type Target = crate::FieldReader<bool, PES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES` writer - Port Enable Status\\nWrite Operation:"]
pub struct PES_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nPort Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PES_A::_0)
    }
    #[doc = "Set port enable.\\nPort Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PES_A::_1)
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
#[doc = "Port Suspend Status\\nThis bit indicates the port is suspended\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSS_A {
    #[doc = "0: No effect.\\nPort is not suspended"]
    _0 = 0,
    #[doc = "1: Set port suspend.\\nPort is selectively suspended"]
    _1 = 1,
}
impl From<PSS_A> for bool {
    #[inline(always)]
    fn from(variant: PSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS` reader - Port Suspend Status\\nThis bit indicates the port is suspended\\nWrite Operation:"]
pub struct PSS_R(crate::FieldReader<bool, PSS_A>);
impl PSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSS_A {
        match self.bits {
            false => PSS_A::_0,
            true => PSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PSS_A::_1
    }
}
impl core::ops::Deref for PSS_R {
    type Target = crate::FieldReader<bool, PSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS` writer - Port Suspend Status\\nThis bit indicates the port is suspended\\nWrite Operation:"]
pub struct PSS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nPort is not suspended"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSS_A::_0)
    }
    #[doc = "Set port suspend.\\nPort is selectively suspended"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSS_A::_1)
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
#[doc = "Port over Current Indicator (Read) or Clear Port Suspend (Write)\\nThis bit reflects the state of the over current status pin dedicated to this port. This field is only valid if NOCP (HcRhDescriptorA\\[12\\]) is cleared and OCPM (HcRhDescriptorA\\[11\\]) is set.\\nThis bit is also used to initiate the selective result sequence for the port.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POCI_A {
    #[doc = "0: No effect.\\nNo over current condition"]
    _0 = 0,
    #[doc = "1: Clear port suspend.\\nOver current condition"]
    _1 = 1,
}
impl From<POCI_A> for bool {
    #[inline(always)]
    fn from(variant: POCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POCI` reader - Port over Current Indicator (Read) or Clear Port Suspend (Write)\\nThis bit reflects the state of the over current status pin dedicated to this port. This field is only valid if NOCP (HcRhDescriptorA\\[12\\]) is cleared and OCPM (HcRhDescriptorA\\[11\\]) is set.\\nThis bit is also used to initiate the selective result sequence for the port.\\nWrite Operation:"]
pub struct POCI_R(crate::FieldReader<bool, POCI_A>);
impl POCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        POCI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POCI_A {
        match self.bits {
            false => POCI_A::_0,
            true => POCI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POCI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POCI_A::_1
    }
}
impl core::ops::Deref for POCI_R {
    type Target = crate::FieldReader<bool, POCI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POCI` writer - Port over Current Indicator (Read) or Clear Port Suspend (Write)\\nThis bit reflects the state of the over current status pin dedicated to this port. This field is only valid if NOCP (HcRhDescriptorA\\[12\\]) is cleared and OCPM (HcRhDescriptorA\\[11\\]) is set.\\nThis bit is also used to initiate the selective result sequence for the port.\\nWrite Operation:"]
pub struct POCI_W<'a> {
    w: &'a mut W,
}
impl<'a> POCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POCI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nNo over current condition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POCI_A::_0)
    }
    #[doc = "Clear port suspend.\\nOver current condition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POCI_A::_1)
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
#[doc = "Port Reset Status\\nThis bit reflects the reset state of the port.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRS_A {
    #[doc = "0: No effect.\\nPort reset signal is not active"]
    _0 = 0,
    #[doc = "1: Set port reset.\\nPort reset signal is active"]
    _1 = 1,
}
impl From<PRS_A> for bool {
    #[inline(always)]
    fn from(variant: PRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRS` reader - Port Reset Status\\nThis bit reflects the reset state of the port.\\nWrite Operation:"]
pub struct PRS_R(crate::FieldReader<bool, PRS_A>);
impl PRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRS_A {
        match self.bits {
            false => PRS_A::_0,
            true => PRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRS_A::_1
    }
}
impl core::ops::Deref for PRS_R {
    type Target = crate::FieldReader<bool, PRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRS` writer - Port Reset Status\\nThis bit reflects the reset state of the port.\\nWrite Operation:"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nPort reset signal is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRS_A::_0)
    }
    #[doc = "Set port reset.\\nPort reset signal is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRS_A::_1)
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
#[doc = "Port Power Status\\nThis bit reflects the power state of the port regardless of the power switching mode.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS_A {
    #[doc = "0: No effect.\\nPort power is Diabled"]
    _0 = 0,
    #[doc = "1: Port Power Enabled.\\nPort power is Enabled"]
    _1 = 1,
}
impl From<PPS_A> for bool {
    #[inline(always)]
    fn from(variant: PPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPS` reader - Port Power Status\\nThis bit reflects the power state of the port regardless of the power switching mode.\\nWrite Operation:"]
pub struct PPS_R(crate::FieldReader<bool, PPS_A>);
impl PPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS_A {
        match self.bits {
            false => PPS_A::_0,
            true => PPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PPS_A::_1
    }
}
impl core::ops::Deref for PPS_R {
    type Target = crate::FieldReader<bool, PPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS` writer - Port Power Status\\nThis bit reflects the power state of the port regardless of the power switching mode.\\nWrite Operation:"]
pub struct PPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nPort power is Diabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PPS_A::_0)
    }
    #[doc = "Port Power Enabled.\\nPort power is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PPS_A::_1)
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
#[doc = "Low Speed Device Attached (Read) or Clear Port Power (Write)\\nThis bit defines the speed (and bud idle) of the attached device. It is only valid when CCS (HcRhPortStatus1\\[0\\]) is set.\\nThis bit is also used to clear port power.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSDA_A {
    #[doc = "0: No effect.\\nFull Speed device"]
    _0 = 0,
    #[doc = "1: Clear PPS (HcRhPortStatus1\\[8\\]).\\nLow-speed device"]
    _1 = 1,
}
impl From<LSDA_A> for bool {
    #[inline(always)]
    fn from(variant: LSDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSDA` reader - Low Speed Device Attached (Read) or Clear Port Power (Write)\\nThis bit defines the speed (and bud idle) of the attached device. It is only valid when CCS (HcRhPortStatus1\\[0\\]) is set.\\nThis bit is also used to clear port power.\\nWrite Operation:"]
pub struct LSDA_R(crate::FieldReader<bool, LSDA_A>);
impl LSDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSDA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSDA_A {
        match self.bits {
            false => LSDA_A::_0,
            true => LSDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LSDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LSDA_A::_1
    }
}
impl core::ops::Deref for LSDA_R {
    type Target = crate::FieldReader<bool, LSDA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSDA` writer - Low Speed Device Attached (Read) or Clear Port Power (Write)\\nThis bit defines the speed (and bud idle) of the attached device. It is only valid when CCS (HcRhPortStatus1\\[0\\]) is set.\\nThis bit is also used to clear port power.\\nWrite Operation:"]
pub struct LSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSDA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nFull Speed device"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSDA_A::_0)
    }
    #[doc = "Clear PPS (HcRhPortStatus1\\[8\\]).\\nLow-speed device"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSDA_A::_1)
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
#[doc = "Connect Status Change\\nThis bit indicates connect or disconnect event has been detected (CCS (HcRhPortStatus1\\[0\\]) changed).\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSC_A {
    #[doc = "0: No connect/disconnect event (CCS (HcRhPortStatus1\\[0\\]) didn't change)"]
    _0 = 0,
    #[doc = "1: Hardware detection of connect/disconnect event (CCS (HcRhPortStatus1\\[0\\]) changed)"]
    _1 = 1,
}
impl From<CSC_A> for bool {
    #[inline(always)]
    fn from(variant: CSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSC` reader - Connect Status Change\\nThis bit indicates connect or disconnect event has been detected (CCS (HcRhPortStatus1\\[0\\]) changed).\\nWrite 1 to clear this bit to zero."]
pub struct CSC_R(crate::FieldReader<bool, CSC_A>);
impl CSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSC_A {
        match self.bits {
            false => CSC_A::_0,
            true => CSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSC_A::_1
    }
}
impl core::ops::Deref for CSC_R {
    type Target = crate::FieldReader<bool, CSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSC` writer - Connect Status Change\\nThis bit indicates connect or disconnect event has been detected (CCS (HcRhPortStatus1\\[0\\]) changed).\\nWrite 1 to clear this bit to zero."]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No connect/disconnect event (CCS (HcRhPortStatus1\\[0\\]) didn't change)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSC_A::_0)
    }
    #[doc = "Hardware detection of connect/disconnect event (CCS (HcRhPortStatus1\\[0\\]) changed)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSC_A::_1)
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
#[doc = "Port Enable Status Change\\nThis bit indicates that the port has been disabled (PES (HcRhPortStatus1\\[1\\]) cleared) due to a hardware event.\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESC_A {
    #[doc = "0: PES (HcRhPortStatus1\\[1\\]) didn't change"]
    _0 = 0,
    #[doc = "1: PES (HcRhPortStatus1\\[1\\]) changed"]
    _1 = 1,
}
impl From<PESC_A> for bool {
    #[inline(always)]
    fn from(variant: PESC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESC` reader - Port Enable Status Change\\nThis bit indicates that the port has been disabled (PES (HcRhPortStatus1\\[1\\]) cleared) due to a hardware event.\\nWrite 1 to clear this bit to zero."]
pub struct PESC_R(crate::FieldReader<bool, PESC_A>);
impl PESC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESC_A {
        match self.bits {
            false => PESC_A::_0,
            true => PESC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PESC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PESC_A::_1
    }
}
impl core::ops::Deref for PESC_R {
    type Target = crate::FieldReader<bool, PESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESC` writer - Port Enable Status Change\\nThis bit indicates that the port has been disabled (PES (HcRhPortStatus1\\[1\\]) cleared) due to a hardware event.\\nWrite 1 to clear this bit to zero."]
pub struct PESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PES (HcRhPortStatus1\\[1\\]) didn't change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PESC_A::_0)
    }
    #[doc = "PES (HcRhPortStatus1\\[1\\]) changed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PESC_A::_1)
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
#[doc = "Port Suspend Status Change\\nThis bit indicates the completion of the selective resume sequence for the port.\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSSC_A {
    #[doc = "0: Port resume is not completed"]
    _0 = 0,
    #[doc = "1: Port resume completed"]
    _1 = 1,
}
impl From<PSSC_A> for bool {
    #[inline(always)]
    fn from(variant: PSSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSSC` reader - Port Suspend Status Change\\nThis bit indicates the completion of the selective resume sequence for the port.\\nWrite 1 to clear this bit to zero."]
pub struct PSSC_R(crate::FieldReader<bool, PSSC_A>);
impl PSSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSC_A {
        match self.bits {
            false => PSSC_A::_0,
            true => PSSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PSSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PSSC_A::_1
    }
}
impl core::ops::Deref for PSSC_R {
    type Target = crate::FieldReader<bool, PSSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSC` writer - Port Suspend Status Change\\nThis bit indicates the completion of the selective resume sequence for the port.\\nWrite 1 to clear this bit to zero."]
pub struct PSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port resume is not completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSSC_A::_0)
    }
    #[doc = "Port resume completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSSC_A::_1)
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
#[doc = "Port over Current Indicator Change\\nThis bit is set when POCI (HcRhPortStatus1\\[3\\]) changes.\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCIC_A {
    #[doc = "0: POCI (HcRhPortStatus1\\[3\\]) didn't change"]
    _0 = 0,
    #[doc = "1: POCI (HcRhPortStatus1\\[3\\]) changes"]
    _1 = 1,
}
impl From<OCIC_A> for bool {
    #[inline(always)]
    fn from(variant: OCIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCIC` reader - Port over Current Indicator Change\\nThis bit is set when POCI (HcRhPortStatus1\\[3\\]) changes.\\nWrite 1 to clear this bit to zero."]
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
#[doc = "Field `OCIC` writer - Port over Current Indicator Change\\nThis bit is set when POCI (HcRhPortStatus1\\[3\\]) changes.\\nWrite 1 to clear this bit to zero."]
pub struct OCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCIC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "POCI (HcRhPortStatus1\\[3\\]) didn't change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCIC_A::_0)
    }
    #[doc = "POCI (HcRhPortStatus1\\[3\\]) changes"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Port Reset Status Change\\nThis bit indicates that the port reset signal has completed.\\nWrite 1 to clear this bit to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSC_A {
    #[doc = "0: Port reset is not complete"]
    _0 = 0,
    #[doc = "1: Port reset is complete"]
    _1 = 1,
}
impl From<PRSC_A> for bool {
    #[inline(always)]
    fn from(variant: PRSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSC` reader - Port Reset Status Change\\nThis bit indicates that the port reset signal has completed.\\nWrite 1 to clear this bit to zero."]
pub struct PRSC_R(crate::FieldReader<bool, PRSC_A>);
impl PRSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSC_A {
        match self.bits {
            false => PRSC_A::_0,
            true => PRSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRSC_A::_1
    }
}
impl core::ops::Deref for PRSC_R {
    type Target = crate::FieldReader<bool, PRSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRSC` writer - Port Reset Status Change\\nThis bit indicates that the port reset signal has completed.\\nWrite 1 to clear this bit to zero."]
pub struct PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port reset is not complete"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRSC_A::_0)
    }
    #[doc = "Port reset is complete"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRSC_A::_1)
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
impl R {
    #[doc = "Bit 0 - CurrentConnectStatus (Read) or ClearPortEnable Bit (Write)\\nWrite Operation:"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Enable Status\\nWrite Operation:"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Suspend Status\\nThis bit indicates the port is suspended\\nWrite Operation:"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port over Current Indicator (Read) or Clear Port Suspend (Write)\\nThis bit reflects the state of the over current status pin dedicated to this port. This field is only valid if NOCP (HcRhDescriptorA\\[12\\]) is cleared and OCPM (HcRhDescriptorA\\[11\\]) is set.\\nThis bit is also used to initiate the selective result sequence for the port.\\nWrite Operation:"]
    #[inline(always)]
    pub fn poci(&self) -> POCI_R {
        POCI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Reset Status\\nThis bit reflects the reset state of the port.\\nWrite Operation:"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Power Status\\nThis bit reflects the power state of the port regardless of the power switching mode.\\nWrite Operation:"]
    #[inline(always)]
    pub fn pps(&self) -> PPS_R {
        PPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low Speed Device Attached (Read) or Clear Port Power (Write)\\nThis bit defines the speed (and bud idle) of the attached device. It is only valid when CCS (HcRhPortStatus1\\[0\\]) is set.\\nThis bit is also used to clear port power.\\nWrite Operation:"]
    #[inline(always)]
    pub fn lsda(&self) -> LSDA_R {
        LSDA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connect Status Change\\nThis bit indicates connect or disconnect event has been detected (CCS (HcRhPortStatus1\\[0\\]) changed).\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Enable Status Change\\nThis bit indicates that the port has been disabled (PES (HcRhPortStatus1\\[1\\]) cleared) due to a hardware event.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pesc(&self) -> PESC_R {
        PESC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Suspend Status Change\\nThis bit indicates the completion of the selective resume sequence for the port.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port over Current Indicator Change\\nThis bit is set when POCI (HcRhPortStatus1\\[3\\]) changes.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Reset Status Change\\nThis bit indicates that the port reset signal has completed.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CurrentConnectStatus (Read) or ClearPortEnable Bit (Write)\\nWrite Operation:"]
    #[inline(always)]
    pub fn ccs(&mut self) -> CCS_W {
        CCS_W { w: self }
    }
    #[doc = "Bit 1 - Port Enable Status\\nWrite Operation:"]
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W {
        PES_W { w: self }
    }
    #[doc = "Bit 2 - Port Suspend Status\\nThis bit indicates the port is suspended\\nWrite Operation:"]
    #[inline(always)]
    pub fn pss(&mut self) -> PSS_W {
        PSS_W { w: self }
    }
    #[doc = "Bit 3 - Port over Current Indicator (Read) or Clear Port Suspend (Write)\\nThis bit reflects the state of the over current status pin dedicated to this port. This field is only valid if NOCP (HcRhDescriptorA\\[12\\]) is cleared and OCPM (HcRhDescriptorA\\[11\\]) is set.\\nThis bit is also used to initiate the selective result sequence for the port.\\nWrite Operation:"]
    #[inline(always)]
    pub fn poci(&mut self) -> POCI_W {
        POCI_W { w: self }
    }
    #[doc = "Bit 4 - Port Reset Status\\nThis bit reflects the reset state of the port.\\nWrite Operation:"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Bit 8 - Port Power Status\\nThis bit reflects the power state of the port regardless of the power switching mode.\\nWrite Operation:"]
    #[inline(always)]
    pub fn pps(&mut self) -> PPS_W {
        PPS_W { w: self }
    }
    #[doc = "Bit 9 - Low Speed Device Attached (Read) or Clear Port Power (Write)\\nThis bit defines the speed (and bud idle) of the attached device. It is only valid when CCS (HcRhPortStatus1\\[0\\]) is set.\\nThis bit is also used to clear port power.\\nWrite Operation:"]
    #[inline(always)]
    pub fn lsda(&mut self) -> LSDA_W {
        LSDA_W { w: self }
    }
    #[doc = "Bit 16 - Connect Status Change\\nThis bit indicates connect or disconnect event has been detected (CCS (HcRhPortStatus1\\[0\\]) changed).\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 17 - Port Enable Status Change\\nThis bit indicates that the port has been disabled (PES (HcRhPortStatus1\\[1\\]) cleared) due to a hardware event.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pesc(&mut self) -> PESC_W {
        PESC_W { w: self }
    }
    #[doc = "Bit 18 - Port Suspend Status Change\\nThis bit indicates the completion of the selective resume sequence for the port.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pssc(&mut self) -> PSSC_W {
        PSSC_W { w: self }
    }
    #[doc = "Bit 19 - Port over Current Indicator Change\\nThis bit is set when POCI (HcRhPortStatus1\\[3\\]) changes.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 20 - Port Reset Status Change\\nThis bit indicates that the port reset signal has completed.\\nWrite 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn prsc(&mut self) -> PRSC_W {
        PRSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Root Hub Port Status \\[1\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhportstatus1](index.html) module"]
pub struct HCRHPORTSTATUS1_SPEC;
impl crate::RegisterSpec for HCRHPORTSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhportstatus1::R](R) reader structure"]
impl crate::Readable for HCRHPORTSTATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhportstatus1::W](W) writer structure"]
impl crate::Writable for HCRHPORTSTATUS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHPORTSTATUS1 to value 0"]
impl crate::Resettable for HCRHPORTSTATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
