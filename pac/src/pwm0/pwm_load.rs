#[doc = "Register `PWM_LOAD` reader"]
pub struct R(crate::R<PWM_LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_LOAD_SPEC>> for R {
    fn from(reader: crate::R<PWM_LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_LOAD` writer"]
pub struct W(crate::W<PWM_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_LOAD_SPEC>;
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
impl core::convert::From<crate::W<PWM_LOAD_SPEC>> for W {
    fn from(writer: crate::W<PWM_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Re-load PWM Comparator Register (CMPDAT) Control Bit\\nThis bit is software write, hardware clear when current PWM period end. Each bit n controls the corresponding PWM channel n.\\nWrite Operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOADN_A {
    #[doc = "0: No effect.\\nNo load window is set"]
    _0 = 0,
    #[doc = "1: Set load window of window loading mode.\\nLoad window is set"]
    _1 = 1,
}
impl From<LOADN_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOADn` reader - Re-load PWM Comparator Register (CMPDAT) Control Bit\\nThis bit is software write, hardware clear when current PWM period end. Each bit n controls the corresponding PWM channel n.\\nWrite Operation:"]
pub struct LOADN_R(crate::FieldReader<u8, LOADN_A>);
impl LOADN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOADN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOADN_A> {
        match self.bits {
            0 => Some(LOADN_A::_0),
            1 => Some(LOADN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOADN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOADN_A::_1
    }
}
impl core::ops::Deref for LOADN_R {
    type Target = crate::FieldReader<u8, LOADN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOADn` writer - Re-load PWM Comparator Register (CMPDAT) Control Bit\\nThis bit is software write, hardware clear when current PWM period end. Each bit n controls the corresponding PWM channel n.\\nWrite Operation:"]
pub struct LOADN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOADN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect.\\nNo load window is set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOADN_A::_0)
    }
    #[doc = "Set load window of window loading mode.\\nLoad window is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOADN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Re-load PWM Comparator Register (CMPDAT) Control Bit\\nThis bit is software write, hardware clear when current PWM period end. Each bit n controls the corresponding PWM channel n.\\nWrite Operation:"]
    #[inline(always)]
    pub fn loadn(&self) -> LOADN_R {
        LOADN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Re-load PWM Comparator Register (CMPDAT) Control Bit\\nThis bit is software write, hardware clear when current PWM period end. Each bit n controls the corresponding PWM channel n.\\nWrite Operation:"]
    #[inline(always)]
    pub fn loadn(&mut self) -> LOADN_W {
        LOADN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_load](index.html) module"]
pub struct PWM_LOAD_SPEC;
impl crate::RegisterSpec for PWM_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_load::R](R) reader structure"]
impl crate::Readable for PWM_LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_load::W](W) writer structure"]
impl crate::Writable for PWM_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_LOAD to value 0"]
impl crate::Resettable for PWM_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
