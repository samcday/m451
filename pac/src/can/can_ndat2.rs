#[doc = "Register `CAN_NDAT2` reader"]
pub struct R(crate::R<CAN_NDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_NDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_NDAT2_SPEC>> for R {
    fn from(reader: crate::R<CAN_NDAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "New Data Bits 32-17 (of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum NEWDATA32_17_A {
    #[doc = "0: No new data has been written into the data portion of this Message Object by the Message Handler since the last time this flag was cleared by the application software"]
    _0 = 0,
    #[doc = "1: The Message Handler or the application software has written new data into the data portion of this Message Object"]
    _1 = 1,
}
impl From<NEWDATA32_17_A> for u16 {
    #[inline(always)]
    fn from(variant: NEWDATA32_17_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NewData32_17` reader - New Data Bits 32-17 (of All Message Objects)"]
pub struct NEWDATA32_17_R(crate::FieldReader<u16, NEWDATA32_17_A>);
impl NEWDATA32_17_R {
    pub(crate) fn new(bits: u16) -> Self {
        NEWDATA32_17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEWDATA32_17_A> {
        match self.bits {
            0 => Some(NEWDATA32_17_A::_0),
            1 => Some(NEWDATA32_17_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEWDATA32_17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEWDATA32_17_A::_1
    }
}
impl core::ops::Deref for NEWDATA32_17_R {
    type Target = crate::FieldReader<u16, NEWDATA32_17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - New Data Bits 32-17 (of All Message Objects)"]
    #[inline(always)]
    pub fn new_data32_17(&self) -> NEWDATA32_17_R {
        NEWDATA32_17_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "New Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ndat2](index.html) module"]
pub struct CAN_NDAT2_SPEC;
impl crate::RegisterSpec for CAN_NDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_ndat2::R](R) reader structure"]
impl crate::Readable for CAN_NDAT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_NDAT2 to value 0"]
impl crate::Resettable for CAN_NDAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
