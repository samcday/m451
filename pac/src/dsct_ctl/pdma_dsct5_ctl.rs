#[doc = "Register `PDMA_DSCT5_CTL` reader"]
pub struct R(crate::R<PDMA_DSCT5_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_DSCT5_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_DSCT5_CTL_SPEC>> for R {
    fn from(reader: crate::R<PDMA_DSCT5_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_DSCT5_CTL` writer"]
pub struct W(crate::W<PDMA_DSCT5_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_DSCT5_CTL_SPEC>;
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
impl core::convert::From<crate::W<PDMA_DSCT5_CTL_SPEC>> for W {
    fn from(writer: crate::W<PDMA_DSCT5_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Operation Mode Selection\\nNote: Before filling transfer task in the Descriptor Table, user must check if the descriptor table is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: Idle state: Channel is stopped or this table is complete, when PDMA finish channel table task, OPMODE will be cleared to idle state automatically"]
    _0 = 0,
    #[doc = "1: Basic mode: The descriptor table only has one task. When this task is finished, the PDMA_INTSTS\\[n\\]
will be asserted"]
    _1 = 1,
    #[doc = "2: Scatter-Gather mode: When operating in this mode, user must give the next descriptor table address in PDMA_DSCT_NEXT register; PDMA controller will ignore this task, then load the next task to execute"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMODE` reader - PDMA Operation Mode Selection\\nNote: Before filling transfer task in the Descriptor Table, user must check if the descriptor table is complete."]
pub struct OPMODE_R(crate::FieldReader<u8, OPMODE_A>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPMODE_A {
        match self.bits {
            0 => OPMODE_A::_0,
            1 => OPMODE_A::_1,
            2 => OPMODE_A::_2,
            3 => OPMODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OPMODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == OPMODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == OPMODE_A::_3
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, OPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - PDMA Operation Mode Selection\\nNote: Before filling transfer task in the Descriptor Table, user must check if the descriptor table is complete."]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Idle state: Channel is stopped or this table is complete, when PDMA finish channel table task, OPMODE will be cleared to idle state automatically"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPMODE_A::_0)
    }
    #[doc = "Basic mode: The descriptor table only has one task. When this task is finished, the PDMA_INTSTS\\[n\\]
will be asserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPMODE_A::_1)
    }
    #[doc = "Scatter-Gather mode: When operating in this mode, user must give the next descriptor table address in PDMA_DSCT_NEXT register; PDMA controller will ignore this task, then load the next task to execute"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(OPMODE_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(OPMODE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTYPE_A {
    #[doc = "0: Burst transfer type"]
    _0 = 0,
    #[doc = "1: Single transfer type"]
    _1 = 1,
}
impl From<TXTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTYPE` reader - Transfer Type"]
pub struct TXTYPE_R(crate::FieldReader<bool, TXTYPE_A>);
impl TXTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTYPE_A {
        match self.bits {
            false => TXTYPE_A::_0,
            true => TXTYPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXTYPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXTYPE_A::_1
    }
}
impl core::ops::Deref for TXTYPE_R {
    type Target = crate::FieldReader<bool, TXTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTYPE` writer - Transfer Type"]
pub struct TXTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Burst transfer type"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTYPE_A::_0)
    }
    #[doc = "Single transfer type"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTYPE_A::_1)
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
#[doc = "Burst Size\\nThis field is used for peripheral to determine the burst size or used for determine the re-arbitration size.\\nNote: This field is only useful in burst transfer type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSIZE_A {
    #[doc = "0: 128 Transfers"]
    _0 = 0,
    #[doc = "1: 64 Transfers"]
    _1 = 1,
    #[doc = "2: 32 Transfers"]
    _2 = 2,
    #[doc = "3: 16 Transfers"]
    _3 = 3,
    #[doc = "4: 8 Transfers"]
    _4 = 4,
    #[doc = "5: 4 Transfers"]
    _5 = 5,
    #[doc = "6: 2 Transfers"]
    _6 = 6,
    #[doc = "7: 1 Transfers"]
    _7 = 7,
}
impl From<BURSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BURSIZE` reader - Burst Size\\nThis field is used for peripheral to determine the burst size or used for determine the re-arbitration size.\\nNote: This field is only useful in burst transfer type."]
pub struct BURSIZE_R(crate::FieldReader<u8, BURSIZE_A>);
impl BURSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BURSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSIZE_A {
        match self.bits {
            0 => BURSIZE_A::_0,
            1 => BURSIZE_A::_1,
            2 => BURSIZE_A::_2,
            3 => BURSIZE_A::_3,
            4 => BURSIZE_A::_4,
            5 => BURSIZE_A::_5,
            6 => BURSIZE_A::_6,
            7 => BURSIZE_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BURSIZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BURSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BURSIZE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BURSIZE_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == BURSIZE_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == BURSIZE_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == BURSIZE_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == BURSIZE_A::_7
    }
}
impl core::ops::Deref for BURSIZE_R {
    type Target = crate::FieldReader<u8, BURSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSIZE` writer - Burst Size\\nThis field is used for peripheral to determine the burst size or used for determine the re-arbitration size.\\nNote: This field is only useful in burst transfer type."]
