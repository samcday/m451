#[doc = "Register `CLK_CDLOWB` reader"]
pub struct R(crate::R<CLK_CDLOWB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CDLOWB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CDLOWB_SPEC>> for R {
    fn from(reader: crate::R<CLK_CDLOWB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CDLOWB` writer"]
pub struct W(crate::W<CLK_CDLOWB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CDLOWB_SPEC>;
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
impl core::convert::From<crate::W<CLK_CDLOWB_SPEC>> for W {
    fn from(writer: crate::W<CLK_CDLOWB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWERBD` reader - HXT Clock Frequency Detector Lower Boundary\\nThe bits define the low value of frequency monitor window.\\nWhen HXT frequency monitor value lower than this register, the HXT frequency detect fail interrupt flag will set to 1."]
pub struct LOWERBD_R(crate::FieldReader<u16, u16>);
impl LOWERBD_R {
    pub(crate) fn new(bits: u16) -> Self {
        LOWERBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWERBD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWERBD` writer - HXT Clock Frequency Detector Lower Boundary\\nThe bits define the low value of frequency monitor window.\\nWhen HXT frequency monitor value lower than this register, the HXT frequency detect fail interrupt flag will set to 1."]
pub struct LOWERBD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWERBD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - HXT Clock Frequency Detector Lower Boundary\\nThe bits define the low value of frequency monitor window.\\nWhen HXT frequency monitor value lower than this register, the HXT frequency detect fail interrupt flag will set to 1."]
    #[inline(always)]
    pub fn lowerbd(&self) -> LOWERBD_R {
        LOWERBD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HXT Clock Frequency Detector Lower Boundary\\nThe bits define the low value of frequency monitor window.\\nWhen HXT frequency monitor value lower than this register, the HXT frequency detect fail interrupt flag will set to 1."]
    #[inline(always)]
    pub fn lowerbd(&mut self) -> LOWERBD_W {
        LOWERBD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Frequency Detector Lower Boundary Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cdlowb](index.html) module"]
pub struct CLK_CDLOWB_SPEC;
impl crate::RegisterSpec for CLK_CDLOWB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cdlowb::R](R) reader structure"]
impl crate::Readable for CLK_CDLOWB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cdlowb::W](W) writer structure"]
impl crate::Writable for CLK_CDLOWB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CDLOWB to value 0"]
impl crate::Resettable for CLK_CDLOWB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
