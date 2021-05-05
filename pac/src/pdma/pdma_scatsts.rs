#[doc = "Register `PDMA_SCATSTS` reader"]
pub struct R(crate::R<PDMA_SCATSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_SCATSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_SCATSTS_SPEC>> for R {
    fn from(reader: crate::R<PDMA_SCATSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_SCATSTS` writer"]
pub struct W(crate::W<PDMA_SCATSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_SCATSTS_SPEC>;
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
impl core::convert::From<crate::W<PDMA_SCATSTS_SPEC>> for W {
    fn from(writer: crate::W<PDMA_SCATSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scatter-gather Table Empty Flag Register\\nThis bit indicates which PDMA channel n Scatter Gather table is empty when SWREQn (PDMA_SWREQ\\[11:0\\]) set to high or channel has finished transmission and the operation mode is Stop mode. User can write 1 to clear these bits.\\nNote: TEMPTYF8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TEMPTYFN_A {
    #[doc = "0: PDMA channel scatter-gather table is not empty"]
    _0 = 0,
    #[doc = "1: PDMA channel scatter-gather table is empty and PDMA SWREQ has be set"]
    _1 = 1,
}
impl From<TEMPTYFN_A> for u16 {
    #[inline(always)]
    fn from(variant: TEMPTYFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEMPTYFn` reader - Scatter-gather Table Empty Flag Register\\nThis bit indicates which PDMA channel n Scatter Gather table is empty when SWREQn (PDMA_SWREQ\\[11:0\\]) set to high or channel has finished transmission and the operation mode is Stop mode. User can write 1 to clear these bits.\\nNote: TEMPTYF8~11 is M45xG/M45xE only."]
pub struct TEMPTYFN_R(crate::FieldReader<u16, TEMPTYFN_A>);
impl TEMPTYFN_R {
    pub(crate) fn new(bits: u16) -> Self {
        TEMPTYFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEMPTYFN_A> {
        match self.bits {
            0 => Some(TEMPTYFN_A::_0),
            1 => Some(TEMPTYFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TEMPTYFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TEMPTYFN_A::_1
    }
}
impl core::ops::Deref for TEMPTYFN_R {
    type Target = crate::FieldReader<u16, TEMPTYFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPTYFn` writer - Scatter-gather Table Empty Flag Register\\nThis bit indicates which PDMA channel n Scatter Gather table is empty when SWREQn (PDMA_SWREQ\\[11:0\\]) set to high or channel has finished transmission and the operation mode is Stop mode. User can write 1 to clear these bits.\\nNote: TEMPTYF8~11 is M45xG/M45xE only."]
pub struct TEMPTYFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPTYFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPTYFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDMA channel scatter-gather table is not empty"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEMPTYFN_A::_0)
    }
    #[doc = "PDMA channel scatter-gather table is empty and PDMA SWREQ has be set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEMPTYFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Scatter-gather Table Empty Flag Register\\nThis bit indicates which PDMA channel n Scatter Gather table is empty when SWREQn (PDMA_SWREQ\\[11:0\\]) set to high or channel has finished transmission and the operation mode is Stop mode. User can write 1 to clear these bits.\\nNote: TEMPTYF8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn temptyfn(&self) -> TEMPTYFN_R {
        TEMPTYFN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Scatter-gather Table Empty Flag Register\\nThis bit indicates which PDMA channel n Scatter Gather table is empty when SWREQn (PDMA_SWREQ\\[11:0\\]) set to high or channel has finished transmission and the operation mode is Stop mode. User can write 1 to clear these bits.\\nNote: TEMPTYF8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn temptyfn(&mut self) -> TEMPTYFN_W {
        TEMPTYFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Scatter-gather Table Empty Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_scatsts](index.html) module"]
pub struct PDMA_SCATSTS_SPEC;
impl crate::RegisterSpec for PDMA_SCATSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_scatsts::R](R) reader structure"]
impl crate::Readable for PDMA_SCATSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_scatsts::W](W) writer structure"]
impl crate::Writable for PDMA_SCATSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_SCATSTS to value 0"]
impl crate::Resettable for PDMA_SCATSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
