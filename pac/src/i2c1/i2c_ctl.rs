#[doc = "Register `I2C_CTL` reader"]
pub struct R(crate::R<I2C_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_CTL_SPEC>> for R {
    fn from(reader: crate::R<I2C_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTL` writer"]
pub struct W(crate::W<I2C_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTL_SPEC>;
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
impl core::convert::From<crate::W<I2C_CTL_SPEC>> for W {
    fn from(writer: crate::W<I2C_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AA` reader - Assert Acknowledge Control"]
pub struct AA_R(crate::FieldReader<bool, bool>);
impl AA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AA` writer - Assert Acknowledge Control"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
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
#[doc = "Field `SI` reader - I2C Interrupt Flag\\nWhen a new I2C state is present in the I2C_STATUS register, the SI flag is set by hardware. If bit INTEN (I2C_CTL \\[7\\]) is set, the I2C interrupt is requested. SI must be cleared by software. Clear SI by writing 1 to this bit.\\nFor ACKMEN is set in slave read mode, the SI flag is set in 8th clock period for user to confirm the acknowledge bit and 9th clock period for user to read the data in the data buffer."]
pub struct SI_R(crate::FieldReader<bool, bool>);
impl SI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SI` writer - I2C Interrupt Flag\\nWhen a new I2C state is present in the I2C_STATUS register, the SI flag is set by hardware. If bit INTEN (I2C_CTL \\[7\\]) is set, the I2C interrupt is requested. SI must be cleared by software. Clear SI by writing 1 to this bit.\\nFor ACKMEN is set in slave read mode, the SI flag is set in 8th clock period for user to confirm the acknowledge bit and 9th clock period for user to read the data in the data buffer."]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
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
#[doc = "Field `STO` reader - I2C STOP Control\\nIn Master mode, setting STO to transmit a STOP condition to bus then I2C controller will check the bus condition if a STOP condition is detected. This bit will be cleared by hardware automatically."]
pub struct STO_R(crate::FieldReader<bool, bool>);
impl STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STO` writer - I2C STOP Control\\nIn Master mode, setting STO to transmit a STOP condition to bus then I2C controller will check the bus condition if a STOP condition is detected. This bit will be cleared by hardware automatically."]
pub struct STO_W<'a> {
    w: &'a mut W,
}
impl<'a> STO_W<'a> {
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
#[doc = "Field `STA` reader - I2C START Control\\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
pub struct STA_R(crate::FieldReader<bool, bool>);
impl STA_R {
    pub(crate) fn new(bits: bool) -> Self {
        STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STA` writer - I2C START Control\\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
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
#[doc = "I2C Controller Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CEN_A {
    #[doc = "0: I2C Controller Disabled"]
    _0 = 0,
    #[doc = "1: I2C Controller Enabled"]
    _1 = 1,
}
impl From<I2CEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - I2C Controller Enable Bit"]
pub struct I2CEN_R(crate::FieldReader<bool, I2CEN_A>);
impl I2CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2CEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CEN_A {
        match self.bits {
            false => I2CEN_A::_0,
            true => I2CEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2CEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2CEN_A::_1
    }
}
impl core::ops::Deref for I2CEN_R {
    type Target = crate::FieldReader<bool, I2CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CEN` writer - I2C Controller Enable Bit"]
pub struct I2CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C Controller Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2CEN_A::_0)
    }
    #[doc = "I2C Controller Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2CEN_A::_1)
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
#[doc = "Enable Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: I2C interrupt Disabled"]
    _0 = 0,
    #[doc = "1: I2C interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Enable Interrupt"]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::_0,
            true => INTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN_A::_1
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - Enable Interrupt"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
    #[doc = "I2C interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
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
    #[doc = "Bit 2 - Assert Acknowledge Control"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Interrupt Flag\\nWhen a new I2C state is present in the I2C_STATUS register, the SI flag is set by hardware. If bit INTEN (I2C_CTL \\[7\\]) is set, the I2C interrupt is requested. SI must be cleared by software. Clear SI by writing 1 to this bit.\\nFor ACKMEN is set in slave read mode, the SI flag is set in 8th clock period for user to confirm the acknowledge bit and 9th clock period for user to read the data in the data buffer."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C STOP Control\\nIn Master mode, setting STO to transmit a STOP condition to bus then I2C controller will check the bus condition if a STOP condition is detected. This bit will be cleared by hardware automatically."]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C START Control\\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Controller Enable Bit"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Interrupt"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Assert Acknowledge Control"]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W { w: self }
    }
    #[doc = "Bit 3 - I2C Interrupt Flag\\nWhen a new I2C state is present in the I2C_STATUS register, the SI flag is set by hardware. If bit INTEN (I2C_CTL \\[7\\]) is set, the I2C interrupt is requested. SI must be cleared by software. Clear SI by writing 1 to this bit.\\nFor ACKMEN is set in slave read mode, the SI flag is set in 8th clock period for user to confirm the acknowledge bit and 9th clock period for user to read the data in the data buffer."]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bit 4 - I2C STOP Control\\nIn Master mode, setting STO to transmit a STOP condition to bus then I2C controller will check the bus condition if a STOP condition is detected. This bit will be cleared by hardware automatically."]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W { w: self }
    }
    #[doc = "Bit 5 - I2C START Control\\nSetting STA to logic 1 to enter Master mode, the I2C hardware sends a START or repeat START condition to bus when the bus is free."]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 6 - I2C Controller Enable Bit"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable Interrupt"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctl](index.html) module"]
pub struct I2C_CTL_SPEC;
impl crate::RegisterSpec for I2C_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctl::R](R) reader structure"]
impl crate::Readable for I2C_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctl::W](W) writer structure"]
impl crate::Writable for I2C_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTL to value 0"]
impl crate::Resettable for I2C_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
