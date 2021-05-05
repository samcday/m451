#[doc = "Register `SYS_GPC_MFPH` reader"]
pub struct R(crate::R<SYS_GPC_MFPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPC_MFPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPC_MFPH_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPC_MFPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPC_MFPH` writer"]
pub struct W(crate::W<SYS_GPC_MFPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPC_MFPH_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPC_MFPH_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPC_MFPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC8MFP` reader - PC.8 Multi-function Pin Selection"]
pub struct PC8MFP_R(crate::FieldReader<u8, u8>);
impl PC8MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC8MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC8MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC8MFP` writer - PC.8 Multi-function Pin Selection"]
pub struct PC8MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PC9MFP` reader - PC.9 Multi-function Pin Selection"]
pub struct PC9MFP_R(crate::FieldReader<u8, u8>);
impl PC9MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC9MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC9MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC9MFP` writer - PC.9 Multi-function Pin Selection"]
pub struct PC9MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PC10MFP` reader - PC.10 Multi-function Pin Selection"]
pub struct PC10MFP_R(crate::FieldReader<u8, u8>);
impl PC10MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC10MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC10MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC10MFP` writer - PC.10 Multi-function Pin Selection"]
pub struct PC10MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PC11MFP` reader - PC.11 Multi-function Pin Selection"]
pub struct PC11MFP_R(crate::FieldReader<u8, u8>);
impl PC11MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC11MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC11MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC11MFP` writer - PC.11 Multi-function Pin Selection"]
pub struct PC11MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PC12MFP` reader - PC.12 Multi-function Pin Selection"]
pub struct PC12MFP_R(crate::FieldReader<u8, u8>);
impl PC12MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC12MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC12MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC12MFP` writer - PC.12 Multi-function Pin Selection"]
pub struct PC12MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC12MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PC13MFP` reader - PC.13 Multi-function Pin Selection"]
pub struct PC13MFP_R(crate::FieldReader<u8, u8>);
impl PC13MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC13MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC13MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC13MFP` writer - PC.13 Multi-function Pin Selection"]
pub struct PC13MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PC14MFP` reader - PC.14 Multi-function Pin Selection"]
pub struct PC14MFP_R(crate::FieldReader<u8, u8>);
impl PC14MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC14MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC14MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC14MFP` writer - PC.14 Multi-function Pin Selection"]
pub struct PC14MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PC15MFP` reader - PC.15 Multi-function Pin Selection"]
pub struct PC15MFP_R(crate::FieldReader<u8, u8>);
impl PC15MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC15MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC15MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC15MFP` writer - PC.15 Multi-function Pin Selection"]
pub struct PC15MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PC.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc8mfp(&self) -> PC8MFP_R {
        PC8MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PC.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc9mfp(&self) -> PC9MFP_R {
        PC9MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PC.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc10mfp(&self) -> PC10MFP_R {
        PC10MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PC.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc11mfp(&self) -> PC11MFP_R {
        PC11MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PC.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc12mfp(&self) -> PC12MFP_R {
        PC12MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PC.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc13mfp(&self) -> PC13MFP_R {
        PC13MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PC.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc14mfp(&self) -> PC14MFP_R {
        PC14MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PC.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc15mfp(&self) -> PC15MFP_R {
        PC15MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PC.8 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc8mfp(&mut self) -> PC8MFP_W {
        PC8MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PC.9 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc9mfp(&mut self) -> PC9MFP_W {
        PC9MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PC.10 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc10mfp(&mut self) -> PC10MFP_W {
        PC10MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PC.11 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc11mfp(&mut self) -> PC11MFP_W {
        PC11MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PC.12 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc12mfp(&mut self) -> PC12MFP_W {
        PC12MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PC.13 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc13mfp(&mut self) -> PC13MFP_W {
        PC13MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PC.14 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc14mfp(&mut self) -> PC14MFP_W {
        PC14MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PC.15 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc15mfp(&mut self) -> PC15MFP_W {
        PC15MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOC High Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpc_mfph](index.html) module"]
pub struct SYS_GPC_MFPH_SPEC;
impl crate::RegisterSpec for SYS_GPC_MFPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpc_mfph::R](R) reader structure"]
impl crate::Readable for SYS_GPC_MFPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpc_mfph::W](W) writer structure"]
impl crate::Writable for SYS_GPC_MFPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPC_MFPH to value 0"]
impl crate::Resettable for SYS_GPC_MFPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
