#[doc = "Register `RTC_CLKFMT` reader"]
pub struct R(crate::R<RTC_CLKFMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CLKFMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_CLKFMT_SPEC>> for R {
    fn from(reader: crate::R<RTC_CLKFMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CLKFMT` writer"]
pub struct W(crate::W<RTC_CLKFMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CLKFMT_SPEC>;
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
impl core::convert::From<crate::W<RTC_CLKFMT_SPEC>> for W {
    fn from(writer: crate::W<RTC_CLKFMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "24-hour / 12-hour Time Scale Selection\\nIndicates that RTC_TIME and RTC_TALM are in 24-hour time scale or 12-hour time scale\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum _24HEN_A {
    #[doc = "0: 12-hour time scale with AM and PM indication selected"]
    _0 = 0,
    #[doc = "1: 24-hour time scale selected"]
    _1 = 1,
}
impl From<_24HEN_A> for bool {
    #[inline(always)]
    fn from(variant: _24HEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `_24HEN` reader - 24-hour / 12-hour Time Scale Selection\\nIndicates that RTC_TIME and RTC_TALM are in 24-hour time scale or 12-hour time scale"]
pub struct _24HEN_R(crate::FieldReader<bool, _24HEN_A>);
impl _24HEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        _24HEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _24HEN_A {
        match self.bits {
            false => _24HEN_A::_0,
            true => _24HEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == _24HEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == _24HEN_A::_1
    }
}
impl core::ops::Deref for _24HEN_R {
    type Target = crate::FieldReader<bool, _24HEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `_24HEN` writer - 24-hour / 12-hour Time Scale Selection\\nIndicates that RTC_TIME and RTC_TALM are in 24-hour time scale or 12-hour time scale"]
pub struct _24HEN_W<'a> {
    w: &'a mut W,
}
impl<'a> _24HEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: _24HEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "12-hour time scale with AM and PM indication selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(_24HEN_A::_0)
    }
    #[doc = "24-hour time scale selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(_24HEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - 24-hour / 12-hour Time Scale Selection\\nIndicates that RTC_TIME and RTC_TALM are in 24-hour time scale or 12-hour time scale"]
    #[inline(always)]
    pub fn _24hen(&self) -> _24HEN_R {
        _24HEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 24-hour / 12-hour Time Scale Selection\\nIndicates that RTC_TIME and RTC_TALM are in 24-hour time scale or 12-hour time scale"]
    #[inline(always)]
    pub fn _24hen(&mut self) -> _24HEN_W {
        _24HEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Scale Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_clkfmt](index.html) module"]
pub struct RTC_CLKFMT_SPEC;
impl crate::RegisterSpec for RTC_CLKFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_clkfmt::R](R) reader structure"]
impl crate::Readable for RTC_CLKFMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_clkfmt::W](W) writer structure"]
impl crate::Writable for RTC_CLKFMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CLKFMT to value 0x01"]
impl crate::Resettable for RTC_CLKFMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
