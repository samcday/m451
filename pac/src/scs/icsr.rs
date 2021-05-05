#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ICSR_SPEC>> for R {
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl core::convert::From<crate::W<ICSR_SPEC>> for W {
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Number of the Current Active Exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VECTACTIVE_A {
    #[doc = "0: Thread mode"]
    _0 = 0,
}
impl From<VECTACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: VECTACTIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VECTACTIVE` reader - Number of the Current Active Exception"]
pub struct VECTACTIVE_R(crate::FieldReader<u8, VECTACTIVE_A>);
impl VECTACTIVE_R {
    pub(crate) fn new(bits: u8) -> Self {
        VECTACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VECTACTIVE_A> {
        match self.bits {
            0 => Some(VECTACTIVE_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VECTACTIVE_A::_0
    }
}
impl core::ops::Deref for VECTACTIVE_R {
    type Target = crate::FieldReader<u8, VECTACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTACTIVE` writer - Number of the Current Active Exception"]
pub struct VECTACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTACTIVE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Thread mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VECTACTIVE_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Preempted Active Exceptions Indicator\\nIndicate whether There are Preempted Active Exceptions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASE_A {
    #[doc = "0: there are preempted active exceptions to execute"]
    _0 = 0,
    #[doc = "1: there are no active exceptions, or the currently-executing exception is the only active exception"]
    _1 = 1,
}
impl From<RETTOBASE_A> for bool {
    #[inline(always)]
    fn from(variant: RETTOBASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETTOBASE` reader - Preempted Active Exceptions Indicator\\nIndicate whether There are Preempted Active Exceptions"]
pub struct RETTOBASE_R(crate::FieldReader<bool, RETTOBASE_A>);
impl RETTOBASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETTOBASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETTOBASE_A {
        match self.bits {
            false => RETTOBASE_A::_0,
            true => RETTOBASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RETTOBASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RETTOBASE_A::_1
    }
}
impl core::ops::Deref for RETTOBASE_R {
    type Target = crate::FieldReader<bool, RETTOBASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETTOBASE` writer - Preempted Active Exceptions Indicator\\nIndicate whether There are Preempted Active Exceptions"]
pub struct RETTOBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RETTOBASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETTOBASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "there are preempted active exceptions to execute"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RETTOBASE_A::_0)
    }
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RETTOBASE_A::_1)
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
#[doc = "Number of the Highest Pended Exception\\nIndicate the Exception Number of the Highest Priority Pending Enabled Exception\\nThe value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VECTPENDING_A {
    #[doc = "0: no pending exceptions"]
    _0 = 0,
}
impl From<VECTPENDING_A> for u8 {
    #[inline(always)]
    fn from(variant: VECTPENDING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VECTPENDING` reader - Number of the Highest Pended Exception\\nIndicate the Exception Number of the Highest Priority Pending Enabled Exception\\nThe value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register."]
pub struct VECTPENDING_R(crate::FieldReader<u8, VECTPENDING_A>);
impl VECTPENDING_R {
    pub(crate) fn new(bits: u8) -> Self {
        VECTPENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VECTPENDING_A> {
        match self.bits {
            0 => Some(VECTPENDING_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VECTPENDING_A::_0
    }
}
impl core::ops::Deref for VECTPENDING_R {
    type Target = crate::FieldReader<u8, VECTPENDING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTPENDING` writer - Number of the Highest Pended Exception\\nIndicate the Exception Number of the Highest Priority Pending Enabled Exception\\nThe value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register."]
pub struct VECTPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTPENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTPENDING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no pending exceptions"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VECTPENDING_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Interrupt Pending Flag, Excluding NMI and Faults (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDING_A {
    #[doc = "0: Interrupt not pending"]
    _0 = 0,
    #[doc = "1: Interrupt pending"]
    _1 = 1,
}
impl From<ISRPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISRPENDING` reader - Interrupt Pending Flag, Excluding NMI and Faults (Read Only)"]
pub struct ISRPENDING_R(crate::FieldReader<bool, ISRPENDING_A>);
impl ISRPENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISRPENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPENDING_A {
        match self.bits {
            false => ISRPENDING_A::_0,
            true => ISRPENDING_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISRPENDING_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISRPENDING_A::_1
    }
}
impl core::ops::Deref for ISRPENDING_R {
    type Target = crate::FieldReader<bool, ISRPENDING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISRPREEMPT` reader - Interrupt Preempt Bit (Read Only)\\nIf set, a pending exception will be serviced on exit from the debug halt state."]
pub struct ISRPREEMPT_R(crate::FieldReader<bool, bool>);
impl ISRPREEMPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISRPREEMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISRPREEMPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SysTick Exception Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDST bit, you must 'write 0 to PENDSTSET and write 1 to PENDSTRTC_CAL' at the same time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTRTC_CAL_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Removes the pending state from the SysTick exception"]
    _1 = 1,
}
impl From<PENDSTRTC_CAL_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTRTC_CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTRTC_CAL` reader - SysTick Exception Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDST bit, you must 'write 0 to PENDSTSET and write 1 to PENDSTRTC_CAL' at the same time."]
pub struct PENDSTRTC_CAL_R(crate::FieldReader<bool, PENDSTRTC_CAL_A>);
impl PENDSTRTC_CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTRTC_CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTRTC_CAL_A {
        match self.bits {
            false => PENDSTRTC_CAL_A::_0,
            true => PENDSTRTC_CAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSTRTC_CAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSTRTC_CAL_A::_1
    }
}
impl core::ops::Deref for PENDSTRTC_CAL_R {
    type Target = crate::FieldReader<bool, PENDSTRTC_CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTRTC_CAL` writer - SysTick Exception Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDST bit, you must 'write 0 to PENDSTSET and write 1 to PENDSTRTC_CAL' at the same time."]
