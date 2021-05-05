#[doc = "Register `PWM_PHS2_3` reader"]
pub struct R(crate::R<PWM_PHS2_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_PHS2_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_PHS2_3_SPEC>> for R {
    fn from(reader: crate::R<PWM_PHS2_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_PHS2_3` writer"]
pub struct W(crate::W<PWM_PHS2_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_PHS2_3_SPEC>;
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
impl core::convert::From<crate::W<PWM_PHS2_3_SPEC>> for W {
    fn from(writer: crate::W<PWM_PHS2_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHS` reader - PWM Synchronous Start Phase Bits\\nPHS determines the PWM synchronous start phase value. These bits only use in synchronous function."]
pub struct PHS_R(crate::FieldReader<u16, u16>);
impl PHS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHS` writer - PWM Synchronous Start Phase Bits\\nPHS determines the PWM synchronous start phase value. These bits only use in synchronous function."]
pub struct PHS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM Synchronous Start Phase Bits\\nPHS determines the PWM synchronous start phase value. These bits only use in synchronous function."]
    #[inline(always)]
    pub fn phs(&self) -> PHS_R {
        PHS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM Synchronous Start Phase Bits\\nPHS determines the PWM synchronous start phase value. These bits only use in synchronous function."]
    #[inline(always)]
    pub fn phs(&mut self) -> PHS_W {
        PHS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Counter Phase Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_phs2_3](index.html) module"]
pub struct PWM_PHS2_3_SPEC;
impl crate::RegisterSpec for PWM_PHS2_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_phs2_3::R](R) reader structure"]
impl crate::Readable for PWM_PHS2_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_phs2_3::W](W) writer structure"]
impl crate::Writable for PWM_PHS2_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_PHS2_3 to value 0"]
impl crate::Resettable for PWM_PHS2_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
