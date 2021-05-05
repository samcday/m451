#[doc = "Register `TIMER0_CMP` reader"]
pub struct R(crate::R<TIMER0_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER0_CMP_SPEC>> for R {
    fn from(reader: crate::R<TIMER0_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_CMP` writer"]
pub struct W(crate::W<TIMER0_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CMP_SPEC>;
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
impl core::convert::From<crate::W<TIMER0_CMP_SPEC>> for W {
    fn from(writer: crate::W<TIMER0_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPDAT` reader - Timer Compared Value\\nCMPDAT is a 24-bit compared value register. When the internal 24-bit up counter value is equal to CMPDAT value, the TIF (TIMERx_INTSTS\\[0\\]
Timer Interrupt Flag) will set to 1.\\nNote1: Never write 0x0 or 0x1 in CMPDAT field, or the core will run into unknown state.\\nNote2: When timer is operating at continuous counting mode, the 24-bit up counter will keep counting continuously even if user writes a new value into CMPDAT field. But if timer is operating at other modes, the 24-bit up counter will restart counting from 0 and using newest CMPDAT value to be the timer compared value while user writes a new value into CMPDAT field."]
pub struct CMPDAT_R(crate::FieldReader<u32, u32>);
impl CMPDAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CMPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDAT` writer - Timer Compared Value\\nCMPDAT is a 24-bit compared value register. When the internal 24-bit up counter value is equal to CMPDAT value, the TIF (TIMERx_INTSTS\\[0\\]
Timer Interrupt Flag) will set to 1.\\nNote1: Never write 0x0 or 0x1 in CMPDAT field, or the core will run into unknown state.\\nNote2: When timer is operating at continuous counting mode, the 24-bit up counter will keep counting continuously even if user writes a new value into CMPDAT field. But if timer is operating at other modes, the 24-bit up counter will restart counting from 0 and using newest CMPDAT value to be the timer compared value while user writes a new value into CMPDAT field."]
pub struct CMPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer Compared Value\\nCMPDAT is a 24-bit compared value register. When the internal 24-bit up counter value is equal to CMPDAT value, the TIF (TIMERx_INTSTS\\[0\\]
Timer Interrupt Flag) will set to 1.\\nNote1: Never write 0x0 or 0x1 in CMPDAT field, or the core will run into unknown state.\\nNote2: When timer is operating at continuous counting mode, the 24-bit up counter will keep counting continuously even if user writes a new value into CMPDAT field. But if timer is operating at other modes, the 24-bit up counter will restart counting from 0 and using newest CMPDAT value to be the timer compared value while user writes a new value into CMPDAT field."]
    #[inline(always)]
    pub fn cmpdat(&self) -> CMPDAT_R {
        CMPDAT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Timer Compared Value\\nCMPDAT is a 24-bit compared value register. When the internal 24-bit up counter value is equal to CMPDAT value, the TIF (TIMERx_INTSTS\\[0\\]
Timer Interrupt Flag) will set to 1.\\nNote1: Never write 0x0 or 0x1 in CMPDAT field, or the core will run into unknown state.\\nNote2: When timer is operating at continuous counting mode, the 24-bit up counter will keep counting continuously even if user writes a new value into CMPDAT field. But if timer is operating at other modes, the 24-bit up counter will restart counting from 0 and using newest CMPDAT value to be the timer compared value while user writes a new value into CMPDAT field."]
    #[inline(always)]
    pub fn cmpdat(&mut self) -> CMPDAT_W {
        CMPDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0 Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_cmp](index.html) module"]
pub struct TIMER0_CMP_SPEC;
impl crate::RegisterSpec for TIMER0_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_cmp::R](R) reader structure"]
impl crate::Readable for TIMER0_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_cmp::W](W) writer structure"]
impl crate::Writable for TIMER0_CMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_CMP to value 0"]
impl crate::Resettable for TIMER0_CMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
