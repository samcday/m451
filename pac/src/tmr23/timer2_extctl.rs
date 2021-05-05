#[doc = "Register `TIMER2_EXTCTL` reader"]
pub struct R(crate::R<TIMER2_EXTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_EXTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER2_EXTCTL_SPEC>> for R {
    fn from(reader: crate::R<TIMER2_EXTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_EXTCTL` writer"]
pub struct W(crate::W<TIMER2_EXTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_EXTCTL_SPEC>;
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
impl core::convert::From<crate::W<TIMER2_EXTCTL_SPEC>> for W {
    fn from(writer: crate::W<TIMER2_EXTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer External Count Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTPHASE_A {
    #[doc = "0: A Falling edge of external counting pin will be counted"]
    _0 = 0,
    #[doc = "1: A Rising edge of external counting pin will be counted"]
    _1 = 1,
}
impl From<CNTPHASE_A> for bool {
    #[inline(always)]
    fn from(variant: CNTPHASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTPHASE` reader - Timer External Count Phase"]
pub struct CNTPHASE_R(crate::FieldReader<bool, CNTPHASE_A>);
impl CNTPHASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTPHASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTPHASE_A {
        match self.bits {
            false => CNTPHASE_A::_0,
            true => CNTPHASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTPHASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTPHASE_A::_1
    }
}
impl core::ops::Deref for CNTPHASE_R {
    type Target = crate::FieldReader<bool, CNTPHASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTPHASE` writer - Timer External Count Phase"]
pub struct CNTPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTPHASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A Falling edge of external counting pin will be counted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTPHASE_A::_0)
    }
    #[doc = "A Rising edge of external counting pin will be counted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTPHASE_A::_1)
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
#[doc = "Timer External Capture Pin Edge Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPEDGE_A {
    #[doc = "0: A Falling edge on Tx_EXT (x= 0~3) pin will be detected"]
    _0 = 0,
    #[doc = "1: A Rising edge on Tx_EXT (x= 0~3) pin will be detected"]
    _1 = 1,
    #[doc = "2: Either Rising or Falling edge on Tx_EXT (x= 0~3) pin will be detected"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<CAPEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPEDGE` reader - Timer External Capture Pin Edge Detect"]
pub struct CAPEDGE_R(crate::FieldReader<u8, CAPEDGE_A>);
impl CAPEDGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPEDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEDGE_A {
        match self.bits {
            0 => CAPEDGE_A::_0,
            1 => CAPEDGE_A::_1,
            2 => CAPEDGE_A::_2,
            3 => CAPEDGE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEDGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEDGE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CAPEDGE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CAPEDGE_A::_3
    }
}
impl core::ops::Deref for CAPEDGE_R {
    type Target = crate::FieldReader<u8, CAPEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEDGE` writer - Timer External Capture Pin Edge Detect"]
pub struct CAPEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEDGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "A Falling edge on Tx_EXT (x= 0~3) pin will be detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEDGE_A::_0)
    }
    #[doc = "A Rising edge on Tx_EXT (x= 0~3) pin will be detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEDGE_A::_1)
    }
    #[doc = "Either Rising or Falling edge on Tx_EXT (x= 0~3) pin will be detected"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CAPEDGE_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CAPEDGE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Timer External Capture Pin Enable Bit\\nThis bit enables the Tx_EXT pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPEN_A {
    #[doc = "0: Tx_EXT (x= 0~3) pin Disabled"]
    _0 = 0,
    #[doc = "1: Tx_EXT (x= 0~3) pin Enabled"]
    _1 = 1,
}
impl From<CAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPEN` reader - Timer External Capture Pin Enable Bit\\nThis bit enables the Tx_EXT pin."]
pub struct CAPEN_R(crate::FieldReader<bool, CAPEN_A>);
impl CAPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN_A {
        match self.bits {
            false => CAPEN_A::_0,
            true => CAPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPEN_A::_1
    }
}
impl core::ops::Deref for CAPEN_R {
    type Target = crate::FieldReader<bool, CAPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN` writer - Timer External Capture Pin Enable Bit\\nThis bit enables the Tx_EXT pin."]
pub struct CAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx_EXT (x= 0~3) pin Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPEN_A::_0)
    }
    #[doc = "Tx_EXT (x= 0~3) pin Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPEN_A::_1)
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
#[doc = "Capture Function Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPFUNCS_A {
    #[doc = "0: External Capture Mode Enabled"]
    _0 = 0,
    #[doc = "1: External Reset Mode Enabled"]
    _1 = 1,
}
impl From<CAPFUNCS_A> for bool {
    #[inline(always)]
    fn from(variant: CAPFUNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPFUNCS` reader - Capture Function Selection"]
pub struct CAPFUNCS_R(crate::FieldReader<bool, CAPFUNCS_A>);
impl CAPFUNCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPFUNCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPFUNCS_A {
        match self.bits {
            false => CAPFUNCS_A::_0,
            true => CAPFUNCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPFUNCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPFUNCS_A::_1
    }
}
impl core::ops::Deref for CAPFUNCS_R {
    type Target = crate::FieldReader<bool, CAPFUNCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPFUNCS` writer - Capture Function Selection"]
pub struct CAPFUNCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPFUNCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPFUNCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External Capture Mode Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPFUNCS_A::_0)
    }
    #[doc = "External Reset Mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPFUNCS_A::_1)
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
#[doc = "Timer External Capture Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPIEN_A {
    #[doc = "0: Tx_EXT (x= 0~3) pin detection Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Tx_EXT (x= 0~3) pin detection Interrupt Enabled"]
    _1 = 1,
}
impl From<CAPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPIEN` reader - Timer External Capture Interrupt Enable Bit"]
pub struct CAPIEN_R(crate::FieldReader<bool, CAPIEN_A>);
impl CAPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPIEN_A {
        match self.bits {
            false => CAPIEN_A::_0,
            true => CAPIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPIEN_A::_1
    }
}
impl core::ops::Deref for CAPIEN_R {
    type Target = crate::FieldReader<bool, CAPIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPIEN` writer - Timer External Capture Interrupt Enable Bit"]
pub struct CAPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx_EXT (x= 0~3) pin detection Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPIEN_A::_0)
    }
    #[doc = "Tx_EXT (x= 0~3) pin detection Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPIEN_A::_1)
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
#[doc = "Timer External Capture Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx_EXT pin is detected with de-bounce circuit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPDBEN_A {
    #[doc = "0: Tx_EXT (x= 0~3) pin de-bounce Disabled"]
    _0 = 0,
    #[doc = "1: Tx_EXT (x= 0~3) pin de-bounce Enabled"]
    _1 = 1,
}
impl From<CAPDBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAPDBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPDBEN` reader - Timer External Capture Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx_EXT pin is detected with de-bounce circuit."]
pub struct CAPDBEN_R(crate::FieldReader<bool, CAPDBEN_A>);
impl CAPDBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPDBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPDBEN_A {
        match self.bits {
            false => CAPDBEN_A::_0,
            true => CAPDBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPDBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPDBEN_A::_1
    }
}
impl core::ops::Deref for CAPDBEN_R {
    type Target = crate::FieldReader<bool, CAPDBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPDBEN` writer - Timer External Capture Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx_EXT pin is detected with de-bounce circuit."]
pub struct CAPDBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPDBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPDBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx_EXT (x= 0~3) pin de-bounce Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPDBEN_A::_0)
    }
    #[doc = "Tx_EXT (x= 0~3) pin de-bounce Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPDBEN_A::_1)
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
#[doc = "Timer Counter Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx pin is detected with de-bounce circuit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTDBEN_A {
    #[doc = "0: Tx (x= 0~3) pin de-bounce Disabled"]
    _0 = 0,
    #[doc = "1: Tx (x= 0~3) pin de-bounce Enabled"]
    _1 = 1,
}
impl From<CNTDBEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTDBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTDBEN` reader - Timer Counter Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx pin is detected with de-bounce circuit."]
pub struct CNTDBEN_R(crate::FieldReader<bool, CNTDBEN_A>);
impl CNTDBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTDBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTDBEN_A {
        match self.bits {
            false => CNTDBEN_A::_0,
            true => CNTDBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTDBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTDBEN_A::_1
    }
}
impl core::ops::Deref for CNTDBEN_R {
    type Target = crate::FieldReader<bool, CNTDBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTDBEN` writer - Timer Counter Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx pin is detected with de-bounce circuit."]
pub struct CNTDBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTDBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTDBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx (x= 0~3) pin de-bounce Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTDBEN_A::_0)
    }
    #[doc = "Tx (x= 0~3) pin de-bounce Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTDBEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Timer External Count Phase"]
    #[inline(always)]
    pub fn cntphase(&self) -> CNTPHASE_R {
        CNTPHASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Timer External Capture Pin Edge Detect"]
    #[inline(always)]
    pub fn capedge(&self) -> CAPEDGE_R {
        CAPEDGE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Timer External Capture Pin Enable Bit\\nThis bit enables the Tx_EXT pin."]
    #[inline(always)]
    pub fn capen(&self) -> CAPEN_R {
        CAPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Function Selection"]
    #[inline(always)]
    pub fn capfuncs(&self) -> CAPFUNCS_R {
        CAPFUNCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer External Capture Interrupt Enable Bit"]
    #[inline(always)]
    pub fn capien(&self) -> CAPIEN_R {
        CAPIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer External Capture Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx_EXT pin is detected with de-bounce circuit."]
    #[inline(always)]
    pub fn capdben(&self) -> CAPDBEN_R {
        CAPDBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Counter Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx pin is detected with de-bounce circuit."]
    #[inline(always)]
    pub fn cntdben(&self) -> CNTDBEN_R {
        CNTDBEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer External Count Phase"]
    #[inline(always)]
    pub fn cntphase(&mut self) -> CNTPHASE_W {
        CNTPHASE_W { w: self }
    }
    #[doc = "Bits 1:2 - Timer External Capture Pin Edge Detect"]
    #[inline(always)]
    pub fn capedge(&mut self) -> CAPEDGE_W {
        CAPEDGE_W { w: self }
    }
    #[doc = "Bit 3 - Timer External Capture Pin Enable Bit\\nThis bit enables the Tx_EXT pin."]
    #[inline(always)]
    pub fn capen(&mut self) -> CAPEN_W {
        CAPEN_W { w: self }
    }
    #[doc = "Bit 4 - Capture Function Selection"]
    #[inline(always)]
    pub fn capfuncs(&mut self) -> CAPFUNCS_W {
        CAPFUNCS_W { w: self }
    }
    #[doc = "Bit 5 - Timer External Capture Interrupt Enable Bit"]
    #[inline(always)]
    pub fn capien(&mut self) -> CAPIEN_W {
        CAPIEN_W { w: self }
    }
    #[doc = "Bit 6 - Timer External Capture Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx_EXT pin is detected with de-bounce circuit."]
    #[inline(always)]
    pub fn capdben(&mut self) -> CAPDBEN_W {
        CAPDBEN_W { w: self }
    }
    #[doc = "Bit 7 - Timer Counter Pin De-bounce Enable Bit\\nNote: If this bit is enabled, the edge detection of Tx pin is detected with de-bounce circuit."]
    #[inline(always)]
    pub fn cntdben(&mut self) -> CNTDBEN_W {
        CNTDBEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer2 External Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_extctl](index.html) module"]
pub struct TIMER2_EXTCTL_SPEC;
impl crate::RegisterSpec for TIMER2_EXTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_extctl::R](R) reader structure"]
impl crate::Readable for TIMER2_EXTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_extctl::W](W) writer structure"]
impl crate::Writable for TIMER2_EXTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER2_EXTCTL to value 0"]
impl crate::Resettable for TIMER2_EXTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
