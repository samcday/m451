#[doc = "Register `I2C_TOCTL` reader"]
pub struct R(crate::R<I2C_TOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_TOCTL_SPEC>> for R {
    fn from(reader: crate::R<I2C_TOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TOCTL` writer"]
pub struct W(crate::W<I2C_TOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TOCTL_SPEC>;
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
impl core::convert::From<crate::W<I2C_TOCTL_SPEC>> for W {
    fn from(writer: crate::W<I2C_TOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOIF` reader - Time-out Flag\\nThis bit is set by hardware when I2C time-out happened and it can interrupt CPU if I2C interrupt enable bit (INTEN) is set to 1.\\nNote: Software can write 1 to clear this bit."]
pub struct TOIF_R(crate::FieldReader<bool, bool>);
impl TOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIF` writer - Time-out Flag\\nThis bit is set by hardware when I2C time-out happened and it can interrupt CPU if I2C interrupt enable bit (INTEN) is set to 1.\\nNote: Software can write 1 to clear this bit."]
pub struct TOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIF_W<'a> {
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
#[doc = "Time-out Counter Input Clock Divided by 4\\nWhen Enabled, The time-out period is extend 4 times.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOCDIV4_A {
    #[doc = "0: Time-Out Counter Input Clock Divided Disabled"]
    _0 = 0,
    #[doc = "1: Time-Out Counter Input Clock Divided Enabled"]
    _1 = 1,
}
impl From<TOCDIV4_A> for bool {
    #[inline(always)]
    fn from(variant: TOCDIV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOCDIV4` reader - Time-out Counter Input Clock Divided by 4\\nWhen Enabled, The time-out period is extend 4 times."]
pub struct TOCDIV4_R(crate::FieldReader<bool, TOCDIV4_A>);
impl TOCDIV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOCDIV4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOCDIV4_A {
        match self.bits {
            false => TOCDIV4_A::_0,
            true => TOCDIV4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOCDIV4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOCDIV4_A::_1
    }
}
impl core::ops::Deref for TOCDIV4_R {
    type Target = crate::FieldReader<bool, TOCDIV4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOCDIV4` writer - Time-out Counter Input Clock Divided by 4\\nWhen Enabled, The time-out period is extend 4 times."]
pub struct TOCDIV4_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCDIV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOCDIV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time-Out Counter Input Clock Divided Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOCDIV4_A::_0)
    }
    #[doc = "Time-Out Counter Input Clock Divided Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOCDIV4_A::_1)
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
#[doc = "Time-out Counter Enable Bit\\nWhen Enabled, the 14-bit time-out counter will start counting when SI is clear. Setting flag SI to '1' will reset counter and re-start up counting after SI is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOCEN_A {
    #[doc = "0: Time-Out Counter Disabled"]
    _0 = 0,
    #[doc = "1: Time-Out Counter Enabled"]
    _1 = 1,
}
impl From<TOCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOCEN` reader - Time-out Counter Enable Bit\\nWhen Enabled, the 14-bit time-out counter will start counting when SI is clear. Setting flag SI to '1' will reset counter and re-start up counting after SI is cleared."]
pub struct TOCEN_R(crate::FieldReader<bool, TOCEN_A>);
impl TOCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOCEN_A {
        match self.bits {
            false => TOCEN_A::_0,
            true => TOCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOCEN_A::_1
    }
}
impl core::ops::Deref for TOCEN_R {
    type Target = crate::FieldReader<bool, TOCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOCEN` writer - Time-out Counter Enable Bit\\nWhen Enabled, the 14-bit time-out counter will start counting when SI is clear. Setting flag SI to '1' will reset counter and re-start up counting after SI is cleared."]
pub struct TOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time-Out Counter Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOCEN_A::_0)
    }
    #[doc = "Time-Out Counter Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOCEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Time-out Flag\\nThis bit is set by hardware when I2C time-out happened and it can interrupt CPU if I2C interrupt enable bit (INTEN) is set to 1.\\nNote: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn toif(&self) -> TOIF_R {
        TOIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time-out Counter Input Clock Divided by 4\\nWhen Enabled, The time-out period is extend 4 times."]
    #[inline(always)]
    pub fn tocdiv4(&self) -> TOCDIV4_R {
        TOCDIV4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-out Counter Enable Bit\\nWhen Enabled, the 14-bit time-out counter will start counting when SI is clear. Setting flag SI to '1' will reset counter and re-start up counting after SI is cleared."]
    #[inline(always)]
    pub fn tocen(&self) -> TOCEN_R {
        TOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time-out Flag\\nThis bit is set by hardware when I2C time-out happened and it can interrupt CPU if I2C interrupt enable bit (INTEN) is set to 1.\\nNote: Software can write 1 to clear this bit."]
    #[inline(always)]
    pub fn toif(&mut self) -> TOIF_W {
        TOIF_W { w: self }
    }
    #[doc = "Bit 1 - Time-out Counter Input Clock Divided by 4\\nWhen Enabled, The time-out period is extend 4 times."]
    #[inline(always)]
    pub fn tocdiv4(&mut self) -> TOCDIV4_W {
        TOCDIV4_W { w: self }
    }
    #[doc = "Bit 2 - Time-out Counter Enable Bit\\nWhen Enabled, the 14-bit time-out counter will start counting when SI is clear. Setting flag SI to '1' will reset counter and re-start up counting after SI is cleared."]
    #[inline(always)]
    pub fn tocen(&mut self) -> TOCEN_W {
        TOCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Time-out Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_toctl](index.html) module"]
pub struct I2C_TOCTL_SPEC;
impl crate::RegisterSpec for I2C_TOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_toctl::R](R) reader structure"]
impl crate::Readable for I2C_TOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_toctl::W](W) writer structure"]
impl crate::Writable for I2C_TOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TOCTL to value 0"]
impl crate::Resettable for I2C_TOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
