#[doc = "Register `EADC_CURDAT` reader"]
pub struct R(crate::R<EADC_CURDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_CURDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_CURDAT_SPEC>> for R {
    fn from(reader: crate::R<EADC_CURDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURDAT` reader - ADC PDMA Current Transfer Data Register\\nThis is a read only register.\\nNOTE: After PDMA read this register, the VAILD of the shadow EADC_DAT register will be automatically cleared."]
pub struct CURDAT_R(crate::FieldReader<u32, u32>);
impl CURDAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURDAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - ADC PDMA Current Transfer Data Register\\nThis is a read only register.\\nNOTE: After PDMA read this register, the VAILD of the shadow EADC_DAT register will be automatically cleared."]
    #[inline(always)]
    pub fn curdat(&self) -> CURDAT_R {
        CURDAT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "EADC PDMA Current Transfer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_curdat](index.html) module"]
pub struct EADC_CURDAT_SPEC;
impl crate::RegisterSpec for EADC_CURDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_curdat::R](R) reader structure"]
impl crate::Readable for EADC_CURDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EADC_CURDAT to value 0"]
impl crate::Resettable for EADC_CURDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
