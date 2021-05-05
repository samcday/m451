#[doc = "Register `SC_CTL` reader"]
pub struct R(crate::R<SC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_CTL_SPEC>> for R {
    fn from(reader: crate::R<SC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_CTL` writer"]
pub struct W(crate::W<SC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_CTL_SPEC>;
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
impl core::convert::From<crate::W<SC_CTL_SPEC>> for W {
    fn from(writer: crate::W<SC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCEN` reader - SC Engine Enable Bit\\nSet this bit to 1 to enable SC operation. If this bit is cleared, SC will force all transition to IDLE state."]
pub struct SCEN_R(crate::FieldReader<bool, bool>);
impl SCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCEN` writer - SC Engine Enable Bit\\nSet this bit to 1 to enable SC operation. If this bit is cleared, SC will force all transition to IDLE state."]
pub struct SCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
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
#[doc = "RX Transition Disable Control\\nNote: If AUTOCEN (SC_CTL\\[3\\])is enabled, these fields must be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFF_A {
    #[doc = "0: The receiver Enabled"]
    _0 = 0,
    #[doc = "1: The receiver Disabled"]
    _1 = 1,
}
impl From<RXOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOFF` reader - RX Transition Disable Control\\nNote: If AUTOCEN (SC_CTL\\[3\\])is enabled, these fields must be ignored."]
pub struct RXOFF_R(crate::FieldReader<bool, RXOFF_A>);
impl RXOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFF_A {
        match self.bits {
            false => RXOFF_A::_0,
            true => RXOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOFF_A::_1
    }
}
impl core::ops::Deref for RXOFF_R {
    type Target = crate::FieldReader<bool, RXOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOFF` writer - RX Transition Disable Control\\nNote: If AUTOCEN (SC_CTL\\[3\\])is enabled, these fields must be ignored."]
pub struct RXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receiver Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFF_A::_0)
    }
    #[doc = "The receiver Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFF_A::_1)
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
#[doc = "TX Transition Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFF_A {
    #[doc = "0: The transceiver Enabled"]
    _0 = 0,
    #[doc = "1: The transceiver Disabled"]
    _1 = 1,
}
impl From<TXOFF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOFF` reader - TX Transition Disable Control"]
pub struct TXOFF_R(crate::FieldReader<bool, TXOFF_A>);
impl TXOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFF_A {
        match self.bits {
            false => TXOFF_A::_0,
            true => TXOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXOFF_A::_1
    }
}
impl core::ops::Deref for TXOFF_R {
    type Target = crate::FieldReader<bool, TXOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOFF` writer - TX Transition Disable Control"]
pub struct TXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transceiver Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFF_A::_0)
    }
    #[doc = "The transceiver Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFF_A::_1)
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
#[doc = "Auto Convention Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCEN_A {
    #[doc = "0: Auto-convention Disabled"]
    _0 = 0,
    #[doc = "1: Auto-convention Enabled. When hardware receives TS in answer to reset state and the TS is direct convention, CONSEL(SC_CTL\\[5:4\\]) will be set to 00 automatically, otherwise if the TS is inverse convention, and CONSEL (SC_CTL\\[5:4\\]) will be set to 11"]
    _1 = 1,
}
impl From<AUTOCEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCEN` reader - Auto Convention Enable Bit"]
pub struct AUTOCEN_R(crate::FieldReader<bool, AUTOCEN_A>);
impl AUTOCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOCEN_A {
        match self.bits {
            false => AUTOCEN_A::_0,
            true => AUTOCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AUTOCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AUTOCEN_A::_1
    }
}
impl core::ops::Deref for AUTOCEN_R {
    type Target = crate::FieldReader<bool, AUTOCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCEN` writer - Auto Convention Enable Bit"]
pub struct AUTOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-convention Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOCEN_A::_0)
    }
    #[doc = "Auto-convention Enabled. When hardware receives TS in answer to reset state and the TS is direct convention, CONSEL(SC_CTL\\[5:4\\]) will be set to 00 automatically, otherwise if the TS is inverse convention, and CONSEL (SC_CTL\\[5:4\\]) will be set to 11"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOCEN_A::_1)
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
#[doc = "Convention Selection\\nNote: If AUTOCEN(SC_CTL\\[3\\]) enabled, this fields are ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONSEL_A {
    #[doc = "0: Direct convention"]
    _0 = 0,
    #[doc = "1: Reserved."]
    _1 = 1,
    #[doc = "2: Reserved."]
    _2 = 2,
    #[doc = "3: Inverse convention"]
    _3 = 3,
}
impl From<CONSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CONSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CONSEL` reader - Convention Selection\\nNote: If AUTOCEN(SC_CTL\\[3\\]) enabled, this fields are ignored."]
pub struct CONSEL_R(crate::FieldReader<u8, CONSEL_A>);
impl CONSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONSEL_A {
        match self.bits {
            0 => CONSEL_A::_0,
            1 => CONSEL_A::_1,
            2 => CONSEL_A::_2,
            3 => CONSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CONSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CONSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CONSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CONSEL_A::_3
    }
}
impl core::ops::Deref for CONSEL_R {
    type Target = crate::FieldReader<u8, CONSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSEL` writer - Convention Selection\\nNote: If AUTOCEN(SC_CTL\\[3\\]) enabled, this fields are ignored."]
pub struct CONSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Direct convention"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONSEL_A::_0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONSEL_A::_1)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CONSEL_A::_2)
    }
    #[doc = "Inverse convention"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CONSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Rx Buffer Trigger Level \\nWhen the number of bytes in the receiving buffer equals the RXTRGLV, the RDAIF will be set (if SC_INTEN \\[RDAIEN\\]
is enabled, an interrupt will be generated).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXTRGLV_A {
    #[doc = "0: INTR_RDA Trigger Level with 1 Byte"]
    _0 = 0,
    #[doc = "1: INTR_RDA Trigger Level with 2 Bytes"]
    _1 = 1,
    #[doc = "2: INTR_RDA Trigger Level with 3 Bytes"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<RXTRGLV_A> for u8 {
    #[inline(always)]
    fn from(variant: RXTRGLV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXTRGLV` reader - Rx Buffer Trigger Level \\nWhen the number of bytes in the receiving buffer equals the RXTRGLV, the RDAIF will be set (if SC_INTEN \\[RDAIEN\\]
is enabled, an interrupt will be generated)."]
pub struct RXTRGLV_R(crate::FieldReader<u8, RXTRGLV_A>);
impl RXTRGLV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXTRGLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTRGLV_A {
        match self.bits {
            0 => RXTRGLV_A::_0,
            1 => RXTRGLV_A::_1,
            2 => RXTRGLV_A::_2,
            3 => RXTRGLV_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTRGLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTRGLV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == RXTRGLV_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == RXTRGLV_A::_3
    }
}
impl core::ops::Deref for RXTRGLV_R {
    type Target = crate::FieldReader<u8, RXTRGLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTRGLV` writer - Rx Buffer Trigger Level \\nWhen the number of bytes in the receiving buffer equals the RXTRGLV, the RDAIF will be set (if SC_INTEN \\[RDAIEN\\]
is enabled, an interrupt will be generated)."]
pub struct RXTRGLV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRGLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTRGLV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "INTR_RDA Trigger Level with 1 Byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTRGLV_A::_0)
    }
    #[doc = "INTR_RDA Trigger Level with 2 Bytes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTRGLV_A::_1)
    }
    #[doc = "INTR_RDA Trigger Level with 3 Bytes"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RXTRGLV_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RXTRGLV_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `BGT` reader - Block Guard Time (BGT)\\nNote: The real block guard time is BGT + 1."]
pub struct BGT_R(crate::FieldReader<u8, u8>);
impl BGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGT` writer - Block Guard Time (BGT)\\nNote: The real block guard time is BGT + 1."]
pub struct BGT_W<'a> {
    w: &'a mut W,
}
impl<'a> BGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Timer Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRSEL_A {
    #[doc = "0: All internal timer function Disabled"]
    _0 = 0,
    #[doc = "1: Internal 24 bit timer Enabled. Software can configure it by setting SC_TMRCTL0 \\[23:0\\]. SC_TMRCTL1 and SC_TMRCTL2 will be ignored in this mode"]
    _1 = 1,
    #[doc = "2: internal 24 bit timer and 8 bit internal timer Enabled. Software can configure the 24 bit timer by setting SC_TMRCTL0 \\[23:0\\]
and configure the 8 bit timer by setting SC_TMRCTL1\\[7:0\\]. SC_TMRCTL2 will be ignored in this mode"]
    _2 = 2,
    #[doc = "3: Internal 24 bit timer and two 8 bit timers Enabled. Software can configure them by setting SC_TMRCTL0 \\[23:0\\], SC_TMRCTL1 \\[7:0\\]
and SC_TMRCTL2 \\[7:0\\]"]
    _3 = 3,
}
impl From<TMRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRSEL` reader - Timer Selection"]
pub struct TMRSEL_R(crate::FieldReader<u8, TMRSEL_A>);
impl TMRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRSEL_A {
        match self.bits {
            0 => TMRSEL_A::_0,
            1 => TMRSEL_A::_1,
            2 => TMRSEL_A::_2,
            3 => TMRSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMRSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TMRSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TMRSEL_A::_3
    }
}
impl core::ops::Deref for TMRSEL_R {
    type Target = crate::FieldReader<u8, TMRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRSEL` writer - Timer Selection"]
pub struct TMRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "All internal timer function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMRSEL_A::_0)
    }
    #[doc = "Internal 24 bit timer Enabled. Software can configure it by setting SC_TMRCTL0 \\[23:0\\]. SC_TMRCTL1 and SC_TMRCTL2 will be ignored in this mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMRSEL_A::_1)
    }
    #[doc = "internal 24 bit timer and 8 bit internal timer Enabled. Software can configure the 24 bit timer by setting SC_TMRCTL0 \\[23:0\\]
and configure the 8 bit timer by setting SC_TMRCTL1\\[7:0\\]. SC_TMRCTL2 will be ignored in this mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TMRSEL_A::_2)
    }
    #[doc = "Internal 24 bit timer and two 8 bit timers Enabled. Software can configure them by setting SC_TMRCTL0 \\[23:0\\], SC_TMRCTL1 \\[7:0\\]
and SC_TMRCTL2 \\[7:0\\]"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TMRSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Stop Bit Length\\nThis field indicates the length of stop bit.\\nNote: The default stop bit length is 2. SMC and UART adopts NSB to program the stop bit length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSB_A {
    #[doc = "0: The stop bit length is 2 ETU"]
    _0 = 0,
    #[doc = "1: The stop bit length is 1 ETU"]
    _1 = 1,
}
impl From<NSB_A> for bool {
    #[inline(always)]
    fn from(variant: NSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSB` reader - Stop Bit Length\\nThis field indicates the length of stop bit.\\nNote: The default stop bit length is 2. SMC and UART adopts NSB to program the stop bit length"]
pub struct NSB_R(crate::FieldReader<bool, NSB_A>);
impl NSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSB_A {
        match self.bits {
            false => NSB_A::_0,
            true => NSB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NSB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NSB_A::_1
    }
}
impl core::ops::Deref for NSB_R {
    type Target = crate::FieldReader<bool, NSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSB` writer - Stop Bit Length\\nThis field indicates the length of stop bit.\\nNote: The default stop bit length is 2. SMC and UART adopts NSB to program the stop bit length"]
pub struct NSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The stop bit length is 2 ETU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NSB_A::_0)
    }
    #[doc = "The stop bit length is 1 ETU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSB_A::_1)
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
#[doc = "Field `RXRTY` reader - RX Error Retry Count Number\\nThis field indicates the maximum number of receiver retries that are allowed when parity error has occurred\\nNote1: The real retry number is RXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when RXRTYEN enabled. The change flow is to disable RXRTYEN first and then fill in new retry value."]
pub struct RXRTY_R(crate::FieldReader<u8, u8>);
impl RXRTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXRTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRTY` writer - RX Error Retry Count Number\\nThis field indicates the maximum number of receiver retries that are allowed when parity error has occurred\\nNote1: The real retry number is RXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when RXRTYEN enabled. The change flow is to disable RXRTYEN first and then fill in new retry value."]
pub struct RXRTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "RX Error Retry Enable Bit\\nThis bit enables receiver retry function when parity error has occurred.\\nNote: Software must fill in the RXRTY value before enabling this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRTYEN_A {
    #[doc = "0: RX error retry function Disabled"]
    _0 = 0,
    #[doc = "1: RX error retry function Enabled"]
    _1 = 1,
}
impl From<RXRTYEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXRTYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRTYEN` reader - RX Error Retry Enable Bit\\nThis bit enables receiver retry function when parity error has occurred.\\nNote: Software must fill in the RXRTY value before enabling this bit."]
pub struct RXRTYEN_R(crate::FieldReader<bool, RXRTYEN_A>);
impl RXRTYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRTYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRTYEN_A {
        match self.bits {
            false => RXRTYEN_A::_0,
            true => RXRTYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXRTYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXRTYEN_A::_1
    }
}
impl core::ops::Deref for RXRTYEN_R {
    type Target = crate::FieldReader<bool, RXRTYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRTYEN` writer - RX Error Retry Enable Bit\\nThis bit enables receiver retry function when parity error has occurred.\\nNote: Software must fill in the RXRTY value before enabling this bit."]
pub struct RXRTYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRTYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRTYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX error retry function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRTYEN_A::_0)
    }
    #[doc = "RX error retry function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRTYEN_A::_1)
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
#[doc = "Field `TXRTY` reader - TX Error Retry Count Number\\nThis field indicates the maximum number of transmitter retries that are allowed when parity error has occurred.\\nNote1: The real retry number is TXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when TXRTYEN enabled. The change flow is to disable TXRTYEN first and then fill in new retry value."]
pub struct TXRTY_R(crate::FieldReader<u8, u8>);
impl TXRTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXRTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRTY` writer - TX Error Retry Count Number\\nThis field indicates the maximum number of transmitter retries that are allowed when parity error has occurred.\\nNote1: The real retry number is TXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when TXRTYEN enabled. The change flow is to disable TXRTYEN first and then fill in new retry value."]
pub struct TXRTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "TX Error Retry Enable Bit\\nThis bit enables transmitter retry function when parity error has occurred.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTYEN_A {
    #[doc = "0: TX error retry function Disabled"]
    _0 = 0,
    #[doc = "1: TX error retry function Enabled"]
    _1 = 1,
}
impl From<TXRTYEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRTYEN` reader - TX Error Retry Enable Bit\\nThis bit enables transmitter retry function when parity error has occurred."]
pub struct TXRTYEN_R(crate::FieldReader<bool, TXRTYEN_A>);
impl TXRTYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRTYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTYEN_A {
        match self.bits {
            false => TXRTYEN_A::_0,
            true => TXRTYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRTYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRTYEN_A::_1
    }
}
impl core::ops::Deref for TXRTYEN_R {
    type Target = crate::FieldReader<bool, TXRTYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRTYEN` writer - TX Error Retry Enable Bit\\nThis bit enables transmitter retry function when parity error has occurred."]
pub struct TXRTYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRTYEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX error retry function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTYEN_A::_0)
    }
    #[doc = "TX error retry function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTYEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Card Detect De-bounce Selection\\nThis field indicates the card detect de-bounce selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDDBSEL_A {
    #[doc = "0: De-bounce sample card insert once per 384 (128 * 3) peripheral clocks and de-bounce sample card removal once per 128 peripheral clocks"]
    _0 = 0,
    #[doc = "1: De-bounce sample card insert once per 192 (64 * 3) peripheral clocks and de-bounce sample card removal once per 64 peripheral clocks"]
    _1 = 1,
    #[doc = "2: De-bounce sample card insert once per 96 (32 * 3) peripheral clocks and de-bounce sample card removal once per 32 peripheral clocks"]
    _2 = 2,
    #[doc = "3: De-bounce sample card insert once per 48 (16 * 3) peripheral clocks and de-bounce sample card removal once per 16 peripheral clocks"]
    _3 = 3,
}
impl From<CDDBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CDDBSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDDBSEL` reader - Card Detect De-bounce Selection\\nThis field indicates the card detect de-bounce selection."]
pub struct CDDBSEL_R(crate::FieldReader<u8, CDDBSEL_A>);
impl CDDBSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDDBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDDBSEL_A {
        match self.bits {
            0 => CDDBSEL_A::_0,
            1 => CDDBSEL_A::_1,
            2 => CDDBSEL_A::_2,
            3 => CDDBSEL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CDDBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CDDBSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CDDBSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CDDBSEL_A::_3
    }
}
impl core::ops::Deref for CDDBSEL_R {
    type Target = crate::FieldReader<u8, CDDBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDDBSEL` writer - Card Detect De-bounce Selection\\nThis field indicates the card detect de-bounce selection."]
