#[doc = "Register `PDMA_TRGSTS` reader"]
pub struct R(crate::R<PDMA_TRGSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TRGSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TRGSTS_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TRGSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "PDMA Channel Request Status (Read Only)\\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: REQSTS8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum REQSTSN_A {
    #[doc = "0: PDMA Channel n has no request"]
    _0 = 0,
    #[doc = "1: PDMA Channel n has a request"]
    _1 = 1,
}
impl From<REQSTSN_A> for u16 {
    #[inline(always)]
    fn from(variant: REQSTSN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REQSTSn` reader - PDMA Channel Request Status (Read Only)\\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: REQSTS8~11 is M45xG/M45xE only."]
pub struct REQSTSN_R(crate::FieldReader<u16, REQSTSN_A>);
impl REQSTSN_R {
    pub(crate) fn new(bits: u16) -> Self {
        REQSTSN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQSTSN_A> {
        match self.bits {
            0 => Some(REQSTSN_A::_0),
            1 => Some(REQSTSN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQSTSN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQSTSN_A::_1
    }
}
impl core::ops::Deref for REQSTSN_R {
    type Target = crate::FieldReader<u16, REQSTSN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - PDMA Channel Request Status (Read Only)\\nThis flag indicates whether channel\\[n\\]
have a request or not, no matter request from software or peripheral. When PDMA controller finishes channel transfer, this bit will be cleared automatically. \\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: REQSTS8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn reqstsn(&self) -> REQSTSN_R {
        REQSTSN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "PDMA Channel Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_trgsts](index.html) module"]
pub struct PDMA_TRGSTS_SPEC;
impl crate::RegisterSpec for PDMA_TRGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_trgsts::R](R) reader structure"]
impl crate::Readable for PDMA_TRGSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDMA_TRGSTS to value 0"]
impl crate::Resettable for PDMA_TRGSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
