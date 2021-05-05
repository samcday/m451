#[doc = "Register `SYS_RSTSTS` reader"]
pub struct R(crate::R<SYS_RSTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_RSTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_RSTSTS_SPEC>> for R {
    fn from(reader: crate::R<SYS_RSTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_RSTSTS` writer"]
pub struct W(crate::W<SYS_RSTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_RSTSTS_SPEC>;
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
impl core::convert::From<crate::W<SYS_RSTSTS_SPEC>> for W {
    fn from(writer: crate::W<SYS_RSTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "POR Reset Flag\\nThe POR reset flag is set by the 'Reset Signal' from the Power-on Reset (POR) Controller or bit CHIPRST (SYS_IPRST0\\[0\\]) to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORF_A {
    #[doc = "0: No reset from POR or CHIPRST"]
    _0 = 0,
    #[doc = "1: Power-on Reset (POR) or CHIPRST had issued the reset signal to reset the system"]
    _1 = 1,
}
impl From<PORF_A> for bool {
    #[inline(always)]
    fn from(variant: PORF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORF` reader - POR Reset Flag\\nThe POR reset flag is set by the 'Reset Signal' from the Power-on Reset (POR) Controller or bit CHIPRST (SYS_IPRST0\\[0\\]) to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct PORF_R(crate::FieldReader<bool, PORF_A>);
impl PORF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORF_A {
        match self.bits {
            false => PORF_A::_0,
            true => PORF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PORF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PORF_A::_1
    }
}
impl core::ops::Deref for PORF_R {
    type Target = crate::FieldReader<bool, PORF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORF` writer - POR Reset Flag\\nThe POR reset flag is set by the 'Reset Signal' from the Power-on Reset (POR) Controller or bit CHIPRST (SYS_IPRST0\\[0\\]) to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct PORF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from POR or CHIPRST"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORF_A::_0)
    }
    #[doc = "Power-on Reset (POR) or CHIPRST had issued the reset signal to reset the system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORF_A::_1)
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
#[doc = "NRESET Pin Reset Flag\\nThe nRESET pin reset flag is set by the 'Reset Signal' from the nRESET Pin to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINRF_A {
    #[doc = "0: No reset from nRESET pin"]
    _0 = 0,
    #[doc = "1: Pin nRESET had issued the reset signal to reset the system"]
    _1 = 1,
}
impl From<PINRF_A> for bool {
    #[inline(always)]
    fn from(variant: PINRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRF` reader - NRESET Pin Reset Flag\\nThe nRESET pin reset flag is set by the 'Reset Signal' from the nRESET Pin to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct PINRF_R(crate::FieldReader<bool, PINRF_A>);
impl PINRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINRF_A {
        match self.bits {
            false => PINRF_A::_0,
            true => PINRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINRF_A::_1
    }
}
impl core::ops::Deref for PINRF_R {
    type Target = crate::FieldReader<bool, PINRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINRF` writer - NRESET Pin Reset Flag\\nThe nRESET pin reset flag is set by the 'Reset Signal' from the nRESET Pin to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct PINRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PINRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from nRESET pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINRF_A::_0)
    }
    #[doc = "Pin nRESET had issued the reset signal to reset the system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINRF_A::_1)
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
#[doc = "WDT Reset Flag\\nThe WDT reset flag is set by the 'Reset Signal' from the Watchdog Timer or Window Watchdog Timer to indicate the previous reset source.\\nNote1: Write 1 to clear this bit to 0.\\nNote2: Watchdog Timer register RSTF(WDT_CTL\\[2\\]) bit is set if the system has been reset by WDT time-out reset. Window Watchdog Timer register WWDTRF(WWDT_STATUS\\[1\\]) bit is set if the system has been reset by WWDT time-out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRF_A {
    #[doc = "0: No reset from watchdog timer or window watchdog timer"]
    _0 = 0,
    #[doc = "1: The watchdog timer or window watchdog timer had issued the reset signal to reset the system"]
    _1 = 1,
}
impl From<WDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRF` reader - WDT Reset Flag\\nThe WDT reset flag is set by the 'Reset Signal' from the Watchdog Timer or Window Watchdog Timer to indicate the previous reset source.\\nNote1: Write 1 to clear this bit to 0.\\nNote2: Watchdog Timer register RSTF(WDT_CTL\\[2\\]) bit is set if the system has been reset by WDT time-out reset. Window Watchdog Timer register WWDTRF(WWDT_STATUS\\[1\\]) bit is set if the system has been reset by WWDT time-out reset."]
pub struct WDTRF_R(crate::FieldReader<bool, WDTRF_A>);
impl WDTRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTRF_A {
        match self.bits {
            false => WDTRF_A::_0,
            true => WDTRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDTRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDTRF_A::_1
    }
}
impl core::ops::Deref for WDTRF_R {
    type Target = crate::FieldReader<bool, WDTRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTRF` writer - WDT Reset Flag\\nThe WDT reset flag is set by the 'Reset Signal' from the Watchdog Timer or Window Watchdog Timer to indicate the previous reset source.\\nNote1: Write 1 to clear this bit to 0.\\nNote2: Watchdog Timer register RSTF(WDT_CTL\\[2\\]) bit is set if the system has been reset by WDT time-out reset. Window Watchdog Timer register WWDTRF(WWDT_STATUS\\[1\\]) bit is set if the system has been reset by WWDT time-out reset."]
pub struct WDTRF_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from watchdog timer or window watchdog timer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTRF_A::_0)
    }
    #[doc = "The watchdog timer or window watchdog timer had issued the reset signal to reset the system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTRF_A::_1)
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
#[doc = "LVR Reset Flag\\nThe LVR reset flag is set by the 'Reset Signal' from the Low Voltage Reset Controller to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVRF_A {
    #[doc = "0: No reset from LVR"]
    _0 = 0,
    #[doc = "1: LVR controller had issued the reset signal to reset the system"]
    _1 = 1,
}
impl From<LVRF_A> for bool {
    #[inline(always)]
    fn from(variant: LVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVRF` reader - LVR Reset Flag\\nThe LVR reset flag is set by the 'Reset Signal' from the Low Voltage Reset Controller to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct LVRF_R(crate::FieldReader<bool, LVRF_A>);
impl LVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVRF_A {
        match self.bits {
            false => LVRF_A::_0,
            true => LVRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVRF_A::_1
    }
}
impl core::ops::Deref for LVRF_R {
    type Target = crate::FieldReader<bool, LVRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRF` writer - LVR Reset Flag\\nThe LVR reset flag is set by the 'Reset Signal' from the Low Voltage Reset Controller to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct LVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from LVR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVRF_A::_0)
    }
    #[doc = "LVR controller had issued the reset signal to reset the system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVRF_A::_1)
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
#[doc = "BOD Reset Flag\\nThe BOD reset flag is set by the 'Reset Signal' from the Brown-Out Detector to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRF_A {
    #[doc = "0: No reset from BOD"]
    _0 = 0,
    #[doc = "1: The BOD had issued the reset signal to reset the system"]
    _1 = 1,
}
impl From<BODRF_A> for bool {
    #[inline(always)]
    fn from(variant: BODRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRF` reader - BOD Reset Flag\\nThe BOD reset flag is set by the 'Reset Signal' from the Brown-Out Detector to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct BODRF_R(crate::FieldReader<bool, BODRF_A>);
impl BODRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRF_A {
        match self.bits {
            false => BODRF_A::_0,
            true => BODRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODRF_A::_1
    }
}
impl core::ops::Deref for BODRF_R {
    type Target = crate::FieldReader<bool, BODRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRF` writer - BOD Reset Flag\\nThe BOD reset flag is set by the 'Reset Signal' from the Brown-Out Detector to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct BODRF_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from BOD"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODRF_A::_0)
    }
    #[doc = "The BOD had issued the reset signal to reset the system"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODRF_A::_1)
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
#[doc = "System Reset Flag\\nThe system reset flag is set by the 'Reset Signal' from the Cortex-M4 Core to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRF_A {
    #[doc = "0: No reset from Cortex-M4"]
    _0 = 0,
    #[doc = "1: The Cortex-M4 had issued the reset signal to reset the system by writing 1 to the bit SYSRESETREQ(AIRCR\\[2\\], Application Interrupt and Reset Control Register, address = 0xE000ED0C) in system control registers of Cortex-M4 core"]
    _1 = 1,
}
impl From<SYSRF_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRF` reader - System Reset Flag\\nThe system reset flag is set by the 'Reset Signal' from the Cortex-M4 Core to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct SYSRF_R(crate::FieldReader<bool, SYSRF_A>);
impl SYSRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRF_A {
        match self.bits {
            false => SYSRF_A::_0,
            true => SYSRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYSRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYSRF_A::_1
    }
}
impl core::ops::Deref for SYSRF_R {
    type Target = crate::FieldReader<bool, SYSRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRF` writer - System Reset Flag\\nThe system reset flag is set by the 'Reset Signal' from the Cortex-M4 Core to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
pub struct SYSRF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from Cortex-M4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSRF_A::_0)
    }
    #[doc = "The Cortex-M4 had issued the reset signal to reset the system by writing 1 to the bit SYSRESETREQ(AIRCR\\[2\\], Application Interrupt and Reset Control Register, address = 0xE000ED0C) in system control registers of Cortex-M4 core"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSRF_A::_1)
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
#[doc = "CPU Reset Flag\\nThe CPU reset flag is set by hardware if software writes CPURST (SYS_IPRST0\\[1\\]) 1 to reset Cortex-M4 Core and Flash Memory Controller (FMC).\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURF_A {
    #[doc = "0: No reset from CPU"]
    _0 = 0,
    #[doc = "1: The Cortex-M4 Core and FMC are reset by software setting CPURST to 1"]
    _1 = 1,
}
impl From<CPURF_A> for bool {
    #[inline(always)]
    fn from(variant: CPURF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURF` reader - CPU Reset Flag\\nThe CPU reset flag is set by hardware if software writes CPURST (SYS_IPRST0\\[1\\]) 1 to reset Cortex-M4 Core and Flash Memory Controller (FMC).\\nNote: Write 1 to clear this bit to 0."]
pub struct CPURF_R(crate::FieldReader<bool, CPURF_A>);
impl CPURF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPURF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURF_A {
        match self.bits {
            false => CPURF_A::_0,
            true => CPURF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPURF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPURF_A::_1
    }
}
impl core::ops::Deref for CPURF_R {
    type Target = crate::FieldReader<bool, CPURF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPURF` writer - CPU Reset Flag\\nThe CPU reset flag is set by hardware if software writes CPURST (SYS_IPRST0\\[1\\]) 1 to reset Cortex-M4 Core and Flash Memory Controller (FMC).\\nNote: Write 1 to clear this bit to 0."]
pub struct CPURF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from CPU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPURF_A::_0)
    }
    #[doc = "The Cortex-M4 Core and FMC are reset by software setting CPURST to 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPURF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "the CPULK Reset Flag Is Set by Hardware If Cortex-m4 Lockup Happened (M45xD/M45xC Only)\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPULKRF_A {
    #[doc = "0: No reset from CPU lockup happened"]
    _0 = 0,
    #[doc = "1: The Cortex-M4 lockup happened and chip is reset"]
    _1 = 1,
}
impl From<CPULKRF_A> for bool {
    #[inline(always)]
    fn from(variant: CPULKRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPULKRF` reader - the CPULK Reset Flag Is Set by Hardware If Cortex-m4 Lockup Happened (M45xD/M45xC Only)\\nNote: Write 1 to clear this bit to 0."]
pub struct CPULKRF_R(crate::FieldReader<bool, CPULKRF_A>);
impl CPULKRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPULKRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPULKRF_A {
        match self.bits {
            false => CPULKRF_A::_0,
            true => CPULKRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPULKRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPULKRF_A::_1
    }
}
impl core::ops::Deref for CPULKRF_R {
    type Target = crate::FieldReader<bool, CPULKRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPULKRF` writer - the CPULK Reset Flag Is Set by Hardware If Cortex-m4 Lockup Happened (M45xD/M45xC Only)\\nNote: Write 1 to clear this bit to 0."]
pub struct CPULKRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPULKRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPULKRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset from CPU lockup happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPULKRF_A::_0)
    }
    #[doc = "The Cortex-M4 lockup happened and chip is reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPULKRF_A::_1)
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
impl R {
    #[doc = "Bit 0 - POR Reset Flag\\nThe POR reset flag is set by the 'Reset Signal' from the Power-on Reset (POR) Controller or bit CHIPRST (SYS_IPRST0\\[0\\]) to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NRESET Pin Reset Flag\\nThe nRESET pin reset flag is set by the 'Reset Signal' from the nRESET Pin to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn pinrf(&self) -> PINRF_R {
        PINRF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDT Reset Flag\\nThe WDT reset flag is set by the 'Reset Signal' from the Watchdog Timer or Window Watchdog Timer to indicate the previous reset source.\\nNote1: Write 1 to clear this bit to 0.\\nNote2: Watchdog Timer register RSTF(WDT_CTL\\[2\\]) bit is set if the system has been reset by WDT time-out reset. Window Watchdog Timer register WWDTRF(WWDT_STATUS\\[1\\]) bit is set if the system has been reset by WWDT time-out reset."]
    #[inline(always)]
    pub fn wdtrf(&self) -> WDTRF_R {
        WDTRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LVR Reset Flag\\nThe LVR reset flag is set by the 'Reset Signal' from the Low Voltage Reset Controller to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn lvrf(&self) -> LVRF_R {
        LVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOD Reset Flag\\nThe BOD reset flag is set by the 'Reset Signal' from the Brown-Out Detector to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn bodrf(&self) -> BODRF_R {
        BODRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Reset Flag\\nThe system reset flag is set by the 'Reset Signal' from the Cortex-M4 Core to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn sysrf(&self) -> SYSRF_R {
        SYSRF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU Reset Flag\\nThe CPU reset flag is set by hardware if software writes CPURST (SYS_IPRST0\\[1\\]) 1 to reset Cortex-M4 Core and Flash Memory Controller (FMC).\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn cpurf(&self) -> CPURF_R {
        CPURF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - the CPULK Reset Flag Is Set by Hardware If Cortex-m4 Lockup Happened (M45xD/M45xC Only)\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn cpulkrf(&self) -> CPULKRF_R {
        CPULKRF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POR Reset Flag\\nThe POR reset flag is set by the 'Reset Signal' from the Power-on Reset (POR) Controller or bit CHIPRST (SYS_IPRST0\\[0\\]) to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn porf(&mut self) -> PORF_W {
        PORF_W { w: self }
    }
    #[doc = "Bit 1 - NRESET Pin Reset Flag\\nThe nRESET pin reset flag is set by the 'Reset Signal' from the nRESET Pin to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn pinrf(&mut self) -> PINRF_W {
        PINRF_W { w: self }
    }
    #[doc = "Bit 2 - WDT Reset Flag\\nThe WDT reset flag is set by the 'Reset Signal' from the Watchdog Timer or Window Watchdog Timer to indicate the previous reset source.\\nNote1: Write 1 to clear this bit to 0.\\nNote2: Watchdog Timer register RSTF(WDT_CTL\\[2\\]) bit is set if the system has been reset by WDT time-out reset. Window Watchdog Timer register WWDTRF(WWDT_STATUS\\[1\\]) bit is set if the system has been reset by WWDT time-out reset."]
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WDTRF_W {
        WDTRF_W { w: self }
    }
    #[doc = "Bit 3 - LVR Reset Flag\\nThe LVR reset flag is set by the 'Reset Signal' from the Low Voltage Reset Controller to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn lvrf(&mut self) -> LVRF_W {
        LVRF_W { w: self }
    }
    #[doc = "Bit 4 - BOD Reset Flag\\nThe BOD reset flag is set by the 'Reset Signal' from the Brown-Out Detector to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn bodrf(&mut self) -> BODRF_W {
        BODRF_W { w: self }
    }
    #[doc = "Bit 5 - System Reset Flag\\nThe system reset flag is set by the 'Reset Signal' from the Cortex-M4 Core to indicate the previous reset source.\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn sysrf(&mut self) -> SYSRF_W {
        SYSRF_W { w: self }
    }
    #[doc = "Bit 7 - CPU Reset Flag\\nThe CPU reset flag is set by hardware if software writes CPURST (SYS_IPRST0\\[1\\]) 1 to reset Cortex-M4 Core and Flash Memory Controller (FMC).\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn cpurf(&mut self) -> CPURF_W {
        CPURF_W { w: self }
    }
    #[doc = "Bit 8 - the CPULK Reset Flag Is Set by Hardware If Cortex-m4 Lockup Happened (M45xD/M45xC Only)\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn cpulkrf(&mut self) -> CPULKRF_W {
        CPULKRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_rststs](index.html) module"]
pub struct SYS_RSTSTS_SPEC;
impl crate::RegisterSpec for SYS_RSTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_rststs::R](R) reader structure"]
impl crate::Readable for SYS_RSTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_rststs::W](W) writer structure"]
impl crate::Writable for SYS_RSTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_RSTSTS to value 0x43"]
impl crate::Resettable for SYS_RSTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x43
    }
}
