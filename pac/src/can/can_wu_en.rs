#[doc = "Register `CAN_WU_EN` reader"]
pub struct R(crate::R<CAN_WU_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_WU_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_WU_EN_SPEC>> for R {
    fn from(reader: crate::R<CAN_WU_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_WU_EN` writer"]
pub struct W(crate::W<CAN_WU_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_WU_EN_SPEC>;
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
impl core::convert::From<crate::W<CAN_WU_EN_SPEC>> for W {
    fn from(writer: crate::W<CAN_WU_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up Enable Bit\\nNote: User can wake-up system when there is a falling edge in the CAN_Rx pin..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKUP_EN_A {
    #[doc = "0: The wake-up function Disabled"]
    _0 = 0,
    #[doc = "1: The wake-up function Enabled"]
    _1 = 1,
}
impl From<WAKUP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAKUP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKUP_EN` reader - Wake-up Enable Bit\\nNote: User can wake-up system when there is a falling edge in the CAN_Rx pin.."]
pub struct WAKUP_EN_R(crate::FieldReader<bool, WAKUP_EN_A>);
impl WAKUP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKUP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKUP_EN_A {
        match self.bits {
            false => WAKUP_EN_A::_0,
            true => WAKUP_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAKUP_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAKUP_EN_A::_1
    }
}
impl core::ops::Deref for WAKUP_EN_R {
    type Target = crate::FieldReader<bool, WAKUP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKUP_EN` writer - Wake-up Enable Bit\\nNote: User can wake-up system when there is a falling edge in the CAN_Rx pin.."]
pub struct WAKUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKUP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKUP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The wake-up function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKUP_EN_A::_0)
    }
    #[doc = "The wake-up function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKUP_EN_A::_1)
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
    #[doc = "Bit 0 - Wake-up Enable Bit\\nNote: User can wake-up system when there is a falling edge in the CAN_Rx pin.."]
    #[inline(always)]
    pub fn wakup_en(&self) -> WAKUP_EN_R {
        WAKUP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Enable Bit\\nNote: User can wake-up system when there is a falling edge in the CAN_Rx pin.."]
    #[inline(always)]
    pub fn wakup_en(&mut self) -> WAKUP_EN_W {
        WAKUP_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_wu_en](index.html) module"]
pub struct CAN_WU_EN_SPEC;
impl crate::RegisterSpec for CAN_WU_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_wu_en::R](R) reader structure"]
impl crate::Readable for CAN_WU_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_wu_en::W](W) writer structure"]
impl crate::Writable for CAN_WU_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_WU_EN to value 0"]
impl crate::Resettable for CAN_WU_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
