#[doc = "Register `EBI_TCTL1` reader"]
pub struct R(crate::R<EBI_TCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_TCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EBI_TCTL1_SPEC>> for R {
    fn from(reader: crate::R<EBI_TCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_TCTL1` writer"]
pub struct W(crate::W<EBI_TCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_TCTL1_SPEC>;
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
impl core::convert::From<crate::W<EBI_TCTL1_SPEC>> for W {
    fn from(writer: crate::W<EBI_TCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACC` reader - EBI Data Access Time\\nTACC define data access time (tACC)."]
pub struct TACC_R(crate::FieldReader<u8, u8>);
impl TACC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACC` writer - EBI Data Access Time\\nTACC define data access time (tACC)."]
pub struct TACC_W<'a> {
    w: &'a mut W,
}
impl<'a> TACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `TAHD` reader - EBI Data Access Hold Time\\nTAHD define data access hold time (tAHD)."]
pub struct TAHD_R(crate::FieldReader<u8, u8>);
impl TAHD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAHD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAHD` writer - EBI Data Access Hold Time\\nTAHD define data access hold time (tAHD)."]
pub struct TAHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TAHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `W2X` reader - Idle Cycle After Write\\nThis field defines the number of W2X idle cycle.\\nWhen write action is finish, W2X idle cycle is inserted and EBI_nCS return to idle state."]
pub struct W2X_R(crate::FieldReader<u8, u8>);
impl W2X_R {
    pub(crate) fn new(bits: u8) -> Self {
        W2X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W2X_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W2X` writer - Idle Cycle After Write\\nThis field defines the number of W2X idle cycle.\\nWhen write action is finish, W2X idle cycle is inserted and EBI_nCS return to idle state."]
pub struct W2X_W<'a> {
    w: &'a mut W,
}
impl<'a> W2X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Access Hold Time Disable Control When Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAHDOFF_A {
    #[doc = "0: The Data Access Hold Time (tAHD) during EBI reading is Enabled"]
    _0 = 0,
    #[doc = "1: The Data Access Hold Time (tAHD) during EBI reading is Disabled"]
    _1 = 1,
}
impl From<RAHDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RAHDOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAHDOFF` reader - Access Hold Time Disable Control When Read"]
pub struct RAHDOFF_R(crate::FieldReader<bool, RAHDOFF_A>);
impl RAHDOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAHDOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAHDOFF_A {
        match self.bits {
            false => RAHDOFF_A::_0,
            true => RAHDOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RAHDOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RAHDOFF_A::_1
    }
}
impl core::ops::Deref for RAHDOFF_R {
    type Target = crate::FieldReader<bool, RAHDOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAHDOFF` writer - Access Hold Time Disable Control When Read"]
pub struct RAHDOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RAHDOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAHDOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Data Access Hold Time (tAHD) during EBI reading is Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAHDOFF_A::_0)
    }
    #[doc = "The Data Access Hold Time (tAHD) during EBI reading is Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAHDOFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Access Hold Time Disable Control When Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAHDOFF_A {
    #[doc = "0: The Data Access Hold Time (tAHD) during EBI writing is Enabled"]
    _0 = 0,
    #[doc = "1: The Data Access Hold Time (tAHD) during EBI writing is Disabled"]
    _1 = 1,
}
impl From<WAHDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: WAHDOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAHDOFF` reader - Access Hold Time Disable Control When Write"]
pub struct WAHDOFF_R(crate::FieldReader<bool, WAHDOFF_A>);
impl WAHDOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAHDOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAHDOFF_A {
        match self.bits {
            false => WAHDOFF_A::_0,
            true => WAHDOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAHDOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAHDOFF_A::_1
    }
}
impl core::ops::Deref for WAHDOFF_R {
    type Target = crate::FieldReader<bool, WAHDOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAHDOFF` writer - Access Hold Time Disable Control When Write"]
pub struct WAHDOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> WAHDOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAHDOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Data Access Hold Time (tAHD) during EBI writing is Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAHDOFF_A::_0)
    }
    #[doc = "The Data Access Hold Time (tAHD) during EBI writing is Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAHDOFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `R2R` reader - Idle Cycle Between Read-to-read\\nThis field defines the number of R2R idle cycle.\\nWhen read action is finish and next action is going to read, R2R idle cycle is inserted and EBI_nCS return to idle state."]
pub struct R2R_R(crate::FieldReader<u8, u8>);
impl R2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        R2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R2R` writer - Idle Cycle Between Read-to-read\\nThis field defines the number of R2R idle cycle.\\nWhen read action is finish and next action is going to read, R2R idle cycle is inserted and EBI_nCS return to idle state."]
pub struct R2R_W<'a> {
    w: &'a mut W,
}
impl<'a> R2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - EBI Data Access Time\\nTACC define data access time (tACC)."]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - EBI Data Access Hold Time\\nTAHD define data access hold time (tAHD)."]
    #[inline(always)]
    pub fn tahd(&self) -> TAHD_R {
        TAHD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Idle Cycle After Write\\nThis field defines the number of W2X idle cycle.\\nWhen write action is finish, W2X idle cycle is inserted and EBI_nCS return to idle state."]
    #[inline(always)]
    pub fn w2x(&self) -> W2X_R {
        W2X_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Access Hold Time Disable Control When Read"]
    #[inline(always)]
    pub fn rahdoff(&self) -> RAHDOFF_R {
        RAHDOFF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Access Hold Time Disable Control When Write"]
    #[inline(always)]
    pub fn wahdoff(&self) -> WAHDOFF_R {
        WAHDOFF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Idle Cycle Between Read-to-read\\nThis field defines the number of R2R idle cycle.\\nWhen read action is finish and next action is going to read, R2R idle cycle is inserted and EBI_nCS return to idle state."]
    #[inline(always)]
    pub fn r2r(&self) -> R2R_R {
        R2R_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - EBI Data Access Time\\nTACC define data access time (tACC)."]
    #[inline(always)]
    pub fn tacc(&mut self) -> TACC_W {
        TACC_W { w: self }
    }
    #[doc = "Bits 8:10 - EBI Data Access Hold Time\\nTAHD define data access hold time (tAHD)."]
    #[inline(always)]
    pub fn tahd(&mut self) -> TAHD_W {
        TAHD_W { w: self }
    }
    #[doc = "Bits 12:15 - Idle Cycle After Write\\nThis field defines the number of W2X idle cycle.\\nWhen write action is finish, W2X idle cycle is inserted and EBI_nCS return to idle state."]
    #[inline(always)]
    pub fn w2x(&mut self) -> W2X_W {
        W2X_W { w: self }
    }
    #[doc = "Bit 22 - Access Hold Time Disable Control When Read"]
    #[inline(always)]
    pub fn rahdoff(&mut self) -> RAHDOFF_W {
        RAHDOFF_W { w: self }
    }
    #[doc = "Bit 23 - Access Hold Time Disable Control When Write"]
    #[inline(always)]
    pub fn wahdoff(&mut self) -> WAHDOFF_W {
        WAHDOFF_W { w: self }
    }
    #[doc = "Bits 24:27 - Idle Cycle Between Read-to-read\\nThis field defines the number of R2R idle cycle.\\nWhen read action is finish and next action is going to read, R2R idle cycle is inserted and EBI_nCS return to idle state."]
    #[inline(always)]
    pub fn r2r(&mut self) -> R2R_W {
        R2R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Bus Interface Bank1 Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_tctl1](index.html) module"]
pub struct EBI_TCTL1_SPEC;
impl crate::RegisterSpec for EBI_TCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_tctl1::R](R) reader structure"]
impl crate::Readable for EBI_TCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_tctl1::W](W) writer structure"]
impl crate::Writable for EBI_TCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EBI_TCTL1 to value 0"]
impl crate::Resettable for EBI_TCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
