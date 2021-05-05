#[doc = "Register `RTC_INIT` reader"]
pub struct R(crate::R<RTC_INIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_INIT_SPEC>> for R {
    fn from(reader: crate::R<RTC_INIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INIT` writer"]
pub struct W(crate::W<RTC_INIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INIT_SPEC>;
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
impl core::convert::From<crate::W<RTC_INIT_SPEC>> for W {
    fn from(writer: crate::W<RTC_INIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Active Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_ACTIVE_A {
    #[doc = "0: RTC is at reset state"]
    _0 = 0,
    #[doc = "1: RTC is at normal active state"]
    _1 = 1,
}
impl From<INIT_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT_ACTIVE` reader - RTC Active Status (Read Only)"]
pub struct INIT_ACTIVE_R(crate::FieldReader<bool, INIT_ACTIVE_A>);
impl INIT_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_ACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_ACTIVE_A {
        match self.bits {
            false => INIT_ACTIVE_A::_0,
            true => INIT_ACTIVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INIT_ACTIVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INIT_ACTIVE_A::_1
    }
}
impl core::ops::Deref for INIT_ACTIVE_R {
    type Target = crate::FieldReader<bool, INIT_ACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` reader - RTC Initiation\\nWhen RTC block is powered on, RTC is at reset state. User has to write a number (0x a5eb1357) to INIT to make RTC leaving reset state. Once the INIT is written as 0xa5eb1357, the RTC will be in un-reset state permanently.\\nThe INIT is a write-only field and read value will be always 0."]
pub struct INIT_R(crate::FieldReader<u32, u32>);
impl INIT_R {
    pub(crate) fn new(bits: u32) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - RTC Initiation\\nWhen RTC block is powered on, RTC is at reset state. User has to write a number (0x a5eb1357) to INIT to make RTC leaving reset state. Once the INIT is written as 0xa5eb1357, the RTC will be in un-reset state permanently.\\nThe INIT is a write-only field and read value will be always 0."]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Active Status (Read Only)"]
    #[inline(always)]
    pub fn init_active(&self) -> INIT_ACTIVE_R {
        INIT_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:31 - RTC Initiation\\nWhen RTC block is powered on, RTC is at reset state. User has to write a number (0x a5eb1357) to INIT to make RTC leaving reset state. Once the INIT is written as 0xa5eb1357, the RTC will be in un-reset state permanently.\\nThe INIT is a write-only field and read value will be always 0."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:31 - RTC Initiation\\nWhen RTC block is powered on, RTC is at reset state. User has to write a number (0x a5eb1357) to INIT to make RTC leaving reset state. Once the INIT is written as 0xa5eb1357, the RTC will be in un-reset state permanently.\\nThe INIT is a write-only field and read value will be always 0."]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Initiation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_init](index.html) module"]
pub struct RTC_INIT_SPEC;
impl crate::RegisterSpec for RTC_INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_init::R](R) reader structure"]
impl crate::Readable for RTC_INIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_init::W](W) writer structure"]
impl crate::Writable for RTC_INIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INIT to value 0"]
impl crate::Resettable for RTC_INIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
