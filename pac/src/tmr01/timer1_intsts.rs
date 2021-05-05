#[doc = "Register `TIMER1_INTSTS` reader"]
pub struct R(crate::R<TIMER1_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER1_INTSTS_SPEC>> for R {
    fn from(reader: crate::R<TIMER1_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_INTSTS` writer"]
pub struct W(crate::W<TIMER1_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_INTSTS_SPEC>;
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
impl core::convert::From<crate::W<TIMER1_INTSTS_SPEC>> for W {
    fn from(writer: crate::W<TIMER1_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Interrupt Flag\\nThis bit indicates the interrupt flag status of Timer while 24-bit timer up counter CNT (TIMERx_CNT\\[23:0\\]) value reaches to CMPDAT (TIMERx_CMP\\[23:0\\]) value.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: CNT value matches the CMPDAT value"]
    _1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Timer Interrupt Flag\\nThis bit indicates the interrupt flag status of Timer while 24-bit timer up counter CNT (TIMERx_CNT\\[23:0\\]) value reaches to CMPDAT (TIMERx_CMP\\[23:0\\]) value.\\nNote: This bit is cleared by writing 1 to it."]
pub struct TIF_R(crate::FieldReader<bool, TIF_A>);
impl TIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::_0,
            true => TIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIF_A::_1
    }
}
impl core::ops::Deref for TIF_R {
    type Target = crate::FieldReader<bool, TIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF` writer - Timer Interrupt Flag\\nThis bit indicates the interrupt flag status of Timer while 24-bit timer up counter CNT (TIMERx_CNT\\[23:0\\]) value reaches to CMPDAT (TIMERx_CMP\\[23:0\\]) value.\\nNote: This bit is cleared by writing 1 to it."]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF_A::_0)
    }
    #[doc = "CNT value matches the CMPDAT value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Timer Wake-up Flag\\nThis bit indicates the interrupt wake-up flag status of timer.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWKF_A {
    #[doc = "0: Timer does not cause CPU wake-up"]
    _0 = 0,
    #[doc = "1: CPU wake-up from Idle or Power-down mode if timer time-out interrupt signal generated"]
    _1 = 1,
}
impl From<TWKF_A> for bool {
    #[inline(always)]
    fn from(variant: TWKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWKF` reader - Timer Wake-up Flag\\nThis bit indicates the interrupt wake-up flag status of timer.\\nNote: This bit is cleared by writing 1 to it."]
pub struct TWKF_R(crate::FieldReader<bool, TWKF_A>);
impl TWKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWKF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWKF_A {
        match self.bits {
            false => TWKF_A::_0,
            true => TWKF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TWKF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TWKF_A::_1
    }
}
impl core::ops::Deref for TWKF_R {
    type Target = crate::FieldReader<bool, TWKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWKF` writer - Timer Wake-up Flag\\nThis bit indicates the interrupt wake-up flag status of timer.\\nNote: This bit is cleared by writing 1 to it."]
pub struct TWKF_W<'a> {
    w: &'a mut W,
}
impl<'a> TWKF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWKF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer does not cause CPU wake-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWKF_A::_0)
    }
    #[doc = "CPU wake-up from Idle or Power-down mode if timer time-out interrupt signal generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWKF_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Interrupt Flag\\nThis bit indicates the interrupt flag status of Timer while 24-bit timer up counter CNT (TIMERx_CNT\\[23:0\\]) value reaches to CMPDAT (TIMERx_CMP\\[23:0\\]) value.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Wake-up Flag\\nThis bit indicates the interrupt wake-up flag status of timer.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn twkf(&self) -> TWKF_R {
        TWKF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag\\nThis bit indicates the interrupt flag status of Timer while 24-bit timer up counter CNT (TIMERx_CNT\\[23:0\\]) value reaches to CMPDAT (TIMERx_CMP\\[23:0\\]) value.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    #[doc = "Bit 1 - Timer Wake-up Flag\\nThis bit indicates the interrupt wake-up flag status of timer.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn twkf(&mut self) -> TWKF_W {
        TWKF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer1 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_intsts](index.html) module"]
pub struct TIMER1_INTSTS_SPEC;
impl crate::RegisterSpec for TIMER1_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_intsts::R](R) reader structure"]
impl crate::Readable for TIMER1_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_intsts::W](W) writer structure"]
impl crate::Writable for TIMER1_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_INTSTS to value 0"]
impl crate::Resettable for TIMER1_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
