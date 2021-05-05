#[doc = "Register `SYS_IVSCTL` reader"]
pub struct R(crate::R<SYS_IVSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_IVSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_IVSCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_IVSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_IVSCTL` writer"]
pub struct W(crate::W<SYS_IVSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_IVSCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_IVSCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_IVSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Temperature Sensor Enable Bit\\nThis bit is used to enable/disable temperature sensor function.\\nNote: After this bit is set to 1, the value of temperature sensor output can be obtained from ADC conversion result. Please refer to ADC function chapter for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTEMPEN_A {
    #[doc = "0: Temperature sensor function Disabled (default)"]
    _0 = 0,
    #[doc = "1: Temperature sensor function Enabled"]
    _1 = 1,
}
impl From<VTEMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: VTEMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTEMPEN` reader - Temperature Sensor Enable Bit\\nThis bit is used to enable/disable temperature sensor function.\\nNote: After this bit is set to 1, the value of temperature sensor output can be obtained from ADC conversion result. Please refer to ADC function chapter for details."]
pub struct VTEMPEN_R(crate::FieldReader<bool, VTEMPEN_A>);
impl VTEMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTEMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTEMPEN_A {
        match self.bits {
            false => VTEMPEN_A::_0,
            true => VTEMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VTEMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VTEMPEN_A::_1
    }
}
impl core::ops::Deref for VTEMPEN_R {
    type Target = crate::FieldReader<bool, VTEMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTEMPEN` writer - Temperature Sensor Enable Bit\\nThis bit is used to enable/disable temperature sensor function.\\nNote: After this bit is set to 1, the value of temperature sensor output can be obtained from ADC conversion result. Please refer to ADC function chapter for details."]
pub struct VTEMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VTEMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTEMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Temperature sensor function Disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VTEMPEN_A::_0)
    }
    #[doc = "Temperature sensor function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VTEMPEN_A::_1)
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
#[doc = "VBAT Unity Gain Buffer Enable Bit\\nThis bit is used to enable/disable VBAT unity gain buffer function.\\nNote: After this bit is set to 1, the value of VBAT unity gain buffer output voltage can be obtained from ADC conversion result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATUGEN_A {
    #[doc = "0: VBAT unity gain buffer function Disabled (default)"]
    _0 = 0,
    #[doc = "1: VBAT unity gain buffer function Enabled"]
    _1 = 1,
}
impl From<VBATUGEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATUGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATUGEN` reader - VBAT Unity Gain Buffer Enable Bit\\nThis bit is used to enable/disable VBAT unity gain buffer function.\\nNote: After this bit is set to 1, the value of VBAT unity gain buffer output voltage can be obtained from ADC conversion result"]
pub struct VBATUGEN_R(crate::FieldReader<bool, VBATUGEN_A>);
impl VBATUGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATUGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATUGEN_A {
        match self.bits {
            false => VBATUGEN_A::_0,
            true => VBATUGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBATUGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBATUGEN_A::_1
    }
}
impl core::ops::Deref for VBATUGEN_R {
    type Target = crate::FieldReader<bool, VBATUGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATUGEN` writer - VBAT Unity Gain Buffer Enable Bit\\nThis bit is used to enable/disable VBAT unity gain buffer function.\\nNote: After this bit is set to 1, the value of VBAT unity gain buffer output voltage can be obtained from ADC conversion result"]
pub struct VBATUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATUGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATUGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBAT unity gain buffer function Disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBATUGEN_A::_0)
    }
    #[doc = "VBAT unity gain buffer function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBATUGEN_A::_1)
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
    #[doc = "Bit 0 - Temperature Sensor Enable Bit\\nThis bit is used to enable/disable temperature sensor function.\\nNote: After this bit is set to 1, the value of temperature sensor output can be obtained from ADC conversion result. Please refer to ADC function chapter for details."]
    #[inline(always)]
    pub fn vtempen(&self) -> VTEMPEN_R {
        VTEMPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBAT Unity Gain Buffer Enable Bit\\nThis bit is used to enable/disable VBAT unity gain buffer function.\\nNote: After this bit is set to 1, the value of VBAT unity gain buffer output voltage can be obtained from ADC conversion result"]
    #[inline(always)]
    pub fn vbatugen(&self) -> VBATUGEN_R {
        VBATUGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Enable Bit\\nThis bit is used to enable/disable temperature sensor function.\\nNote: After this bit is set to 1, the value of temperature sensor output can be obtained from ADC conversion result. Please refer to ADC function chapter for details."]
    #[inline(always)]
    pub fn vtempen(&mut self) -> VTEMPEN_W {
        VTEMPEN_W { w: self }
    }
    #[doc = "Bit 1 - VBAT Unity Gain Buffer Enable Bit\\nThis bit is used to enable/disable VBAT unity gain buffer function.\\nNote: After this bit is set to 1, the value of VBAT unity gain buffer output voltage can be obtained from ADC conversion result"]
    #[inline(always)]
    pub fn vbatugen(&mut self) -> VBATUGEN_W {
        VBATUGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Voltage Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ivsctl](index.html) module"]
pub struct SYS_IVSCTL_SPEC;
impl crate::RegisterSpec for SYS_IVSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ivsctl::R](R) reader structure"]
impl crate::Readable for SYS_IVSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ivsctl::W](W) writer structure"]
impl crate::Writable for SYS_IVSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_IVSCTL to value 0"]
impl crate::Resettable for SYS_IVSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
