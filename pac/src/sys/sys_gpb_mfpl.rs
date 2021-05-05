#[doc = "Register `SYS_GPB_MFPL` reader"]
pub struct R(crate::R<SYS_GPB_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPB_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPB_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPB_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPB_MFPL` writer"]
pub struct W(crate::W<SYS_GPB_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPB_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPB_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPB_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB0MFP` reader - PB.0 Multi-function Pin Selection"]
pub struct PB0MFP_R(crate::FieldReader<u8, u8>);
impl PB0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB0MFP` writer - PB.0 Multi-function Pin Selection"]
pub struct PB0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PB1MFP` reader - PB.1 Multi-function Pin Selection"]
pub struct PB1MFP_R(crate::FieldReader<u8, u8>);
impl PB1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB1MFP` writer - PB.1 Multi-function Pin Selection"]
pub struct PB1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PB2MFP` reader - PB.2 Multi-function Pin Selection"]
pub struct PB2MFP_R(crate::FieldReader<u8, u8>);
impl PB2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB2MFP` writer - PB.2 Multi-function Pin Selection"]
pub struct PB2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PB3MFP` reader - PB.3 Multi-function Pin Selection"]
pub struct PB3MFP_R(crate::FieldReader<u8, u8>);
impl PB3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB3MFP` writer - PB.3 Multi-function Pin Selection"]
pub struct PB3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PB4MFP` reader - PB.4 Multi-function Pin Selection"]
pub struct PB4MFP_R(crate::FieldReader<u8, u8>);
impl PB4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB4MFP` writer - PB.4 Multi-function Pin Selection"]
pub struct PB4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PB5MFP` reader - PB.5 Multi-function Pin Selection"]
pub struct PB5MFP_R(crate::FieldReader<u8, u8>);
impl PB5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB5MFP` writer - PB.5 Multi-function Pin Selection"]
pub struct PB5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PB6MFP` reader - PB.6 Multi-function Pin Selection"]
pub struct PB6MFP_R(crate::FieldReader<u8, u8>);
impl PB6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB6MFP` writer - PB.6 Multi-function Pin Selection"]
pub struct PB6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PB7MFP` reader - PB.7 Multi-function Pin Selection"]
pub struct PB7MFP_R(crate::FieldReader<u8, u8>);
impl PB7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PB7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB7MFP` writer - PB.7 Multi-function Pin Selection"]
pub struct PB7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PB.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb0mfp(&self) -> PB0MFP_R {
        PB0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PB.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb1mfp(&self) -> PB1MFP_R {
        PB1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PB.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb2mfp(&self) -> PB2MFP_R {
        PB2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PB.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb3mfp(&self) -> PB3MFP_R {
        PB3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PB.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb4mfp(&self) -> PB4MFP_R {
        PB4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PB.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb5mfp(&self) -> PB5MFP_R {
        PB5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PB.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb6mfp(&self) -> PB6MFP_R {
        PB6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PB.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb7mfp(&self) -> PB7MFP_R {
        PB7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PB.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb0mfp(&mut self) -> PB0MFP_W {
        PB0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PB.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb1mfp(&mut self) -> PB1MFP_W {
        PB1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PB.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb2mfp(&mut self) -> PB2MFP_W {
        PB2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PB.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb3mfp(&mut self) -> PB3MFP_W {
        PB3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PB.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb4mfp(&mut self) -> PB4MFP_W {
        PB4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PB.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb5mfp(&mut self) -> PB5MFP_W {
        PB5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PB.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb6mfp(&mut self) -> PB6MFP_W {
        PB6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PB.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pb7mfp(&mut self) -> PB7MFP_W {
        PB7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOB Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpb_mfpl](index.html) module"]
pub struct SYS_GPB_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPB_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpb_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPB_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpb_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPB_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPB_MFPL to value 0"]
impl crate::Resettable for SYS_GPB_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
