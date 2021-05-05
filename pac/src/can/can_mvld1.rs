#[doc = "Register `CAN_MVLD1` reader"]
pub struct R(crate::R<CAN_MVLD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_MVLD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_MVLD1_SPEC>> for R {
    fn from(reader: crate::R<CAN_MVLD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Message Valid Bits 16-1 (of All Message Objects) (Read Only)\\nEx. CAN_MVLD1\\[0\\]
means Message object No.1 is valid or not. If CAN_MVLD1\\[0\\]
is set, message object No.1 is configured.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum MSGVAL16_1_A {
    #[doc = "0: This Message Object is ignored by the Message Handler"]
    _0 = 0,
    #[doc = "1: This Message Object is configured and should be considered by the Message Handler"]
    _1 = 1,
}
impl From<MSGVAL16_1_A> for u16 {
    #[inline(always)]
    fn from(variant: MSGVAL16_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MsgVal16_1` reader - Message Valid Bits 16-1 (of All Message Objects) (Read Only)\\nEx. CAN_MVLD1\\[0\\]
means Message object No.1 is valid or not. If CAN_MVLD1\\[0\\]
is set, message object No.1 is configured."]
pub struct MSGVAL16_1_R(crate::FieldReader<u16, MSGVAL16_1_A>);
impl MSGVAL16_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        MSGVAL16_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSGVAL16_1_A> {
        match self.bits {
            0 => Some(MSGVAL16_1_A::_0),
            1 => Some(MSGVAL16_1_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSGVAL16_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSGVAL16_1_A::_1
    }
}
impl core::ops::Deref for MSGVAL16_1_R {
    type Target = crate::FieldReader<u16, MSGVAL16_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Message Valid Bits 16-1 (of All Message Objects) (Read Only)\\nEx. CAN_MVLD1\\[0\\]
means Message object No.1 is valid or not. If CAN_MVLD1\\[0\\]
is set, message object No.1 is configured."]
    #[inline(always)]
    pub fn msg_val16_1(&self) -> MSGVAL16_1_R {
        MSGVAL16_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Message Valid Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_mvld1](index.html) module"]
pub struct CAN_MVLD1_SPEC;
impl crate::RegisterSpec for CAN_MVLD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_mvld1::R](R) reader structure"]
impl crate::Readable for CAN_MVLD1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_MVLD1 to value 0"]
impl crate::Resettable for CAN_MVLD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
