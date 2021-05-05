#[doc = "Register `PWM_PERIOD2` reader"]
pub struct R(crate::R<PWM_PERIOD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_PERIOD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_PERIOD2_SPEC>> for R {
    fn from(reader: crate::R<PWM_PERIOD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_PERIOD2` writer"]
pub struct W(crate::W<PWM_PERIOD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_PERIOD2_SPEC>;
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
impl core::convert::From<crate::W<PWM_PERIOD2_SPEC>> for W {
    fn from(writer: crate::W<PWM_PERIOD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - PWM Period Register\\nUp-Count mode: In this mode, PWM counter counts from 0 to PERIOD, and restarts from 0.\\nDown-Count mode: In this mode, PWM counter counts from PERIOD to 0, and restarts from PERIOD."]
pub struct PERIOD_R(crate::FieldReader<u16, u16>);
impl PERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - PWM Period Register\\nUp-Count mode: In this mode, PWM counter counts from 0 to PERIOD, and restarts from 0.\\nDown-Count mode: In this mode, PWM counter counts from PERIOD to 0, and restarts from PERIOD."]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Period Register\\nUp-Count mode: In this mode, PWM counter counts from 0 to PERIOD, and restarts from 0.\\nDown-Count mode: In this mode, PWM counter counts from PERIOD to 0, and restarts from PERIOD."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Period Register\\nUp-Count mode: In this mode, PWM counter counts from 0 to PERIOD, and restarts from 0.\\nDown-Count mode: In this mode, PWM counter counts from PERIOD to 0, and restarts from PERIOD."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Period Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_period2](index.html) module"]
pub struct PWM_PERIOD2_SPEC;
impl crate::RegisterSpec for PWM_PERIOD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_period2::R](R) reader structure"]
impl crate::Readable for PWM_PERIOD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_period2::W](W) writer structure"]
impl crate::Writable for PWM_PERIOD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_PERIOD2 to value 0"]
impl crate::Resettable for PWM_PERIOD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
