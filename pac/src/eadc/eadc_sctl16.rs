#[doc = "Register `EADC_SCTL16` reader"]
pub struct R(crate::R<EADC_SCTL16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_SCTL16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_SCTL16_SPEC>> for R {
    fn from(reader: crate::R<EADC_SCTL16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_SCTL16` writer"]
pub struct W(crate::W<EADC_SCTL16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_SCTL16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<EADC_SCTL16_SPEC>> for W {
    fn from(writer: crate::W<EADC_SCTL16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTSMPT` reader - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, SW can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
pub struct EXTSMPT_R(crate::FieldReader<u8, u8>);
impl EXTSMPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSMPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSMPT` writer - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, SW can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
pub struct EXTSMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSMPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, SW can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
    #[inline(always)]
    pub fn extsmpt(&self) -> EXTSMPT_R {
        EXTSMPT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ADC Sampling Time Extend\\nWhen A/D converting at high conversion rate, the sampling time of analog input voltage may not enough if input channel loading is heavy, SW can extend A/D sampling time after trigger source is coming to get enough sampling time.\\nThe range of start delay time is from 0~255 ADC clock."]
    #[inline(always)]
    pub fn extsmpt(&mut self) -> EXTSMPT_W {
        EXTSMPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample Module 16 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_sctl16](index.html) module"]
pub struct EADC_SCTL16_SPEC;
impl crate::RegisterSpec for EADC_SCTL16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_sctl16::R](R) reader structure"]
impl crate::Readable for EADC_SCTL16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_sctl16::W](W) writer structure"]
impl crate::Writable for EADC_SCTL16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_SCTL16 to value 0"]
impl crate::Resettable for EADC_SCTL16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
