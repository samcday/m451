#[doc = "Register `NMISTS` reader"]
pub struct R(crate::R<NMISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NMISTS_SPEC>> for R {
    fn from(reader: crate::R<NMISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BOD Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODOUT_A {
    #[doc = "0: BOD interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: BOD interrupt is asserted"]
    _1 = 1,
}
impl From<BODOUT_A> for bool {
    #[inline(always)]
    fn from(variant: BODOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODOUT` reader - BOD Interrupt Flag (Read Only)"]
pub struct BODOUT_R(crate::FieldReader<bool, BODOUT_A>);
impl BODOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODOUT_A {
        match self.bits {
            false => BODOUT_A::_0,
            true => BODOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODOUT_A::_1
    }
}
impl core::ops::Deref for BODOUT_R {
    type Target = crate::FieldReader<bool, BODOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "IRC TRIM Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_INT_A {
    #[doc = "0: HIRC TRIM interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: HIRC TRIM interrupt is asserted"]
    _1 = 1,
}
impl From<IRC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC_INT` reader - IRC TRIM Interrupt Flag (Read Only)"]
pub struct IRC_INT_R(crate::FieldReader<bool, IRC_INT_A>);
impl IRC_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRC_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC_INT_A {
        match self.bits {
            false => IRC_INT_A::_0,
            true => IRC_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRC_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRC_INT_A::_1
    }
}
impl core::ops::Deref for IRC_INT_R {
    type Target = crate::FieldReader<bool, IRC_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Power-down Mode Wake-up Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRWU_INT_A {
    #[doc = "0: Power-down mode wake-up interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: Power-down mode wake-up interrupt is asserted"]
    _1 = 1,
}
impl From<PWRWU_INT_A> for bool {
    #[inline(always)]
    fn from(variant: PWRWU_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRWU_INT` reader - Power-down Mode Wake-up Interrupt Flag (Read Only)"]
pub struct PWRWU_INT_R(crate::FieldReader<bool, PWRWU_INT_A>);
impl PWRWU_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRWU_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRWU_INT_A {
        match self.bits {
            false => PWRWU_INT_A::_0,
            true => PWRWU_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWRWU_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWRWU_INT_A::_1
    }
}
impl core::ops::Deref for PWRWU_INT_R {
    type Target = crate::FieldReader<bool, PWRWU_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM ParityCheck Error Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PERR_A {
    #[doc = "0: SRAM parity check error interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: SRAM parity check error interrupt is asserted"]
    _1 = 1,
}
impl From<SRAM_PERR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PERR` reader - SRAM ParityCheck Error Interrupt Flag (Read Only)"]
pub struct SRAM_PERR_R(crate::FieldReader<bool, SRAM_PERR_A>);
impl SRAM_PERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PERR_A {
        match self.bits {
            false => SRAM_PERR_A::_0,
            true => SRAM_PERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRAM_PERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRAM_PERR_A::_1
    }
}
impl core::ops::Deref for SRAM_PERR_R {
    type Target = crate::FieldReader<bool, SRAM_PERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clock Fail Detected Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKFAIL_A {
    #[doc = "0: Clock fail detected interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: Clock fail detected interrupt is asserted"]
    _1 = 1,
}
impl From<CLKFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKFAIL` reader - Clock Fail Detected Interrupt Flag (Read Only)"]
pub struct CLKFAIL_R(crate::FieldReader<bool, CLKFAIL_A>);
impl CLKFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKFAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKFAIL_A {
        match self.bits {
            false => CLKFAIL_A::_0,
            true => CLKFAIL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKFAIL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKFAIL_A::_1
    }
}
impl core::ops::Deref for CLKFAIL_R {
    type Target = crate::FieldReader<bool, CLKFAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_INT_A {
    #[doc = "0: RTC interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: RTC interrupt is asserted"]
    _1 = 1,
}
impl From<RTC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_INT` reader - RTC Interrupt Flag (Read Only)"]
pub struct RTC_INT_R(crate::FieldReader<bool, RTC_INT_A>);
impl RTC_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_INT_A {
        match self.bits {
            false => RTC_INT_A::_0,
            true => RTC_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTC_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTC_INT_A::_1
    }
}
impl core::ops::Deref for RTC_INT_R {
    type Target = crate::FieldReader<bool, RTC_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TAMPER_INT Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPER_INT_A {
    #[doc = "0: Backup register tamper detected interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: Backup register tamper detected interrupt is asserted"]
    _1 = 1,
}
impl From<TAMPER_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPER_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER_INT` reader - TAMPER_INT Interrupt Flag (Read Only)"]
pub struct TAMPER_INT_R(crate::FieldReader<bool, TAMPER_INT_A>);
impl TAMPER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPER_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPER_INT_A {
        match self.bits {
            false => TAMPER_INT_A::_0,
            true => TAMPER_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TAMPER_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TAMPER_INT_A::_1
    }
}
impl core::ops::Deref for TAMPER_INT_R {
    type Target = crate::FieldReader<bool, TAMPER_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PA.0, PD.2 or PE.4 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT0_A {
    #[doc = "0: External Interrupt from PA.0, PD.2 or PE.4 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PA.0, PD.2 or PE.4 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT0` reader - External Interrupt From PA.0, PD.2 or PE.4 Pin Interrupt Flag (Read Only)"]
pub struct EINT0_R(crate::FieldReader<bool, EINT0_A>);
impl EINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::_0,
            true => EINT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT0_A::_1
    }
}
impl core::ops::Deref for EINT0_R {
    type Target = crate::FieldReader<bool, EINT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PB.0, PD.3 or PE.5 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT1_A {
    #[doc = "0: External Interrupt from PB.0, PD.3 or PE.5 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PB.0, PD.3 or PE.5 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT1` reader - External Interrupt From PB.0, PD.3 or PE.5 Pin Interrupt Flag (Read Only)"]
pub struct EINT1_R(crate::FieldReader<bool, EINT1_A>);
impl EINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::_0,
            true => EINT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT1_A::_1
    }
}
impl core::ops::Deref for EINT1_R {
    type Target = crate::FieldReader<bool, EINT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PC.0 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT2_A {
    #[doc = "0: External Interrupt from PC.0 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PC.0 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT2_A> for bool {
    #[inline(always)]
    fn from(variant: EINT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT2` reader - External Interrupt From PC.0 Pin Interrupt Flag (Read Only)"]
pub struct EINT2_R(crate::FieldReader<bool, EINT2_A>);
impl EINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT2_A {
        match self.bits {
            false => EINT2_A::_0,
            true => EINT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT2_A::_1
    }
}
impl core::ops::Deref for EINT2_R {
    type Target = crate::FieldReader<bool, EINT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PD.0 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT3_A {
    #[doc = "0: External Interrupt from PD.0 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PD.0 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT3_A> for bool {
    #[inline(always)]
    fn from(variant: EINT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT3` reader - External Interrupt From PD.0 Pin Interrupt Flag (Read Only)"]
pub struct EINT3_R(crate::FieldReader<bool, EINT3_A>);
impl EINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT3_A {
        match self.bits {
            false => EINT3_A::_0,
            true => EINT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT3_A::_1
    }
}
impl core::ops::Deref for EINT3_R {
    type Target = crate::FieldReader<bool, EINT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PE.0 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT4_A {
    #[doc = "0: External Interrupt from PE.0 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PE.0 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT4_A> for bool {
    #[inline(always)]
    fn from(variant: EINT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT4` reader - External Interrupt From PE.0 Pin Interrupt Flag (Read Only)"]
pub struct EINT4_R(crate::FieldReader<bool, EINT4_A>);
impl EINT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT4_A {
        match self.bits {
            false => EINT4_A::_0,
            true => EINT4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT4_A::_1
    }
}
impl core::ops::Deref for EINT4_R {
    type Target = crate::FieldReader<bool, EINT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Interrupt From PF.0 Pin Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT5_A {
    #[doc = "0: External Interrupt from PF.0 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: External Interrupt from PF.0 interrupt is asserted"]
    _1 = 1,
}
impl From<EINT5_A> for bool {
    #[inline(always)]
    fn from(variant: EINT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT5` reader - External Interrupt From PF.0 Pin Interrupt Flag (Read Only)"]
pub struct EINT5_R(crate::FieldReader<bool, EINT5_A>);
impl EINT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT5_A {
        match self.bits {
            false => EINT5_A::_0,
            true => EINT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EINT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EINT5_A::_1
    }
}
impl core::ops::Deref for EINT5_R {
    type Target = crate::FieldReader<bool, EINT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART0 Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_INT_A {
    #[doc = "0: UART1 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: UART1 interrupt is asserted"]
    _1 = 1,
}
impl From<UART0_INT_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0_INT` reader - UART0 Interrupt Flag (Read Only)"]
pub struct UART0_INT_R(crate::FieldReader<bool, UART0_INT_A>);
impl UART0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_INT_A {
        match self.bits {
            false => UART0_INT_A::_0,
            true => UART0_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0_INT_A::_1
    }
}
impl core::ops::Deref for UART0_INT_R {
    type Target = crate::FieldReader<bool, UART0_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART1 Interrupt Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_INT_A {
    #[doc = "0: UART1 interrupt is deasserted"]
    _0 = 0,
    #[doc = "1: UART1 interrupt is asserted"]
    _1 = 1,
}
impl From<UART1_INT_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1_INT` reader - UART1 Interrupt Flag (Read Only)"]
pub struct UART1_INT_R(crate::FieldReader<bool, UART1_INT_A>);
impl UART1_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_INT_A {
        match self.bits {
            false => UART1_INT_A::_0,
            true => UART1_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART1_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART1_INT_A::_1
    }
}
impl core::ops::Deref for UART1_INT_R {
    type Target = crate::FieldReader<bool, UART1_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - BOD Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn bodout(&self) -> BODOUT_R {
        BODOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC TRIM Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn irc_int(&self) -> IRC_INT_R {
        IRC_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power-down Mode Wake-up Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn pwrwu_int(&self) -> PWRWU_INT_R {
        PWRWU_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRAM ParityCheck Error Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn sram_perr(&self) -> SRAM_PERR_R {
        SRAM_PERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Fail Detected Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn clkfail(&self) -> CLKFAIL_R {
        CLKFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn rtc_int(&self) -> RTC_INT_R {
        RTC_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMPER_INT Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn tamper_int(&self) -> TAMPER_INT_R {
        TAMPER_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt From PA.0, PD.2 or PE.4 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt From PB.0, PD.3 or PE.5 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt From PC.0 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint2(&self) -> EINT2_R {
        EINT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt From PD.0 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint3(&self) -> EINT3_R {
        EINT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt From PE.0 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint4(&self) -> EINT4_R {
        EINT4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt From PF.0 Pin Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn eint5(&self) -> EINT5_R {
        EINT5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - UART0 Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn uart0_int(&self) -> UART0_INT_R {
        UART0_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UART1 Interrupt Flag (Read Only)"]
    #[inline(always)]
    pub fn uart1_int(&self) -> UART1_INT_R {
        UART1_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "NMI Source Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmists](index.html) module"]
pub struct NMISTS_SPEC;
impl crate::RegisterSpec for NMISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmists::R](R) reader structure"]
impl crate::Readable for NMISTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NMISTS to value 0"]
impl crate::Resettable for NMISTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
