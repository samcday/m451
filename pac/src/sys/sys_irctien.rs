#[doc = "Register `SYS_IRCTIEN` reader"]
pub struct R(crate::R<SYS_IRCTIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IRCTIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_IRCTIEN_SPEC>> for R {
    fn from(reader: crate::R<SYS_IRCTIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IRCTIEN` writer"]
pub struct W(crate::W<SYS_IRCTIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IRCTIEN_SPEC>;
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
impl core::convert::From<crate::W<SYS_IRCTIEN_SPEC>> for W {
    fn from(writer: crate::W<SYS_IRCTIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trim Failure Interrupt Enable Bit\\nThis bit controls if an interrupt will be triggered while HIRC trim value update limitation count reached and HIRC frequency still not locked on target frequency set by FREQSEL(SYS_IRCTCTL\\[1:0\\]).\\nIf this bit is high and TFAILIF(SYS_IRCTSTS\\[1\\]) is set during auto trim operation, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFAILIEN_A {
    #[doc = "0: Disable TFAILIF(SYS_IRCTSTS\\[1\\]) status to trigger an interrupt to CPU"]
    _0 = 0,
    #[doc = "1: Enable TFAILIF(SYS_IRCTSTS\\[1\\]) status to trigger an interrupt to CPU"]
    _1 = 1,
}
impl From<TFAILIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TFAILIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFAILIEN` reader - Trim Failure Interrupt Enable Bit\\nThis bit controls if an interrupt will be triggered while HIRC trim value update limitation count reached and HIRC frequency still not locked on target frequency set by FREQSEL(SYS_IRCTCTL\\[1:0\\]).\\nIf this bit is high and TFAILIF(SYS_IRCTSTS\\[1\\]) is set during auto trim operation, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached."]
pub struct TFAILIEN_R(crate::FieldReader<bool, TFAILIEN_A>);
impl TFAILIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFAILIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFAILIEN_A {
        match self.bits {
            false => TFAILIEN_A::_0,
            true => TFAILIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFAILIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFAILIEN_A::_1
    }
}
impl core::ops::Deref for TFAILIEN_R {
    type Target = crate::FieldReader<bool, TFAILIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFAILIEN` writer - Trim Failure Interrupt Enable Bit\\nThis bit controls if an interrupt will be triggered while HIRC trim value update limitation count reached and HIRC frequency still not locked on target frequency set by FREQSEL(SYS_IRCTCTL\\[1:0\\]).\\nIf this bit is high and TFAILIF(SYS_IRCTSTS\\[1\\]) is set during auto trim operation, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached."]
pub struct TFAILIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TFAILIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFAILIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable TFAILIF(SYS_IRCTSTS\\[1\\]) status to trigger an interrupt to CPU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFAILIEN_A::_0)
    }
    #[doc = "Enable TFAILIF(SYS_IRCTSTS\\[1\\]) status to trigger an interrupt to CPU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFAILIEN_A::_1)
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
#[doc = "Clock Error Interrupt Enable Bit\\nThis bit controls if CPU would get an interrupt while clock is inaccuracy during auto trim operation.\\nIf this bit is set to1, and CLKERRIF(SYS_IRCTSTS\\[2\\]) is set during auto trim operation, an interrupt will be triggered to notify the clock frequency is inaccuracy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEIEN_A {
    #[doc = "0: Disable CLKERRIF(SYS_IRCTSTS\\[2\\]) status to trigger an interrupt to CPU"]
    _0 = 0,
    #[doc = "1: Enable CLKERRIF(SYS_IRCTSTS\\[2\\]) status to trigger an interrupt to CPU"]
    _1 = 1,
}
impl From<CLKEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEIEN` reader - Clock Error Interrupt Enable Bit\\nThis bit controls if CPU would get an interrupt while clock is inaccuracy during auto trim operation.\\nIf this bit is set to1, and CLKERRIF(SYS_IRCTSTS\\[2\\]) is set during auto trim operation, an interrupt will be triggered to notify the clock frequency is inaccuracy."]
pub struct CLKEIEN_R(crate::FieldReader<bool, CLKEIEN_A>);
impl CLKEIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKEIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEIEN_A {
        match self.bits {
            false => CLKEIEN_A::_0,
            true => CLKEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKEIEN_A::_1
    }
}
impl core::ops::Deref for CLKEIEN_R {
    type Target = crate::FieldReader<bool, CLKEIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEIEN` writer - Clock Error Interrupt Enable Bit\\nThis bit controls if CPU would get an interrupt while clock is inaccuracy during auto trim operation.\\nIf this bit is set to1, and CLKERRIF(SYS_IRCTSTS\\[2\\]) is set during auto trim operation, an interrupt will be triggered to notify the clock frequency is inaccuracy."]
pub struct CLKEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable CLKERRIF(SYS_IRCTSTS\\[2\\]) status to trigger an interrupt to CPU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKEIEN_A::_0)
    }
    #[doc = "Enable CLKERRIF(SYS_IRCTSTS\\[2\\]) status to trigger an interrupt to CPU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKEIEN_A::_1)
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
    #[doc = "Bit 1 - Trim Failure Interrupt Enable Bit\\nThis bit controls if an interrupt will be triggered while HIRC trim value update limitation count reached and HIRC frequency still not locked on target frequency set by FREQSEL(SYS_IRCTCTL\\[1:0\\]).\\nIf this bit is high and TFAILIF(SYS_IRCTSTS\\[1\\]) is set during auto trim operation, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached."]
    #[inline(always)]
    pub fn tfailien(&self) -> TFAILIEN_R {
        TFAILIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Error Interrupt Enable Bit\\nThis bit controls if CPU would get an interrupt while clock is inaccuracy during auto trim operation.\\nIf this bit is set to1, and CLKERRIF(SYS_IRCTSTS\\[2\\]) is set during auto trim operation, an interrupt will be triggered to notify the clock frequency is inaccuracy."]
    #[inline(always)]
    pub fn clkeien(&self) -> CLKEIEN_R {
        CLKEIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trim Failure Interrupt Enable Bit\\nThis bit controls if an interrupt will be triggered while HIRC trim value update limitation count reached and HIRC frequency still not locked on target frequency set by FREQSEL(SYS_IRCTCTL\\[1:0\\]).\\nIf this bit is high and TFAILIF(SYS_IRCTSTS\\[1\\]) is set during auto trim operation, an interrupt will be triggered to notify that HIRC trim value update limitation count was reached."]
    #[inline(always)]
    pub fn tfailien(&mut self) -> TFAILIEN_W {
        TFAILIEN_W { w: self }
    }
    #[doc = "Bit 2 - Clock Error Interrupt Enable Bit\\nThis bit controls if CPU would get an interrupt while clock is inaccuracy during auto trim operation.\\nIf this bit is set to1, and CLKERRIF(SYS_IRCTSTS\\[2\\]) is set during auto trim operation, an interrupt will be triggered to notify the clock frequency is inaccuracy."]
    #[inline(always)]
    pub fn clkeien(&mut self) -> CLKEIEN_W {
        CLKEIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIRC Trim Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_irctien](index.html) module"]
pub struct SYS_IRCTIEN_SPEC;
impl crate::RegisterSpec for SYS_IRCTIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_irctien::R](R) reader structure"]
impl crate::Readable for SYS_IRCTIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_irctien::W](W) writer structure"]
impl crate::Writable for SYS_IRCTIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IRCTIEN to value 0"]
impl crate::Resettable for SYS_IRCTIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
