#[doc = "Register `PWM_FTCI` reader"]
pub struct R(crate::R<PWM_FTCI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FTCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_FTCI_SPEC>> for R {
    fn from(reader: crate::R<PWM_FTCI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FTCI` writer"]
pub struct W(crate::W<PWM_FTCI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FTCI_SPEC>;
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
impl core::convert::From<crate::W<PWM_FTCI_SPEC>> for W {
    fn from(writer: crate::W<PWM_FTCI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTCMUn` reader - PWM FTCMPDAT Up Indicator"]
pub struct FTCMUN_R(crate::FieldReader<u8, u8>);
impl FTCMUN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTCMUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTCMUN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTCMUn` writer - PWM FTCMPDAT Up Indicator"]
pub struct FTCMUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FTCMUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FTCMDn` reader - PWM FTCMPDAT Down Indicator"]
pub struct FTCMDN_R(crate::FieldReader<u8, u8>);
impl FTCMDN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTCMDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTCMDN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTCMDn` writer - PWM FTCMPDAT Down Indicator"]
pub struct FTCMDN_W<'a> {
    w: &'a mut W,
}
impl<'a> FTCMDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PWM FTCMPDAT Up Indicator"]
    #[inline(always)]
    pub fn ftcmun(&self) -> FTCMUN_R {
        FTCMUN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - PWM FTCMPDAT Down Indicator"]
    #[inline(always)]
    pub fn ftcmdn(&self) -> FTCMDN_R {
        FTCMDN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM FTCMPDAT Up Indicator"]
    #[inline(always)]
    pub fn ftcmun(&mut self) -> FTCMUN_W {
        FTCMUN_W { w: self }
    }
    #[doc = "Bits 8:10 - PWM FTCMPDAT Down Indicator"]
    #[inline(always)]
    pub fn ftcmdn(&mut self) -> FTCMDN_W {
        FTCMDN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM FTCMPDAT Indicator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ftci](index.html) module"]
pub struct PWM_FTCI_SPEC;
impl crate::RegisterSpec for PWM_FTCI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ftci::R](R) reader structure"]
impl crate::Readable for PWM_FTCI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ftci::W](W) writer structure"]
impl crate::Writable for PWM_FTCI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FTCI to value 0"]
impl crate::Resettable for PWM_FTCI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
