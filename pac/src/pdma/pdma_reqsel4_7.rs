#[doc = "Register `PDMA_REQSEL4_7` reader"]
pub struct R(crate::R<PDMA_REQSEL4_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_REQSEL4_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_REQSEL4_7_SPEC>> for R {
    fn from(reader: crate::R<PDMA_REQSEL4_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_REQSEL4_7` writer"]
pub struct W(crate::W<PDMA_REQSEL4_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_REQSEL4_7_SPEC>;
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
impl core::convert::From<crate::W<PDMA_REQSEL4_7_SPEC>> for W {
    fn from(writer: crate::W<PDMA_REQSEL4_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQSRC4` reader - Channel 4 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 4. User can configure the peripheral setting by REQSRC4. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC4_R(crate::FieldReader<u8, u8>);
impl REQSRC4_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC4` writer - Channel 4 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 4. User can configure the peripheral setting by REQSRC4. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `REQSRC5` reader - Channel 5 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 5. User can configure the peripheral setting by REQSRC5. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC5_R(crate::FieldReader<u8, u8>);
impl REQSRC5_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC5` writer - Channel 5 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 5. User can configure the peripheral setting by REQSRC5. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `REQSRC6` reader - Channel 6 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 6. User can configure the peripheral setting by REQSRC6. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC6_R(crate::FieldReader<u8, u8>);
impl REQSRC6_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC6` writer - Channel 6 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 6. User can configure the peripheral setting by REQSRC6. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `REQSRC7` reader - Channel 7 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 7. User can configure the peripheral setting by REQSRC7. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC7_R(crate::FieldReader<u8, u8>);
impl REQSRC7_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQSRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQSRC7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSRC7` writer - Channel 7 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 7. User can configure the peripheral setting by REQSRC7. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
pub struct REQSRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSRC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Channel 4 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 4. User can configure the peripheral setting by REQSRC4. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc4(&self) -> REQSRC4_R {
        REQSRC4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Channel 5 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 5. User can configure the peripheral setting by REQSRC5. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc5(&self) -> REQSRC5_R {
        REQSRC5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Channel 6 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 6. User can configure the peripheral setting by REQSRC6. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc6(&self) -> REQSRC6_R {
        REQSRC6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Channel 7 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 7. User can configure the peripheral setting by REQSRC7. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc7(&self) -> REQSRC7_R {
        REQSRC7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Channel 4 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 4. User can configure the peripheral setting by REQSRC4. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc4(&mut self) -> REQSRC4_W {
        REQSRC4_W { w: self }
    }
    #[doc = "Bits 8:12 - Channel 5 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 5. User can configure the peripheral setting by REQSRC5. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc5(&mut self) -> REQSRC5_W {
        REQSRC5_W { w: self }
    }
    #[doc = "Bits 16:20 - Channel 6 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 6. User can configure the peripheral setting by REQSRC6. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc6(&mut self) -> REQSRC6_W {
        REQSRC6_W { w: self }
    }
    #[doc = "Bits 24:28 - Channel 7 Request Source Selection\\nThis filed defines which peripheral is connected to PDMA channel 7. User can configure the peripheral setting by REQSRC7. \\nNote: The channel configuration is the same as REQSRC0 field. Please refer to the explanation of REQSRC0."]
    #[inline(always)]
    pub fn reqsrc7(&mut self) -> REQSRC7_W {
        REQSRC7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Request Source Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_reqsel4_7](index.html) module"]
pub struct PDMA_REQSEL4_7_SPEC;
impl crate::RegisterSpec for PDMA_REQSEL4_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_reqsel4_7::R](R) reader structure"]
impl crate::Readable for PDMA_REQSEL4_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_reqsel4_7::W](W) writer structure"]
impl crate::Writable for PDMA_REQSEL4_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_REQSEL4_7 to value 0x1f1f_1f1f"]
impl crate::Resettable for PDMA_REQSEL4_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f1f_1f1f
    }
}
