#[doc = "Register `PWM_CMPBUF3` reader"]
pub struct R(crate::R<PWM_CMPBUF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CMPBUF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CMPBUF3_SPEC>> for R {
    fn from(reader: crate::R<PWM_CMPBUF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPBUF` reader - PWM Comparator Register Buffer (Read Only)\\nUsed as CMP active register."]
pub struct CMPBUF_R(crate::FieldReader<u16, u16>);
impl CMPBUF_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Comparator Register Buffer (Read Only)\\nUsed as CMP active register."]
    #[inline(always)]
    pub fn cmpbuf(&self) -> CMPBUF_R {
        CMPBUF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM CMPDAT3 Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpbuf3](index.html) module"]
pub struct PWM_CMPBUF3_SPEC;
impl crate::RegisterSpec for PWM_CMPBUF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cmpbuf3::R](R) reader structure"]
impl crate::Readable for PWM_CMPBUF3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_CMPBUF3 to value 0"]
impl crate::Resettable for PWM_CMPBUF3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
