#[doc = "Register `CLK_AHBCLK` reader"]
pub struct R(crate::R<CLK_AHBCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_AHBCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_AHBCLK_SPEC>> for R {
    fn from(reader: crate::R<CLK_AHBCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_AHBCLK` writer"]
pub struct W(crate::W<CLK_AHBCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_AHBCLK_SPEC>;
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
impl core::convert::From<crate::W<CLK_AHBCLK_SPEC>> for W {
    fn from(writer: crate::W<CLK_AHBCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Controller Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMACKEN_A {
    #[doc = "0: PDMA peripheral clock Disabled"]
    _0 = 0,
    #[doc = "1: PDMA peripheral clock Enabled"]
    _1 = 1,
}
impl From<PDMACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMACKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMACKEN` reader - PDMA Controller Clock Enable Bit"]
pub struct PDMACKEN_R(crate::FieldReader<bool, PDMACKEN_A>);
impl PDMACKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMACKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMACKEN_A {
        match self.bits {
            false => PDMACKEN_A::_0,
            true => PDMACKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMACKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMACKEN_A::_1
    }
}
impl core::ops::Deref for PDMACKEN_R {
    type Target = crate::FieldReader<bool, PDMACKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMACKEN` writer - PDMA Controller Clock Enable Bit"]
pub struct PDMACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMACKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMACKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA peripheral clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMACKEN_A::_0)
    }
    #[doc = "PDMA peripheral clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMACKEN_A::_1)
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
#[doc = "Flash ISP Controller Clock Enable Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISPCKEN_A {
    #[doc = "0: Flash ISP peripheral clock Disabled"]
    _0 = 0,
    #[doc = "1: Flash ISP peripheral clock Enabled"]
    _1 = 1,
}
impl From<ISPCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ISPCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISPCKEN` reader - Flash ISP Controller Clock Enable Bit"]
pub struct ISPCKEN_R(crate::FieldReader<bool, ISPCKEN_A>);
impl ISPCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISPCKEN_A {
        match self.bits {
            false => ISPCKEN_A::_0,
            true => ISPCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISPCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISPCKEN_A::_1
    }
}
impl core::ops::Deref for ISPCKEN_R {
    type Target = crate::FieldReader<bool, ISPCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPCKEN` writer - Flash ISP Controller Clock Enable Bit"]
pub struct ISPCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISPCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash ISP peripheral clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISPCKEN_A::_0)
    }
    #[doc = "Flash ISP peripheral clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISPCKEN_A::_1)
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
#[doc = "EBI Controller Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBICKEN_A {
    #[doc = "0: EBI peripheral clock Disabled"]
    _0 = 0,
    #[doc = "1: EBI peripheral clock Enabled"]
    _1 = 1,
}
impl From<EBICKEN_A> for bool {
    #[inline(always)]
    fn from(variant: EBICKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBICKEN` reader - EBI Controller Clock Enable Bit"]
pub struct EBICKEN_R(crate::FieldReader<bool, EBICKEN_A>);
impl EBICKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBICKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBICKEN_A {
        match self.bits {
            false => EBICKEN_A::_0,
            true => EBICKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EBICKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EBICKEN_A::_1
    }
}
impl core::ops::Deref for EBICKEN_R {
    type Target = crate::FieldReader<bool, EBICKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBICKEN` writer - EBI Controller Clock Enable Bit"]
pub struct EBICKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBICKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBICKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EBI peripheral clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EBICKEN_A::_0)
    }
    #[doc = "EBI peripheral clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EBICKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "USB HOST Controller Clock Enable Bit (M45xG/M45xE Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHCKEN_A {
    #[doc = "0: USB HOST peripheral clock Disabled"]
    _0 = 0,
    #[doc = "1: USB HOST peripheral clock Enabled"]
    _1 = 1,
}
impl From<USBHCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBHCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHCKEN` reader - USB HOST Controller Clock Enable Bit (M45xG/M45xE Only)"]
pub struct USBHCKEN_R(crate::FieldReader<bool, USBHCKEN_A>);
impl USBHCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHCKEN_A {
        match self.bits {
            false => USBHCKEN_A::_0,
            true => USBHCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBHCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBHCKEN_A::_1
    }
}
impl core::ops::Deref for USBHCKEN_R {
    type Target = crate::FieldReader<bool, USBHCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHCKEN` writer - USB HOST Controller Clock Enable Bit (M45xG/M45xE Only)"]
pub struct USBHCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB HOST peripheral clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBHCKEN_A::_0)
    }
    #[doc = "USB HOST peripheral clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBHCKEN_A::_1)
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
#[doc = "CRC Generator Controller Clock Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCCKEN_A {
    #[doc = "0: CRC peripheral clock Disabled"]
    _0 = 0,
    #[doc = "1: CRC peripheral clock Enabled"]
    _1 = 1,
}
impl From<CRCCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCCKEN` reader - CRC Generator Controller Clock Enable Bit"]
pub struct CRCCKEN_R(crate::FieldReader<bool, CRCCKEN_A>);
impl CRCCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCCKEN_A {
        match self.bits {
            false => CRCCKEN_A::_0,
            true => CRCCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCCKEN_A::_1
    }
}
impl core::ops::Deref for CRCCKEN_R {
    type Target = crate::FieldReader<bool, CRCCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCCKEN` writer - CRC Generator Controller Clock Enable Bit"]
pub struct CRCCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CRC peripheral clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCCKEN_A::_0)
    }
    #[doc = "CRC peripheral clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCCKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Flash Memory Controller Clock Enable Bit in IDLE Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCIDLE_A {
    #[doc = "0: FMC peripheral clock Disabled when chip operating at IDLE mode"]
    _0 = 0,
    #[doc = "1: FMC peripheral clock Enabled when chip operating at IDLE mode"]
    _1 = 1,
}
impl From<FMCIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: FMCIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCIDLE` reader - Flash Memory Controller Clock Enable Bit in IDLE Mode"]
pub struct FMCIDLE_R(crate::FieldReader<bool, FMCIDLE_A>);
impl FMCIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCIDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCIDLE_A {
        match self.bits {
            false => FMCIDLE_A::_0,
            true => FMCIDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FMCIDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FMCIDLE_A::_1
    }
}
impl core::ops::Deref for FMCIDLE_R {
    type Target = crate::FieldReader<bool, FMCIDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCIDLE` writer - Flash Memory Controller Clock Enable Bit in IDLE Mode"]
pub struct FMCIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCIDLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FMC peripheral clock Disabled when chip operating at IDLE mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FMCIDLE_A::_0)
    }
    #[doc = "FMC peripheral clock Enabled when chip operating at IDLE mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FMCIDLE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - PDMA Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn pdmacken(&self) -> PDMACKEN_R {
        PDMACKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash ISP Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn ispcken(&self) -> ISPCKEN_R {
        ISPCKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EBI Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn ebicken(&self) -> EBICKEN_R {
        EBICKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB HOST Controller Clock Enable Bit (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn usbhcken(&self) -> USBHCKEN_R {
        USBHCKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC Generator Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn crccken(&self) -> CRCCKEN_R {
        CRCCKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flash Memory Controller Clock Enable Bit in IDLE Mode"]
    #[inline(always)]
    pub fn fmcidle(&self) -> FMCIDLE_R {
        FMCIDLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PDMA Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn pdmacken(&mut self) -> PDMACKEN_W {
        PDMACKEN_W { w: self }
    }
    #[doc = "Bit 2 - Flash ISP Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn ispcken(&mut self) -> ISPCKEN_W {
        ISPCKEN_W { w: self }
    }
    #[doc = "Bit 3 - EBI Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn ebicken(&mut self) -> EBICKEN_W {
        EBICKEN_W { w: self }
    }
    #[doc = "Bit 4 - USB HOST Controller Clock Enable Bit (M45xG/M45xE Only)"]
    #[inline(always)]
    pub fn usbhcken(&mut self) -> USBHCKEN_W {
        USBHCKEN_W { w: self }
    }
    #[doc = "Bit 7 - CRC Generator Controller Clock Enable Bit"]
    #[inline(always)]
    pub fn crccken(&mut self) -> CRCCKEN_W {
        CRCCKEN_W { w: self }
    }
    #[doc = "Bit 15 - Flash Memory Controller Clock Enable Bit in IDLE Mode"]
    #[inline(always)]
    pub fn fmcidle(&mut self) -> FMCIDLE_W {
        FMCIDLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Devices Clock Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ahbclk](index.html) module"]
pub struct CLK_AHBCLK_SPEC;
impl crate::RegisterSpec for CLK_AHBCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ahbclk::R](R) reader structure"]
impl crate::Readable for CLK_AHBCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ahbclk::W](W) writer structure"]
impl crate::Writable for CLK_AHBCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_AHBCLK to value 0x8004"]
impl crate::Resettable for CLK_AHBCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8004
    }
}
