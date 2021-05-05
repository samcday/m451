#[doc = "Register `UART_MODEMSTS` reader"]
pub struct R(crate::R<UART_MODEMSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MODEMSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_MODEMSTS_SPEC>> for R {
    fn from(reader: crate::R<UART_MODEMSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MODEMSTS` writer"]
pub struct W(crate::W<UART_MODEMSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MODEMSTS_SPEC>;
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
impl core::convert::From<crate::W<UART_MODEMSTS_SPEC>> for W {
    fn from(writer: crate::W<UART_MODEMSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Detect nCTS State Change Flag (Read Only)\\nThis bit is set whenever nCTS input has change state, and it will generate Modem interrupt to CPU when MODEMIEN (UART_INTEN \\[3\\]) is set to 1.\\nNote: This bit is read only, but can be cleared by writing '1' to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSDETF_A {
    #[doc = "0: nCTS input has not change state"]
    _0 = 0,
    #[doc = "1: nCTS input has change state"]
    _1 = 1,
}
impl From<CTSDETF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSDETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSDETF` reader - Detect nCTS State Change Flag (Read Only)\\nThis bit is set whenever nCTS input has change state, and it will generate Modem interrupt to CPU when MODEMIEN (UART_INTEN \\[3\\]) is set to 1.\\nNote: This bit is read only, but can be cleared by writing '1' to it."]
pub struct CTSDETF_R(crate::FieldReader<bool, CTSDETF_A>);
impl CTSDETF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSDETF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSDETF_A {
        match self.bits {
            false => CTSDETF_A::_0,
            true => CTSDETF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSDETF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSDETF_A::_1
    }
}
impl core::ops::Deref for CTSDETF_R {
    type Target = crate::FieldReader<bool, CTSDETF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nCTS Pin Status (Read Only)\\nThis bit mirror from nCTS pin input of voltage logic status.\\nNote: This bit echoes when UART Controller peripheral clock is enabled, and nCTS multi-function port is selected.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSSTS_A {
    #[doc = "0: nCTS pin input is low level voltage logic state"]
    _0 = 0,
    #[doc = "1: nCTS pin input is high level voltage logic state"]
    _1 = 1,
}
impl From<CTSSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTSSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSSTS` reader - nCTS Pin Status (Read Only)\\nThis bit mirror from nCTS pin input of voltage logic status.\\nNote: This bit echoes when UART Controller peripheral clock is enabled, and nCTS multi-function port is selected."]
pub struct CTSSTS_R(crate::FieldReader<bool, CTSSTS_A>);
impl CTSSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSSTS_A {
        match self.bits {
            false => CTSSTS_A::_0,
            true => CTSSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSSTS_A::_1
    }
}
impl core::ops::Deref for CTSSTS_R {
    type Target = crate::FieldReader<bool, CTSSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "nCTS Pin Active Level\\nThis bit defines the active level state of nCTS pin input.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSACTLV_A {
    #[doc = "0: nCTS pin input is high level active"]
    _0 = 0,
    #[doc = "1: nCTS pin input is low level active. (Default)"]
    _1 = 1,
}
impl From<CTSACTLV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSACTLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSACTLV` reader - nCTS Pin Active Level\\nThis bit defines the active level state of nCTS pin input."]
pub struct CTSACTLV_R(crate::FieldReader<bool, CTSACTLV_A>);
impl CTSACTLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSACTLV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSACTLV_A {
        match self.bits {
            false => CTSACTLV_A::_0,
            true => CTSACTLV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTSACTLV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSACTLV_A::_1
    }
}
impl core::ops::Deref for CTSACTLV_R {
    type Target = crate::FieldReader<bool, CTSACTLV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSACTLV` writer - nCTS Pin Active Level\\nThis bit defines the active level state of nCTS pin input."]
pub struct CTSACTLV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSACTLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSACTLV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "nCTS pin input is high level active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSACTLV_A::_0)
    }
    #[doc = "nCTS pin input is low level active. (Default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSACTLV_A::_1)
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
impl R {
    #[doc = "Bit 0 - Detect nCTS State Change Flag (Read Only)\\nThis bit is set whenever nCTS input has change state, and it will generate Modem interrupt to CPU when MODEMIEN (UART_INTEN \\[3\\]) is set to 1.\\nNote: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn ctsdetf(&self) -> CTSDETF_R {
        CTSDETF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - nCTS Pin Status (Read Only)\\nThis bit mirror from nCTS pin input of voltage logic status.\\nNote: This bit echoes when UART Controller peripheral clock is enabled, and nCTS multi-function port is selected."]
    #[inline(always)]
    pub fn ctssts(&self) -> CTSSTS_R {
        CTSSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - nCTS Pin Active Level\\nThis bit defines the active level state of nCTS pin input."]
    #[inline(always)]
    pub fn ctsactlv(&self) -> CTSACTLV_R {
        CTSACTLV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - nCTS Pin Active Level\\nThis bit defines the active level state of nCTS pin input."]
    #[inline(always)]
    pub fn ctsactlv(&mut self) -> CTSACTLV_W {
        CTSACTLV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_modemsts](index.html) module"]
pub struct UART_MODEMSTS_SPEC;
impl crate::RegisterSpec for UART_MODEMSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_modemsts::R](R) reader structure"]
impl crate::Readable for UART_MODEMSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_modemsts::W](W) writer structure"]
impl crate::Writable for UART_MODEMSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MODEMSTS to value 0x0110"]
impl crate::Resettable for UART_MODEMSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110
    }
}
