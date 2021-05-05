#[doc = "Register `DAC_DATOUT` reader"]
pub struct R(crate::R<DAC_DATOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DATOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_DATOUT_SPEC>> for R {
    fn from(reader: crate::R<DAC_DATOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATOUT` reader - DAC 12-bit Output Data\\nThese bits are current digital data for DAC output conversion.\\nIt is loaded from DAC_DAT register and user cannot write it directly."]
pub struct DATOUT_R(crate::FieldReader<u16, u16>);
impl DATOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC 12-bit Output Data\\nThese bits are current digital data for DAC output conversion.\\nIt is loaded from DAC_DAT register and user cannot write it directly."]
    #[inline(always)]
    pub fn datout(&self) -> DATOUT_R {
        DATOUT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_datout](index.html) module"]
pub struct DAC_DATOUT_SPEC;
impl crate::RegisterSpec for DAC_DATOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_datout::R](R) reader structure"]
impl crate::Readable for DAC_DATOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAC_DATOUT to value 0"]
impl crate::Resettable for DAC_DATOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
