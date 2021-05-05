#[doc = "Register `CAN_IF2_DAT_B1` reader"]
pub struct R(crate::R<CAN_IF2_DAT_B1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_DAT_B1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_DAT_B1_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_DAT_B1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_DAT_B1` writer"]
pub struct W(crate::W<CAN_IF2_DAT_B1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_DAT_B1_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_DAT_B1_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_DAT_B1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Data_4` reader - Data Byte 4\\n5th data byte of CAN Data Frame"]
pub struct DATA_4_R(crate::FieldReader<u8, u8>);
impl DATA_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_4` writer - Data Byte 4\\n5th data byte of CAN Data Frame"]
pub struct DATA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Data_5` reader - Data Byte 5\\n6th data byte of CAN Data Frame"]
pub struct DATA_5_R(crate::FieldReader<u8, u8>);
impl DATA_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_5` writer - Data Byte 5\\n6th data byte of CAN Data Frame"]
pub struct DATA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Byte 4\\n5th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_4(&self) -> DATA_4_R {
        DATA_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 5\\n6th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_5(&self) -> DATA_5_R {
        DATA_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 4\\n5th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_4(&mut self) -> DATA_4_W {
        DATA_4_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Byte 5\\n6th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_5(&mut self) -> DATA_5_W {
        DATA_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Data B1 Registers (Register Map Note 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_dat_b1](index.html) module"]
pub struct CAN_IF2_DAT_B1_SPEC;
impl crate::RegisterSpec for CAN_IF2_DAT_B1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_dat_b1::R](R) reader structure"]
impl crate::Readable for CAN_IF2_DAT_B1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_dat_b1::W](W) writer structure"]
impl crate::Writable for CAN_IF2_DAT_B1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_DAT_B1 to value 0"]
impl crate::Resettable for CAN_IF2_DAT_B1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
