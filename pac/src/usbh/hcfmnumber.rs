#[doc = "Register `HCFMNUMBER` reader"]
pub struct R(crate::R<HCFMNUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMNUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCFMNUMBER_SPEC>> for R {
    fn from(reader: crate::R<HCFMNUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FN` reader - Frame Number\\nThis 16-bit incrementing counter field is incremented coincident with the re-load of FR (HcFmRemaining\\[13:0\\]). The count rolls over from 'FFFFh' to '0h.'"]
pub struct FN_R(crate::FieldReader<u16, u16>);
impl FN_R {
    pub(crate) fn new(bits: u16) -> Self {
        FN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Number\\nThis 16-bit incrementing counter field is incremented coincident with the re-load of FR (HcFmRemaining\\[13:0\\]). The count rolls over from 'FFFFh' to '0h.'"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Host Controller Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmnumber](index.html) module"]
pub struct HCFMNUMBER_SPEC;
impl crate::RegisterSpec for HCFMNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfmnumber::R](R) reader structure"]
impl crate::Readable for HCFMNUMBER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCFMNUMBER to value 0"]
impl crate::Resettable for HCFMNUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
