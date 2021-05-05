#[doc = "Register `CAN_IF2_MASK1` reader"]
pub struct R(crate::R<CAN_IF2_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_MASK1_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_MASK1` writer"]
pub struct W(crate::W<CAN_IF2_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_MASK1_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_MASK1_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Identifier Mask 15-0\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum MSK_A {
    #[doc = "0: The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering"]
    _0 = 0,
    #[doc = "1: The corresponding identifier bit is used for acceptance filtering"]
    _1 = 1,
}
impl From<MSK_A> for u16 {
    #[inline(always)]
    fn from(variant: MSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Msk` reader - Identifier Mask 15-0"]
pub struct MSK_R(crate::FieldReader<u16, MSK_A>);
impl MSK_R {
    pub(crate) fn new(bits: u16) -> Self {
        MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSK_A> {
        match self.bits {
            0 => Some(MSK_A::_0),
            1 => Some(MSK_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSK_A::_1
    }
}
impl core::ops::Deref for MSK_R {
    type Target = crate::FieldReader<u16, MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Msk` writer - Identifier Mask 15-0"]
pub struct MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSK_A::_0)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSK_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Identifier Mask 15-0"]
    #[inline(always)]
    pub fn msk(&self) -> MSK_R {
        MSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Identifier Mask 15-0"]
    #[inline(always)]
    pub fn msk(&mut self) -> MSK_W {
        MSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Mask 1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mask1](index.html) module"]
pub struct CAN_IF2_MASK1_SPEC;
impl crate::RegisterSpec for CAN_IF2_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_mask1::R](R) reader structure"]
impl crate::Readable for CAN_IF2_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_mask1::W](W) writer structure"]
impl crate::Writable for CAN_IF2_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_MASK1 to value 0xffff"]
impl crate::Resettable for CAN_IF2_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
