#[doc = "Register `SC_INTEN` reader"]
pub struct R(crate::R<SC_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_INTEN_SPEC>> for R {
    fn from(reader: crate::R<SC_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_INTEN` writer"]
pub struct W(crate::W<SC_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_INTEN_SPEC>;
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
impl core::convert::From<crate::W<SC_INTEN_SPEC>> for W {
    fn from(writer: crate::W<SC_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Data Reach Interrupt Enable Bit\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAIEN_A {
    #[doc = "0: Receive data reach trigger level interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receive data reach trigger level interrupt Enabled"]
    _1 = 1,
}
impl From<RDAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDAIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDAIEN` reader - Receive Data Reach Interrupt Enable Bit\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt enable."]
pub struct RDAIEN_R(crate::FieldReader<bool, RDAIEN_A>);
impl RDAIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDAIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDAIEN_A {
        match self.bits {
            false => RDAIEN_A::_0,
            true => RDAIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDAIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDAIEN_A::_1
    }
}
impl core::ops::Deref for RDAIEN_R {
    type Target = crate::FieldReader<bool, RDAIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDAIEN` writer - Receive Data Reach Interrupt Enable Bit\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt enable."]
pub struct RDAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDAIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive data reach trigger level interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDAIEN_A::_0)
    }
    #[doc = "Receive data reach trigger level interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDAIEN_A::_1)
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
#[doc = "Transmit Buffer Empty Interrupt Enable Bit\\nThis field is used for transmit buffer empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEIEN_A {
    #[doc = "0: Transmit buffer empty interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Transmit buffer empty interrupt Enabled"]
    _1 = 1,
}
impl From<TBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEIEN` reader - Transmit Buffer Empty Interrupt Enable Bit\\nThis field is used for transmit buffer empty interrupt enable."]
pub struct TBEIEN_R(crate::FieldReader<bool, TBEIEN_A>);
impl TBEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEIEN_A {
        match self.bits {
            false => TBEIEN_A::_0,
            true => TBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TBEIEN_A::_1
    }
}
impl core::ops::Deref for TBEIEN_R {
    type Target = crate::FieldReader<bool, TBEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEIEN` writer - Transmit Buffer Empty Interrupt Enable Bit\\nThis field is used for transmit buffer empty interrupt enable."]
pub struct TBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit buffer empty interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBEIEN_A::_0)
    }
    #[doc = "Transmit buffer empty interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBEIEN_A::_1)
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
#[doc = "Transfer Error Interrupt Enable Bit\\nThis field is used for transfer error interrupt enable. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\]), parity error PEF(SC_STATUS\\[4\\]), receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]) and transmitter retry over limit error TXOVERR (SC_STATUS\\[30\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERRIEN_A {
    #[doc = "0: Transfer error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Transfer error interrupt Enabled"]
    _1 = 1,
}
impl From<TERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIEN` reader - Transfer Error Interrupt Enable Bit\\nThis field is used for transfer error interrupt enable. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\]), parity error PEF(SC_STATUS\\[4\\]), receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]) and transmitter retry over limit error TXOVERR (SC_STATUS\\[30\\])."]
pub struct TERRIEN_R(crate::FieldReader<bool, TERRIEN_A>);
impl TERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERRIEN_A {
        match self.bits {
            false => TERRIEN_A::_0,
            true => TERRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TERRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TERRIEN_A::_1
    }
}
impl core::ops::Deref for TERRIEN_R {
    type Target = crate::FieldReader<bool, TERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERRIEN` writer - Transfer Error Interrupt Enable Bit\\nThis field is used for transfer error interrupt enable. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\]), parity error PEF(SC_STATUS\\[4\\]), receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]) and transmitter retry over limit error TXOVERR (SC_STATUS\\[30\\])."]
pub struct TERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TERRIEN_A::_0)
    }
    #[doc = "Transfer error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TERRIEN_A::_1)
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
#[doc = "Timer0 Interrupt Enable Bit\\nThis field is used to enable TMR0 interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0IEN_A {
    #[doc = "0: Timer0 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Timer0 interrupt Enabled"]
    _1 = 1,
}
impl From<TMR0IEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0IEN` reader - Timer0 Interrupt Enable Bit\\nThis field is used to enable TMR0 interrupt enable."]
pub struct TMR0IEN_R(crate::FieldReader<bool, TMR0IEN_A>);
impl TMR0IEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR0IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0IEN_A {
        match self.bits {
            false => TMR0IEN_A::_0,
            true => TMR0IEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR0IEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR0IEN_A::_1
    }
}
impl core::ops::Deref for TMR0IEN_R {
    type Target = crate::FieldReader<bool, TMR0IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0IEN` writer - Timer0 Interrupt Enable Bit\\nThis field is used to enable TMR0 interrupt enable."]
pub struct TMR0IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer0 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0IEN_A::_0)
    }
    #[doc = "Timer0 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0IEN_A::_1)
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
#[doc = "Timer1 Interrupt Enable Bit\\nThis field is used to enable the TMR1 interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1IEN_A {
    #[doc = "0: Timer1 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Timer1 interrupt Enabled"]
    _1 = 1,
}
impl From<TMR1IEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1IEN` reader - Timer1 Interrupt Enable Bit\\nThis field is used to enable the TMR1 interrupt."]
pub struct TMR1IEN_R(crate::FieldReader<bool, TMR1IEN_A>);
impl TMR1IEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR1IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1IEN_A {
        match self.bits {
            false => TMR1IEN_A::_0,
            true => TMR1IEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR1IEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR1IEN_A::_1
    }
}
impl core::ops::Deref for TMR1IEN_R {
    type Target = crate::FieldReader<bool, TMR1IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1IEN` writer - Timer1 Interrupt Enable Bit\\nThis field is used to enable the TMR1 interrupt."]
pub struct TMR1IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer1 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1IEN_A::_0)
    }
    #[doc = "Timer1 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1IEN_A::_1)
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
#[doc = "Timer2 Interrupt Enable Bit\\nThis field is used for TMR2 interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2IEN_A {
    #[doc = "0: Timer2 interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Timer2 interrupt Enabled"]
    _1 = 1,
}
impl From<TMR2IEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2IEN` reader - Timer2 Interrupt Enable Bit\\nThis field is used for TMR2 interrupt enable."]
pub struct TMR2IEN_R(crate::FieldReader<bool, TMR2IEN_A>);
impl TMR2IEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR2IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2IEN_A {
        match self.bits {
            false => TMR2IEN_A::_0,
            true => TMR2IEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TMR2IEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TMR2IEN_A::_1
    }
}
impl core::ops::Deref for TMR2IEN_R {
    type Target = crate::FieldReader<bool, TMR2IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2IEN` writer - Timer2 Interrupt Enable Bit\\nThis field is used for TMR2 interrupt enable."]
pub struct TMR2IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer2 interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2IEN_A::_0)
    }
    #[doc = "Timer2 interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2IEN_A::_1)
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
#[doc = "Block Guard Time Interrupt Enable Bit\\nThis field is used for block guard time interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGTIEN_A {
    #[doc = "0: Block guard time Disabled"]
    _0 = 0,
    #[doc = "1: Block guard time Enabled"]
    _1 = 1,
}
impl From<BGTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGTIEN` reader - Block Guard Time Interrupt Enable Bit\\nThis field is used for block guard time interrupt enable."]
pub struct BGTIEN_R(crate::FieldReader<bool, BGTIEN_A>);
impl BGTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGTIEN_A {
        match self.bits {
            false => BGTIEN_A::_0,
            true => BGTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BGTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BGTIEN_A::_1
    }
}
impl core::ops::Deref for BGTIEN_R {
    type Target = crate::FieldReader<bool, BGTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGTIEN` writer - Block Guard Time Interrupt Enable Bit\\nThis field is used for block guard time interrupt enable."]
pub struct BGTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Block guard time Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGTIEN_A::_0)
    }
    #[doc = "Block guard time Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGTIEN_A::_1)
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
#[doc = "Card Detect Interrupt Enable Bit\\nThis field is used for card detect interrupt enable. The card detect status is CINSERT(SC_STATUS\\[12\\])\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIEN_A {
    #[doc = "0: Card detect interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Card detect interrupt Enabled"]
    _1 = 1,
}
impl From<CDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDIEN` reader - Card Detect Interrupt Enable Bit\\nThis field is used for card detect interrupt enable. The card detect status is CINSERT(SC_STATUS\\[12\\])"]
pub struct CDIEN_R(crate::FieldReader<bool, CDIEN_A>);
impl CDIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIEN_A {
        match self.bits {
            false => CDIEN_A::_0,
            true => CDIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CDIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CDIEN_A::_1
    }
}
impl core::ops::Deref for CDIEN_R {
    type Target = crate::FieldReader<bool, CDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDIEN` writer - Card Detect Interrupt Enable Bit\\nThis field is used for card detect interrupt enable. The card detect status is CINSERT(SC_STATUS\\[12\\])"]
pub struct CDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card detect interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDIEN_A::_0)
    }
    #[doc = "Card detect interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDIEN_A::_1)
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
#[doc = "Initial End Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITIEN_A {
    #[doc = "0: Initial end interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Initial end interrupt Enabled"]
    _1 = 1,
}
impl From<INITIEN_A> for bool {
    #[inline(always)]
    fn from(variant: INITIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITIEN` reader - Initial End Interrupt Enable Bit"]
pub struct INITIEN_R(crate::FieldReader<bool, INITIEN_A>);
impl INITIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITIEN_A {
        match self.bits {
            false => INITIEN_A::_0,
            true => INITIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INITIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INITIEN_A::_1
    }
}
impl core::ops::Deref for INITIEN_R {
    type Target = crate::FieldReader<bool, INITIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITIEN` writer - Initial End Interrupt Enable Bit"]
pub struct INITIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INITIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Initial end interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITIEN_A::_0)
    }
    #[doc = "Initial end interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Receiver Buffer Time-out Interrupt Enable Bit \\nThis field is used for receiver buffer time-out interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIF_A {
    #[doc = "0: Receiver buffer time-out interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Receiver buffer time-out interrupt Enabled"]
    _1 = 1,
}
impl From<RXTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIF` reader - Receiver Buffer Time-out Interrupt Enable Bit \\nThis field is used for receiver buffer time-out interrupt enable."]
pub struct RXTOIF_R(crate::FieldReader<bool, RXTOIF_A>);
impl RXTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIF_A {
        match self.bits {
            false => RXTOIF_A::_0,
            true => RXTOIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXTOIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXTOIF_A::_1
    }
}
impl core::ops::Deref for RXTOIF_R {
    type Target = crate::FieldReader<bool, RXTOIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOIF` writer - Receiver Buffer Time-out Interrupt Enable Bit \\nThis field is used for receiver buffer time-out interrupt enable."]
pub struct RXTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTOIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver buffer time-out interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTOIF_A::_0)
    }
    #[doc = "Receiver buffer time-out interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTOIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Auto Convention Error Interrupt Enable Bit \\nThis field is used for auto-convention error interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACERRIEN_A {
    #[doc = "0: Auto-convention error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Auto-convention error interrupt Enabled"]
    _1 = 1,
}
impl From<ACERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACERRIEN` reader - Auto Convention Error Interrupt Enable Bit \\nThis field is used for auto-convention error interrupt enable."]
pub struct ACERRIEN_R(crate::FieldReader<bool, ACERRIEN_A>);
impl ACERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACERRIEN_A {
        match self.bits {
            false => ACERRIEN_A::_0,
            true => ACERRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACERRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACERRIEN_A::_1
    }
}
impl core::ops::Deref for ACERRIEN_R {
    type Target = crate::FieldReader<bool, ACERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACERRIEN` writer - Auto Convention Error Interrupt Enable Bit \\nThis field is used for auto-convention error interrupt enable."]
pub struct ACERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto-convention error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACERRIEN_A::_0)
    }
    #[doc = "Auto-convention error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACERRIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Reach Interrupt Enable Bit\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt enable."]
    #[inline(always)]
    pub fn rdaien(&self) -> RDAIEN_R {
        RDAIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty Interrupt Enable Bit\\nThis field is used for transmit buffer empty interrupt enable."]
    #[inline(always)]
    pub fn tbeien(&self) -> TBEIEN_R {
        TBEIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable Bit\\nThis field is used for transfer error interrupt enable. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\]), parity error PEF(SC_STATUS\\[4\\]), receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]) and transmitter retry over limit error TXOVERR (SC_STATUS\\[30\\])."]
    #[inline(always)]
    pub fn terrien(&self) -> TERRIEN_R {
        TERRIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer0 Interrupt Enable Bit\\nThis field is used to enable TMR0 interrupt enable."]
    #[inline(always)]
    pub fn tmr0ien(&self) -> TMR0IEN_R {
        TMR0IEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer1 Interrupt Enable Bit\\nThis field is used to enable the TMR1 interrupt."]
    #[inline(always)]
    pub fn tmr1ien(&self) -> TMR1IEN_R {
        TMR1IEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer2 Interrupt Enable Bit\\nThis field is used for TMR2 interrupt enable."]
    #[inline(always)]
    pub fn tmr2ien(&self) -> TMR2IEN_R {
        TMR2IEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Block Guard Time Interrupt Enable Bit\\nThis field is used for block guard time interrupt enable."]
    #[inline(always)]
    pub fn bgtien(&self) -> BGTIEN_R {
        BGTIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Interrupt Enable Bit\\nThis field is used for card detect interrupt enable. The card detect status is CINSERT(SC_STATUS\\[12\\])"]
    #[inline(always)]
    pub fn cdien(&self) -> CDIEN_R {
        CDIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Initial End Interrupt Enable Bit"]
    #[inline(always)]
    pub fn initien(&self) -> INITIEN_R {
        INITIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receiver Buffer Time-out Interrupt Enable Bit \\nThis field is used for receiver buffer time-out interrupt enable."]
    #[inline(always)]
    pub fn rxtoif(&self) -> RXTOIF_R {
        RXTOIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Auto Convention Error Interrupt Enable Bit \\nThis field is used for auto-convention error interrupt enable."]
    #[inline(always)]
    pub fn acerrien(&self) -> ACERRIEN_R {
        ACERRIEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Reach Interrupt Enable Bit\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt enable."]
    #[inline(always)]
    pub fn rdaien(&mut self) -> RDAIEN_W {
        RDAIEN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Buffer Empty Interrupt Enable Bit\\nThis field is used for transmit buffer empty interrupt enable."]
    #[inline(always)]
    pub fn tbeien(&mut self) -> TBEIEN_W {
        TBEIEN_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable Bit\\nThis field is used for transfer error interrupt enable. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\]), parity error PEF(SC_STATUS\\[4\\]), receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]) and transmitter retry over limit error TXOVERR (SC_STATUS\\[30\\])."]
    #[inline(always)]
    pub fn terrien(&mut self) -> TERRIEN_W {
        TERRIEN_W { w: self }
    }
    #[doc = "Bit 3 - Timer0 Interrupt Enable Bit\\nThis field is used to enable TMR0 interrupt enable."]
    #[inline(always)]
    pub fn tmr0ien(&mut self) -> TMR0IEN_W {
        TMR0IEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer1 Interrupt Enable Bit\\nThis field is used to enable the TMR1 interrupt."]
    #[inline(always)]
    pub fn tmr1ien(&mut self) -> TMR1IEN_W {
        TMR1IEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer2 Interrupt Enable Bit\\nThis field is used for TMR2 interrupt enable."]
    #[inline(always)]
    pub fn tmr2ien(&mut self) -> TMR2IEN_W {
        TMR2IEN_W { w: self }
    }
    #[doc = "Bit 6 - Block Guard Time Interrupt Enable Bit\\nThis field is used for block guard time interrupt enable."]
    #[inline(always)]
    pub fn bgtien(&mut self) -> BGTIEN_W {
        BGTIEN_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Interrupt Enable Bit\\nThis field is used for card detect interrupt enable. The card detect status is CINSERT(SC_STATUS\\[12\\])"]
    #[inline(always)]
    pub fn cdien(&mut self) -> CDIEN_W {
        CDIEN_W { w: self }
    }
    #[doc = "Bit 8 - Initial End Interrupt Enable Bit"]
    #[inline(always)]
    pub fn initien(&mut self) -> INITIEN_W {
        INITIEN_W { w: self }
    }
    #[doc = "Bit 9 - Receiver Buffer Time-out Interrupt Enable Bit \\nThis field is used for receiver buffer time-out interrupt enable."]
    #[inline(always)]
    pub fn rxtoif(&mut self) -> RXTOIF_W {
        RXTOIF_W { w: self }
    }
    #[doc = "Bit 10 - Auto Convention Error Interrupt Enable Bit \\nThis field is used for auto-convention error interrupt enable."]
    #[inline(always)]
    pub fn acerrien(&mut self) -> ACERRIEN_W {
        ACERRIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Interrupt Enable Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_inten](index.html) module"]
pub struct SC_INTEN_SPEC;
impl crate::RegisterSpec for SC_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_inten::R](R) reader structure"]
impl crate::Readable for SC_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_inten::W](W) writer structure"]
impl crate::Writable for SC_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_INTEN to value 0"]
impl crate::Resettable for SC_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
