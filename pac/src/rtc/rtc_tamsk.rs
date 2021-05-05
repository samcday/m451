#[doc = "Register `RTC_TAMSK` reader"]
pub struct R(crate::R<RTC_TAMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TAMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_TAMSK_SPEC>> for R {
    fn from(reader: crate::R<RTC_TAMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TAMSK` writer"]
pub struct W(crate::W<RTC_TAMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TAMSK_SPEC>;
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
impl core::convert::From<crate::W<RTC_TAMSK_SPEC>> for W {
    fn from(writer: crate::W<RTC_TAMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEC` reader - Mask 1-Sec Time Digit of Alarm Setting (0~9)"]
pub struct MSEC_R(crate::FieldReader<bool, bool>);
impl MSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEC` writer - Mask 1-Sec Time Digit of Alarm Setting (0~9)"]
pub struct MSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEC_W<'a> {
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
#[doc = "Field `MTENSEC` reader - Mask 10-Sec Time Digit of Alarm Setting (0~5)"]
pub struct MTENSEC_R(crate::FieldReader<bool, bool>);
impl MTENSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENSEC` writer - Mask 10-Sec Time Digit of Alarm Setting (0~5)"]
pub struct MTENSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENSEC_W<'a> {
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
#[doc = "Field `MMIN` reader - Mask 1-Min Time Digit of Alarm Setting (0~9)"]
pub struct MMIN_R(crate::FieldReader<bool, bool>);
impl MMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMIN` writer - Mask 1-Min Time Digit of Alarm Setting (0~9)"]
pub struct MMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MMIN_W<'a> {
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
#[doc = "Field `MTENMIN` reader - Mask 10-Min Time Digit of Alarm Setting (0~5)"]
pub struct MTENMIN_R(crate::FieldReader<bool, bool>);
impl MTENMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENMIN` writer - Mask 10-Min Time Digit of Alarm Setting (0~5)"]
pub struct MTENMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENMIN_W<'a> {
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
#[doc = "Field `MHR` reader - Mask 1-Hour Time Digit of Alarm Setting (0~9)"]
pub struct MHR_R(crate::FieldReader<bool, bool>);
impl MHR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MHR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MHR` writer - Mask 1-Hour Time Digit of Alarm Setting (0~9)"]
pub struct MHR_W<'a> {
    w: &'a mut W,
}
impl<'a> MHR_W<'a> {
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
#[doc = "Field `MTENHR` reader - Mask 10-Hour Time Digit of Alarm Setting (0~2)"]
pub struct MTENHR_R(crate::FieldReader<bool, bool>);
impl MTENHR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTENHR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENHR` writer - Mask 10-Hour Time Digit of Alarm Setting (0~2)"]
pub struct MTENHR_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENHR_W<'a> {
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
    #[doc = "Bit 0 - Mask 1-Sec Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn msec(&self) -> MSEC_R {
        MSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask 10-Sec Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn mtensec(&self) -> MTENSEC_R {
        MTENSEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask 1-Min Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mmin(&self) -> MMIN_R {
        MMIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask 10-Min Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn mtenmin(&self) -> MTENMIN_R {
        MTENMIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask 1-Hour Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mhr(&self) -> MHR_R {
        MHR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask 10-Hour Time Digit of Alarm Setting (0~2)"]
    #[inline(always)]
    pub fn mtenhr(&self) -> MTENHR_R {
        MTENHR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask 1-Sec Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn msec(&mut self) -> MSEC_W {
        MSEC_W { w: self }
    }
    #[doc = "Bit 1 - Mask 10-Sec Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn mtensec(&mut self) -> MTENSEC_W {
        MTENSEC_W { w: self }
    }
    #[doc = "Bit 2 - Mask 1-Min Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mmin(&mut self) -> MMIN_W {
        MMIN_W { w: self }
    }
    #[doc = "Bit 3 - Mask 10-Min Time Digit of Alarm Setting (0~5)"]
    #[inline(always)]
    pub fn mtenmin(&mut self) -> MTENMIN_W {
        MTENMIN_W { w: self }
    }
    #[doc = "Bit 4 - Mask 1-Hour Time Digit of Alarm Setting (0~9)"]
    #[inline(always)]
    pub fn mhr(&mut self) -> MHR_W {
        MHR_W { w: self }
    }
    #[doc = "Bit 5 - Mask 10-Hour Time Digit of Alarm Setting (0~2)"]
    #[inline(always)]
    pub fn mtenhr(&mut self) -> MTENHR_W {
        MTENHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Alarm Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tamsk](index.html) module"]
pub struct RTC_TAMSK_SPEC;
impl crate::RegisterSpec for RTC_TAMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tamsk::R](R) reader structure"]
impl crate::Readable for RTC_TAMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tamsk::W](W) writer structure"]
impl crate::Writable for RTC_TAMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TAMSK to value 0"]
impl crate::Resettable for RTC_TAMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
