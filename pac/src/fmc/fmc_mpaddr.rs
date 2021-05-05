#[doc = "Register `FMC_MPADDR` reader"]
pub struct R(crate::R<FMC_MPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_MPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_MPADDR_SPEC>> for R {
    fn from(reader: crate::R<FMC_MPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MPADDR` reader - ISP Multi-word Program Address\\nMPADDR is the address of ISP multi-word program operation when ISPGO flag is 1.\\nMPADDR will keep the final ISP address when ISP multi-word program is complete."]
pub struct MPADDR_R(crate::FieldReader<u32, u32>);
impl MPADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        MPADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Multi-word Program Address\\nMPADDR is the address of ISP multi-word program operation when ISPGO flag is 1.\\nMPADDR will keep the final ISP address when ISP multi-word program is complete."]
    #[inline(always)]
    pub fn mpaddr(&self) -> MPADDR_R {
        MPADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ISP Multi-program Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_mpaddr](index.html) module"]
pub struct FMC_MPADDR_SPEC;
impl crate::RegisterSpec for FMC_MPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_mpaddr::R](R) reader structure"]
impl crate::Readable for FMC_MPADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_MPADDR to value 0"]
impl crate::Resettable for FMC_MPADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
