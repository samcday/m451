#[doc = "Register `TIMER0_EINTSTS` reader"]
pub struct R(crate::R<TIMER0_EINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_EINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMER0_EINTSTS_SPEC>> for R {
    fn from(reader: crate::R<TIMER0_EINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_EINTSTS` writer"]
pub struct W(crate::W<TIMER0_EINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_EINTSTS_SPEC>;
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
impl core::convert::From<crate::W<TIMER0_EINTSTS_SPEC>> for W {
    fn from(writer: crate::W<TIMER0_EINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer External Capture Interrupt Flag\\nThis bit indicates the timer external capture interrupt flag status.\\nNote3: There is a new incoming capture event detected before CPU clearing the CAPIF status. If the above condition occurred, the Timer will keep register TIMERx_CAP unchanged and drop the new capture value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPIF_A {
    #[doc = "0: Tx_EXT (x= 0~3) pin interrupt did not occur"]
    _0 = 0,
    #[doc = "1: Tx_EXT (x= 0~3) pin interrupt occurred"]
    _1 = 1,
}
impl From<CAPIF_A> for bool {
    #[inline(always)]
    fn from(variant: CAPIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPIF` reader - Timer External Capture Interrupt Flag\\nThis bit indicates the timer external capture interrupt flag status.\\nNote3: There is a new incoming capture event detected before CPU clearing the CAPIF status. If the above condition occurred, the Timer will keep register TIMERx_CAP unchanged and drop the new capture value."]
pub struct CAPIF_R(crate::FieldReader<bool, CAPIF_A>);
impl CAPIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPIF_A {
        match self.bits {
            false => CAPIF_A::_0,
            true => CAPIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPIF_A::_1
    }
}
impl core::ops::Deref for CAPIF_R {
    type Target = crate::FieldReader<bool, CAPIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPIF` writer - Timer External Capture Interrupt Flag\\nThis bit indicates the timer external capture interrupt flag status.\\nNote3: There is a new incoming capture event detected before CPU clearing the CAPIF status. If the above condition occurred, the Timer will keep register TIMERx_CAP unchanged and drop the new capture value."]
pub struct CAPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx_EXT (x= 0~3) pin interrupt did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPIF_A::_0)
    }
    #[doc = "Tx_EXT (x= 0~3) pin interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPIF_A::_1)
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
impl R {
    #[doc = "Bit 0 - Timer External Capture Interrupt Flag\\nThis bit indicates the timer external capture interrupt flag status.\\nNote3: There is a new incoming capture event detected before CPU clearing the CAPIF status. If the above condition occurred, the Timer will keep register TIMERx_CAP unchanged and drop the new capture value."]
    #[inline(always)]
    pub fn capif(&self) -> CAPIF_R {
        CAPIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer External Capture Interrupt Flag\\nThis bit indicates the timer external capture interrupt flag status.\\nNote3: There is a new incoming capture event detected before CPU clearing the CAPIF status. If the above condition occurred, the Timer will keep register TIMERx_CAP unchanged and drop the new capture value."]
    #[inline(always)]
    pub fn capif(&mut self) -> CAPIF_W {
        CAPIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0 External Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_eintsts](index.html) module"]
pub struct TIMER0_EINTSTS_SPEC;
impl crate::RegisterSpec for TIMER0_EINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_eintsts::R](R) reader structure"]
impl crate::Readable for TIMER0_EINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_eintsts::W](W) writer structure"]
impl crate::Writable for TIMER0_EINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_EINTSTS to value 0"]
impl crate::Resettable for TIMER0_EINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
