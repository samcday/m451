#[doc = "Register `CAN_IF2_ARB1` reader"]
pub struct R(crate::R<CAN_IF2_ARB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_ARB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_ARB1_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_ARB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_ARB1` writer"]
pub struct W(crate::W<CAN_IF2_ARB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_ARB1_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_ARB1_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_ARB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Message Identifier 15-0\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Message Identifier 15-0\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Message Identifier 15-0\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Identifier 15-0\\nID28 - ID0, 29-bit Identifier ('Extended Frame').\\nID28 - ID18, 11-bit Identifier ('Standard Frame')"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Arbitration 1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_arb1](index.html) module"]
pub struct CAN_IF2_ARB1_SPEC;
impl crate::RegisterSpec for CAN_IF2_ARB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_arb1::R](R) reader structure"]
impl crate::Readable for CAN_IF2_ARB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_arb1::W](W) writer structure"]
impl crate::Writable for CAN_IF2_ARB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_ARB1 to value 0"]
impl crate::Resettable for CAN_IF2_ARB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
