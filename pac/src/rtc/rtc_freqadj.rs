#[doc = "Register `RTC_FREQADJ` reader"]
pub struct R(crate::R<RTC_FREQADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_FREQADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_FREQADJ_SPEC>> for R {
    fn from(reader: crate::R<RTC_FREQADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_FREQADJ` writer"]
pub struct W(crate::W<RTC_FREQADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_FREQADJ_SPEC>;
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
impl core::convert::From<crate::W<RTC_FREQADJ_SPEC>> for W {
    fn from(writer: crate::W<RTC_FREQADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACTION` reader - Fraction Part\\nNote: Digit in RTC_FREQADJ must be expressed as hexadecimal number."]
pub struct FRACTION_R(crate::FieldReader<u8, u8>);
impl FRACTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRACTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACTION` writer - Fraction Part\\nNote: Digit in RTC_FREQADJ must be expressed as hexadecimal number."]
pub struct FRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `INTEGER` reader - Integer Part"]
pub struct INTEGER_R(crate::FieldReader<u8, u8>);
impl INTEGER_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTEGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEGER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEGER` writer - Integer Part"]
pub struct INTEGER_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fraction Part\\nNote: Digit in RTC_FREQADJ must be expressed as hexadecimal number."]
    #[inline(always)]
    pub fn fraction(&self) -> FRACTION_R {
        FRACTION_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Integer Part"]
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fraction Part\\nNote: Digit in RTC_FREQADJ must be expressed as hexadecimal number."]
    #[inline(always)]
    pub fn fraction(&mut self) -> FRACTION_W {
        FRACTION_W { w: self }
    }
    #[doc = "Bits 8:11 - Integer Part"]
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W {
        INTEGER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Frequency Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_freqadj](index.html) module"]
pub struct RTC_FREQADJ_SPEC;
impl crate::RegisterSpec for RTC_FREQADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_freqadj::R](R) reader structure"]
impl crate::Readable for RTC_FREQADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_freqadj::W](W) writer structure"]
impl crate::Writable for RTC_FREQADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_FREQADJ to value 0x0700"]
impl crate::Resettable for RTC_FREQADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700
    }
}
