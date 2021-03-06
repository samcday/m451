#[doc = "Register `SC_TMRDAT0` reader"]
pub struct R(crate::R<SC_TMRDAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_TMRDAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_TMRDAT0_SPEC>> for R {
    fn from(reader: crate::R<SC_TMRDAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT0` reader - Timer0 Current Data Value (Read Only)\\nThis field indicates the current count values of timer0."]
pub struct CNT0_R(crate::FieldReader<u32, u32>);
impl CNT0_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer0 Current Data Value (Read Only)\\nThis field indicates the current count values of timer0."]
    #[inline(always)]
    pub fn cnt0(&self) -> CNT0_R {
        CNT0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "SC Timer Current Data Register A.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_tmrdat0](index.html) module"]
pub struct SC_TMRDAT0_SPEC;
impl crate::RegisterSpec for SC_TMRDAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_tmrdat0::R](R) reader structure"]
impl crate::Readable for SC_TMRDAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SC_TMRDAT0 to value 0x07ff"]
impl crate::Resettable for SC_TMRDAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
