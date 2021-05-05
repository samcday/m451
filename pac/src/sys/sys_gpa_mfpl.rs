#[doc = "Register `SYS_GPA_MFPL` reader"]
pub struct R(crate::R<SYS_GPA_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPA_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPA_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPA_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPA_MFPL` writer"]
pub struct W(crate::W<SYS_GPA_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPA_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPA_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPA_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA0MFP` reader - PA.0 Multi-function Pin Selection"]
pub struct PA0MFP_R(crate::FieldReader<u8, u8>);
impl PA0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA0MFP` writer - PA.0 Multi-function Pin Selection"]
pub struct PA0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PA1MFP` reader - PA.1 Multi-function Pin Selection"]
pub struct PA1MFP_R(crate::FieldReader<u8, u8>);
impl PA1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA1MFP` writer - PA.1 Multi-function Pin Selection"]
pub struct PA1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PA2MFP` reader - PA.2 Multi-function Pin Selection"]
pub struct PA2MFP_R(crate::FieldReader<u8, u8>);
impl PA2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA2MFP` writer - PA.2 Multi-function Pin Selection"]
pub struct PA2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PA3MFP` reader - PA.3 Multi-function Pin Selection"]
pub struct PA3MFP_R(crate::FieldReader<u8, u8>);
impl PA3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA3MFP` writer - PA.3 Multi-function Pin Selection"]
pub struct PA3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PA4MFP` reader - PA.4 Multi-function Pin Selection"]
pub struct PA4MFP_R(crate::FieldReader<u8, u8>);
impl PA4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA4MFP` writer - PA.4 Multi-function Pin Selection"]
pub struct PA4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PA5MFP` reader - PA.5 Multi-function Pin Selection"]
pub struct PA5MFP_R(crate::FieldReader<u8, u8>);
impl PA5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA5MFP` writer - PA.5 Multi-function Pin Selection"]
pub struct PA5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PA6MFP` reader - PA.6 Multi-function Pin Selection"]
pub struct PA6MFP_R(crate::FieldReader<u8, u8>);
impl PA6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA6MFP` writer - PA.6 Multi-function Pin Selection"]
pub struct PA6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PA7MFP` reader - PA.7 Multi-function Pin Selection"]
pub struct PA7MFP_R(crate::FieldReader<u8, u8>);
impl PA7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA7MFP` writer - PA.7 Multi-function Pin Selection"]
pub struct PA7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PA.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa0mfp(&self) -> PA0MFP_R {
        PA0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PA.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa1mfp(&self) -> PA1MFP_R {
        PA1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PA.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa2mfp(&self) -> PA2MFP_R {
        PA2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PA.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa3mfp(&self) -> PA3MFP_R {
        PA3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PA.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa4mfp(&self) -> PA4MFP_R {
        PA4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PA.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa5mfp(&self) -> PA5MFP_R {
        PA5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PA.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa6mfp(&self) -> PA6MFP_R {
        PA6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PA.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa7mfp(&self) -> PA7MFP_R {
        PA7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PA.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa0mfp(&mut self) -> PA0MFP_W {
        PA0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PA.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa1mfp(&mut self) -> PA1MFP_W {
        PA1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PA.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa2mfp(&mut self) -> PA2MFP_W {
        PA2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PA.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa3mfp(&mut self) -> PA3MFP_W {
        PA3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PA.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa4mfp(&mut self) -> PA4MFP_W {
        PA4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PA.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa5mfp(&mut self) -> PA5MFP_W {
        PA5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PA.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa6mfp(&mut self) -> PA6MFP_W {
        PA6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PA.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pa7mfp(&mut self) -> PA7MFP_W {
        PA7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOA Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpa_mfpl](index.html) module"]
pub struct SYS_GPA_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPA_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpa_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPA_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpa_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPA_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPA_MFPL to value 0"]
impl crate::Resettable for SYS_GPA_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
