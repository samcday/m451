#[doc = "Register `SYS_SRAM_ERRADDR` reader"]
pub struct R(crate::R<SYS_SRAM_ERRADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_ERRADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_SRAM_ERRADDR_SPEC>> for R {
    fn from(reader: crate::R<SYS_SRAM_ERRADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRADDR` reader - System SRAM Parity Error Address\\nThis register shows system SRAM parity error byte address."]
pub struct ERRADDR_R(crate::FieldReader<u32, u32>);
impl ERRADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ERRADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - System SRAM Parity Error Address\\nThis register shows system SRAM parity error byte address."]
    #[inline(always)]
    pub fn erraddr(&self) -> ERRADDR_R {
        ERRADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "System SRAM Parity Check Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_erraddr](index.html) module"]
pub struct SYS_SRAM_ERRADDR_SPEC;
impl crate::RegisterSpec for SYS_SRAM_ERRADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_erraddr::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_ERRADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_SRAM_ERRADDR to value 0"]
impl crate::Resettable for SYS_SRAM_ERRADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
