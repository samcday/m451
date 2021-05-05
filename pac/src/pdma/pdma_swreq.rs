#[doc = "Register `PDMA_SWREQ` writer"]
pub struct W(crate::W<PDMA_SWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_SWREQ_SPEC>;
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
impl core::convert::From<crate::W<PDMA_SWREQ_SPEC>> for W {
    fn from(writer: crate::W<PDMA_SWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Software Request Register (Write Only)\\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\\nNote1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\\nNote2: If user does not enable corresponding PDMA channel, the software request will be ignored.\\nNote3: SWREQ8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SWREQN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Generate a software request"]
    _1 = 1,
}
impl From<SWREQN_AW> for u16 {
    #[inline(always)]
    fn from(variant: SWREQN_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SWREQn` writer - PDMA Software Request Register (Write Only)\\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\\nNote1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\\nNote2: If user does not enable corresponding PDMA channel, the software request will be ignored.\\nNote3: SWREQ8~11 is M45xG/M45xE only."]
pub struct SWREQN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQN_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWREQN_AW::_0)
    }
    #[doc = "Generate a software request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWREQN_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Software Request Register (Write Only)\\nSet this bit to 1 to generate a software request to PDMA \\[n\\].\\nNote1: User can read PDMA_TRGSTS register to know which channel is on active. Active flag may be triggered by software request or peripheral request.\\nNote2: If user does not enable corresponding PDMA channel, the software request will be ignored.\\nNote3: SWREQ8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn swreqn(&mut self) -> SWREQN_W {
        SWREQN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_swreq](index.html) module"]
pub struct PDMA_SWREQ_SPEC;
impl crate::RegisterSpec for PDMA_SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_swreq::W](W) writer structure"]
impl crate::Writable for PDMA_SWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_SWREQ to value 0"]
impl crate::Resettable for PDMA_SWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
