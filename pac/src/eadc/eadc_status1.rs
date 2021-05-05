#[doc = "Register `EADC_STATUS1` reader"]
pub struct R(crate::R<EADC_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_STATUS1_SPEC>> for R {
    fn from(reader: crate::R<EADC_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - EADC_DAT16~18 Data Valid Flag"]
pub struct VALID_R(crate::FieldReader<u8, u8>);
impl VALID_R {
    pub(crate) fn new(bits: u8) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OV` reader - EADC_DAT16~18 Overrun Flag"]
pub struct OV_R(crate::FieldReader<u8, u8>);
impl OV_R {
    pub(crate) fn new(bits: u8) -> Self {
        OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - EADC_DAT16~18 Data Valid Flag"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - EADC_DAT16~18 Overrun Flag"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "A/D Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_status1](index.html) module"]
pub struct EADC_STATUS1_SPEC;
impl crate::RegisterSpec for EADC_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_status1::R](R) reader structure"]
impl crate::Readable for EADC_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EADC_STATUS1 to value 0"]
impl crate::Resettable for EADC_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
