#[doc = "Register `PDMA_SCATBA` reader"]
pub struct R(crate::R<PDMA_SCATBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_SCATBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_SCATBA_SPEC>> for R {
    fn from(reader: crate::R<PDMA_SCATBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_SCATBA` writer"]
pub struct W(crate::W<PDMA_SCATBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_SCATBA_SPEC>;
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
impl core::convert::From<crate::W<PDMA_SCATBA_SPEC>> for W {
    fn from(writer: crate::W<PDMA_SCATBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCATBA` reader - PDMA Scatter-gather Descriptor Table Address Register\\nIn Scatter-Gather mode, this is the base address for calculating the next link - list address. The next link address equation is \\nNote: Only useful in Scatter-Gather mode."]
pub struct SCATBA_R(crate::FieldReader<u16, u16>);
impl SCATBA_R {
    pub(crate) fn new(bits: u16) -> Self {
        SCATBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCATBA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCATBA` writer - PDMA Scatter-gather Descriptor Table Address Register\\nIn Scatter-Gather mode, this is the base address for calculating the next link - list address. The next link address equation is \\nNote: Only useful in Scatter-Gather mode."]
pub struct SCATBA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCATBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - PDMA Scatter-gather Descriptor Table Address Register\\nIn Scatter-Gather mode, this is the base address for calculating the next link - list address. The next link address equation is \\nNote: Only useful in Scatter-Gather mode."]
    #[inline(always)]
    pub fn scatba(&self) -> SCATBA_R {
        SCATBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - PDMA Scatter-gather Descriptor Table Address Register\\nIn Scatter-Gather mode, this is the base address for calculating the next link - list address. The next link address equation is \\nNote: Only useful in Scatter-Gather mode."]
    #[inline(always)]
    pub fn scatba(&mut self) -> SCATBA_W {
        SCATBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Scatter-gather Descriptor Table Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_scatba](index.html) module"]
pub struct PDMA_SCATBA_SPEC;
impl crate::RegisterSpec for PDMA_SCATBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_scatba::R](R) reader structure"]
impl crate::Readable for PDMA_SCATBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_scatba::W](W) writer structure"]
impl crate::Writable for PDMA_SCATBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_SCATBA to value 0x2000_0000"]
impl crate::Resettable for PDMA_SCATBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
