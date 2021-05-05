#[doc = "Register `PWM_PDMACTL` reader"]
pub struct R(crate::R<PWM_PDMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_PDMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_PDMACTL_SPEC>> for R {
    fn from(reader: crate::R<PWM_PDMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_PDMACTL` writer"]
pub struct W(crate::W<PWM_PDMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_PDMACTL_SPEC>;
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
impl core::convert::From<crate::W<PWM_PDMACTL_SPEC>> for W {
    fn from(writer: crate::W<PWM_PDMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0/1 PDMA Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_1_A {
    #[doc = "0: Channel 0/1 PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Channel 0/1 PDMA function Enabled for the channel 0/1 captured data and transfer to memory"]
    _1 = 1,
}
impl From<CHEN0_1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0_1` reader - Channel 0/1 PDMA Enable Bit"]
pub struct CHEN0_1_R(crate::FieldReader<bool, CHEN0_1_A>);
impl CHEN0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_1_A {
        match self.bits {
            false => CHEN0_1_A::_0,
            true => CHEN0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN0_1_A::_1
    }
}
impl core::ops::Deref for CHEN0_1_R {
    type Target = crate::FieldReader<bool, CHEN0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN0_1` writer - Channel 0/1 PDMA Enable Bit"]
pub struct CHEN0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 0/1 PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN0_1_A::_0)
    }
    #[doc = "Channel 0/1 PDMA function Enabled for the channel 0/1 captured data and transfer to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN0_1_A::_1)
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
#[doc = "Select PWM_RCAPDAT0/1 or PWM_FCAPDAT0/1 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPMOD0_1_A {
    #[doc = "0: Reserved."]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT0/1"]
    _1 = 1,
    #[doc = "2: PWM_FCAPDAT0/1"]
    _2 = 2,
    #[doc = "3: Both PWM_RCAPDAT0/1 and PWM_FCAPDAT0/1"]
    _3 = 3,
}
impl From<CAPMOD0_1_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPMOD0_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPMOD0_1` reader - Select PWM_RCAPDAT0/1 or PWM_FCAPDAT0/1 to Do PDMA Transfer"]
pub struct CAPMOD0_1_R(crate::FieldReader<u8, CAPMOD0_1_A>);
impl CAPMOD0_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPMOD0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPMOD0_1_A {
        match self.bits {
            0 => CAPMOD0_1_A::_0,
            1 => CAPMOD0_1_A::_1,
            2 => CAPMOD0_1_A::_2,
            3 => CAPMOD0_1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPMOD0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPMOD0_1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CAPMOD0_1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CAPMOD0_1_A::_3
    }
}
impl core::ops::Deref for CAPMOD0_1_R {
    type Target = crate::FieldReader<u8, CAPMOD0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPMOD0_1` writer - Select PWM_RCAPDAT0/1 or PWM_FCAPDAT0/1 to Do PDMA Transfer"]
pub struct CAPMOD0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMOD0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPMOD0_1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPMOD0_1_A::_0)
    }
    #[doc = "PWM_RCAPDAT0/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPMOD0_1_A::_1)
    }
    #[doc = "PWM_FCAPDAT0/1"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CAPMOD0_1_A::_2)
    }
    #[doc = "Both PWM_RCAPDAT0/1 and PWM_FCAPDAT0/1"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CAPMOD0_1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Capture Channel 0/1 Rising/Falling Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPORD0_1_A {
    #[doc = "0: PWM_FCAPDAT0/1 is the first captured data to memory"]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT0/1 is the first captured data to memory"]
    _1 = 1,
}
impl From<CAPORD0_1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPORD0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPORD0_1` reader - Capture Channel 0/1 Rising/Falling Order"]
pub struct CAPORD0_1_R(crate::FieldReader<bool, CAPORD0_1_A>);
impl CAPORD0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPORD0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPORD0_1_A {
        match self.bits {
            false => CAPORD0_1_A::_0,
            true => CAPORD0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPORD0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPORD0_1_A::_1
    }
}
impl core::ops::Deref for CAPORD0_1_R {
    type Target = crate::FieldReader<bool, CAPORD0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPORD0_1` writer - Capture Channel 0/1 Rising/Falling Order"]
pub struct CAPORD0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPORD0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPORD0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_FCAPDAT0/1 is the first captured data to memory"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPORD0_1_A::_0)
    }
    #[doc = "PWM_RCAPDAT0/1 is the first captured data to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPORD0_1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Select Channel 0/1 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL0_1_A {
    #[doc = "0: Channel0"]
    _0 = 0,
    #[doc = "1: Channel1"]
    _1 = 1,
}
impl From<CHSEL0_1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL0_1` reader - Select Channel 0/1 to Do PDMA Transfer"]
pub struct CHSEL0_1_R(crate::FieldReader<bool, CHSEL0_1_A>);
impl CHSEL0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL0_1_A {
        match self.bits {
            false => CHSEL0_1_A::_0,
            true => CHSEL0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHSEL0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHSEL0_1_A::_1
    }
}
impl core::ops::Deref for CHSEL0_1_R {
    type Target = crate::FieldReader<bool, CHSEL0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL0_1` writer - Select Channel 0/1 to Do PDMA Transfer"]
pub struct CHSEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHSEL0_1_A::_0)
    }
    #[doc = "Channel1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHSEL0_1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel 2/3 PDMA Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_3_A {
    #[doc = "0: Channel 2/3 PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Channel 2/3 PDMA function Enabled for the channel 2/3 captured data and transfer to memory"]
    _1 = 1,
}
impl From<CHEN2_3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2_3` reader - Channel 2/3 PDMA Enable Bit"]
pub struct CHEN2_3_R(crate::FieldReader<bool, CHEN2_3_A>);
impl CHEN2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_3_A {
        match self.bits {
            false => CHEN2_3_A::_0,
            true => CHEN2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN2_3_A::_1
    }
}
impl core::ops::Deref for CHEN2_3_R {
    type Target = crate::FieldReader<bool, CHEN2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN2_3` writer - Channel 2/3 PDMA Enable Bit"]
pub struct CHEN2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 2/3 PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN2_3_A::_0)
    }
    #[doc = "Channel 2/3 PDMA function Enabled for the channel 2/3 captured data and transfer to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN2_3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Select PWM_RCAPDAT2/3 or PWM_FCAODAT2/3 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPMOD2_3_A {
    #[doc = "0: Reserved."]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT2/3"]
    _1 = 1,
    #[doc = "2: PWM_FCAPDAT2/3"]
    _2 = 2,
    #[doc = "3: Both PWM_RCAPDAT2/3 and PWM_FCAPDAT2/3"]
    _3 = 3,
}
impl From<CAPMOD2_3_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPMOD2_3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPMOD2_3` reader - Select PWM_RCAPDAT2/3 or PWM_FCAODAT2/3 to Do PDMA Transfer"]
pub struct CAPMOD2_3_R(crate::FieldReader<u8, CAPMOD2_3_A>);
impl CAPMOD2_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPMOD2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPMOD2_3_A {
        match self.bits {
            0 => CAPMOD2_3_A::_0,
            1 => CAPMOD2_3_A::_1,
            2 => CAPMOD2_3_A::_2,
            3 => CAPMOD2_3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPMOD2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPMOD2_3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CAPMOD2_3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CAPMOD2_3_A::_3
    }
}
impl core::ops::Deref for CAPMOD2_3_R {
    type Target = crate::FieldReader<u8, CAPMOD2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPMOD2_3` writer - Select PWM_RCAPDAT2/3 or PWM_FCAODAT2/3 to Do PDMA Transfer"]
pub struct CAPMOD2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMOD2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPMOD2_3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPMOD2_3_A::_0)
    }
    #[doc = "PWM_RCAPDAT2/3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPMOD2_3_A::_1)
    }
    #[doc = "PWM_FCAPDAT2/3"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CAPMOD2_3_A::_2)
    }
    #[doc = "Both PWM_RCAPDAT2/3 and PWM_FCAPDAT2/3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CAPMOD2_3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Capture Channel 2/3 Rising/Falling Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPORD2_3_A {
    #[doc = "0: PWM_FCAPDAT2/3 is the first captured data to memory"]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT2/3 is the first captured data to memory"]
    _1 = 1,
}
impl From<CAPORD2_3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPORD2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPORD2_3` reader - Capture Channel 2/3 Rising/Falling Order"]
pub struct CAPORD2_3_R(crate::FieldReader<bool, CAPORD2_3_A>);
impl CAPORD2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPORD2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPORD2_3_A {
        match self.bits {
            false => CAPORD2_3_A::_0,
            true => CAPORD2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPORD2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPORD2_3_A::_1
    }
}
impl core::ops::Deref for CAPORD2_3_R {
    type Target = crate::FieldReader<bool, CAPORD2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPORD2_3` writer - Capture Channel 2/3 Rising/Falling Order"]
pub struct CAPORD2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPORD2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPORD2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_FCAPDAT2/3 is the first captured data to memory"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPORD2_3_A::_0)
    }
    #[doc = "PWM_RCAPDAT2/3 is the first captured data to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPORD2_3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Select Channel 2/3 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL2_3_A {
    #[doc = "0: Channel2"]
    _0 = 0,
    #[doc = "1: Channel3"]
    _1 = 1,
}
impl From<CHSEL2_3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL2_3` reader - Select Channel 2/3 to Do PDMA Transfer"]
pub struct CHSEL2_3_R(crate::FieldReader<bool, CHSEL2_3_A>);
impl CHSEL2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL2_3_A {
        match self.bits {
            false => CHSEL2_3_A::_0,
            true => CHSEL2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHSEL2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHSEL2_3_A::_1
    }
}
impl core::ops::Deref for CHSEL2_3_R {
    type Target = crate::FieldReader<bool, CHSEL2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL2_3` writer - Select Channel 2/3 to Do PDMA Transfer"]
pub struct CHSEL2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHSEL2_3_A::_0)
    }
    #[doc = "Channel3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHSEL2_3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Channel 4/5 PDMA Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_5_A {
    #[doc = "0: Channel 4/5 PDMA function Disabled"]
    _0 = 0,
    #[doc = "1: Channel 4/5 PDMA function Enabled for the channel 4/5 captured data and transfer to memory"]
    _1 = 1,
}
impl From<CHEN4_5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4_5` reader - Channel 4/5 PDMA Enable Bit"]
pub struct CHEN4_5_R(crate::FieldReader<bool, CHEN4_5_A>);
impl CHEN4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_5_A {
        match self.bits {
            false => CHEN4_5_A::_0,
            true => CHEN4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHEN4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHEN4_5_A::_1
    }
}
impl core::ops::Deref for CHEN4_5_R {
    type Target = crate::FieldReader<bool, CHEN4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN4_5` writer - Channel 4/5 PDMA Enable Bit"]
pub struct CHEN4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 4/5 PDMA function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHEN4_5_A::_0)
    }
    #[doc = "Channel 4/5 PDMA function Enabled for the channel 4/5 captured data and transfer to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHEN4_5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Select PWM_RCAPDAT4/5 or PWM_FCAPDAT4/5 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPMOD4_5_A {
    #[doc = "0: Reserved."]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT4/5"]
    _1 = 1,
    #[doc = "2: PWM_FCAPDAT4/5"]
    _2 = 2,
    #[doc = "3: Both PWM_RCAPDAT4/5 and PWM_FCAPDAT4/5"]
    _3 = 3,
}
impl From<CAPMOD4_5_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPMOD4_5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPMOD4_5` reader - Select PWM_RCAPDAT4/5 or PWM_FCAPDAT4/5 to Do PDMA Transfer"]
pub struct CAPMOD4_5_R(crate::FieldReader<u8, CAPMOD4_5_A>);
impl CAPMOD4_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPMOD4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPMOD4_5_A {
        match self.bits {
            0 => CAPMOD4_5_A::_0,
            1 => CAPMOD4_5_A::_1,
            2 => CAPMOD4_5_A::_2,
            3 => CAPMOD4_5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPMOD4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPMOD4_5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CAPMOD4_5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CAPMOD4_5_A::_3
    }
}
impl core::ops::Deref for CAPMOD4_5_R {
    type Target = crate::FieldReader<u8, CAPMOD4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPMOD4_5` writer - Select PWM_RCAPDAT4/5 or PWM_FCAPDAT4/5 to Do PDMA Transfer"]
pub struct CAPMOD4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMOD4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPMOD4_5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPMOD4_5_A::_0)
    }
    #[doc = "PWM_RCAPDAT4/5"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPMOD4_5_A::_1)
    }
    #[doc = "PWM_FCAPDAT4/5"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CAPMOD4_5_A::_2)
    }
    #[doc = "Both PWM_RCAPDAT4/5 and PWM_FCAPDAT4/5"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CAPMOD4_5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Capture Channel 4/5 Rising/Falling Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPORD4_5_A {
    #[doc = "0: PWM_FCAPDAT4/5 is the first captured data to memory"]
    _0 = 0,
    #[doc = "1: PWM_RCAPDAT4/5 is the first captured data to memory"]
    _1 = 1,
}
impl From<CAPORD4_5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPORD4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPORD4_5` reader - Capture Channel 4/5 Rising/Falling Order"]
pub struct CAPORD4_5_R(crate::FieldReader<bool, CAPORD4_5_A>);
impl CAPORD4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPORD4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPORD4_5_A {
        match self.bits {
            false => CAPORD4_5_A::_0,
            true => CAPORD4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CAPORD4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CAPORD4_5_A::_1
    }
}
impl core::ops::Deref for CAPORD4_5_R {
    type Target = crate::FieldReader<bool, CAPORD4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPORD4_5` writer - Capture Channel 4/5 Rising/Falling Order"]
pub struct CAPORD4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPORD4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPORD4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PWM_FCAPDAT4/5 is the first captured data to memory"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPORD4_5_A::_0)
    }
    #[doc = "PWM_RCAPDAT4/5 is the first captured data to memory"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPORD4_5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select Channel 4/5 to Do PDMA Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL4_5_A {
    #[doc = "0: Channel4"]
    _0 = 0,
    #[doc = "1: Channel5"]
    _1 = 1,
}
impl From<CHSEL4_5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL4_5` reader - Select Channel 4/5 to Do PDMA Transfer"]
pub struct CHSEL4_5_R(crate::FieldReader<bool, CHSEL4_5_A>);
impl CHSEL4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL4_5_A {
        match self.bits {
            false => CHSEL4_5_A::_0,
            true => CHSEL4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHSEL4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHSEL4_5_A::_1
    }
}
impl core::ops::Deref for CHSEL4_5_R {
    type Target = crate::FieldReader<bool, CHSEL4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL4_5` writer - Select Channel 4/5 to Do PDMA Transfer"]
pub struct CHSEL4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHSEL4_5_A::_0)
    }
    #[doc = "Channel5"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHSEL4_5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0/1 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen0_1(&self) -> CHEN0_1_R {
        CHEN0_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Select PWM_RCAPDAT0/1 or PWM_FCAPDAT0/1 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod0_1(&self) -> CAPMOD0_1_R {
        CAPMOD0_1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Capture Channel 0/1 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord0_1(&self) -> CAPORD0_1_R {
        CAPORD0_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select Channel 0/1 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel0_1(&self) -> CHSEL0_1_R {
        CHSEL0_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 2/3 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen2_3(&self) -> CHEN2_3_R {
        CHEN2_3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Select PWM_RCAPDAT2/3 or PWM_FCAODAT2/3 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod2_3(&self) -> CAPMOD2_3_R {
        CAPMOD2_3_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Capture Channel 2/3 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord2_3(&self) -> CAPORD2_3_R {
        CAPORD2_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select Channel 2/3 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel2_3(&self) -> CHSEL2_3_R {
        CHSEL2_3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 4/5 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen4_5(&self) -> CHEN4_5_R {
        CHEN4_5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Select PWM_RCAPDAT4/5 or PWM_FCAPDAT4/5 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod4_5(&self) -> CAPMOD4_5_R {
        CAPMOD4_5_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Capture Channel 4/5 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord4_5(&self) -> CAPORD4_5_R {
        CAPORD4_5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select Channel 4/5 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel4_5(&self) -> CHSEL4_5_R {
        CHSEL4_5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0/1 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen0_1(&mut self) -> CHEN0_1_W {
        CHEN0_1_W { w: self }
    }
    #[doc = "Bits 1:2 - Select PWM_RCAPDAT0/1 or PWM_FCAPDAT0/1 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod0_1(&mut self) -> CAPMOD0_1_W {
        CAPMOD0_1_W { w: self }
    }
    #[doc = "Bit 3 - Capture Channel 0/1 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord0_1(&mut self) -> CAPORD0_1_W {
        CAPORD0_1_W { w: self }
    }
    #[doc = "Bit 4 - Select Channel 0/1 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel0_1(&mut self) -> CHSEL0_1_W {
        CHSEL0_1_W { w: self }
    }
    #[doc = "Bit 8 - Channel 2/3 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen2_3(&mut self) -> CHEN2_3_W {
        CHEN2_3_W { w: self }
    }
    #[doc = "Bits 9:10 - Select PWM_RCAPDAT2/3 or PWM_FCAODAT2/3 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod2_3(&mut self) -> CAPMOD2_3_W {
        CAPMOD2_3_W { w: self }
    }
    #[doc = "Bit 11 - Capture Channel 2/3 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord2_3(&mut self) -> CAPORD2_3_W {
        CAPORD2_3_W { w: self }
    }
    #[doc = "Bit 12 - Select Channel 2/3 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel2_3(&mut self) -> CHSEL2_3_W {
        CHSEL2_3_W { w: self }
    }
    #[doc = "Bit 16 - Channel 4/5 PDMA Enable Bit"]
    #[inline(always)]
    pub fn chen4_5(&mut self) -> CHEN4_5_W {
        CHEN4_5_W { w: self }
    }
    #[doc = "Bits 17:18 - Select PWM_RCAPDAT4/5 or PWM_FCAPDAT4/5 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn capmod4_5(&mut self) -> CAPMOD4_5_W {
        CAPMOD4_5_W { w: self }
    }
    #[doc = "Bit 19 - Capture Channel 4/5 Rising/Falling Order"]
    #[inline(always)]
    pub fn capord4_5(&mut self) -> CAPORD4_5_W {
        CAPORD4_5_W { w: self }
    }
    #[doc = "Bit 20 - Select Channel 4/5 to Do PDMA Transfer"]
    #[inline(always)]
    pub fn chsel4_5(&mut self) -> CHSEL4_5_W {
        CHSEL4_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM PDMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_pdmactl](index.html) module"]
pub struct PWM_PDMACTL_SPEC;
impl crate::RegisterSpec for PWM_PDMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_pdmactl::R](R) reader structure"]
impl crate::Readable for PWM_PDMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_pdmactl::W](W) writer structure"]
impl crate::Writable for PWM_PDMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_PDMACTL to value 0"]
impl crate::Resettable for PWM_PDMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
