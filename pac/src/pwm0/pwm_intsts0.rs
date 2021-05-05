#[doc = "Register `PWM_INTSTS0` reader"]
pub struct R(crate::R<PWM_INTSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INTSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_INTSTS0_SPEC>> for R {
    fn from(reader: crate::R<PWM_INTSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_INTSTS0` writer"]
pub struct W(crate::W<PWM_INTSTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INTSTS0_SPEC>;
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
impl core::convert::From<crate::W<PWM_INTSTS0_SPEC>> for W {
    fn from(writer: crate::W<PWM_INTSTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZIFn` reader - PWM Zero Point Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nThis bit is set by hardware when PWM counter reaches zero, software can write 1 to clear this bit to zero."]
pub struct ZIFN_R(crate::FieldReader<u8, u8>);
impl ZIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZIFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZIFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIFn` writer - PWM Zero Point Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nThis bit is set by hardware when PWM counter reaches zero, software can write 1 to clear this bit to zero."]
pub struct ZIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `IFAIF0_1` reader - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL0_1 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF0_1_R(crate::FieldReader<bool, bool>);
impl IFAIF0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIF0_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFAIF0_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIF0_1` writer - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL0_1 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIF0_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PIFn` reader - PWM Period Point Interrupt Flag\\nThis bit is set by hardware when PWM counter reaches PWM_PERIODn, software can write 1 to clear this bit to zero. Each bit n controls the corresponding PWM channel n."]
pub struct PIFN_R(crate::FieldReader<u8, u8>);
impl PIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIFn` writer - PWM Period Point Interrupt Flag\\nThis bit is set by hardware when PWM counter reaches PWM_PERIODn, software can write 1 to clear this bit to zero. Each bit n controls the corresponding PWM channel n."]
pub struct PIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `IFAIF2_3` reader - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL2_3 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF2_3_R(crate::FieldReader<bool, bool>);
impl IFAIF2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIF2_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFAIF2_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIF2_3` writer - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL2_3 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIF2_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CMPUIFn` reader - PWM Compare Up Count Interrupt Flag\\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding PWM channel n.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection.\\nNote2: In complementary mode, CMPUIF1, 3, 5 use as another CMPUIF for channel 0, 2, 4."]
pub struct CMPUIFN_R(crate::FieldReader<u8, u8>);
impl CMPUIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUIFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPUIFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIFn` writer - PWM Compare Up Count Interrupt Flag\\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding PWM channel n.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection.\\nNote2: In complementary mode, CMPUIF1, 3, 5 use as another CMPUIF for channel 0, 2, 4."]
pub struct CMPUIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `IFAIF4_5` reader - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL4_5 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF4_5_R(crate::FieldReader<bool, bool>);
impl IFAIF4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIF4_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFAIF4_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIF4_5` writer - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL4_5 in PWM_IFA register, software can clear this bit by writing 1 to it."]
pub struct IFAIF4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIF4_5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CMPDIFn` reader - PWM Compare Down Count Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection.\\nNote2: In complementary mode, CMPDIF1, 3, 5 use as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIFN_R(crate::FieldReader<u8, u8>);
impl CMPDIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDIFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDIFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIFn` writer - PWM Compare Down Count Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection.\\nNote2: In complementary mode, CMPDIF1, 3, 5 use as another CMPDIF for channel 0, 2, 4."]
pub struct CMPDIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Zero Point Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nThis bit is set by hardware when PWM counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn zifn(&self) -> ZIFN_R {
        ZIFN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL0_1 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif0_1(&self) -> IFAIF0_1_R {
        IFAIF0_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - PWM Period Point Interrupt Flag\\nThis bit is set by hardware when PWM counter reaches PWM_PERIODn, software can write 1 to clear this bit to zero. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pifn(&self) -> PIFN_R {
        PIFN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL2_3 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif2_3(&self) -> IFAIF2_3_R {
        IFAIF2_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Flag\\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding PWM channel n.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection.\\nNote2: In complementary mode, CMPUIF1, 3, 5 use as another CMPUIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuifn(&self) -> CMPUIFN_R {
        CMPUIFN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL4_5 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif4_5(&self) -> IFAIF4_5_R {
        IFAIF4_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection.\\nNote2: In complementary mode, CMPDIF1, 3, 5 use as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdifn(&self) -> CMPDIFN_R {
        CMPDIFN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Zero Point Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nThis bit is set by hardware when PWM counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn zifn(&mut self) -> ZIFN_W {
        ZIFN_W { w: self }
    }
    #[doc = "Bit 7 - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL0_1 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif0_1(&mut self) -> IFAIF0_1_W {
        IFAIF0_1_W { w: self }
    }
    #[doc = "Bits 8:13 - PWM Period Point Interrupt Flag\\nThis bit is set by hardware when PWM counter reaches PWM_PERIODn, software can write 1 to clear this bit to zero. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pifn(&mut self) -> PIFN_W {
        PIFN_W { w: self }
    }
    #[doc = "Bit 15 - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL2_3 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif2_3(&mut self) -> IFAIF2_3_W {
        IFAIF2_3_W { w: self }
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Flag\\nFlag is set by hardware when PWM counter up count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it. Each bit n controls the corresponding PWM channel n.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in up counter type selection.\\nNote2: In complementary mode, CMPUIF1, 3, 5 use as another CMPUIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuifn(&mut self) -> CMPUIFN_W {
        CMPUIFN_W { w: self }
    }
    #[doc = "Bit 23 - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Flag\\nFlag is set by hardware when condition match IFSEL4_5 in PWM_IFA register, software can clear this bit by writing 1 to it."]
    #[inline(always)]
    pub fn ifaif4_5(&mut self) -> IFAIF4_5_W {
        IFAIF4_5_W { w: self }
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Interrupt Flag\\nEach bit n controls the corresponding PWM channel n.\\nFlag is set by hardware when PWM counter down count and reaches PWM_CMPDATn, software can clear this bit by writing 1 to it.\\nNote1: If CMPDAT equal to PERIOD, this flag is not working in down counter type selection.\\nNote2: In complementary mode, CMPDIF1, 3, 5 use as another CMPDIF for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdifn(&mut self) -> CMPDIFN_W {
        CMPDIFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Flag Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_intsts0](index.html) module"]
pub struct PWM_INTSTS0_SPEC;
impl crate::RegisterSpec for PWM_INTSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_intsts0::R](R) reader structure"]
impl crate::Readable for PWM_INTSTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_intsts0::W](W) writer structure"]
impl crate::Writable for PWM_INTSTS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_INTSTS0 to value 0"]
impl crate::Resettable for PWM_INTSTS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
