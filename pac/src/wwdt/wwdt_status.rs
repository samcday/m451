#[doc = "Register `WWDT_STATUS` reader"]
pub struct R(crate::R<WWDT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WWDT_STATUS_SPEC>> for R {
    fn from(reader: crate::R<WWDT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDT_STATUS` writer"]
pub struct W(crate::W<WWDT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDT_STATUS_SPEC>;
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
impl core::convert::From<crate::W<WWDT_STATUS_SPEC>> for W {
    fn from(writer: crate::W<WWDT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WWDT Compare Match Interrupt Flag\\nThis bit indicates the interrupt flag status of WWDT while WWDT counter value matches CMPDAT (WWDT_CTL\\[21:16\\]).\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTIF_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: WWDT counter value matches CMPDAT"]
    _1 = 1,
}
impl From<WWDTIF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTIF` reader - WWDT Compare Match Interrupt Flag\\nThis bit indicates the interrupt flag status of WWDT while WWDT counter value matches CMPDAT (WWDT_CTL\\[21:16\\]).\\nNote: This bit is cleared by writing 1 to it."]
pub struct WWDTIF_R(crate::FieldReader<bool, WWDTIF_A>);
impl WWDTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDTIF_A {
        match self.bits {
            false => WWDTIF_A::_0,
            true => WWDTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WWDTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WWDTIF_A::_1
    }
}
impl core::ops::Deref for WWDTIF_R {
    type Target = crate::FieldReader<bool, WWDTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDTIF` writer - WWDT Compare Match Interrupt Flag\\nThis bit indicates the interrupt flag status of WWDT while WWDT counter value matches CMPDAT (WWDT_CTL\\[21:16\\]).\\nNote: This bit is cleared by writing 1 to it."]
pub struct WWDTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WWDTIF_A::_0)
    }
    #[doc = "WWDT counter value matches CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WWDTIF_A::_1)
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
#[doc = "WWDT Timer-out Reset Flag\\nThis bit indicates the system has been reset by WWDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTRF_A {
    #[doc = "0: WWDT time-out reset did not occur"]
    _0 = 0,
    #[doc = "1: WWDT time-out reset occurred"]
    _1 = 1,
}
impl From<WWDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTRF` reader - WWDT Timer-out Reset Flag\\nThis bit indicates the system has been reset by WWDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
pub struct WWDTRF_R(crate::FieldReader<bool, WWDTRF_A>);
impl WWDTRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDTRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDTRF_A {
        match self.bits {
            false => WWDTRF_A::_0,
            true => WWDTRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WWDTRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WWDTRF_A::_1
    }
}
impl core::ops::Deref for WWDTRF_R {
    type Target = crate::FieldReader<bool, WWDTRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDTRF` writer - WWDT Timer-out Reset Flag\\nThis bit indicates the system has been reset by WWDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
pub struct WWDTRF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WWDT time-out reset did not occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WWDTRF_A::_0)
    }
    #[doc = "WWDT time-out reset occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WWDTRF_A::_1)
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
    #[doc = "Bit 0 - WWDT Compare Match Interrupt Flag\\nThis bit indicates the interrupt flag status of WWDT while WWDT counter value matches CMPDAT (WWDT_CTL\\[21:16\\]).\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wwdtif(&self) -> WWDTIF_R {
        WWDTIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WWDT Timer-out Reset Flag\\nThis bit indicates the system has been reset by WWDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wwdtrf(&self) -> WWDTRF_R {
        WWDTRF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDT Compare Match Interrupt Flag\\nThis bit indicates the interrupt flag status of WWDT while WWDT counter value matches CMPDAT (WWDT_CTL\\[21:16\\]).\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wwdtif(&mut self) -> WWDTIF_W {
        WWDTIF_W { w: self }
    }
    #[doc = "Bit 1 - WWDT Timer-out Reset Flag\\nThis bit indicates the system has been reset by WWDT time-out reset or not.\\nNote: This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn wwdtrf(&mut self) -> WWDTRF_W {
        WWDTRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdt_status](index.html) module"]
pub struct WWDT_STATUS_SPEC;
impl crate::RegisterSpec for WWDT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdt_status::R](R) reader structure"]
impl crate::Readable for WWDT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdt_status::W](W) writer structure"]
impl crate::Writable for WWDT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDT_STATUS to value 0"]
impl crate::Resettable for WWDT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
