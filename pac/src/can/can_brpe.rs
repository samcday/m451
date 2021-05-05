#[doc = "Register `CAN_BRPE` reader"]
pub struct R(crate::R<CAN_BRPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_BRPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_BRPE_SPEC>> for R {
    fn from(reader: crate::R<CAN_BRPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_BRPE` writer"]
pub struct W(crate::W<CAN_BRPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_BRPE_SPEC>;
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
impl core::convert::From<crate::W<CAN_BRPE_SPEC>> for W {
    fn from(writer: crate::W<CAN_BRPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRPE` reader - BRPE: Baud Rate Prescaler Extension\\n0x00-0x0F: By programming BRPE, the Baud Rate Prescaler can be extended to values up to 1023. The actual interpretation by the hardware is that one more than the value programmed by BRPE (MSBs) and BTIME (LSBs) is used."]
pub struct BRPE_R(crate::FieldReader<u8, u8>);
impl BRPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRPE` writer - BRPE: Baud Rate Prescaler Extension\\n0x00-0x0F: By programming BRPE, the Baud Rate Prescaler can be extended to values up to 1023. The actual interpretation by the hardware is that one more than the value programmed by BRPE (MSBs) and BTIME (LSBs) is used."]
pub struct BRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - BRPE: Baud Rate Prescaler Extension\\n0x00-0x0F: By programming BRPE, the Baud Rate Prescaler can be extended to values up to 1023. The actual interpretation by the hardware is that one more than the value programmed by BRPE (MSBs) and BTIME (LSBs) is used."]
    #[inline(always)]
    pub fn brpe(&self) -> BRPE_R {
        BRPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BRPE: Baud Rate Prescaler Extension\\n0x00-0x0F: By programming BRPE, the Baud Rate Prescaler can be extended to values up to 1023. The actual interpretation by the hardware is that one more than the value programmed by BRPE (MSBs) and BTIME (LSBs) is used."]
    #[inline(always)]
    pub fn brpe(&mut self) -> BRPE_W {
        BRPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Prescaler Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_brpe](index.html) module"]
pub struct CAN_BRPE_SPEC;
impl crate::RegisterSpec for CAN_BRPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_brpe::R](R) reader structure"]
impl crate::Readable for CAN_BRPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_brpe::W](W) writer structure"]
impl crate::Writable for CAN_BRPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_BRPE to value 0"]
impl crate::Resettable for CAN_BRPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
