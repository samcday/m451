#[doc = "Register `PDMA_TACTSTS` reader"]
pub struct R(crate::R<PDMA_TACTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TACTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TACTSTS_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TACTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transfer on Active Flag Register (Read Only)\\nThis bit indicates which PDMA channel is in active.\\nNote: TXACTF8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXACTFN_A {
    #[doc = "0: PDMA channel is not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel is active"]
    _1 = 1,
}
impl From<TXACTFN_A> for u16 {
    #[inline(always)]
    fn from(variant: TXACTFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXACTFn` reader - Transfer on Active Flag Register (Read Only)\\nThis bit indicates which PDMA channel is in active.\\nNote: TXACTF8~11 is M45xG/M45xE only."]
pub struct TXACTFN_R(crate::FieldReader<u16, TXACTFN_A>);
impl TXACTFN_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXACTFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXACTFN_A> {
        match self.bits {
            0 => Some(TXACTFN_A::_0),
            1 => Some(TXACTFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXACTFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXACTFN_A::_1
    }
}
impl core::ops::Deref for TXACTFN_R {
    type Target = crate::FieldReader<u16, TXACTFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer on Active Flag Register (Read Only)\\nThis bit indicates which PDMA channel is in active.\\nNote: TXACTF8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn txactfn(&self) -> TXACTFN_R {
        TXACTFN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "PDMA Transfer Active Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_tactsts](index.html) module"]
pub struct PDMA_TACTSTS_SPEC;
impl crate::RegisterSpec for PDMA_TACTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_tactsts::R](R) reader structure"]
impl crate::Readable for PDMA_TACTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDMA_TACTSTS to value 0"]
impl crate::Resettable for PDMA_TACTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
