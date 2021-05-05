#[doc = "Register `VBUSDET` reader"]
pub struct R(crate::R<VBUSDET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBUSDET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VBUSDET_SPEC>> for R {
    fn from(reader: crate::R<VBUSDET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device VBUS Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSDET_A {
    #[doc = "0: Controller is not attached to the USB host"]
    _0 = 0,
    #[doc = "1: Controller is attached to the USB host"]
    _1 = 1,
}
impl From<VBUSDET_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSDET` reader - Device VBUS Detection"]
pub struct VBUSDET_R(crate::FieldReader<bool, VBUSDET_A>);
impl VBUSDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSDET_A {
        match self.bits {
            false => VBUSDET_A::_0,
            true => VBUSDET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBUSDET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBUSDET_A::_1
    }
}
impl core::ops::Deref for VBUSDET_R {
    type Target = crate::FieldReader<bool, VBUSDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Device VBUS Detection"]
    #[inline(always)]
    pub fn vbusdet(&self) -> VBUSDET_R {
        VBUSDET_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "USB Device VBUS Detection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbusdet](index.html) module"]
pub struct VBUSDET_SPEC;
impl crate::RegisterSpec for VBUSDET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vbusdet::R](R) reader structure"]
impl crate::Readable for VBUSDET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VBUSDET to value 0"]
impl crate::Resettable for VBUSDET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
