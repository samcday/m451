#[doc = "Register `PWM_CAPCTL` reader"]
pub struct R(crate::R<PWM_CAPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CAPCTL_SPEC>> for R {
    fn from(reader: crate::R<PWM_CAPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CAPCTL` writer"]
pub struct W(crate::W<PWM_CAPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CAPCTL_SPEC>;
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
impl core::convert::From<crate::W<PWM_CAPCTL_SPEC>> for W {
    fn from(writer: crate::W<PWM_CAPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture Function Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPENN_A {
    #[doc = "0: Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    _0 = 0,
    #[doc = "1: Capture function Enabled. Capture latched the PWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    _1 = 1,
}
impl From<CAPENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPENn` reader - Capture Function Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPENN_R(crate::FieldReader<u8, CAPENN_A>);
impl CAPENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPENN_A> {
        match self.bits {
            0 => Some(CAPENN_A::_0),
            1 => Some(CAPENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPENN_A::_1
    }
}
impl core::ops::Deref for CAPENN_R {
    type Target = crate::FieldReader<u8, CAPENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPENn` writer - Capture Function Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture function Disabled. RCAPDAT/FCAPDAT register will not be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPENN_A::_0)
    }
    #[doc = "Capture function Enabled. Capture latched the PWM counter value when detected rising or falling edge of input signal and saved to RCAPDAT (Rising latch) and FCAPDAT (Falling latch)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Capture Inverter Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPINVN_A {
    #[doc = "0: Capture source inverter Disabled"]
    _0 = 0,
    #[doc = "1: Capture source inverter Enabled. Reverse the input signal from GPIO"]
    _1 = 1,
}
impl From<CAPINVN_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPINVN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPINVn` reader - Capture Inverter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPINVN_R(crate::FieldReader<u8, CAPINVN_A>);
impl CAPINVN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPINVN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPINVN_A> {
        match self.bits {
            0 => Some(CAPINVN_A::_0),
            1 => Some(CAPINVN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPINVN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPINVN_A::_1
    }
}
impl core::ops::Deref for CAPINVN_R {
    type Target = crate::FieldReader<u8, CAPINVN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPINVn` writer - Capture Inverter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct CAPINVN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPINVN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPINVN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture source inverter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPINVN_A::_0)
    }
    #[doc = "Capture source inverter Enabled. Reverse the input signal from GPIO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPINVN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Rising Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRLDENN_A {
    #[doc = "0: Rising capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Rising capture reload counter Enabled"]
    _1 = 1,
}
impl From<RCRLDENN_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRLDENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCRLDENn` reader - Rising Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct RCRLDENN_R(crate::FieldReader<u8, RCRLDENN_A>);
impl RCRLDENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCRLDENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RCRLDENN_A> {
        match self.bits {
            0 => Some(RCRLDENN_A::_0),
            1 => Some(RCRLDENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCRLDENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCRLDENN_A::_1
    }
}
impl core::ops::Deref for RCRLDENN_R {
    type Target = crate::FieldReader<u8, RCRLDENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRLDENn` writer - Rising Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct RCRLDENN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRLDENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRLDENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rising capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCRLDENN_A::_0)
    }
    #[doc = "Rising capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCRLDENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Falling Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCRLDENN_A {
    #[doc = "0: Falling capture reload counter Disabled"]
    _0 = 0,
    #[doc = "1: Falling capture reload counter Enabled"]
    _1 = 1,
}
impl From<FCRLDENN_A> for u8 {
    #[inline(always)]
    fn from(variant: FCRLDENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCRLDENn` reader - Falling Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct FCRLDENN_R(crate::FieldReader<u8, FCRLDENN_A>);
impl FCRLDENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCRLDENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCRLDENN_A> {
        match self.bits {
            0 => Some(FCRLDENN_A::_0),
            1 => Some(FCRLDENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCRLDENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCRLDENN_A::_1
    }
}
impl core::ops::Deref for FCRLDENN_R {
    type Target = crate::FieldReader<u8, FCRLDENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRLDENn` writer - Falling Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
pub struct FCRLDENN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRLDENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRLDENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Falling capture reload counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCRLDENN_A::_0)
    }
    #[doc = "Falling capture reload counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCRLDENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Capture Function Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capenn(&self) -> CAPENN_R {
        CAPENN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Capture Inverter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capinvn(&self) -> CAPINVN_R {
        CAPINVN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rising Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn rcrldenn(&self) -> RCRLDENN_R {
        RCRLDENN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Falling Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn fcrldenn(&self) -> FCRLDENN_R {
        FCRLDENN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Capture Function Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capenn(&mut self) -> CAPENN_W {
        CAPENN_W { w: self }
    }
    #[doc = "Bits 8:13 - Capture Inverter Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn capinvn(&mut self) -> CAPINVN_W {
        CAPINVN_W { w: self }
    }
    #[doc = "Bits 16:21 - Rising Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn rcrldenn(&mut self) -> RCRLDENN_W {
        RCRLDENN_W { w: self }
    }
    #[doc = "Bits 24:29 - Falling Capture Reload Enable Bits\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn fcrldenn(&mut self) -> FCRLDENN_W {
        FCRLDENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_capctl](index.html) module"]
pub struct PWM_CAPCTL_SPEC;
impl crate::RegisterSpec for PWM_CAPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_capctl::R](R) reader structure"]
impl crate::Readable for PWM_CAPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_capctl::W](W) writer structure"]
impl crate::Writable for PWM_CAPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CAPCTL to value 0"]
impl crate::Resettable for PWM_CAPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
