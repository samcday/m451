#[doc = "Register `SYS_GPB_MFPH` reader"]
pub struct R(crate::R<SYS_GPB_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPB_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPB_MFPH_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPB_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPB_MFPH` writer"]
pub struct W(crate::W<SYS_GPB_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPB_MFPH_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPB_MFPH_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPB_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB8MFP` reader - PB.8 Multi-function Pin Selection"]
pub struct PB8MFP_R(crate::FieldReader<u8, u8>);
impl PB8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB8MFP` writer - PB.8 Multi-function Pin Selection"]
pub struct PB8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PB9MFP` reader - PB.9 Multi-function Pin Selection"]
pub struct PB9MFP_R(crate::FieldReader<u8, u8>);
impl PB9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB9MFP` writer - PB.9 Multi-function Pin Selection"]
pub struct PB9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PB10MFP` reader - PB.10 Multi-function Pin Selection"]
pub struct PB10MFP_R(crate::FieldReader<u8, u8>);
impl PB10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB10MFP` writer - PB.10 Multi-function Pin Selection"]
pub struct PB10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PB11MFP` reader - PB.11 Multi-function Pin Selection"]
pub struct PB11MFP_R(crate::FieldReader<u8, u8>);
impl PB11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB11MFP` writer - PB.11 Multi-function Pin Selection"]
pub struct PB11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PB12MFP` reader - PB.12 Multi-function Pin Selection"]
pub struct PB12MFP_R(crate::FieldReader<u8, u8>);
impl PB12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB12MFP` writer - PB.12 Multi-function Pin Selection"]
pub struct PB12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PB13MFP` reader - PB.13 Multi-function Pin Selection"]
pub struct PB13MFP_R(crate::FieldReader<u8, u8>);
impl PB13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB13MFP` writer - PB.13 Multi-function Pin Selection"]
pub struct PB13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PB14MFP` reader - PB.14 Multi-function Pin Selection"]
pub struct PB14MFP_R(crate::FieldReader<u8, u8>);
impl PB14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB14MFP` writer - PB.14 Multi-function Pin Selection"]
pub struct PB14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PB15MFP` reader - PB.15 Multi-function Pin Selection"]
pub struct PB15MFP_R(crate::FieldReader<u8, u8>);
impl PB15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB15MFP` writer - PB.15 Multi-function Pin Selection"]
pub struct PB15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PB.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb8mfp(&self) -> PB8MFP_R {
        PB8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PB.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb9mfp(&self) -> PB9MFP_R {
        PB9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PB.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb10mfp(&self) -> PB10MFP_R {
        PB10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PB.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb11mfp(&self) -> PB11MFP_R {
        PB11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PB.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb12mfp(&self) -> PB12MFP_R {
        PB12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PB.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb13mfp(&self) -> PB13MFP_R {
        PB13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PB.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb14mfp(&self) -> PB14MFP_R {
        PB14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PB.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb15mfp(&self) -> PB15MFP_R {
        PB15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PB.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb8mfp(&mut self) -> PB8MFP_W {
        PB8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PB.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb9mfp(&mut self) -> PB9MFP_W {
        PB9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PB.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb10mfp(&mut self) -> PB10MFP_W {
        PB10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PB.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb11mfp(&mut self) -> PB11MFP_W {
        PB11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PB.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb12mfp(&mut self) -> PB12MFP_W {
        PB12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PB.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb13mfp(&mut self) -> PB13MFP_W {
        PB13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PB.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb14mfp(&mut self) -> PB14MFP_W {
        PB14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PB.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb15mfp(&mut self) -> PB15MFP_W {
        PB15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOB High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpb_mfph](index.html) module"]
pub struct SYS_GPB_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPB_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpb_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPB_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpb_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPB_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPB_MFPH to value 0"]
impl crate::Resettable for SYS_GPB_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
