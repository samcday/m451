#[doc = "Register `UART_DAT` reader"]
pub struct R(crate::R<UART_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_DAT_SPEC>> for R {
    fn from(reader: crate::R<UART_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DAT` writer"]
pub struct W(crate::W<UART_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DAT_SPEC>;
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
impl core::convert::From<crate::W<UART_DAT_SPEC>> for W {
    fn from(writer: crate::W<UART_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Receiving/Transmit Buffer\\nWrite Operation:\\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART Controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation:\\nBy reading this register, the UART will return an 8-bit data received from receiving FIFO."]
pub struct DAT_R(crate::FieldReader<u8, u8>);
impl DAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT` writer - Receiving/Transmit Buffer\\nWrite Operation:\\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART Controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation:\\nBy reading this register, the UART will return an 8-bit data received from receiving FIFO."]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receiving/Transmit Buffer\\nWrite Operation:\\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART Controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation:\\nBy reading this register, the UART will return an 8-bit data received from receiving FIFO."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receiving/Transmit Buffer\\nWrite Operation:\\nBy writing one byte to this register, the data byte will be stored in transmitter FIFO. The UART Controller will send out the data stored in transmitter FIFO top location through the UART_TXD. Read Operation:\\nBy reading this register, the UART will return an 8-bit data received from receiving FIFO."]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Receive/Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dat](index.html) module"]
pub struct UART_DAT_SPEC;
impl crate::RegisterSpec for UART_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_dat::R](R) reader structure"]
impl crate::Readable for UART_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_dat::W](W) writer structure"]
impl crate::Writable for UART_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DAT to value 0"]
impl crate::Resettable for UART_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
