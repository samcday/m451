#[doc = "Register `RTC_CAMSK` reader"]
pub struct R(crate::R<RTC_CAMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CAMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_CAMSK_SPEC>> for R {
    fn from(reader: crate::R<RTC_CAMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CAMSK` writer"]
pub struct W(crate::W<RTC_CAMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CAMSK_SPEC>;
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
impl core::convert::From<crate::W<RTC_CAMSK_SPEC>> for W {
    fn from(writer: crate::W<RTC_CAMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDAY` reader - Mask 1-Day Calendar Digit of Alarm Setting (0~9)"]
pub struct MDAY_R(crate::FieldReader<bool, bool>);
impl MDAY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDAY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDAY` writer - Mask 1-Day Calendar Digit of Alarm Setting (0~9)"]
pub struct MDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MDAY_W<'a> {
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
#[doc = "Field `MTENDAY` reader - Mask 10-Day Calendar Digit of Alarm Setting (0~3)"]
pub struct MTENDAY_R(crate::FieldReader<bool, bool>);
impl MTENDAY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENDAY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENDAY` writer - Mask 10-Day Calendar Digit of Alarm Setting (0~3)"]
pub struct MTENDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENDAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MMON` reader - Mask 1-Month Calendar Digit of Alarm Setting (0~9)"]
pub struct MMON_R(crate::FieldReader<bool, bool>);
impl MMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMON` writer - Mask 1-Month Calendar Digit of Alarm Setting (0~9)"]
pub struct MMON_W<'a> {
    w: &'a mut W,
}
impl<'a> MMON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MTENMON` reader - Mask 10-Month Calendar Digit of Alarm Setting (0~1)"]
pub struct MTENMON_R(crate::FieldReader<bool, bool>);
impl MTENMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENMON` writer - Mask 10-Month Calendar Digit of Alarm Setting (0~1)"]
pub struct MTENMON_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENMON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MYEAR` reader - Mask 1-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct MYEAR_R(crate::FieldReader<bool, bool>);
impl MYEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MYEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MYEAR` writer - Mask 1-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct MYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MYEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MTENYEAR` reader - Mask 10-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct MTENYEAR_R(crate::FieldReader<bool, bool>);
impl MTENYEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENYEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENYEAR` writer - Mask 10-Year Calendar Digit of Alarm Setting (0~9)"]
pub struct MTENYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENYEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mask 1-Day Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mday(&self) -> MDAY_R {
        MDAY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask 10-Day Calendar Digit of Alarm Setting (0~3)"]
    #[inline(always)]
    pub fn mtenday(&self) -> MTENDAY_R {
        MTENDAY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask 1-Month Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mmon(&self) -> MMON_R {
        MMON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask 10-Month Calendar Digit of Alarm Setting (0~1)"]
    #[inline(always)]
    pub fn mtenmon(&self) -> MTENMON_R {
        MTENMON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask 1-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn myear(&self) -> MYEAR_R {
        MYEAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask 10-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mtenyear(&self) -> MTENYEAR_R {
        MTENYEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask 1-Day Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mday(&mut self) -> MDAY_W {
        MDAY_W { w: self }
    }
    #[doc = "Bit 1 - Mask 10-Day Calendar Digit of Alarm Setting (0~3)"]
    #[inline(always)]
    pub fn mtenday(&mut self) -> MTENDAY_W {
        MTENDAY_W { w: self }
    }
    #[doc = "Bit 2 - Mask 1-Month Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mmon(&mut self) -> MMON_W {
        MMON_W { w: self }
    }
    #[doc = "Bit 3 - Mask 10-Month Calendar Digit of Alarm Setting (0~1)"]
    #[inline(always)]
    pub fn mtenmon(&mut self) -> MTENMON_W {
        MTENMON_W { w: self }
    }
    #[doc = "Bit 4 - Mask 1-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn myear(&mut self) -> MYEAR_W {
        MYEAR_W { w: self }
    }
    #[doc = "Bit 5 - Mask 10-Year Calendar Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mtenyear(&mut self) -> MTENYEAR_W {
        MTENYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Calendar Alarm Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_camsk](index.html) module"]
pub struct RTC_CAMSK_SPEC;
impl crate::RegisterSpec for RTC_CAMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_camsk::R](R) reader structure"]
impl crate::Readable for RTC_CAMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_camsk::W](W) writer structure"]
impl crate::Writable for RTC_CAMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CAMSK to value 0"]
impl crate::Resettable for RTC_CAMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