pub struct BURSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "128 Transfers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BURSIZE_A::_0)
    }
    #[doc = "64 Transfers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BURSIZE_A::_1)
    }
    #[doc = "32 Transfers"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BURSIZE_A::_2)
    }
    #[doc = "16 Transfers"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BURSIZE_A::_3)
    }
    #[doc = "8 Transfers"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(BURSIZE_A::_4)
    }
    #[doc = "4 Transfers"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(BURSIZE_A::_5)
    }
    #[doc = "2 Transfers"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(BURSIZE_A::_6)
    }
    #[doc = "1 Transfers"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(BURSIZE_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Table Interrupt Disable Bit\\nThis field can be used to decide whether to enable table interrupt or not. If the TBINTDIS bit is enabled when PDMA controller finishes transfer task, it will not generates interrupt. \\nNote: If this bit set to '1', the TEMPTYF will not be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBINTDIS_A {
    #[doc = "0: Table interrupt Enabled"]
    _0 = 0,
    #[doc = "1: Table interrupt Disabled"]
    _1 = 1,
}
impl From<TBINTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TBINTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBINTDIS` reader - Table Interrupt Disable Bit\\nThis field can be used to decide whether to enable table interrupt or not. If the TBINTDIS bit is enabled when PDMA controller finishes transfer task, it will not generates interrupt. \\nNote: If this bit set to '1', the TEMPTYF will not be set."]
pub struct TBINTDIS_R(crate::FieldReader<bool, TBINTDIS_A>);
impl TBINTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBINTDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBINTDIS_A {
        match self.bits {
            false => TBINTDIS_A::_0,
            true => TBINTDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TBINTDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TBINTDIS_A::_1
    }
}
impl core::ops::Deref for TBINTDIS_R {
    type Target = crate::FieldReader<bool, TBINTDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBINTDIS` writer - Table Interrupt Disable Bit\\nThis field can be used to decide whether to enable table interrupt or not. If the TBINTDIS bit is enabled when PDMA controller finishes transfer task, it will not generates interrupt. \\nNote: If this bit set to '1', the TEMPTYF will not be set."]
pub struct TBINTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBINTDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBINTDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Table interrupt Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBINTDIS_A::_0)
    }
    #[doc = "Table interrupt Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBINTDIS_A::_1)
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
#[doc = "Source Address Increment\\nThis field is used to set the source address increment size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAINC_A {
    #[doc = "3: No increment (fixed address)"]
    _3 = 3,
}
impl From<SAINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAINC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SAINC` reader - Source Address Increment\\nThis field is used to set the source address increment size."]
pub struct SAINC_R(crate::FieldReader<u8, SAINC_A>);
impl SAINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAINC_A> {
        match self.bits {
            3 => Some(SAINC_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SAINC_A::_3
    }
}
impl core::ops::Deref for SAINC_R {
    type Target = crate::FieldReader<u8, SAINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAINC` writer - Source Address Increment\\nThis field is used to set the source address increment size."]
