#[doc = "Register `OTG_STATUS` reader"]
pub struct R(crate::R<OTG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OTG_STATUS_SPEC>> for R {
    fn from(reader: crate::R<OTG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "over Current Condition\\nThe voltage on VBUS cannot reach a minimum VBUS valid threshold, 4.4V minimum, within a maximum time of 100ms after OTG A-device drives VBUS high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERCUR_A {
    #[doc = "0: OTG A-device drives VBUS successfully"]
    _0 = 0,
    #[doc = "1: OTG A-device cannot drives VBUS high in this interval"]
    _1 = 1,
}
impl From<OVERCUR_A> for bool {
    #[inline(always)]
    fn from(variant: OVERCUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERCUR` reader - over Current Condition\\nThe voltage on VBUS cannot reach a minimum VBUS valid threshold, 4.4V minimum, within a maximum time of 100ms after OTG A-device drives VBUS high."]
pub struct OVERCUR_R(crate::FieldReader<bool, OVERCUR_A>);
impl OVERCUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERCUR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERCUR_A {
        match self.bits {
            false => OVERCUR_A::_0,
            true => OVERCUR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OVERCUR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVERCUR_A::_1
    }
}
impl core::ops::Deref for OVERCUR_R {
    type Target = crate::FieldReader<bool, OVERCUR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB_ID Pin State of Mini-b/Micro-plug\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDSTS_A {
    #[doc = "0: Mini-A/Micro-A plug is attached"]
    _0 = 0,
    #[doc = "1: Mini-B/Micro-B plug is attached"]
    _1 = 1,
}
impl From<IDSTS_A> for bool {
    #[inline(always)]
    fn from(variant: IDSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDSTS` reader - USB_ID Pin State of Mini-b/Micro-plug"]
pub struct IDSTS_R(crate::FieldReader<bool, IDSTS_A>);
impl IDSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDSTS_A {
        match self.bits {
            false => IDSTS_A::_0,
            true => IDSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDSTS_A::_1
    }
}
impl core::ops::Deref for IDSTS_R {
    type Target = crate::FieldReader<bool, IDSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Session End Status\\nWhen VBUS voltage is lower than 0.4V, this bit will be set to 1. Session end means no meaningful power on VBUS.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESSEND_A {
    #[doc = "0: Session is not end"]
    _0 = 0,
    #[doc = "1: Session is end"]
    _1 = 1,
}
impl From<SESSEND_A> for bool {
    #[inline(always)]
    fn from(variant: SESSEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SESSEND` reader - Session End Status\\nWhen VBUS voltage is lower than 0.4V, this bit will be set to 1. Session end means no meaningful power on VBUS."]
pub struct SESSEND_R(crate::FieldReader<bool, SESSEND_A>);
impl SESSEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESSEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSEND_A {
        match self.bits {
            false => SESSEND_A::_0,
            true => SESSEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SESSEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SESSEND_A::_1
    }
}
impl core::ops::Deref for SESSEND_R {
    type Target = crate::FieldReader<bool, SESSEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "B-device Session Valid Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVLD_A {
    #[doc = "0: B-device session is not valid"]
    _0 = 0,
    #[doc = "1: B-device session is valid"]
    _1 = 1,
}
impl From<BVLD_A> for bool {
    #[inline(always)]
    fn from(variant: BVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BVLD` reader - B-device Session Valid Status"]
pub struct BVLD_R(crate::FieldReader<bool, BVLD_A>);
impl BVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVLD_A {
        match self.bits {
            false => BVLD_A::_0,
            true => BVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BVLD_A::_1
    }
}
impl core::ops::Deref for BVLD_R {
    type Target = crate::FieldReader<bool, BVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A-device Session Valid Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLD_A {
    #[doc = "0: A-device session is not valid"]
    _0 = 0,
    #[doc = "1: A-device session is valid"]
    _1 = 1,
}
impl From<AVLD_A> for bool {
    #[inline(always)]
    fn from(variant: AVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLD` reader - A-device Session Valid Status"]
pub struct AVLD_R(crate::FieldReader<bool, AVLD_A>);
impl AVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLD_A {
        match self.bits {
            false => AVLD_A::_0,
            true => AVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVLD_A::_1
    }
}
impl core::ops::Deref for AVLD_R {
    type Target = crate::FieldReader<bool, AVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "VBUS Valid Status\\nWhen VBUS is larger than 4.7V, this bit will be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVLD_A {
    #[doc = "0: VBUS is not valid"]
    _0 = 0,
    #[doc = "1: VBUS is valid"]
    _1 = 1,
}
impl From<VBUSVLD_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVLD` reader - VBUS Valid Status\\nWhen VBUS is larger than 4.7V, this bit will be set to 1."]
pub struct VBUSVLD_R(crate::FieldReader<bool, VBUSVLD_A>);
impl VBUSVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVLD_A {
        match self.bits {
            false => VBUSVLD_A::_0,
            true => VBUSVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VBUSVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VBUSVLD_A::_1
    }
}
impl core::ops::Deref for VBUSVLD_R {
    type Target = crate::FieldReader<bool, VBUSVLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - over Current Condition\\nThe voltage on VBUS cannot reach a minimum VBUS valid threshold, 4.4V minimum, within a maximum time of 100ms after OTG A-device drives VBUS high."]
    #[inline(always)]
    pub fn overcur(&self) -> OVERCUR_R {
        OVERCUR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB_ID Pin State of Mini-b/Micro-plug"]
    #[inline(always)]
    pub fn idsts(&self) -> IDSTS_R {
        IDSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Session End Status\\nWhen VBUS voltage is lower than 0.4V, this bit will be set to 1. Session end means no meaningful power on VBUS."]
    #[inline(always)]
    pub fn sessend(&self) -> SESSEND_R {
        SESSEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B-device Session Valid Status"]
    #[inline(always)]
    pub fn bvld(&self) -> BVLD_R {
        BVLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A-device Session Valid Status"]
    #[inline(always)]
    pub fn avld(&self) -> AVLD_R {
        AVLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VBUS Valid Status\\nWhen VBUS is larger than 4.7V, this bit will be set to 1."]
    #[inline(always)]
    pub fn vbusvld(&self) -> VBUSVLD_R {
        VBUSVLD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "OTG Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_status](index.html) module"]
pub struct OTG_STATUS_SPEC;
impl crate::RegisterSpec for OTG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_status::R](R) reader structure"]
impl crate::Readable for OTG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_STATUS to value 0x06"]
impl crate::Resettable for OTG_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
