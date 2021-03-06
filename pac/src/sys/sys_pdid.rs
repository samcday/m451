#[doc = "Register `SYS_PDID` reader"]
pub struct R(crate::R<SYS_PDID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_PDID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_PDID_SPEC>> for R {
    fn from(reader: crate::R<SYS_PDID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDID` reader - Part Device Identification Number (Read Only)\\nThis register reflects device part number code. Software can read this register to identify which device is used."]
pub struct PDID_R(crate::FieldReader<u32, u32>);
impl PDID_R {
    pub(crate) fn new(bits: u32) -> Self {
        PDID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Device Identification Number (Read Only)\\nThis register reflects device part number code. Software can read this register to identify which device is used."]
    #[inline(always)]
    pub fn pdid(&self) -> PDID_R {
        PDID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Part Device Identification Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_pdid](index.html) module"]
pub struct SYS_PDID_SPEC;
impl crate::RegisterSpec for SYS_PDID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_pdid::R](R) reader structure"]
impl crate::Readable for SYS_PDID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_PDID to value 0"]
impl crate::Resettable for SYS_PDID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
