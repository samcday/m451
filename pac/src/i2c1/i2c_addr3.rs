#[doc = "Register `I2C_ADDR3` reader"]
pub struct R(crate::R<I2C_ADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_ADDR3_SPEC>> for R {
    fn from(reader: crate::R<I2C_ADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ADDR3` writer"]
pub struct W(crate::W<I2C_ADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ADDR3_SPEC>;
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
impl core::convert::From<crate::W<I2C_ADDR3_SPEC>> for W {
    fn from(writer: crate::W<I2C_ADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "General Call Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GC_A {
    #[doc = "0: General Call Function Disabled"]
    _0 = 0,
    #[doc = "1: General Call Function Enabled"]
    _1 = 1,
}
impl From<GC_A> for bool {
    #[inline(always)]
    fn from(variant: GC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC` reader - General Call Function"]
pub struct GC_R(crate::FieldReader<bool, GC_A>);
impl GC_R {
    pub(crate) fn new(bits: bool) -> Self {
        GC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_A {
        match self.bits {
            false => GC_A::_0,
            true => GC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GC_A::_1
    }
}
impl core::ops::Deref for GC_R {
    type Target = crate::FieldReader<bool, GC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC` writer - General Call Function"]
pub struct GC_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "General Call Function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GC_A::_0)
    }
    #[doc = "General Call Function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GC_A::_1)
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
#[doc = "Field `ADDR` reader - I2C Address \\nThe content of this register is irrelevant when I2C is in Master mode. In the slave mode, the seven most significant bits must be loaded with the chip's own address. The I2C hardware will react if either of the address is matched."]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - I2C Address \\nThe content of this register is irrelevant when I2C is in Master mode. In the slave mode, the seven most significant bits must be loaded with the chip's own address. The I2C hardware will react if either of the address is matched."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Call Function"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - I2C Address \\nThe content of this register is irrelevant when I2C is in Master mode. In the slave mode, the seven most significant bits must be loaded with the chip's own address. The I2C hardware will react if either of the address is matched."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Function"]
    #[inline(always)]
    pub fn gc(&mut self) -> GC_W {
        GC_W { w: self }
    }
    #[doc = "Bits 1:7 - I2C Address \\nThe content of this register is irrelevant when I2C is in Master mode. In the slave mode, the seven most significant bits must be loaded with the chip's own address. The I2C hardware will react if either of the address is matched."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_addr3](index.html) module"]
pub struct I2C_ADDR3_SPEC;
impl crate::RegisterSpec for I2C_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_addr3::R](R) reader structure"]
impl crate::Readable for I2C_ADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_addr3::W](W) writer structure"]
impl crate::Writable for I2C_ADDR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ADDR3 to value 0"]
impl crate::Resettable for I2C_ADDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
