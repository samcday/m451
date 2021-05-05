#[doc = "Register `RTC_RWEN` reader"]
pub struct R(crate::R<RTC_RWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_RWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_RWEN_SPEC>> for R {
    fn from(reader: crate::R<RTC_RWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_RWEN` writer"]
pub struct W(crate::W<RTC_RWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_RWEN_SPEC>;
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
impl core::convert::From<crate::W<RTC_RWEN_SPEC>> for W {
    fn from(writer: crate::W<RTC_RWEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWEN` writer - RTC Register Access Enable Password (Write Only)\\nWriting 0xA965 to this register will enable RTC access and keep 1024 RTC clock."]
pub struct RWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "RTC Register Access Enable Flag (Read Only)\\nThis bit will be set after RTC_RWEN\\[15:0\\]
register is load a 0xA965, and be cleared automatically after 1024 RTC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWENF_A {
    #[doc = "0: RTC register read/write Disabled"]
    _0 = 0,
    #[doc = "1: RTC register read/write Enabled"]
    _1 = 1,
}
impl From<RWENF_A> for bool {
    #[inline(always)]
    fn from(variant: RWENF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWENF` reader - RTC Register Access Enable Flag (Read Only)\\nThis bit will be set after RTC_RWEN\\[15:0\\]
register is load a 0xA965, and be cleared automatically after 1024 RTC clock."]
pub struct RWENF_R(crate::FieldReader<bool, RWENF_A>);
impl RWENF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWENF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWENF_A {
        match self.bits {
            false => RWENF_A::_0,
            true => RWENF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RWENF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RWENF_A::_1
    }
}
impl core::ops::Deref for RWENF_R {
    type Target = crate::FieldReader<bool, RWENF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16 - RTC Register Access Enable Flag (Read Only)\\nThis bit will be set after RTC_RWEN\\[15:0\\]
register is load a 0xA965, and be cleared automatically after 1024 RTC clock."]
    #[inline(always)]
    pub fn rwenf(&self) -> RWENF_R {
        RWENF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Register Access Enable Password (Write Only)\\nWriting 0xA965 to this register will enable RTC access and keep 1024 RTC clock."]
    #[inline(always)]
    pub fn rwen(&mut self) -> RWEN_W {
        RWEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Access Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_rwen](index.html) module"]
pub struct RTC_RWEN_SPEC;
impl crate::RegisterSpec for RTC_RWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_rwen::R](R) reader structure"]
impl crate::Readable for RTC_RWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_rwen::W](W) writer structure"]
impl crate::Writable for RTC_RWEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_RWEN to value 0"]
impl crate::Resettable for RTC_RWEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
