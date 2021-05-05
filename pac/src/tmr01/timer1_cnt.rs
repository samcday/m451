#[doc = "Register `TIMER1_CNT` reader"]
pub struct R(crate::R<TIMER1_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER1_CNT_SPEC>> for R {
    fn from(reader: crate::R<TIMER1_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Timer Data Register\\nIf EXTCNTEN (TIMERx_CTL\\[24\\]
) is 0, user can read CNT value for getting current 24- bit counter value .\\nIf EXTCNTEN (TIMERx_CTL\\[24\\]
) is 1, user can read CNT value for getting current 24- bit event input counter value."]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer Data Register\\nIf EXTCNTEN (TIMERx_CTL\\[24\\]
) is 0, user can read CNT value for getting current 24- bit counter value .\\nIf EXTCNTEN (TIMERx_CTL\\[24\\]
) is 1, user can read CNT value for getting current 24- bit event input counter value."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Timer1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_cnt](index.html) module"]
pub struct TIMER1_CNT_SPEC;
impl crate::RegisterSpec for TIMER1_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_cnt::R](R) reader structure"]
impl crate::Readable for TIMER1_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER1_CNT to value 0"]
impl crate::Resettable for TIMER1_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
