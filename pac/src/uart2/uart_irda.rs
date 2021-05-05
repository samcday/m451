#[doc = "Register `UART_IRDA` reader"]
pub struct R(crate::R<UART_IRDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IRDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_IRDA_SPEC>> for R {
    fn from(reader: crate::R<UART_IRDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IRDA` writer"]
pub struct W(crate::W<UART_IRDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IRDA_SPEC>;
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
impl core::convert::From<crate::W<UART_IRDA_SPEC>> for W {
    fn from(writer: crate::W<UART_IRDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IrDA Receiver/Transmitter Selection Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_A {
    #[doc = "0: IrDA Transmitter Disabled and Receiver Enabled. (Default)"]
    _0 = 0,
    #[doc = "1: IrDA Transmitter Enabled and Receiver Disabled"]
    _1 = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` reader - IrDA Receiver/Transmitter Selection Enable Bit"]
pub struct TXEN_R(crate::FieldReader<bool, TXEN_A>);
impl TXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::_0,
            true => TXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEN_A::_1
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, TXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` writer - IrDA Receiver/Transmitter Selection Enable Bit"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA Transmitter Disabled and Receiver Enabled. (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEN_A::_0)
    }
    #[doc = "IrDA Transmitter Enabled and Receiver Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEN_A::_1)
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
#[doc = "IrDA Inverse Transmitting Output Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINV_A {
    #[doc = "0: None inverse transmitting signal. (Default)"]
    _0 = 0,
    #[doc = "1: Inverse transmitting output signal"]
    _1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - IrDA Inverse Transmitting Output Signal"]
pub struct TXINV_R(crate::FieldReader<bool, TXINV_A>);
impl TXINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::_0,
            true => TXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXINV_A::_1
    }
}
impl core::ops::Deref for TXINV_R {
    type Target = crate::FieldReader<bool, TXINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXINV` writer - IrDA Inverse Transmitting Output Signal"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None inverse transmitting signal. (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXINV_A::_0)
    }
    #[doc = "Inverse transmitting output signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXINV_A::_1)
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
#[doc = "IrDA Inverse Receive Input Signal\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINV_A {
    #[doc = "0: None inverse receiving input signal"]
    _0 = 0,
    #[doc = "1: Inverse receiving input signal. (Default)"]
    _1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - IrDA Inverse Receive Input Signal"]
pub struct RXINV_R(crate::FieldReader<bool, RXINV_A>);
impl RXINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::_0,
            true => RXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXINV_A::_1
    }
}
impl core::ops::Deref for RXINV_R {
    type Target = crate::FieldReader<bool, RXINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINV` writer - IrDA Inverse Receive Input Signal"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "None inverse receiving input signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINV_A::_0)
    }
    #[doc = "Inverse receiving input signal. (Default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINV_A::_1)
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
    #[doc = "Bit 1 - IrDA Receiver/Transmitter Selection Enable Bit"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IrDA Inverse Transmitting Output Signal"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IrDA Inverse Receive Input Signal"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IrDA Receiver/Transmitter Selection Enable Bit"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 5 - IrDA Inverse Transmitting Output Signal"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 6 - IrDA Inverse Receive Input Signal"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART IrDA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_irda](index.html) module"]
pub struct UART_IRDA_SPEC;
impl crate::RegisterSpec for UART_IRDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_irda::R](R) reader structure"]
impl crate::Readable for UART_IRDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_irda::W](W) writer structure"]
impl crate::Writable for UART_IRDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IRDA to value 0x40"]
impl crate::Resettable for UART_IRDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
