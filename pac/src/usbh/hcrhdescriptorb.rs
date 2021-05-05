#[doc = "Register `HCRHDESCRIPTORB` reader"]
pub struct R(crate::R<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCRHDESCRIPTORB_SPEC>> for R {
    fn from(reader: crate::R<HCRHDESCRIPTORB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORB` writer"]
pub struct W(crate::W<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORB_SPEC>;
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
impl core::convert::From<crate::W<HCRHDESCRIPTORB_SPEC>> for W {
    fn from(writer: crate::W<HCRHDESCRIPTORB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Power Control Mask\\nGlobal power switching. This field is only valid if PowerSwitchingMode is set (individual port switching). When set, the port only responds to individual port power switching commands (Set/ClearPortPower). When cleared, the port only responds to global power switching commands (Set/ClearGlobalPower).\\nNote: PPCM\\[15:2\\]
and PPCM\\[0\\]
are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PPCM_A {
    #[doc = "0: Port power controlled by global power switching"]
    _0 = 0,
    #[doc = "1: Port power controlled by port power switching"]
    _1 = 1,
}
impl From<PPCM_A> for u16 {
    #[inline(always)]
    fn from(variant: PPCM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PPCM` reader - Port Power Control Mask\\nGlobal power switching. This field is only valid if PowerSwitchingMode is set (individual port switching). When set, the port only responds to individual port power switching commands (Set/ClearPortPower). When cleared, the port only responds to global power switching commands (Set/ClearGlobalPower).\\nNote: PPCM\\[15:2\\]
and PPCM\\[0\\]
are reserved."]
pub struct PPCM_R(crate::FieldReader<u16, PPCM_A>);
impl PPCM_R {
    pub(crate) fn new(bits: u16) -> Self {
        PPCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPCM_A> {
        match self.bits {
            0 => Some(PPCM_A::_0),
            1 => Some(PPCM_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PPCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PPCM_A::_1
    }
}
impl core::ops::Deref for PPCM_R {
    type Target = crate::FieldReader<u16, PPCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM` writer - Port Power Control Mask\\nGlobal power switching. This field is only valid if PowerSwitchingMode is set (individual port switching). When set, the port only responds to individual port power switching commands (Set/ClearPortPower). When cleared, the port only responds to global power switching commands (Set/ClearGlobalPower).\\nNote: PPCM\\[15:2\\]
and PPCM\\[0\\]
are reserved."]
pub struct PPCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPCM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port power controlled by global power switching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PPCM_A::_0)
    }
    #[doc = "Port power controlled by port power switching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PPCM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Port Power Control Mask\\nGlobal power switching. This field is only valid if PowerSwitchingMode is set (individual port switching). When set, the port only responds to individual port power switching commands (Set/ClearPortPower). When cleared, the port only responds to global power switching commands (Set/ClearGlobalPower).\\nNote: PPCM\\[15:2\\]
and PPCM\\[0\\]
are reserved."]
    #[inline(always)]
    pub fn ppcm(&self) -> PPCM_R {
        PPCM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Port Power Control Mask\\nGlobal power switching. This field is only valid if PowerSwitchingMode is set (individual port switching). When set, the port only responds to individual port power switching commands (Set/ClearPortPower). When cleared, the port only responds to global power switching commands (Set/ClearGlobalPower).\\nNote: PPCM\\[15:2\\]
and PPCM\\[0\\]
are reserved."]
    #[inline(always)]
    pub fn ppcm(&mut self) -> PPCM_W {
        PPCM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Root Hub Descriptor B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptorb](index.html) module"]
pub struct HCRHDESCRIPTORB_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptorb::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptorb::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORB to value 0"]
impl crate::Resettable for HCRHDESCRIPTORB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
