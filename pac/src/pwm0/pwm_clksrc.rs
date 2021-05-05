#[doc = "Register `PWM_CLKSRC` reader"]
pub struct R(crate::R<PWM_CLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CLKSRC_SPEC>> for R {
    fn from(reader: crate::R<PWM_CLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CLKSRC` writer"]
pub struct W(crate::W<PWM_CLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CLKSRC_SPEC>;
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
impl core::convert::From<crate::W<PWM_CLKSRC_SPEC>> for W {
    fn from(writer: crate::W<PWM_CLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM_CH01 External Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECLKSRC0_A {
    #[doc = "0: PWMx_CLK, x denotes 0 or 1"]
    _0 = 0,
    #[doc = "1: TIMER0 overflow"]
    _1 = 1,
    #[doc = "2: TIMER1 overflow"]
    _2 = 2,
    #[doc = "3: TIMER2 overflow"]
    _3 = 3,
    #[doc = "4: TIMER3 overflow"]
    _4 = 4,
}
impl From<ECLKSRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ECLKSRC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECLKSRC0` reader - PWM_CH01 External Clock Source Select"]
pub struct ECLKSRC0_R(crate::FieldReader<u8, ECLKSRC0_A>);
impl ECLKSRC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECLKSRC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECLKSRC0_A> {
        match self.bits {
            0 => Some(ECLKSRC0_A::_0),
            1 => Some(ECLKSRC0_A::_1),
            2 => Some(ECLKSRC0_A::_2),
            3 => Some(ECLKSRC0_A::_3),
            4 => Some(ECLKSRC0_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ECLKSRC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ECLKSRC0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ECLKSRC0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ECLKSRC0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == ECLKSRC0_A::_4
    }
}
impl core::ops::Deref for ECLKSRC0_R {
    type Target = crate::FieldReader<u8, ECLKSRC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECLKSRC0` writer - PWM_CH01 External Clock Source Select"]
pub struct ECLKSRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ECLKSRC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECLKSRC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWMx_CLK, x denotes 0 or 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_0)
    }
    #[doc = "TIMER0 overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_1)
    }
    #[doc = "TIMER1 overflow"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_2)
    }
    #[doc = "TIMER2 overflow"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_3)
    }
    #[doc = "TIMER3 overflow"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ECLKSRC0_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "PWM_CH23 External Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECLKSRC2_A {
    #[doc = "0: PWMx_CLK, x denotes 0 or 1"]
    _0 = 0,
    #[doc = "1: TIMER0 overflow"]
    _1 = 1,
    #[doc = "2: TIMER1 overflow"]
    _2 = 2,
    #[doc = "3: TIMER2 overflow"]
    _3 = 3,
    #[doc = "4: TIMER3 overflow"]
    _4 = 4,
}
impl From<ECLKSRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: ECLKSRC2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECLKSRC2` reader - PWM_CH23 External Clock Source Select"]
pub struct ECLKSRC2_R(crate::FieldReader<u8, ECLKSRC2_A>);
impl ECLKSRC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECLKSRC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECLKSRC2_A> {
        match self.bits {
            0 => Some(ECLKSRC2_A::_0),
            1 => Some(ECLKSRC2_A::_1),
            2 => Some(ECLKSRC2_A::_2),
            3 => Some(ECLKSRC2_A::_3),
            4 => Some(ECLKSRC2_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ECLKSRC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ECLKSRC2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ECLKSRC2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ECLKSRC2_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == ECLKSRC2_A::_4
    }
}
impl core::ops::Deref for ECLKSRC2_R {
    type Target = crate::FieldReader<u8, ECLKSRC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECLKSRC2` writer - PWM_CH23 External Clock Source Select"]
pub struct ECLKSRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECLKSRC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECLKSRC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWMx_CLK, x denotes 0 or 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECLKSRC2_A::_0)
    }
    #[doc = "TIMER0 overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECLKSRC2_A::_1)
    }
    #[doc = "TIMER1 overflow"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ECLKSRC2_A::_2)
    }
    #[doc = "TIMER2 overflow"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ECLKSRC2_A::_3)
    }
    #[doc = "TIMER3 overflow"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ECLKSRC2_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "PWM_CH45 External Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECLKSRC4_A {
    #[doc = "0: PWMx_CLK, x denotes 0 or 1"]
    _0 = 0,
    #[doc = "1: TIMER0 overflow"]
    _1 = 1,
    #[doc = "2: TIMER1 overflow"]
    _2 = 2,
    #[doc = "3: TIMER2 overflow"]
    _3 = 3,
    #[doc = "4: TIMER3 overflow"]
    _4 = 4,
}
impl From<ECLKSRC4_A> for u8 {
    #[inline(always)]
    fn from(variant: ECLKSRC4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECLKSRC4` reader - PWM_CH45 External Clock Source Select"]
pub struct ECLKSRC4_R(crate::FieldReader<u8, ECLKSRC4_A>);
impl ECLKSRC4_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECLKSRC4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECLKSRC4_A> {
        match self.bits {
            0 => Some(ECLKSRC4_A::_0),
            1 => Some(ECLKSRC4_A::_1),
            2 => Some(ECLKSRC4_A::_2),
            3 => Some(ECLKSRC4_A::_3),
            4 => Some(ECLKSRC4_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ECLKSRC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ECLKSRC4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == ECLKSRC4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == ECLKSRC4_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == ECLKSRC4_A::_4
    }
}
impl core::ops::Deref for ECLKSRC4_R {
    type Target = crate::FieldReader<u8, ECLKSRC4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECLKSRC4` writer - PWM_CH45 External Clock Source Select"]
pub struct ECLKSRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ECLKSRC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECLKSRC4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWMx_CLK, x denotes 0 or 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECLKSRC4_A::_0)
    }
    #[doc = "TIMER0 overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECLKSRC4_A::_1)
    }
    #[doc = "TIMER1 overflow"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ECLKSRC4_A::_2)
    }
    #[doc = "TIMER2 overflow"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(ECLKSRC4_A::_3)
    }
    #[doc = "TIMER3 overflow"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ECLKSRC4_A::_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PWM_CH01 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc0(&self) -> ECLKSRC0_R {
        ECLKSRC0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - PWM_CH23 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc2(&self) -> ECLKSRC2_R {
        ECLKSRC2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - PWM_CH45 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc4(&self) -> ECLKSRC4_R {
        ECLKSRC4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM_CH01 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc0(&mut self) -> ECLKSRC0_W {
        ECLKSRC0_W { w: self }
    }
    #[doc = "Bits 8:10 - PWM_CH23 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc2(&mut self) -> ECLKSRC2_W {
        ECLKSRC2_W { w: self }
    }
    #[doc = "Bits 16:18 - PWM_CH45 External Clock Source Select"]
    #[inline(always)]
    pub fn eclksrc4(&mut self) -> ECLKSRC4_W {
        ECLKSRC4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clock Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_clksrc](index.html) module"]
pub struct PWM_CLKSRC_SPEC;
impl crate::RegisterSpec for PWM_CLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_clksrc::R](R) reader structure"]
impl crate::Readable for PWM_CLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_clksrc::W](W) writer structure"]
impl crate::Writable for PWM_CLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CLKSRC to value 0"]
impl crate::Resettable for PWM_CLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
