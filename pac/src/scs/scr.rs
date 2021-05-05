#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCR_SPEC>> for R {
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl core::convert::From<crate::W<SCR_SPEC>> for W {
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sleep-on-exit Enable Control\\nThis bit indicate Sleep-On-Exit when Returning from Handler Mode to Thread Mode.\\nSetting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXIT_A {
    #[doc = "0: Do not sleep when returning to Thread mode"]
    _0 = 0,
    #[doc = "1: Enters sleep, or deep sleep, on return from an ISR to Thread mode"]
    _1 = 1,
}
impl From<SLEEPONEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Sleep-on-exit Enable Control\\nThis bit indicate Sleep-On-Exit when Returning from Handler Mode to Thread Mode.\\nSetting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
pub struct SLEEPONEXIT_R(crate::FieldReader<bool, SLEEPONEXIT_A>);
impl SLEEPONEXIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPONEXIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::_0,
            true => SLEEPONEXIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLEEPONEXIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLEEPONEXIT_A::_1
    }
}
impl core::ops::Deref for SLEEPONEXIT_R {
    type Target = crate::FieldReader<bool, SLEEPONEXIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Sleep-on-exit Enable Control\\nThis bit indicate Sleep-On-Exit when Returning from Handler Mode to Thread Mode.\\nSetting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
pub struct SLEEPONEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPONEXIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPONEXIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::_0)
    }
    #[doc = "Enters sleep, or deep sleep, on return from an ISR to Thread mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::_1)
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
#[doc = "Processor Deep Sleep and Sleep Mode Selection\\nControl Whether the Processor Uses Sleep Or Deep Sleep as its Low Power Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEP_A {
    #[doc = "0: Sleep"]
    _0 = 0,
    #[doc = "1: Deep sleep"]
    _1 = 1,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - Processor Deep Sleep and Sleep Mode Selection\\nControl Whether the Processor Uses Sleep Or Deep Sleep as its Low Power Mode."]
pub struct SLEEPDEEP_R(crate::FieldReader<bool, SLEEPDEEP_A>);
impl SLEEPDEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPDEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::_0,
            true => SLEEPDEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLEEPDEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLEEPDEEP_A::_1
    }
}
impl core::ops::Deref for SLEEPDEEP_R {
    type Target = crate::FieldReader<bool, SLEEPDEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPDEEP` writer - Processor Deep Sleep and Sleep Mode Selection\\nControl Whether the Processor Uses Sleep Or Deep Sleep as its Low Power Mode."]
pub struct SLEEPDEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPDEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPDEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::_0)
    }
    #[doc = "Deep sleep"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Send Event on Pending\\nWhen an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE.\\nThe processor also wakes up on execution of an SEV instruction or an external event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPEND_A {
    #[doc = "0: Only enabled interrupts or events can wake up the processor, while disabled interrupts are excluded"]
    _0 = 0,
    #[doc = "1: Enabled events and all interrupts, including disabled interrupts, can wake up the processor"]
    _1 = 1,
}
impl From<SEVONPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending\\nWhen an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE.\\nThe processor also wakes up on execution of an SEV instruction or an external event."]
pub struct SEVONPEND_R(crate::FieldReader<bool, SEVONPEND_A>);
impl SEVONPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEVONPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::_0,
            true => SEVONPEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SEVONPEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SEVONPEND_A::_1
    }
}
impl core::ops::Deref for SEVONPEND_R {
    type Target = crate::FieldReader<bool, SEVONPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending\\nWhen an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE.\\nThe processor also wakes up on execution of an SEV instruction or an external event."]
pub struct SEVONPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVONPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only enabled interrupts or events can wake up the processor, while disabled interrupts are excluded"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEVONPEND_A::_0)
    }
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wake up the processor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEVONPEND_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Sleep-on-exit Enable Control\\nThis bit indicate Sleep-On-Exit when Returning from Handler Mode to Thread Mode.\\nSetting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Processor Deep Sleep and Sleep Mode Selection\\nControl Whether the Processor Uses Sleep Or Deep Sleep as its Low Power Mode."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending\\nWhen an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE.\\nThe processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep-on-exit Enable Control\\nThis bit indicate Sleep-On-Exit when Returning from Handler Mode to Thread Mode.\\nSetting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W { w: self }
    }
    #[doc = "Bit 2 - Processor Deep Sleep and Sleep Mode Selection\\nControl Whether the Processor Uses Sleep Or Deep Sleep as its Low Power Mode."]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W { w: self }
    }
    #[doc = "Bit 4 - Send Event on Pending\\nWhen an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE.\\nThe processor also wakes up on execution of an SEV instruction or an external event."]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
