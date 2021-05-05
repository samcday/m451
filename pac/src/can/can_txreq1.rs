#[doc = "Register `CAN_TXREQ1` reader"]
pub struct R(crate::R<CAN_TXREQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TXREQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_TXREQ1_SPEC>> for R {
    fn from(reader: crate::R<CAN_TXREQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transmission Request Bits 16-1 (of All Message Objects)\\nThese bits are read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXRQST16_1_A {
    #[doc = "0: This Message Object is not waiting for transmission"]
    _0 = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done"]
    _1 = 1,
}
impl From<TXRQST16_1_A> for u16 {
    #[inline(always)]
    fn from(variant: TXRQST16_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TxRqst16_1` reader - Transmission Request Bits 16-1 (of All Message Objects)\\nThese bits are read only."]
pub struct TXRQST16_1_R(crate::FieldReader<u16, TXRQST16_1_A>);
impl TXRQST16_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXRQST16_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXRQST16_1_A> {
        match self.bits {
            0 => Some(TXRQST16_1_A::_0),
            1 => Some(TXRQST16_1_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRQST16_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRQST16_1_A::_1
    }
}
impl core::ops::Deref for TXRQST16_1_R {
    type Target = crate::FieldReader<u16, TXRQST16_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmission Request Bits 16-1 (of All Message Objects)\\nThese bits are read only."]
    #[inline(always)]
    pub fn tx_rqst16_1(&self) -> TXRQST16_1_R {
        TXRQST16_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmission Request Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_txreq1](index.html) module"]
pub struct CAN_TXREQ1_SPEC;
impl crate::RegisterSpec for CAN_TXREQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_txreq1::R](R) reader structure"]
impl crate::Readable for CAN_TXREQ1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_TXREQ1 to value 0"]
impl crate::Resettable for CAN_TXREQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
