#[doc = "Register `SYS_SRAM_STATUS` reader"]
pub struct R(crate::R<SYS_SRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_SRAM_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SYS_SRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SRAM Parity Check Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERRIF_A {
    #[doc = "0: No System SRAM parity error"]
    _0 = 0,
    #[doc = "1: System SRAM parity error occur"]
    _1 = 1,
}
impl From<PERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIF` reader - SRAM Parity Check Error Flag"]
pub struct PERRIF_R(crate::FieldReader<bool, PERRIF_A>);
impl PERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERRIF_A {
        match self.bits {
            false => PERRIF_A::_0,
            true => PERRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PERRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PERRIF_A::_1
    }
}
impl core::ops::Deref for PERRIF_R {
    type Target = crate::FieldReader<bool, PERRIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SRAM Parity Check Error Flag"]
    #[inline(always)]
    pub fn perrif(&self) -> PERRIF_R {
        PERRIF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "System SRAM Parity Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_status](index.html) module"]
pub struct SYS_SRAM_STATUS_SPEC;
impl crate::RegisterSpec for SYS_SRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_status::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_SRAM_STATUS to value 0"]
impl crate::Resettable for SYS_SRAM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
