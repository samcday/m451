#[doc = "Register `PA1_PDIO` reader"]
pub struct R(crate::R<PA1_PDIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA1_PDIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PA1_PDIO_SPEC>> for R {
    fn from(reader: crate::R<PA1_PDIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA1_PDIO` writer"]
pub struct W(crate::W<PA1_PDIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA1_PDIO_SPEC>;
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
impl core::convert::From<crate::W<PA1_PDIO_SPEC>> for W {
    fn from(writer: crate::W<PA1_PDIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO Px.N Pin Data Input/Output\\nWriting this bit can control one GPIO pin output value.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIO_A {
    #[doc = "0: Corresponding GPIO pin set to low"]
    _0 = 0,
    #[doc = "1: Corresponding GPIO pin set to high"]
    _1 = 1,
}
impl From<PDIO_A> for bool {
    #[inline(always)]
    fn from(variant: PDIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIO` reader - GPIO Px.N Pin Data Input/Output\\nWriting this bit can control one GPIO pin output value.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PDIO_R(crate::FieldReader<bool, PDIO_A>);
impl PDIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIO_A {
        match self.bits {
            false => PDIO_A::_0,
            true => PDIO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDIO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDIO_A::_1
    }
}
impl core::ops::Deref for PDIO_R {
    type Target = crate::FieldReader<bool, PDIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIO` writer - GPIO Px.N Pin Data Input/Output\\nWriting this bit can control one GPIO pin output value.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding GPIO pin set to low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDIO_A::_0)
    }
    #[doc = "Corresponding GPIO pin set to high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDIO_A::_1)
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
    #[doc = "Bit 0 - GPIO Px.N Pin Data Input/Output\\nWriting this bit can control one GPIO pin output value.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pdio(&self) -> PDIO_R {
        PDIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Px.N Pin Data Input/Output\\nWriting this bit can control one GPIO pin output value.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pdio(&mut self) -> PDIO_W {
        PDIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PA.n Pin Data Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa1_pdio](index.html) module"]
pub struct PA1_PDIO_SPEC;
impl crate::RegisterSpec for PA1_PDIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa1_pdio::R](R) reader structure"]
impl crate::Readable for PA1_PDIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa1_pdio::W](W) writer structure"]
impl crate::Writable for PA1_PDIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA1_PDIO to value 0"]
impl crate::Resettable for PA1_PDIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
