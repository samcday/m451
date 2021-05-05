#[doc = "Register `PWM_SSTRG` writer"]
pub struct W(crate::W<PWM_SSTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SSTRG_SPEC>;
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
impl core::convert::From<crate::W<PWM_SSTRG_SPEC>> for W {
    fn from(writer: crate::W<PWM_SSTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTSEN` writer - PWM Counter Synchronous Start Enable Bit (Write Only)\\nPMW counter synchronous enable function is used to make selected PWM channels (include PWM0_CHx and PWM1_CHx) start counting at the same time.\\nWriting this bit to 1 will also set the counter enable bit (CNTENn, n denotes channel 0 to 5) if correlated PWM channel counter synchronous start function is enabled.\\nNote: This bit only present in PWM0_BA."]
pub struct CNTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - PWM Counter Synchronous Start Enable Bit (Write Only)\\nPMW counter synchronous enable function is used to make selected PWM channels (include PWM0_CHx and PWM1_CHx) start counting at the same time.\\nWriting this bit to 1 will also set the counter enable bit (CNTENn, n denotes channel 0 to 5) if correlated PWM channel counter synchronous start function is enabled.\\nNote: This bit only present in PWM0_BA."]
    #[inline(always)]
    pub fn cntsen(&mut self) -> CNTSEN_W {
        CNTSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Synchronous Start Trigger Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sstrg](index.html) module"]
pub struct PWM_SSTRG_SPEC;
impl crate::RegisterSpec for PWM_SSTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_sstrg::W](W) writer structure"]
impl crate::Writable for PWM_SSTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SSTRG to value 0"]
impl crate::Resettable for PWM_SSTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
