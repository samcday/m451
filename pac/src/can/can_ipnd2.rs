#[doc = "Register `CAN_IPND2` reader"]
pub struct R(crate::R<CAN_IPND2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IPND2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IPND2_SPEC>> for R {
    fn from(reader: crate::R<CAN_IPND2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt Pending Bits 32-17 (of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum INTPND32_17_A {
    #[doc = "0: This message object is not the source of an interrupt"]
    _0 = 0,
    #[doc = "1: This message object is the source of an interrupt"]
    _1 = 1,
}
impl From<INTPND32_17_A> for u16 {
    #[inline(always)]
    fn from(variant: INTPND32_17_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IntPnd32_17` reader - Interrupt Pending Bits 32-17 (of All Message Objects)"]
pub struct INTPND32_17_R(crate::FieldReader<u16, INTPND32_17_A>);
impl INTPND32_17_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTPND32_17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTPND32_17_A> {
        match self.bits {
            0 => Some(INTPND32_17_A::_0),
            1 => Some(INTPND32_17_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTPND32_17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTPND32_17_A::_1
    }
}
impl core::ops::Deref for INTPND32_17_R {
    type Target = crate::FieldReader<u16, INTPND32_17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits 32-17 (of All Message Objects)"]
    #[inline(always)]
    pub fn int_pnd32_17(&self) -> INTPND32_17_R {
        INTPND32_17_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Interrupt Pending Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ipnd2](index.html) module"]
pub struct CAN_IPND2_SPEC;
impl crate::RegisterSpec for CAN_IPND2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_ipnd2::R](R) reader structure"]
impl crate::Readable for CAN_IPND2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_IPND2 to value 0"]
impl crate::Resettable for CAN_IPND2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
