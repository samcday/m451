#[doc = "Register `CLK_CLKDIV0` reader"]
pub struct R(crate::R<CLK_CLKDIV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKDIV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKDIV0_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKDIV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKDIV0` writer"]
pub struct W(crate::W<CLK_CLKDIV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKDIV0_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKDIV0_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKDIV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCLKDIV` reader - HCLK Clock Divide Number From HCLK Clock Source"]
pub struct HCLKDIV_R(crate::FieldReader<u8, u8>);
impl HCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLKDIV` writer - HCLK Clock Divide Number From HCLK Clock Source"]
pub struct HCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `USBDIV` reader - USB Clock Divide Number From PLL Clock"]
pub struct USBDIV_R(crate::FieldReader<u8, u8>);
impl USBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDIV` writer - USB Clock Divide Number From PLL Clock"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `UARTDIV` reader - UART Clock Divide Number From UART Clock Source"]
pub struct UARTDIV_R(crate::FieldReader<u8, u8>);
impl UARTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UARTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTDIV` writer - UART Clock Divide Number From UART Clock Source"]
pub struct UARTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `EADCDIV` reader - EADC Clock Divide Number From EADC Clock Source"]
pub struct EADCDIV_R(crate::FieldReader<u8, u8>);
impl EADCDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        EADCDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EADCDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EADCDIV` writer - EADC Clock Divide Number From EADC Clock Source"]
pub struct EADCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EADCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - HCLK Clock Divide Number From HCLK Clock Source"]
    #[inline(always)]
    pub fn hclkdiv(&self) -> HCLKDIV_R {
        HCLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - USB Clock Divide Number From PLL Clock"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - UART Clock Divide Number From UART Clock Source"]
    #[inline(always)]
    pub fn uartdiv(&self) -> UARTDIV_R {
        UARTDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - EADC Clock Divide Number From EADC Clock Source"]
    #[inline(always)]
    pub fn eadcdiv(&self) -> EADCDIV_R {
        EADCDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - HCLK Clock Divide Number From HCLK Clock Source"]
    #[inline(always)]
    pub fn hclkdiv(&mut self) -> HCLKDIV_W {
        HCLKDIV_W { w: self }
    }
    #[doc = "Bits 4:7 - USB Clock Divide Number From PLL Clock"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Bits 8:11 - UART Clock Divide Number From UART Clock Source"]
    #[inline(always)]
    pub fn uartdiv(&mut self) -> UARTDIV_W {
        UARTDIV_W { w: self }
    }
    #[doc = "Bits 16:23 - EADC Clock Divide Number From EADC Clock Source"]
    #[inline(always)]
    pub fn eadcdiv(&mut self) -> EADCDIV_W {
        EADCDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider Number Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkdiv0](index.html) module"]
pub struct CLK_CLKDIV0_SPEC;
impl crate::RegisterSpec for CLK_CLKDIV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkdiv0::R](R) reader structure"]
impl crate::Readable for CLK_CLKDIV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkdiv0::W](W) writer structure"]
impl crate::Writable for CLK_CLKDIV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKDIV0 to value 0"]
impl crate::Resettable for CLK_CLKDIV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
