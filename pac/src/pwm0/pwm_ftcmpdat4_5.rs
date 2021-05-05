#[doc = "Register `PWM_FTCMPDAT4_5` reader"]
pub struct R(crate::R<PWM_FTCMPDAT4_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FTCMPDAT4_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_FTCMPDAT4_5_SPEC>> for R {
    fn from(reader: crate::R<PWM_FTCMPDAT4_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FTCMPDAT4_5` writer"]
pub struct W(crate::W<PWM_FTCMPDAT4_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FTCMPDAT4_5_SPEC>;
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
impl core::convert::From<crate::W<PWM_FTCMPDAT4_5_SPEC>> for W {
    fn from(writer: crate::W<PWM_FTCMPDAT4_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTCMP` reader - PWM Free Trigger Compare Register\\nFTCMP use to compare with even CNTR to trigger EADC. FTCMPDAT0, 2, 4 corresponding complementary pairs PWM_CH0and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
pub struct FTCMP_R(crate::FieldReader<u16, u16>);
impl FTCMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        FTCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTCMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTCMP` writer - PWM Free Trigger Compare Register\\nFTCMP use to compare with even CNTR to trigger EADC. FTCMPDAT0, 2, 4 corresponding complementary pairs PWM_CH0and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
pub struct FTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FTCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Free Trigger Compare Register\\nFTCMP use to compare with even CNTR to trigger EADC. FTCMPDAT0, 2, 4 corresponding complementary pairs PWM_CH0and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
    #[inline(always)]
    pub fn ftcmp(&self) -> FTCMP_R {
        FTCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Free Trigger Compare Register\\nFTCMP use to compare with even CNTR to trigger EADC. FTCMPDAT0, 2, 4 corresponding complementary pairs PWM_CH0and PWM_CH1, PWM_CH2 and PWM_CH3, PWM_CH4 and PWM_CH5."]
    #[inline(always)]
    pub fn ftcmp(&mut self) -> FTCMP_W {
        FTCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Free Trigger Compare Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ftcmpdat4_5](index.html) module"]
pub struct PWM_FTCMPDAT4_5_SPEC;
impl crate::RegisterSpec for PWM_FTCMPDAT4_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ftcmpdat4_5::R](R) reader structure"]
impl crate::Readable for PWM_FTCMPDAT4_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ftcmpdat4_5::W](W) writer structure"]
impl crate::Writable for PWM_FTCMPDAT4_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FTCMPDAT4_5 to value 0"]
impl crate::Resettable for PWM_FTCMPDAT4_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
