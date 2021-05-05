#[doc = "Register `RTC_TALM` reader"]
pub struct R(crate::R<RTC_TALM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TALM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_TALM_SPEC>> for R {
    fn from(reader: crate::R<RTC_TALM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TALM` writer"]
pub struct W(crate::W<RTC_TALM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TALM_SPEC>;
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
impl core::convert::From<crate::W<RTC_TALM_SPEC>> for W {
    fn from(writer: crate::W<RTC_TALM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 1-Sec Time Digit of Alarm Setting (0~9)"]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` writer - 1-Sec Time Digit of Alarm Setting (0~9)"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TENSEC` reader - 10-Sec Time Digit of Alarm Setting (0~5)"]
pub struct TENSEC_R(crate::FieldReader<u8, u8>);
impl TENSEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENSEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENSEC` writer - 10-Sec Time Digit of Alarm Setting (0~5)"]
pub struct TENSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TENSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MIN` reader - 1-Min Time Digit of Alarm Setting (0~9)"]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` writer - 1-Min Time Digit of Alarm Setting (0~9)"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TENMIN` reader - 10-Min Time Digit of Alarm Setting (0~5)"]
pub struct TENMIN_R(crate::FieldReader<u8, u8>);
impl TENMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENMIN` writer - 10-Min Time Digit of Alarm Setting (0~5)"]
pub struct TENMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `HR` reader - 1-Hour Time Digit of Alarm Setting (0~9)"]
pub struct HR_R(crate::FieldReader<u8, u8>);
impl HR_R {
    pub(crate) fn new(bits: u8) -> Self {
        HR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HR` writer - 1-Hour Time Digit of Alarm Setting (0~9)"]
pub struct HR_W<'a> {
    w: &'a mut W,
}
impl<'a> HR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TENHR` reader - 10-hour Time Digit of Alarm Setting (0~2)\\nWhen RTC runs as 12-hour time scale mode, the high bit of TENHR (RTC_TIME\\[21\\]) means AM/PM indication."]
pub struct TENHR_R(crate::FieldReader<u8, u8>);
impl TENHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENHR` writer - 10-hour Time Digit of Alarm Setting (0~2)\\nWhen RTC runs as 12-hour time scale mode, the high bit of TENHR (RTC_TIME\\[21\\]) means AM/PM indication."]
pub struct TENHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TENHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Sec Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 10-Sec Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn tensec(&self) -> TENSEC_R {
        TENSEC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - 1-Min Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 10-Min Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn tenmin(&self) -> TENMIN_R {
        TENMIN_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - 1-Hour Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn hr(&self) -> HR_R {
        HR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - 10-hour Time Digit of Alarm Setting (0~2)\\nWhen RTC runs as 12-hour time scale mode, the high bit of TENHR (RTC_TIME\\[21\\]) means AM/PM indication."]
    #[inline(always)]
    pub fn tenhr(&self) -> TENHR_R {
        TENHR_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Sec Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 4:6 - 10-Sec Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn tensec(&mut self) -> TENSEC_W {
        TENSEC_W { w: self }
    }
    #[doc = "Bits 8:11 - 1-Min Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 12:14 - 10-Min Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn tenmin(&mut self) -> TENMIN_W {
        TENMIN_W { w: self }
    }
    #[doc = "Bits 16:19 - 1-Hour Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn hr(&mut self) -> HR_W {
        HR_W { w: self }
    }
    #[doc = "Bits 20:21 - 10-hour Time Digit of Alarm Setting (0~2)\\nWhen RTC runs as 12-hour time scale mode, the high bit of TENHR (RTC_TIME\\[21\\]) means AM/PM indication."]
    #[inline(always)]
    pub fn tenhr(&mut self) -> TENHR_W {
        TENHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_talm](index.html) module"]
pub struct RTC_TALM_SPEC;
impl crate::RegisterSpec for RTC_TALM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_talm::R](R) reader structure"]
impl crate::Readable for RTC_TALM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_talm::W](W) writer structure"]
impl crate::Writable for RTC_TALM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TALM to value 0"]
impl crate::Resettable for RTC_TALM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
