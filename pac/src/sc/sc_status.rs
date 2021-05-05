#[doc = "Register `SC_STATUS` reader"]
pub struct R(crate::R<SC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_STATUS` writer"]
pub struct W(crate::W<SC_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_STATUS_SPEC>;
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
impl core::convert::From<crate::W<SC_STATUS_SPEC>> for W {
    fn from(writer: crate::W<SC_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOV` reader - RX Overflow Error Status Flag (Read Only) \\nThis bit is set when RX buffer overflow.\\nIf the number of received bytes is greater than Rx Buffer size (4 bytes), this bit will be set.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct RXOV_R(crate::FieldReader<bool, bool>);
impl RXOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEMPTY` reader - Receiver Buffer Empty Status Flag(Read Only)\\nThis bit indicates RX buffer empty or not.\\nWhen the last byte of Rx buffer has been read by CPU, hardware sets this bit high. It will be cleared when SC receives any new data."]
pub struct RXEMPTY_R(crate::FieldReader<bool, bool>);
impl RXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULL` reader - Receiver Buffer Full Status Flag (Read Only)\\nThis bit indicates RX buffer full or not.\\nThis bit is set when RX pointer is equal to 4, otherwise it is cleared by hardware."]
pub struct RXFULL_R(crate::FieldReader<bool, bool>);
impl RXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEF` reader - Receiver Parity Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
pub struct PEF_R(crate::FieldReader<bool, bool>);
impl PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEF` reader - Receiver Frame Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). \\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
pub struct FEF_R(crate::FieldReader<bool, bool>);
impl FEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEF` reader - Receiver Break Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received data input (RX) held in the 'spacing state' (logic 0) is longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). .\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
pub struct BEF_R(crate::FieldReader<bool, bool>);
impl BEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOV` reader - TX Overflow Error Interrupt Status Flag (Read Only)\\nIf TX buffer is full, an additional write to DAT(SC_DAT\\[7:0\\]) will cause this bit be set to '1' by hardware. \\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct TXOV_R(crate::FieldReader<bool, bool>);
impl TXOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit Buffer Empty Status Flag (Read Only)\\nThis bit indicates TX buffer empty or not.\\nWhen the last byte of TX buffer has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into DAT(SC_DAT\\[7:0\\]) (TX buffer not empty)."]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFULL` reader - Transmit Buffer Full Status Flag (Read Only)\\nThis bit indicates TX buffer full or not.This bit is set when TX pointer is equal to 4, otherwise is cleared by hardware."]
pub struct TXFULL_R(crate::FieldReader<bool, bool>);
impl TXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Detect Removal Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been removal.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: Card detect engine will start after SCEN (SC_CTL\\[0\\])set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREMOVE_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Card removed"]
    _1 = 1,
}
impl From<CREMOVE_A> for bool {
    #[inline(always)]
    fn from(variant: CREMOVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREMOVE` reader - Card Detect Removal Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been removal.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: Card detect engine will start after SCEN (SC_CTL\\[0\\])set."]
pub struct CREMOVE_R(crate::FieldReader<bool, CREMOVE_A>);
impl CREMOVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CREMOVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREMOVE_A {
        match self.bits {
            false => CREMOVE_A::_0,
            true => CREMOVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CREMOVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CREMOVE_A::_1
    }
}
impl core::ops::Deref for CREMOVE_R {
    type Target = crate::FieldReader<bool, CREMOVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Detect Insert Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been inserted.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: The card detect engine will start after SCEN (SC_CTL\\[0\\]) set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSERT_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Card insert"]
    _1 = 1,
}
impl From<CINSERT_A> for bool {
    #[inline(always)]
    fn from(variant: CINSERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINSERT` reader - Card Detect Insert Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been inserted.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: The card detect engine will start after SCEN (SC_CTL\\[0\\]) set."]
pub struct CINSERT_R(crate::FieldReader<bool, CINSERT_A>);
impl CINSERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CINSERT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSERT_A {
        match self.bits {
            false => CINSERT_A::_0,
            true => CINSERT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CINSERT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CINSERT_A::_1
    }
}
impl core::ops::Deref for CINSERT_R {
    type Target = crate::FieldReader<bool, CINSERT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Detect Status of SC_CD Pin Status (Read Only)\\nThis bit is the pin status flag of SC_CD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDPINSTS_A {
    #[doc = "0: The SC_CD pin state at low"]
    _0 = 0,
    #[doc = "1: The SC_CD pin state at high"]
    _1 = 1,
}
impl From<CDPINSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CDPINSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDPINSTS` reader - Card Detect Status of SC_CD Pin Status (Read Only)\\nThis bit is the pin status flag of SC_CD"]
pub struct CDPINSTS_R(crate::FieldReader<bool, CDPINSTS_A>);
impl CDPINSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDPINSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDPINSTS_A {
        match self.bits {
            false => CDPINSTS_A::_0,
            true => CDPINSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CDPINSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CDPINSTS_A::_1
    }
}
impl core::ops::Deref for CDPINSTS_R {
    type Target = crate::FieldReader<bool, CDPINSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPOINT` reader - Receiver Buffer Pointer Status Flag (Read Only)\\nThis field indicates the RX buffer pointer status flag. When SC receives one byte from external device, RXPOINT(SC_STATUS\\[17:16\\]) increases one. When one byte of RX buffer is read by CPU, RXPOINT(SC_STATUS\\[17:16\\]) decreases one."]
pub struct RXPOINT_R(crate::FieldReader<u8, u8>);
impl RXPOINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPOINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRERR` reader - Receiver Retry Error (Read Only)\\nThis bit is set by hardware when RX has any error and retries transfer.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2 This bit is a flag and cannot generate any interrupt to CPU.\\nNote3: If CPU enables receiver retry function by setting RXRTYEN (SC_CTL\\[19\\]) , the PEF(SC_STATUS\\[4\\]) flag will be ignored (hardware will not set PEF(SC_STATUS\\[4\\]))."]
pub struct RXRERR_R(crate::FieldReader<bool, bool>);
impl RXRERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERR` reader - Receiver over Retry Error (Read Only)\\nThis bit is set by hardware when RX transfer error retry over retry number limit.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU enables receiver retries function by setting RXRTYEN (SC_CTL\\[19\\]), the PEF(SC_STATUS\\[4\\]) flag will be ignored (hardware will not set PEF(SC_STATUS\\[4\\]))."]
pub struct RXOVERR_R(crate::FieldReader<bool, bool>);
impl RXOVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXACT` reader - Receiver in Active Status Flag (Read Only)\\nThis bit is set by hardware when RX transfer is in active.\\nThis bit is cleared automatically when RX transfer is finished."]
pub struct RXACT_R(crate::FieldReader<bool, bool>);
impl RXACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPOINT` reader - Transmit Buffer Pointer Status Flag (Read Only)\\nThis field indicates the TX buffer pointer status flag. When CPU writes data into SC_DAT, TXPOINT increases one. When one byte of TX Buffer is transferred to transmitter shift register, TXPOINT decreases one."]
pub struct TXPOINT_R(crate::FieldReader<u8, u8>);
impl TXPOINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPOINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRERR` reader - Transmitter Retry Error (Read Only)\\nThis bit is set by hardware when transmitter re-transmits.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2 This bit is a flag and cannot generate any interrupt to CPU."]
pub struct TXRERR_R(crate::FieldReader<bool, bool>);
impl TXRERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOVERR` reader - Transmitter over Retry Error (Read Only)\\nThis bit is set by hardware when transmitter re-transmits over retry number limitation.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct TXOVERR_R(crate::FieldReader<bool, bool>);
impl TXOVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit in Active Status Flag (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXACT_A {
    #[doc = "0: This bit is cleared automatically when TX transfer is finished or the last byte transmission has completed"]
    _0 = 0,
    #[doc = "1: This bit is set by hardware when TX transfer is in active and the STOP bit of the last byte has been transmitted"]
    _1 = 1,
}
impl From<TXACT_A> for bool {
    #[inline(always)]
    fn from(variant: TXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACT` reader - Transmit in Active Status Flag (Read Only)"]
pub struct TXACT_R(crate::FieldReader<bool, TXACT_A>);
impl TXACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXACT_A {
        match self.bits {
            false => TXACT_A::_0,
            true => TXACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACT_A::_1
    }
}
impl core::ops::Deref for TXACT_R {
    type Target = crate::FieldReader<bool, TXACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RX Overflow Error Status Flag (Read Only) \\nThis bit is set when RX buffer overflow.\\nIf the number of received bytes is greater than Rx Buffer size (4 bytes), this bit will be set.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Buffer Empty Status Flag(Read Only)\\nThis bit indicates RX buffer empty or not.\\nWhen the last byte of Rx buffer has been read by CPU, hardware sets this bit high. It will be cleared when SC receives any new data."]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Buffer Full Status Flag (Read Only)\\nThis bit indicates RX buffer full or not.\\nThis bit is set when RX pointer is equal to 4, otherwise it is cleared by hardware."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receiver Parity Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received character does not have a valid 'parity bit'.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Frame Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received character does not have a valid 'stop bit' (that is, the stop bit following the last data bit or parity bit is detected as logic 0). \\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receiver Break Error Status Flag (Read Only)\\nThis bit is set to logic 1 whenever the received data input (RX) held in the 'spacing state' (logic 0) is longer than a full word transmission time (that is, the total time of 'start bit' + data bits + parity + stop bits). .\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU sets receiver retries function by setting RXRTYEN(SC_CTL\\[19\\]) , hardware will not set this flag."]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TX Overflow Error Interrupt Status Flag (Read Only)\\nIf TX buffer is full, an additional write to DAT(SC_DAT\\[7:0\\]) will cause this bit be set to '1' by hardware. \\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn txov(&self) -> TXOV_R {
        TXOV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Buffer Empty Status Flag (Read Only)\\nThis bit indicates TX buffer empty or not.\\nWhen the last byte of TX buffer has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into DAT(SC_DAT\\[7:0\\]) (TX buffer not empty)."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Buffer Full Status Flag (Read Only)\\nThis bit indicates TX buffer full or not.This bit is set when TX pointer is equal to 4, otherwise is cleared by hardware."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Card Detect Removal Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been removal.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: Card detect engine will start after SCEN (SC_CTL\\[0\\])set."]
    #[inline(always)]
    pub fn cremove(&self) -> CREMOVE_R {
        CREMOVE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Card Detect Insert Status of SC_CD Pin (Read Only)\\nThis bit is set whenever card has been inserted.\\nNote1: This bit is read only, but it can be cleared by writing '1' to it.\\nNote2: The card detect engine will start after SCEN (SC_CTL\\[0\\]) set."]
    #[inline(always)]
    pub fn cinsert(&self) -> CINSERT_R {
        CINSERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Card Detect Status of SC_CD Pin Status (Read Only)\\nThis bit is the pin status flag of SC_CD"]
    #[inline(always)]
    pub fn cdpinsts(&self) -> CDPINSTS_R {
        CDPINSTS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Receiver Buffer Pointer Status Flag (Read Only)\\nThis field indicates the RX buffer pointer status flag. When SC receives one byte from external device, RXPOINT(SC_STATUS\\[17:16\\]) increases one. When one byte of RX buffer is read by CPU, RXPOINT(SC_STATUS\\[17:16\\]) decreases one."]
    #[inline(always)]
    pub fn rxpoint(&self) -> RXPOINT_R {
        RXPOINT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Receiver Retry Error (Read Only)\\nThis bit is set by hardware when RX has any error and retries transfer.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2 This bit is a flag and cannot generate any interrupt to CPU.\\nNote3: If CPU enables receiver retry function by setting RXRTYEN (SC_CTL\\[19\\]) , the PEF(SC_STATUS\\[4\\]) flag will be ignored (hardware will not set PEF(SC_STATUS\\[4\\]))."]
    #[inline(always)]
    pub fn rxrerr(&self) -> RXRERR_R {
        RXRERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receiver over Retry Error (Read Only)\\nThis bit is set by hardware when RX transfer error retry over retry number limit.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2: If CPU enables receiver retries function by setting RXRTYEN (SC_CTL\\[19\\]), the PEF(SC_STATUS\\[4\\]) flag will be ignored (hardware will not set PEF(SC_STATUS\\[4\\]))."]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Receiver in Active Status Flag (Read Only)\\nThis bit is set by hardware when RX transfer is in active.\\nThis bit is cleared automatically when RX transfer is finished."]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Transmit Buffer Pointer Status Flag (Read Only)\\nThis field indicates the TX buffer pointer status flag. When CPU writes data into SC_DAT, TXPOINT increases one. When one byte of TX Buffer is transferred to transmitter shift register, TXPOINT decreases one."]
    #[inline(always)]
    pub fn txpoint(&self) -> TXPOINT_R {
        TXPOINT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Transmitter Retry Error (Read Only)\\nThis bit is set by hardware when transmitter re-transmits.\\nNote1: This bit is read only, but it can be cleared by writing 1 to it.\\nNote2 This bit is a flag and cannot generate any interrupt to CPU."]
    #[inline(always)]
    pub fn txrerr(&self) -> TXRERR_R {
        TXRERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmitter over Retry Error (Read Only)\\nThis bit is set by hardware when transmitter re-transmits over retry number limitation.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn txoverr(&self) -> TXOVERR_R {
        TXOVERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmit in Active Status Flag (Read Only)"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_status](index.html) module"]
pub struct SC_STATUS_SPEC;
impl crate::RegisterSpec for SC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_status::R](R) reader structure"]
impl crate::Readable for SC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_status::W](W) writer structure"]
impl crate::Writable for SC_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_STATUS to value 0x0202"]
impl crate::Resettable for SC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202
    }
}
