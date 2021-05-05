#[doc = "Register `RTC_SPRCTL` reader"]
pub struct R(crate::R<RTC_SPRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_SPRCTL_SPEC>> for R {
    fn from(reader: crate::R<RTC_SPRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SPRCTL` writer"]
pub struct W(crate::W<RTC_SPRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPRCTL_SPEC>;
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
impl core::convert::From<crate::W<RTC_SPRCTL_SPEC>> for W {
    fn from(writer: crate::W<RTC_SPRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Snoop Detection Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNPDEN_A {
    #[doc = "0: TAMPER pin detection is Disabled"]
    _0 = 0,
    #[doc = "1: TAMPER pin detection is Enabled"]
    _1 = 1,
}
impl From<SNPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNPDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNPDEN` reader - Snoop Detection Enable Bit"]
pub struct SNPDEN_R(crate::FieldReader<bool, SNPDEN_A>);
impl SNPDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNPDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNPDEN_A {
        match self.bits {
            false => SNPDEN_A::_0,
            true => SNPDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SNPDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SNPDEN_A::_1
    }
}
impl core::ops::Deref for SNPDEN_R {
    type Target = crate::FieldReader<bool, SNPDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNPDEN` writer - Snoop Detection Enable Bit"]
pub struct SNPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNPDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TAMPER pin detection is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNPDEN_A::_0)
    }
    #[doc = "TAMPER pin detection is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNPDEN_A::_1)
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
#[doc = "Snoop Detection Level\\nThis bit controls TAMPER detect event is high level/rising edge or low level/falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNPTYPE0_A {
    #[doc = "0: Low level/Falling edge detection"]
    _0 = 0,
    #[doc = "1: High level/Rising edge detection."]
    _1 = 1,
}
impl From<SNPTYPE0_A> for bool {
    #[inline(always)]
    fn from(variant: SNPTYPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNPTYPE0` reader - Snoop Detection Level\\nThis bit controls TAMPER detect event is high level/rising edge or low level/falling edge."]
pub struct SNPTYPE0_R(crate::FieldReader<bool, SNPTYPE0_A>);
impl SNPTYPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNPTYPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNPTYPE0_A {
        match self.bits {
            false => SNPTYPE0_A::_0,
            true => SNPTYPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SNPTYPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SNPTYPE0_A::_1
    }
}
impl core::ops::Deref for SNPTYPE0_R {
    type Target = crate::FieldReader<bool, SNPTYPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNPTYPE0` writer - Snoop Detection Level\\nThis bit controls TAMPER detect event is high level/rising edge or low level/falling edge."]
pub struct SNPTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNPTYPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level/Falling edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNPTYPE0_A::_0)
    }
    #[doc = "High level/Rising edge detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNPTYPE0_A::_1)
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
#[doc = "Spare Register Enable Bit\\nNote: When spare register is disabled, RTC_SPR0 ~ RTC_SPR19 cannot be accessed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRRWEN_A {
    #[doc = "0: Spare register is Disabled"]
    _0 = 0,
    #[doc = "1: Spare register is Enabled"]
    _1 = 1,
}
impl From<SPRRWEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPRRWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRRWEN` reader - Spare Register Enable Bit\\nNote: When spare register is disabled, RTC_SPR0 ~ RTC_SPR19 cannot be accessed."]
pub struct SPRRWEN_R(crate::FieldReader<bool, SPRRWEN_A>);
impl SPRRWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPRRWEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRRWEN_A {
        match self.bits {
            false => SPRRWEN_A::_0,
            true => SPRRWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPRRWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPRRWEN_A::_1
    }
}
impl core::ops::Deref for SPRRWEN_R {
    type Target = crate::FieldReader<bool, SPRRWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPRRWEN` writer - Spare Register Enable Bit\\nNote: When spare register is disabled, RTC_SPR0 ~ RTC_SPR19 cannot be accessed."]
pub struct SPRRWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRRWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPRRWEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Spare register is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRRWEN_A::_0)
    }
    #[doc = "Spare register is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRRWEN_A::_1)
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
#[doc = "Snoop Detection Mode\\nThis bit controls TAMPER pin is edge or level detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNPTYPE1_A {
    #[doc = "0: Level detection"]
    _0 = 0,
    #[doc = "1: Edge detection"]
    _1 = 1,
}
impl From<SNPTYPE1_A> for bool {
    #[inline(always)]
    fn from(variant: SNPTYPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNPTYPE1` reader - Snoop Detection Mode\\nThis bit controls TAMPER pin is edge or level detection"]
pub struct SNPTYPE1_R(crate::FieldReader<bool, SNPTYPE1_A>);
impl SNPTYPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNPTYPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNPTYPE1_A {
        match self.bits {
            false => SNPTYPE1_A::_0,
            true => SNPTYPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SNPTYPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SNPTYPE1_A::_1
    }
}
impl core::ops::Deref for SNPTYPE1_R {
    type Target = crate::FieldReader<bool, SNPTYPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNPTYPE1` writer - Snoop Detection Mode\\nThis bit controls TAMPER pin is edge or level detection"]
pub struct SNPTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPTYPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNPTYPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Level detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNPTYPE1_A::_0)
    }
    #[doc = "Edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNPTYPE1_A::_1)
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
#[doc = "SPR Clear Flag \\nThis bit indicates if the RTC_SPR0 ~RTC_SPR19 content is cleared when specify snoop event is detected.\\nWrites 1 to clear this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRCSTS_A {
    #[doc = "0: Spare register content is not cleared"]
    _0 = 0,
    #[doc = "1: Spare register content is cleared"]
    _1 = 1,
}
impl From<SPRCSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SPRCSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRCSTS` reader - SPR Clear Flag \\nThis bit indicates if the RTC_SPR0 ~RTC_SPR19 content is cleared when specify snoop event is detected.\\nWrites 1 to clear this bit."]
pub struct SPRCSTS_R(crate::FieldReader<bool, SPRCSTS_A>);
impl SPRCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPRCSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRCSTS_A {
        match self.bits {
            false => SPRCSTS_A::_0,
            true => SPRCSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPRCSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPRCSTS_A::_1
    }
}
impl core::ops::Deref for SPRCSTS_R {
    type Target = crate::FieldReader<bool, SPRCSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPRCSTS` writer - SPR Clear Flag \\nThis bit indicates if the RTC_SPR0 ~RTC_SPR19 content is cleared when specify snoop event is detected.\\nWrites 1 to clear this bit."]
pub struct SPRCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRCSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPRCSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Spare register content is not cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRCSTS_A::_0)
    }
    #[doc = "Spare register content is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRCSTS_A::_1)
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
#[doc = "SPR Register Ready\\nThis bit indicates if the registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are ready to be accessed.\\nAfter user writing registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19, read this bit to check if these registers are updated done is necessary.\\nNote: This bit is read only and any write to it won't take any effect.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRRWRDY_A {
    #[doc = "0: RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 updating is in progress"]
    _0 = 0,
    #[doc = "1: RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are updated done and ready to be accessed"]
    _1 = 1,
}
impl From<SPRRWRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SPRRWRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRRWRDY` reader - SPR Register Ready\\nThis bit indicates if the registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are ready to be accessed.\\nAfter user writing registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19, read this bit to check if these registers are updated done is necessary.\\nNote: This bit is read only and any write to it won't take any effect."]
pub struct SPRRWRDY_R(crate::FieldReader<bool, SPRRWRDY_A>);
impl SPRRWRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPRRWRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRRWRDY_A {
        match self.bits {
            false => SPRRWRDY_A::_0,
            true => SPRRWRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPRRWRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPRRWRDY_A::_1
    }
}
impl core::ops::Deref for SPRRWRDY_R {
    type Target = crate::FieldReader<bool, SPRRWRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPRRWRDY` writer - SPR Register Ready\\nThis bit indicates if the registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are ready to be accessed.\\nAfter user writing registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19, read this bit to check if these registers are updated done is necessary.\\nNote: This bit is read only and any write to it won't take any effect."]
pub struct SPRRWRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRRWRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPRRWRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 updating is in progress"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRRWRDY_A::_0)
    }
    #[doc = "RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are updated done and ready to be accessed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRRWRDY_A::_1)
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
impl R {
    #[doc = "Bit 0 - Snoop Detection Enable Bit"]
    #[inline(always)]
    pub fn snpden(&self) -> SNPDEN_R {
        SNPDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Snoop Detection Level\\nThis bit controls TAMPER detect event is high level/rising edge or low level/falling edge."]
    #[inline(always)]
    pub fn snptype0(&self) -> SNPTYPE0_R {
        SNPTYPE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Spare Register Enable Bit\\nNote: When spare register is disabled, RTC_SPR0 ~ RTC_SPR19 cannot be accessed."]
    #[inline(always)]
    pub fn sprrwen(&self) -> SPRRWEN_R {
        SPRRWEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Snoop Detection Mode\\nThis bit controls TAMPER pin is edge or level detection"]
    #[inline(always)]
    pub fn snptype1(&self) -> SNPTYPE1_R {
        SNPTYPE1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPR Clear Flag \\nThis bit indicates if the RTC_SPR0 ~RTC_SPR19 content is cleared when specify snoop event is detected.\\nWrites 1 to clear this bit."]
    #[inline(always)]
    pub fn sprcsts(&self) -> SPRCSTS_R {
        SPRCSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPR Register Ready\\nThis bit indicates if the registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are ready to be accessed.\\nAfter user writing registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19, read this bit to check if these registers are updated done is necessary.\\nNote: This bit is read only and any write to it won't take any effect."]
    #[inline(always)]
    pub fn sprrwrdy(&self) -> SPRRWRDY_R {
        SPRRWRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Snoop Detection Enable Bit"]
    #[inline(always)]
    pub fn snpden(&mut self) -> SNPDEN_W {
        SNPDEN_W { w: self }
    }
    #[doc = "Bit 1 - Snoop Detection Level\\nThis bit controls TAMPER detect event is high level/rising edge or low level/falling edge."]
    #[inline(always)]
    pub fn snptype0(&mut self) -> SNPTYPE0_W {
        SNPTYPE0_W { w: self }
    }
    #[doc = "Bit 2 - Spare Register Enable Bit\\nNote: When spare register is disabled, RTC_SPR0 ~ RTC_SPR19 cannot be accessed."]
    #[inline(always)]
    pub fn sprrwen(&mut self) -> SPRRWEN_W {
        SPRRWEN_W { w: self }
    }
    #[doc = "Bit 3 - Snoop Detection Mode\\nThis bit controls TAMPER pin is edge or level detection"]
    #[inline(always)]
    pub fn snptype1(&mut self) -> SNPTYPE1_W {
        SNPTYPE1_W { w: self }
    }
    #[doc = "Bit 5 - SPR Clear Flag \\nThis bit indicates if the RTC_SPR0 ~RTC_SPR19 content is cleared when specify snoop event is detected.\\nWrites 1 to clear this bit."]
    #[inline(always)]
    pub fn sprcsts(&mut self) -> SPRCSTS_W {
        SPRCSTS_W { w: self }
    }
    #[doc = "Bit 7 - SPR Register Ready\\nThis bit indicates if the registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19 are ready to be accessed.\\nAfter user writing registers RTC_SPRCTL, RTC_SPR0 ~ RTC_SPR19, read this bit to check if these registers are updated done is necessary.\\nNote: This bit is read only and any write to it won't take any effect."]
    #[inline(always)]
    pub fn sprrwrdy(&mut self) -> SPRRWRDY_W {
        SPRRWRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Spare Functional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sprctl](index.html) module"]
pub struct RTC_SPRCTL_SPEC;
impl crate::RegisterSpec for RTC_SPRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_sprctl::R](R) reader structure"]
impl crate::Readable for RTC_SPRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_sprctl::W](W) writer structure"]
impl crate::Writable for RTC_SPRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SPRCTL to value 0x80"]
impl crate::Resettable for RTC_SPRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
