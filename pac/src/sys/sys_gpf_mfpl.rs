#[doc = "Register `SYS_GPF_MFPL` reader"]
pub struct R(crate::R<SYS_GPF_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPF_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPF_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPF_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPF_MFPL` writer"]
pub struct W(crate::W<SYS_GPF_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPF_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPF_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPF_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF0MFP` reader - PF.0 Multi-function Pin Selection"]
pub struct PF0MFP_R(crate::FieldReader<u8, u8>);
impl PF0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF0MFP` writer - PF.0 Multi-function Pin Selection"]
pub struct PF0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PF1MFP` reader - PF.1 Multi-function Pin Selection"]
pub struct PF1MFP_R(crate::FieldReader<u8, u8>);
impl PF1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF1MFP` writer - PF.1 Multi-function Pin Selection"]
pub struct PF1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PF2MFP` reader - PF.2 Multi-function Pin Selection"]
pub struct PF2MFP_R(crate::FieldReader<u8, u8>);
impl PF2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF2MFP` writer - PF.2 Multi-function Pin Selection"]
pub struct PF2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PF3MFP` reader - PF.3 Multi-function Pin Selection"]
pub struct PF3MFP_R(crate::FieldReader<u8, u8>);
impl PF3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF3MFP` writer - PF.3 Multi-function Pin Selection"]
pub struct PF3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PF4MFP` reader - PF.4 Multi-function Pin Selection"]
pub struct PF4MFP_R(crate::FieldReader<u8, u8>);
impl PF4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF4MFP` writer - PF.4 Multi-function Pin Selection"]
pub struct PF4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PF5MFP` reader - PF.5 Multi-function Pin Selection"]
pub struct PF5MFP_R(crate::FieldReader<u8, u8>);
impl PF5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF5MFP` writer - PF.5 Multi-function Pin Selection"]
pub struct PF5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PF6MFP` reader - PF.6 Multi-function Pin Selection"]
pub struct PF6MFP_R(crate::FieldReader<u8, u8>);
impl PF6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF6MFP` writer - PF.6 Multi-function Pin Selection"]
pub struct PF6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PF7MFP` reader - PF.7 Multi-function Pin Selection"]
pub struct PF7MFP_R(crate::FieldReader<u8, u8>);
impl PF7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF7MFP` writer - PF.7 Multi-function Pin Selection"]
pub struct PF7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PF.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf0mfp(&self) -> PF0MFP_R {
        PF0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PF.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf1mfp(&self) -> PF1MFP_R {
        PF1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PF.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf2mfp(&self) -> PF2MFP_R {
        PF2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PF.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf3mfp(&self) -> PF3MFP_R {
        PF3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PF.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf4mfp(&self) -> PF4MFP_R {
        PF4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PF.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf5mfp(&self) -> PF5MFP_R {
        PF5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PF.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf6mfp(&self) -> PF6MFP_R {
        PF6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PF.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf7mfp(&self) -> PF7MFP_R {
        PF7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PF.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf0mfp(&mut self) -> PF0MFP_W {
        PF0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PF.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf1mfp(&mut self) -> PF1MFP_W {
        PF1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PF.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf2mfp(&mut self) -> PF2MFP_W {
        PF2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PF.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf3mfp(&mut self) -> PF3MFP_W {
        PF3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PF.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf4mfp(&mut self) -> PF4MFP_W {
        PF4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PF.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf5mfp(&mut self) -> PF5MFP_W {
        PF5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PF.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf6mfp(&mut self) -> PF6MFP_W {
        PF6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PF.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pf7mfp(&mut self) -> PF7MFP_W {
        PF7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOF Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpf_mfpl](index.html) module"]
pub struct SYS_GPF_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPF_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpf_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPF_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpf_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPF_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPF_MFPL to value 0"]
impl crate::Resettable for SYS_GPF_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
