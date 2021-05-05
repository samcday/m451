#[doc = "Register `SYS_GPA_MFPH` reader"]
pub struct R(crate::R<SYS_GPA_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPA_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPA_MFPH_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPA_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPA_MFPH` writer"]
pub struct W(crate::W<SYS_GPA_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPA_MFPH_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPA_MFPH_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPA_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA8MFP` reader - PA.8 Multi-function Pin Selection"]
pub struct PA8MFP_R(crate::FieldReader<u8, u8>);
impl PA8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA8MFP` writer - PA.8 Multi-function Pin Selection"]
pub struct PA8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PA9MFP` reader - PA.9 Multi-function Pin Selection"]
pub struct PA9MFP_R(crate::FieldReader<u8, u8>);
impl PA9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA9MFP` writer - PA.9 Multi-function Pin Selection"]
pub struct PA9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PA10MFP` reader - PA.10 Multi-function Pin Selection"]
pub struct PA10MFP_R(crate::FieldReader<u8, u8>);
impl PA10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA10MFP` writer - PA.10 Multi-function Pin Selection"]
pub struct PA10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PA11MFP` reader - PA.11 Multi-function Pin Selection"]
pub struct PA11MFP_R(crate::FieldReader<u8, u8>);
impl PA11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA11MFP` writer - PA.11 Multi-function Pin Selection"]
pub struct PA11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PA12MFP` reader - PA.12 Multi-function Pin Selection"]
pub struct PA12MFP_R(crate::FieldReader<u8, u8>);
impl PA12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA12MFP` writer - PA.12 Multi-function Pin Selection"]
pub struct PA12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PA13MFP` reader - PA.13 Multi-function Pin Selection"]
pub struct PA13MFP_R(crate::FieldReader<u8, u8>);
impl PA13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA13MFP` writer - PA.13 Multi-function Pin Selection"]
pub struct PA13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PA14MFP` reader - PA.14 Multi-function Pin Selection"]
pub struct PA14MFP_R(crate::FieldReader<u8, u8>);
impl PA14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA14MFP` writer - PA.14 Multi-function Pin Selection"]
pub struct PA14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PA15MFP` reader - PA.15 Multi-function Pin Selection"]
pub struct PA15MFP_R(crate::FieldReader<u8, u8>);
impl PA15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA15MFP` writer - PA.15 Multi-function Pin Selection"]
pub struct PA15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PA.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa8mfp(&self) -> PA8MFP_R {
        PA8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PA.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa9mfp(&self) -> PA9MFP_R {
        PA9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PA.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa10mfp(&self) -> PA10MFP_R {
        PA10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PA.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa11mfp(&self) -> PA11MFP_R {
        PA11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PA.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa12mfp(&self) -> PA12MFP_R {
        PA12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PA.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa13mfp(&self) -> PA13MFP_R {
        PA13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PA.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa14mfp(&self) -> PA14MFP_R {
        PA14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PA.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa15mfp(&self) -> PA15MFP_R {
        PA15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PA.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa8mfp(&mut self) -> PA8MFP_W {
        PA8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PA.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa9mfp(&mut self) -> PA9MFP_W {
        PA9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PA.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa10mfp(&mut self) -> PA10MFP_W {
        PA10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PA.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa11mfp(&mut self) -> PA11MFP_W {
        PA11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PA.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa12mfp(&mut self) -> PA12MFP_W {
        PA12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PA.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa13mfp(&mut self) -> PA13MFP_W {
        PA13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PA.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa14mfp(&mut self) -> PA14MFP_W {
        PA14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PA.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa15mfp(&mut self) -> PA15MFP_W {
        PA15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOA High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpa_mfph](index.html) module"]
pub struct SYS_GPA_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPA_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpa_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPA_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpa_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPA_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPA_MFPH to value 0"]
impl crate::Resettable for SYS_GPA_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
