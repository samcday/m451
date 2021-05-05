#[doc = "Register `PWM_WGCTL1` reader"]
pub struct R(crate::R<PWM_WGCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_WGCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_WGCTL1_SPEC>> for R {
    fn from(reader: crate::R<PWM_WGCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_WGCTL1` writer"]
pub struct W(crate::W<PWM_WGCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_WGCTL1_SPEC>;
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
impl core::convert::From<crate::W<PWM_WGCTL1_SPEC>> for W {
    fn from(writer: crate::W<PWM_WGCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Compare Up Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter up count to CMPDAT.\\nNote: In complementary mode, CMPUCTL1, 3, 5 use as another CMPUCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CMPUCTLN_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare up point output Low"]
    _1 = 1,
    #[doc = "10: PWM compare up point output High"]
    _10 = 10,
    #[doc = "11: PWM compare up point output Toggle"]
    _11 = 11,
}
impl From<CMPUCTLN_A> for u16 {
    #[inline(always)]
    fn from(variant: CMPUCTLN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUCTLn` reader - PWM Compare Up Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter up count to CMPDAT.\\nNote: In complementary mode, CMPUCTL1, 3, 5 use as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTLN_R(crate::FieldReader<u16, CMPUCTLN_A>);
impl CMPUCTLN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPUCTLN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPUCTLN_A> {
        match self.bits {
            0 => Some(CMPUCTLN_A::_0),
            1 => Some(CMPUCTLN_A::_1),
            10 => Some(CMPUCTLN_A::_10),
            11 => Some(CMPUCTLN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUCTLN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUCTLN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CMPUCTLN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMPUCTLN_A::_11
    }
}
impl core::ops::Deref for CMPUCTLN_R {
    type Target = crate::FieldReader<u16, CMPUCTLN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUCTLn` writer - PWM Compare Up Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter up count to CMPDAT.\\nNote: In complementary mode, CMPUCTL1, 3, 5 use as another CMPUCTL for channel 0, 2, 4."]
pub struct CMPUCTLN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUCTLN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUCTLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUCTLN_A::_0)
    }
    #[doc = "PWM compare up point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUCTLN_A::_1)
    }
    #[doc = "PWM compare up point output High"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPUCTLN_A::_10)
    }
    #[doc = "PWM compare up point output Toggle"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPUCTLN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "PWM Compare Down Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter down count to CMPDAT.\\nNote: In complementary mode, CMPDCTL1, 3, 5 use as another CMPDCTL for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CMPDCTLN_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM compare down point output Low"]
    _1 = 1,
    #[doc = "10: PWM compare down point output High"]
    _10 = 10,
    #[doc = "11: PWM compare down point output Toggle"]
    _11 = 11,
}
impl From<CMPDCTLN_A> for u16 {
    #[inline(always)]
    fn from(variant: CMPDCTLN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDCTLn` reader - PWM Compare Down Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter down count to CMPDAT.\\nNote: In complementary mode, CMPDCTL1, 3, 5 use as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTLN_R(crate::FieldReader<u16, CMPDCTLN_A>);
impl CMPDCTLN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPDCTLN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPDCTLN_A> {
        match self.bits {
            0 => Some(CMPDCTLN_A::_0),
            1 => Some(CMPDCTLN_A::_1),
            10 => Some(CMPDCTLN_A::_10),
            11 => Some(CMPDCTLN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDCTLN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDCTLN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CMPDCTLN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMPDCTLN_A::_11
    }
}
impl core::ops::Deref for CMPDCTLN_R {
    type Target = crate::FieldReader<u16, CMPDCTLN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDCTLn` writer - PWM Compare Down Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter down count to CMPDAT.\\nNote: In complementary mode, CMPDCTL1, 3, 5 use as another CMPDCTL for channel 0, 2, 4."]
pub struct CMPDCTLN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDCTLN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDCTLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDCTLN_A::_0)
    }
    #[doc = "PWM compare down point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDCTLN_A::_1)
    }
    #[doc = "PWM compare down point output High"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMPDCTLN_A::_10)
    }
    #[doc = "PWM compare down point output Toggle"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMPDCTLN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PWM Compare Up Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter up count to CMPDAT.\\nNote: In complementary mode, CMPUCTL1, 3, 5 use as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctln(&self) -> CMPUCTLN_R {
        CMPUCTLN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - PWM Compare Down Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter down count to CMPDAT.\\nNote: In complementary mode, CMPDCTL1, 3, 5 use as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctln(&self) -> CMPDCTLN_R {
        CMPDCTLN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PWM Compare Up Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter up count to CMPDAT.\\nNote: In complementary mode, CMPUCTL1, 3, 5 use as another CMPUCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuctln(&mut self) -> CMPUCTLN_W {
        CMPUCTLN_W { w: self }
    }
    #[doc = "Bits 16:27 - PWM Compare Down Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter down count to CMPDAT.\\nNote: In complementary mode, CMPDCTL1, 3, 5 use as another CMPDCTL for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdctln(&mut self) -> CMPDCTLN_W {
        CMPDCTLN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Generation Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wgctl1](index.html) module"]
pub struct PWM_WGCTL1_SPEC;
impl crate::RegisterSpec for PWM_WGCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_wgctl1::R](R) reader structure"]
impl crate::Readable for PWM_WGCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_wgctl1::W](W) writer structure"]
impl crate::Writable for PWM_WGCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_WGCTL1 to value 0"]
impl crate::Resettable for PWM_WGCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
