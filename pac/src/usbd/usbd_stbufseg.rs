#[doc = "Register `USBD_STBUFSEG` reader"]
pub struct R(crate::R<USBD_STBUFSEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_STBUFSEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBD_STBUFSEG_SPEC>> for R {
    fn from(reader: crate::R<USBD_STBUFSEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_STBUFSEG` writer"]
pub struct W(crate::W<USBD_STBUFSEG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_STBUFSEG_SPEC>;
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
impl core::convert::From<crate::W<USBD_STBUFSEG_SPEC>> for W {
    fn from(writer: crate::W<USBD_STBUFSEG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STBUFSEG` reader - SETUP Token Buffer Segmentation\\nIt is used to indicate the offset address for the SETUP token with the USB Device SRAM starting address The effective starting address is\\nUSBD_SRAM address + {STBUFSEG, 3'b000} \\nNote: It is used for SETUP token only."]
pub struct STBUFSEG_R(crate::FieldReader<u8, u8>);
impl STBUFSEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        STBUFSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STBUFSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBUFSEG` writer - SETUP Token Buffer Segmentation\\nIt is used to indicate the offset address for the SETUP token with the USB Device SRAM starting address The effective starting address is\\nUSBD_SRAM address + {STBUFSEG, 3'b000} \\nNote: It is used for SETUP token only."]
pub struct STBUFSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> STBUFSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | ((value as u32 & 0x3f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - SETUP Token Buffer Segmentation\\nIt is used to indicate the offset address for the SETUP token with the USB Device SRAM starting address The effective starting address is\\nUSBD_SRAM address + {STBUFSEG, 3'b000} \\nNote: It is used for SETUP token only."]
    #[inline(always)]
    pub fn stbufseg(&self) -> STBUFSEG_R {
        STBUFSEG_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:8 - SETUP Token Buffer Segmentation\\nIt is used to indicate the offset address for the SETUP token with the USB Device SRAM starting address The effective starting address is\\nUSBD_SRAM address + {STBUFSEG, 3'b000} \\nNote: It is used for SETUP token only."]
    #[inline(always)]
    pub fn stbufseg(&mut self) -> STBUFSEG_W {
        STBUFSEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Setup Token Buffer Segmentation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_stbufseg](index.html) module"]
pub struct USBD_STBUFSEG_SPEC;
impl crate::RegisterSpec for USBD_STBUFSEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_stbufseg::R](R) reader structure"]
impl crate::Readable for USBD_STBUFSEG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_stbufseg::W](W) writer structure"]
impl crate::Writable for USBD_STBUFSEG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_STBUFSEG to value 0"]
impl crate::Resettable for USBD_STBUFSEG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
