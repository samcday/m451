#[doc = "Register `NMIEN` reader"]
pub struct R(crate::R<NMIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NMIEN_SPEC>> for R {
    fn from(reader: crate::R<NMIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIEN` writer"]
pub struct W(crate::W<NMIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIEN_SPEC>;
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
impl core::convert::From<crate::W<NMIEN_SPEC>> for W {
    fn from(writer: crate::W<NMIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BOD NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODOUT_A {
    #[doc = "0: BOD NMI source Disabled"]
    _0 = 0,
    #[doc = "1: BOD NMI source Enabled"]
    _1 = 1,
}
impl From<BODOUT_A> for bool {
    #[inline(always)]
    fn from(variant: BODOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODOUT` reader - BOD NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `BODOUT` writer - BOD NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BOD NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODOUT_A::_0)
    }
    #[doc = "BOD NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODOUT_A::_1)
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
#[doc = "IRC TRIM NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_INT_A {
    #[doc = "0: IRC TRIM NMI source Disabled"]
    _0 = 0,
    #[doc = "1: IRC TRIM NMI source Enabled"]
    _1 = 1,
}
impl From<IRC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC_INT` reader - IRC TRIM NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `IRC_INT` writer - IRC TRIM NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct IRC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRC_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRC TRIM NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRC_INT_A::_0)
    }
    #[doc = "IRC TRIM NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRC_INT_A::_1)
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
#[doc = "Power-down Mode Wake-up NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRWU_INT_A {
    #[doc = "0: Power-down mode wake-up NMI source Disabled"]
    _0 = 0,
    #[doc = "1: Power-down mode wake-up NMI source Enabled"]
    _1 = 1,
}
impl From<PWRWU_INT_A> for bool {
    #[inline(always)]
    fn from(variant: PWRWU_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRWU_INT` reader - Power-down Mode Wake-up NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `PWRWU_INT` writer - Power-down Mode Wake-up NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PWRWU_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRWU_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRWU_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power-down mode wake-up NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWRWU_INT_A::_0)
    }
    #[doc = "Power-down mode wake-up NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWRWU_INT_A::_1)
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
#[doc = "SRAM ParityCheck Error NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PERR_A {
    #[doc = "0: SRAM parity check error NMI source Disabled"]
    _0 = 0,
    #[doc = "1: SRAM parity check error NMI source Enabled"]
    _1 = 1,
}
impl From<SRAM_PERR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PERR` reader - SRAM ParityCheck Error NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `SRAM_PERR` writer - SRAM ParityCheck Error NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRAM_PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_PERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM parity check error NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAM_PERR_A::_0)
    }
    #[doc = "SRAM parity check error NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAM_PERR_A::_1)
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
#[doc = "Clock Fail Detected NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKFAIL_A {
    #[doc = "0: Clock fail detected interrupt NMI source Disabled"]
    _0 = 0,
    #[doc = "1: Clock fail detected interrupt NMI source Enabled"]
    _1 = 1,
}
impl From<CLKFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKFAIL` reader - Clock Fail Detected NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `CLKFAIL` writer - Clock Fail Detected NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CLKFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKFAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKFAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock fail detected interrupt NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKFAIL_A::_0)
    }
    #[doc = "Clock fail detected interrupt NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKFAIL_A::_1)
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
#[doc = "RTC NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_INT_A {
    #[doc = "0: RTC NMI source Disabled"]
    _0 = 0,
    #[doc = "1: RTC NMI source Enabled"]
    _1 = 1,
}
impl From<RTC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_INT` reader - RTC NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `RTC_INT` writer - RTC NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct RTC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_INT_A::_0)
    }
    #[doc = "RTC NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_INT_A::_1)
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
#[doc = "TAMPER_INT NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPER_INT_A {
    #[doc = "0: Backup register tamper detected interrupt.NMI source Disabled"]
    _0 = 0,
    #[doc = "1: Backup register tamper detected interrupt.NMI source Enabled"]
    _1 = 1,
}
impl From<TAMPER_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPER_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPER_INT` reader - TAMPER_INT NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `TAMPER_INT` writer - TAMPER_INT NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct TAMPER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPER_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPER_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Backup register tamper detected interrupt.NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAMPER_INT_A::_0)
    }
    #[doc = "Backup register tamper detected interrupt.NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAMPER_INT_A::_1)
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
#[doc = "External Interrupt From PA.0, PD.2 or PE.4 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT0_A {
    #[doc = "0: External interrupt from PA.0, PD.2 or PE.4 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PA.0, PD.2 or PE.4 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT0` reader - External Interrupt From PA.0, PD.2 or PE.4 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT0` writer - External Interrupt From PA.0, PD.2 or PE.4 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PA.0, PD.2 or PE.4 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT0_A::_0)
    }
    #[doc = "External interrupt from PA.0, PD.2 or PE.4 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT0_A::_1)
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
#[doc = "External Interrupt From PB.0, PD.3 or PE.5 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT1_A {
    #[doc = "0: External interrupt from PB.0, PD.3 or PE.5 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PB.0, PD.3 or PE.5 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT1` reader - External Interrupt From PB.0, PD.3 or PE.5 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT1` writer - External Interrupt From PB.0, PD.3 or PE.5 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PB.0, PD.3 or PE.5 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT1_A::_0)
    }
    #[doc = "External interrupt from PB.0, PD.3 or PE.5 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT1_A::_1)
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
#[doc = "External Interrupt From PC.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT2_A {
    #[doc = "0: External interrupt from PC.0 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PC.0 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT2_A> for bool {
    #[inline(always)]
    fn from(variant: EINT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT2` reader - External Interrupt From PC.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT2` writer - External Interrupt From PC.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PC.0 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT2_A::_0)
    }
    #[doc = "External interrupt from PC.0 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "External Interrupt From PD.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT3_A {
    #[doc = "0: External interrupt from PD.0 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PD.0 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT3_A> for bool {
    #[inline(always)]
    fn from(variant: EINT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT3` reader - External Interrupt From PD.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT3` writer - External Interrupt From PD.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PD.0 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT3_A::_0)
    }
    #[doc = "External interrupt from PD.0 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT3_A::_1)
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
#[doc = "External Interrupt From PE.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT4_A {
    #[doc = "0: External interrupt from PE.0 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PE.0 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT4_A> for bool {
    #[inline(always)]
    fn from(variant: EINT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT4` reader - External Interrupt From PE.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT4` writer - External Interrupt From PE.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PE.0 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT4_A::_0)
    }
    #[doc = "External interrupt from PE.0 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT4_A::_1)
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
#[doc = "External Interrupt From PF.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT5_A {
    #[doc = "0: External interrupt from PF.0 pin NMI source Disabled"]
    _0 = 0,
    #[doc = "1: External interrupt from PF.0 pin NMI source Enabled"]
    _1 = 1,
}
impl From<EINT5_A> for bool {
    #[inline(always)]
    fn from(variant: EINT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT5` reader - External Interrupt From PF.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `EINT5` writer - External Interrupt From PF.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct EINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External interrupt from PF.0 pin NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EINT5_A::_0)
    }
    #[doc = "External interrupt from PF.0 pin NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EINT5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "UART0 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_INT_A {
    #[doc = "0: UART0 NMI source Disabled"]
    _0 = 0,
    #[doc = "1: UART0 NMI source Enabled"]
    _1 = 1,
}
impl From<UART0_INT_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0_INT` reader - UART0 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `UART0_INT` writer - UART0 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct UART0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART0 NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0_INT_A::_0)
    }
    #[doc = "UART0 NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0_INT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "UART1 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_INT_A {
    #[doc = "0: UART1 NMI source Disabled"]
    _0 = 0,
    #[doc = "1: UART1 NMI source Enabled"]
    _1 = 1,
}
impl From<UART1_INT_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1_INT` reader - UART1 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
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
#[doc = "Field `UART1_INT` writer - UART1 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct UART1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART1 NMI source Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1_INT_A::_0)
    }
    #[doc = "UART1 NMI source Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1_INT_A::_1)
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
impl R {
    #[doc = "Bit 0 - BOD NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodout(&self) -> BODOUT_R {
        BODOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC TRIM NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn irc_int(&self) -> IRC_INT_R {
        IRC_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power-down Mode Wake-up NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pwrwu_int(&self) -> PWRWU_INT_R {
        PWRWU_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRAM ParityCheck Error NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn sram_perr(&self) -> SRAM_PERR_R {
        SRAM_PERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Fail Detected NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn clkfail(&self) -> CLKFAIL_R {
        CLKFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn rtc_int(&self) -> RTC_INT_R {
        RTC_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMPER_INT NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn tamper_int(&self) -> TAMPER_INT_R {
        TAMPER_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt From PA.0, PD.2 or PE.4 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt From PB.0, PD.3 or PE.5 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt From PC.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint2(&self) -> EINT2_R {
        EINT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt From PD.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint3(&self) -> EINT3_R {
        EINT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt From PE.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint4(&self) -> EINT4_R {
        EINT4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt From PF.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint5(&self) -> EINT5_R {
        EINT5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - UART0 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn uart0_int(&self) -> UART0_INT_R {
        UART0_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UART1 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn uart1_int(&self) -> UART1_INT_R {
        UART1_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodout(&mut self) -> BODOUT_W {
        BODOUT_W { w: self }
    }
    #[doc = "Bit 1 - IRC TRIM NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn irc_int(&mut self) -> IRC_INT_W {
        IRC_INT_W { w: self }
    }
    #[doc = "Bit 2 - Power-down Mode Wake-up NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pwrwu_int(&mut self) -> PWRWU_INT_W {
        PWRWU_INT_W { w: self }
    }
    #[doc = "Bit 3 - SRAM ParityCheck Error NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn sram_perr(&mut self) -> SRAM_PERR_W {
        SRAM_PERR_W { w: self }
    }
    #[doc = "Bit 4 - Clock Fail Detected NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn clkfail(&mut self) -> CLKFAIL_W {
        CLKFAIL_W { w: self }
    }
    #[doc = "Bit 6 - RTC NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn rtc_int(&mut self) -> RTC_INT_W {
        RTC_INT_W { w: self }
    }
    #[doc = "Bit 7 - TAMPER_INT NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn tamper_int(&mut self) -> TAMPER_INT_W {
        TAMPER_INT_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt From PA.0, PD.2 or PE.4 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W {
        EINT0_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt From PB.0, PD.3 or PE.5 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W {
        EINT1_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt From PC.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint2(&mut self) -> EINT2_W {
        EINT2_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt From PD.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint3(&mut self) -> EINT3_W {
        EINT3_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt From PE.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint4(&mut self) -> EINT4_W {
        EINT4_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt From PF.0 Pin NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn eint5(&mut self) -> EINT5_W {
        EINT5_W { w: self }
    }
    #[doc = "Bit 14 - UART0 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn uart0_int(&mut self) -> UART0_INT_W {
        UART0_INT_W { w: self }
    }
    #[doc = "Bit 15 - UART1 NMI Source Enable (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn uart1_int(&mut self) -> UART1_INT_W {
        UART1_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Source Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmien](index.html) module"]
pub struct NMIEN_SPEC;
impl crate::RegisterSpec for NMIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmien::R](R) reader structure"]
impl crate::Readable for NMIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmien::W](W) writer structure"]
impl crate::Writable for NMIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIEN to value 0"]
impl crate::Resettable for NMIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
