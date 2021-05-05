#[doc = "Register `SYS_SRAM_INTCTL` reader"]
pub struct R(crate::R<SYS_SRAM_INTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_INTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_SRAM_INTCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_SRAM_INTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_INTCTL` writer"]
pub struct W(crate::W<SYS_SRAM_INTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_INTCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_SRAM_INTCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_SRAM_INTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM Parity Check Error Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERRIEN_A {
    #[doc = "0: SRAM parity check error interrupt Disabled"]
    _0 = 0,
    #[doc = "1: SRAM parity check error interrupt Enabled"]
    _1 = 1,
}
impl From<PERRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIEN` reader - SRAM Parity Check Error Interrupt Enable Bit"]
pub struct PERRIEN_R(crate::FieldReader<bool, PERRIEN_A>);
impl PERRIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIEN_A {
        match self.bits {
            false => PERRIEN_A::_0,
            true => PERRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PERRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PERRIEN_A::_1
    }
}
impl core::ops::Deref for PERRIEN_R {
    type Target = crate::FieldReader<bool, PERRIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRIEN` writer - SRAM Parity Check Error Interrupt Enable Bit"]
pub struct PERRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERRIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM parity check error interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PERRIEN_A::_0)
    }
    #[doc = "SRAM parity check error interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PERRIEN_A::_1)
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
    #[doc = "Bit 0 - SRAM Parity Check Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn perrien(&self) -> PERRIEN_R {
        PERRIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Parity Check Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn perrien(&mut self) -> PERRIEN_W {
        PERRIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM Interrupt Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_intctl](index.html) module"]
pub struct SYS_SRAM_INTCTL_SPEC;
impl crate::RegisterSpec for SYS_SRAM_INTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_intctl::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_INTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_intctl::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_INTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_INTCTL to value 0"]
impl crate::Resettable for SYS_SRAM_INTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
