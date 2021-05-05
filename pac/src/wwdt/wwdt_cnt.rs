#[doc = "Register `WWDT_CNT` reader"]
pub struct R(crate::R<WWDT_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDT_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WWDT_CNT_SPEC>> for R {
    fn from(reader: crate::R<WWDT_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTDAT` reader - WWDT Counter Value\\nCNTDAT will be updated continuously to monitor 6-bit WWDT down counter value."]
pub struct CNTDAT_R(crate::FieldReader<u8, u8>);
impl CNTDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - WWDT Counter Value\\nCNTDAT will be updated continuously to monitor 6-bit WWDT down counter value."]
    #[inline(always)]
    pub fn cntdat(&self) -> CNTDAT_R {
        CNTDAT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "WWDT Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdt_cnt](index.html) module"]
pub struct WWDT_CNT_SPEC;
impl crate::RegisterSpec for WWDT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdt_cnt::R](R) reader structure"]
impl crate::Readable for WWDT_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WWDT_CNT to value 0x3f"]
impl crate::Resettable for WWDT_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
