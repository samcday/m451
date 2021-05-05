#[doc = "Register `HCINTERRUPTENABLE` reader"]
pub struct R(crate::R<HCINTERRUPTENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTERRUPTENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCINTERRUPTENABLE_SPEC>> for R {
    fn from(reader: crate::R<HCINTERRUPTENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTERRUPTENABLE` writer"]
pub struct W(crate::W<HCINTERRUPTENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTERRUPTENABLE_SPEC>;
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
impl core::convert::From<crate::W<HCINTERRUPTENABLE_SPEC>> for W {
    fn from(writer: crate::W<HCINTERRUPTENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scheduling Overrun Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SO_A {
    #[doc = "0: No effect.\\nInterrupt generation due to SO (HcInterruptStatus\\[0\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to SO (HcInterruptStatus\\[0\\]) Enabled"]
    _1 = 1,
}
impl From<SO_A> for bool {
    #[inline(always)]
    fn from(variant: SO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SO` reader - Scheduling Overrun Enable Bit\\nWrite Operation:"]
pub struct SO_R(crate::FieldReader<bool, SO_A>);
impl SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SO_A {
        match self.bits {
            false => SO_A::_0,
            true => SO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SO_A::_1
    }
}
impl core::ops::Deref for SO_R {
    type Target = crate::FieldReader<bool, SO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SO` writer - Scheduling Overrun Enable Bit\\nWrite Operation:"]
pub struct SO_W<'a> {
    w: &'a mut W,
}
impl<'a> SO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to SO (HcInterruptStatus\\[0\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SO_A::_0)
    }
    #[doc = "Interrupt generation due to SO (HcInterruptStatus\\[0\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SO_A::_1)
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
#[doc = "Write Back Done Head Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDH_A {
    #[doc = "0: No effect.\\nInterrupt generation due to WDH (HcInterruptStatus\\[1\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to WDH (HcInterruptStatus\\[1\\]) Enabled"]
    _1 = 1,
}
impl From<WDH_A> for bool {
    #[inline(always)]
    fn from(variant: WDH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDH` reader - Write Back Done Head Enable Bit\\nWrite Operation:"]
pub struct WDH_R(crate::FieldReader<bool, WDH_A>);
impl WDH_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDH_A {
        match self.bits {
            false => WDH_A::_0,
            true => WDH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDH_A::_1
    }
}
impl core::ops::Deref for WDH_R {
    type Target = crate::FieldReader<bool, WDH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDH` writer - Write Back Done Head Enable Bit\\nWrite Operation:"]
pub struct WDH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to WDH (HcInterruptStatus\\[1\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDH_A::_0)
    }
    #[doc = "Interrupt generation due to WDH (HcInterruptStatus\\[1\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDH_A::_1)
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
#[doc = "Start of Frame Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SF_A {
    #[doc = "0: No effect.\\nInterrupt generation due to SF (HcInterruptStatus\\[2\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to SF (HcInterruptStatus\\[2\\]) Enabled"]
    _1 = 1,
}
impl From<SF_A> for bool {
    #[inline(always)]
    fn from(variant: SF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SF` reader - Start of Frame Enable Bit\\nWrite Operation:"]
pub struct SF_R(crate::FieldReader<bool, SF_A>);
impl SF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SF_A {
        match self.bits {
            false => SF_A::_0,
            true => SF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SF_A::_1
    }
}
impl core::ops::Deref for SF_R {
    type Target = crate::FieldReader<bool, SF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SF` writer - Start of Frame Enable Bit\\nWrite Operation:"]
pub struct SF_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to SF (HcInterruptStatus\\[2\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SF_A::_0)
    }
    #[doc = "Interrupt generation due to SF (HcInterruptStatus\\[2\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SF_A::_1)
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
#[doc = "Resume Detected Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_A {
    #[doc = "0: No effect.\\nInterrupt generation due to RD (HcInterruptStatus\\[3\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to RD (HcInterruptStatus\\[3\\]) Enabled"]
    _1 = 1,
}
impl From<RD_A> for bool {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD` reader - Resume Detected Enable Bit\\nWrite Operation:"]
pub struct RD_R(crate::FieldReader<bool, RD_A>);
impl RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_A {
        match self.bits {
            false => RD_A::_0,
            true => RD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RD_A::_1
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<bool, RD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD` writer - Resume Detected Enable Bit\\nWrite Operation:"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to RD (HcInterruptStatus\\[3\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RD_A::_0)
    }
    #[doc = "Interrupt generation due to RD (HcInterruptStatus\\[3\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RD_A::_1)
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
#[doc = "Frame Number Overflow Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FNO_A {
    #[doc = "0: No effect.\\nInterrupt generation due to FNO (HcInterruptStatus\\[5\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to FNO (HcInterruptStatus\\[5\\]) Enabled"]
    _1 = 1,
}
impl From<FNO_A> for bool {
    #[inline(always)]
    fn from(variant: FNO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FNO` reader - Frame Number Overflow Enable Bit\\nWrite Operation:"]
pub struct FNO_R(crate::FieldReader<bool, FNO_A>);
impl FNO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FNO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FNO_A {
        match self.bits {
            false => FNO_A::_0,
            true => FNO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FNO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FNO_A::_1
    }
}
impl core::ops::Deref for FNO_R {
    type Target = crate::FieldReader<bool, FNO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNO` writer - Frame Number Overflow Enable Bit\\nWrite Operation:"]
pub struct FNO_W<'a> {
    w: &'a mut W,
}
impl<'a> FNO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FNO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to FNO (HcInterruptStatus\\[5\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FNO_A::_0)
    }
    #[doc = "Interrupt generation due to FNO (HcInterruptStatus\\[5\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FNO_A::_1)
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
#[doc = "Root Hub Status Change Enable Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RHSC_A {
    #[doc = "0: No effect.\\nInterrupt generation due to RHSC (HcInterruptStatus\\[6\\]) Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to RHSC (HcInterruptStatus\\[6\\]) Enabled"]
    _1 = 1,
}
impl From<RHSC_A> for bool {
    #[inline(always)]
    fn from(variant: RHSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RHSC` reader - Root Hub Status Change Enable Bit\\nWrite Operation:"]
pub struct RHSC_R(crate::FieldReader<bool, RHSC_A>);
impl RHSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RHSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHSC_A {
        match self.bits {
            false => RHSC_A::_0,
            true => RHSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RHSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RHSC_A::_1
    }
}
impl core::ops::Deref for RHSC_R {
    type Target = crate::FieldReader<bool, RHSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHSC` writer - Root Hub Status Change Enable Bit\\nWrite Operation:"]
pub struct RHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RHSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to RHSC (HcInterruptStatus\\[6\\]) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RHSC_A::_0)
    }
    #[doc = "Interrupt generation due to RHSC (HcInterruptStatus\\[6\\]) Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RHSC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Master Interrupt Enable Bit\\nThis bit is a global interrupt enable. A write of '1' allows interrupts to be enabled via the specific enable bits listed above.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIE_A {
    #[doc = "0: No effect.\\nInterrupt generation due to RHSC (HcInterruptStatus\\[6\\]), FNO (HcInterruptStatus\\[5\\]), RD (HcInterruptStatus\\[3\\]), SF (HcInterruptStatus\\[2\\]), WDH (HcInterruptStatus\\[1\\]) or SO (HcInterruptStatus\\[0\\]) Disabled even if the corresponding bit in HcInterruptEnable is high"]
    _0 = 0,
    #[doc = "1: Interrupt generation due to RHSC (HcInterruptStatus\\[6\\]), FNO (HcInterruptStatus\\[5\\]), RD (HcInterruptStatus\\[3\\]), SF (HcInterruptStatus\\[2\\]), WDH (HcInterruptStatus\\[1\\]) or SO (HcInterruptStatus\\[0\\]) Enabled if the corresponding bit in HcInterruptEnable is high"]
    _1 = 1,
}
impl From<MIE_A> for bool {
    #[inline(always)]
    fn from(variant: MIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIE` reader - Master Interrupt Enable Bit\\nThis bit is a global interrupt enable. A write of '1' allows interrupts to be enabled via the specific enable bits listed above.\\nWrite Operation:"]
pub struct MIE_R(crate::FieldReader<bool, MIE_A>);
impl MIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIE_A {
        match self.bits {
            false => MIE_A::_0,
            true => MIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MIE_A::_1
    }
}
impl core::ops::Deref for MIE_R {
    type Target = crate::FieldReader<bool, MIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIE` writer - Master Interrupt Enable Bit\\nThis bit is a global interrupt enable. A write of '1' allows interrupts to be enabled via the specific enable bits listed above.\\nWrite Operation:"]
pub struct MIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nInterrupt generation due to RHSC (HcInterruptStatus\\[6\\]), FNO (HcInterruptStatus\\[5\\]), RD (HcInterruptStatus\\[3\\]), SF (HcInterruptStatus\\[2\\]), WDH (HcInterruptStatus\\[1\\]) or SO (HcInterruptStatus\\[0\\]) Disabled even if the corresponding bit in HcInterruptEnable is high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MIE_A::_0)
    }
    #[doc = "Interrupt generation due to RHSC (HcInterruptStatus\\[6\\]), FNO (HcInterruptStatus\\[5\\]), RD (HcInterruptStatus\\[3\\]), SF (HcInterruptStatus\\[2\\]), WDH (HcInterruptStatus\\[1\\]) or SO (HcInterruptStatus\\[0\\]) Enabled if the corresponding bit in HcInterruptEnable is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MIE_A::_1)
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
    #[doc = "Bit 0 - Scheduling Overrun Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Back Done Head Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Resume Detected Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Frame Number Overflow Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Root Hub Status Change Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master Interrupt Enable Bit\\nThis bit is a global interrupt enable. A write of '1' allows interrupts to be enabled via the specific enable bits listed above.\\nWrite Operation:"]
    #[inline(always)]
    pub fn mie(&self) -> MIE_R {
        MIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scheduling Overrun Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn so(&mut self) -> SO_W {
        SO_W { w: self }
    }
    #[doc = "Bit 1 - Write Back Done Head Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn wdh(&mut self) -> WDH_W {
        WDH_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn sf(&mut self) -> SF_W {
        SF_W { w: self }
    }
    #[doc = "Bit 3 - Resume Detected Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bit 5 - Frame Number Overflow Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn fno(&mut self) -> FNO_W {
        FNO_W { w: self }
    }
    #[doc = "Bit 6 - Root Hub Status Change Enable Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn rhsc(&mut self) -> RHSC_W {
        RHSC_W { w: self }
    }
    #[doc = "Bit 31 - Master Interrupt Enable Bit\\nThis bit is a global interrupt enable. A write of '1' allows interrupts to be enabled via the specific enable bits listed above.\\nWrite Operation:"]
    #[inline(always)]
    pub fn mie(&mut self) -> MIE_W {
        MIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptenable](index.html) module"]
pub struct HCINTERRUPTENABLE_SPEC;
impl crate::RegisterSpec for HCINTERRUPTENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcinterruptenable::R](R) reader structure"]
impl crate::Readable for HCINTERRUPTENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcinterruptenable::W](W) writer structure"]
impl crate::Writable for HCINTERRUPTENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINTERRUPTENABLE to value 0"]
impl crate::Resettable for HCINTERRUPTENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
