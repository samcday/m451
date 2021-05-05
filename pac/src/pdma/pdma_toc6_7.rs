#[doc = "Register `PDMA_TOC6_7` reader"]
pub struct R(crate::R<PDMA_TOC6_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_TOC6_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_TOC6_7_SPEC>> for R {
    fn from(reader: crate::R<PDMA_TOC6_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_TOC6_7` writer"]
pub struct W(crate::W<PDMA_TOC6_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_TOC6_7_SPEC>;
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
impl core::convert::From<crate::W<PDMA_TOC6_7_SPEC>> for W {
    fn from(writer: crate::W<PDMA_TOC6_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC6` reader - Time-out Period Counter for Channel 6\\nThis controls the period of time-out function for channel 6. The calculation unit is based on 10 kHz clock."]
pub struct TOC6_R(crate::FieldReader<u16, u16>);
impl TOC6_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOC6` writer - Time-out Period Counter for Channel 6\\nThis controls the period of time-out function for channel 6. The calculation unit is based on 10 kHz clock."]
pub struct TOC6_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TOC7` reader - Time-out Period Counter for Channel 7\\nThis controls the period of time-out function for channel 7. The calculation unit is based on 10 kHz clock."]
pub struct TOC7_R(crate::FieldReader<u16, u16>);
impl TOC7_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOC7` writer - Time-out Period Counter for Channel 7\\nThis controls the period of time-out function for channel 7. The calculation unit is based on 10 kHz clock."]
pub struct TOC7_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time-out Period Counter for Channel 6\\nThis controls the period of time-out function for channel 6. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc6(&self) -> TOC6_R {
        TOC6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Time-out Period Counter for Channel 7\\nThis controls the period of time-out function for channel 7. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc7(&self) -> TOC7_R {
        TOC7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time-out Period Counter for Channel 6\\nThis controls the period of time-out function for channel 6. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc6(&mut self) -> TOC6_W {
        TOC6_W { w: self }
    }
    #[doc = "Bits 16:31 - Time-out Period Counter for Channel 7\\nThis controls the period of time-out function for channel 7. The calculation unit is based on 10 kHz clock."]
    #[inline(always)]
    pub fn toc7(&mut self) -> TOC7_W {
        TOC7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Time-out Counter Ch7 and Ch6 Register (M45xD/M45xC Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_toc6_7](index.html) module"]
pub struct PDMA_TOC6_7_SPEC;
impl crate::RegisterSpec for PDMA_TOC6_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_toc6_7::R](R) reader structure"]
impl crate::Readable for PDMA_TOC6_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_toc6_7::W](W) writer structure"]
impl crate::Writable for PDMA_TOC6_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_TOC6_7 to value 0xffff_ffff"]
impl crate::Resettable for PDMA_TOC6_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
