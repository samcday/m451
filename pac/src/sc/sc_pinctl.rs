#[doc = "Register `SC_PINCTL` reader"]
pub struct R(crate::R<SC_PINCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_PINCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_PINCTL_SPEC>> for R {
    fn from(reader: crate::R<SC_PINCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_PINCTL` writer"]
pub struct W(crate::W<SC_PINCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_PINCTL_SPEC>;
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
impl core::convert::From<crate::W<SC_PINCTL_SPEC>> for W {
    fn from(writer: crate::W<SC_PINCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SC_PWREN Pin Signal\\nSoftware can set PWREN (SC_PINCTL\\[0\\]) and PWRINV (SC_PINCTL\\[11\\])to decide SC_PWR pin is in high or low level.\\nWrite this field to drive SC_PWR pin\\nRefer PWRINV (SC_PINCTL\\[11\\]) description for programming SC_PWR pin voltage level. \\nRead this field to get SC_PWR pin status.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWREN_A {
    #[doc = "0: SC_PWR pin status is low"]
    _0 = 0,
    #[doc = "1: SC_PWR pin status is high"]
    _1 = 1,
}
impl From<PWREN_A> for bool {
    #[inline(always)]
    fn from(variant: PWREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN` reader - SC_PWREN Pin Signal\\nSoftware can set PWREN (SC_PINCTL\\[0\\]) and PWRINV (SC_PINCTL\\[11\\])to decide SC_PWR pin is in high or low level.\\nWrite this field to drive SC_PWR pin\\nRefer PWRINV (SC_PINCTL\\[11\\]) description for programming SC_PWR pin voltage level. \\nRead this field to get SC_PWR pin status.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct PWREN_R(crate::FieldReader<bool, PWREN_A>);
impl PWREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWREN_A {
        match self.bits {
            false => PWREN_A::_0,
            true => PWREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWREN_A::_1
    }
}
impl core::ops::Deref for PWREN_R {
    type Target = crate::FieldReader<bool, PWREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWREN` writer - SC_PWREN Pin Signal\\nSoftware can set PWREN (SC_PINCTL\\[0\\]) and PWRINV (SC_PINCTL\\[11\\])to decide SC_PWR pin is in high or low level.\\nWrite this field to drive SC_PWR pin\\nRefer PWRINV (SC_PINCTL\\[11\\]) description for programming SC_PWR pin voltage level. \\nRead this field to get SC_PWR pin status.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct PWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC_PWR pin status is low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWREN_A::_0)
    }
    #[doc = "SC_PWR pin status is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWREN_A::_1)
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
#[doc = "SC_RST Pin Signal\\nThis bit is the pin status of SC_RST but user can drive SC_RST pin to high or low by setting this bit.\\nWrite this field to drive SC_RST pin.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCRST_A {
    #[doc = "0: Drive SC_RST pin to low.\\nSC_RST pin status is low"]
    _0 = 0,
    #[doc = "1: Drive SC_RST pin to high.\\nSC_RST pin status is high"]
    _1 = 1,
}
impl From<SCRST_A> for bool {
    #[inline(always)]
    fn from(variant: SCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCRST` reader - SC_RST Pin Signal\\nThis bit is the pin status of SC_RST but user can drive SC_RST pin to high or low by setting this bit.\\nWrite this field to drive SC_RST pin.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct SCRST_R(crate::FieldReader<bool, SCRST_A>);
impl SCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCRST_A {
        match self.bits {
            false => SCRST_A::_0,
            true => SCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCRST_A::_1
    }
}
impl core::ops::Deref for SCRST_R {
    type Target = crate::FieldReader<bool, SCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRST` writer - SC_RST Pin Signal\\nThis bit is the pin status of SC_RST but user can drive SC_RST pin to high or low by setting this bit.\\nWrite this field to drive SC_RST pin.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct SCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Drive SC_RST pin to low.\\nSC_RST pin status is low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCRST_A::_0)
    }
    #[doc = "Drive SC_RST pin to high.\\nSC_RST pin status is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCRST_A::_1)
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
#[doc = "SC Clock Enable Bit \\nNote: When operating in activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKKEEP_A {
    #[doc = "0: SC clock generation Disabled"]
    _0 = 0,
    #[doc = "1: SC clock always keeps free running"]
    _1 = 1,
}
impl From<CLKKEEP_A> for bool {
    #[inline(always)]
    fn from(variant: CLKKEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKKEEP` reader - SC Clock Enable Bit \\nNote: When operating in activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct CLKKEEP_R(crate::FieldReader<bool, CLKKEEP_A>);
impl CLKKEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKKEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKKEEP_A {
        match self.bits {
            false => CLKKEEP_A::_0,
            true => CLKKEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKKEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKKEEP_A::_1
    }
}
impl core::ops::Deref for CLKKEEP_R {
    type Target = crate::FieldReader<bool, CLKKEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKKEEP` writer - SC Clock Enable Bit \\nNote: When operating in activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
pub struct CLKKEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKKEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKKEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC clock generation Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKKEEP_A::_0)
    }
    #[doc = "SC clock always keeps free running"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKKEEP_A::_1)
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
#[doc = "SC Data Output Pin \\nThis bit is the pin status of SCDATOUT but user can drive SCDATOUT pin to high or low by setting this bit.\\nNote: When SC is at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when SC is in these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCDOUT_A {
    #[doc = "0: Drive SCDATOUT pin to low"]
    _0 = 0,
    #[doc = "1: Drive SCDATOUT pin to high"]
    _1 = 1,
}
impl From<SCDOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SCDOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCDOUT` reader - SC Data Output Pin \\nThis bit is the pin status of SCDATOUT but user can drive SCDATOUT pin to high or low by setting this bit.\\nNote: When SC is at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when SC is in these modes."]
pub struct SCDOUT_R(crate::FieldReader<bool, SCDOUT_A>);
impl SCDOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDOUT_A {
        match self.bits {
            false => SCDOUT_A::_0,
            true => SCDOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCDOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCDOUT_A::_1
    }
}
impl core::ops::Deref for SCDOUT_R {
    type Target = crate::FieldReader<bool, SCDOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDOUT` writer - SC Data Output Pin \\nThis bit is the pin status of SCDATOUT but user can drive SCDATOUT pin to high or low by setting this bit.\\nNote: When SC is at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when SC is in these modes."]
pub struct SCDOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCDOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Drive SCDATOUT pin to low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCDOUT_A::_0)
    }
    #[doc = "Drive SCDATOUT pin to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCDOUT_A::_1)
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
#[doc = "SC_POW Pin Inverse\\nThis bit is used for inverse the SC_POW pin.\\nThere are four kinds of combination for SC_POW pin setting by PWRINV(SC_PINCTL\\[11\\]) and PWREN(SC_PINCTL\\[0\\]). PWRINV (SC_PINCTL\\[11\\]) is bit 1 and PWREN(SC_PINCTL\\[0\\]) is bit 0 for SC_POW_Pin as high or low voltage selection.\\nNote: Software must select PWRINV (SC_PINCTL\\[11\\]) before Smart Card is enabled by SCEN (SC_CTL\\[0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRINV_A {
    #[doc = "0: SC_POW_ Pin is 0"]
    _0 = 0,
    #[doc = "1: SC_POW _Pin is 1"]
    _1 = 1,
    #[doc = "2: SC_POW _Pin is 1"]
    _10 = 2,
    #[doc = "3: SC_POW_ Pin is 0"]
    _11 = 3,
}
impl From<PWRINV_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRINV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRINV` reader - SC_POW Pin Inverse\\nThis bit is used for inverse the SC_POW pin.\\nThere are four kinds of combination for SC_POW pin setting by PWRINV(SC_PINCTL\\[11\\]) and PWREN(SC_PINCTL\\[0\\]). PWRINV (SC_PINCTL\\[11\\]) is bit 1 and PWREN(SC_PINCTL\\[0\\]) is bit 0 for SC_POW_Pin as high or low voltage selection.\\nNote: Software must select PWRINV (SC_PINCTL\\[11\\]) before Smart Card is enabled by SCEN (SC_CTL\\[0\\])."]
pub struct PWRINV_R(crate::FieldReader<u8, PWRINV_A>);
impl PWRINV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRINV_A {
        match self.bits {
            0 => PWRINV_A::_0,
            1 => PWRINV_A::_1,
            2 => PWRINV_A::_10,
            3 => PWRINV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWRINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWRINV_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PWRINV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PWRINV_A::_11
    }
}
impl core::ops::Deref for PWRINV_R {
    type Target = crate::FieldReader<u8, PWRINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRINV` writer - SC_POW Pin Inverse\\nThis bit is used for inverse the SC_POW pin.\\nThere are four kinds of combination for SC_POW pin setting by PWRINV(SC_PINCTL\\[11\\]) and PWREN(SC_PINCTL\\[0\\]). PWRINV (SC_PINCTL\\[11\\]) is bit 1 and PWREN(SC_PINCTL\\[0\\]) is bit 0 for SC_POW_Pin as high or low voltage selection.\\nNote: Software must select PWRINV (SC_PINCTL\\[11\\]) before Smart Card is enabled by SCEN (SC_CTL\\[0\\])."]
pub struct PWRINV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRINV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SC_POW_ Pin is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWRINV_A::_0)
    }
    #[doc = "SC_POW _Pin is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWRINV_A::_1)
    }
    #[doc = "SC_POW _Pin is 1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PWRINV_A::_10)
    }
    #[doc = "SC_POW_ Pin is 0"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PWRINV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "SC Data Pin Output Status \\nThis bit is the pin status of SCDATOUT \\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCDOSTS_A {
    #[doc = "0: SCDATOUT pin to low"]
    _0 = 0,
    #[doc = "1: SCDATOUT pin to high"]
    _1 = 1,
}
impl From<SCDOSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SCDOSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCDOSTS` reader - SC Data Pin Output Status \\nThis bit is the pin status of SCDATOUT \\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
pub struct SCDOSTS_R(crate::FieldReader<bool, SCDOSTS_A>);
impl SCDOSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDOSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDOSTS_A {
        match self.bits {
            false => SCDOSTS_A::_0,
            true => SCDOSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCDOSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCDOSTS_A::_1
    }
}
impl core::ops::Deref for SCDOSTS_R {
    type Target = crate::FieldReader<bool, SCDOSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDOSTS` writer - SC Data Pin Output Status \\nThis bit is the pin status of SCDATOUT \\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
pub struct SCDOSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDOSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCDOSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCDATOUT pin to low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCDOSTS_A::_0)
    }
    #[doc = "SCDATOUT pin to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCDOSTS_A::_1)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATSTS_A {
    #[doc = "0: The SC_DAT pin is low"]
    _0 = 0,
    #[doc = "1: The SC_DAT pin is high"]
    _1 = 1,
}
impl From<DATSTS_A> for bool {
    #[inline(always)]
    fn from(variant: DATSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATSTS` reader - "]
pub struct DATSTS_R(crate::FieldReader<bool, DATSTS_A>);
impl DATSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATSTS_A {
        match self.bits {
            false => DATSTS_A::_0,
            true => DATSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATSTS_A::_1
    }
}
impl core::ops::Deref for DATSTS_R {
    type Target = crate::FieldReader<bool, DATSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATSTS` writer - "]
pub struct DATSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SC_DAT pin is low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATSTS_A::_0)
    }
    #[doc = "The SC_DAT pin is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATSTS_A::_1)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSTS_A {
    #[doc = "0: SC_PWR pin to low"]
    _0 = 0,
    #[doc = "1: SC_PWR pin to high"]
    _1 = 1,
}
impl From<PWRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTS` reader - "]
pub struct PWRSTS_R(crate::FieldReader<bool, PWRSTS_A>);
impl PWRSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSTS_A {
        match self.bits {
            false => PWRSTS_A::_0,
            true => PWRSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PWRSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PWRSTS_A::_1
    }
}
impl core::ops::Deref for PWRSTS_R {
    type Target = crate::FieldReader<bool, PWRSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSTS` writer - "]
pub struct PWRSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC_PWR pin to low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWRSTS_A::_0)
    }
    #[doc = "SC_PWR pin to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWRSTS_A::_1)
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
#[doc = "SCRST Pin Signals\\nThis bit is the pin status of SC_RST\\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTSTS_A {
    #[doc = "0: SC_RST pin is low"]
    _0 = 0,
    #[doc = "1: SC_RST pin is high"]
    _1 = 1,
}
impl From<RSTSTS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTSTS` reader - SCRST Pin Signals\\nThis bit is the pin status of SC_RST\\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
pub struct RSTSTS_R(crate::FieldReader<bool, RSTSTS_A>);
impl RSTSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTSTS_A {
        match self.bits {
            false => RSTSTS_A::_0,
            true => RSTSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RSTSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RSTSTS_A::_1
    }
}
impl core::ops::Deref for RSTSTS_R {
    type Target = crate::FieldReader<bool, RSTSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSTS` writer - SCRST Pin Signals\\nThis bit is the pin status of SC_RST\\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
pub struct RSTSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SC_RST pin is low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTSTS_A::_0)
    }
    #[doc = "SC_RST pin is high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTSTS_A::_1)
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
#[doc = "SYNC Flag Indicator\\nDue to synchronization, software should check this bit when writing a new value to SC_PINCTL register.\\nNote: This bit is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: Synchronizing is completion, user can write new data to SC_PINCTL register"]
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
#[doc = "Field `SYNC` reader - SYNC Flag Indicator\\nDue to synchronization, software should check this bit when writing a new value to SC_PINCTL register.\\nNote: This bit is read only."]
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
#[doc = "Field `SYNC` writer - SYNC Flag Indicator\\nDue to synchronization, software should check this bit when writing a new value to SC_PINCTL register.\\nNote: This bit is read only."]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronizing is completion, user can write new data to SC_PINCTL register"]
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
impl R {
    #[doc = "Bit 0 - SC_PWREN Pin Signal\\nSoftware can set PWREN (SC_PINCTL\\[0\\]) and PWRINV (SC_PINCTL\\[11\\])to decide SC_PWR pin is in high or low level.\\nWrite this field to drive SC_PWR pin\\nRefer PWRINV (SC_PINCTL\\[11\\]) description for programming SC_PWR pin voltage level. \\nRead this field to get SC_PWR pin status.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SC_RST Pin Signal\\nThis bit is the pin status of SC_RST but user can drive SC_RST pin to high or low by setting this bit.\\nWrite this field to drive SC_RST pin.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn scrst(&self) -> SCRST_R {
        SCRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SC Clock Enable Bit \\nNote: When operating in activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn clkkeep(&self) -> CLKKEEP_R {
        CLKKEEP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SC Data Output Pin \\nThis bit is the pin status of SCDATOUT but user can drive SCDATOUT pin to high or low by setting this bit.\\nNote: When SC is at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when SC is in these modes."]
    #[inline(always)]
    pub fn scdout(&self) -> SCDOUT_R {
        SCDOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - SC_POW Pin Inverse\\nThis bit is used for inverse the SC_POW pin.\\nThere are four kinds of combination for SC_POW pin setting by PWRINV(SC_PINCTL\\[11\\]) and PWREN(SC_PINCTL\\[0\\]). PWRINV (SC_PINCTL\\[11\\]) is bit 1 and PWREN(SC_PINCTL\\[0\\]) is bit 0 for SC_POW_Pin as high or low voltage selection.\\nNote: Software must select PWRINV (SC_PINCTL\\[11\\]) before Smart Card is enabled by SCEN (SC_CTL\\[0\\])."]
    #[inline(always)]
    pub fn pwrinv(&self) -> PWRINV_R {
        PWRINV_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SC Data Pin Output Status \\nThis bit is the pin status of SCDATOUT \\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
    #[inline(always)]
    pub fn scdosts(&self) -> SCDOSTS_R {
        SCDOSTS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn datsts(&self) -> DATSTS_R {
        DATSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwrsts(&self) -> PWRSTS_R {
        PWRSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SCRST Pin Signals\\nThis bit is the pin status of SC_RST\\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
    #[inline(always)]
    pub fn rststs(&self) -> RSTSTS_R {
        RSTSTS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SYNC Flag Indicator\\nDue to synchronization, software should check this bit when writing a new value to SC_PINCTL register.\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SC_PWREN Pin Signal\\nSoftware can set PWREN (SC_PINCTL\\[0\\]) and PWRINV (SC_PINCTL\\[11\\])to decide SC_PWR pin is in high or low level.\\nWrite this field to drive SC_PWR pin\\nRefer PWRINV (SC_PINCTL\\[11\\]) description for programming SC_PWR pin voltage level. \\nRead this field to get SC_PWR pin status.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W {
        PWREN_W { w: self }
    }
    #[doc = "Bit 1 - SC_RST Pin Signal\\nThis bit is the pin status of SC_RST but user can drive SC_RST pin to high or low by setting this bit.\\nWrite this field to drive SC_RST pin.\\nNote: When operating at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn scrst(&mut self) -> SCRST_W {
        SCRST_W { w: self }
    }
    #[doc = "Bit 6 - SC Clock Enable Bit \\nNote: When operating in activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when operating in these modes."]
    #[inline(always)]
    pub fn clkkeep(&mut self) -> CLKKEEP_W {
        CLKKEEP_W { w: self }
    }
    #[doc = "Bit 9 - SC Data Output Pin \\nThis bit is the pin status of SCDATOUT but user can drive SCDATOUT pin to high or low by setting this bit.\\nNote: When SC is at activation, warm reset or deactivation mode, this bit will be changed automatically. So don't fill this field when SC is in these modes."]
    #[inline(always)]
    pub fn scdout(&mut self) -> SCDOUT_W {
        SCDOUT_W { w: self }
    }
    #[doc = "Bits 11:12 - SC_POW Pin Inverse\\nThis bit is used for inverse the SC_POW pin.\\nThere are four kinds of combination for SC_POW pin setting by PWRINV(SC_PINCTL\\[11\\]) and PWREN(SC_PINCTL\\[0\\]). PWRINV (SC_PINCTL\\[11\\]) is bit 1 and PWREN(SC_PINCTL\\[0\\]) is bit 0 for SC_POW_Pin as high or low voltage selection.\\nNote: Software must select PWRINV (SC_PINCTL\\[11\\]) before Smart Card is enabled by SCEN (SC_CTL\\[0\\])."]
    #[inline(always)]
    pub fn pwrinv(&mut self) -> PWRINV_W {
        PWRINV_W { w: self }
    }
    #[doc = "Bit 12 - SC Data Pin Output Status \\nThis bit is the pin status of SCDATOUT \\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
    #[inline(always)]
    pub fn scdosts(&mut self) -> SCDOSTS_W {
        SCDOSTS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn datsts(&mut self) -> DATSTS_W {
        DATSTS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwrsts(&mut self) -> PWRSTS_W {
        PWRSTS_W { w: self }
    }
    #[doc = "Bit 18 - SCRST Pin Signals\\nThis bit is the pin status of SC_RST\\nNote: When SC is operated at activation, warm reset or deactivation mode, this bit will be changed automatically. This bit is not allowed to program when SC is operated at these modes."]
    #[inline(always)]
    pub fn rststs(&mut self) -> RSTSTS_W {
        RSTSTS_W { w: self }
    }
    #[doc = "Bit 30 - SYNC Flag Indicator\\nDue to synchronization, software should check this bit when writing a new value to SC_PINCTL register.\\nNote: This bit is read only."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Pin Control State Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_pinctl](index.html) module"]
pub struct SC_PINCTL_SPEC;
impl crate::RegisterSpec for SC_PINCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_pinctl::R](R) reader structure"]
impl crate::Readable for SC_PINCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_pinctl::W](W) writer structure"]
impl crate::Writable for SC_PINCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_PINCTL to value 0"]
impl crate::Resettable for SC_PINCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
