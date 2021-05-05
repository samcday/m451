#[doc = "Register `DAC_TCTL` reader"]
pub struct R(crate::R<DAC_TCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_TCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_TCTL_SPEC>> for R {
    fn from(reader: crate::R<DAC_TCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_TCTL` writer"]
pub struct W(crate::W<DAC_TCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_TCTL_SPEC>;
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
impl core::convert::From<crate::W<DAC_TCTL_SPEC>> for W {
    fn from(writer: crate::W<DAC_TCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETTLET` reader - DAC Output Settling Time\\nUser software needs to write appropriate value to these bits to meet DAC conversion settling time base on PCLK (APB clock) speed.\\nFor example, DAC controller clock speed is 72MHz and DAC conversion setting time is 1 us, SETTLET value must be greater than 0x48."]
pub struct SETTLET_R(crate::FieldReader<u16, u16>);
impl SETTLET_R {
    pub(crate) fn new(bits: u16) -> Self {
        SETTLET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETTLET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETTLET` writer - DAC Output Settling Time\\nUser software needs to write appropriate value to these bits to meet DAC conversion settling time base on PCLK (APB clock) speed.\\nFor example, DAC controller clock speed is 72MHz and DAC conversion setting time is 1 us, SETTLET value must be greater than 0x48."]
pub struct SETTLET_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC Output Settling Time\\nUser software needs to write appropriate value to these bits to meet DAC conversion settling time base on PCLK (APB clock) speed.\\nFor example, DAC controller clock speed is 72MHz and DAC conversion setting time is 1 us, SETTLET value must be greater than 0x48."]
    #[inline(always)]
    pub fn settlet(&self) -> SETTLET_R {
        SETTLET_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Output Settling Time\\nUser software needs to write appropriate value to these bits to meet DAC conversion settling time base on PCLK (APB clock) speed.\\nFor example, DAC controller clock speed is 72MHz and DAC conversion setting time is 1 us, SETTLET value must be greater than 0x48."]
    #[inline(always)]
    pub fn settlet(&mut self) -> SETTLET_W {
        SETTLET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_tctl](index.html) module"]
pub struct DAC_TCTL_SPEC;
impl crate::RegisterSpec for DAC_TCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_tctl::R](R) reader structure"]
impl crate::Readable for DAC_TCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_tctl::W](W) writer structure"]
impl crate::Writable for DAC_TCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_TCTL to value 0"]
impl crate::Resettable for DAC_TCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
