#[doc = "Register `RTC_CALM` reader"]
pub struct R(crate::R<RTC_CALM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CALM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_CALM_SPEC>> for R {
    fn from(reader: crate::R<RTC_CALM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CALM` writer"]
pub struct W(crate::W<RTC_CALM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CALM_SPEC>;
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
impl core::convert::From<crate::W<RTC_CALM_SPEC>> for W {
    fn from(writer: crate::W<RTC_CALM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - 1-Day Calendar Digit of Alarm Setting (0~9)"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY` writer - 1-Day Calendar Digit of Alarm Setting (0~9)"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TENDAY` reader - 10-Day Calendar Digit of Alarm Setting (0~3)"]
pub struct TENDAY_R(crate::FieldReader<u8, u8>);
impl TENDAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENDAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENDAY` writer - 10-Day Calendar Digit of Alarm Setting (0~3)"]
pub struct TENDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TENDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `MON` reader - 1-Month Calendar Digit of Alarm Setting (0~9)"]
pub struct MON_R(crate::FieldReader<u8, u8>);
impl MON_R {
    pub(crate) fn new(bits: u8) -> Self {
        MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON` writer - 1-Month Calendar Digit of Alarm Setting (0~9)"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TENMON` reader - 10-Month Calendar Digit of Alarm Setting (0~1)"]
pub struct TENMON_R(crate::FieldReader<bool, bool>);
impl TENMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TENMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENMON` writer - 10-Month Calendar Digit of Alarm Setting (0~1)"]
pub struct TENMON_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `YEAR` reader - 1-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct YEAR_R(crate::FieldReader<u8, u8>);
impl YEAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` writer - 1-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TENYEAR` reader - 10-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct TENYEAR_R(crate::FieldReader<u8, u8>);
impl TENYEAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENYEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENYEAR` writer - 10-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct TENYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TENYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Day Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 10-Day Calendar Digit of Alarm Setting (0~3)"]
    #[inline(always)]
    pub fn tenday(&self) -> TENDAY_R {
        TENDAY_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 1-Month Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 10-Month Calendar Digit of Alarm Setting (0~1)"]
    #[inline(always)]
    pub fn tenmon(&self) -> TENMON_R {
        TENMON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - 1-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 10-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn tenyear(&self) -> TENYEAR_R {
        TENYEAR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Day Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bits 4:5 - 10-Day Calendar Digit of Alarm Setting (0~3)"]
    #[inline(always)]
    pub fn tenday(&mut self) -> TENDAY_W {
        TENDAY_W { w: self }
    }
    #[doc = "Bits 8:11 - 1-Month Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bit 12 - 10-Month Calendar Digit of Alarm Setting (0~1)"]
    #[inline(always)]
    pub fn tenmon(&mut self) -> TENMON_W {
        TENMON_W { w: self }
    }
    #[doc = "Bits 16:19 - 1-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 20:23 - 10-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn tenyear(&mut self) -> TENYEAR_W {
        TENYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Calendar Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calm](index.html) module"]
pub struct RTC_CALM_SPEC;
impl crate::RegisterSpec for RTC_CALM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_calm::R](R) reader structure"]
impl crate::Readable for RTC_CALM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_calm::W](W) writer structure"]
impl crate::Writable for RTC_CALM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CALM to value 0"]
impl crate::Resettable for RTC_CALM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
