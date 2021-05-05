#[doc = "Register `RTC_SPR7` reader"]
pub struct R(crate::R<RTC_SPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_SPR7_SPEC>> for R {
    fn from(reader: crate::R<RTC_SPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SPR7` writer"]
pub struct W(crate::W<RTC_SPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPR7_SPEC>;
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
impl core::convert::From<crate::W<RTC_SPR7_SPEC>> for W {
    fn from(writer: crate::W<RTC_SPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE` reader - Spare Register\\nThis field is used to store back-up information defined by user.\\nThis field will be cleared by hardware automatically once a snooper pin event is detected.\\nBefore storing back-up information in to RTC_SPRx register, user should write 0xA965 to RTC_RWEN\\[15:0\\]
to make sure register read/write enable bit REWNF (RTC_RWEN\\[16\\]) is enabled."]
pub struct SPARE_R(crate::FieldReader<u32, u32>);
impl SPARE_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - Spare Register\\nThis field is used to store back-up information defined by user.\\nThis field will be cleared by hardware automatically once a snooper pin event is detected.\\nBefore storing back-up information in to RTC_SPRx register, user should write 0xA965 to RTC_RWEN\\[15:0\\]
to make sure register read/write enable bit REWNF (RTC_RWEN\\[16\\]) is enabled."]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Spare Register\\nThis field is used to store back-up information defined by user.\\nThis field will be cleared by hardware automatically once a snooper pin event is detected.\\nBefore storing back-up information in to RTC_SPRx register, user should write 0xA965 to RTC_RWEN\\[15:0\\]
to make sure register read/write enable bit REWNF (RTC_RWEN\\[16\\]) is enabled."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spare Register\\nThis field is used to store back-up information defined by user.\\nThis field will be cleared by hardware automatically once a snooper pin event is detected.\\nBefore storing back-up information in to RTC_SPRx register, user should write 0xA965 to RTC_RWEN\\[15:0\\]
to make sure register read/write enable bit REWNF (RTC_RWEN\\[16\\]) is enabled."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Spare Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_spr7](index.html) module"]
pub struct RTC_SPR7_SPEC;
impl crate::RegisterSpec for RTC_SPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_spr7::R](R) reader structure"]
impl crate::Readable for RTC_SPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_spr7::W](W) writer structure"]
impl crate::Writable for RTC_SPR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SPR7 to value 0"]
impl crate::Resettable for RTC_SPR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
