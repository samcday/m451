#[doc = "Register `PWM_FTCBUF4_5` reader"]
pub struct R(crate::R<PWM_FTCBUF4_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FTCBUF4_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_FTCBUF4_5_SPEC>> for R {
    fn from(reader: crate::R<PWM_FTCBUF4_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FTCMPBUF` reader - PWM FTCMPDAT Buffer (Read Only)\\nUsed as FTCMPDAT active register."]
pub struct FTCMPBUF_R(crate::FieldReader<u16, u16>);
impl FTCMPBUF_R {
    pub(crate) fn new(bits: u16) -> Self {
        FTCMPBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTCMPBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM FTCMPDAT Buffer (Read Only)\\nUsed as FTCMPDAT active register."]
    #[inline(always)]
    pub fn ftcmpbuf(&self) -> FTCMPBUF_R {
        FTCMPBUF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM FTCMPDAT4_5 Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ftcbuf4_5](index.html) module"]
pub struct PWM_FTCBUF4_5_SPEC;
impl crate::RegisterSpec for PWM_FTCBUF4_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ftcbuf4_5::R](R) reader structure"]
impl crate::Readable for PWM_FTCBUF4_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_FTCBUF4_5 to value 0"]
impl crate::Resettable for PWM_FTCBUF4_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
