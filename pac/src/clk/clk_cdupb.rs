#[doc = "Register `CLK_CDUPB` reader"]
pub struct R(crate::R<CLK_CDUPB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CDUPB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CDUPB_SPEC>> for R {
    fn from(reader: crate::R<CLK_CDUPB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CDUPB` writer"]
pub struct W(crate::W<CLK_CDUPB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CDUPB_SPEC>;
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
impl core::convert::From<crate::W<CLK_CDUPB_SPEC>> for W {
    fn from(writer: crate::W<CLK_CDUPB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPERBD` reader - HXT Clock Frequency Detector Upper Boundary\\nThe bits define the high value of frequency monitor window.\\nWhen HXT frequency monitor value higher than this register, the HXT frequency detect fail interrupt flag will set to 1."]
pub struct UPERBD_R(crate::FieldReader<u16, u16>);
impl UPERBD_R {
    pub(crate) fn new(bits: u16) -> Self {
        UPERBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPERBD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPERBD` writer - HXT Clock Frequency Detector Upper Boundary\\nThe bits define the high value of frequency monitor window.\\nWhen HXT frequency monitor value higher than this register, the HXT frequency detect fail interrupt flag will set to 1."]
pub struct UPERBD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPERBD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - HXT Clock Frequency Detector Upper Boundary\\nThe bits define the high value of frequency monitor window.\\nWhen HXT frequency monitor value higher than this register, the HXT frequency detect fail interrupt flag will set to 1."]
    #[inline(always)]
    pub fn uperbd(&self) -> UPERBD_R {
        UPERBD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HXT Clock Frequency Detector Upper Boundary\\nThe bits define the high value of frequency monitor window.\\nWhen HXT frequency monitor value higher than this register, the HXT frequency detect fail interrupt flag will set to 1."]
    #[inline(always)]
    pub fn uperbd(&mut self) -> UPERBD_W {
        UPERBD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Frequency Detector Upper Boundary Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cdupb](index.html) module"]
pub struct CLK_CDUPB_SPEC;
impl crate::RegisterSpec for CLK_CDUPB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cdupb::R](R) reader structure"]
impl crate::Readable for CLK_CDUPB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cdupb::W](W) writer structure"]
impl crate::Writable for CLK_CDUPB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CDUPB to value 0"]
impl crate::Resettable for CLK_CDUPB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
