#[doc = "Register `SC_ALTCTL` reader"]
pub struct R(crate::R<SC_ALTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_ALTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_ALTCTL_SPEC>> for R {
    fn from(reader: crate::R<SC_ALTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_ALTCTL` writer"]
pub struct W(crate::W<SC_ALTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_ALTCTL_SPEC>;
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
impl core::convert::From<crate::W<SC_ALTCTL_SPEC>> for W {
    fn from(writer: crate::W<SC_ALTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX Software Reset\\nWhen TXRST is set, all the bytes in the transmit buffer and TX internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the TX internal state machine and pointers"]
    _1 = 1,
}
impl From<TXRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRST` reader - TX Software Reset\\nWhen TXRST is set, all the bytes in the transmit buffer and TX internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
pub struct TXRST_R(crate::FieldReader<bool, TXRST_A>);
impl TXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRST_A {
        match self.bits {
            false => TXRST_A::_0,
            true => TXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRST_A::_1
    }
}
impl core::ops::Deref for TXRST_R {
    type Target = crate::FieldReader<bool, TXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRST` writer - TX Software Reset\\nWhen TXRST is set, all the bytes in the transmit buffer and TX internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
pub struct TXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRST_A::_0)
    }
    #[doc = "Reset the TX internal state machine and pointers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRST_A::_1)
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
#[doc = "Rx Software Reset\\nWhen RXRST is set, all the bytes in the receiver buffer and Rx internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Reset the Rx internal state machine and pointers"]
    _1 = 1,
}
impl From<RXRST_A> for bool {
    #[inline(always)]
    fn from(variant: RXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRST` reader - Rx Software Reset\\nWhen RXRST is set, all the bytes in the receiver buffer and Rx internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
pub struct RXRST_R(crate::FieldReader<bool, RXRST_A>);
impl RXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRST_A {
        match self.bits {
            false => RXRST_A::_0,
            true => RXRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXRST_A::_1
    }
}
impl core::ops::Deref for RXRST_R {
    type Target = crate::FieldReader<bool, RXRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRST` writer - Rx Software Reset\\nWhen RXRST is set, all the bytes in the receiver buffer and Rx internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
pub struct RXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRST_A::_0)
    }
    #[doc = "Reset the Rx internal state machine and pointers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRST_A::_1)
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
#[doc = "Deactivation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by deactivation sequence\\nNote1: When the deactivation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST (SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN (SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACTEN_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Deactivation sequence generator Enabled"]
    _1 = 1,
}
impl From<DACTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACTEN` reader - Deactivation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by deactivation sequence\\nNote1: When the deactivation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST (SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN (SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct DACTEN_R(crate::FieldReader<bool, DACTEN_A>);
impl DACTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACTEN_A {
        match self.bits {
            false => DACTEN_A::_0,
            true => DACTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACTEN_A::_1
    }
}
impl core::ops::Deref for DACTEN_R {
    type Target = crate::FieldReader<bool, DACTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACTEN` writer - Deactivation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by deactivation sequence\\nNote1: When the deactivation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST (SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN (SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct DACTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTEN_A::_0)
    }
    #[doc = "Deactivation sequence generator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTEN_A::_1)
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
#[doc = "Activation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by activation sequence\\nNote1: When the activation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTEN_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Activation sequence generator Enabled"]
    _1 = 1,
}
impl From<ACTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTEN` reader - Activation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by activation sequence\\nNote1: When the activation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct ACTEN_R(crate::FieldReader<bool, ACTEN_A>);
impl ACTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTEN_A {
        match self.bits {
            false => ACTEN_A::_0,
            true => ACTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACTEN_A::_1
    }
}
impl core::ops::Deref for ACTEN_R {
    type Target = crate::FieldReader<bool, ACTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEN` writer - Activation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by activation sequence\\nNote1: When the activation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct ACTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACTEN_A::_0)
    }
    #[doc = "Activation sequence generator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACTEN_A::_1)
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
#[doc = "Warm Reset Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by warm reset sequence\\nNote1: When the warm reset sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARSTEN_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Warm reset sequence generator Enabled"]
    _1 = 1,
}
impl From<WARSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WARSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WARSTEN` reader - Warm Reset Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by warm reset sequence\\nNote1: When the warm reset sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct WARSTEN_R(crate::FieldReader<bool, WARSTEN_A>);
impl WARSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WARSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARSTEN_A {
        match self.bits {
            false => WARSTEN_A::_0,
            true => WARSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WARSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WARSTEN_A::_1
    }
}
impl core::ops::Deref for WARSTEN_R {
    type Target = crate::FieldReader<bool, WARSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WARSTEN` writer - Warm Reset Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by warm reset sequence\\nNote1: When the warm reset sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct WARSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WARSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WARSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WARSTEN_A::_0)
    }
    #[doc = "Warm reset sequence generator Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WARSTEN_A::_1)
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
#[doc = "Internal Timer0 Start Enable Bit\\nThis bit enables Timer 0 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST and RXRST at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN0_A {
    #[doc = "0: Stops counting"]
    _0 = 0,
    #[doc = "1: Start counting"]
    _1 = 1,
}
impl From<CNTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN0` reader - Internal Timer0 Start Enable Bit\\nThis bit enables Timer 0 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST and RXRST at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN0_R(crate::FieldReader<bool, CNTEN0_A>);
impl CNTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN0_A {
        match self.bits {
            false => CNTEN0_A::_0,
            true => CNTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN0_A::_1
    }
}
impl core::ops::Deref for CNTEN0_R {
    type Target = crate::FieldReader<bool, CNTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN0` writer - Internal Timer0 Start Enable Bit\\nThis bit enables Timer 0 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST and RXRST at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stops counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN0_A::_0)
    }
    #[doc = "Start counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN0_A::_1)
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
#[doc = "Internal Timer1 Start Enable Bit\\nThis bit enables Timer 1 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN1_A {
    #[doc = "0: Stops counting"]
    _0 = 0,
    #[doc = "1: Start counting"]
    _1 = 1,
}
impl From<CNTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN1` reader - Internal Timer1 Start Enable Bit\\nThis bit enables Timer 1 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN1_R(crate::FieldReader<bool, CNTEN1_A>);
impl CNTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN1_A {
        match self.bits {
            false => CNTEN1_A::_0,
            true => CNTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN1_A::_1
    }
}
impl core::ops::Deref for CNTEN1_R {
    type Target = crate::FieldReader<bool, CNTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN1` writer - Internal Timer1 Start Enable Bit\\nThis bit enables Timer 1 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stops counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN1_A::_0)
    }
    #[doc = "Start counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN1_A::_1)
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
#[doc = "Internal Timer2 Start Enable Bit\\nThis bit enables Timer 2 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN2_A {
    #[doc = "0: Stops counting"]
    _0 = 0,
    #[doc = "1: Start counting"]
    _1 = 1,
}
impl From<CNTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTEN2` reader - Internal Timer2 Start Enable Bit\\nThis bit enables Timer 2 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN2_R(crate::FieldReader<bool, CNTEN2_A>);
impl CNTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN2_A {
        match self.bits {
            false => CNTEN2_A::_0,
            true => CNTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTEN2_A::_1
    }
}
impl core::ops::Deref for CNTEN2_R {
    type Target = crate::FieldReader<bool, CNTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEN2` writer - Internal Timer2 Start Enable Bit\\nThis bit enables Timer 2 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
pub struct CNTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stops counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN2_A::_0)
    }
    #[doc = "Start counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN2_A::_1)
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
#[doc = "Field `INITSEL` reader - Initial Timing Selection\\nThis fields indicates the timing of hardware initial state (activation or warm-reset or deactivation).\\nUnit: SC clock\\nActivation: refer to SC Activation Sequence in Figure 6.144\\nWarm-reset: refer to Warm-Reset Sequence in Figure 6.145\\nDeactivation: refer to Deactivation Sequence in Figure 6.146"]
pub struct INITSEL_R(crate::FieldReader<u8, u8>);
impl INITSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INITSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITSEL` writer - Initial Timing Selection\\nThis fields indicates the timing of hardware initial state (activation or warm-reset or deactivation).\\nUnit: SC clock\\nActivation: refer to SC Activation Sequence in Figure 6.144\\nWarm-reset: refer to Warm-Reset Sequence in Figure 6.145\\nDeactivation: refer to Deactivation Sequence in Figure 6.146"]
pub struct INITSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INITSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Auto Deactivation When Card Removal\\nNote: When the card is removed, hardware will stop any process and then do deactivation sequence (if this bit is set). If this process completes, hardware will generate an interrupt INITIF to CPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACEN_A {
    #[doc = "0: Auto deactivation Disabled when hardware detected the card removal"]
    _0 = 0,
    #[doc = "1: Auto deactivation Enabled when hardware detected the card removal"]
    _1 = 1,
}
impl From<ADACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACEN` reader - Auto Deactivation When Card Removal\\nNote: When the card is removed, hardware will stop any process and then do deactivation sequence (if this bit is set). If this process completes, hardware will generate an interrupt INITIF to CPU."]
pub struct ADACEN_R(crate::FieldReader<bool, ADACEN_A>);
impl ADACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACEN_A {
        match self.bits {
            false => ADACEN_A::_0,
            true => ADACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADACEN_A::_1
    }
}
impl core::ops::Deref for ADACEN_R {
    type Target = crate::FieldReader<bool, ADACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADACEN` writer - Auto Deactivation When Card Removal\\nNote: When the card is removed, hardware will stop any process and then do deactivation sequence (if this bit is set). If this process completes, hardware will generate an interrupt INITIF to CPU."]
pub struct ADACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto deactivation Disabled when hardware detected the card removal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADACEN_A::_0)
    }
    #[doc = "Auto deactivation Enabled when hardware detected the card removal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADACEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Receiver Block Guard Time Function Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBGTEN_A {
    #[doc = "0: Receiver block guard time function Disabled"]
    _0 = 0,
    #[doc = "1: Receiver block guard time function Enabled"]
    _1 = 1,
}
impl From<RXBGTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXBGTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBGTEN` reader - Receiver Block Guard Time Function Enable Bit"]
pub struct RXBGTEN_R(crate::FieldReader<bool, RXBGTEN_A>);
impl RXBGTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBGTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBGTEN_A {
        match self.bits {
            false => RXBGTEN_A::_0,
            true => RXBGTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXBGTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBGTEN_A::_1
    }
}
impl core::ops::Deref for RXBGTEN_R {
    type Target = crate::FieldReader<bool, RXBGTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBGTEN` writer - Receiver Block Guard Time Function Enable Bit"]
pub struct RXBGTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBGTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBGTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver block guard time function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBGTEN_A::_0)
    }
    #[doc = "Receiver block guard time function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBGTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Internal Timer0 Active State (Read Only)\\nThis bit indicates the timer counter status of timer0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSTS0_A {
    #[doc = "0: Timer0 is not active"]
    _0 = 0,
    #[doc = "1: Timer0 is active"]
    _1 = 1,
}
impl From<ACTSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: ACTSTS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSTS0` reader - Internal Timer0 Active State (Read Only)\\nThis bit indicates the timer counter status of timer0."]
pub struct ACTSTS0_R(crate::FieldReader<bool, ACTSTS0_A>);
impl ACTSTS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTSTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSTS0_A {
        match self.bits {
            false => ACTSTS0_A::_0,
            true => ACTSTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACTSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACTSTS0_A::_1
    }
}
impl core::ops::Deref for ACTSTS0_R {
    type Target = crate::FieldReader<bool, ACTSTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Timer1 Active State (Read Only)\\nThis bit indicates the timer counter status of timer1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSTS1_A {
    #[doc = "0: Timer1 is not active"]
    _0 = 0,
    #[doc = "1: Timer1 is active"]
    _1 = 1,
}
impl From<ACTSTS1_A> for bool {
    #[inline(always)]
    fn from(variant: ACTSTS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSTS1` reader - Internal Timer1 Active State (Read Only)\\nThis bit indicates the timer counter status of timer1."]
pub struct ACTSTS1_R(crate::FieldReader<bool, ACTSTS1_A>);
impl ACTSTS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTSTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSTS1_A {
        match self.bits {
            false => ACTSTS1_A::_0,
            true => ACTSTS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACTSTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACTSTS1_A::_1
    }
}
impl core::ops::Deref for ACTSTS1_R {
    type Target = crate::FieldReader<bool, ACTSTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Timer2 Active State (Read Only)\\nThis bit indicates the timer counter status of timer2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTSTS2_A {
    #[doc = "0: Timer2 is not active"]
    _0 = 0,
    #[doc = "1: Timer2 is active"]
    _1 = 1,
}
impl From<ACTSTS2_A> for bool {
    #[inline(always)]
    fn from(variant: ACTSTS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSTS2` reader - Internal Timer2 Active State (Read Only)\\nThis bit indicates the timer counter status of timer2."]
pub struct ACTSTS2_R(crate::FieldReader<bool, ACTSTS2_A>);
impl ACTSTS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTSTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSTS2_A {
        match self.bits {
            false => ACTSTS2_A::_0,
            true => ACTSTS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACTSTS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACTSTS2_A::_1
    }
}
impl core::ops::Deref for ACTSTS2_R {
    type Target = crate::FieldReader<bool, ACTSTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TX Software Reset\\nWhen TXRST is set, all the bytes in the transmit buffer and TX internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
    #[inline(always)]
    pub fn txrst(&self) -> TXRST_R {
        TXRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx Software Reset\\nWhen RXRST is set, all the bytes in the receiver buffer and Rx internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
    #[inline(always)]
    pub fn rxrst(&self) -> RXRST_R {
        RXRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Deactivation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by deactivation sequence\\nNote1: When the deactivation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST (SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN (SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn dacten(&self) -> DACTEN_R {
        DACTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Activation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by activation sequence\\nNote1: When the activation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn acten(&self) -> ACTEN_R {
        ACTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Warm Reset Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by warm reset sequence\\nNote1: When the warm reset sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn warsten(&self) -> WARSTEN_R {
        WARSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Internal Timer0 Start Enable Bit\\nThis bit enables Timer 0 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST and RXRST at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten0(&self) -> CNTEN0_R {
        CNTEN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Internal Timer1 Start Enable Bit\\nThis bit enables Timer 1 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten1(&self) -> CNTEN1_R {
        CNTEN1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal Timer2 Start Enable Bit\\nThis bit enables Timer 2 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten2(&self) -> CNTEN2_R {
        CNTEN2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Initial Timing Selection\\nThis fields indicates the timing of hardware initial state (activation or warm-reset or deactivation).\\nUnit: SC clock\\nActivation: refer to SC Activation Sequence in Figure 6.144\\nWarm-reset: refer to Warm-Reset Sequence in Figure 6.145\\nDeactivation: refer to Deactivation Sequence in Figure 6.146"]
    #[inline(always)]
    pub fn initsel(&self) -> INITSEL_R {
        INITSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Auto Deactivation When Card Removal\\nNote: When the card is removed, hardware will stop any process and then do deactivation sequence (if this bit is set). If this process completes, hardware will generate an interrupt INITIF to CPU."]
    #[inline(always)]
    pub fn adacen(&self) -> ADACEN_R {
        ADACEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receiver Block Guard Time Function Enable Bit"]
    #[inline(always)]
    pub fn rxbgten(&self) -> RXBGTEN_R {
        RXBGTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Internal Timer0 Active State (Read Only)\\nThis bit indicates the timer counter status of timer0."]
    #[inline(always)]
    pub fn actsts0(&self) -> ACTSTS0_R {
        ACTSTS0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Internal Timer1 Active State (Read Only)\\nThis bit indicates the timer counter status of timer1."]
    #[inline(always)]
    pub fn actsts1(&self) -> ACTSTS1_R {
        ACTSTS1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Internal Timer2 Active State (Read Only)\\nThis bit indicates the timer counter status of timer2."]
    #[inline(always)]
    pub fn actsts2(&self) -> ACTSTS2_R {
        ACTSTS2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Software Reset\\nWhen TXRST is set, all the bytes in the transmit buffer and TX internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
    #[inline(always)]
    pub fn txrst(&mut self) -> TXRST_W {
        TXRST_W { w: self }
    }
    #[doc = "Bit 1 - Rx Software Reset\\nWhen RXRST is set, all the bytes in the receiver buffer and Rx internal state machine will be cleared.\\nNote: This bit will be auto cleared after reset is complete."]
    #[inline(always)]
    pub fn rxrst(&mut self) -> RXRST_W {
        RXRST_W { w: self }
    }
    #[doc = "Bit 2 - Deactivation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by deactivation sequence\\nNote1: When the deactivation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST (SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN (SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn dacten(&mut self) -> DACTEN_W {
        DACTEN_W { w: self }
    }
    #[doc = "Bit 3 - Activation Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by activation sequence\\nNote1: When the activation sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn acten(&mut self) -> ACTEN_W {
        ACTEN_W { w: self }
    }
    #[doc = "Bit 4 - Warm Reset Sequence Generator Enable Bit\\nThis bit enables SC controller to initiate the card by warm reset sequence\\nNote1: When the warm reset sequence completed, this bit will be cleared automatically and the INITIF(SC_INTSTS\\[8\\]) will be set to 1.\\nNote2: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST, and RXRST at the same time.\\nNote3: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn warsten(&mut self) -> WARSTEN_W {
        WARSTEN_W { w: self }
    }
    #[doc = "Bit 5 - Internal Timer0 Start Enable Bit\\nThis bit enables Timer 0 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST and RXRST at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten0(&mut self) -> CNTEN0_W {
        CNTEN0_W { w: self }
    }
    #[doc = "Bit 6 - Internal Timer1 Start Enable Bit\\nThis bit enables Timer 1 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]), so don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten1(&mut self) -> CNTEN1_W {
        CNTEN1_W { w: self }
    }
    #[doc = "Bit 7 - Internal Timer2 Start Enable Bit\\nThis bit enables Timer 2 to start counting. Software can fill 0 to stop it and set 1 to reload and count.\\nNote3: This field will be cleared by TXRST(SC_ALTCTL\\[0\\]) and RXRST(SC_ALTCTL\\[1\\]). So don't fill this bit, TXRST(SC_ALTCTL\\[0\\]), and RXRST(SC_ALTCTL\\[1\\]) at the same time.\\nNote4: If SCEN(SC_CTL\\[0\\]) is not enabled, this filed cannot be programmed."]
    #[inline(always)]
    pub fn cnten2(&mut self) -> CNTEN2_W {
        CNTEN2_W { w: self }
    }
    #[doc = "Bits 8:9 - Initial Timing Selection\\nThis fields indicates the timing of hardware initial state (activation or warm-reset or deactivation).\\nUnit: SC clock\\nActivation: refer to SC Activation Sequence in Figure 6.144\\nWarm-reset: refer to Warm-Reset Sequence in Figure 6.145\\nDeactivation: refer to Deactivation Sequence in Figure 6.146"]
    #[inline(always)]
    pub fn initsel(&mut self) -> INITSEL_W {
        INITSEL_W { w: self }
    }
    #[doc = "Bit 11 - Auto Deactivation When Card Removal\\nNote: When the card is removed, hardware will stop any process and then do deactivation sequence (if this bit is set). If this process completes, hardware will generate an interrupt INITIF to CPU."]
    #[inline(always)]
    pub fn adacen(&mut self) -> ADACEN_W {
        ADACEN_W { w: self }
    }
    #[doc = "Bit 12 - Receiver Block Guard Time Function Enable Bit"]
    #[inline(always)]
    pub fn rxbgten(&mut self) -> RXBGTEN_W {
        RXBGTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Alternate Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_altctl](index.html) module"]
pub struct SC_ALTCTL_SPEC;
impl crate::RegisterSpec for SC_ALTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_altctl::R](R) reader structure"]
impl crate::Readable for SC_ALTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_altctl::W](W) writer structure"]
impl crate::Writable for SC_ALTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_ALTCTL to value 0"]
impl crate::Resettable for SC_ALTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
