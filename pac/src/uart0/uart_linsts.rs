#[doc = "Register `UART_LINSTS` reader"]
pub struct R(crate::R<UART_LINSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LINSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_LINSTS_SPEC>> for R {
    fn from(reader: crate::R<UART_LINSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LINSTS` writer"]
pub struct W(crate::W<UART_LINSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LINSTS_SPEC>;
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
impl core::convert::From<crate::W<UART_LINSTS_SPEC>> for W {
    fn from(writer: crate::W<UART_LINSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LIN Slave Header Detection Flag (Read Only))\\nThis bit is set by hardware when a LIN header is detected in LIN slave mode and be cleared by writing 1 to it.\\nNote3: When enable ID parity check IDPEN (UART_LINCTL \\[9\\]), if hardware detect complete header ('break + sync + frame ID'), the SLVHDETF will be set whether the frame ID correct or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVHDETF_A {
    #[doc = "0: LIN header not detected"]
    _0 = 0,
    #[doc = "1: LIN header detected (break + sync + frame ID)"]
    _1 = 1,
}
impl From<SLVHDETF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVHDETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVHDETF` reader - LIN Slave Header Detection Flag (Read Only))\\nThis bit is set by hardware when a LIN header is detected in LIN slave mode and be cleared by writing 1 to it.\\nNote3: When enable ID parity check IDPEN (UART_LINCTL \\[9\\]), if hardware detect complete header ('break + sync + frame ID'), the SLVHDETF will be set whether the frame ID correct or not."]
pub struct SLVHDETF_R(crate::FieldReader<bool, SLVHDETF_A>);
impl SLVHDETF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVHDETF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVHDETF_A {
        match self.bits {
            false => SLVHDETF_A::_0,
            true => SLVHDETF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVHDETF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVHDETF_A::_1
    }
}
impl core::ops::Deref for SLVHDETF_R {
    type Target = crate::FieldReader<bool, SLVHDETF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Slave Header Error Flag (Read Only)\\nThis bit is set by hardware when a LIN header error is detected in LIN slave mode and be cleared by writing 1 to it. The header errors include 'break delimiter is too short (less than 0.5 bit time)', 'frame error in sync field or Identifier field', 'sync field data is not 0x55 in Non-Automatic Resynchronization mode', 'sync field deviation error with Automatic Resynchronization mode', 'sync field measure time-out with Automatic Resynchronization mode' and 'LIN header reception time-out'.\\n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVHEF_A {
    #[doc = "0: LIN header error not detected"]
    _0 = 0,
    #[doc = "1: LIN header error detected"]
    _1 = 1,
}
impl From<SLVHEF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVHEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVHEF` reader - LIN Slave Header Error Flag (Read Only)\\nThis bit is set by hardware when a LIN header error is detected in LIN slave mode and be cleared by writing 1 to it. The header errors include 'break delimiter is too short (less than 0.5 bit time)', 'frame error in sync field or Identifier field', 'sync field data is not 0x55 in Non-Automatic Resynchronization mode', 'sync field deviation error with Automatic Resynchronization mode', 'sync field measure time-out with Automatic Resynchronization mode' and 'LIN header reception time-out'.\\n"]
pub struct SLVHEF_R(crate::FieldReader<bool, SLVHEF_A>);
impl SLVHEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVHEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVHEF_A {
        match self.bits {
            false => SLVHEF_A::_0,
            true => SLVHEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVHEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVHEF_A::_1
    }
}
impl core::ops::Deref for SLVHEF_R {
    type Target = crate::FieldReader<bool, SLVHEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Slave ID Parity Error Flag \\nThis bit is set by hardware when receipted frame ID parity is not correct.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVIDPEF_A {
    #[doc = "0: No active"]
    _0 = 0,
    #[doc = "1: Receipted frame ID parity is not correct"]
    _1 = 1,
}
impl From<SLVIDPEF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVIDPEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVIDPEF` reader - LIN Slave ID Parity Error Flag \\nThis bit is set by hardware when receipted frame ID parity is not correct."]
pub struct SLVIDPEF_R(crate::FieldReader<bool, SLVIDPEF_A>);
impl SLVIDPEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVIDPEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVIDPEF_A {
        match self.bits {
            false => SLVIDPEF_A::_0,
            true => SLVIDPEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVIDPEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVIDPEF_A::_1
    }
}
impl core::ops::Deref for SLVIDPEF_R {
    type Target = crate::FieldReader<bool, SLVIDPEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVIDPEF` writer - LIN Slave ID Parity Error Flag \\nThis bit is set by hardware when receipted frame ID parity is not correct."]
pub struct SLVIDPEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIDPEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVIDPEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVIDPEF_A::_0)
    }
    #[doc = "Receipted frame ID parity is not correct"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVIDPEF_A::_1)
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
#[doc = "LIN Slave Sync Field (Read Only)\\nThis bit indicates that the LIN sync field is being analyzed in Automatic Resynchronization mode. When the receiver header have some error been detect, user must reset the internal circuit to re-search new frame header by writing 1 to this bit.\\nNote2: This bit is read only, but it can be cleared by writing 1 to it.\\nNote3: When writing 1 to it, hardware will reload the initial baud rate and re-search a new frame header.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSYNCF_A {
    #[doc = "0: The current character is not at LIN sync state"]
    _0 = 0,
    #[doc = "1: The current character is at LIN sync state"]
    _1 = 1,
}
impl From<SLVSYNCF_A> for bool {
    #[inline(always)]
    fn from(variant: SLVSYNCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVSYNCF` reader - LIN Slave Sync Field (Read Only)\\nThis bit indicates that the LIN sync field is being analyzed in Automatic Resynchronization mode. When the receiver header have some error been detect, user must reset the internal circuit to re-search new frame header by writing 1 to this bit.\\nNote2: This bit is read only, but it can be cleared by writing 1 to it.\\nNote3: When writing 1 to it, hardware will reload the initial baud rate and re-search a new frame header."]
pub struct SLVSYNCF_R(crate::FieldReader<bool, SLVSYNCF_A>);
impl SLVSYNCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVSYNCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVSYNCF_A {
        match self.bits {
            false => SLVSYNCF_A::_0,
            true => SLVSYNCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVSYNCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVSYNCF_A::_1
    }
}
impl core::ops::Deref for SLVSYNCF_R {
    type Target = crate::FieldReader<bool, SLVSYNCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Break Detection Flag (Read Only)\\nThis bit is set by hardware when a break is detected and be cleared by writing 1 to it through software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKDETF_A {
    #[doc = "0: LIN break not detected"]
    _0 = 0,
    #[doc = "1: LIN break detected"]
    _1 = 1,
}
impl From<BRKDETF_A> for bool {
    #[inline(always)]
    fn from(variant: BRKDETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKDETF` reader - LIN Break Detection Flag (Read Only)\\nThis bit is set by hardware when a break is detected and be cleared by writing 1 to it through software."]
pub struct BRKDETF_R(crate::FieldReader<bool, BRKDETF_A>);
impl BRKDETF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKDETF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKDETF_A {
        match self.bits {
            false => BRKDETF_A::_0,
            true => BRKDETF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRKDETF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRKDETF_A::_1
    }
}
impl core::ops::Deref for BRKDETF_R {
    type Target = crate::FieldReader<bool, BRKDETF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITEF` reader - Bit Error Detect Status Flag (Read Only) \\nAt TX transfer state, hardware will monitoring the bus state, if the input pin (SIN) state not equals to the output pin (SOUT) state, BITEF (UART_LINSTS\\[9\\]) will be set."]
pub struct BITEF_R(crate::FieldReader<bool, bool>);
impl BITEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LIN Slave Header Detection Flag (Read Only))\\nThis bit is set by hardware when a LIN header is detected in LIN slave mode and be cleared by writing 1 to it.\\nNote3: When enable ID parity check IDPEN (UART_LINCTL \\[9\\]), if hardware detect complete header ('break + sync + frame ID'), the SLVHDETF will be set whether the frame ID correct or not."]
    #[inline(always)]
    pub fn slvhdetf(&self) -> SLVHDETF_R {
        SLVHDETF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LIN Slave Header Error Flag (Read Only)\\nThis bit is set by hardware when a LIN header error is detected in LIN slave mode and be cleared by writing 1 to it. The header errors include 'break delimiter is too short (less than 0.5 bit time)', 'frame error in sync field or Identifier field', 'sync field data is not 0x55 in Non-Automatic Resynchronization mode', 'sync field deviation error with Automatic Resynchronization mode', 'sync field measure time-out with Automatic Resynchronization mode' and 'LIN header reception time-out'.\\n"]
    #[inline(always)]
    pub fn slvhef(&self) -> SLVHEF_R {
        SLVHEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LIN Slave ID Parity Error Flag \\nThis bit is set by hardware when receipted frame ID parity is not correct."]
    #[inline(always)]
    pub fn slvidpef(&self) -> SLVIDPEF_R {
        SLVIDPEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LIN Slave Sync Field (Read Only)\\nThis bit indicates that the LIN sync field is being analyzed in Automatic Resynchronization mode. When the receiver header have some error been detect, user must reset the internal circuit to re-search new frame header by writing 1 to this bit.\\nNote2: This bit is read only, but it can be cleared by writing 1 to it.\\nNote3: When writing 1 to it, hardware will reload the initial baud rate and re-search a new frame header."]
    #[inline(always)]
    pub fn slvsyncf(&self) -> SLVSYNCF_R {
        SLVSYNCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN Break Detection Flag (Read Only)\\nThis bit is set by hardware when a break is detected and be cleared by writing 1 to it through software."]
    #[inline(always)]
    pub fn brkdetf(&self) -> BRKDETF_R {
        BRKDETF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bit Error Detect Status Flag (Read Only) \\nAt TX transfer state, hardware will monitoring the bus state, if the input pin (SIN) state not equals to the output pin (SOUT) state, BITEF (UART_LINSTS\\[9\\]) will be set."]
    #[inline(always)]
    pub fn bitef(&self) -> BITEF_R {
        BITEF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - LIN Slave ID Parity Error Flag \\nThis bit is set by hardware when receipted frame ID parity is not correct."]
    #[inline(always)]
    pub fn slvidpef(&mut self) -> SLVIDPEF_W {
        SLVIDPEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART LIN Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_linsts](index.html) module"]
pub struct UART_LINSTS_SPEC;
impl crate::RegisterSpec for UART_LINSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_linsts::R](R) reader structure"]
impl crate::Readable for UART_LINSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_linsts::W](W) writer structure"]
impl crate::Writable for UART_LINSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LINSTS to value 0"]
impl crate::Resettable for UART_LINSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
