#[doc = "Register `CAN_IF2_ARB2` reader"]
pub struct R(crate::R<CAN_IF2_ARB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_ARB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_ARB2_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_ARB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_ARB2` writer"]
pub struct W(crate::W<CAN_IF2_ARB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_ARB2_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_ARB2_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_ARB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Message Identifier 28-16\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Message Identifier 28-16\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Message Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Direction is receive"]
    _0 = 0,
    #[doc = "1: Direction is transmit"]
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Dir` reader - Message Direction"]
pub struct DIR_R(crate::FieldReader<bool, DIR_A>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIR_A::_1
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Dir` writer - Message Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Direction is receive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIR_A::_0)
    }
    #[doc = "Direction is transmit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Extended Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTD_A {
    #[doc = "0: The 11-bit ('standard') Identifier will be used for this Message Object"]
    _0 = 0,
    #[doc = "1: The 29-bit ('extended') Identifier will be used for this Message Object"]
    _1 = 1,
}
impl From<XTD_A> for bool {
    #[inline(always)]
    fn from(variant: XTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Xtd` reader - Extended Identifier"]
pub struct XTD_R(crate::FieldReader<bool, XTD_A>);
impl XTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTD_A {
        match self.bits {
            false => XTD_A::_0,
            true => XTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == XTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == XTD_A::_1
    }
}
impl core::ops::Deref for XTD_R {
    type Target = crate::FieldReader<bool, XTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Xtd` writer - Extended Identifier"]
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The 11-bit ('standard') Identifier will be used for this Message Object"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(XTD_A::_0)
    }
    #[doc = "The 29-bit ('extended') Identifier will be used for this Message Object"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(XTD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Message Valid\\nNote: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init (CAN_CON\\[0\\]). This bit must also be reset before the identifier Id28-0 (CAN_IFn_ARB1/2), the control bits Xtd (CAN_IFn_ARB2\\[14\\]), Dir (CAN_IFn_ARB2\\[13\\]), or the Data Length Code DLC3-0 (CAN_IFn_MCON\\[3:0\\]) are modified, or if the Messages Object is no longer required.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGVAL_A {
    #[doc = "0: The Message Object is ignored by the Message Handler"]
    _0 = 0,
    #[doc = "1: The Message Object is configured and should be considered by the Message Handler"]
    _1 = 1,
}
impl From<MSGVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MSGVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MsgVal` reader - Message Valid\\nNote: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init (CAN_CON\\[0\\]). This bit must also be reset before the identifier Id28-0 (CAN_IFn_ARB1/2), the control bits Xtd (CAN_IFn_ARB2\\[14\\]), Dir (CAN_IFn_ARB2\\[13\\]), or the Data Length Code DLC3-0 (CAN_IFn_MCON\\[3:0\\]) are modified, or if the Messages Object is no longer required."]
pub struct MSGVAL_R(crate::FieldReader<bool, MSGVAL_A>);
impl MSGVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSGVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGVAL_A {
        match self.bits {
            false => MSGVAL_A::_0,
            true => MSGVAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSGVAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSGVAL_A::_1
    }
}
impl core::ops::Deref for MSGVAL_R {
    type Target = crate::FieldReader<bool, MSGVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MsgVal` writer - Message Valid\\nNote: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init (CAN_CON\\[0\\]). This bit must also be reset before the identifier Id28-0 (CAN_IFn_ARB1/2), the control bits Xtd (CAN_IFn_ARB2\\[14\\]), Dir (CAN_IFn_ARB2\\[13\\]), or the Data Length Code DLC3-0 (CAN_IFn_MCON\\[3:0\\]) are modified, or if the Messages Object is no longer required."]
pub struct MSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Message Object is ignored by the Message Handler"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSGVAL_A::_0)
    }
    #[doc = "The Message Object is configured and should be considered by the Message Handler"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSGVAL_A::_1)
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
    #[doc = "Bits 0:12 - Message Identifier 28-16\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Message Valid\\nNote: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init (CAN_CON\\[0\\]). This bit must also be reset before the identifier Id28-0 (CAN_IFn_ARB1/2), the control bits Xtd (CAN_IFn_ARB2\\[14\\]), Dir (CAN_IFn_ARB2\\[13\\]), or the Data Length Code DLC3-0 (CAN_IFn_MCON\\[3:0\\]) are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msg_val(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Message Identifier 28-16\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    #[doc = "Bit 15 - Message Valid\\nNote: The application software must reset the MsgVal bit of all unused Messages Objects during the initialization before it resets bit Init (CAN_CON\\[0\\]). This bit must also be reset before the identifier Id28-0 (CAN_IFn_ARB1/2), the control bits Xtd (CAN_IFn_ARB2\\[14\\]), Dir (CAN_IFn_ARB2\\[13\\]), or the Data Length Code DLC3-0 (CAN_IFn_MCON\\[3:0\\]) are modified, or if the Messages Object is no longer required."]
    #[inline(always)]
    pub fn msg_val(&mut self) -> MSGVAL_W {
        MSGVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Arbitration 2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_arb2](index.html) module"]
pub struct CAN_IF2_ARB2_SPEC;
impl crate::RegisterSpec for CAN_IF2_ARB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_arb2::R](R) reader structure"]
impl crate::Readable for CAN_IF2_ARB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_arb2::W](W) writer structure"]
impl crate::Writable for CAN_IF2_ARB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_ARB2 to value 0"]
impl crate::Resettable for CAN_IF2_ARB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
