#[doc = "Register `PWM_PBUF0` reader"]
pub struct R(crate::R<PWM_PBUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_PBUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_PBUF0_SPEC>> for R {
    fn from(reader: crate::R<PWM_PBUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PBUF` reader - PWM Period Register Buffer (Read Only)\\nUsed as PERIOD active register."]
pub struct PBUF_R(crate::FieldReader<u16, u16>);
impl PBUF_R {
    pub(crate) fn new(bits: u16) -> Self {
        PBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Period Register Buffer (Read Only)\\nUsed as PERIOD active register."]
    #[inline(always)]
    pub fn pbuf(&self) -> PBUF_R {
        PBUF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM PERIOD0 Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_pbuf0](index.html) module"]
pub struct PWM_PBUF0_SPEC;
impl crate::RegisterSpec for PWM_PBUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_pbuf0::R](R) reader structure"]
impl crate::Readable for PWM_PBUF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_PBUF0 to value 0"]
impl crate::Resettable for PWM_PBUF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
