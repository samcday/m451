#[doc = "Register `UART_TOUT` reader"]
pub struct R(crate::R<UART_TOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_TOUT_SPEC>> for R {
    fn from(reader: crate::R<UART_TOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TOUT` writer"]
pub struct W(crate::W<UART_TOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TOUT_SPEC>;
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
impl core::convert::From<crate::W<UART_TOUT_SPEC>> for W {
    fn from(writer: crate::W<UART_TOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIC` reader - Time-out Interrupt Comparator"]
pub struct TOIC_R(crate::FieldReader<u8, u8>);
impl TOIC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIC` writer - Time-out Interrupt Comparator"]
pub struct TOIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DLY` reader - TX Delay Time Value \\nThis field is used to programming the transfer delay time between the last stop bit and next start bit. The unit is bit time."]
pub struct DLY_R(crate::FieldReader<u8, u8>);
impl DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY` writer - TX Delay Time Value \\nThis field is used to programming the transfer delay time between the last stop bit and next start bit. The unit is bit time."]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Time-out Interrupt Comparator"]
    #[inline(always)]
    pub fn toic(&self) -> TOIC_R {
        TOIC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX Delay Time Value \\nThis field is used to programming the transfer delay time between the last stop bit and next start bit. The unit is bit time."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time-out Interrupt Comparator"]
    #[inline(always)]
    pub fn toic(&mut self) -> TOIC_W {
        TOIC_W { w: self }
    }
    #[doc = "Bits 8:15 - TX Delay Time Value \\nThis field is used to programming the transfer delay time between the last stop bit and next start bit. The unit is bit time."]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tout](index.html) module"]
pub struct UART_TOUT_SPEC;
impl crate::RegisterSpec for UART_TOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tout::R](R) reader structure"]
impl crate::Readable for UART_TOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tout::W](W) writer structure"]
impl crate::Writable for UART_TOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TOUT to value 0"]
impl crate::Resettable for UART_TOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
