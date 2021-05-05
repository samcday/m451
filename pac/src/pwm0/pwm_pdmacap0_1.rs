#[doc = "Register `PWM_PDMACAP0_1` reader"]
pub struct R(crate::R<PWM_PDMACAP0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_PDMACAP0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_PDMACAP0_1_SPEC>> for R {
    fn from(reader: crate::R<PWM_PDMACAP0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPBUF` reader - PWM Capture PDMA Register (Read Only)\\nThis register is use as a buffer to transfer PWM capture rising or falling data to memory by PDMA."]
pub struct CAPBUF_R(crate::FieldReader<u16, u16>);
impl CAPBUF_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAPBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Capture PDMA Register (Read Only)\\nThis register is use as a buffer to transfer PWM capture rising or falling data to memory by PDMA."]
    #[inline(always)]
    pub fn capbuf(&self) -> CAPBUF_R {
        CAPBUF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM Capture Channel 01 PDMA Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_pdmacap0_1](index.html) module"]
pub struct PWM_PDMACAP0_1_SPEC;
impl crate::RegisterSpec for PWM_PDMACAP0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_pdmacap0_1::R](R) reader structure"]
impl crate::Readable for PWM_PDMACAP0_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_PDMACAP0_1 to value 0"]
impl crate::Resettable for PWM_PDMACAP0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
