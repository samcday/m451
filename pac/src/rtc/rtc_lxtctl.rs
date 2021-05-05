#[doc = "Register `RTC_LXTCTL` reader"]
pub struct R(crate::R<RTC_LXTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_LXTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_LXTCTL_SPEC>> for R {
    fn from(reader: crate::R<RTC_LXTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_LXTCTL` writer"]
pub struct W(crate::W<RTC_LXTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_LXTCTL_SPEC>;
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
impl core::convert::From<crate::W<RTC_LXTCTL_SPEC>> for W {
    fn from(writer: crate::W<RTC_LXTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Backup Domain 32K Oscillator Enable Bit\\nThis bit controls 32 kHz oscillator on/off. User can set either LXTEN in RTC battery power domain or system manager control register CLK_PWRCTL\\[1\\]
(LXTEN) to enable 32 kHz oscillator. If this bit is set 1, X32 kHz oscillator keep running after system core power is turned off, if this bit is clear to 0, oscillator is turned off when system core power is turned off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTEN_A {
    #[doc = "0: Oscillator is Disabled"]
    _0 = 0,
    #[doc = "1: Oscillator is Enabled"]
    _1 = 1,
}
impl From<LXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTEN` reader - Backup Domain 32K Oscillator Enable Bit\\nThis bit controls 32 kHz oscillator on/off. User can set either LXTEN in RTC battery power domain or system manager control register CLK_PWRCTL\\[1\\]
(LXTEN) to enable 32 kHz oscillator. If this bit is set 1, X32 kHz oscillator keep running after system core power is turned off, if this bit is clear to 0, oscillator is turned off when system core power is turned off."]
pub struct LXTEN_R(crate::FieldReader<bool, LXTEN_A>);
impl LXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTEN_A {
        match self.bits {
            false => LXTEN_A::_0,
            true => LXTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTEN_A::_1
    }
}
impl core::ops::Deref for LXTEN_R {
    type Target = crate::FieldReader<bool, LXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTEN` writer - Backup Domain 32K Oscillator Enable Bit\\nThis bit controls 32 kHz oscillator on/off. User can set either LXTEN in RTC battery power domain or system manager control register CLK_PWRCTL\\[1\\]
(LXTEN) to enable 32 kHz oscillator. If this bit is set 1, X32 kHz oscillator keep running after system core power is turned off, if this bit is clear to 0, oscillator is turned off when system core power is turned off."]
pub struct LXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Oscillator is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTEN_A::_0)
    }
    #[doc = "Oscillator is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTEN_A::_1)
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
#[doc = "Oscillator Gain Option\\nUser can select oscillator gain according to crystal external loading and operating temperature range. The larger gain value corresponding to stronger driving capability and higher power consumption.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
    #[doc = "0: L0 mode"]
    _0 = 0,
    #[doc = "1: L1 mode"]
    _1 = 1,
    #[doc = "2: L2 mode"]
    _2 = 2,
    #[doc = "3: L3 mode"]
    _3 = 3,
    #[doc = "4: L4 mode"]
    _4 = 4,
    #[doc = "5: L5 mode"]
    _5 = 5,
    #[doc = "6: L6 mode"]
    _6 = 6,
    #[doc = "7: L7 mode (Default)"]
    _7 = 7,
}
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAIN` reader - Oscillator Gain Option\\nUser can select oscillator gain according to crystal external loading and operating temperature range. The larger gain value corresponding to stronger driving capability and higher power consumption."]
pub struct GAIN_R(crate::FieldReader<u8, GAIN_A>);
impl GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN_A {
        match self.bits {
            0 => GAIN_A::_0,
            1 => GAIN_A::_1,
            2 => GAIN_A::_2,
            3 => GAIN_A::_3,
            4 => GAIN_A::_4,
            5 => GAIN_A::_5,
            6 => GAIN_A::_6,
            7 => GAIN_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GAIN_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == GAIN_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == GAIN_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == GAIN_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == GAIN_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == GAIN_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == GAIN_A::_7
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u8, GAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain Option\\nUser can select oscillator gain according to crystal external loading and operating temperature range. The larger gain value corresponding to stronger driving capability and higher power consumption."]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "L0 mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAIN_A::_0)
    }
    #[doc = "L1 mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAIN_A::_1)
    }
    #[doc = "L2 mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(GAIN_A::_2)
    }
    #[doc = "L3 mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(GAIN_A::_3)
    }
    #[doc = "L4 mode"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(GAIN_A::_4)
    }
    #[doc = "L5 mode"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(GAIN_A::_5)
    }
    #[doc = "L6 mode"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(GAIN_A::_6)
    }
    #[doc = "L7 mode (Default)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(GAIN_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Backup Domain 32K Oscillator Enable Bit\\nThis bit controls 32 kHz oscillator on/off. User can set either LXTEN in RTC battery power domain or system manager control register CLK_PWRCTL\\[1\\]
(LXTEN) to enable 32 kHz oscillator. If this bit is set 1, X32 kHz oscillator keep running after system core power is turned off, if this bit is clear to 0, oscillator is turned off when system core power is turned off."]
    #[inline(always)]
    pub fn lxten(&self) -> LXTEN_R {
        LXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Oscillator Gain Option\\nUser can select oscillator gain according to crystal external loading and operating temperature range. The larger gain value corresponding to stronger driving capability and higher power consumption."]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Backup Domain 32K Oscillator Enable Bit\\nThis bit controls 32 kHz oscillator on/off. User can set either LXTEN in RTC battery power domain or system manager control register CLK_PWRCTL\\[1\\]
(LXTEN) to enable 32 kHz oscillator. If this bit is set 1, X32 kHz oscillator keep running after system core power is turned off, if this bit is clear to 0, oscillator is turned off when system core power is turned off."]
    #[inline(always)]
    pub fn lxten(&mut self) -> LXTEN_W {
        LXTEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Oscillator Gain Option\\nUser can select oscillator gain according to crystal external loading and operating temperature range. The larger gain value corresponding to stronger driving capability and higher power consumption."]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC 32.768 KHz Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_lxtctl](index.html) module"]
pub struct RTC_LXTCTL_SPEC;
impl crate::RegisterSpec for RTC_LXTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_lxtctl::R](R) reader structure"]
impl crate::Readable for RTC_LXTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_lxtctl::W](W) writer structure"]
impl crate::Writable for RTC_LXTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_LXTCTL to value 0x0e"]
impl crate::Resettable for RTC_LXTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}
