#[doc = "Register `EADC_STATUS3` reader"]
pub struct R(crate::R<EADC_STATUS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_STATUS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_STATUS3_SPEC>> for R {
    fn from(reader: crate::R<EADC_STATUS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURSPL` reader - ADC Current Sample Module\\nThis register show the current ADC is controlled by which sample module control logic modules.\\nIf the ADC is Idle, this bit filed will set to 0x1F.\\nThis is a read only register."]
pub struct CURSPL_R(crate::FieldReader<u8, u8>);
impl CURSPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURSPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURSPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - ADC Current Sample Module\\nThis register show the current ADC is controlled by which sample module control logic modules.\\nIf the ADC is Idle, this bit filed will set to 0x1F.\\nThis is a read only register."]
    #[inline(always)]
    pub fn curspl(&self) -> CURSPL_R {
        CURSPL_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "A/D Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_status3](index.html) module"]
pub struct EADC_STATUS3_SPEC;
impl crate::RegisterSpec for EADC_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_status3::R](R) reader structure"]
impl crate::Readable for EADC_STATUS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EADC_STATUS3 to value 0x1f"]
impl crate::Resettable for EADC_STATUS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
