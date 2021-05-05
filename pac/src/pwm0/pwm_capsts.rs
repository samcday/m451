#[doc = "Register `PWM_CAPSTS` reader"]
pub struct R(crate::R<PWM_CAPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CAPSTS_SPEC>> for R {
    fn from(reader: crate::R<PWM_CAPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRLIFOVn` reader - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1. Each bit n controls the corresponding PWM channel n.\\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
pub struct CRLIFOVN_R(crate::FieldReader<u8, u8>);
impl CRLIFOVN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRLIFOVN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRLIFOVN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFOVn` reader - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1. Each bit n controls the corresponding PWM channel n.\\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
pub struct CFLIFOVN_R(crate::FieldReader<u8, u8>);
impl CFLIFOVN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFLIFOVN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFLIFOVN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Capture Rising Latch Interrupt Flag Overrun Status (Read Only)\\nThis flag indicates if rising latch happened when the corresponding CRLIF is 1. Each bit n controls the corresponding PWM channel n.\\nNote: This bit will be cleared automatically when user clear corresponding CRLIF."]
    #[inline(always)]
    pub fn crlifovn(&self) -> CRLIFOVN_R {
        CRLIFOVN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Capture Falling Latch Interrupt Flag Overrun Status (Read Only)\\nThis flag indicates if falling latch happened when the corresponding CFLIF is 1. Each bit n controls the corresponding PWM channel n.\\nNote: This bit will be cleared automatically when user clear corresponding CFLIF."]
    #[inline(always)]
    pub fn cflifovn(&self) -> CFLIFOVN_R {
        CFLIFOVN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[doc = "PWM Capture Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capsts](index.html) module"]
pub struct PWM_CAPSTS_SPEC;
impl crate::RegisterSpec for PWM_CAPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capsts::R](R) reader structure"]
impl crate::Readable for PWM_CAPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_CAPSTS to value 0"]
impl crate::Resettable for PWM_CAPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
