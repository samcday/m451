#[doc = "Register `PDMA_CHCTL` reader"]
pub struct R(crate::R<PDMA_CHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_CHCTL_SPEC>> for R {
    fn from(reader: crate::R<PDMA_CHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CHCTL` writer"]
pub struct W(crate::W<PDMA_CHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CHCTL_SPEC>;
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
impl core::convert::From<crate::W<PDMA_CHCTL_SPEC>> for W {
    fn from(writer: crate::W<PDMA_CHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Channel Enable Bit\\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: CHEN8~11 is M45xG/M45xE only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CHENN_A {
    #[doc = "0: PDMA channel \\[n\\]
Disabled"]
    _0 = 0,
    #[doc = "1: PDMA channel \\[n\\]
Enabled"]
    _1 = 1,
}
impl From<CHENN_A> for u16 {
    #[inline(always)]
    fn from(variant: CHENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHENn` reader - PDMA Channel Enable Bit\\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: CHEN8~11 is M45xG/M45xE only."]
pub struct CHENN_R(crate::FieldReader<u16, CHENN_A>);
impl CHENN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHENN_A> {
        match self.bits {
            0 => Some(CHENN_A::_0),
            1 => Some(CHENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHENN_A::_1
    }
}
impl core::ops::Deref for CHENN_R {
    type Target = crate::FieldReader<u16, CHENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHENn` writer - PDMA Channel Enable Bit\\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: CHEN8~11 is M45xG/M45xE only."]
pub struct CHENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDMA channel \\[n\\]
Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHENN_A::_0)
    }
    #[doc = "PDMA channel \\[n\\]
Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PDMA Channel Enable Bit\\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: CHEN8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn chenn(&self) -> CHENN_R {
        CHENN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PDMA Channel Enable Bit\\nSet this bit to 1 to enable PDMAn operation. Channel cannot be active if it is not set as enabled.\\nNote1: If software stops corresponding PDMA transfer by setting PDMA_STOP register, this bit will be cleared automatically after finishing current transfer.\\nNote2: Software reset (writing 0xFFFF_FFFF to PDMA_STOP register) will also clear this bit.\\nNote3: CHEN8~11 is M45xG/M45xE only."]
    #[inline(always)]
    pub fn chenn(&mut self) -> CHENN_W {
        CHENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_chctl](index.html) module"]
pub struct PDMA_CHCTL_SPEC;
impl crate::RegisterSpec for PDMA_CHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_chctl::R](R) reader structure"]
impl crate::Readable for PDMA_CHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_chctl::W](W) writer structure"]
impl crate::Writable for PDMA_CHCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_CHCTL to value 0"]
impl crate::Resettable for PDMA_CHCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