pub struct PENDSTRTC_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTRTC_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTRTC_CAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTRTC_CAL_A::_0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTRTC_CAL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "SysTick Exception Set-pending Bit\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: No effect.\\nSysTick exception is not pending"]
    _0 = 0,
    #[doc = "1: Changes SysTick exception state to pending.\\nSysTick exception is pending"]
    _1 = 1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick Exception Set-pending Bit\\nWrite Operation:"]
pub struct PENDSTSET_R(crate::FieldReader<bool, PENDSTSET_A>);
impl PENDSTSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::_0,
            true => PENDSTSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSTSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSTSET_A::_1
    }
}
impl core::ops::Deref for PENDSTSET_R {
    type Target = crate::FieldReader<bool, PENDSTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick Exception Set-pending Bit\\nWrite Operation:"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nSysTick exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_0)
    }
    #[doc = "Changes SysTick exception state to pending.\\nSysTick exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "PendSV Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDSV bit, you must 'write 0 to PENDSVSET and write 1 to PENDSVRTC_CAL' at the same time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVRTC_CAL_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Removes the pending state from the PendSV exception"]
    _1 = 1,
}
impl From<PENDSVRTC_CAL_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVRTC_CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVRTC_CAL` reader - PendSV Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDSV bit, you must 'write 0 to PENDSVSET and write 1 to PENDSVRTC_CAL' at the same time."]
pub struct PENDSVRTC_CAL_R(crate::FieldReader<bool, PENDSVRTC_CAL_A>);
impl PENDSVRTC_CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVRTC_CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVRTC_CAL_A {
        match self.bits {
            false => PENDSVRTC_CAL_A::_0,
            true => PENDSVRTC_CAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSVRTC_CAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSVRTC_CAL_A::_1
    }
}
impl core::ops::Deref for PENDSVRTC_CAL_R {
    type Target = crate::FieldReader<bool, PENDSVRTC_CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVRTC_CAL` writer - PendSV Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDSV bit, you must 'write 0 to PENDSVSET and write 1 to PENDSVRTC_CAL' at the same time."]
pub struct PENDSVRTC_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVRTC_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVRTC_CAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVRTC_CAL_A::_0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVRTC_CAL_A::_1)
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
#[doc = "PendSV Set-pending Bit\\nWrite Operation:\\nNote: Writing 1 to this bit is the only way to set the PendSV exception state to pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: No effect.\\nPendSV exception is not pending"]
    _0 = 0,
    #[doc = "1: Changes PendSV exception state to pending.\\nPendSV exception is pending"]
    _1 = 1,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV Set-pending Bit\\nWrite Operation:\\nNote: Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
