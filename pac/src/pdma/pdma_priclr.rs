#[doc = "Register `PDMA_PRICLR` writer"]
pub struct W(crate::W<PDMA_PRICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_PRICLR_SPEC>;
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
impl core::convert::From<crate::W<PDMA_PRICLR_SPEC>> for W {
    fn from(writer: crate::W<PDMA_PRICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Fixed Priority Clear Register (Write Only)\\nSet this bit to 1 to clear fixed priority level.\\nNote1: User can read PDMA_PRISET register to know the channel priority.\\nNote2: FPRICLR8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FPRICLRN_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clear PDMA channel \\[n\\]
fixed priority setting"]
    _1 = 1,
}
impl From<FPRICLRN_AW> for u16 {
    #[inline(always)]
    fn from(variant: FPRICLRN_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `FPRICLRn` writer - PDMA Fixed Priority Clear Register (Write Only)\\nSet this bit to 1 to clear fixed priority level.\\nNote1: User can read PDMA_PRISET register to know the channel priority.\\nNote2: FPRICLR8~11 is M45xG/M45xE only."]
pub struct FPRICLRN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRICLRN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRICLRN_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRICLRN_AW::_0)
    }
    #[doc = "Clear PDMA channel \\[n\\]
fixed priority setting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRICLRN_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Fixed Priority Clear Register (Write Only)\\nSet this bit to 1 to clear fixed priority level.\\nNote1: User can read PDMA_PRISET register to know the channel priority.\\nNote2: FPRICLR8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn fpriclrn(&mut self) -> FPRICLRN_W {
        FPRICLRN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Fixed Priority Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_priclr](index.html) module"]
pub struct PDMA_PRICLR_SPEC;
impl crate::RegisterSpec for PDMA_PRICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdma_priclr::W](W) writer structure"]
impl crate::Writable for PDMA_PRICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_PRICLR to value 0"]
impl crate::Resettable for PDMA_PRICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
