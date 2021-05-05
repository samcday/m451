#[doc = "Register `PWM_CTL1` reader"]
pub struct R(crate::R<PWM_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CTL1_SPEC>> for R {
    fn from(reader: crate::R<PWM_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL1` writer"]
pub struct W(crate::W<PWM_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL1_SPEC>;
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
impl core::convert::From<crate::W<PWM_CTL1_SPEC>> for W {
    fn from(writer: crate::W<PWM_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Counter Behavior Type\\nEach bit n controls corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CNTTYPEN_A {
    #[doc = "0: Up counter type (supports in capture mode)"]
    _0 = 0,
    #[doc = "1: Down count type (supports in capture mode)"]
    _1 = 1,
    #[doc = "10: Up-down counter type"]
    _10 = 10,
    #[doc = "11: Reserved."]
    _11 = 11,
}
impl From<CNTTYPEN_A> for u16 {
    #[inline(always)]
    fn from(variant: CNTTYPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTTYPEn` reader - PWM Counter Behavior Type\\nEach bit n controls corresponding PWM channel n."]
pub struct CNTTYPEN_R(crate::FieldReader<u16, CNTTYPEN_A>);
impl CNTTYPEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNTTYPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTTYPEN_A> {
        match self.bits {
            0 => Some(CNTTYPEN_A::_0),
            1 => Some(CNTTYPEN_A::_1),
            10 => Some(CNTTYPEN_A::_10),
            11 => Some(CNTTYPEN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTTYPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTTYPEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CNTTYPEN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CNTTYPEN_A::_11
    }
}
impl core::ops::Deref for CNTTYPEN_R {
    type Target = crate::FieldReader<u16, CNTTYPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTTYPEn` writer - PWM Counter Behavior Type\\nEach bit n controls corresponding PWM channel n."]
pub struct CNTTYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTYPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTTYPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Up counter type (supports in capture mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTTYPEN_A::_0)
    }
    #[doc = "Down count type (supports in capture mode)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTTYPEN_A::_1)
    }
    #[doc = "Up-down counter type"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CNTTYPEN_A::_10)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CNTTYPEN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "PWM Counter Mode\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTMODEN_A {
    #[doc = "0: Auto-reload mode"]
    _0 = 0,
    #[doc = "1: One-shot mode"]
    _1 = 1,
}
impl From<CNTMODEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTMODEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTMODEn` reader - PWM Counter Mode\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTMODEN_R(crate::FieldReader<u8, CNTMODEN_A>);
impl CNTMODEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTMODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTMODEN_A> {
        match self.bits {
            0 => Some(CNTMODEN_A::_0),
            1 => Some(CNTMODEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CNTMODEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CNTMODEN_A::_1
    }
}
impl core::ops::Deref for CNTMODEN_R {
    type Target = crate::FieldReader<u8, CNTMODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMODEn` writer - PWM Counter Mode\\nEach bit n controls the corresponding PWM channel n."]
pub struct CNTMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMODEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto-reload mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMODEN_A::_0)
    }
    #[doc = "One-shot mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMODEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "PWM Output Mode\\nEach bit n controls the output mode of corresponding PWM channel n.\\nNote: When operating in group function, these bits must all set to the same mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMODEN_A {
    #[doc = "0: PWM independent mode"]
    _0 = 0,
    #[doc = "1: PWM complementary mode"]
    _1 = 1,
}
impl From<OUTMODEN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUTMODEn` reader - PWM Output Mode\\nEach bit n controls the output mode of corresponding PWM channel n.\\nNote: When operating in group function, these bits must all set to the same mode."]
pub struct OUTMODEN_R(crate::FieldReader<u8, OUTMODEN_A>);
impl OUTMODEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTMODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTMODEN_A> {
        match self.bits {
            0 => Some(OUTMODEN_A::_0),
            1 => Some(OUTMODEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OUTMODEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OUTMODEN_A::_1
    }
}
impl core::ops::Deref for OUTMODEN_R {
    type Target = crate::FieldReader<u8, OUTMODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTMODEn` writer - PWM Output Mode\\nEach bit n controls the output mode of corresponding PWM channel n.\\nNote: When operating in group function, these bits must all set to the same mode."]
pub struct OUTMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMODEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM independent mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTMODEN_A::_0)
    }
    #[doc = "PWM complementary mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTMODEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PWM Counter Behavior Type\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn cnttypen(&self) -> CNTTYPEN_R {
        CNTTYPEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - PWM Counter Mode\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntmoden(&self) -> CNTMODEN_R {
        CNTMODEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - PWM Output Mode\\nEach bit n controls the output mode of corresponding PWM channel n.\\nNote: When operating in group function, these bits must all set to the same mode."]
    #[inline(always)]
    pub fn outmoden(&self) -> OUTMODEN_R {
        OUTMODEN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PWM Counter Behavior Type\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn cnttypen(&mut self) -> CNTTYPEN_W {
        CNTTYPEN_W { w: self }
    }
    #[doc = "Bits 16:21 - PWM Counter Mode\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn cntmoden(&mut self) -> CNTMODEN_W {
        CNTMODEN_W { w: self }
    }
    #[doc = "Bits 24:26 - PWM Output Mode\\nEach bit n controls the output mode of corresponding PWM channel n.\\nNote: When operating in group function, these bits must all set to the same mode."]
    #[inline(always)]
    pub fn outmoden(&mut self) -> OUTMODEN_W {
        OUTMODEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl1](index.html) module"]
pub struct PWM_CTL1_SPEC;
impl crate::RegisterSpec for PWM_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl1::R](R) reader structure"]
impl crate::Readable for PWM_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl1::W](W) writer structure"]
impl crate::Writable for PWM_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CTL1 to value 0"]
impl crate::Resettable for PWM_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
