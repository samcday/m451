#[doc = "Register `SYS_GPE_MFPL` reader"]
pub struct R(crate::R<SYS_GPE_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPE_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPE_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPE_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPE_MFPL` writer"]
pub struct W(crate::W<SYS_GPE_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPE_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPE_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPE_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE0MFP` reader - PE.0 Multi-function Pin Selection"]
pub struct PE0MFP_R(crate::FieldReader<u8, u8>);
impl PE0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE0MFP` writer - PE.0 Multi-function Pin Selection"]
pub struct PE0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PE1MFP` reader - PE.1 Multi-function Pin Selection"]
pub struct PE1MFP_R(crate::FieldReader<u8, u8>);
impl PE1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE1MFP` writer - PE.1 Multi-function Pin Selection"]
pub struct PE1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PE2MFP` reader - PE.2 Multi-function Pin Selection"]
pub struct PE2MFP_R(crate::FieldReader<u8, u8>);
impl PE2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE2MFP` writer - PE.2 Multi-function Pin Selection"]
pub struct PE2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PE3MFP` reader - PE.3 Multi-function Pin Selection"]
pub struct PE3MFP_R(crate::FieldReader<u8, u8>);
impl PE3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE3MFP` writer - PE.3 Multi-function Pin Selection"]
pub struct PE3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PE4MFP` reader - PE.4 Multi-function Pin Selection"]
pub struct PE4MFP_R(crate::FieldReader<u8, u8>);
impl PE4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE4MFP` writer - PE.4 Multi-function Pin Selection"]
pub struct PE4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PE5MFP` reader - PE.5 Multi-function Pin Selection"]
pub struct PE5MFP_R(crate::FieldReader<u8, u8>);
impl PE5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE5MFP` writer - PE.5 Multi-function Pin Selection"]
pub struct PE5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PE6MFP` reader - PE.6 Multi-function Pin Selection"]
pub struct PE6MFP_R(crate::FieldReader<u8, u8>);
impl PE6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE6MFP` writer - PE.6 Multi-function Pin Selection"]
pub struct PE6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PE7MFP` reader - PE.7 Multi-function Pin Selection"]
pub struct PE7MFP_R(crate::FieldReader<u8, u8>);
impl PE7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PE7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE7MFP` writer - PE.7 Multi-function Pin Selection"]
pub struct PE7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PE7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PE.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe0mfp(&self) -> PE0MFP_R {
        PE0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe1mfp(&self) -> PE1MFP_R {
        PE1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PE.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe2mfp(&self) -> PE2MFP_R {
        PE2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PE.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe3mfp(&self) -> PE3MFP_R {
        PE3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PE.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe4mfp(&self) -> PE4MFP_R {
        PE4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PE.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe5mfp(&self) -> PE5MFP_R {
        PE5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PE.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe6mfp(&self) -> PE6MFP_R {
        PE6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PE.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe7mfp(&self) -> PE7MFP_R {
        PE7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe0mfp(&mut self) -> PE0MFP_W {
        PE0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PE.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe1mfp(&mut self) -> PE1MFP_W {
        PE1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PE.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe2mfp(&mut self) -> PE2MFP_W {
        PE2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PE.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe3mfp(&mut self) -> PE3MFP_W {
        PE3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PE.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe4mfp(&mut self) -> PE4MFP_W {
        PE4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PE.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe5mfp(&mut self) -> PE5MFP_W {
        PE5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PE.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe6mfp(&mut self) -> PE6MFP_W {
        PE6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PE.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pe7mfp(&mut self) -> PE7MFP_W {
        PE7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpe_mfpl](index.html) module"]
pub struct SYS_GPE_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPE_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpe_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPE_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpe_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPE_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPE_MFPL to value 0"]
impl crate::Resettable for SYS_GPE_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
