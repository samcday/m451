#[doc = "Register `TIMER3_CAP` reader"]
pub struct R(crate::R<TIMER3_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER3_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER3_CAP_SPEC>> for R {
    fn from(reader: crate::R<TIMER3_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPDAT` reader - Timer Capture Data Register\\nWhen CAPEN (TIMERx_EXTCTL\\[3\\]) bit is set, CAPFUNCS (TIMERx_EXTCTL\\[4\\]) bit is 0, and a transition on Tx_EXT pin matched the CAPEDGE (TIMERx_EXTCTL\\[2:1\\]) setting, CAPIF (TIMERx_EINTSTS\\[0\\]) will set to 1 and the current timer counter value CNT (TIMERx_CNT\\[23:0\\]) will be auto-loaded into this CAPDAT field."]
pub struct CAPDAT_R(crate::FieldReader<u32, u32>);
impl CAPDAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CAPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPDAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer Capture Data Register\\nWhen CAPEN (TIMERx_EXTCTL\\[3\\]) bit is set, CAPFUNCS (TIMERx_EXTCTL\\[4\\]) bit is 0, and a transition on Tx_EXT pin matched the CAPEDGE (TIMERx_EXTCTL\\[2:1\\]) setting, CAPIF (TIMERx_EINTSTS\\[0\\]) will set to 1 and the current timer counter value CNT (TIMERx_CNT\\[23:0\\]) will be auto-loaded into this CAPDAT field."]
    #[inline(always)]
    pub fn capdat(&self) -> CAPDAT_R {
        CAPDAT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Timer3 Capture Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3_cap](index.html) module"]
pub struct TIMER3_CAP_SPEC;
impl crate::RegisterSpec for TIMER3_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer3_cap::R](R) reader structure"]
impl crate::Readable for TIMER3_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER3_CAP to value 0"]
impl crate::Resettable for TIMER3_CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
