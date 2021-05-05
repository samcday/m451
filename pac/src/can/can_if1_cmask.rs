#[doc = "Register `CAN_IF1_CMASK` reader"]
pub struct R(crate::R<CAN_IF1_CMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF1_CMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF1_CMASK_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF1_CMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF1_CMASK` writer"]
pub struct W(crate::W<CAN_IF1_CMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF1_CMASK_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF1_CMASK_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF1_CMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Access Data Bytes \\[7:4\\]\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_B_A {
    #[doc = "0: Data Bytes \\[7:4\\]
unchanged"]
    _0 = 0,
    #[doc = "1: Transfer Data Bytes \\[7:4\\]
to Message Object.\\nTransfer Data Bytes \\[7:4\\]
to IFn Message Buffer Register"]
    _1 = 1,
}
impl From<DAT_B_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_B` reader - Access Data Bytes \\[7:4\\]\\nWrite Operation:"]
pub struct DAT_B_R(crate::FieldReader<bool, DAT_B_A>);
impl DAT_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAT_B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_B_A {
        match self.bits {
            false => DAT_B_A::_0,
            true => DAT_B_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DAT_B_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DAT_B_A::_1
    }
}
impl core::ops::Deref for DAT_B_R {
    type Target = crate::FieldReader<bool, DAT_B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_B` writer - Access Data Bytes \\[7:4\\]\\nWrite Operation:"]
pub struct DAT_B_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data Bytes \\[7:4\\]
unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAT_B_A::_0)
    }
    #[doc = "Transfer Data Bytes \\[7:4\\]
to Message Object.\\nTransfer Data Bytes \\[7:4\\]
to IFn Message Buffer Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAT_B_A::_1)
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
#[doc = "Access Data Bytes \\[3:0\\]\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_A_A {
    #[doc = "0: Data Bytes \\[3:0\\]
unchanged"]
    _0 = 0,
    #[doc = "1: Transfer Data Bytes \\[3:0\\]
to Message Object.\\nTransfer Data Bytes \\[3:0\\]
to IFn Message Buffer Register"]
    _1 = 1,
}
impl From<DAT_A_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_A` reader - Access Data Bytes \\[3:0\\]\\nWrite Operation:"]
pub struct DAT_A_R(crate::FieldReader<bool, DAT_A_A>);
impl DAT_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAT_A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_A_A {
        match self.bits {
            false => DAT_A_A::_0,
            true => DAT_A_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DAT_A_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DAT_A_A::_1
    }
}
impl core::ops::Deref for DAT_A_R {
    type Target = crate::FieldReader<bool, DAT_A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_A` writer - Access Data Bytes \\[3:0\\]\\nWrite Operation:"]
pub struct DAT_A_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_A_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data Bytes \\[3:0\\]
unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAT_A_A::_0)
    }
    #[doc = "Transfer Data Bytes \\[3:0\\]
to Message Object.\\nTransfer Data Bytes \\[3:0\\]
to IFn Message Buffer Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAT_A_A::_1)
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
#[doc = "Access Transmission Request Bit When Write Operation\\nNote: A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IFn Message Control Register always reflect the status before resetting these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQST_NEWDAT_A {
    #[doc = "0: TxRqst bit unchanged.\\nNewDat bit remains unchanged"]
    _0 = 0,
    #[doc = "1: Set TxRqst bit.\\nClear NewDat bit in the Message Object"]
    _1 = 1,
}
impl From<TXRQST_NEWDAT_A> for bool {
    #[inline(always)]
    fn from(variant: TXRQST_NEWDAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxRqst_NewDat` reader - Access Transmission Request Bit When Write Operation\\nNote: A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IFn Message Control Register always reflect the status before resetting these bits."]
pub struct TXRQST_NEWDAT_R(crate::FieldReader<bool, TXRQST_NEWDAT_A>);
impl TXRQST_NEWDAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRQST_NEWDAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRQST_NEWDAT_A {
        match self.bits {
            false => TXRQST_NEWDAT_A::_0,
            true => TXRQST_NEWDAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRQST_NEWDAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRQST_NEWDAT_A::_1
    }
}
impl core::ops::Deref for TXRQST_NEWDAT_R {
    type Target = crate::FieldReader<bool, TXRQST_NEWDAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxRqst_NewDat` writer - Access Transmission Request Bit When Write Operation\\nNote: A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IFn Message Control Register always reflect the status before resetting these bits."]
pub struct TXRQST_NEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQST_NEWDAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRQST_NEWDAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TxRqst bit unchanged.\\nNewDat bit remains unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRQST_NEWDAT_A::_0)
    }
    #[doc = "Set TxRqst bit.\\nClear NewDat bit in the Message Object"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRQST_NEWDAT_A::_1)
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
#[doc = "Clear Interrupt Pending Bit\\nWrite Operation:\\nWhen writing to a Message Object, this bit is ignored.\\nRead Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRINTPND_A {
    #[doc = "0: IntPnd bit (CAN_IFn_MCON\\[13\\]) remains unchanged"]
    _0 = 0,
    #[doc = "1: Clear IntPnd bit in the Message Object"]
    _1 = 1,
}
impl From<CLRINTPND_A> for bool {
    #[inline(always)]
    fn from(variant: CLRINTPND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClrIntPnd` reader - Clear Interrupt Pending Bit\\nWrite Operation:\\nWhen writing to a Message Object, this bit is ignored.\\nRead Operation:"]
pub struct CLRINTPND_R(crate::FieldReader<bool, CLRINTPND_A>);
impl CLRINTPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRINTPND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRINTPND_A {
        match self.bits {
            false => CLRINTPND_A::_0,
            true => CLRINTPND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLRINTPND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLRINTPND_A::_1
    }
}
impl core::ops::Deref for CLRINTPND_R {
    type Target = crate::FieldReader<bool, CLRINTPND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ClrIntPnd` writer - Clear Interrupt Pending Bit\\nWrite Operation:\\nWhen writing to a Message Object, this bit is ignored.\\nRead Operation:"]
pub struct CLRINTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINTPND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRINTPND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IntPnd bit (CAN_IFn_MCON\\[13\\]) remains unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLRINTPND_A::_0)
    }
    #[doc = "Clear IntPnd bit in the Message Object"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLRINTPND_A::_1)
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
#[doc = "Control Access Control Bits\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTROL_A {
    #[doc = "0: Control Bits unchanged"]
    _0 = 0,
    #[doc = "1: Transfer Control Bits to Message Object.\\nTransfer Control Bits to IFn Message Buffer Register"]
    _1 = 1,
}
impl From<CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Control` reader - Control Access Control Bits\\nWrite Operation:"]
pub struct CONTROL_R(crate::FieldReader<bool, CONTROL_A>);
impl CONTROL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONTROL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTROL_A {
        match self.bits {
            false => CONTROL_A::_0,
            true => CONTROL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CONTROL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CONTROL_A::_1
    }
}
impl core::ops::Deref for CONTROL_R {
    type Target = crate::FieldReader<bool, CONTROL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Control` writer - Control Access Control Bits\\nWrite Operation:"]
pub struct CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTROL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Control Bits unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTROL_A::_0)
    }
    #[doc = "Transfer Control Bits to Message Object.\\nTransfer Control Bits to IFn Message Buffer Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTROL_A::_1)
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
#[doc = "Access Arbitration Bits\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_A {
    #[doc = "0: Arbitration bits unchanged"]
    _0 = 0,
    #[doc = "1: Transfer Identifier + Dir (CAN_IFn_ARB2\\[13\\]) + Xtd (CAN_IFn_ARB2\\[14\\]) + MsgVal (CAN_IFn_ARB2\\[15\\]) to Message Object.\\nTransfer Identifier + Dir + Xtd + MsgVal to IFn Message Buffer Register"]
    _1 = 1,
}
impl From<ARB_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Arb` reader - Access Arbitration Bits\\nWrite Operation:"]
pub struct ARB_R(crate::FieldReader<bool, ARB_A>);
impl ARB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_A {
        match self.bits {
            false => ARB_A::_0,
            true => ARB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ARB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ARB_A::_1
    }
}
impl core::ops::Deref for ARB_R {
    type Target = crate::FieldReader<bool, ARB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Arb` writer - Access Arbitration Bits\\nWrite Operation:"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Arbitration bits unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARB_A::_0)
    }
    #[doc = "Transfer Identifier + Dir (CAN_IFn_ARB2\\[13\\]) + Xtd (CAN_IFn_ARB2\\[14\\]) + MsgVal (CAN_IFn_ARB2\\[15\\]) to Message Object.\\nTransfer Identifier + Dir + Xtd + MsgVal to IFn Message Buffer Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARB_A::_1)
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
#[doc = "Access Mask Bits\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_A {
    #[doc = "0: Mask bits unchanged"]
    _0 = 0,
    #[doc = "1: Transfer Identifier Mask + MDir + MXtd to Message Object.\\nTransfer Identifier Mask + MDir + MXtd to IFn Message Buffer Register"]
    _1 = 1,
}
impl From<MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Mask` reader - Access Mask Bits\\nWrite Operation:"]
pub struct MASK_R(crate::FieldReader<bool, MASK_A>);
impl MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_A {
        match self.bits {
            false => MASK_A::_0,
            true => MASK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MASK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MASK_A::_1
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<bool, MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Mask` writer - Access Mask Bits\\nWrite Operation:"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask bits unchanged"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MASK_A::_0)
    }
    #[doc = "Transfer Identifier Mask + MDir + MXtd to Message Object.\\nTransfer Identifier Mask + MDir + MXtd to IFn Message Buffer Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MASK_A::_1)
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
#[doc = "Write / Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_RD_A {
    #[doc = "0: Read: Transfer data from the Message Object addressed by the Command Request Register into the selected Message Buffer Registers"]
    _0 = 0,
    #[doc = "1: Write: Transfer data from the selected Message Buffer Registers to the Message Object addressed by the Command Request Register"]
    _1 = 1,
}
impl From<WR_RD_A> for bool {
    #[inline(always)]
    fn from(variant: WR_RD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_RD` reader - Write / Read Mode"]
pub struct WR_RD_R(crate::FieldReader<bool, WR_RD_A>);
impl WR_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_RD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_RD_A {
        match self.bits {
            false => WR_RD_A::_0,
            true => WR_RD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WR_RD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WR_RD_A::_1
    }
}
impl core::ops::Deref for WR_RD_R {
    type Target = crate::FieldReader<bool, WR_RD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_RD` writer - Write / Read Mode"]
pub struct WR_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WR_RD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read: Transfer data from the Message Object addressed by the Command Request Register into the selected Message Buffer Registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WR_RD_A::_0)
    }
    #[doc = "Write: Transfer data from the selected Message Buffer Registers to the Message Object addressed by the Command Request Register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WR_RD_A::_1)
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
    #[doc = "Bit 0 - Access Data Bytes \\[7:4\\]\\nWrite Operation:"]
    #[inline(always)]
    pub fn dat_b(&self) -> DAT_B_R {
        DAT_B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes \\[3:0\\]\\nWrite Operation:"]
    #[inline(always)]
    pub fn dat_a(&self) -> DAT_A_R {
        DAT_A_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Access Transmission Request Bit When Write Operation\\nNote: A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IFn Message Control Register always reflect the status before resetting these bits."]
    #[inline(always)]
    pub fn tx_rqst_new_dat(&self) -> TXRQST_NEWDAT_R {
        TXRQST_NEWDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit\\nWrite Operation:\\nWhen writing to a Message Object, this bit is ignored.\\nRead Operation:"]
    #[inline(always)]
    pub fn clr_int_pnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control Access Control Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write / Read Mode"]
    #[inline(always)]
    pub fn wr_rd(&self) -> WR_RD_R {
        WR_RD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Data Bytes \\[7:4\\]\\nWrite Operation:"]
    #[inline(always)]
    pub fn dat_b(&mut self) -> DAT_B_W {
        DAT_B_W { w: self }
    }
    #[doc = "Bit 1 - Access Data Bytes \\[3:0\\]\\nWrite Operation:"]
    #[inline(always)]
    pub fn dat_a(&mut self) -> DAT_A_W {
        DAT_A_W { w: self }
    }
    #[doc = "Bit 2 - Access Transmission Request Bit When Write Operation\\nNote: A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IFn Message Control Register always reflect the status before resetting these bits."]
    #[inline(always)]
    pub fn tx_rqst_new_dat(&mut self) -> TXRQST_NEWDAT_W {
        TXRQST_NEWDAT_W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit\\nWrite Operation:\\nWhen writing to a Message Object, this bit is ignored.\\nRead Operation:"]
    #[inline(always)]
    pub fn clr_int_pnd(&mut self) -> CLRINTPND_W {
        CLRINTPND_W { w: self }
    }
    #[doc = "Bit 4 - Control Access Control Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn control(&mut self) -> CONTROL_W {
        CONTROL_W { w: self }
    }
    #[doc = "Bit 5 - Access Arbitration Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 6 - Access Mask Bits\\nWrite Operation:"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 7 - Write / Read Mode"]
    #[inline(always)]
    pub fn wr_rd(&mut self) -> WR_RD_W {
        WR_RD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Command Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_cmask](index.html) module"]
pub struct CAN_IF1_CMASK_SPEC;
impl crate::RegisterSpec for CAN_IF1_CMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if1_cmask::R](R) reader structure"]
impl crate::Readable for CAN_IF1_CMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if1_cmask::W](W) writer structure"]
impl crate::Writable for CAN_IF1_CMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF1_CMASK to value 0"]
impl crate::Resettable for CAN_IF1_CMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
