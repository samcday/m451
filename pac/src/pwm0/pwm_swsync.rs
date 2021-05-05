#[doc = "Register `PWM_SWSYNC` reader"]
pub struct R(crate::R<PWM_SWSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SWSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_SWSYNC_SPEC>> for R {
    fn from(reader: crate::R<PWM_SWSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SWSYNC` writer"]
pub struct W(crate::W<PWM_SWSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SWSYNC_SPEC>;
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
impl core::convert::From<crate::W<PWM_SWSYNC_SPEC>> for W {
    fn from(writer: crate::W<PWM_SWSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWSYNCn` reader - Software SYNC Function\\nEach bit n controls corresponding PWM channel n.\\nWhen SINSRCn (PWM_SYNC\\[13:8\\]) is selected to 0, SYNC_OUT source is come from SYNC_IN or this bit."]
pub struct SWSYNCN_R(crate::FieldReader<u8, u8>);
impl SWSYNCN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWSYNCN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWSYNCN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWSYNCn` writer - Software SYNC Function\\nEach bit n controls corresponding PWM channel n.\\nWhen SINSRCn (PWM_SYNC\\[13:8\\]) is selected to 0, SYNC_OUT source is come from SYNC_IN or this bit."]
pub struct SWSYNCN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYNCN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Software SYNC Function\\nEach bit n controls corresponding PWM channel n.\\nWhen SINSRCn (PWM_SYNC\\[13:8\\]) is selected to 0, SYNC_OUT source is come from SYNC_IN or this bit."]
    #[inline(always)]
    pub fn swsyncn(&self) -> SWSYNCN_R {
        SWSYNCN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Software SYNC Function\\nEach bit n controls corresponding PWM channel n.\\nWhen SINSRCn (PWM_SYNC\\[13:8\\]) is selected to 0, SYNC_OUT source is come from SYNC_IN or this bit."]
    #[inline(always)]
    pub fn swsyncn(&mut self) -> SWSYNCN_W {
        SWSYNCN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Software Control Synchronization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_swsync](index.html) module"]
pub struct PWM_SWSYNC_SPEC;
impl crate::RegisterSpec for PWM_SWSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_swsync::R](R) reader structure"]
impl crate::Readable for PWM_SWSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_swsync::W](W) writer structure"]
impl crate::Writable for PWM_SWSYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SWSYNC to value 0"]
impl crate::Resettable for PWM_SWSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
