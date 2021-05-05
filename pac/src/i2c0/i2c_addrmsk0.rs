#[doc = "Register `I2C_ADDRMSK0` reader"]
pub struct R(crate::R<I2C_ADDRMSK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ADDRMSK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_ADDRMSK0_SPEC>> for R {
    fn from(reader: crate::R<I2C_ADDRMSK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ADDRMSK0` writer"]
pub struct W(crate::W<I2C_ADDRMSK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ADDRMSK0_SPEC>;
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
impl core::convert::From<crate::W<I2C_ADDRMSK0_SPEC>> for W {
    fn from(writer: crate::W<I2C_ADDRMSK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C Address Mask\\nI2C bus controllers support multiple address recognition with four address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDRMSK_A {
    #[doc = "0: Mask Disabled (the received corresponding register bit should be exact the same as address register.)"]
    _0 = 0,
    #[doc = "1: Mask Enabled (the received corresponding address bit is don't care.)"]
    _1 = 1,
}
impl From<ADDRMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDRMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDRMSK` reader - I2C Address Mask\\nI2C bus controllers support multiple address recognition with four address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
pub struct ADDRMSK_R(crate::FieldReader<u8, ADDRMSK_A>);
impl ADDRMSK_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDRMSK_A> {
        match self.bits {
            0 => Some(ADDRMSK_A::_0),
            1 => Some(ADDRMSK_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDRMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDRMSK_A::_1
    }
}
impl core::ops::Deref for ADDRMSK_R {
    type Target = crate::FieldReader<u8, ADDRMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMSK` writer - I2C Address Mask\\nI2C bus controllers support multiple address recognition with four address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
pub struct ADDRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRMSK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mask Disabled (the received corresponding register bit should be exact the same as address register.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDRMSK_A::_0)
    }
    #[doc = "Mask Enabled (the received corresponding address bit is don't care.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDRMSK_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - I2C Address Mask\\nI2C bus controllers support multiple address recognition with four address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn addrmsk(&self) -> ADDRMSK_R {
        ADDRMSK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - I2C Address Mask\\nI2C bus controllers support multiple address recognition with four address mask register. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn addrmsk(&mut self) -> ADDRMSK_W {
        ADDRMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Mask Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_addrmsk0](index.html) module"]
pub struct I2C_ADDRMSK0_SPEC;
impl crate::RegisterSpec for I2C_ADDRMSK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_addrmsk0::R](R) reader structure"]
impl crate::Readable for I2C_ADDRMSK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_addrmsk0::W](W) writer structure"]
impl crate::Writable for I2C_ADDRMSK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ADDRMSK0 to value 0"]
impl crate::Resettable for I2C_ADDRMSK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