pub struct SAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No increment (fixed address)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SAINC_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Destination Address Increment\\nThis field is used to set the destination address increment size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAINC_A {
    #[doc = "3: No increment (fixed address)"]
    _3 = 3,
}
impl From<DAINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DAINC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAINC` reader - Destination Address Increment\\nThis field is used to set the destination address increment size."]
pub struct DAINC_R(crate::FieldReader<u8, DAINC_A>);
impl DAINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAINC_A> {
        match self.bits {
            3 => Some(DAINC_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == DAINC_A::_3
    }
}
impl core::ops::Deref for DAINC_R {
    type Target = crate::FieldReader<u8, DAINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAINC` writer - Destination Address Increment\\nThis field is used to set the destination address increment size."]
pub struct DAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No increment (fixed address)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(DAINC_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Transfer Width Selection\\nThis field is used for transfer width.\\nNote: The PDMA transfer source address (PDMA_DSCT_SA) and PDMA transfer destination address (PDMA_DSCT_DA) should be alignment under the TXWIDTH selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXWIDTH_A {
    #[doc = "0: One byte (8 bit) is transferred for every operation"]
    _0 = 0,
    #[doc = "1: One half-word (16 bit) is transferred for every operation"]
    _1 = 1,
    #[doc = "2: One word (32-bit) is transferred for every operation"]
    _2 = 2,
    #[doc = "3: Reserved."]
    _3 = 3,
}
impl From<TXWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: TXWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXWIDTH` reader - Transfer Width Selection\\nThis field is used for transfer width.\\nNote: The PDMA transfer source address (PDMA_DSCT_SA) and PDMA transfer destination address (PDMA_DSCT_DA) should be alignment under the TXWIDTH selection"]
pub struct TXWIDTH_R(crate::FieldReader<u8, TXWIDTH_A>);
impl TXWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWIDTH_A {
        match self.bits {
            0 => TXWIDTH_A::_0,
            1 => TXWIDTH_A::_1,
            2 => TXWIDTH_A::_2,
            3 => TXWIDTH_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXWIDTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXWIDTH_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == TXWIDTH_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == TXWIDTH_A::_3
    }
}
impl core::ops::Deref for TXWIDTH_R {
    type Target = crate::FieldReader<u8, TXWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXWIDTH` writer - Transfer Width Selection\\nThis field is used for transfer width.\\nNote: The PDMA transfer source address (PDMA_DSCT_SA) and PDMA transfer destination address (PDMA_DSCT_DA) should be alignment under the TXWIDTH selection"]
pub struct TXWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXWIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "One byte (8 bit) is transferred for every operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXWIDTH_A::_0)
    }
    #[doc = "One half-word (16 bit) is transferred for every operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXWIDTH_A::_1)
    }
    #[doc = "One word (32-bit) is transferred for every operation"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TXWIDTH_A::_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TXWIDTH_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `TXCNT` reader - Transfer Count\\nThe TXCNT represents the required number of PDMA transfer, the real transfer count is (TXCNT + 1); The maximum transfer count is 16384 , every transfer may be byte, half-word or word that is dependent on TXWIDTH field.\\nNote: When PDMA finish each transfer data, this field will be decrease immediately."]
