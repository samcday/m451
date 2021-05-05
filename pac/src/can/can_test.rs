#[doc = "Register `CAN_TEST` reader"]
pub struct R(crate::R<CAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_TEST_SPEC>> for R {
    fn from(reader: crate::R<CAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_TEST` writer"]
pub struct W(crate::W<CAN_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_TEST_SPEC>;
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
impl core::convert::From<crate::W<CAN_TEST_SPEC>> for W {
    fn from(writer: crate::W<CAN_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Basic Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASIC_A {
    #[doc = "0: Basic Mode Disabled"]
    _0 = 0,
    #[doc = "1: IF1 Registers used as Tx Buffer, IF2 Registers used as Rx Buffer"]
    _1 = 1,
}
impl From<BASIC_A> for bool {
    #[inline(always)]
    fn from(variant: BASIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Basic` reader - Basic Mode"]
pub struct BASIC_R(crate::FieldReader<bool, BASIC_A>);
impl BASIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BASIC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BASIC_A {
        match self.bits {
            false => BASIC_A::_0,
            true => BASIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BASIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BASIC_A::_1
    }
}
impl core::ops::Deref for BASIC_R {
    type Target = crate::FieldReader<bool, BASIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Basic` writer - Basic Mode"]
pub struct BASIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BASIC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Basic Mode Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BASIC_A::_0)
    }
    #[doc = "IF1 Registers used as Tx Buffer, IF2 Registers used as Rx Buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BASIC_A::_1)
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
#[doc = "Silent Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILENT_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: The module is in Silent Mode"]
    _1 = 1,
}
impl From<SILENT_A> for bool {
    #[inline(always)]
    fn from(variant: SILENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Silent` reader - Silent Mode"]
pub struct SILENT_R(crate::FieldReader<bool, SILENT_A>);
impl SILENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SILENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SILENT_A {
        match self.bits {
            false => SILENT_A::_0,
            true => SILENT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SILENT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SILENT_A::_1
    }
}
impl core::ops::Deref for SILENT_R {
    type Target = crate::FieldReader<bool, SILENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Silent` writer - Silent Mode"]
pub struct SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SILENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SILENT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SILENT_A::_0)
    }
    #[doc = "The module is in Silent Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SILENT_A::_1)
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
#[doc = "Loop Back Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBACK_A {
    #[doc = "0: Loop Back Mode is Disabled"]
    _0 = 0,
    #[doc = "1: Loop Back Mode is Enabled"]
    _1 = 1,
}
impl From<LBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LBACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBack` reader - Loop Back Mode Enable Bit"]
pub struct LBACK_R(crate::FieldReader<bool, LBACK_A>);
impl LBACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBACK_A {
        match self.bits {
            false => LBACK_A::_0,
            true => LBACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LBACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LBACK_A::_1
    }
}
impl core::ops::Deref for LBACK_R {
    type Target = crate::FieldReader<bool, LBACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBack` writer - Loop Back Mode Enable Bit"]
pub struct LBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loop Back Mode is Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBACK_A::_0)
    }
    #[doc = "Loop Back Mode is Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBACK_A::_1)
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
#[doc = "Tx Control of CAN_TX Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, CAN_TX pin is controlled by the CAN Core"]
    _0 = 0,
    #[doc = "1: Sample Point can be monitored at CAN_TX pin"]
    _1 = 1,
    #[doc = "2: CAN_TX pin drives a dominant ('0') value"]
    _2 = 2,
    #[doc = "3: CAN_TX pin drives a recessive ('1') value"]
    _3 = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Tx` reader - Tx Control of CAN_TX Pin"]
pub struct TX_R(crate::FieldReader<u8, TX_A>);
impl TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::_0,
            1 => TX_A::_1,
            2 => TX_A::_2,
            3 => TX_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TX_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TX_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TX_A::_3
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<u8, TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Tx` writer - Tx Control of CAN_TX Pin"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reset value, CAN_TX pin is controlled by the CAN Core"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_A::_0)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_A::_1)
    }
    #[doc = "CAN_TX pin drives a dominant ('0') value"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_A::_2)
    }
    #[doc = "CAN_TX pin drives a recessive ('1') value"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Monitors the Actual Value of CAN_RX Pin (Read Only) *(1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: The CAN bus is dominant (CAN_RX = '0')"]
    _0 = 0,
    #[doc = "1: The CAN bus is recessive (CAN_RX = '1')"]
    _1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Rx` reader - Monitors the Actual Value of CAN_RX Pin (Read Only) *(1)"]
pub struct RX_R(crate::FieldReader<bool, RX_A>);
impl RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::_0,
            true => RX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RX_A::_1
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode Enable Bit"]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Tx Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin (Read Only) *(1)"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W { w: self }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W { w: self }
    }
    #[doc = "Bit 4 - Loop Back Mode Enable Bit"]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W { w: self }
    }
    #[doc = "Bits 5:6 - Tx Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register (Register Map Note 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_test](index.html) module"]
pub struct CAN_TEST_SPEC;
impl crate::RegisterSpec for CAN_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_test::R](R) reader structure"]
impl crate::Readable for CAN_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_test::W](W) writer structure"]
impl crate::Writable for CAN_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_TEST to value 0x80"]
impl crate::Resettable for CAN_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
