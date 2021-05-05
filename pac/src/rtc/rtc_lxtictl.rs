#[doc = "Register `RTC_LXTICTL` reader"]
pub struct R(crate::R<RTC_LXTICTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_LXTICTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_LXTICTL_SPEC>> for R {
    fn from(reader: crate::R<RTC_LXTICTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_LXTICTL` writer"]
pub struct W(crate::W<RTC_LXTICTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_LXTICTL_SPEC>;
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
impl core::convert::From<crate::W<RTC_LXTICTL_SPEC>> for W {
    fn from(writer: crate::W<RTC_LXTICTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IO Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: X32KI (PF.1) is input only mode, without pull-up resistor"]
    _0 = 0,
    #[doc = "1: X32KI (PF.1) is output push pull mode"]
    _1 = 1,
    #[doc = "2: X32KI (PF.1) is open drain mode"]
    _2 = 2,
    #[doc = "3: X32KI (PF.1) is input only mode with internal pull up"]
    _3 = 3,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMODE` reader - IO Operation Mode"]
pub struct OPMODE_R(crate::FieldReader<u8, OPMODE_A>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPMODE_A {
        match self.bits {
            0 => OPMODE_A::_0,
            1 => OPMODE_A::_1,
            2 => OPMODE_A::_2,
            3 => OPMODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OPMODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == OPMODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == OPMODE_A::_3
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, OPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - IO Operation Mode"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "X32KI (PF.1) is input only mode, without pull-up resistor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPMODE_A::_0)
    }
    #[doc = "X32KI (PF.1) is output push pull mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPMODE_A::_1)
    }
    #[doc = "X32KI (PF.1) is open drain mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(OPMODE_A::_2)
    }
    #[doc = "X32KI (PF.1) is input only mode with internal pull up"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(OPMODE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "IO Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT_A {
    #[doc = "0: X32KI (PF.1) output low"]
    _0 = 0,
    #[doc = "1: X32KI (PF.1) output high"]
    _1 = 1,
}
impl From<DOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT` reader - IO Output Data"]
pub struct DOUT_R(crate::FieldReader<bool, DOUT_A>);
impl DOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_A {
        match self.bits {
            false => DOUT_A::_0,
            true => DOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOUT_A::_1
    }
}
impl core::ops::Deref for DOUT_R {
    type Target = crate::FieldReader<bool, DOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT` writer - IO Output Data"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "X32KI (PF.1) output low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUT_A::_0)
    }
    #[doc = "X32KI (PF.1) output high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUT_A::_1)
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
#[doc = "IO Pin State Backup Selection\\nWhen low speed 32 kHz oscillator is disabled, X32KI (PF.1) pin can be used as GPIO function. User can program CTLSEL bit to decide X32KI (PF.1) I/O function is controlled by system power domain GPIO module or VBAT power domain RTC_LXTICTL control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLSEL_A {
    #[doc = "0: X32KI (PF.1) pin I/O function is controlled by GPIO module. It becomes floating state when system power is turned off"]
    _0 = 0,
    #[doc = "1: X32KI (PF.1) pin I/O function is controlled by VBAT power domain, X32KI (PF.1) pin function and I/O status are controlled by OPMODE\\[1:0\\]
and DOUT after CTLSEL it set to 1. I/O pin keeps the previous state after system power is turned off"]
    _1 = 1,
}
impl From<CTLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CTLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLSEL` reader - IO Pin State Backup Selection\\nWhen low speed 32 kHz oscillator is disabled, X32KI (PF.1) pin can be used as GPIO function. User can program CTLSEL bit to decide X32KI (PF.1) I/O function is controlled by system power domain GPIO module or VBAT power domain RTC_LXTICTL control register."]
pub struct CTLSEL_R(crate::FieldReader<bool, CTLSEL_A>);
impl CTLSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLSEL_A {
        match self.bits {
            false => CTLSEL_A::_0,
            true => CTLSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTLSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTLSEL_A::_1
    }
}
impl core::ops::Deref for CTLSEL_R {
    type Target = crate::FieldReader<bool, CTLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLSEL` writer - IO Pin State Backup Selection\\nWhen low speed 32 kHz oscillator is disabled, X32KI (PF.1) pin can be used as GPIO function. User can program CTLSEL bit to decide X32KI (PF.1) I/O function is controlled by system power domain GPIO module or VBAT power domain RTC_LXTICTL control register."]
pub struct CTLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "X32KI (PF.1) pin I/O function is controlled by GPIO module. It becomes floating state when system power is turned off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTLSEL_A::_0)
    }
    #[doc = "X32KI (PF.1) pin I/O function is controlled by VBAT power domain, X32KI (PF.1) pin function and I/O status are controlled by OPMODE\\[1:0\\]
and DOUT after CTLSEL it set to 1. I/O pin keeps the previous state after system power is turned off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTLSEL_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - IO Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - IO Output Data"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO Pin State Backup Selection\\nWhen low speed 32 kHz oscillator is disabled, X32KI (PF.1) pin can be used as GPIO function. User can program CTLSEL bit to decide X32KI (PF.1) I/O function is controlled by system power domain GPIO module or VBAT power domain RTC_LXTICTL control register."]
    #[inline(always)]
    pub fn ctlsel(&self) -> CTLSEL_R {
        CTLSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - IO Operation Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bit 2 - IO Output Data"]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W { w: self }
    }
    #[doc = "Bit 3 - IO Pin State Backup Selection\\nWhen low speed 32 kHz oscillator is disabled, X32KI (PF.1) pin can be used as GPIO function. User can program CTLSEL bit to decide X32KI (PF.1) I/O function is controlled by system power domain GPIO module or VBAT power domain RTC_LXTICTL control register."]
    #[inline(always)]
    pub fn ctlsel(&mut self) -> CTLSEL_W {
        CTLSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "X32KI Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_lxtictl](index.html) module"]
pub struct RTC_LXTICTL_SPEC;
impl crate::RegisterSpec for RTC_LXTICTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_lxtictl::R](R) reader structure"]
impl crate::Readable for RTC_LXTICTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_lxtictl::W](W) writer structure"]
impl crate::Writable for RTC_LXTICTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_LXTICTL to value 0"]
impl crate::Resettable for RTC_LXTICTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
