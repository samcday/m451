#[doc = "Register `CRC_SEED` reader"]
pub struct R(crate::R<CRC_SEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_SEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_SEED_SPEC>> for R {
    fn from(reader: crate::R<CRC_SEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_SEED` writer"]
pub struct W(crate::W<CRC_SEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_SEED_SPEC>;
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
impl core::convert::From<crate::W<CRC_SEED_SPEC>> for W {
    fn from(writer: crate::W<CRC_SEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEED` reader - CRC Seed Value\\nThis field indicates the CRC seed value."]
pub struct SEED_R(crate::FieldReader<u32, u32>);
impl SEED_R {
    pub(crate) fn new(bits: u32) -> Self {
        SEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEED` writer - CRC Seed Value\\nThis field indicates the CRC seed value."]
pub struct SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Seed Value\\nThis field indicates the CRC seed value."]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Seed Value\\nThis field indicates the CRC seed value."]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W {
        SEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Seed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_seed](index.html) module"]
pub struct CRC_SEED_SPEC;
impl crate::RegisterSpec for CRC_SEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_seed::R](R) reader structure"]
impl crate::Readable for CRC_SEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_seed::W](W) writer structure"]
impl crate::Writable for CRC_SEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_SEED to value 0xffff_ffff"]
impl crate::Resettable for CRC_SEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
