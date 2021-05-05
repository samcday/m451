#[doc = "Register `SYS_GPC_MFPL` reader"]
pub struct R(crate::R<SYS_GPC_MFPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPC_MFPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_GPC_MFPL_SPEC>> for R {
    fn from(reader: crate::R<SYS_GPC_MFPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPC_MFPL` writer"]
pub struct W(crate::W<SYS_GPC_MFPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPC_MFPL_SPEC>;
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
impl core::convert::From<crate::W<SYS_GPC_MFPL_SPEC>> for W {
    fn from(writer: crate::W<SYS_GPC_MFPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC0MFP` reader - PC.0 Multi-function Pin Selection"]
pub struct PC0MFP_R(crate::FieldReader<u8, u8>);
impl PC0MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC0MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC0MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC0MFP` writer - PC.0 Multi-function Pin Selection"]
pub struct PC0MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PC1MFP` reader - PC.1 Multi-function Pin Selection"]
pub struct PC1MFP_R(crate::FieldReader<u8, u8>);
impl PC1MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC1MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC1MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC1MFP` writer - PC.1 Multi-function Pin Selection"]
pub struct PC1MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PC2MFP` reader - PC.2 Multi-function Pin Selection"]
pub struct PC2MFP_R(crate::FieldReader<u8, u8>);
impl PC2MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC2MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC2MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC2MFP` writer - PC.2 Multi-function Pin Selection"]
pub struct PC2MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PC3MFP` reader - PC.3 Multi-function Pin Selection"]
pub struct PC3MFP_R(crate::FieldReader<u8, u8>);
impl PC3MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC3MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC3MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC3MFP` writer - PC.3 Multi-function Pin Selection"]
pub struct PC3MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PC4MFP` reader - PC.4 Multi-function Pin Selection"]
pub struct PC4MFP_R(crate::FieldReader<u8, u8>);
impl PC4MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC4MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC4MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC4MFP` writer - PC.4 Multi-function Pin Selection"]
pub struct PC4MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PC5MFP` reader - PC.5 Multi-function Pin Selection"]
pub struct PC5MFP_R(crate::FieldReader<u8, u8>);
impl PC5MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC5MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC5MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC5MFP` writer - PC.5 Multi-function Pin Selection"]
pub struct PC5MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PC6MFP` reader - PC.6 Multi-function Pin Selection"]
pub struct PC6MFP_R(crate::FieldReader<u8, u8>);
impl PC6MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC6MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC6MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC6MFP` writer - PC.6 Multi-function Pin Selection"]
pub struct PC6MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PC7MFP` reader - PC.7 Multi-function Pin Selection"]
pub struct PC7MFP_R(crate::FieldReader<u8, u8>);
impl PC7MFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC7MFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC7MFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC7MFP` writer - PC.7 Multi-function Pin Selection"]
pub struct PC7MFP_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7MFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PC.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc0mfp(&self) -> PC0MFP_R {
        PC0MFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PC.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc1mfp(&self) -> PC1MFP_R {
        PC1MFP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PC.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc2mfp(&self) -> PC2MFP_R {
        PC2MFP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PC.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc3mfp(&self) -> PC3MFP_R {
        PC3MFP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PC.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc4mfp(&self) -> PC4MFP_R {
        PC4MFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PC.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc5mfp(&self) -> PC5MFP_R {
        PC5MFP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PC.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc6mfp(&self) -> PC6MFP_R {
        PC6MFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PC.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc7mfp(&self) -> PC7MFP_R {
        PC7MFP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PC.0 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc0mfp(&mut self) -> PC0MFP_W {
        PC0MFP_W { w: self }
    }
    #[doc = "Bits 4:7 - PC.1 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc1mfp(&mut self) -> PC1MFP_W {
        PC1MFP_W { w: self }
    }
    #[doc = "Bits 8:11 - PC.2 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc2mfp(&mut self) -> PC2MFP_W {
        PC2MFP_W { w: self }
    }
    #[doc = "Bits 12:15 - PC.3 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc3mfp(&mut self) -> PC3MFP_W {
        PC3MFP_W { w: self }
    }
    #[doc = "Bits 16:19 - PC.4 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc4mfp(&mut self) -> PC4MFP_W {
        PC4MFP_W { w: self }
    }
    #[doc = "Bits 20:23 - PC.5 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc5mfp(&mut self) -> PC5MFP_W {
        PC5MFP_W { w: self }
    }
    #[doc = "Bits 24:27 - PC.6 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc6mfp(&mut self) -> PC6MFP_W {
        PC6MFP_W { w: self }
    }
    #[doc = "Bits 28:31 - PC.7 Multi-function Pin Selection"]
    #[inline(always)]
    pub fn pc7mfp(&mut self) -> PC7MFP_W {
        PC7MFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOC Low Byte Multiple Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpc_mfpl](index.html) module"]
pub struct SYS_GPC_MFPL_SPEC;
impl crate::RegisterSpec for SYS_GPC_MFPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpc_mfpl::R](R) reader structure"]
impl crate::Readable for SYS_GPC_MFPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpc_mfpl::W](W) writer structure"]
impl crate::Writable for SYS_GPC_MFPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_GPC_MFPL to value 0"]
impl crate::Resettable for SYS_GPC_MFPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
