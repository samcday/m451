#[doc = "Register `RTC_TICK` reader"]
pub struct R(crate::R<RTC_TICK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TICK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_TICK_SPEC>> for R {
    fn from(reader: crate::R<RTC_TICK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TICK` writer"]
pub struct W(crate::W<RTC_TICK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TICK_SPEC>;
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
impl core::convert::From<crate::W<RTC_TICK_SPEC>> for W {
    fn from(writer: crate::W<RTC_TICK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Tick Register\\nThese bits are used to select RTC time tick period for Periodic Time Tick Interrupt request. \\nNote: This register can be read back after the RTC register access enable bit RWENF (RTC_RWEN\\[16\\]) is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TICK_A {
    #[doc = "0: Time tick is 1 second"]
    _0 = 0,
    #[doc = "1: Time tick is 1/2 second"]
    _1 = 1,
    #[doc = "2: Time tick is 1/4 second"]
    _2 = 2,
    #[doc = "3: Time tick is 1/8 second"]
    _3 = 3,
    #[doc = "4: Time tick is 1/16 second"]
    _4 = 4,
    #[doc = "5: Time tick is 1/32 second"]
    _5 = 5,
    #[doc = "6: Time tick is 1/64 second"]
    _6 = 6,
    #[doc = "7: Time tick is 1/28 second"]
    _7 = 7,
}
impl From<TICK_A> for u8 {
    #[inline(always)]
    fn from(variant: TICK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TICK` reader - Time Tick Register\\nThese bits are used to select RTC time tick period for Periodic Time Tick Interrupt request. \\nNote: This register can be read back after the RTC register access enable bit RWENF (RTC_RWEN\\[16\\]) is active."]
pub struct TICK_R(crate::FieldReader<u8, TICK_A>);
impl TICK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TICK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_A {
        match self.bits {
            0 => TICK_A::_0,
            1 => TICK_A::_1,
            2 => TICK_A::_2,
            3 => TICK_A::_3,
            4 => TICK_A::_4,
            5 => TICK_A::_5,
            6 => TICK_A::_6,
            7 => TICK_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TICK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TICK_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TICK_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TICK_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == TICK_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == TICK_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == TICK_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == TICK_A::_7
    }
}
impl core::ops::Deref for TICK_R {
    type Target = crate::FieldReader<u8, TICK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK` writer - Time Tick Register\\nThese bits are used to select RTC time tick period for Periodic Time Tick Interrupt request. \\nNote: This register can be read back after the RTC register access enable bit RWENF (RTC_RWEN\\[16\\]) is active."]
pub struct TICK_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Time tick is 1 second"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TICK_A::_0)
    }
    #[doc = "Time tick is 1/2 second"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TICK_A::_1)
    }
    #[doc = "Time tick is 1/4 second"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TICK_A::_2)
    }
    #[doc = "Time tick is 1/8 second"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TICK_A::_3)
    }
    #[doc = "Time tick is 1/16 second"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TICK_A::_4)
    }
    #[doc = "Time tick is 1/32 second"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TICK_A::_5)
    }
    #[doc = "Time tick is 1/64 second"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TICK_A::_6)
    }
    #[doc = "Time tick is 1/28 second"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TICK_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Time Tick Register\\nThese bits are used to select RTC time tick period for Periodic Time Tick Interrupt request. \\nNote: This register can be read back after the RTC register access enable bit RWENF (RTC_RWEN\\[16\\]) is active."]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Time Tick Register\\nThese bits are used to select RTC time tick period for Periodic Time Tick Interrupt request. \\nNote: This register can be read back after the RTC register access enable bit RWENF (RTC_RWEN\\[16\\]) is active."]
    #[inline(always)]
    pub fn tick(&mut self) -> TICK_W {
        TICK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Tick Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tick](index.html) module"]
pub struct RTC_TICK_SPEC;
impl crate::RegisterSpec for RTC_TICK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tick::R](R) reader structure"]
impl crate::Readable for RTC_TICK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tick::W](W) writer structure"]
impl crate::Writable for RTC_TICK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TICK to value 0"]
impl crate::Resettable for RTC_TICK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
