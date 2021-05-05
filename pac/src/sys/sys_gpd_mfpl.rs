#[doc = "Register `SYS_GPD_MFPL` reader"]
pub struct R(crate::R<SYS_GPD_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPD_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPD_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPD_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPD_MFPL` writer"]
pub struct W(crate::W<SYS_GPD_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPD_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPD_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPD_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0MFP` reader - PD.0 Multi-function Pin Selection"]
pub struct PD0MFP_R(crate::FieldReader<u8, u8>);
impl PD0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD0MFP` writer - PD.0 Multi-function Pin Selection"]
pub struct PD0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PD1MFP` reader - PD.1 Multi-function Pin Selection"]
pub struct PD1MFP_R(crate::FieldReader<u8, u8>);
impl PD1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD1MFP` writer - PD.1 Multi-function Pin Selection"]
pub struct PD1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PD2MFP` reader - PD.2 Multi-function Pin Selection"]
pub struct PD2MFP_R(crate::FieldReader<u8, u8>);
impl PD2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD2MFP` writer - PD.2 Multi-function Pin Selection"]
pub struct PD2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PD3MFP` reader - PD.3 Multi-function Pin Selection"]
pub struct PD3MFP_R(crate::FieldReader<u8, u8>);
impl PD3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD3MFP` writer - PD.3 Multi-function Pin Selection"]
pub struct PD3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PD4MFP` reader - PD.4 Multi-function Pin Selection"]
pub struct PD4MFP_R(crate::FieldReader<u8, u8>);
impl PD4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD4MFP` writer - PD.4 Multi-function Pin Selection"]
pub struct PD4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PD5MFP` reader - PD.5 Multi-function Pin Selection"]
pub struct PD5MFP_R(crate::FieldReader<u8, u8>);
impl PD5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD5MFP` writer - PD.5 Multi-function Pin Selection"]
pub struct PD5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PD6MFP` reader - PD.6 Multi-function Pin Selection"]
pub struct PD6MFP_R(crate::FieldReader<u8, u8>);
impl PD6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD6MFP` writer - PD.6 Multi-function Pin Selection"]
pub struct PD6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PD7MFP` reader - PD.7 Multi-function Pin Selection"]
pub struct PD7MFP_R(crate::FieldReader<u8, u8>);
impl PD7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD7MFP` writer - PD.7 Multi-function Pin Selection"]
pub struct PD7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PD.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd0mfp(&self) -> PD0MFP_R {
        PD0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PD.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd1mfp(&self) -> PD1MFP_R {
        PD1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PD.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd2mfp(&self) -> PD2MFP_R {
        PD2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PD.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd3mfp(&self) -> PD3MFP_R {
        PD3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PD.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd4mfp(&self) -> PD4MFP_R {
        PD4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PD.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd5mfp(&self) -> PD5MFP_R {
        PD5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PD.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd6mfp(&self) -> PD6MFP_R {
        PD6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PD.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd7mfp(&self) -> PD7MFP_R {
        PD7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PD.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd0mfp(&mut self) -> PD0MFP_W {
        PD0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PD.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd1mfp(&mut self) -> PD1MFP_W {
        PD1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PD.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd2mfp(&mut self) -> PD2MFP_W {
        PD2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PD.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd3mfp(&mut self) -> PD3MFP_W {
        PD3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PD.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd4mfp(&mut self) -> PD4MFP_W {
        PD4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PD.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd5mfp(&mut self) -> PD5MFP_W {
        PD5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PD.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd6mfp(&mut self) -> PD6MFP_W {
        PD6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PD.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pd7mfp(&mut self) -> PD7MFP_W {
        PD7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpd_mfpl](index.html) module"]
pub struct SYS_GPD_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPD_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpd_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPD_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpd_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPD_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPD_MFPL to value 0"]
impl crate::Resettable for SYS_GPD_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
