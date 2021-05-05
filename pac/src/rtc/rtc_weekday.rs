#[doc = "Register `RTC_WEEKDAY` reader"]
pub struct R(crate::R<RTC_WEEKDAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WEEKDAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_WEEKDAY_SPEC>> for R {
    fn from(reader: crate::R<RTC_WEEKDAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WEEKDAY` writer"]
pub struct W(crate::W<RTC_WEEKDAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WEEKDAY_SPEC>;
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
impl core::convert::From<crate::W<RTC_WEEKDAY_SPEC>> for W {
    fn from(writer: crate::W<RTC_WEEKDAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Day of the Week Register\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WEEKDAY_A {
    #[doc = "0: Sunday"]
    _0 = 0,
    #[doc = "1: Monday"]
    _1 = 1,
    #[doc = "2: Tuesday"]
    _2 = 2,
    #[doc = "3: Wednesday"]
    _3 = 3,
    #[doc = "4: Thursday"]
    _4 = 4,
    #[doc = "5: Friday"]
    _5 = 5,
    #[doc = "6: Saturday"]
    _6 = 6,
    #[doc = "7: Reserved."]
    _7 = 7,
}
impl From<WEEKDAY_A> for u8 {
    #[inline(always)]
    fn from(variant: WEEKDAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WEEKDAY` reader - Day of the Week Register"]
pub struct WEEKDAY_R(crate::FieldReader<u8, WEEKDAY_A>);
impl WEEKDAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        WEEKDAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEEKDAY_A {
        match self.bits {
            0 => WEEKDAY_A::_0,
            1 => WEEKDAY_A::_1,
            2 => WEEKDAY_A::_2,
            3 => WEEKDAY_A::_3,
            4 => WEEKDAY_A::_4,
            5 => WEEKDAY_A::_5,
            6 => WEEKDAY_A::_6,
            7 => WEEKDAY_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WEEKDAY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WEEKDAY_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == WEEKDAY_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == WEEKDAY_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == WEEKDAY_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == WEEKDAY_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == WEEKDAY_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == WEEKDAY_A::_7
    }
}
impl core::ops::Deref for WEEKDAY_R {
    type Target = crate::FieldReader<u8, WEEKDAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEEKDAY` writer - Day of the Week Register"]
pub struct WEEKDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WEEKDAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WEEKDAY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_0)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_1)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_2)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_3)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_4)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_5)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_6)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(WEEKDAY_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Day of the Week Register"]
    #[inline(always)]
    pub fn weekday(&self) -> WEEKDAY_R {
        WEEKDAY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of the Week Register"]
    #[inline(always)]
    pub fn weekday(&mut self) -> WEEKDAY_W {
        WEEKDAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Day of the Week Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_weekday](index.html) module"]
pub struct RTC_WEEKDAY_SPEC;
impl crate::RegisterSpec for RTC_WEEKDAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_weekday::R](R) reader structure"]
impl crate::Readable for RTC_WEEKDAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_weekday::W](W) writer structure"]
impl crate::Writable for RTC_WEEKDAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WEEKDAY to value 0x06"]
impl crate::Resettable for RTC_WEEKDAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
