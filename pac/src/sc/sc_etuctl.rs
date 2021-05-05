#[doc = "Register `SC_ETUCTL` reader"]
pub struct R(crate::R<SC_ETUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_ETUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_ETUCTL_SPEC>> for R {
    fn from(reader: crate::R<SC_ETUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_ETUCTL` writer"]
pub struct W(crate::W<SC_ETUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_ETUCTL_SPEC>;
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
impl core::convert::From<crate::W<SC_ETUCTL_SPEC>> for W {
    fn from(writer: crate::W<SC_ETUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETURDIV` reader - ETU Rate Divider\\nThe field indicates the clock rate divider.\\nThe real ETU is ETURDIV + 1.\\nNote: Software can configure this field, but this field must be greater than 0x004."]
pub struct ETURDIV_R(crate::FieldReader<u16, u16>);
impl ETURDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        ETURDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETURDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETURDIV` writer - ETU Rate Divider\\nThe field indicates the clock rate divider.\\nThe real ETU is ETURDIV + 1.\\nNote: Software can configure this field, but this field must be greater than 0x004."]
pub struct ETURDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETURDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Compensation Mode Enable Bit\\nThis bit enables clock compensation function. When this bit enabled, hardware will alternate between n clock cycles and n-1 clock cycles, where n is the value to be written into the ETURDIV .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPEN_A {
    #[doc = "0: Compensation function Disabled"]
    _0 = 0,
    #[doc = "1: Compensation function Enabled"]
    _1 = 1,
}
impl From<CMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPEN` reader - Compensation Mode Enable Bit\\nThis bit enables clock compensation function. When this bit enabled, hardware will alternate between n clock cycles and n-1 clock cycles, where n is the value to be written into the ETURDIV ."]
pub struct CMPEN_R(crate::FieldReader<bool, CMPEN_A>);
impl CMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPEN_A {
        match self.bits {
            false => CMPEN_A::_0,
            true => CMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPEN_A::_1
    }
}
impl core::ops::Deref for CMPEN_R {
    type Target = crate::FieldReader<bool, CMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEN` writer - Compensation Mode Enable Bit\\nThis bit enables clock compensation function. When this bit enabled, hardware will alternate between n clock cycles and n-1 clock cycles, where n is the value to be written into the ETURDIV ."]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compensation function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPEN_A::_0)
    }
    #[doc = "Compensation function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ETU Rate Divider\\nThe field indicates the clock rate divider.\\nThe real ETU is ETURDIV + 1.\\nNote: Software can configure this field, but this field must be greater than 0x004."]
    #[inline(always)]
    pub fn eturdiv(&self) -> ETURDIV_R {
        ETURDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Compensation Mode Enable Bit\\nThis bit enables clock compensation function. When this bit enabled, hardware will alternate between n clock cycles and n-1 clock cycles, where n is the value to be written into the ETURDIV ."]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ETU Rate Divider\\nThe field indicates the clock rate divider.\\nThe real ETU is ETURDIV + 1.\\nNote: Software can configure this field, but this field must be greater than 0x004."]
    #[inline(always)]
    pub fn eturdiv(&mut self) -> ETURDIV_W {
        ETURDIV_W { w: self }
    }
    #[doc = "Bit 15 - Compensation Mode Enable Bit\\nThis bit enables clock compensation function. When this bit enabled, hardware will alternate between n clock cycles and n-1 clock cycles, where n is the value to be written into the ETURDIV ."]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC ETU Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_etuctl](index.html) module"]
pub struct SC_ETUCTL_SPEC;
impl crate::RegisterSpec for SC_ETUCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_etuctl::R](R) reader structure"]
impl crate::Readable for SC_ETUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_etuctl::W](W) writer structure"]
impl crate::Writable for SC_ETUCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_ETUCTL to value 0x0173"]
impl crate::Resettable for SC_ETUCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0173
    }
}
