#[doc = "Register `PWM_DTCTL0_1` reader"]
pub struct R(crate::R<PWM_DTCTL0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_DTCTL0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_DTCTL0_1_SPEC>> for R {
    fn from(reader: crate::R<PWM_DTCTL0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_DTCTL0_1` writer"]
pub struct W(crate::W<PWM_DTCTL0_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DTCTL0_1_SPEC>;
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
impl core::convert::From<crate::W<PWM_DTCTL0_1_SPEC>> for W {
    fn from(writer: crate::W<PWM_DTCTL0_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCNT` reader - Dead-time Counter (Write Protect)\\nThe dead-time can be calculated from the following formula: \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DTCNT_R(crate::FieldReader<u16, u16>);
impl DTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCNT` writer - Dead-time Counter (Write Protect)\\nThe dead-time can be calculated from the following formula: \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Dead-time Insertion for PWM Pair (PWM_CH0, PWM_CH1) (PWM_CH2, PWM_CH3) (PWM_CH4, PWM_CH5) Enable Bit (Write Protect)\\nDead-time insertion is only active when this pair of complementary PWM is enabled. If dead- time insertion is inactive, the outputs of pin pair are complementary without any delay.\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN_A {
    #[doc = "0: Dead-time insertion on the pin pair Disabled"]
    _0 = 0,
    #[doc = "1: Dead-time insertion on the pin pair Enabled"]
    _1 = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - Dead-time Insertion for PWM Pair (PWM_CH0, PWM_CH1) (PWM_CH2, PWM_CH3) (PWM_CH4, PWM_CH5) Enable Bit (Write Protect)\\nDead-time insertion is only active when this pair of complementary PWM is enabled. If dead- time insertion is inactive, the outputs of pin pair are complementary without any delay.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DTEN_R(crate::FieldReader<bool, DTEN_A>);
impl DTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::_0,
            true => DTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEN_A::_1
    }
}
impl core::ops::Deref for DTEN_R {
    type Target = crate::FieldReader<bool, DTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN` writer - Dead-time Insertion for PWM Pair (PWM_CH0, PWM_CH1) (PWM_CH2, PWM_CH3) (PWM_CH4, PWM_CH5) Enable Bit (Write Protect)\\nDead-time insertion is only active when this pair of complementary PWM is enabled. If dead- time insertion is inactive, the outputs of pin pair are complementary without any delay.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dead-time insertion on the pin pair Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN_A::_0)
    }
    #[doc = "Dead-time insertion on the pin pair Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Dead-time Clock Select (Write Protect) (M45xD/M45xC Only)\\nNote: This register is write protected. Refer to REGWRPROT register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCKSEL_A {
    #[doc = "0: Dead-time clock source from PWM_CLK"]
    _0 = 0,
    #[doc = "1: Dead-time clock source from prescaler output"]
    _1 = 1,
}
impl From<DTCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCKSEL` reader - Dead-time Clock Select (Write Protect) (M45xD/M45xC Only)\\nNote: This register is write protected. Refer to REGWRPROT register."]
pub struct DTCKSEL_R(crate::FieldReader<bool, DTCKSEL_A>);
impl DTCKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCKSEL_A {
        match self.bits {
            false => DTCKSEL_A::_0,
            true => DTCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTCKSEL_A::_1
    }
}
impl core::ops::Deref for DTCKSEL_R {
    type Target = crate::FieldReader<bool, DTCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCKSEL` writer - Dead-time Clock Select (Write Protect) (M45xD/M45xC Only)\\nNote: This register is write protected. Refer to REGWRPROT register."]
pub struct DTCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dead-time clock source from PWM_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCKSEL_A::_0)
    }
    #[doc = "Dead-time clock source from prescaler output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Dead-time Counter (Write Protect)\\nThe dead-time can be calculated from the following formula: \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dtcnt(&self) -> DTCNT_R {
        DTCNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Dead-time Insertion for PWM Pair (PWM_CH0, PWM_CH1) (PWM_CH2, PWM_CH3) (PWM_CH4, PWM_CH5) Enable Bit (Write Protect)\\nDead-time insertion is only active when this pair of complementary PWM is enabled. If dead- time insertion is inactive, the outputs of pin pair are complementary without any delay.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Dead-time Clock Select (Write Protect) (M45xD/M45xC Only)\\nNote: This register is write protected. Refer to REGWRPROT register."]
    #[inline(always)]
    pub fn dtcksel(&self) -> DTCKSEL_R {
        DTCKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-time Counter (Write Protect)\\nThe dead-time can be calculated from the following formula: \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dtcnt(&mut self) -> DTCNT_W {
        DTCNT_W { w: self }
    }
    #[doc = "Bit 16 - Dead-time Insertion for PWM Pair (PWM_CH0, PWM_CH1) (PWM_CH2, PWM_CH3) (PWM_CH4, PWM_CH5) Enable Bit (Write Protect)\\nDead-time insertion is only active when this pair of complementary PWM is enabled. If dead- time insertion is inactive, the outputs of pin pair are complementary without any delay.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 24 - Dead-time Clock Select (Write Protect) (M45xD/M45xC Only)\\nNote: This register is write protected. Refer to REGWRPROT register."]
    #[inline(always)]
    pub fn dtcksel(&mut self) -> DTCKSEL_W {
        DTCKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Dead-time Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dtctl0_1](index.html) module"]
pub struct PWM_DTCTL0_1_SPEC;
impl crate::RegisterSpec for PWM_DTCTL0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_dtctl0_1::R](R) reader structure"]
impl crate::Readable for PWM_DTCTL0_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_dtctl0_1::W](W) writer structure"]
impl crate::Writable for PWM_DTCTL0_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DTCTL0_1 to value 0"]
impl crate::Resettable for PWM_DTCTL0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
