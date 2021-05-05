#[doc = "Register `CAN_IF1_DAT_A2` reader"]
pub struct R(crate::R<CAN_IF1_DAT_A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF1_DAT_A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF1_DAT_A2_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF1_DAT_A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF1_DAT_A2` writer"]
pub struct W(crate::W<CAN_IF1_DAT_A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF1_DAT_A2_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF1_DAT_A2_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF1_DAT_A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Data_2` reader - Data Byte 2\\n3rd data byte of CAN Data Frame"]
pub struct DATA_2_R(crate::FieldReader<u8, u8>);
impl DATA_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_2` writer - Data Byte 2\\n3rd data byte of CAN Data Frame"]
pub struct DATA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Data_3` reader - Data Byte 3\\n4th data byte of CAN Data Frame"]
pub struct DATA_3_R(crate::FieldReader<u8, u8>);
impl DATA_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Data_3` writer - Data Byte 3\\n4th data byte of CAN Data Frame"]
pub struct DATA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Byte 2\\n3rd data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_2(&self) -> DATA_2_R {
        DATA_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 3\\n4th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_3(&self) -> DATA_3_R {
        DATA_3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 2\\n3rd data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_2(&mut self) -> DATA_2_W {
        DATA_2_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Byte 3\\n4th data byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data_3(&mut self) -> DATA_3_W {
        DATA_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Data A2 Registers (Register Map Note 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_dat_a2](index.html) module"]
pub struct CAN_IF1_DAT_A2_SPEC;
impl crate::RegisterSpec for CAN_IF1_DAT_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if1_dat_a2::R](R) reader structure"]
impl crate::Readable for CAN_IF1_DAT_A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if1_dat_a2::W](W) writer structure"]
impl crate::Writable for CAN_IF1_DAT_A2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF1_DAT_A2 to value 0"]
impl crate::Resettable for CAN_IF1_DAT_A2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
