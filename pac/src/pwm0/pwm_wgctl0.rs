#[doc = "Register `PWM_WGCTL0` reader"]
pub struct R(crate::R<PWM_WGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_WGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_WGCTL0_SPEC>> for R {
    fn from(reader: crate::R<PWM_WGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_WGCTL0` writer"]
pub struct W(crate::W<PWM_WGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_WGCTL0_SPEC>;
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
impl core::convert::From<crate::W<PWM_WGCTL0_SPEC>> for W {
    fn from(writer: crate::W<PWM_WGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Zero Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ZPCTLN_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM zero point output Low"]
    _1 = 1,
    #[doc = "10: PWM zero point output High"]
    _10 = 10,
    #[doc = "11: PWM zero point output Toggle"]
    _11 = 11,
}
impl From<ZPCTLN_A> for u16 {
    #[inline(always)]
    fn from(variant: ZPCTLN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZPCTLn` reader - PWM Zero Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to zero."]
pub struct ZPCTLN_R(crate::FieldReader<u16, ZPCTLN_A>);
impl ZPCTLN_R {
    pub(crate) fn new(bits: u16) -> Self {
        ZPCTLN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ZPCTLN_A> {
        match self.bits {
            0 => Some(ZPCTLN_A::_0),
            1 => Some(ZPCTLN_A::_1),
            10 => Some(ZPCTLN_A::_10),
            11 => Some(ZPCTLN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZPCTLN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZPCTLN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ZPCTLN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ZPCTLN_A::_11
    }
}
impl core::ops::Deref for ZPCTLN_R {
    type Target = crate::FieldReader<u16, ZPCTLN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPCTLn` writer - PWM Zero Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to zero."]
pub struct ZPCTLN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPCTLN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPCTLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPCTLN_A::_0)
    }
    #[doc = "PWM zero point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPCTLN_A::_1)
    }
    #[doc = "PWM zero point output High"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ZPCTLN_A::_10)
    }
    #[doc = "PWM zero point output Toggle"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ZPCTLN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "PWM Period (Center) Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to (PERIODn+1).\\nNote: This bit is center point control when PWM counter operating in up-down counter type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PRDPCTLN_A {
    #[doc = "0: Do nothing"]
    _0 = 0,
    #[doc = "1: PWM period (center) point output Low"]
    _1 = 1,
    #[doc = "10: PWM period (center) point output High"]
    _10 = 10,
    #[doc = "11: PWM period (center) point output Toggle"]
    _11 = 11,
}
impl From<PRDPCTLN_A> for u16 {
    #[inline(always)]
    fn from(variant: PRDPCTLN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRDPCTLn` reader - PWM Period (Center) Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to (PERIODn+1).\\nNote: This bit is center point control when PWM counter operating in up-down counter type."]
pub struct PRDPCTLN_R(crate::FieldReader<u16, PRDPCTLN_A>);
impl PRDPCTLN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRDPCTLN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRDPCTLN_A> {
        match self.bits {
            0 => Some(PRDPCTLN_A::_0),
            1 => Some(PRDPCTLN_A::_1),
            10 => Some(PRDPCTLN_A::_10),
            11 => Some(PRDPCTLN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRDPCTLN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRDPCTLN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PRDPCTLN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PRDPCTLN_A::_11
    }
}
impl core::ops::Deref for PRDPCTLN_R {
    type Target = crate::FieldReader<u16, PRDPCTLN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDPCTLn` writer - PWM Period (Center) Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to (PERIODn+1).\\nNote: This bit is center point control when PWM counter operating in up-down counter type."]
pub struct PRDPCTLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDPCTLN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDPCTLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDPCTLN_A::_0)
    }
    #[doc = "PWM period (center) point output Low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDPCTLN_A::_1)
    }
    #[doc = "PWM period (center) point output High"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PRDPCTLN_A::_10)
    }
    #[doc = "PWM period (center) point output Toggle"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PRDPCTLN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PWM Zero Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to zero."]
    #[inline(always)]
    pub fn zpctln(&self) -> ZPCTLN_R {
        ZPCTLN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - PWM Period (Center) Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to (PERIODn+1).\\nNote: This bit is center point control when PWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctln(&self) -> PRDPCTLN_R {
        PRDPCTLN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PWM Zero Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to zero."]
    #[inline(always)]
    pub fn zpctln(&mut self) -> ZPCTLN_W {
        ZPCTLN_W { w: self }
    }
    #[doc = "Bits 16:27 - PWM Period (Center) Point Control\\nEach bit n controls the corresponding PWM channel n.\\nPWM can control output level when PWM counter count to (PERIODn+1).\\nNote: This bit is center point control when PWM counter operating in up-down counter type."]
    #[inline(always)]
    pub fn prdpctln(&mut self) -> PRDPCTLN_W {
        PRDPCTLN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Generation Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_wgctl0](index.html) module"]
pub struct PWM_WGCTL0_SPEC;
impl crate::RegisterSpec for PWM_WGCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_wgctl0::R](R) reader structure"]
impl crate::Readable for PWM_WGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_wgctl0::W](W) writer structure"]
impl crate::Writable for PWM_WGCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_WGCTL0 to value 0"]
impl crate::Resettable for PWM_WGCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
