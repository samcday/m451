#[doc = "Register `PDMA_TOC0_1` reader"]
pub struct R(crate::R<PDMA_TOC0_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOC0_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TOC0_1_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TOC0_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOC0_1` writer"]
pub struct W(crate::W<PDMA_TOC0_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOC0_1_SPEC>;
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
impl core::convert::From<crate::W<PDMA_TOC0_1_SPEC>> for W {
    fn from(writer: crate::W<PDMA_TOC0_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC0` reader - Time-out Counter for Channel 0\\nThis controls the period of time-out function for channel 0. The calculation unit is based on 10 kHz clock."]
pub struct TOC0_R(crate::FieldReader<u16, u16>);
impl TOC0_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOC0` writer - Time-out Counter for Channel 0\\nThis controls the period of time-out function for channel 0. The calculation unit is based on 10 kHz clock."]
pub struct TOC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TOC1` reader - Time-out Counter for Channel 1\\nThis controls the period of time-out function for channel 1. The calculation unit is based on 10 kHz clock."]
pub struct TOC1_R(crate::FieldReader<u16, u16>);
impl TOC1_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOC1` writer - Time-out Counter for Channel 1\\nThis controls the period of time-out function for channel 1. The calculation unit is based on 10 kHz clock."]
pub struct TOC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time-out Counter for Channel 0\\nThis controls the period of time-out function for channel 0. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc0(&self) -> TOC0_R {
        TOC0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Time-out Counter for Channel 1\\nThis controls the period of time-out function for channel 1. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc1(&self) -> TOC1_R {
        TOC1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time-out Counter for Channel 0\\nThis controls the period of time-out function for channel 0. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc0(&mut self) -> TOC0_W {
        TOC0_W { w: self }
    }
    #[doc = "Bits 16:31 - Time-out Counter for Channel 1\\nThis controls the period of time-out function for channel 1. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc1(&mut self) -> TOC1_W {
        TOC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Counter Ch1 and Ch0 Register (M45xD/M45xC Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_toc0_1](index.html) module"]
pub struct PDMA_TOC0_1_SPEC;
impl crate::RegisterSpec for PDMA_TOC0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_toc0_1::R](R) reader structure"]
impl crate::Readable for PDMA_TOC0_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_toc0_1::W](W) writer structure"]
impl crate::Writable for PDMA_TOC0_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOC0_1 to value 0xffff_ffff"]
impl crate::Resettable for PDMA_TOC0_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
