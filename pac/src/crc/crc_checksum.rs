#[doc = "Register `CRC_CHECKSUM` reader"]
pub struct R(crate::R<CRC_CHECKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CHECKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_CHECKSUM_SPEC>> for R {
    fn from(reader: crate::R<CRC_CHECKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHECKSUM` reader - CRC Checksum Results\\nThis field indicates the CRC checksum result."]
pub struct CHECKSUM_R(crate::FieldReader<u32, u32>);
impl CHECKSUM_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHECKSUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECKSUM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Checksum Results\\nThis field indicates the CRC checksum result."]
    #[inline(always)]
    pub fn checksum(&self) -> CHECKSUM_R {
        CHECKSUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "CRC Checksum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_checksum](index.html) module"]
pub struct CRC_CHECKSUM_SPEC;
impl crate::RegisterSpec for CRC_CHECKSUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_checksum::R](R) reader structure"]
impl crate::Readable for CRC_CHECKSUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRC_CHECKSUM to value 0"]
impl crate::Resettable for CRC_CHECKSUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
