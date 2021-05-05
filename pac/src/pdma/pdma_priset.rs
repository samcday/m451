#[doc = "Register `PDMA_PRISET` reader"]
pub struct R(crate::R<PDMA_PRISET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_PRISET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_PRISET_SPEC>> for R {
    fn from(reader: crate::R<PDMA_PRISET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_PRISET` writer"]
pub struct W(crate::W<PDMA_PRISET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_PRISET_SPEC>;
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
impl core::convert::From<crate::W<PDMA_PRISET_SPEC>> for W {
    fn from(writer: crate::W<PDMA_PRISET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Fixed Priority Setting Register\\nSet this bit to 1 to enable fixed priority level.\\nWrite Operation:\\nNote1: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\\nNote2: FPRISET8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FPRISETN_A {
    #[doc = "0: No effect.\\nCorresponding PDMA channel is round-robin priority"]
    _0 = 0,
    #[doc = "1: Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    _1 = 1,
}
impl From<FPRISETN_A> for u16 {
    #[inline(always)]
    fn from(variant: FPRISETN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FPRISETn` reader - PDMA Fixed Priority Setting Register\\nSet this bit to 1 to enable fixed priority level.\\nWrite Operation:\\nNote1: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\\nNote2: FPRISET8~11 is M45xG/M45xE only."]
pub struct FPRISETN_R(crate::FieldReader<u16, FPRISETN_A>);
impl FPRISETN_R {
    pub(crate) fn new(bits: u16) -> Self {
        FPRISETN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FPRISETN_A> {
        match self.bits {
            0 => Some(FPRISETN_A::_0),
            1 => Some(FPRISETN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FPRISETN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FPRISETN_A::_1
    }
}
impl core::ops::Deref for FPRISETN_R {
    type Target = crate::FieldReader<u16, FPRISETN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRISETn` writer - PDMA Fixed Priority Setting Register\\nSet this bit to 1 to enable fixed priority level.\\nWrite Operation:\\nNote1: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\\nNote2: FPRISET8~11 is M45xG/M45xE only."]
pub struct FPRISETN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRISETN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRISETN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect.\\nCorresponding PDMA channel is round-robin priority"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRISETN_A::_0)
    }
    #[doc = "Set PDMA channel \\[n\\]
to fixed priority channel.\\nCorresponding PDMA channel is fixed priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRISETN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PDMA Fixed Priority Setting Register\\nSet this bit to 1 to enable fixed priority level.\\nWrite Operation:\\nNote1: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\\nNote2: FPRISET8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn fprisetn(&self) -> FPRISETN_R {
        FPRISETN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Fixed Priority Setting Register\\nSet this bit to 1 to enable fixed priority level.\\nWrite Operation:\\nNote1: This field only set to fixed priority, clear fixed priority use PDMA_PRICLR register.\\nNote2: FPRISET8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn fprisetn(&mut self) -> FPRISETN_W {
        FPRISETN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Fixed Priority Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_priset](index.html) module"]
pub struct PDMA_PRISET_SPEC;
impl crate::RegisterSpec for PDMA_PRISET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_priset::R](R) reader structure"]
impl crate::Readable for PDMA_PRISET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_priset::W](W) writer structure"]
impl crate::Writable for PDMA_PRISET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_PRISET to value 0"]
impl crate::Resettable for PDMA_PRISET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
