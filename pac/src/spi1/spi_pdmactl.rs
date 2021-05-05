#[doc = "Register `SPI_PDMACTL` reader"]
pub struct R(crate::R<SPI_PDMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PDMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_PDMACTL_SPEC>> for R {
    fn from(reader: crate::R<SPI_PDMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_PDMACTL` writer"]
pub struct W(crate::W<SPI_PDMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PDMACTL_SPEC>;
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
impl core::convert::From<crate::W<SPI_PDMACTL_SPEC>> for W {
    fn from(writer: crate::W<SPI_PDMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit PDMA Enable Bit\\nNote: In SPI master mode with full duplex transfer, if both TX and RX PDMA functions are enabled, RX PDMA function cannot be enabled prior to TX PDMA function. User can enable TX PDMA function firstly or enable both functions simultaneously.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPDMAEN_A {
    #[doc = "0: Transmit PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Transmit PDMA function Enabled"]
    _1 = 1,
}
impl From<TXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPDMAEN` reader - Transmit PDMA Enable Bit\\nNote: In SPI master mode with full duplex transfer, if both TX and RX PDMA functions are enabled, RX PDMA function cannot be enabled prior to TX PDMA function. User can enable TX PDMA function firstly or enable both functions simultaneously."]
pub struct TXPDMAEN_R(crate::FieldReader<bool, TXPDMAEN_A>);
impl TXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPDMAEN_A {
        match self.bits {
            false => TXPDMAEN_A::_0,
            true => TXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXPDMAEN_A::_1
    }
}
impl core::ops::Deref for TXPDMAEN_R {
    type Target = crate::FieldReader<bool, TXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPDMAEN` writer - Transmit PDMA Enable Bit\\nNote: In SPI master mode with full duplex transfer, if both TX and RX PDMA functions are enabled, RX PDMA function cannot be enabled prior to TX PDMA function. User can enable TX PDMA function firstly or enable both functions simultaneously."]
pub struct TXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_0)
    }
    #[doc = "Transmit PDMA function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPDMAEN_A::_1)
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
#[doc = "Receive PDMA Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPDMAEN_A {
    #[doc = "0: Receive PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Receive PDMA function Enabled"]
    _1 = 1,
}
impl From<RXPDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXPDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPDMAEN` reader - Receive PDMA Enable Bit"]
pub struct RXPDMAEN_R(crate::FieldReader<bool, RXPDMAEN_A>);
impl RXPDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPDMAEN_A {
        match self.bits {
            false => RXPDMAEN_A::_0,
            true => RXPDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXPDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXPDMAEN_A::_1
    }
}
impl core::ops::Deref for RXPDMAEN_R {
    type Target = crate::FieldReader<bool, RXPDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPDMAEN` writer - Receive PDMA Enable Bit"]
pub struct RXPDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_0)
    }
    #[doc = "Receive PDMA function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPDMAEN_A::_1)
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
#[doc = "PDMA Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMARST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the PDMA control logic of the SPI controller. This bit will be automatically cleared to 0"]
    _1 = 1,
}
impl From<PDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: PDMARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMARST` reader - PDMA Reset"]
pub struct PDMARST_R(crate::FieldReader<bool, PDMARST_A>);
impl PDMARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMARST_A {
        match self.bits {
            false => PDMARST_A::_0,
            true => PDMARST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMARST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMARST_A::_1
    }
}
impl core::ops::Deref for PDMARST_R {
    type Target = crate::FieldReader<bool, PDMARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMARST` writer - PDMA Reset"]
pub struct PDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMARST_A::_0)
    }
    #[doc = "Reset the PDMA control logic of the SPI controller. This bit will be automatically cleared to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMARST_A::_1)
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
    #[doc = "Bit 0 - Transmit PDMA Enable Bit\\nNote: In SPI master mode with full duplex transfer, if both TX and RX PDMA functions are enabled, RX PDMA function cannot be enabled prior to TX PDMA function. User can enable TX PDMA function firstly or enable both functions simultaneously."]
    #[inline(always)]
    pub fn txpdmaen(&self) -> TXPDMAEN_R {
        TXPDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive PDMA Enable Bit"]
    #[inline(always)]
    pub fn rxpdmaen(&self) -> RXPDMAEN_R {
        RXPDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Reset"]
    #[inline(always)]
    pub fn pdmarst(&self) -> PDMARST_R {
        PDMARST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit PDMA Enable Bit\\nNote: In SPI master mode with full duplex transfer, if both TX and RX PDMA functions are enabled, RX PDMA function cannot be enabled prior to TX PDMA function. User can enable TX PDMA function firstly or enable both functions simultaneously."]
    #[inline(always)]
    pub fn txpdmaen(&mut self) -> TXPDMAEN_W {
        TXPDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Receive PDMA Enable Bit"]
    #[inline(always)]
    pub fn rxpdmaen(&mut self) -> RXPDMAEN_W {
        RXPDMAEN_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Reset"]
    #[inline(always)]
    pub fn pdmarst(&mut self) -> PDMARST_W {
        PDMARST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI PDMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_pdmactl](index.html) module"]
pub struct SPI_PDMACTL_SPEC;
impl crate::RegisterSpec for SPI_PDMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_pdmactl::R](R) reader structure"]
impl crate::Readable for SPI_PDMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_pdmactl::W](W) writer structure"]
impl crate::Writable for SPI_PDMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_PDMACTL to value 0"]
impl crate::Resettable for SPI_PDMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
