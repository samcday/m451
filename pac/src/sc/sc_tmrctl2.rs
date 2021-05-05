#[doc = "Register `SC_TMRCTL2` reader"]
pub struct R(crate::R<SC_TMRCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_TMRCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_TMRCTL2_SPEC>> for R {
    fn from(reader: crate::R<SC_TMRCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_TMRCTL2` writer"]
pub struct W(crate::W<SC_TMRCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_TMRCTL2_SPEC>;
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
impl core::convert::From<crate::W<SC_TMRCTL2_SPEC>> for W {
    fn from(writer: crate::W<SC_TMRCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Timer 2 Counter Value (ETU Base)\\nThis field indicates the internal timer operation values."]
pub struct CNT_R(crate::FieldReader<u8, u8>);
impl CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Timer 2 Counter Value (ETU Base)\\nThis field indicates the internal timer operation values."]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `OPMODE` reader - Timer 2 Operation Mode Selection\\nThis field indicates the internal 8-bit timer operation selection\\nRefer to 6.14.5.4 for programming Timer2"]
pub struct OPMODE_R(crate::FieldReader<u8, u8>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - Timer 2 Operation Mode Selection\\nThis field indicates the internal 8-bit timer operation selection\\nRefer to 6.14.5.4 for programming Timer2"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer 2 Counter Value (ETU Base)\\nThis field indicates the internal timer operation values."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Timer 2 Operation Mode Selection\\nThis field indicates the internal 8-bit timer operation selection\\nRefer to 6.14.5.4 for programming Timer2"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer 2 Counter Value (ETU Base)\\nThis field indicates the internal timer operation values."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Bits 24:27 - Timer 2 Operation Mode Selection\\nThis field indicates the internal 8-bit timer operation selection\\nRefer to 6.14.5.4 for programming Timer2"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Internal Timer Control Register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_tmrctl2](index.html) module"]
pub struct SC_TMRCTL2_SPEC;
impl crate::RegisterSpec for SC_TMRCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_tmrctl2::R](R) reader structure"]
impl crate::Readable for SC_TMRCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_tmrctl2::W](W) writer structure"]
impl crate::Writable for SC_TMRCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_TMRCTL2 to value 0"]
impl crate::Resettable for SC_TMRCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