pub struct TXCNT_R(crate::FieldReader<u16, u16>);
impl TXCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCNT` writer - Transfer Count\\nThe TXCNT represents the required number of PDMA transfer, the real transfer count is (TXCNT + 1); The maximum transfer count is 16384 , every transfer may be byte, half-word or word that is dependent on TXWIDTH field.\\nNote: When PDMA finish each transfer data, this field will be decrease immediately."]
pub struct TXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PDMA Operation Mode Selection\\nNote: Before filling transfer task in the Descriptor Table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Transfer Type"]
    #[inline(always)]
    pub fn txtype(&self) -> TXTYPE_R {
        TXTYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Burst Size\\nThis field is used for peripheral to determine the burst size or used for determine the re-arbitration size.\\nNote: This field is only useful in burst transfer type."]
    #[inline(always)]
    pub fn bursize(&self) -> BURSIZE_R {
        BURSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Table Interrupt Disable Bit\\nThis field can be used to decide whether to enable table interrupt or not. If the TBINTDIS bit is enabled when PDMA controller finishes transfer task, it will not generates interrupt. \\nNote: If this bit set to '1', the TEMPTYF will not be set."]
    #[inline(always)]
    pub fn tbintdis(&self) -> TBINTDIS_R {
        TBINTDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Source Address Increment\\nThis field is used to set the source address increment size."]
    #[inline(always)]
    pub fn sainc(&self) -> SAINC_R {
        SAINC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Destination Address Increment\\nThis field is used to set the destination address increment size."]
    #[inline(always)]
    pub fn dainc(&self) -> DAINC_R {
        DAINC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Transfer Width Selection\\nThis field is used for transfer width.\\nNote: The PDMA transfer source address (PDMA_DSCT_SA) and PDMA transfer destination address (PDMA_DSCT_DA) should be alignment under the TXWIDTH selection"]
    #[inline(always)]
    pub fn txwidth(&self) -> TXWIDTH_R {
        TXWIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - Transfer Count\\nThe TXCNT represents the required number of PDMA transfer, the real transfer count is (TXCNT + 1); The maximum transfer count is 16384 , every transfer may be byte, half-word or word that is dependent on TXWIDTH field.\\nNote: When PDMA finish each transfer data, this field will be decrease immediately."]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDMA Operation Mode Selection\\nNote: Before filling transfer task in the Descriptor Table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Type"]
    #[inline(always)]
    pub fn txtype(&mut self) -> TXTYPE_W {
        TXTYPE_W { w: self }
    }
    #[doc = "Bits 4:6 - Burst Size\\nThis field is used for peripheral to determine the burst size or used for determine the re-arbitration size.\\nNote: This field is only useful in burst transfer type."]
    #[inline(always)]
    pub fn bursize(&mut self) -> BURSIZE_W {
        BURSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Table Interrupt Disable Bit\\nThis field can be used to decide whether to enable table interrupt or not. If the TBINTDIS bit is enabled when PDMA controller finishes transfer task, it will not generates interrupt. \\nNote: If this bit set to '1', the TEMPTYF will not be set."]
    #[inline(always)]
    pub fn tbintdis(&mut self) -> TBINTDIS_W {
        TBINTDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - Source Address Increment\\nThis field is used to set the source address increment size."]
    #[inline(always)]
    pub fn sainc(&mut self) -> SAINC_W {
        SAINC_W { w: self }
    }
    #[doc = "Bits 10:11 - Destination Address Increment\\nThis field is used to set the destination address increment size."]
    #[inline(always)]
    pub fn dainc(&mut self) -> DAINC_W {
        DAINC_W { w: self }
    }
    #[doc = "Bits 12:13 - Transfer Width Selection\\nThis field is used for transfer width.\\nNote: The PDMA transfer source address (PDMA_DSCT_SA) and PDMA transfer destination address (PDMA_DSCT_DA) should be alignment under the TXWIDTH selection"]
    #[inline(always)]
    pub fn txwidth(&mut self) -> TXWIDTH_W {
        TXWIDTH_W { w: self }
    }
    #[doc = "Bits 16:29 - Transfer Count\\nThe TXCNT represents the required number of PDMA transfer, the real transfer count is (TXCNT + 1); The maximum transfer count is 16384 , every transfer may be byte, half-word or word that is dependent on TXWIDTH field.\\nNote: When PDMA finish each transfer data, this field will be decrease immediately."]
    #[inline(always)]
    pub fn txcnt(&mut self) -> TXCNT_W {
        TXCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Table Control Register of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_dsct5_ctl](index.html) module"]
pub struct PDMA_DSCT5_CTL_SPEC;
impl crate::RegisterSpec for PDMA_DSCT5_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_dsct5_ctl::R](R) reader structure"]
impl crate::Readable for PDMA_DSCT5_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_dsct5_ctl::W](W) writer structure"]
impl crate::Writable for PDMA_DSCT5_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_DSCT5_CTL to value 0"]
impl crate::Resettable for PDMA_DSCT5_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
