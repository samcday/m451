#[doc = "Register `CRC_CTL` reader"]
pub struct R(crate::R<CRC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_CTL_SPEC>> for R {
    fn from(reader: crate::R<CRC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_CTL` writer"]
pub struct W(crate::W<CRC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CTL_SPEC>;
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
impl core::convert::From<crate::W<CRC_CTL_SPEC>> for W {
    fn from(writer: crate::W<CRC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC Channel Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEN_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: CRC operation Enabled"]
    _1 = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC Channel Enable Bit"]
pub struct CRCEN_R(crate::FieldReader<bool, CRCEN_A>);
impl CRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::_0,
            true => CRCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCEN_A::_1
    }
}
impl core::ops::Deref for CRCEN_R {
    type Target = crate::FieldReader<bool, CRCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEN` writer - CRC Channel Enable Bit"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCEN_A::_0)
    }
    #[doc = "CRC operation Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCEN_A::_1)
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
#[doc = "CRC Engine Reset\\nNote1: This bit will be cleared automatically.\\nNote2: Setting this bit will reload the seed value from CRC_SEED register as checksum initial vale.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the internal CRC state machine. The others contents of CRC_CTL register will not be cleared"]
    _1 = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC Engine Reset\\nNote1: This bit will be cleared automatically.\\nNote2: Setting this bit will reload the seed value from CRC_SEED register as checksum initial vale."]
pub struct CRCRST_R(crate::FieldReader<bool, CRCRST_A>);
impl CRCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::_0,
            true => CRCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCRST_A::_1
    }
}
impl core::ops::Deref for CRCRST_R {
    type Target = crate::FieldReader<bool, CRCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCRST` writer - CRC Engine Reset\\nNote1: This bit will be cleared automatically.\\nNote2: Setting this bit will reload the seed value from CRC_SEED register as checksum initial vale."]
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCRST_A::_0)
    }
    #[doc = "Reset the internal CRC state machine. The others contents of CRC_CTL register will not be cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCRST_A::_1)
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
#[doc = "Write Data Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_DAT register.\\nNote: If the write data is 0xAABBCCDD, the bit order reverse for CRC write data is 0x55DD33BB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATREV_A {
    #[doc = "0: Bit order reversed for CRC write data in Disabled"]
    _0 = 0,
    #[doc = "1: Bit order reversed for CRC write data in Enabled (per byte)"]
    _1 = 1,
}
impl From<DATREV_A> for bool {
    #[inline(always)]
    fn from(variant: DATREV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATREV` reader - Write Data Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_DAT register.\\nNote: If the write data is 0xAABBCCDD, the bit order reverse for CRC write data is 0x55DD33BB."]
pub struct DATREV_R(crate::FieldReader<bool, DATREV_A>);
impl DATREV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATREV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATREV_A {
        match self.bits {
            false => DATREV_A::_0,
            true => DATREV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATREV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATREV_A::_1
    }
}
impl core::ops::Deref for DATREV_R {
    type Target = crate::FieldReader<bool, DATREV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATREV` writer - Write Data Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_DAT register.\\nNote: If the write data is 0xAABBCCDD, the bit order reverse for CRC write data is 0x55DD33BB."]
pub struct DATREV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATREV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit order reversed for CRC write data in Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATREV_A::_0)
    }
    #[doc = "Bit order reversed for CRC write data in Enabled (per byte)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATREV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Checksum Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_CHECKSUM register.\\nNote: If the checksum result is 0xDD7B0F2E, the bit order reverse for CRC checksum is 0x74F0DEBB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHKSREV_A {
    #[doc = "0: Bit order reverse for CRC checksum Disabled"]
    _0 = 0,
    #[doc = "1: Bit order reverse for CRC checksum Enabled"]
    _1 = 1,
}
impl From<CHKSREV_A> for bool {
    #[inline(always)]
    fn from(variant: CHKSREV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHKSREV` reader - Checksum Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_CHECKSUM register.\\nNote: If the checksum result is 0xDD7B0F2E, the bit order reverse for CRC checksum is 0x74F0DEBB."]
pub struct CHKSREV_R(crate::FieldReader<bool, CHKSREV_A>);
impl CHKSREV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHKSREV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHKSREV_A {
        match self.bits {
            false => CHKSREV_A::_0,
            true => CHKSREV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHKSREV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHKSREV_A::_1
    }
}
impl core::ops::Deref for CHKSREV_R {
    type Target = crate::FieldReader<bool, CHKSREV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKSREV` writer - Checksum Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_CHECKSUM register.\\nNote: If the checksum result is 0xDD7B0F2E, the bit order reverse for CRC checksum is 0x74F0DEBB."]
pub struct CHKSREV_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKSREV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit order reverse for CRC checksum Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHKSREV_A::_0)
    }
    #[doc = "Bit order reverse for CRC checksum Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSREV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Write Data 1's Complement\\nThis bit is used to enable the 1's complement function for write data value in CRC_DAT register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATFMT_A {
    #[doc = "0: 1's complement for CRC writes data in Disabled"]
    _0 = 0,
    #[doc = "1: 1's complement for CRC writes data in Enabled"]
    _1 = 1,
}
impl From<DATFMT_A> for bool {
    #[inline(always)]
    fn from(variant: DATFMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATFMT` reader - Write Data 1's Complement\\nThis bit is used to enable the 1's complement function for write data value in CRC_DAT register."]
pub struct DATFMT_R(crate::FieldReader<bool, DATFMT_A>);
impl DATFMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATFMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATFMT_A {
        match self.bits {
            false => DATFMT_A::_0,
            true => DATFMT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATFMT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATFMT_A::_1
    }
}
impl core::ops::Deref for DATFMT_R {
    type Target = crate::FieldReader<bool, DATFMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATFMT` writer - Write Data 1's Complement\\nThis bit is used to enable the 1's complement function for write data value in CRC_DAT register."]
pub struct DATFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATFMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATFMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1's complement for CRC writes data in Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATFMT_A::_0)
    }
    #[doc = "1's complement for CRC writes data in Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATFMT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Checksum 1's Complement\\nThis bit is used to enable the 1's complement function for checksum result in CRC_CHECKSUM register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHKSFMT_A {
    #[doc = "0: 1's complement for CRC checksum Disabled"]
    _0 = 0,
    #[doc = "1: 1's complement for CRC checksum Enabled"]
    _1 = 1,
}
impl From<CHKSFMT_A> for bool {
    #[inline(always)]
    fn from(variant: CHKSFMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHKSFMT` reader - Checksum 1's Complement\\nThis bit is used to enable the 1's complement function for checksum result in CRC_CHECKSUM register."]
pub struct CHKSFMT_R(crate::FieldReader<bool, CHKSFMT_A>);
impl CHKSFMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHKSFMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHKSFMT_A {
        match self.bits {
            false => CHKSFMT_A::_0,
            true => CHKSFMT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHKSFMT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHKSFMT_A::_1
    }
}
impl core::ops::Deref for CHKSFMT_R {
    type Target = crate::FieldReader<bool, CHKSFMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKSFMT` writer - Checksum 1's Complement\\nThis bit is used to enable the 1's complement function for checksum result in CRC_CHECKSUM register."]
pub struct CHKSFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSFMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKSFMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1's complement for CRC checksum Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHKSFMT_A::_0)
    }
    #[doc = "1's complement for CRC checksum Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSFMT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "CPU Write Data Length\\nThis field indicates the write data length.\\nNote: When the write data length is 8-bit mode, the valid data in CRC_DAT register is only DATA\\[7:0\\]
bits; if the write data length is 16-bit mode, the valid data in CRC_DAT register is only DATA\\[15:0\\].\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: Data length is 8-bit mode"]
    _0 = 0,
    #[doc = "1: Data length is 16-bit mode.\\nData length is 32-bit mode"]
    _1 = 1,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATLEN` reader - CPU Write Data Length\\nThis field indicates the write data length.\\nNote: When the write data length is 8-bit mode, the valid data in CRC_DAT register is only DATA\\[7:0\\]
bits; if the write data length is 16-bit mode, the valid data in CRC_DAT register is only DATA\\[15:0\\]."]
pub struct DATLEN_R(crate::FieldReader<u8, DATLEN_A>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::_0),
            1 => Some(DATLEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATLEN_A::_1
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, DATLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLEN` writer - CPU Write Data Length\\nThis field indicates the write data length.\\nNote: When the write data length is 8-bit mode, the valid data in CRC_DAT register is only DATA\\[7:0\\]
bits; if the write data length is 16-bit mode, the valid data in CRC_DAT register is only DATA\\[15:0\\]."]
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data length is 8-bit mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATLEN_A::_0)
    }
    #[doc = "Data length is 16-bit mode.\\nData length is 32-bit mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATLEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "CRC Polynomial Mode\\nThis field indicates the CRC operation polynomial mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCMODE_A {
    #[doc = "0: CRC-CCITT Polynomial mode"]
    _0 = 0,
    #[doc = "1: CRC-8 Polynomial mode"]
    _1 = 1,
    #[doc = "2: CRC-16 Polynomial mode"]
    _2 = 2,
    #[doc = "3: CRC-32 Polynomial mode"]
    _3 = 3,
}
impl From<CRCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCMODE` reader - CRC Polynomial Mode\\nThis field indicates the CRC operation polynomial mode."]
pub struct CRCMODE_R(crate::FieldReader<u8, CRCMODE_A>);
impl CRCMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCMODE_A {
        match self.bits {
            0 => CRCMODE_A::_0,
            1 => CRCMODE_A::_1,
            2 => CRCMODE_A::_2,
            3 => CRCMODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCMODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CRCMODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CRCMODE_A::_3
    }
}
impl core::ops::Deref for CRCMODE_R {
    type Target = crate::FieldReader<u8, CRCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCMODE` writer - CRC Polynomial Mode\\nThis field indicates the CRC operation polynomial mode."]
pub struct CRCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CRC-CCITT Polynomial mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCMODE_A::_0)
    }
    #[doc = "CRC-8 Polynomial mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCMODE_A::_1)
    }
    #[doc = "CRC-16 Polynomial mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CRCMODE_A::_2)
    }
    #[doc = "CRC-32 Polynomial mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CRCMODE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC Channel Enable Bit"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Engine Reset\\nNote1: This bit will be cleared automatically.\\nNote2: Setting this bit will reload the seed value from CRC_SEED register as checksum initial vale."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write Data Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_DAT register.\\nNote: If the write data is 0xAABBCCDD, the bit order reverse for CRC write data is 0x55DD33BB."]
    #[inline(always)]
    pub fn datrev(&self) -> DATREV_R {
        DATREV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Checksum Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_CHECKSUM register.\\nNote: If the checksum result is 0xDD7B0F2E, the bit order reverse for CRC checksum is 0x74F0DEBB."]
    #[inline(always)]
    pub fn chksrev(&self) -> CHKSREV_R {
        CHKSREV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write Data 1's Complement\\nThis bit is used to enable the 1's complement function for write data value in CRC_DAT register."]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Checksum 1's Complement\\nThis bit is used to enable the 1's complement function for checksum result in CRC_CHECKSUM register."]
    #[inline(always)]
    pub fn chksfmt(&self) -> CHKSFMT_R {
        CHKSFMT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - CPU Write Data Length\\nThis field indicates the write data length.\\nNote: When the write data length is 8-bit mode, the valid data in CRC_DAT register is only DATA\\[7:0\\]
bits; if the write data length is 16-bit mode, the valid data in CRC_DAT register is only DATA\\[15:0\\]."]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - CRC Polynomial Mode\\nThis field indicates the CRC operation polynomial mode."]
    #[inline(always)]
    pub fn crcmode(&self) -> CRCMODE_R {
        CRCMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Channel Enable Bit"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 1 - CRC Engine Reset\\nNote1: This bit will be cleared automatically.\\nNote2: Setting this bit will reload the seed value from CRC_SEED register as checksum initial vale."]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
    #[doc = "Bit 24 - Write Data Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_DAT register.\\nNote: If the write data is 0xAABBCCDD, the bit order reverse for CRC write data is 0x55DD33BB."]
    #[inline(always)]
    pub fn datrev(&mut self) -> DATREV_W {
        DATREV_W { w: self }
    }
    #[doc = "Bit 25 - Checksum Bit Order Reverse\\nThis bit is used to enable the bit order reverse function for write data value in CRC_CHECKSUM register.\\nNote: If the checksum result is 0xDD7B0F2E, the bit order reverse for CRC checksum is 0x74F0DEBB."]
    #[inline(always)]
    pub fn chksrev(&mut self) -> CHKSREV_W {
        CHKSREV_W { w: self }
    }
    #[doc = "Bit 26 - Write Data 1's Complement\\nThis bit is used to enable the 1's complement function for write data value in CRC_DAT register."]
    #[inline(always)]
    pub fn datfmt(&mut self) -> DATFMT_W {
        DATFMT_W { w: self }
    }
    #[doc = "Bit 27 - Checksum 1's Complement\\nThis bit is used to enable the 1's complement function for checksum result in CRC_CHECKSUM register."]
    #[inline(always)]
    pub fn chksfmt(&mut self) -> CHKSFMT_W {
        CHKSFMT_W { w: self }
    }
    #[doc = "Bits 28:29 - CPU Write Data Length\\nThis field indicates the write data length.\\nNote: When the write data length is 8-bit mode, the valid data in CRC_DAT register is only DATA\\[7:0\\]
bits; if the write data length is 16-bit mode, the valid data in CRC_DAT register is only DATA\\[15:0\\]."]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    #[doc = "Bits 30:31 - CRC Polynomial Mode\\nThis field indicates the CRC operation polynomial mode."]
    #[inline(always)]
    pub fn crcmode(&mut self) -> CRCMODE_W {
        CRCMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctl](index.html) module"]
pub struct CRC_CTL_SPEC;
impl crate::RegisterSpec for CRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_ctl::R](R) reader structure"]
impl crate::Readable for CRC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_ctl::W](W) writer structure"]
impl crate::Writable for CRC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_CTL to value 0x2000_0000"]
impl crate::Resettable for CRC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
