#[doc = "Register `PDMA_STOP` writer"]
pub struct W(crate::W<PDMA_STOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_STOP_SPEC>;
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
impl core::convert::From<crate::W<PDMA_STOP_SPEC>> for W {
    fn from(writer: crate::W<PDMA_STOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Transfer Stop Control Register (Write Only)\\nUser can stop the PDMA transfer by STOPn bit field or by software reset (writing '0xFFFF_FFFF' to PDMA_STOP register).\\nBy bit field:\\nBy write 0xFFFF_FFFF to PDMA_STOP:\\nSetting all PDMA_STOP bit to '1' will generate software reset to reset internal state machine (the DSCT will not be reset). When software reset, the operation will be stopped imminently that include the on-going transfer and the channel enable bit (PDMA_CHCTL \\[CHEN\\]) and request active flag will be cleared to '0'. \\nNote1: User can read channel enable bit to know if the on-going transfer is finished.\\nNote2: STOP8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum STOPN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Stop PDMA transfer\\[n\\]. When software set PDMA_STOP bit, the operation will finish the on-going transfer channel and then clear the channel enable bit (PDMA_CHCTL \\[CHEN\\]) and request active flag"]
    _1 = 1,
}
impl From<STOPN_AW> for u16 {
    #[inline(always)]
    fn from(variant: STOPN_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `STOPn` writer - PDMA Transfer Stop Control Register (Write Only)\\nUser can stop the PDMA transfer by STOPn bit field or by software reset (writing '0xFFFF_FFFF' to PDMA_STOP register).\\nBy bit field:\\nBy write 0xFFFF_FFFF to PDMA_STOP:\\nSetting all PDMA_STOP bit to '1' will generate software reset to reset internal state machine (the DSCT will not be reset). When software reset, the operation will be stopped imminently that include the on-going transfer and the channel enable bit (PDMA_CHCTL \\[CHEN\\]) and request active flag will be cleared to '0'. \\nNote1: User can read channel enable bit to know if the on-going transfer is finished.\\nNote2: STOP8~11 is M45xG/M45xE only."]
pub struct STOPN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPN_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPN_AW::_0)
    }
    #[doc = "Stop PDMA transfer\\[n\\]. When software set PDMA_STOP bit, the operation will finish the on-going transfer channel and then clear the channel enable bit (PDMA_CHCTL \\[CHEN\\]) and request active flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPN_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Transfer Stop Control Register (Write Only)\\nUser can stop the PDMA transfer by STOPn bit field or by software reset (writing '0xFFFF_FFFF' to PDMA_STOP register).\\nBy bit field:\\nBy write 0xFFFF_FFFF to PDMA_STOP:\\nSetting all PDMA_STOP bit to '1' will generate software reset to reset internal state machine (the DSCT will not be reset). When software reset, the operation will be stopped imminently that include the on-going transfer and the channel enable bit (PDMA_CHCTL \\[CHEN\\]) and request active flag will be cleared to '0'. \\nNote1: User can read channel enable bit to know if the on-going transfer is finished.\\nNote2: STOP8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn stopn(&mut self) -> STOPN_W {
        STOPN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Transfer Stop Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_stop](index.html) module"]
pub struct PDMA_STOP_SPEC;
impl crate::RegisterSpec for PDMA_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_stop::W](W) writer structure"]
impl crate::Writable for PDMA_STOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_STOP to value 0"]
impl crate::Resettable for PDMA_STOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
