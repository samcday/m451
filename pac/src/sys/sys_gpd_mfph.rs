#[doc = "Register `SYS_GPD_MFPH` reader"]
pub struct R(crate::R<SYS_GPD_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPD_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPD_MFPH_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPD_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPD_MFPH` writer"]
pub struct W(crate::W<SYS_GPD_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPD_MFPH_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPD_MFPH_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPD_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD8MFP` reader - PD.8 Multi-function Pin Selection"]
pub struct PD8MFP_R(crate::FieldReader<u8, u8>);
impl PD8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD8MFP` writer - PD.8 Multi-function Pin Selection"]
pub struct PD8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PD9MFP` reader - PD.9 Multi-function Pin Selection"]
pub struct PD9MFP_R(crate::FieldReader<u8, u8>);
impl PD9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD9MFP` writer - PD.9 Multi-function Pin Selection"]
pub struct PD9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PD10MFP` reader - PD.10 Multi-function Pin Selection"]
pub struct PD10MFP_R(crate::FieldReader<u8, u8>);
impl PD10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD10MFP` writer - PD.10 Multi-function Pin Selection"]
pub struct PD10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PD11MFP` reader - PD.11 Multi-function Pin Selection"]
pub struct PD11MFP_R(crate::FieldReader<u8, u8>);
impl PD11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD11MFP` writer - PD.11 Multi-function Pin Selection"]
pub struct PD11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PD12MFP` reader - PD.12 Multi-function Pin Selection"]
pub struct PD12MFP_R(crate::FieldReader<u8, u8>);
impl PD12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD12MFP` writer - PD.12 Multi-function Pin Selection"]
pub struct PD12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PD13MFP` reader - PD.13 Multi-function Pin Selection"]
pub struct PD13MFP_R(crate::FieldReader<u8, u8>);
impl PD13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD13MFP` writer - PD.13 Multi-function Pin Selection"]
pub struct PD13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PD14MFP` reader - PD.14 Multi-function Pin Selection"]
pub struct PD14MFP_R(crate::FieldReader<u8, u8>);
impl PD14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD14MFP` writer - PD.14 Multi-function Pin Selection"]
pub struct PD14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PD15MFP` reader - PD.15 Multi-function Pin Selection"]
pub struct PD15MFP_R(crate::FieldReader<u8, u8>);
impl PD15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD15MFP` writer - PD.15 Multi-function Pin Selection"]
pub struct PD15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PD.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd8mfp(&self) -> PD8MFP_R {
        PD8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PD.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd9mfp(&self) -> PD9MFP_R {
        PD9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PD.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd10mfp(&self) -> PD10MFP_R {
        PD10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PD.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd11mfp(&self) -> PD11MFP_R {
        PD11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PD.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd12mfp(&self) -> PD12MFP_R {
        PD12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PD.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd13mfp(&self) -> PD13MFP_R {
        PD13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PD.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd14mfp(&self) -> PD14MFP_R {
        PD14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PD.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd15mfp(&self) -> PD15MFP_R {
        PD15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PD.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd8mfp(&mut self) -> PD8MFP_W {
        PD8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PD.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd9mfp(&mut self) -> PD9MFP_W {
        PD9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PD.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd10mfp(&mut self) -> PD10MFP_W {
        PD10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PD.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd11mfp(&mut self) -> PD11MFP_W {
        PD11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PD.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd12mfp(&mut self) -> PD12MFP_W {
        PD12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PD.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd13mfp(&mut self) -> PD13MFP_W {
        PD13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PD.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd14mfp(&mut self) -> PD14MFP_W {
        PD14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PD.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd15mfp(&mut self) -> PD15MFP_W {
        PD15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpd_mfph](index.html) module"]
pub struct SYS_GPD_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPD_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpd_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPD_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpd_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPD_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPD_MFPH to value 0"]
impl crate::Resettable for SYS_GPD_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
