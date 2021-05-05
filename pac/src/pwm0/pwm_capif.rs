#[doc = "Register `PWM_CAPIF` reader"]
pub struct R(crate::R<PWM_CAPIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CAPIF_SPEC>> for R {
    fn from(reader: crate::R<PWM_CAPIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CAPIF` writer"]
pub struct W(crate::W<PWM_CAPIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CAPIF_SPEC>;
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
impl core::convert::From<crate::W<PWM_CAPIF_SPEC>> for W {
    fn from(writer: crate::W<PWM_CAPIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Capture Rising Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CRLIF will cleared by hardware after PDMA transfer data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRLIFN_A {
    #[doc = "0: No capture rising latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture rising latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CRLIFN_A> for u8 {
    #[inline(always)]
    fn from(variant: CRLIFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRLIFn` reader - PWM Capture Rising Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CRLIF will cleared by hardware after PDMA transfer data."]
pub struct CRLIFN_R(crate::FieldReader<u8, CRLIFN_A>);
impl CRLIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRLIFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRLIFN_A> {
        match self.bits {
            0 => Some(CRLIFN_A::_0),
            1 => Some(CRLIFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRLIFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRLIFN_A::_1
    }
}
impl core::ops::Deref for CRLIFN_R {
    type Target = crate::FieldReader<u8, CRLIFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRLIFn` writer - PWM Capture Rising Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CRLIF will cleared by hardware after PDMA transfer data."]
pub struct CRLIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLIFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRLIFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No capture rising latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLIFN_A::_0)
    }
    #[doc = "Capture rising latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLIFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "PWM Capture Falling Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CFLIF will cleared by hardware after PDMA transfer data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFLIFN_A {
    #[doc = "0: No capture falling latch condition happened"]
    _0 = 0,
    #[doc = "1: Capture falling latch condition happened, this flag will be set to high"]
    _1 = 1,
}
impl From<CFLIFN_A> for u8 {
    #[inline(always)]
    fn from(variant: CFLIFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFLIFn` reader - PWM Capture Falling Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CFLIF will cleared by hardware after PDMA transfer data."]
pub struct CFLIFN_R(crate::FieldReader<u8, CFLIFN_A>);
impl CFLIFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFLIFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFLIFN_A> {
        match self.bits {
            0 => Some(CFLIFN_A::_0),
            1 => Some(CFLIFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFLIFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFLIFN_A::_1
    }
}
impl core::ops::Deref for CFLIFN_R {
    type Target = crate::FieldReader<u8, CFLIFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFLIFn` writer - PWM Capture Falling Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CFLIF will cleared by hardware after PDMA transfer data."]
pub struct CFLIFN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLIFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFLIFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No capture falling latch condition happened"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFLIFN_A::_0)
    }
    #[doc = "Capture falling latch condition happened, this flag will be set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFLIFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Capture Rising Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CRLIF will cleared by hardware after PDMA transfer data."]
    #[inline(always)]
    pub fn crlifn(&self) -> CRLIFN_R {
        CRLIFN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - PWM Capture Falling Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CFLIF will cleared by hardware after PDMA transfer data."]
    #[inline(always)]
    pub fn cflifn(&self) -> CFLIFN_R {
        CFLIFN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Capture Rising Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CRLIF will cleared by hardware after PDMA transfer data."]
    #[inline(always)]
    pub fn crlifn(&mut self) -> CRLIFN_W {
        CRLIFN_W { w: self }
    }
    #[doc = "Bits 8:13 - PWM Capture Falling Latch Interrupt Flag\\nThis bit is writing 1 to clear. Each bit n controls the corresponding PWM channel n.\\nNote: When Capture with PDMA operating, CIFR corresponding channel CFLIF will cleared by hardware after PDMA transfer data."]
    #[inline(always)]
    pub fn cflifn(&mut self) -> CFLIFN_W {
        CFLIFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Capture Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capif](index.html) module"]
pub struct PWM_CAPIF_SPEC;
impl crate::RegisterSpec for PWM_CAPIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capif::R](R) reader structure"]
impl crate::Readable for PWM_CAPIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_capif::W](W) writer structure"]
impl crate::Writable for PWM_CAPIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CAPIF to value 0"]
impl crate::Resettable for PWM_CAPIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
