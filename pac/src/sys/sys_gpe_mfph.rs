#[doc = "Register `SYS_GPE_MFPH` reader"]
pub struct R(crate::R<SYS_GPE_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPE_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPE_MFPH_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPE_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPE_MFPH` writer"]
pub struct W(crate::W<SYS_GPE_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPE_MFPH_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPE_MFPH_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPE_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE8MFP` reader - PE.8 Multi-function Pin Selection"]
pub struct PE8MFP_R(crate::FieldReader<u8, u8>);
impl PE8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE8MFP` writer - PE.8 Multi-function Pin Selection"]
pub struct PE8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PE9MFP` reader - PE.9 Multi-function Pin Selection"]
pub struct PE9MFP_R(crate::FieldReader<u8, u8>);
impl PE9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE9MFP` writer - PE.9 Multi-function Pin Selection"]
pub struct PE9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PE10MFP` reader - PE.10 Multi-function Pin Selection"]
pub struct PE10MFP_R(crate::FieldReader<u8, u8>);
impl PE10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE10MFP` writer - PE.10 Multi-function Pin Selection"]
pub struct PE10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PE11MFP` reader - PE.11 Multi-function Pin Selection"]
pub struct PE11MFP_R(crate::FieldReader<u8, u8>);
impl PE11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE11MFP` writer - PE.11 Multi-function Pin Selection"]
pub struct PE11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PE12MFP` reader - PE.12 Multi-function Pin Selection"]
pub struct PE12MFP_R(crate::FieldReader<u8, u8>);
impl PE12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE12MFP` writer - PE.12 Multi-function Pin Selection"]
pub struct PE12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PE13MFP` reader - PE.13 Multi-function Pin Selection"]
pub struct PE13MFP_R(crate::FieldReader<u8, u8>);
impl PE13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE13MFP` writer - PE.13 Multi-function Pin Selection"]
pub struct PE13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PE14_MFP` reader - PE.14 Multi-function Pin Selection"]
pub struct PE14_MFP_R(crate::FieldReader<u8, u8>);
impl PE14_MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE14_MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE14_MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE14_MFP` writer - PE.14 Multi-function Pin Selection"]
pub struct PE14_MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE14_MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PE.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe8mfp(&self) -> PE8MFP_R {
        PE8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe9mfp(&self) -> PE9MFP_R {
        PE9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PE.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe10mfp(&self) -> PE10MFP_R {
        PE10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PE.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe11mfp(&self) -> PE11MFP_R {
        PE11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PE.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe12mfp(&self) -> PE12MFP_R {
        PE12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PE.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe13mfp(&self) -> PE13MFP_R {
        PE13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PE.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe14_mfp(&self) -> PE14_MFP_R {
        PE14_MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe8mfp(&mut self) -> PE8MFP_W {
        PE8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PE.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe9mfp(&mut self) -> PE9MFP_W {
        PE9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PE.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe10mfp(&mut self) -> PE10MFP_W {
        PE10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PE.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe11mfp(&mut self) -> PE11MFP_W {
        PE11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PE.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe12mfp(&mut self) -> PE12MFP_W {
        PE12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PE.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe13mfp(&mut self) -> PE13MFP_W {
        PE13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PE.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe14_mfp(&mut self) -> PE14_MFP_W {
        PE14_MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpe_mfph](index.html) module"]
pub struct SYS_GPE_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPE_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpe_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPE_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpe_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPE_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPE_MFPH to value 0"]
impl crate::Resettable for SYS_GPE_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