pub struct CDDBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDDBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDDBSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "De-bounce sample card insert once per 384 (128 * 3) peripheral clocks and de-bounce sample card removal once per 128 peripheral clocks"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDDBSEL_A::_0)
    }
    #[doc = "De-bounce sample card insert once per 192 (64 * 3) peripheral clocks and de-bounce sample card removal once per 64 peripheral clocks"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDDBSEL_A::_1)
    }
    #[doc = "De-bounce sample card insert once per 96 (32 * 3) peripheral clocks and de-bounce sample card removal once per 32 peripheral clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CDDBSEL_A::_2)
    }
    #[doc = "De-bounce sample card insert once per 48 (16 * 3) peripheral clocks and de-bounce sample card removal once per 16 peripheral clocks"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CDDBSEL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Card Detect Level \\nNote: Software must select card detect level before Smart Card engine enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDLV_A {
    #[doc = "0: When hardware detects the card detect pin (SC_CD) from high to low, it indicates a card is detected"]
    _0 = 0,
    #[doc = "1: When hardware detects the card detect pin from low to high, it indicates a card is detected"]
    _1 = 1,
}
impl From<CDLV_A> for bool {
    #[inline(always)]
    fn from(variant: CDLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDLV` reader - Card Detect Level \\nNote: Software must select card detect level before Smart Card engine enabled."]
pub struct CDLV_R(crate::FieldReader<bool, CDLV_A>);
impl CDLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDLV_A {
        match self.bits {
            false => CDLV_A::_0,
            true => CDLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CDLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CDLV_A::_1
    }
}
impl core::ops::Deref for CDLV_R {
    type Target = crate::FieldReader<bool, CDLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDLV` writer - Card Detect Level \\nNote: Software must select card detect level before Smart Card engine enabled."]
pub struct CDLV_W<'a> {
    w: &'a mut W,
}
impl<'a> CDLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDLV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When hardware detects the card detect pin (SC_CD) from high to low, it indicates a card is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDLV_A::_0)
    }
    #[doc = "When hardware detects the card detect pin from low to high, it indicates a card is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDLV_A::_1)
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
#[doc = "SYNC Flag Indicator\\nDue to synchronization, software should check this bit before writing a new value to RXRTY and TXRTY.\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: synchronizing is completion, user can write new data to RXRTY and TXRTY"]
    _0 = 0,
    #[doc = "1: Last value is synchronizing"]
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - SYNC Flag Indicator\\nDue to synchronization, software should check this bit before writing a new value to RXRTY and TXRTY.\\nNote: This bit is read only."]
pub struct SYNC_R(crate::FieldReader<bool, SYNC_A>);
impl SYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::_0,
            true => SYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNC_A::_1
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<bool, SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC` writer - SYNC Flag Indicator\\nDue to synchronization, software should check this bit before writing a new value to RXRTY and TXRTY.\\nNote: This bit is read only."]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "synchronizing is completion, user can write new data to RXRTY and TXRTY"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNC_A::_0)
    }
    #[doc = "Last value is synchronizing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "ICE Debug Mode Acknowledge Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGOFF_A {
    #[doc = "0: When DBGACK is high, the internal counter will be hold"]
    _0 = 0,
    #[doc = "1: No matter DBGACK is high or low, the internal counter will not be hold"]
    _1 = 1,
}
impl From<DBGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DBGOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGOFF` reader - ICE Debug Mode Acknowledge Enable Bit"]
pub struct DBGOFF_R(crate::FieldReader<bool, DBGOFF_A>);
impl DBGOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGOFF_A {
        match self.bits {
            false => DBGOFF_A::_0,
            true => DBGOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGOFF_A::_1
    }
}
impl core::ops::Deref for DBGOFF_R {
    type Target = crate::FieldReader<bool, DBGOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGOFF` writer - ICE Debug Mode Acknowledge Enable Bit"]
pub struct DBGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When DBGACK is high, the internal counter will be hold"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGOFF_A::_0)
    }
    #[doc = "No matter DBGACK is high or low, the internal counter will not be hold"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGOFF_A::_1)
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
    #[doc = "Bit 0 - SC Engine Enable Bit\\nSet this bit to 1 to enable SC operation. If this bit is cleared, SC will force all transition to IDLE state."]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Transition Disable Control\\nNote: If AUTOCEN (SC_CTL\\[3\\])is enabled, these fields must be ignored."]
    #[inline(always)]
    pub fn rxoff(&self) -> RXOFF_R {
        RXOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX Transition Disable Control"]
    #[inline(always)]
    pub fn txoff(&self) -> TXOFF_R {
        TXOFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto Convention Enable Bit"]
    #[inline(always)]
    pub fn autocen(&self) -> AUTOCEN_R {
        AUTOCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Convention Selection\\nNote: If AUTOCEN(SC_CTL\\[3\\]) enabled, this fields are ignored."]
    #[inline(always)]
    pub fn consel(&self) -> CONSEL_R {
        CONSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Rx Buffer Trigger Level \\nWhen the number of bytes in the receiving buffer equals the RXTRGLV, the RDAIF will be set (if SC_INTEN \\[RDAIEN\\]
is enabled, an interrupt will be generated)."]
    #[inline(always)]
    pub fn rxtrglv(&self) -> RXTRGLV_R {
        RXTRGLV_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Block Guard Time (BGT)\\nNote: The real block guard time is BGT + 1."]
    #[inline(always)]
    pub fn bgt(&self) -> BGT_R {
        BGT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Timer Selection"]
    #[inline(always)]
    pub fn tmrsel(&self) -> TMRSEL_R {
        TMRSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Stop Bit Length\\nThis field indicates the length of stop bit.\\nNote: The default stop bit length is 2. SMC and UART adopts NSB to program the stop bit length"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - RX Error Retry Count Number\\nThis field indicates the maximum number of receiver retries that are allowed when parity error has occurred\\nNote1: The real retry number is RXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when RXRTYEN enabled. The change flow is to disable RXRTYEN first and then fill in new retry value."]
    #[inline(always)]
    pub fn rxrty(&self) -> RXRTY_R {
        RXRTY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - RX Error Retry Enable Bit\\nThis bit enables receiver retry function when parity error has occurred.\\nNote: Software must fill in the RXRTY value before enabling this bit."]
    #[inline(always)]
    pub fn rxrtyen(&self) -> RXRTYEN_R {
        RXRTYEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - TX Error Retry Count Number\\nThis field indicates the maximum number of transmitter retries that are allowed when parity error has occurred.\\nNote1: The real retry number is TXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when TXRTYEN enabled. The change flow is to disable TXRTYEN first and then fill in new retry value."]
    #[inline(always)]
    pub fn txrty(&self) -> TXRTY_R {
        TXRTY_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - TX Error Retry Enable Bit\\nThis bit enables transmitter retry function when parity error has occurred."]
    #[inline(always)]
    pub fn txrtyen(&self) -> TXRTYEN_R {
        TXRTYEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Card Detect De-bounce Selection\\nThis field indicates the card detect de-bounce selection."]
    #[inline(always)]
    pub fn cddbsel(&self) -> CDDBSEL_R {
        CDDBSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Card Detect Level \\nNote: Software must select card detect level before Smart Card engine enabled."]
    #[inline(always)]
    pub fn cdlv(&self) -> CDLV_R {
        CDLV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SYNC Flag Indicator\\nDue to synchronization, software should check this bit before writing a new value to RXRTY and TXRTY.\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Enable Bit"]
    #[inline(always)]
    pub fn dbgoff(&self) -> DBGOFF_R {
        DBGOFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SC Engine Enable Bit\\nSet this bit to 1 to enable SC operation. If this bit is cleared, SC will force all transition to IDLE state."]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W { w: self }
    }
    #[doc = "Bit 1 - RX Transition Disable Control\\nNote: If AUTOCEN (SC_CTL\\[3\\])is enabled, these fields must be ignored."]
    #[inline(always)]
    pub fn rxoff(&mut self) -> RXOFF_W {
        RXOFF_W { w: self }
    }
    #[doc = "Bit 2 - TX Transition Disable Control"]
    #[inline(always)]
    pub fn txoff(&mut self) -> TXOFF_W {
        TXOFF_W { w: self }
    }
    #[doc = "Bit 3 - Auto Convention Enable Bit"]
    #[inline(always)]
    pub fn autocen(&mut self) -> AUTOCEN_W {
        AUTOCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Convention Selection\\nNote: If AUTOCEN(SC_CTL\\[3\\]) enabled, this fields are ignored."]
    #[inline(always)]
    pub fn consel(&mut self) -> CONSEL_W {
        CONSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Rx Buffer Trigger Level \\nWhen the number of bytes in the receiving buffer equals the RXTRGLV, the RDAIF will be set (if SC_INTEN \\[RDAIEN\\]
is enabled, an interrupt will be generated)."]
    #[inline(always)]
    pub fn rxtrglv(&mut self) -> RXTRGLV_W {
        RXTRGLV_W { w: self }
    }
    #[doc = "Bits 8:12 - Block Guard Time (BGT)\\nNote: The real block guard time is BGT + 1."]
    #[inline(always)]
    pub fn bgt(&mut self) -> BGT_W {
        BGT_W { w: self }
    }
    #[doc = "Bits 13:14 - Timer Selection"]
    #[inline(always)]
    pub fn tmrsel(&mut self) -> TMRSEL_W {
        TMRSEL_W { w: self }
    }
    #[doc = "Bit 15 - Stop Bit Length\\nThis field indicates the length of stop bit.\\nNote: The default stop bit length is 2. SMC and UART adopts NSB to program the stop bit length"]
    #[inline(always)]
    pub fn nsb(&mut self) -> NSB_W {
        NSB_W { w: self }
    }
    #[doc = "Bits 16:18 - RX Error Retry Count Number\\nThis field indicates the maximum number of receiver retries that are allowed when parity error has occurred\\nNote1: The real retry number is RXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when RXRTYEN enabled. The change flow is to disable RXRTYEN first and then fill in new retry value."]
    #[inline(always)]
    pub fn rxrty(&mut self) -> RXRTY_W {
        RXRTY_W { w: self }
    }
    #[doc = "Bit 19 - RX Error Retry Enable Bit\\nThis bit enables receiver retry function when parity error has occurred.\\nNote: Software must fill in the RXRTY value before enabling this bit."]
    #[inline(always)]
    pub fn rxrtyen(&mut self) -> RXRTYEN_W {
        RXRTYEN_W { w: self }
    }
    #[doc = "Bits 20:22 - TX Error Retry Count Number\\nThis field indicates the maximum number of transmitter retries that are allowed when parity error has occurred.\\nNote1: The real retry number is TXRTY + 1, so 8 is the maximum retry number.\\nNote2: This field cannot be changed when TXRTYEN enabled. The change flow is to disable TXRTYEN first and then fill in new retry value."]
    #[inline(always)]
    pub fn txrty(&mut self) -> TXRTY_W {
        TXRTY_W { w: self }
    }
    #[doc = "Bit 23 - TX Error Retry Enable Bit\\nThis bit enables transmitter retry function when parity error has occurred."]
    #[inline(always)]
    pub fn txrtyen(&mut self) -> TXRTYEN_W {
        TXRTYEN_W { w: self }
    }
    #[doc = "Bits 24:25 - Card Detect De-bounce Selection\\nThis field indicates the card detect de-bounce selection."]
    #[inline(always)]
    pub fn cddbsel(&mut self) -> CDDBSEL_W {
        CDDBSEL_W { w: self }
    }
    #[doc = "Bit 26 - Card Detect Level \\nNote: Software must select card detect level before Smart Card engine enabled."]
    #[inline(always)]
    pub fn cdlv(&mut self) -> CDLV_W {
        CDLV_W { w: self }
    }
    #[doc = "Bit 30 - SYNC Flag Indicator\\nDue to synchronization, software should check this bit before writing a new value to RXRTY and TXRTY.\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Enable Bit"]
    #[inline(always)]
    pub fn dbgoff(&mut self) -> DBGOFF_W {
        DBGOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_ctl](index.html) module"]
pub struct SC_CTL_SPEC;
impl crate::RegisterSpec for SC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_ctl::R](R) reader structure"]
impl crate::Readable for SC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_ctl::W](W) writer structure"]
impl crate::Writable for SC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_CTL to value 0"]
impl crate::Resettable for SC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
