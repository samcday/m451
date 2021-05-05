#[doc = "Register `CLK_CLKOCTL` reader"]
pub struct R(crate::R<CLK_CLKOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKOCTL_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKOCTL` writer"]
pub struct W(crate::W<CLK_CLKOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKOCTL_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKOCTL_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQSEL` reader - Clock Output Frequency Selection\\nThe formula of output frequency is\\nFin is the input clock frequency.\\nFout is the frequency of divider output clock.\\nN is the 4-bit value of FREQSEL\\[3:0\\]."]
pub struct FREQSEL_R(crate::FieldReader<u8, u8>);
impl FREQSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FREQSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQSEL` writer - Clock Output Frequency Selection\\nThe formula of output frequency is\\nFin is the input clock frequency.\\nFout is the frequency of divider output clock.\\nN is the 4-bit value of FREQSEL\\[3:0\\]."]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Clock Output Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOEN_A {
    #[doc = "0: Clock Output function Disabled"]
    _0 = 0,
    #[doc = "1: Clock Output function Enabled"]
    _1 = 1,
}
impl From<CLKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOEN` reader - Clock Output Enable Bit"]
pub struct CLKOEN_R(crate::FieldReader<bool, CLKOEN_A>);
impl CLKOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOEN_A {
        match self.bits {
            false => CLKOEN_A::_0,
            true => CLKOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKOEN_A::_1
    }
}
impl core::ops::Deref for CLKOEN_R {
    type Target = crate::FieldReader<bool, CLKOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOEN` writer - Clock Output Enable Bit"]
pub struct CLKOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Output function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOEN_A::_0)
    }
    #[doc = "Clock Output function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOEN_A::_1)
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
#[doc = "Clock Output Divide One Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV1EN_A {
    #[doc = "0: Clock Output will output clock with source frequency divided by FREQSEL"]
    _0 = 0,
    #[doc = "1: Clock Output will output clock with source frequency"]
    _1 = 1,
}
impl From<DIV1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIV1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV1EN` reader - Clock Output Divide One Enable Bit"]
pub struct DIV1EN_R(crate::FieldReader<bool, DIV1EN_A>);
impl DIV1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV1EN_A {
        match self.bits {
            false => DIV1EN_A::_0,
            true => DIV1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIV1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIV1EN_A::_1
    }
}
impl core::ops::Deref for DIV1EN_R {
    type Target = crate::FieldReader<bool, DIV1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV1EN` writer - Clock Output Divide One Enable Bit"]
pub struct DIV1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Output will output clock with source frequency divided by FREQSEL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIV1EN_A::_0)
    }
    #[doc = "Clock Output will output clock with source frequency"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIV1EN_A::_1)
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
#[doc = "Clock Output 1Hz Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK1HZEN_A {
    #[doc = "0: 1 Hz clock output for 32.768 kHz frequency compensation Disabled"]
    _0 = 0,
    #[doc = "1: 1 Hz clock output for 32.768 kHz frequency compensation Enabled"]
    _1 = 1,
}
impl From<CLK1HZEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK1HZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK1HZEN` reader - Clock Output 1Hz Enable Bit"]
pub struct CLK1HZEN_R(crate::FieldReader<bool, CLK1HZEN_A>);
impl CLK1HZEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK1HZEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK1HZEN_A {
        match self.bits {
            false => CLK1HZEN_A::_0,
            true => CLK1HZEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLK1HZEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLK1HZEN_A::_1
    }
}
impl core::ops::Deref for CLK1HZEN_R {
    type Target = crate::FieldReader<bool, CLK1HZEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1HZEN` writer - Clock Output 1Hz Enable Bit"]
pub struct CLK1HZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK1HZEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 Hz clock output for 32.768 kHz frequency compensation Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLK1HZEN_A::_0)
    }
    #[doc = "1 Hz clock output for 32.768 kHz frequency compensation Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLK1HZEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Output Frequency Selection\\nThe formula of output frequency is\\nFin is the input clock frequency.\\nFout is the frequency of divider output clock.\\nN is the 4-bit value of FREQSEL\\[3:0\\]."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Clock Output Enable Bit"]
    #[inline(always)]
    pub fn clkoen(&self) -> CLKOEN_R {
        CLKOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Output Divide One Enable Bit"]
    #[inline(always)]
    pub fn div1en(&self) -> DIV1EN_R {
        DIV1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock Output 1Hz Enable Bit"]
    #[inline(always)]
    pub fn clk1hzen(&self) -> CLK1HZEN_R {
        CLK1HZEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Output Frequency Selection\\nThe formula of output frequency is\\nFin is the input clock frequency.\\nFout is the frequency of divider output clock.\\nN is the 4-bit value of FREQSEL\\[3:0\\]."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
    #[doc = "Bit 4 - Clock Output Enable Bit"]
    #[inline(always)]
    pub fn clkoen(&mut self) -> CLKOEN_W {
        CLKOEN_W { w: self }
    }
    #[doc = "Bit 5 - Clock Output Divide One Enable Bit"]
    #[inline(always)]
    pub fn div1en(&mut self) -> DIV1EN_W {
        DIV1EN_W { w: self }
    }
    #[doc = "Bit 6 - Clock Output 1Hz Enable Bit"]
    #[inline(always)]
    pub fn clk1hzen(&mut self) -> CLK1HZEN_W {
        CLK1HZEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkoctl](index.html) module"]
pub struct CLK_CLKOCTL_SPEC;
impl crate::RegisterSpec for CLK_CLKOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkoctl::R](R) reader structure"]
impl crate::Readable for CLK_CLKOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkoctl::W](W) writer structure"]
impl crate::Writable for CLK_CLKOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKOCTL to value 0"]
impl crate::Resettable for CLK_CLKOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
