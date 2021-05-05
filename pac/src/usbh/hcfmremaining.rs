#[doc = "Register `HCFMREMAINING` reader"]
pub struct R(crate::R<HCFMREMAINING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMREMAINING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCFMREMAINING_SPEC>> for R {
    fn from(reader: crate::R<HCFMREMAINING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FR` reader - Frame Remaining\\nWhen the Host Controller is in the USBOPERATIONAL state, this 14-bit field decrements each 12 MHz clock period. When the count reaches 0, (end of frame) the counter reloads with Frame Interval. In addition, the counter loads when the Host Controller transitions into USBOPERATIONAL."]
pub struct FR_R(crate::FieldReader<u16, u16>);
impl FR_R {
    pub(crate) fn new(bits: u16) -> Self {
        FR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRT` reader - Frame Remaining Toggle\\nThis bit is loaded from the FIT (HcFmInterval\\[31\\]) whenever FR (HcFmRemaining\\[13:0\\]) reaches 0."]
pub struct FRT_R(crate::FieldReader<bool, bool>);
impl FRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Remaining\\nWhen the Host Controller is in the USBOPERATIONAL state, this 14-bit field decrements each 12 MHz clock period. When the count reaches 0, (end of frame) the counter reloads with Frame Interval. In addition, the counter loads when the Host Controller transitions into USBOPERATIONAL."]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Frame Remaining Toggle\\nThis bit is loaded from the FIT (HcFmInterval\\[31\\]) whenever FR (HcFmRemaining\\[13:0\\]) reaches 0."]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Host Controller Frame Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmremaining](index.html) module"]
pub struct HCFMREMAINING_SPEC;
impl crate::RegisterSpec for HCFMREMAINING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfmremaining::R](R) reader structure"]
impl crate::Readable for HCFMREMAINING_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCFMREMAINING to value 0"]
impl crate::Resettable for HCFMREMAINING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
