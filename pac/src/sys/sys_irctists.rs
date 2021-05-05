#[doc = "Register `SYS_IRCTISTS` reader"]
pub struct R(crate::R<SYS_IRCTISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IRCTISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_IRCTISTS_SPEC>> for R {
    fn from(reader: crate::R<SYS_IRCTISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IRCTISTS` writer"]
pub struct W(crate::W<SYS_IRCTISTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IRCTISTS_SPEC>;
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
impl core::convert::From<crate::W<SYS_IRCTISTS_SPEC>> for W {
    fn from(writer: crate::W<SYS_IRCTISTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HIRC Frequency Lock Status\\nThis bit indicates the HIRC frequency is locked.\\nThis is a status bit and doesn't trigger any interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQLOCK_A {
    #[doc = "0: The internal high-speed oscillator frequency doesn't lock at 22.1184 MHz yet"]
    _0 = 0,
    #[doc = "1: The internal high-speed oscillator frequency locked at 22.1184 MHz"]
    _1 = 1,
}
impl From<FREQLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: FREQLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQLOCK` reader - HIRC Frequency Lock Status\\nThis bit indicates the HIRC frequency is locked.\\nThis is a status bit and doesn't trigger any interrupt."]
pub struct FREQLOCK_R(crate::FieldReader<bool, FREQLOCK_A>);
impl FREQLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQLOCK_A {
        match self.bits {
            false => FREQLOCK_A::_0,
            true => FREQLOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FREQLOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FREQLOCK_A::_1
    }
}
impl core::ops::Deref for FREQLOCK_R {
    type Target = crate::FieldReader<bool, FREQLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQLOCK` writer - HIRC Frequency Lock Status\\nThis bit indicates the HIRC frequency is locked.\\nThis is a status bit and doesn't trigger any interrupt."]
pub struct FREQLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The internal high-speed oscillator frequency doesn't lock at 22.1184 MHz yet"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FREQLOCK_A::_0)
    }
    #[doc = "The internal high-speed oscillator frequency locked at 22.1184 MHz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FREQLOCK_A::_1)
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
#[doc = "Trim Failure Interrupt Status\\nThis bit indicates that HIRC trim value update limitation count reached and the HIRC clock frequency still doesn't be locked. Once this bit is set, the auto trim operation stopped and FREQSEL(SYS_iRCTCTL\\[1:0\\]) will be cleared to 00 by hardware automatically.\\nIf this bit is set and TFAILIEN(SYS_IRCTIEN\\[1\\]) is high, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached. Write 1 to clear this to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFAILIF_A {
    #[doc = "0: Trim value update limitation count does not reach"]
    _0 = 0,
    #[doc = "1: Trim value update limitation count reached and HIRC frequency still not locked"]
    _1 = 1,
}
impl From<TFAILIF_A> for bool {
    #[inline(always)]
    fn from(variant: TFAILIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFAILIF` reader - Trim Failure Interrupt Status\\nThis bit indicates that HIRC trim value update limitation count reached and the HIRC clock frequency still doesn't be locked. Once this bit is set, the auto trim operation stopped and FREQSEL(SYS_iRCTCTL\\[1:0\\]) will be cleared to 00 by hardware automatically.\\nIf this bit is set and TFAILIEN(SYS_IRCTIEN\\[1\\]) is high, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached. Write 1 to clear this to 0."]
pub struct TFAILIF_R(crate::FieldReader<bool, TFAILIF_A>);
impl TFAILIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFAILIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFAILIF_A {
        match self.bits {
            false => TFAILIF_A::_0,
            true => TFAILIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFAILIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFAILIF_A::_1
    }
}
impl core::ops::Deref for TFAILIF_R {
    type Target = crate::FieldReader<bool, TFAILIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFAILIF` writer - Trim Failure Interrupt Status\\nThis bit indicates that HIRC trim value update limitation count reached and the HIRC clock frequency still doesn't be locked. Once this bit is set, the auto trim operation stopped and FREQSEL(SYS_iRCTCTL\\[1:0\\]) will be cleared to 00 by hardware automatically.\\nIf this bit is set and TFAILIEN(SYS_IRCTIEN\\[1\\]) is high, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached. Write 1 to clear this to 0."]
pub struct TFAILIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFAILIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFAILIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trim value update limitation count does not reach"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFAILIF_A::_0)
    }
    #[doc = "Trim value update limitation count reached and HIRC frequency still not locked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFAILIF_A::_1)
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
#[doc = "Clock Error Interrupt Status\\nWhen the frequency of 32.768 kHz external low speed crystal oscillator (LXT) or 22.1184 MHz internal high speed RC oscillator (HIRC) is shift larger to unreasonable value, this bit will be set and to be an indicate that clock frequency is inaccuracy\\nOnce this bit is set to 1, the auto trim operation stopped and FREQSEL(SYS_IRCTCL\\[1:0\\]) will be cleared to 00 by hardware automatically if CESTOPEN(SYS_IRCTCTL\\[8\\]) is set to 1.\\nIf this bit is set and CLKEIEN(SYS_IRCTIEN\\[2\\]) is high, an interrupt will be triggered to notify the clock frequency is inaccuracy. Write 1 to clear this to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKERRIF_A {
    #[doc = "0: Clock frequency is accuracy"]
    _0 = 0,
    #[doc = "1: Clock frequency is inaccuracy"]
    _1 = 1,
}
impl From<CLKERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: CLKERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKERRIF` reader - Clock Error Interrupt Status\\nWhen the frequency of 32.768 kHz external low speed crystal oscillator (LXT) or 22.1184 MHz internal high speed RC oscillator (HIRC) is shift larger to unreasonable value, this bit will be set and to be an indicate that clock frequency is inaccuracy\\nOnce this bit is set to 1, the auto trim operation stopped and FREQSEL(SYS_IRCTCL\\[1:0\\]) will be cleared to 00 by hardware automatically if CESTOPEN(SYS_IRCTCTL\\[8\\]) is set to 1.\\nIf this bit is set and CLKEIEN(SYS_IRCTIEN\\[2\\]) is high, an interrupt will be triggered to notify the clock frequency is inaccuracy. Write 1 to clear this to 0."]
pub struct CLKERRIF_R(crate::FieldReader<bool, CLKERRIF_A>);
impl CLKERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKERRIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKERRIF_A {
        match self.bits {
            false => CLKERRIF_A::_0,
            true => CLKERRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKERRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKERRIF_A::_1
    }
}
impl core::ops::Deref for CLKERRIF_R {
    type Target = crate::FieldReader<bool, CLKERRIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKERRIF` writer - Clock Error Interrupt Status\\nWhen the frequency of 32.768 kHz external low speed crystal oscillator (LXT) or 22.1184 MHz internal high speed RC oscillator (HIRC) is shift larger to unreasonable value, this bit will be set and to be an indicate that clock frequency is inaccuracy\\nOnce this bit is set to 1, the auto trim operation stopped and FREQSEL(SYS_IRCTCL\\[1:0\\]) will be cleared to 00 by hardware automatically if CESTOPEN(SYS_IRCTCTL\\[8\\]) is set to 1.\\nIf this bit is set and CLKEIEN(SYS_IRCTIEN\\[2\\]) is high, an interrupt will be triggered to notify the clock frequency is inaccuracy. Write 1 to clear this to 0."]
pub struct CLKERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKERRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKERRIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock frequency is accuracy"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKERRIF_A::_0)
    }
    #[doc = "Clock frequency is inaccuracy"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKERRIF_A::_1)
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
impl R {
    #[doc = "Bit 0 - HIRC Frequency Lock Status\\nThis bit indicates the HIRC frequency is locked.\\nThis is a status bit and doesn't trigger any interrupt."]
    #[inline(always)]
    pub fn freqlock(&self) -> FREQLOCK_R {
        FREQLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trim Failure Interrupt Status\\nThis bit indicates that HIRC trim value update limitation count reached and the HIRC clock frequency still doesn't be locked. Once this bit is set, the auto trim operation stopped and FREQSEL(SYS_iRCTCTL\\[1:0\\]) will be cleared to 00 by hardware automatically.\\nIf this bit is set and TFAILIEN(SYS_IRCTIEN\\[1\\]) is high, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn tfailif(&self) -> TFAILIF_R {
        TFAILIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Error Interrupt Status\\nWhen the frequency of 32.768 kHz external low speed crystal oscillator (LXT) or 22.1184 MHz internal high speed RC oscillator (HIRC) is shift larger to unreasonable value, this bit will be set and to be an indicate that clock frequency is inaccuracy\\nOnce this bit is set to 1, the auto trim operation stopped and FREQSEL(SYS_IRCTCL\\[1:0\\]) will be cleared to 00 by hardware automatically if CESTOPEN(SYS_IRCTCTL\\[8\\]) is set to 1.\\nIf this bit is set and CLKEIEN(SYS_IRCTIEN\\[2\\]) is high, an interrupt will be triggered to notify the clock frequency is inaccuracy. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn clkerrif(&self) -> CLKERRIF_R {
        CLKERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HIRC Frequency Lock Status\\nThis bit indicates the HIRC frequency is locked.\\nThis is a status bit and doesn't trigger any interrupt."]
    #[inline(always)]
    pub fn freqlock(&mut self) -> FREQLOCK_W {
        FREQLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Trim Failure Interrupt Status\\nThis bit indicates that HIRC trim value update limitation count reached and the HIRC clock frequency still doesn't be locked. Once this bit is set, the auto trim operation stopped and FREQSEL(SYS_iRCTCTL\\[1:0\\]) will be cleared to 00 by hardware automatically.\\nIf this bit is set and TFAILIEN(SYS_IRCTIEN\\[1\\]) is high, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn tfailif(&mut self) -> TFAILIF_W {
        TFAILIF_W { w: self }
    }
    #[doc = "Bit 2 - Clock Error Interrupt Status\\nWhen the frequency of 32.768 kHz external low speed crystal oscillator (LXT) or 22.1184 MHz internal high speed RC oscillator (HIRC) is shift larger to unreasonable value, this bit will be set and to be an indicate that clock frequency is inaccuracy\\nOnce this bit is set to 1, the auto trim operation stopped and FREQSEL(SYS_IRCTCL\\[1:0\\]) will be cleared to 00 by hardware automatically if CESTOPEN(SYS_IRCTCTL\\[8\\]) is set to 1.\\nIf this bit is set and CLKEIEN(SYS_IRCTIEN\\[2\\]) is high, an interrupt will be triggered to notify the clock frequency is inaccuracy. Write 1 to clear this to 0."]
    #[inline(always)]
    pub fn clkerrif(&mut self) -> CLKERRIF_W {
        CLKERRIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIRC Trim Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_irctists](index.html) module"]
pub struct SYS_IRCTISTS_SPEC;
impl crate::RegisterSpec for SYS_IRCTISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_irctists::R](R) reader structure"]
impl crate::Readable for SYS_IRCTISTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_irctists::W](W) writer structure"]
impl crate::Writable for SYS_IRCTISTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IRCTISTS to value 0"]
impl crate::Resettable for SYS_IRCTISTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
