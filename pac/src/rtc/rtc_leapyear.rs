#[doc = "Register `RTC_LEAPYEAR` reader"]
pub struct R(crate::R<RTC_LEAPYEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_LEAPYEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_LEAPYEAR_SPEC>> for R {
    fn from(reader: crate::R<RTC_LEAPYEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Leap Year Indication Register (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAPYEAR_A {
    #[doc = "0: This year is not a leap year"]
    _0 = 0,
    #[doc = "1: This year is leap year"]
    _1 = 1,
}
impl From<LEAPYEAR_A> for bool {
    #[inline(always)]
    fn from(variant: LEAPYEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAPYEAR` reader - Leap Year Indication Register (Read Only)"]
pub struct LEAPYEAR_R(crate::FieldReader<bool, LEAPYEAR_A>);
impl LEAPYEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEAPYEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAPYEAR_A {
        match self.bits {
            false => LEAPYEAR_A::_0,
            true => LEAPYEAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LEAPYEAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LEAPYEAR_A::_1
    }
}
impl core::ops::Deref for LEAPYEAR_R {
    type Target = crate::FieldReader<bool, LEAPYEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Leap Year Indication Register (Read Only)"]
    #[inline(always)]
    pub fn leapyear(&self) -> LEAPYEAR_R {
        LEAPYEAR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RTC Leap Year Indicator Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_leapyear](index.html) module"]
pub struct RTC_LEAPYEAR_SPEC;
impl crate::RegisterSpec for RTC_LEAPYEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_leapyear::R](R) reader structure"]
impl crate::Readable for RTC_LEAPYEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_LEAPYEAR to value 0"]
impl crate::Resettable for RTC_LEAPYEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
