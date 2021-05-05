#[doc = "Register `FMC_DFBA` reader"]
pub struct R(crate::R<FMC_DFBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_DFBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_DFBA_SPEC>> for R {
    fn from(reader: crate::R<FMC_DFBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFBA` reader - Data Flash Base Address\\nThis register indicates Data Flash start address. It is a read only register.\\nThe Data Flash is shared with APROM. the content of this register is loaded from CONFIG1"]
pub struct DFBA_R(crate::FieldReader<u32, u32>);
impl DFBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DFBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Flash Base Address\\nThis register indicates Data Flash start address. It is a read only register.\\nThe Data Flash is shared with APROM. the content of this register is loaded from CONFIG1"]
    #[inline(always)]
    pub fn dfba(&self) -> DFBA_R {
        DFBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Data Flash Base Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_dfba](index.html) module"]
pub struct FMC_DFBA_SPEC;
impl crate::RegisterSpec for FMC_DFBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_dfba::R](R) reader structure"]
impl crate::Readable for FMC_DFBA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_DFBA to value 0"]
impl crate::Resettable for FMC_DFBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
