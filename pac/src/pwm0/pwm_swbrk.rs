#[doc = "Register `PWM_SWBRK` writer"]
pub struct W(crate::W<PWM_SWBRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SWBRK_SPEC>;
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
impl core::convert::From<crate::W<PWM_SWBRK_SPEC>> for W {
    fn from(writer: crate::W<PWM_SWBRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRKETRGn` writer - PWM Edge Brake Software Trigger (Write Only) (Write Protect) (M45xD/M45xC Only)\\nEach bit n controls the corresponding PWM pair n.\\nWrite 1 to this bit will trigger edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKETRGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKETRGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `BRKLTRGn` writer - PWM Level Brake Software Trigger (Write Only) (Write Protect)\\nEach bit n controls the corresponding PWM pair n.\\nWrite 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct BRKLTRGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKLTRGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM Edge Brake Software Trigger (Write Only) (Write Protect) (M45xD/M45xC Only)\\nEach bit n controls the corresponding PWM pair n.\\nWrite 1 to this bit will trigger edge brake, and set BRKEIFn to 1 in PWM_INTSTS1 register. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brketrgn(&mut self) -> BRKETRGN_W {
        BRKETRGN_W { w: self }
    }
    #[doc = "Bits 8:10 - PWM Level Brake Software Trigger (Write Only) (Write Protect)\\nEach bit n controls the corresponding PWM pair n.\\nWrite 1 to this bit will trigger level brake, and set BRKLIFn to 1 in PWM_INTSTS1 register. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn brkltrgn(&mut self) -> BRKLTRGN_W {
        BRKLTRGN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Software Brake Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_swbrk](index.html) module"]
pub struct PWM_SWBRK_SPEC;
impl crate::RegisterSpec for PWM_SWBRK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_swbrk::W](W) writer structure"]
impl crate::Writable for PWM_SWBRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SWBRK to value 0"]
impl crate::Resettable for PWM_SWBRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
