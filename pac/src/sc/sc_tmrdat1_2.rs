#[doc = "Register `SC_TMRDAT1_2` reader"]
pub struct R(crate::R<SC_TMRDAT1_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_TMRDAT1_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_TMRDAT1_2_SPEC>> for R {
    fn from(reader: crate::R<SC_TMRDAT1_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT1` reader - Timer1 Current Data Value (Read Only)\\nThis field indicates the current count values of timer1."]
pub struct CNT1_R(crate::FieldReader<u8, u8>);
impl CNT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT2` reader - Timer2 Current Data Value (Read Only)\\nThis field indicates the current count values of timer2."]
pub struct CNT2_R(crate::FieldReader<u8, u8>);
impl CNT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer1 Current Data Value (Read Only)\\nThis field indicates the current count values of timer1."]
    #[inline(always)]
    pub fn cnt1(&self) -> CNT1_R {
        CNT1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer2 Current Data Value (Read Only)\\nThis field indicates the current count values of timer2."]
    #[inline(always)]
    pub fn cnt2(&self) -> CNT2_R {
        CNT2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "SC Timer Current Data Register B.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_tmrdat1_2](index.html) module"]
pub struct SC_TMRDAT1_2_SPEC;
impl crate::RegisterSpec for SC_TMRDAT1_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_tmrdat1_2::R](R) reader structure"]
impl crate::Readable for SC_TMRDAT1_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SC_TMRDAT1_2 to value 0x7f7f"]
impl crate::Resettable for SC_TMRDAT1_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f7f
    }
}
