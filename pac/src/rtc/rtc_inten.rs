#[doc = "Register `RTC_INTEN` reader"]
pub struct R(crate::R<RTC_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_INTEN_SPEC>> for R {
    fn from(reader: crate::R<RTC_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INTEN` writer"]
pub struct W(crate::W<RTC_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INTEN_SPEC>;
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
impl core::convert::From<crate::W<RTC_INTEN_SPEC>> for W {
    fn from(writer: crate::W<RTC_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alarm Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALMIEN_A {
    #[doc = "0: RTC Alarm interrupt Disabled"]
    _0 = 0,
    #[doc = "1: RTC Alarm interrupt Enabled"]
    _1 = 1,
}
impl From<ALMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALMIEN` reader - Alarm Interrupt Enable Bit"]
pub struct ALMIEN_R(crate::FieldReader<bool, ALMIEN_A>);
impl ALMIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALMIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALMIEN_A {
        match self.bits {
            false => ALMIEN_A::_0,
            true => ALMIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALMIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALMIEN_A::_1
    }
}
impl core::ops::Deref for ALMIEN_R {
    type Target = crate::FieldReader<bool, ALMIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMIEN` writer - Alarm Interrupt Enable Bit"]
pub struct ALMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC Alarm interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALMIEN_A::_0)
    }
    #[doc = "RTC Alarm interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALMIEN_A::_1)
    }
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
#[doc = "Time Tick Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKIEN_A {
    #[doc = "0: RTC Time Tick interrupt Disabled"]
    _0 = 0,
    #[doc = "1: RTC Time Tick interrupt Enabled"]
    _1 = 1,
}
impl From<TICKIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TICKIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKIEN` reader - Time Tick Interrupt Enable Bit"]
pub struct TICKIEN_R(crate::FieldReader<bool, TICKIEN_A>);
impl TICKIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKIEN_A {
        match self.bits {
            false => TICKIEN_A::_0,
            true => TICKIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TICKIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TICKIEN_A::_1
    }
}
impl core::ops::Deref for TICKIEN_R {
    type Target = crate::FieldReader<bool, TICKIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKIEN` writer - Time Tick Interrupt Enable Bit"]
pub struct TICKIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC Time Tick interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TICKIEN_A::_0)
    }
    #[doc = "RTC Time Tick interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TICKIEN_A::_1)
    }
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
#[doc = "Snoop Detection Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNPDIEN_A {
    #[doc = "0: Snoop detected interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Snoop detected interrupt Enabled"]
    _1 = 1,
}
impl From<SNPDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNPDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNPDIEN` reader - Snoop Detection Interrupt Enable Bit"]
pub struct SNPDIEN_R(crate::FieldReader<bool, SNPDIEN_A>);
impl SNPDIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNPDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNPDIEN_A {
        match self.bits {
            false => SNPDIEN_A::_0,
            true => SNPDIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SNPDIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SNPDIEN_A::_1
    }
}
impl core::ops::Deref for SNPDIEN_R {
    type Target = crate::FieldReader<bool, SNPDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNPDIEN` writer - Snoop Detection Interrupt Enable Bit"]
pub struct SNPDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNPDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Snoop detected interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNPDIEN_A::_0)
    }
    #[doc = "Snoop detected interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNPDIEN_A::_1)
    }
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
impl R {
    #[doc = "Bit 0 - Alarm Interrupt Enable Bit"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Tick Interrupt Enable Bit"]
    #[inline(always)]
    pub fn tickien(&self) -> TICKIEN_R {
        TICKIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Snoop Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn snpdien(&self) -> SNPDIEN_R {
        SNPDIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm Interrupt Enable Bit"]
    #[inline(always)]
    pub fn almien(&mut self) -> ALMIEN_W {
        ALMIEN_W { w: self }
    }
    #[doc = "Bit 1 - Time Tick Interrupt Enable Bit"]
    #[inline(always)]
    pub fn tickien(&mut self) -> TICKIEN_W {
        TICKIEN_W { w: self }
    }
    #[doc = "Bit 2 - Snoop Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn snpdien(&mut self) -> SNPDIEN_W {
        SNPDIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_inten](index.html) module"]
pub struct RTC_INTEN_SPEC;
impl crate::RegisterSpec for RTC_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_inten::R](R) reader structure"]
impl crate::Readable for RTC_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_inten::W](W) writer structure"]
impl crate::Writable for RTC_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INTEN to value 0"]
impl crate::Resettable for RTC_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