pub struct PENDSVSET_R(crate::FieldReader<bool, PENDSVSET_A>);
impl PENDSVSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::_0,
            true => PENDSVSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PENDSVSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PENDSVSET_A::_1
    }
}
impl core::ops::Deref for PENDSVSET_R {
    type Target = crate::FieldReader<bool, PENDSVSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV Set-pending Bit\\nWrite Operation:\\nNote: Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nPendSV exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_0)
    }
    #[doc = "Changes PendSV exception state to pending.\\nPendSV exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "NMI Set-pending Bit\\nWrite Operation:\\nNote: Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "0: No effect.\\nNMI exception is not pending"]
    _0 = 0,
    #[doc = "1: Changes NMI exception state to pending.\\nNMI exception is pending"]
    _1 = 1,
}
impl From<NMIPENDSET_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI Set-pending Bit\\nWrite Operation:\\nNote: Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
pub struct NMIPENDSET_R(crate::FieldReader<bool, NMIPENDSET_A>);
impl NMIPENDSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIPENDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::_0,
            true => NMIPENDSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NMIPENDSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NMIPENDSET_A::_1
    }
}
impl core::ops::Deref for NMIPENDSET_R {
    type Target = crate::FieldReader<bool, NMIPENDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI Set-pending Bit\\nWrite Operation:\\nNote: Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect.\\nNMI exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_0)
    }
    #[doc = "Changes NMI exception state to pending.\\nNMI exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_1)
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
    #[doc = "Bits 0:5 - Number of the Current Active Exception"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 11 - Preempted Active Exceptions Indicator\\nIndicate whether There are Preempted Active Exceptions"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Number of the Highest Pended Exception\\nIndicate the Exception Number of the Highest Priority Pending Enabled Exception\\nThe value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register."]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt Pending Flag, Excluding NMI and Faults (Read Only)"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt Preempt Bit (Read Only)\\nIf set, a pending exception will be serviced on exit from the debug halt state."]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SysTick Exception Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDST bit, you must 'write 0 to PENDSTSET and write 1 to PENDSTRTC_CAL' at the same time."]
    #[inline(always)]
    pub fn pendstrtc_cal(&self) -> PENDSTRTC_CAL_R {
        PENDSTRTC_CAL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PendSV Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDSV bit, you must 'write 0 to PENDSVSET and write 1 to PENDSVRTC_CAL' at the same time."]
    #[inline(always)]
    pub fn pendsvrtc_cal(&self) -> PENDSVRTC_CAL_R {
        PENDSVRTC_CAL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV Set-pending Bit\\nWrite Operation:\\nNote: Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NMI Set-pending Bit\\nWrite Operation:\\nNote: Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of the Current Active Exception"]
    #[inline(always)]
    pub fn vectactive(&mut self) -> VECTACTIVE_W {
        VECTACTIVE_W { w: self }
    }
    #[doc = "Bit 11 - Preempted Active Exceptions Indicator\\nIndicate whether There are Preempted Active Exceptions"]
    #[inline(always)]
    pub fn rettobase(&mut self) -> RETTOBASE_W {
        RETTOBASE_W { w: self }
    }
    #[doc = "Bits 12:17 - Number of the Highest Pended Exception\\nIndicate the Exception Number of the Highest Priority Pending Enabled Exception\\nThe value indicated by this field includes the effect of the BASEPRI and FAULTMASK registers, but not any effect of the PRIMASK register."]
    #[inline(always)]
    pub fn vectpending(&mut self) -> VECTPENDING_W {
        VECTPENDING_W { w: self }
    }
    #[doc = "Bit 25 - SysTick Exception Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDST bit, you must 'write 0 to PENDSTSET and write 1 to PENDSTRTC_CAL' at the same time."]
    #[inline(always)]
    pub fn pendstrtc_cal(&mut self) -> PENDSTRTC_CAL_W {
        PENDSTRTC_CAL_W { w: self }
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending Bit\\nWrite Operation:"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV Clear-pending Bit\\nWrite Operation:\\nNote: This is a write only bit. To clear the PENDSV bit, you must 'write 0 to PENDSVSET and write 1 to PENDSVRTC_CAL' at the same time."]
    #[inline(always)]
    pub fn pendsvrtc_cal(&mut self) -> PENDSVRTC_CAL_W {
        PENDSVRTC_CAL_W { w: self }
    }
    #[doc = "Bit 28 - PendSV Set-pending Bit\\nWrite Operation:\\nNote: Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - NMI Set-pending Bit\\nWrite Operation:\\nNote: Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
