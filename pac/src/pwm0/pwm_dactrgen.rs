#[doc = "Register `PWM_DACTRGEN` reader"]
pub struct R(crate::R<PWM_DACTRGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_DACTRGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_DACTRGEN_SPEC>> for R {
    fn from(reader: crate::R<PWM_DACTRGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_DACTRGEN` writer"]
pub struct W(crate::W<PWM_DACTRGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DACTRGEN_SPEC>;
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
impl core::convert::From<crate::W<PWM_DACTRGEN_SPEC>> for W {
    fn from(writer: crate::W<PWM_DACTRGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Zero Point Trigger DAC Enable Bit\\nPWM can trigger EADC/DAC/DMA to start action when PWM counter down count to zero if this bit is set to1. Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZTEN_A {
    #[doc = "0: PWM period point trigger DAC function Disabled"]
    _0 = 0,
    #[doc = "1: PWM period point trigger DAC function Enabled"]
    _1 = 1,
}
impl From<ZTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ZTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZTEn` reader - PWM Zero Point Trigger DAC Enable Bit\\nPWM can trigger EADC/DAC/DMA to start action when PWM counter down count to zero if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
pub struct ZTEN_R(crate::FieldReader<u8, ZTEN_A>);
impl ZTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ZTEN_A> {
        match self.bits {
            0 => Some(ZTEN_A::_0),
            1 => Some(ZTEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZTEN_A::_1
    }
}
impl core::ops::Deref for ZTEN_R {
    type Target = crate::FieldReader<u8, ZTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZTEn` writer - PWM Zero Point Trigger DAC Enable Bit\\nPWM can trigger EADC/DAC/DMA to start action when PWM counter down count to zero if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
pub struct ZTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZTEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM period point trigger DAC function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZTEN_A::_0)
    }
    #[doc = "PWM period point trigger DAC function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZTEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "PWM Period Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to (PERIODn+1) if this bit is set to1. Each bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTEN_A {
    #[doc = "0: PWM period point trigger DAC function Disabled"]
    _0 = 0,
    #[doc = "1: PWM period point trigger DAC function Enabled"]
    _1 = 1,
}
impl From<PTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTEn` reader - PWM Period Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to (PERIODn+1) if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
pub struct PTEN_R(crate::FieldReader<u8, PTEN_A>);
impl PTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTEN_A> {
        match self.bits {
            0 => Some(PTEN_A::_0),
            1 => Some(PTEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEN_A::_1
    }
}
impl core::ops::Deref for PTEN_R {
    type Target = crate::FieldReader<u8, PTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEn` writer - PWM Period Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to (PERIODn+1) if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM period point trigger DAC function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEN_A::_0)
    }
    #[doc = "PWM period point trigger DAC function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "PWM Compare Up Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in down counter type.\\nNote2: In complementary mode, CUTRGE1, 3, 5 use as another CUTRGE for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CUTRGEN_A {
    #[doc = "0: PWM Compare Up point trigger DAC function Disabled"]
    _0 = 0,
    #[doc = "1: PWM Compare Up point trigger DAC function Enabled"]
    _1 = 1,
}
impl From<CUTRGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CUTRGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CUTRGEn` reader - PWM Compare Up Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in down counter type.\\nNote2: In complementary mode, CUTRGE1, 3, 5 use as another CUTRGE for channel 0, 2, 4."]
pub struct CUTRGEN_R(crate::FieldReader<u8, CUTRGEN_A>);
impl CUTRGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CUTRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CUTRGEN_A> {
        match self.bits {
            0 => Some(CUTRGEN_A::_0),
            1 => Some(CUTRGEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CUTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CUTRGEN_A::_1
    }
}
impl core::ops::Deref for CUTRGEN_R {
    type Target = crate::FieldReader<u8, CUTRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUTRGEn` writer - PWM Compare Up Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in down counter type.\\nNote2: In complementary mode, CUTRGE1, 3, 5 use as another CUTRGE for channel 0, 2, 4."]
pub struct CUTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CUTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CUTRGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM Compare Up point trigger DAC function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CUTRGEN_A::_0)
    }
    #[doc = "PWM Compare Up point trigger DAC function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CUTRGEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "PWM Compare Down Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter down count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in up counter type.\\nNote2: In complementary mode, CDTRGE1, 3, 5 use as another CDTRGE for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDTRGEN_A {
    #[doc = "0: PWM Compare Down count point trigger DAC function Disabled"]
    _0 = 0,
    #[doc = "1: PWM Compare Down count point trigger DAC function Enabled"]
    _1 = 1,
}
impl From<CDTRGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTRGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDTRGEn` reader - PWM Compare Down Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter down count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in up counter type.\\nNote2: In complementary mode, CDTRGE1, 3, 5 use as another CDTRGE for channel 0, 2, 4."]
pub struct CDTRGEN_R(crate::FieldReader<u8, CDTRGEN_A>);
impl CDTRGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDTRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTRGEN_A> {
        match self.bits {
            0 => Some(CDTRGEN_A::_0),
            1 => Some(CDTRGEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CDTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CDTRGEN_A::_1
    }
}
impl core::ops::Deref for CDTRGEN_R {
    type Target = crate::FieldReader<u8, CDTRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDTRGEn` writer - PWM Compare Down Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter down count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in up counter type.\\nNote2: In complementary mode, CDTRGE1, 3, 5 use as another CDTRGE for channel 0, 2, 4."]
pub struct CDTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTRGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM Compare Down count point trigger DAC function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDTRGEN_A::_0)
    }
    #[doc = "PWM Compare Down count point trigger DAC function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDTRGEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Zero Point Trigger DAC Enable Bit\\nPWM can trigger EADC/DAC/DMA to start action when PWM counter down count to zero if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn zten(&self) -> ZTEN_R {
        ZTEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - PWM Period Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to (PERIODn+1) if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in down counter type.\\nNote2: In complementary mode, CUTRGE1, 3, 5 use as another CUTRGE for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cutrgen(&self) -> CUTRGEN_R {
        CUTRGEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter down count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in up counter type.\\nNote2: In complementary mode, CDTRGE1, 3, 5 use as another CDTRGE for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cdtrgen(&self) -> CDTRGEN_R {
        CDTRGEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Zero Point Trigger DAC Enable Bit\\nPWM can trigger EADC/DAC/DMA to start action when PWM counter down count to zero if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn zten(&mut self) -> ZTEN_W {
        ZTEN_W { w: self }
    }
    #[doc = "Bits 8:13 - PWM Period Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to (PERIODn+1) if this bit is set to1. Each bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter up count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in down counter type.\\nNote2: In complementary mode, CUTRGE1, 3, 5 use as another CUTRGE for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cutrgen(&mut self) -> CUTRGEN_W {
        CUTRGEN_W { w: self }
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Point Trigger DAC Enable Bit\\nPWM can trigger DAC to start action when PWM counter down count to CMPDAT if this bit is set to1. Each bit n controls the corresponding PWM channel n.\\nNote1: This bit should keep at 0 when PWM counter operating in up counter type.\\nNote2: In complementary mode, CDTRGE1, 3, 5 use as another CDTRGE for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cdtrgen(&mut self) -> CDTRGEN_W {
        CDTRGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Trigger DAC Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dactrgen](index.html) module"]
pub struct PWM_DACTRGEN_SPEC;
impl crate::RegisterSpec for PWM_DACTRGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_dactrgen::R](R) reader structure"]
impl crate::Readable for PWM_DACTRGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_dactrgen::W](W) writer structure"]
impl crate::Writable for PWM_DACTRGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DACTRGEN to value 0"]
impl crate::Resettable for PWM_DACTRGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
