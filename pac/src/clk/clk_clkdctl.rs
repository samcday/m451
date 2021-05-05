#[doc = "Register `CLK_CLKDCTL` reader"]
pub struct R(crate::R<CLK_CLKDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKDCTL_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKDCTL` writer"]
pub struct W(crate::W<CLK_CLKDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKDCTL_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKDCTL_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HXT Clock Fail Detector Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFDEN_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock fail detector Disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock fail detector Enabled"]
    _1 = 1,
}
impl From<HXTFDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFDEN` reader - HXT Clock Fail Detector Enable Bit"]
pub struct HXTFDEN_R(crate::FieldReader<bool, HXTFDEN_A>);
impl HXTFDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFDEN_A {
        match self.bits {
            false => HXTFDEN_A::_0,
            true => HXTFDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFDEN_A::_1
    }
}
impl core::ops::Deref for HXTFDEN_R {
    type Target = crate::FieldReader<bool, HXTFDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFDEN` writer - HXT Clock Fail Detector Enable Bit"]
pub struct HXTFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock fail detector Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFDEN_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock fail detector Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "HXT Clock Fail Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFIEN_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock fail interrupt Disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock fail interrupt Enabled"]
    _1 = 1,
}
impl From<HXTFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFIEN` reader - HXT Clock Fail Interrupt Enable Bit"]
pub struct HXTFIEN_R(crate::FieldReader<bool, HXTFIEN_A>);
impl HXTFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFIEN_A {
        match self.bits {
            false => HXTFIEN_A::_0,
            true => HXTFIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFIEN_A::_1
    }
}
impl core::ops::Deref for HXTFIEN_R {
    type Target = crate::FieldReader<bool, HXTFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFIEN` writer - HXT Clock Fail Interrupt Enable Bit"]
pub struct HXTFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock fail interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFIEN_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock fail interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "LXT Clock Fail Detector Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTFDEN_A {
    #[doc = "0: 32.768 kHz external low speed crystal oscillator (LXT) clock fail detector Disabled"]
    _0 = 0,
    #[doc = "1: 32.768 kHz external low speed crystal oscillator (LXT) clock fail detector Enabled"]
    _1 = 1,
}
impl From<LXTFDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTFDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTFDEN` reader - LXT Clock Fail Detector Enable Bit"]
pub struct LXTFDEN_R(crate::FieldReader<bool, LXTFDEN_A>);
impl LXTFDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTFDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTFDEN_A {
        match self.bits {
            false => LXTFDEN_A::_0,
            true => LXTFDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTFDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTFDEN_A::_1
    }
}
impl core::ops::Deref for LXTFDEN_R {
    type Target = crate::FieldReader<bool, LXTFDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTFDEN` writer - LXT Clock Fail Detector Enable Bit"]
pub struct LXTFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTFDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTFDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) clock fail detector Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTFDEN_A::_0)
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) clock fail detector Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTFDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "LXT Clock Fail Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTFIEN_A {
    #[doc = "0: 32.768 kHz external low speed crystal oscillator (LXT) clock fail interrupt Disabled"]
    _0 = 0,
    #[doc = "1: 32.768 kHz external low speed crystal oscillator (LXT) clock fail interrupt Enabled"]
    _1 = 1,
}
impl From<LXTFIEN_A> for bool {
    #[inline(always)]
    fn from(variant: LXTFIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTFIEN` reader - LXT Clock Fail Interrupt Enable Bit"]
pub struct LXTFIEN_R(crate::FieldReader<bool, LXTFIEN_A>);
impl LXTFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTFIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTFIEN_A {
        match self.bits {
            false => LXTFIEN_A::_0,
            true => LXTFIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTFIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTFIEN_A::_1
    }
}
impl core::ops::Deref for LXTFIEN_R {
    type Target = crate::FieldReader<bool, LXTFIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTFIEN` writer - LXT Clock Fail Interrupt Enable Bit"]
pub struct LXTFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTFIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTFIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) clock fail interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTFIEN_A::_0)
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) clock fail interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTFIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "HXT Clock Frequency Monitor Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFQDEN_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor Disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor Enabled"]
    _1 = 1,
}
impl From<HXTFQDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFQDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFQDEN` reader - HXT Clock Frequency Monitor Enable Bit"]
pub struct HXTFQDEN_R(crate::FieldReader<bool, HXTFQDEN_A>);
impl HXTFQDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFQDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFQDEN_A {
        match self.bits {
            false => HXTFQDEN_A::_0,
            true => HXTFQDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFQDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFQDEN_A::_1
    }
}
impl core::ops::Deref for HXTFQDEN_R {
    type Target = crate::FieldReader<bool, HXTFQDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFQDEN` writer - HXT Clock Frequency Monitor Enable Bit"]
pub struct HXTFQDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFQDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFQDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFQDEN_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFQDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "HXT Clock Frequency Monitor Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFQIEN_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor fail interrupt Disabled"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor fail interrupt Enabled"]
    _1 = 1,
}
impl From<HXTFQIEN_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFQIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFQIEN` reader - HXT Clock Frequency Monitor Interrupt Enable Bit"]
pub struct HXTFQIEN_R(crate::FieldReader<bool, HXTFQIEN_A>);
impl HXTFQIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFQIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFQIEN_A {
        match self.bits {
            false => HXTFQIEN_A::_0,
            true => HXTFQIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFQIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFQIEN_A::_1
    }
}
impl core::ops::Deref for HXTFQIEN_R {
    type Target = crate::FieldReader<bool, HXTFQIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFQIEN` writer - HXT Clock Frequency Monitor Interrupt Enable Bit"]
pub struct HXTFQIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFQIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFQIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor fail interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFQIEN_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock frequency monitor fail interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFQIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - HXT Clock Fail Detector Enable Bit"]
    #[inline(always)]
    pub fn hxtfden(&self) -> HXTFDEN_R {
        HXTFDEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HXT Clock Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hxtfien(&self) -> HXTFIEN_R {
        HXTFIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LXT Clock Fail Detector Enable Bit"]
    #[inline(always)]
    pub fn lxtfden(&self) -> LXTFDEN_R {
        LXTFDEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LXT Clock Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn lxtfien(&self) -> LXTFIEN_R {
        LXTFIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HXT Clock Frequency Monitor Enable Bit"]
    #[inline(always)]
    pub fn hxtfqden(&self) -> HXTFQDEN_R {
        HXTFQDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HXT Clock Frequency Monitor Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hxtfqien(&self) -> HXTFQIEN_R {
        HXTFQIEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - HXT Clock Fail Detector Enable Bit"]
    #[inline(always)]
    pub fn hxtfden(&mut self) -> HXTFDEN_W {
        HXTFDEN_W { w: self }
    }
    #[doc = "Bit 5 - HXT Clock Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hxtfien(&mut self) -> HXTFIEN_W {
        HXTFIEN_W { w: self }
    }
    #[doc = "Bit 12 - LXT Clock Fail Detector Enable Bit"]
    #[inline(always)]
    pub fn lxtfden(&mut self) -> LXTFDEN_W {
        LXTFDEN_W { w: self }
    }
    #[doc = "Bit 13 - LXT Clock Fail Interrupt Enable Bit"]
    #[inline(always)]
    pub fn lxtfien(&mut self) -> LXTFIEN_W {
        LXTFIEN_W { w: self }
    }
    #[doc = "Bit 16 - HXT Clock Frequency Monitor Enable Bit"]
    #[inline(always)]
    pub fn hxtfqden(&mut self) -> HXTFQDEN_W {
        HXTFQDEN_W { w: self }
    }
    #[doc = "Bit 17 - HXT Clock Frequency Monitor Interrupt Enable Bit"]
    #[inline(always)]
    pub fn hxtfqien(&mut self) -> HXTFQIEN_W {
        HXTFQIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Fail Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkdctl](index.html) module"]
pub struct CLK_CLKDCTL_SPEC;
impl crate::RegisterSpec for CLK_CLKDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkdctl::R](R) reader structure"]
impl crate::Readable for CLK_CLKDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkdctl::W](W) writer structure"]
impl crate::Writable for CLK_CLKDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKDCTL to value 0"]
impl crate::Resettable for CLK_CLKDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
