#[doc = "Register `WDT_CTL` reader"]
pub struct R(crate::R<WDT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDT_CTL_SPEC>> for R {
    fn from(reader: crate::R<WDT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CTL` writer"]
pub struct W(crate::W<WDT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CTL_SPEC>;
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
impl core::convert::From<crate::W<WDT_CTL_SPEC>> for W {
    fn from(writer: crate::W<WDT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset WDT Up Counter (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit will be automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCNT_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the internal 18-bit WDT up counter value"]
    _1 = 1,
}
impl From<RSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCNT` reader - Reset WDT Up Counter (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit will be automatically cleared by hardware."]
pub struct RSTCNT_R(crate::FieldReader<bool, RSTCNT_A>);
impl RSTCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCNT_A {
        match self.bits {
            false => RSTCNT_A::_0,
            true => RSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTCNT_A::_1
    }
}
impl core::ops::Deref for RSTCNT_R {
    type Target = crate::FieldReader<bool, RSTCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCNT` writer - Reset WDT Up Counter (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit will be automatically cleared by hardware."]
pub struct RSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTCNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTCNT_A::_0)
    }
    #[doc = "Reset the internal 18-bit WDT up counter value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTCNT_A::_1)
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
#[doc = "WDT Time-out Reset Enable Control (Write Protect)\\nSetting this bit will enable the WDT time-out reset function If the WDT up counter value has not been cleared after the specific WDT reset delay period expires.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTEN_A {
    #[doc = "0: WDT time-out reset function Disabled"]
    _0 = 0,
    #[doc = "1: WDT time-out reset function Enabled"]
    _1 = 1,
}
impl From<RSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTEN` reader - WDT Time-out Reset Enable Control (Write Protect)\\nSetting this bit will enable the WDT time-out reset function If the WDT up counter value has not been cleared after the specific WDT reset delay period expires.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct RSTEN_R(crate::FieldReader<bool, RSTEN_A>);
impl RSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTEN_A {
        match self.bits {
            false => RSTEN_A::_0,
            true => RSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTEN_A::_1
    }
}
impl core::ops::Deref for RSTEN_R {
    type Target = crate::FieldReader<bool, RSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTEN` writer - WDT Time-out Reset Enable Control (Write Protect)\\nSetting this bit will enable the WDT time-out reset function If the WDT up counter value has not been cleared after the specific WDT reset delay period expires.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT time-out reset function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTEN_A::_0)
    }
    #[doc = "WDT time-out reset function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTEN_A::_1)
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
#[doc = "WDT Time-out Reset Flag\\nThis bit indicates the system has been reset by WDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTF_A {
    #[doc = "0: WDT time-out reset did not occur"]
    _0 = 0,
    #[doc = "1: WDT time-out reset occurred"]
    _1 = 1,
}
impl From<RSTF_A> for bool {
    #[inline(always)]
    fn from(variant: RSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTF` reader - WDT Time-out Reset Flag\\nThis bit indicates the system has been reset by WDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
pub struct RSTF_R(crate::FieldReader<bool, RSTF_A>);
impl RSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTF_A {
        match self.bits {
            false => RSTF_A::_0,
            true => RSTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTF_A::_1
    }
}
impl core::ops::Deref for RSTF_R {
    type Target = crate::FieldReader<bool, RSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTF` writer - WDT Time-out Reset Flag\\nThis bit indicates the system has been reset by WDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
pub struct RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT time-out reset did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTF_A::_0)
    }
    #[doc = "WDT time-out reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTF_A::_1)
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
#[doc = "WDT Time-out Interrupt Flag\\nThis bit will set to 1 while WDT up counter value reaches the selected WDT time-out interval\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF_A {
    #[doc = "0: WDT time-out interrupt did not occur"]
    _0 = 0,
    #[doc = "1: WDT time-out interrupt occurred"]
    _1 = 1,
}
impl From<IF_A> for bool {
    #[inline(always)]
    fn from(variant: IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF` reader - WDT Time-out Interrupt Flag\\nThis bit will set to 1 while WDT up counter value reaches the selected WDT time-out interval\\nNote: This bit is cleared by writing 1 to it."]
pub struct IF_R(crate::FieldReader<bool, IF_A>);
impl IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF_A {
        match self.bits {
            false => IF_A::_0,
            true => IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IF_A::_1
    }
}
impl core::ops::Deref for IF_R {
    type Target = crate::FieldReader<bool, IF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF` writer - WDT Time-out Interrupt Flag\\nThis bit will set to 1 while WDT up counter value reaches the selected WDT time-out interval\\nNote: This bit is cleared by writing 1 to it."]
pub struct IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT time-out interrupt did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IF_A::_0)
    }
    #[doc = "WDT time-out interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IF_A::_1)
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
#[doc = "WDT Time-out Wake-up Function Control (Write Protect)\\nIf this bit is set to 1, while WDT time-out interrupt flag IF (WDT_CTL\\[3\\]) is generated to 1 and interrupt enable bit INTEN (WDT_CTL\\[6\\]) is enabled, the WDT time-out interrupt signal will generate a wake-up trigger event to chip.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Chip can be woken-up by WDT time-out interrupt signal generated only if WDT clock source is selected to LIRC or LXT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKEN_A {
    #[doc = "0: Wake-up trigger event Disabled if WDT time-out interrupt signal generated"]
    _0 = 0,
    #[doc = "1: Wake-up trigger event Enabled if WDT time-out interrupt signal generated"]
    _1 = 1,
}
impl From<WKEN_A> for bool {
    #[inline(always)]
    fn from(variant: WKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKEN` reader - WDT Time-out Wake-up Function Control (Write Protect)\\nIf this bit is set to 1, while WDT time-out interrupt flag IF (WDT_CTL\\[3\\]) is generated to 1 and interrupt enable bit INTEN (WDT_CTL\\[6\\]) is enabled, the WDT time-out interrupt signal will generate a wake-up trigger event to chip.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Chip can be woken-up by WDT time-out interrupt signal generated only if WDT clock source is selected to LIRC or LXT."]
pub struct WKEN_R(crate::FieldReader<bool, WKEN_A>);
impl WKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKEN_A {
        match self.bits {
            false => WKEN_A::_0,
            true => WKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKEN_A::_1
    }
}
impl core::ops::Deref for WKEN_R {
    type Target = crate::FieldReader<bool, WKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKEN` writer - WDT Time-out Wake-up Function Control (Write Protect)\\nIf this bit is set to 1, while WDT time-out interrupt flag IF (WDT_CTL\\[3\\]) is generated to 1 and interrupt enable bit INTEN (WDT_CTL\\[6\\]) is enabled, the WDT time-out interrupt signal will generate a wake-up trigger event to chip.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Chip can be woken-up by WDT time-out interrupt signal generated only if WDT clock source is selected to LIRC or LXT."]
pub struct WKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake-up trigger event Disabled if WDT time-out interrupt signal generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKEN_A::_0)
    }
    #[doc = "Wake-up trigger event Enabled if WDT time-out interrupt signal generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKEN_A::_1)
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
#[doc = "WDT Time-out Wake-up Flag (Write Protect)\\nThis bit indicates the interrupt wake-up flag status of WDT\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKF_A {
    #[doc = "0: WDT does not cause chip wake-up"]
    _0 = 0,
    #[doc = "1: Chip wake-up from Idle or Power-down mode if WDT time-out interrupt signal generated"]
    _1 = 1,
}
impl From<WKF_A> for bool {
    #[inline(always)]
    fn from(variant: WKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKF` reader - WDT Time-out Wake-up Flag (Write Protect)\\nThis bit indicates the interrupt wake-up flag status of WDT\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit is cleared by writing 1 to it."]
pub struct WKF_R(crate::FieldReader<bool, WKF_A>);
impl WKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKF_A {
        match self.bits {
            false => WKF_A::_0,
            true => WKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WKF_A::_1
    }
}
impl core::ops::Deref for WKF_R {
    type Target = crate::FieldReader<bool, WKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKF` writer - WDT Time-out Wake-up Flag (Write Protect)\\nThis bit indicates the interrupt wake-up flag status of WDT\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit is cleared by writing 1 to it."]
pub struct WKF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT does not cause chip wake-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WKF_A::_0)
    }
    #[doc = "Chip wake-up from Idle or Power-down mode if WDT time-out interrupt signal generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WKF_A::_1)
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
#[doc = "WDT Time-out Interrupt Enable Control (Write Protect)\\nIf this bit is enabled, the WDT time-out interrupt signal is generated and inform to CPU. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: WDT time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: WDT time-out interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - WDT Time-out Interrupt Enable Control (Write Protect)\\nIf this bit is enabled, the WDT time-out interrupt signal is generated and inform to CPU. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::_0,
            true => INTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN_A::_1
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - WDT Time-out Interrupt Enable Control (Write Protect)\\nIf this bit is enabled, the WDT time-out interrupt signal is generated and inform to CPU. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
    #[doc = "WDT time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
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
#[doc = "WDT Enable Control (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: If CWDTEN\\[2:0\\]
(combined by Config0\\[31\\]
and Config0\\[4:3\\]) bits is not configure to 111, this bit is forced as 1 and user cannot change this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTEN_A {
    #[doc = "0: WDT Disabled (This action will reset the internal up counter value)"]
    _0 = 0,
    #[doc = "1: WDT Enabled"]
    _1 = 1,
}
impl From<WDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTEN` reader - WDT Enable Control (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: If CWDTEN\\[2:0\\]
(combined by Config0\\[31\\]
and Config0\\[4:3\\]) bits is not configure to 111, this bit is forced as 1 and user cannot change this bit to 0."]
pub struct WDTEN_R(crate::FieldReader<bool, WDTEN_A>);
impl WDTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTEN_A {
        match self.bits {
            false => WDTEN_A::_0,
            true => WDTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDTEN_A::_1
    }
}
impl core::ops::Deref for WDTEN_R {
    type Target = crate::FieldReader<bool, WDTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTEN` writer - WDT Enable Control (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: If CWDTEN\\[2:0\\]
(combined by Config0\\[31\\]
and Config0\\[4:3\\]) bits is not configure to 111, this bit is forced as 1 and user cannot change this bit to 0."]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT Disabled (This action will reset the internal up counter value)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTEN_A::_0)
    }
    #[doc = "WDT Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTEN_A::_1)
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
#[doc = "WDT Time-out Interval Selection (Write Protect)\\nThese three bits select the time-out interval period for the WDT.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOUTSEL_A {
    #[doc = "0: 24 * WDT_CLK"]
    _0 = 0,
    #[doc = "1: 26 * WDT_CLK"]
    _1 = 1,
    #[doc = "2: 28 * WDT_CLK"]
    _2 = 2,
    #[doc = "3: 210 * WDT_CLK"]
    _3 = 3,
    #[doc = "4: 212 * WDT_CLK"]
    _4 = 4,
    #[doc = "5: 214 * WDT_CLK"]
    _5 = 5,
    #[doc = "6: 216 * WDT_CLK"]
    _6 = 6,
    #[doc = "7: 218 * WDT_CLK"]
    _7 = 7,
}
impl From<TOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOUTSEL` reader - WDT Time-out Interval Selection (Write Protect)\\nThese three bits select the time-out interval period for the WDT.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct TOUTSEL_R(crate::FieldReader<u8, TOUTSEL_A>);
impl TOUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUTSEL_A {
        match self.bits {
            0 => TOUTSEL_A::_0,
            1 => TOUTSEL_A::_1,
            2 => TOUTSEL_A::_2,
            3 => TOUTSEL_A::_3,
            4 => TOUTSEL_A::_4,
            5 => TOUTSEL_A::_5,
            6 => TOUTSEL_A::_6,
            7 => TOUTSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOUTSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TOUTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TOUTSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TOUTSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TOUTSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TOUTSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TOUTSEL_A::_7
    }
}
impl core::ops::Deref for TOUTSEL_R {
    type Target = crate::FieldReader<u8, TOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUTSEL` writer - WDT Time-out Interval Selection (Write Protect)\\nThese three bits select the time-out interval period for the WDT.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct TOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "24 * WDT_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_0)
    }
    #[doc = "26 * WDT_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_1)
    }
    #[doc = "28 * WDT_CLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_2)
    }
    #[doc = "210 * WDT_CLK"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_3)
    }
    #[doc = "212 * WDT_CLK"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_4)
    }
    #[doc = "214 * WDT_CLK"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_5)
    }
    #[doc = "216 * WDT_CLK"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_6)
    }
    #[doc = "218 * WDT_CLK"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TOUTSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "ICE Debug Mode Acknowledge Disable Control (Write Protect)\\nWDT up counter will keep going no matter CPU is held by ICE or not.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEDEBUG_A {
    #[doc = "0: ICE debug mode acknowledgement affects WDT counting"]
    _0 = 0,
    #[doc = "1: ICE debug mode acknowledgement Disabled"]
    _1 = 1,
}
impl From<ICEDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: ICEDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEDEBUG` reader - ICE Debug Mode Acknowledge Disable Control (Write Protect)\\nWDT up counter will keep going no matter CPU is held by ICE or not.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ICEDEBUG_R(crate::FieldReader<bool, ICEDEBUG_A>);
impl ICEDEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEDEBUG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDEBUG_A {
        match self.bits {
            false => ICEDEBUG_A::_0,
            true => ICEDEBUG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICEDEBUG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICEDEBUG_A::_1
    }
}
impl core::ops::Deref for ICEDEBUG_R {
    type Target = crate::FieldReader<bool, ICEDEBUG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEDEBUG` writer - ICE Debug Mode Acknowledge Disable Control (Write Protect)\\nWDT up counter will keep going no matter CPU is held by ICE or not.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ICEDEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEDEBUG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEDEBUG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode acknowledgement affects WDT counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_0)
    }
    #[doc = "ICE debug mode acknowledgement Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_1)
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
    #[doc = "Bit 0 - Reset WDT Up Counter (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit will be automatically cleared by hardware."]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDT Time-out Reset Enable Control (Write Protect)\\nSetting this bit will enable the WDT time-out reset function If the WDT up counter value has not been cleared after the specific WDT reset delay period expires.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDT Time-out Reset Flag\\nThis bit indicates the system has been reset by WDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rstf(&self) -> RSTF_R {
        RSTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDT Time-out Interrupt Flag\\nThis bit will set to 1 while WDT up counter value reaches the selected WDT time-out interval\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDT Time-out Wake-up Function Control (Write Protect)\\nIf this bit is set to 1, while WDT time-out interrupt flag IF (WDT_CTL\\[3\\]) is generated to 1 and interrupt enable bit INTEN (WDT_CTL\\[6\\]) is enabled, the WDT time-out interrupt signal will generate a wake-up trigger event to chip.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Chip can be woken-up by WDT time-out interrupt signal generated only if WDT clock source is selected to LIRC or LXT."]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDT Time-out Wake-up Flag (Write Protect)\\nThis bit indicates the interrupt wake-up flag status of WDT\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wkf(&self) -> WKF_R {
        WKF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WDT Time-out Interrupt Enable Control (Write Protect)\\nIf this bit is enabled, the WDT time-out interrupt signal is generated and inform to CPU. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WDT Enable Control (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: If CWDTEN\\[2:0\\]
(combined by Config0\\[31\\]
and Config0\\[4:3\\]) bits is not configure to 111, this bit is forced as 1 and user cannot change this bit to 0."]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - WDT Time-out Interval Selection (Write Protect)\\nThese three bits select the time-out interval period for the WDT.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn toutsel(&self) -> TOUTSEL_R {
        TOUTSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Control (Write Protect)\\nWDT up counter will keep going no matter CPU is held by ICE or not.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn icedebug(&self) -> ICEDEBUG_R {
        ICEDEBUG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset WDT Up Counter (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit will be automatically cleared by hardware."]
    #[inline(always)]
    pub fn rstcnt(&mut self) -> RSTCNT_W {
        RSTCNT_W { w: self }
    }
    #[doc = "Bit 1 - WDT Time-out Reset Enable Control (Write Protect)\\nSetting this bit will enable the WDT time-out reset function If the WDT up counter value has not been cleared after the specific WDT reset delay period expires.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W { w: self }
    }
    #[doc = "Bit 2 - WDT Time-out Reset Flag\\nThis bit indicates the system has been reset by WDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rstf(&mut self) -> RSTF_W {
        RSTF_W { w: self }
    }
    #[doc = "Bit 3 - WDT Time-out Interrupt Flag\\nThis bit will set to 1 while WDT up counter value reaches the selected WDT time-out interval\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn if_(&mut self) -> IF_W {
        IF_W { w: self }
    }
    #[doc = "Bit 4 - WDT Time-out Wake-up Function Control (Write Protect)\\nIf this bit is set to 1, while WDT time-out interrupt flag IF (WDT_CTL\\[3\\]) is generated to 1 and interrupt enable bit INTEN (WDT_CTL\\[6\\]) is enabled, the WDT time-out interrupt signal will generate a wake-up trigger event to chip.\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Chip can be woken-up by WDT time-out interrupt signal generated only if WDT clock source is selected to LIRC or LXT."]
    #[inline(always)]
    pub fn wken(&mut self) -> WKEN_W {
        WKEN_W { w: self }
    }
    #[doc = "Bit 5 - WDT Time-out Wake-up Flag (Write Protect)\\nThis bit indicates the interrupt wake-up flag status of WDT\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wkf(&mut self) -> WKF_W {
        WKF_W { w: self }
    }
    #[doc = "Bit 6 - WDT Time-out Interrupt Enable Control (Write Protect)\\nIf this bit is enabled, the WDT time-out interrupt signal is generated and inform to CPU. \\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 7 - WDT Enable Control (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: If CWDTEN\\[2:0\\]
(combined by Config0\\[31\\]
and Config0\\[4:3\\]) bits is not configure to 111, this bit is forced as 1 and user cannot change this bit to 0."]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
    #[doc = "Bits 8:10 - WDT Time-out Interval Selection (Write Protect)\\nThese three bits select the time-out interval period for the WDT.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn toutsel(&mut self) -> TOUTSEL_W {
        TOUTSEL_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Control (Write Protect)\\nWDT up counter will keep going no matter CPU is held by ICE or not.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn icedebug(&mut self) -> ICEDEBUG_W {
        ICEDEBUG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctl](index.html) module"]
pub struct WDT_CTL_SPEC;
impl crate::RegisterSpec for WDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_ctl::R](R) reader structure"]
impl crate::Readable for WDT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_ctl::W](W) writer structure"]
impl crate::Writable for WDT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CTL to value 0x0700"]
impl crate::Resettable for WDT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700
    }
}
