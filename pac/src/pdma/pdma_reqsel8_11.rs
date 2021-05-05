#[doc = "Register `PDMA_REQSEL8_11` reader"]
pub struct R(crate::R<PDMA_REQSEL8_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_REQSEL8_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_REQSEL8_11_SPEC>> for R {
    fn from(reader: crate::R<PDMA_REQSEL8_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_REQSEL8_11` writer"]
pub struct W(crate::W<PDMA_REQSEL8_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_REQSEL8_11_SPEC>;
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
impl core::convert::From<crate::W<PDMA_REQSEL8_11_SPEC>> for W {
    fn from(writer: crate::W<PDMA_REQSEL8_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQSRC8` reader - Channel 8 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC8_R(crate::FieldReader<u8, u8>);
impl REQSRC8_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC8` writer - Channel 8 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC8_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `REQSRC9` reader - Channel 9 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 9. User can configure the peripheral setting by REQSRC9. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC9_R(crate::FieldReader<u8, u8>);
impl REQSRC9_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC9` writer - Channel 9 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 9. User can configure the peripheral setting by REQSRC9. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC9_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `REQSRC10` reader - Channel 10 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 10. User can configure the peripheral setting by REQSRC10. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC10_R(crate::FieldReader<u8, u8>);
impl REQSRC10_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC10` writer - Channel 10 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 10. User can configure the peripheral setting by REQSRC10. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC10_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `REQSRC11` reader - Channel 11 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 11. User can configure the peripheral setting by REQSRC11. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC11_R(crate::FieldReader<u8, u8>);
impl REQSRC11_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC11` writer - Channel 11 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 11. User can configure the peripheral setting by REQSRC11. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC11_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Channel 8 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc8(&self) -> REQSRC8_R {
        REQSRC8_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Channel 9 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 9. User can configure the peripheral setting by REQSRC9. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc9(&self) -> REQSRC9_R {
        REQSRC9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Channel 10 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 10. User can configure the peripheral setting by REQSRC10. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc10(&self) -> REQSRC10_R {
        REQSRC10_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Channel 11 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 11. User can configure the peripheral setting by REQSRC11. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc11(&self) -> REQSRC11_R {
        REQSRC11_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Channel 8 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 8. User can configure the peripheral setting by REQSRC8. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc8(&mut self) -> REQSRC8_W {
        REQSRC8_W { w: self }
    }
    #[doc = "Bits 8:12 - Channel 9 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 9. User can configure the peripheral setting by REQSRC9. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc9(&mut self) -> REQSRC9_W {
        REQSRC9_W { w: self }
    }
    #[doc = "Bits 16:20 - Channel 10 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 10. User can configure the peripheral setting by REQSRC10. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc10(&mut self) -> REQSRC10_W {
        REQSRC10_W { w: self }
    }
    #[doc = "Bits 24:28 - Channel 11 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 11. User can configure the peripheral setting by REQSRC11. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc11(&mut self) -> REQSRC11_W {
        REQSRC11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Request Source Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_reqsel8_11](index.html) module"]
pub struct PDMA_REQSEL8_11_SPEC;
impl crate::RegisterSpec for PDMA_REQSEL8_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_reqsel8_11::R](R) reader structure"]
impl crate::Readable for PDMA_REQSEL8_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_reqsel8_11::W](W) writer structure"]
impl crate::Writable for PDMA_REQSEL8_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_REQSEL8_11 to value 0x1f1f_1f1f"]
impl crate::Resettable for PDMA_REQSEL8_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f1f_1f1f
    }
}
