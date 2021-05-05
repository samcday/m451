#[doc = "Register `PDMA_INTEN` reader"]
pub struct R(crate::R<PDMA_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_INTEN_SPEC>> for R {
    fn from(reader: crate::R<PDMA_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_INTEN` writer"]
pub struct W(crate::W<PDMA_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_INTEN_SPEC>;
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
impl core::convert::From<crate::W<PDMA_INTEN_SPEC>> for W {
    fn from(writer: crate::W<PDMA_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Interrupt Enable Register\\nThis field is used for enabling PDMA channel\\[n\\]
interrupt.\\nNote: INTEN8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum INTENN_A {
    #[doc = "0: PDMA channel n interrupt Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel n interrupt Enabled"]
    _1 = 1,
}
impl From<INTENN_A> for u16 {
    #[inline(always)]
    fn from(variant: INTENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTENn` reader - PDMA Interrupt Enable Register\\nThis field is used for enabling PDMA channel\\[n\\]
interrupt.\\nNote: INTEN8~11 is M45xG/M45xE only."]
pub struct INTENN_R(crate::FieldReader<u16, INTENN_A>);
impl INTENN_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTENN_A> {
        match self.bits {
            0 => Some(INTENN_A::_0),
            1 => Some(INTENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTENN_A::_1
    }
}
impl core::ops::Deref for INTENN_R {
    type Target = crate::FieldReader<u16, INTENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTENn` writer - PDMA Interrupt Enable Register\\nThis field is used for enabling PDMA channel\\[n\\]
interrupt.\\nNote: INTEN8~11 is M45xG/M45xE only."]
pub struct INTENN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDMA channel n interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTENN_A::_0)
    }
    #[doc = "PDMA channel n interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PDMA Interrupt Enable Register\\nThis field is used for enabling PDMA channel\\[n\\]
interrupt.\\nNote: INTEN8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn intenn(&self) -> INTENN_R {
        INTENN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Interrupt Enable Register\\nThis field is used for enabling PDMA channel\\[n\\]
interrupt.\\nNote: INTEN8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn intenn(&mut self) -> INTENN_W {
        INTENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_inten](index.html) module"]
pub struct PDMA_INTEN_SPEC;
impl crate::RegisterSpec for PDMA_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_inten::R](R) reader structure"]
impl crate::Readable for PDMA_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_inten::W](W) writer structure"]
impl crate::Writable for PDMA_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_INTEN to value 0"]
impl crate::Resettable for PDMA_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
