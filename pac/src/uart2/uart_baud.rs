#[doc = "Register `UART_BAUD` reader"]
pub struct R(crate::R<UART_BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_BAUD_SPEC>> for R {
    fn from(reader: crate::R<UART_BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_BAUD` writer"]
pub struct W(crate::W<UART_BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BAUD_SPEC>;
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
impl core::convert::From<crate::W<UART_BAUD_SPEC>> for W {
    fn from(writer: crate::W<UART_BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRD` reader - Baud Rate Divider\\nThe field indicates the baud rate divider. This filed is used in baud rate calculation. The detail description is shown in Table 623."]
pub struct BRD_R(crate::FieldReader<u16, u16>);
impl BRD_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRD` writer - Baud Rate Divider\\nThe field indicates the baud rate divider. This filed is used in baud rate calculation. The detail description is shown in Table 623."]
pub struct BRD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EDIVM1` reader - Extra Divider for BAUD Rate Mode 1\\nThis field is used for baud rate calculation in mode 1 and has no effect for baud rate calculation in mode 0 and mode 2. The detail description is shown in Table 623."]
pub struct EDIVM1_R(crate::FieldReader<u8, u8>);
impl EDIVM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDIVM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDIVM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDIVM1` writer - Extra Divider for BAUD Rate Mode 1\\nThis field is used for baud rate calculation in mode 1 and has no effect for baud rate calculation in mode 0 and mode 2. The detail description is shown in Table 623."]
pub struct EDIVM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIVM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `BAUDM0` reader - BAUD Rate Mode Selection Bit 0\\nThis bit is baud rate mode selection bit 0. UART provides three baud rate calculation modes. This bit combines with BAUDM1 (UART_BAUD\\[29\\]) to select baud rate calculation mode. The detail description is shown in Table 623."]
pub struct BAUDM0_R(crate::FieldReader<bool, bool>);
impl BAUDM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BAUDM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUDM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUDM0` writer - BAUD Rate Mode Selection Bit 0\\nThis bit is baud rate mode selection bit 0. UART provides three baud rate calculation modes. This bit combines with BAUDM1 (UART_BAUD\\[29\\]) to select baud rate calculation mode. The detail description is shown in Table 623."]
pub struct BAUDM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `BAUDM1` reader - BAUD Rate Mode Selection Bit 1\\nThis bit is baud rate mode selection bit 1. UART provides three baud rate calculation modes. This bit combines with BAUDM0 (UART_BAUD\\[28\\]) to select baud rate calculation mode. The detail description is shown in Table 623.\\nNote: In IrDA mode must be operated in mode 0."]
pub struct BAUDM1_R(crate::FieldReader<bool, bool>);
impl BAUDM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BAUDM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUDM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUDM1` writer - BAUD Rate Mode Selection Bit 1\\nThis bit is baud rate mode selection bit 1. UART provides three baud rate calculation modes. This bit combines with BAUDM0 (UART_BAUD\\[28\\]) to select baud rate calculation mode. The detail description is shown in Table 623.\\nNote: In IrDA mode must be operated in mode 0."]
pub struct BAUDM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Baud Rate Divider\\nThe field indicates the baud rate divider. This filed is used in baud rate calculation. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn brd(&self) -> BRD_R {
        BRD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Extra Divider for BAUD Rate Mode 1\\nThis field is used for baud rate calculation in mode 1 and has no effect for baud rate calculation in mode 0 and mode 2. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn edivm1(&self) -> EDIVM1_R {
        EDIVM1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - BAUD Rate Mode Selection Bit 0\\nThis bit is baud rate mode selection bit 0. UART provides three baud rate calculation modes. This bit combines with BAUDM1 (UART_BAUD\\[29\\]) to select baud rate calculation mode. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn baudm0(&self) -> BAUDM0_R {
        BAUDM0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BAUD Rate Mode Selection Bit 1\\nThis bit is baud rate mode selection bit 1. UART provides three baud rate calculation modes. This bit combines with BAUDM0 (UART_BAUD\\[28\\]) to select baud rate calculation mode. The detail description is shown in Table 623.\\nNote: In IrDA mode must be operated in mode 0."]
    #[inline(always)]
    pub fn baudm1(&self) -> BAUDM1_R {
        BAUDM1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Divider\\nThe field indicates the baud rate divider. This filed is used in baud rate calculation. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn brd(&mut self) -> BRD_W {
        BRD_W { w: self }
    }
    #[doc = "Bits 24:27 - Extra Divider for BAUD Rate Mode 1\\nThis field is used for baud rate calculation in mode 1 and has no effect for baud rate calculation in mode 0 and mode 2. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn edivm1(&mut self) -> EDIVM1_W {
        EDIVM1_W { w: self }
    }
    #[doc = "Bit 28 - BAUD Rate Mode Selection Bit 0\\nThis bit is baud rate mode selection bit 0. UART provides three baud rate calculation modes. This bit combines with BAUDM1 (UART_BAUD\\[29\\]) to select baud rate calculation mode. The detail description is shown in Table 623."]
    #[inline(always)]
    pub fn baudm0(&mut self) -> BAUDM0_W {
        BAUDM0_W { w: self }
    }
    #[doc = "Bit 29 - BAUD Rate Mode Selection Bit 1\\nThis bit is baud rate mode selection bit 1. UART provides three baud rate calculation modes. This bit combines with BAUDM0 (UART_BAUD\\[28\\]) to select baud rate calculation mode. The detail description is shown in Table 623.\\nNote: In IrDA mode must be operated in mode 0."]
    #[inline(always)]
    pub fn baudm1(&mut self) -> BAUDM1_W {
        BAUDM1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_baud](index.html) module"]
pub struct UART_BAUD_SPEC;
impl crate::RegisterSpec for UART_BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_baud::R](R) reader structure"]
impl crate::Readable for UART_BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_baud::W](W) writer structure"]
impl crate::Writable for UART_BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_BAUD to value 0x0f00_0000"]
impl crate::Resettable for UART_BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_0000
    }
}
