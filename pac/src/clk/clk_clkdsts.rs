#[doc = "Register `CLK_CLKDSTS` reader"]
pub struct R(crate::R<CLK_CLKDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKDSTS_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKDSTS` writer"]
pub struct W(crate::W<CLK_CLKDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKDSTS_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKDSTS_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFIF_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock is normal"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock stops"]
    _1 = 1,
}
impl From<HXTFIF_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFIF` reader - HXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct HXTFIF_R(crate::FieldReader<bool, HXTFIF_A>);
impl HXTFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFIF_A {
        match self.bits {
            false => HXTFIF_A::_0,
            true => HXTFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFIF_A::_1
    }
}
impl core::ops::Deref for HXTFIF_R {
    type Target = crate::FieldReader<bool, HXTFIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFIF` writer - HXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct HXTFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock is normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFIF_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFIF_A::_1)
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
#[doc = "LXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LXTFIF_A {
    #[doc = "0: 32.768 kHz external low speed crystal oscillator (LXT) clock is normal"]
    _0 = 0,
    #[doc = "1: 32.768 kHz external low speed crystal oscillator (LXT) stops"]
    _1 = 1,
}
impl From<LXTFIF_A> for bool {
    #[inline(always)]
    fn from(variant: LXTFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTFIF` reader - LXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct LXTFIF_R(crate::FieldReader<bool, LXTFIF_A>);
impl LXTFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LXTFIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LXTFIF_A {
        match self.bits {
            false => LXTFIF_A::_0,
            true => LXTFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LXTFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LXTFIF_A::_1
    }
}
impl core::ops::Deref for LXTFIF_R {
    type Target = crate::FieldReader<bool, LXTFIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LXTFIF` writer - LXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct LXTFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTFIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LXTFIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) clock is normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LXTFIF_A::_0)
    }
    #[doc = "32.768 kHz external low speed crystal oscillator (LXT) stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LXTFIF_A::_1)
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
#[doc = "HXT Clock Frequency Monitor Interrupt Flag\\nNote: Write 1 to clear the bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTFQIF_A {
    #[doc = "0: 4~20 MHz external high speed crystal oscillator (HXT) clock is normal"]
    _0 = 0,
    #[doc = "1: 4~20 MHz external high speed crystal oscillator (HXT) clock frequency is abnormal"]
    _1 = 1,
}
impl From<HXTFQIF_A> for bool {
    #[inline(always)]
    fn from(variant: HXTFQIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTFQIF` reader - HXT Clock Frequency Monitor Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct HXTFQIF_R(crate::FieldReader<bool, HXTFQIF_A>);
impl HXTFQIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HXTFQIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTFQIF_A {
        match self.bits {
            false => HXTFQIF_A::_0,
            true => HXTFQIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HXTFQIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HXTFQIF_A::_1
    }
}
impl core::ops::Deref for HXTFQIF_R {
    type Target = crate::FieldReader<bool, HXTFQIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HXTFQIF` writer - HXT Clock Frequency Monitor Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
pub struct HXTFQIF_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTFQIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HXTFQIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock is normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HXTFQIF_A::_0)
    }
    #[doc = "4~20 MHz external high speed crystal oscillator (HXT) clock frequency is abnormal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HXTFQIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn hxtfif(&self) -> HXTFIF_R {
        HXTFIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn lxtfif(&self) -> LXTFIF_R {
        LXTFIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HXT Clock Frequency Monitor Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn hxtfqif(&self) -> HXTFQIF_R {
        HXTFQIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn hxtfif(&mut self) -> HXTFIF_W {
        HXTFIF_W { w: self }
    }
    #[doc = "Bit 1 - LXT Clock Fail Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn lxtfif(&mut self) -> LXTFIF_W {
        LXTFIF_W { w: self }
    }
    #[doc = "Bit 8 - HXT Clock Frequency Monitor Interrupt Flag\\nNote: Write 1 to clear the bit to 0."]
    #[inline(always)]
    pub fn hxtfqif(&mut self) -> HXTFQIF_W {
        HXTFQIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Fail Detector Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkdsts](index.html) module"]
pub struct CLK_CLKDSTS_SPEC;
impl crate::RegisterSpec for CLK_CLKDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkdsts::R](R) reader structure"]
impl crate::Readable for CLK_CLKDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkdsts::W](W) writer structure"]
impl crate::Writable for CLK_CLKDSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKDSTS to value 0"]
impl crate::Resettable for CLK_CLKDSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
