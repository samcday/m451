#[doc = "Register `CAN_ERR` reader"]
pub struct R(crate::R<CAN_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_ERR_SPEC>> for R {
    fn from(reader: crate::R<CAN_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEC` reader - Transmit Error Counter\\nActual state of the Transmit Error Counter. Values between 0 and 255."]
pub struct TEC_R(crate::FieldReader<u8, u8>);
impl TEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REC` reader - Receive Error Counter\\nActual state of the Receive Error Counter. Values between 0 and 127."]
pub struct REC_R(crate::FieldReader<u8, u8>);
impl REC_R {
    pub(crate) fn new(bits: u8) -> Self {
        REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Error Passive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RP_A {
    #[doc = "0: The Receive Error Counter is below the error passive level"]
    _0 = 0,
    #[doc = "1: The Receive Error Counter has reached the error passive level as defined in the CAN Specification"]
    _1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP` reader - Receive Error Passive"]
pub struct RP_R(crate::FieldReader<bool, RP_A>);
impl RP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::_0,
            true => RP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RP_A::_1
    }
}
impl core::ops::Deref for RP_R {
    type Target = crate::FieldReader<bool, RP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter\\nActual state of the Transmit Error Counter. Values between 0 and 255."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter\\nActual state of the Receive Error Counter. Values between 0 and 127."]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_err](index.html) module"]
pub struct CAN_ERR_SPEC;
impl crate::RegisterSpec for CAN_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_err::R](R) reader structure"]
impl crate::Readable for CAN_ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_ERR to value 0"]
impl crate::Resettable for CAN_ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
