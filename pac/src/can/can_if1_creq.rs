#[doc = "Register `CAN_IF1_CREQ` reader"]
pub struct R(crate::R<CAN_IF1_CREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF1_CREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF1_CREQ_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF1_CREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF1_CREQ` writer"]
pub struct W(crate::W<CAN_IF1_CREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF1_CREQ_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF1_CREQ_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF1_CREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MessageNumber` reader - Message Number\\n0x01-0x20: Valid Message Number, the Message Object in the Message\\nRAM is selected for data transfer.\\n0x00: Not a valid Message Number, interpreted as 0x20.\\n0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
pub struct MESSAGENUMBER_R(crate::FieldReader<u8, u8>);
impl MESSAGENUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        MESSAGENUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MESSAGENUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MessageNumber` writer - Message Number\\n0x01-0x20: Valid Message Number, the Message Object in the Message\\nRAM is selected for data transfer.\\n0x00: Not a valid Message Number, interpreted as 0x20.\\n0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
pub struct MESSAGENUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> MESSAGENUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Read/write action has finished"]
    _0 = 0,
    #[doc = "1: Writing to the IFn Command Request Register is in progress. This bit can only be read by the software"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Busy` reader - Busy Flag"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Busy` writer - Busy Flag"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read/write action has finished"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSY_A::_0)
    }
    #[doc = "Writing to the IFn Command Request Register is in progress. This bit can only be read by the software"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSY_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Message Number\\n0x01-0x20: Valid Message Number, the Message Object in the Message\\nRAM is selected for data transfer.\\n0x00: Not a valid Message Number, interpreted as 0x20.\\n0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
    #[inline(always)]
    pub fn message_number(&self) -> MESSAGENUMBER_R {
        MESSAGENUMBER_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number\\n0x01-0x20: Valid Message Number, the Message Object in the Message\\nRAM is selected for data transfer.\\n0x00: Not a valid Message Number, interpreted as 0x20.\\n0x21-0x3F: Not a valid Message Number, interpreted as 0x01-0x1F."]
    #[inline(always)]
    pub fn message_number(&mut self) -> MESSAGENUMBER_W {
        MESSAGENUMBER_W { w: self }
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn (Register Map Note 2) Command Request Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_creq](index.html) module"]
pub struct CAN_IF1_CREQ_SPEC;
impl crate::RegisterSpec for CAN_IF1_CREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if1_creq::R](R) reader structure"]
impl crate::Readable for CAN_IF1_CREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if1_creq::W](W) writer structure"]
impl crate::Writable for CAN_IF1_CREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF1_CREQ to value 0x01"]
impl crate::Resettable for CAN_IF1_CREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
