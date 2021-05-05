#[doc = "Register `PDMA_CURSCAT8` reader"]
pub struct R(crate::R<PDMA_CURSCAT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CURSCAT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_CURSCAT8_SPEC>> for R {
    fn from(reader: crate::R<PDMA_CURSCAT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURADDR` reader - PDMA Current Description Address Register (Read Only)\\nThis field indicates a 32-bit current external description address of PDMA controller.\\nNote: This field is read only and only used for Scatter-Gather mode to indicate the current external description address."]
pub struct CURADDR_R(crate::FieldReader<u32, u32>);
impl CURADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PDMA Current Description Address Register (Read Only)\\nThis field indicates a 32-bit current external description address of PDMA controller.\\nNote: This field is read only and only used for Scatter-Gather mode to indicate the current external description address."]
    #[inline(always)]
    pub fn curaddr(&self) -> CURADDR_R {
        CURADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Current Scatter-gather Descriptor Table Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_curscat8](index.html) module"]
pub struct PDMA_CURSCAT8_SPEC;
impl crate::RegisterSpec for PDMA_CURSCAT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_curscat8::R](R) reader structure"]
impl crate::Readable for PDMA_CURSCAT8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDMA_CURSCAT8 to value 0"]
impl crate::Resettable for PDMA_CURSCAT8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
