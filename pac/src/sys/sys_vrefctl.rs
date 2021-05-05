#[doc = "Register `SYS_VREFCTL` reader"]
pub struct R(crate::R<SYS_VREFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_VREFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_VREFCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_VREFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_VREFCTL` writer"]
pub struct W(crate::W<SYS_VREFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_VREFCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_VREFCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_VREFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VREF Control Bits (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Connecting a 1uF capacitor to AVSS will make internal reference voltage more stable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREFCTL_A {
    #[doc = "0: VREF is from external pin"]
    _0 = 0,
    #[doc = "3: VREF is internal 2.56V"]
    _3 = 3,
    #[doc = "7: VREF is internal 2.048V"]
    _7 = 7,
    #[doc = "11: VREF is internal 3.072V"]
    _11 = 11,
    #[doc = "15: VREF is internal 4.096V"]
    _15 = 15,
}
impl From<VREFCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREFCTL` reader - VREF Control Bits (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Connecting a 1uF capacitor to AVSS will make internal reference voltage more stable."]
pub struct VREFCTL_R(crate::FieldReader<u8, VREFCTL_A>);
impl VREFCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREFCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREFCTL_A> {
        match self.bits {
            0 => Some(VREFCTL_A::_0),
            3 => Some(VREFCTL_A::_3),
            7 => Some(VREFCTL_A::_7),
            11 => Some(VREFCTL_A::_11),
            15 => Some(VREFCTL_A::_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VREFCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == VREFCTL_A::_3
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == VREFCTL_A::_7
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == VREFCTL_A::_11
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == VREFCTL_A::_15
    }
}
impl core::ops::Deref for VREFCTL_R {
    type Target = crate::FieldReader<u8, VREFCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFCTL` writer - VREF Control Bits (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Connecting a 1uF capacitor to AVSS will make internal reference voltage more stable."]
pub struct VREFCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VREF is from external pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFCTL_A::_0)
    }
    #[doc = "VREF is internal 2.56V"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(VREFCTL_A::_3)
    }
    #[doc = "VREF is internal 2.048V"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(VREFCTL_A::_7)
    }
    #[doc = "VREF is internal 3.072V"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VREFCTL_A::_11)
    }
    #[doc = "VREF is internal 4.096V"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(VREFCTL_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - VREF Control Bits (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Connecting a 1uF capacitor to AVSS will make internal reference voltage more stable."]
    #[inline(always)]
    pub fn vrefctl(&self) -> VREFCTL_R {
        VREFCTL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - VREF Control Bits (Write Protect)\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: Connecting a 1uF capacitor to AVSS will make internal reference voltage more stable."]
    #[inline(always)]
    pub fn vrefctl(&mut self) -> VREFCTL_W {
        VREFCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_vrefctl](index.html) module"]
pub struct SYS_VREFCTL_SPEC;
impl crate::RegisterSpec for SYS_VREFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_vrefctl::R](R) reader structure"]
impl crate::Readable for SYS_VREFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_vrefctl::W](W) writer structure"]
impl crate::Writable for SYS_VREFCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_VREFCTL to value 0"]
impl crate::Resettable for SYS_VREFCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
