#[doc = "Register `CAN_STATUS` reader"]
pub struct R(crate::R<CAN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_STATUS_SPEC>> for R {
    fn from(reader: crate::R<CAN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_STATUS` writer"]
pub struct W(crate::W<CAN_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_STATUS_SPEC>;
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
impl core::convert::From<crate::W<CAN_STATUS_SPEC>> for W {
    fn from(writer: crate::W<CAN_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEC` reader - Last Error Code (Type of the Last Error to Occur on the CAN Bus)\\nThe LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. The Table 640 describes the error code."]
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEC` writer - Last Error Code (Type of the Last Error to Occur on the CAN Bus)\\nThe LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. The Table 640 describes the error code."]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Transmitted a Message Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOK_A {
    #[doc = "0: Since this bit was reset by the CPU, no message has been successfully transmitted. This bit is never reset by the CAN Core"]
    _0 = 0,
    #[doc = "1: Since this bit was last reset by the CPU, a message has been successfully (error free and acknowledged by at least one other node) transmitted"]
    _1 = 1,
}
impl From<TXOK_A> for bool {
    #[inline(always)]
    fn from(variant: TXOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxOK` reader - Transmitted a Message Successfully"]
pub struct TXOK_R(crate::FieldReader<bool, TXOK_A>);
impl TXOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOK_A {
        match self.bits {
            false => TXOK_A::_0,
            true => TXOK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXOK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXOK_A::_1
    }
}
impl core::ops::Deref for TXOK_R {
    type Target = crate::FieldReader<bool, TXOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxOK` writer - Transmitted a Message Successfully"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Since this bit was reset by the CPU, no message has been successfully transmitted. This bit is never reset by the CAN Core"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOK_A::_0)
    }
    #[doc = "Since this bit was last reset by the CPU, a message has been successfully (error free and acknowledged by at least one other node) transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOK_A::_1)
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
#[doc = "Received a Message Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOK_A {
    #[doc = "0: No message has been successfully received since this bit was last reset by the CPU. This bit is never reset by the CAN Core"]
    _0 = 0,
    #[doc = "1: A message has been successfully received since this bit was last reset by the CPU (independent of the result of acceptance filtering)"]
    _1 = 1,
}
impl From<RXOK_A> for bool {
    #[inline(always)]
    fn from(variant: RXOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RxOK` reader - Received a Message Successfully"]
pub struct RXOK_R(crate::FieldReader<bool, RXOK_A>);
impl RXOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOK_A {
        match self.bits {
            false => RXOK_A::_0,
            true => RXOK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXOK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXOK_A::_1
    }
}
impl core::ops::Deref for RXOK_R {
    type Target = crate::FieldReader<bool, RXOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxOK` writer - Received a Message Successfully"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No message has been successfully received since this bit was last reset by the CPU. This bit is never reset by the CAN Core"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOK_A::_0)
    }
    #[doc = "A message has been successfully received since this bit was last reset by the CPU (independent of the result of acceptance filtering)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOK_A::_1)
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
#[doc = "Error Passive (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPASS_A {
    #[doc = "0: The CAN Core is error active"]
    _0 = 0,
    #[doc = "1: The CAN Core is in the error passive state as defined in the CAN Specification"]
    _1 = 1,
}
impl From<EPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPass` reader - Error Passive (Read Only)"]
pub struct EPASS_R(crate::FieldReader<bool, EPASS_A>);
impl EPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPASS_A {
        match self.bits {
            false => EPASS_A::_0,
            true => EPASS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPASS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPASS_A::_1
    }
}
impl core::ops::Deref for EPASS_R {
    type Target = crate::FieldReader<bool, EPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Error Warning Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWARN_A {
    #[doc = "0: Both error counters are below the error warning limit of 96"]
    _0 = 0,
    #[doc = "1: At least one of the error counters in the EML has reached the error warning limit of 96"]
    _1 = 1,
}
impl From<EWARN_A> for bool {
    #[inline(always)]
    fn from(variant: EWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWarn` reader - Error Warning Status (Read Only)"]
pub struct EWARN_R(crate::FieldReader<bool, EWARN_A>);
impl EWARN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWARN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWARN_A {
        match self.bits {
            false => EWARN_A::_0,
            true => EWARN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EWARN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EWARN_A::_1
    }
}
impl core::ops::Deref for EWARN_R {
    type Target = crate::FieldReader<bool, EWARN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bus-off Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF_A {
    #[doc = "0: The CAN module is not in bus-off state"]
    _0 = 0,
    #[doc = "1: The CAN module is in bus-off state"]
    _1 = 1,
}
impl From<BOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOff` reader - Bus-off Status (Read Only)"]
pub struct BOFF_R(crate::FieldReader<bool, BOFF_A>);
impl BOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF_A {
        match self.bits {
            false => BOFF_A::_0,
            true => BOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BOFF_A::_1
    }
}
impl core::ops::Deref for BOFF_R {
    type Target = crate::FieldReader<bool, BOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code (Type of the Last Error to Occur on the CAN Bus)\\nThe LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. The Table 640 describes the error code."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn tx_ok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rx_ok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Passive (Read Only)"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status (Read Only)"]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus-off Status (Read Only)"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code (Type of the Last Error to Occur on the CAN Bus)\\nThe LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. The Table 640 describes the error code."]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn tx_ok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_status](index.html) module"]
pub struct CAN_STATUS_SPEC;
impl crate::RegisterSpec for CAN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_status::R](R) reader structure"]
impl crate::Readable for CAN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_status::W](W) writer structure"]
impl crate::Writable for CAN_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_STATUS to value 0"]
impl crate::Resettable for CAN_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
