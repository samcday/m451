#[doc = "Register `SYS_SRAM_BISTSTS` reader"]
pub struct R(crate::R<SYS_SRAM_BISTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BISTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_SRAM_BISTSTS_SPEC>> for R {
    fn from(reader: crate::R<SYS_SRAM_BISTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "1st System SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBISTEF0_A {
    #[doc = "0: 1st system SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: 1st system SRAM BIST test fail"]
    _1 = 1,
}
impl From<SRBISTEF0_A> for bool {
    #[inline(always)]
    fn from(variant: SRBISTEF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBISTEF0` reader - 1st System SRAM BIST Fail Flag"]
pub struct SRBISTEF0_R(crate::FieldReader<bool, SRBISTEF0_A>);
impl SRBISTEF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBISTEF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBISTEF0_A {
        match self.bits {
            false => SRBISTEF0_A::_0,
            true => SRBISTEF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBISTEF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBISTEF0_A::_1
    }
}
impl core::ops::Deref for SRBISTEF0_R {
    type Target = crate::FieldReader<bool, SRBISTEF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "2nd System SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBISTEF1_A {
    #[doc = "0: 2nd system SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: 2nd system SRAM BIST test fail"]
    _1 = 1,
}
impl From<SRBISTEF1_A> for bool {
    #[inline(always)]
    fn from(variant: SRBISTEF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBISTEF1` reader - 2nd System SRAM BIST Fail Flag"]
pub struct SRBISTEF1_R(crate::FieldReader<bool, SRBISTEF1_A>);
impl SRBISTEF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBISTEF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBISTEF1_A {
        match self.bits {
            false => SRBISTEF1_A::_0,
            true => SRBISTEF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBISTEF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBISTEF1_A::_1
    }
}
impl core::ops::Deref for SRBISTEF1_R {
    type Target = crate::FieldReader<bool, SRBISTEF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CACHE SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRBISTEF_A {
    #[doc = "0: System CACHE RAM BIST test pass"]
    _0 = 0,
    #[doc = "1: System CACHE RAM BIST test fail"]
    _1 = 1,
}
impl From<CRBISTEF_A> for bool {
    #[inline(always)]
    fn from(variant: CRBISTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBISTEF` reader - CACHE SRAM BIST Fail Flag"]
pub struct CRBISTEF_R(crate::FieldReader<bool, CRBISTEF_A>);
impl CRBISTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRBISTEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRBISTEF_A {
        match self.bits {
            false => CRBISTEF_A::_0,
            true => CRBISTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRBISTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRBISTEF_A::_1
    }
}
impl core::ops::Deref for CRBISTEF_R {
    type Target = crate::FieldReader<bool, CRBISTEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CAN SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANBEF_A {
    #[doc = "0: CAN SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: CAN SRAM BIST test fail"]
    _1 = 1,
}
impl From<CANBEF_A> for bool {
    #[inline(always)]
    fn from(variant: CANBEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CANBEF` reader - CAN SRAM BIST Fail Flag"]
pub struct CANBEF_R(crate::FieldReader<bool, CANBEF_A>);
impl CANBEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CANBEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANBEF_A {
        match self.bits {
            false => CANBEF_A::_0,
            true => CANBEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CANBEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CANBEF_A::_1
    }
}
impl core::ops::Deref for CANBEF_R {
    type Target = crate::FieldReader<bool, CANBEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB SRAM BIST Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBEF_A {
    #[doc = "0: USB SRAM BIST test pass"]
    _0 = 0,
    #[doc = "1: USB SRAM BIST test fail"]
    _1 = 1,
}
impl From<USBBEF_A> for bool {
    #[inline(always)]
    fn from(variant: USBBEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBEF` reader - USB SRAM BIST Fail Flag"]
pub struct USBBEF_R(crate::FieldReader<bool, USBBEF_A>);
impl USBBEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBEF_A {
        match self.bits {
            false => USBBEF_A::_0,
            true => USBBEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBEF_A::_1
    }
}
impl core::ops::Deref for USBBEF_R {
    type Target = crate::FieldReader<bool, USBBEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "1st SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBEND0_A {
    #[doc = "0: 1st system SRAM BIST active"]
    _0 = 0,
    #[doc = "1: 1st system SRAM BIST finish"]
    _1 = 1,
}
impl From<SRBEND0_A> for bool {
    #[inline(always)]
    fn from(variant: SRBEND0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBEND0` reader - 1st SRAM BIST Test Finish"]
pub struct SRBEND0_R(crate::FieldReader<bool, SRBEND0_A>);
impl SRBEND0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBEND0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBEND0_A {
        match self.bits {
            false => SRBEND0_A::_0,
            true => SRBEND0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBEND0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBEND0_A::_1
    }
}
impl core::ops::Deref for SRBEND0_R {
    type Target = crate::FieldReader<bool, SRBEND0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "2nd SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBEND1_A {
    #[doc = "0: 2nd system SRAM BIST is active"]
    _0 = 0,
    #[doc = "1: 2nd system SRAM BIST finish"]
    _1 = 1,
}
impl From<SRBEND1_A> for bool {
    #[inline(always)]
    fn from(variant: SRBEND1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBEND1` reader - 2nd SRAM BIST Test Finish"]
pub struct SRBEND1_R(crate::FieldReader<bool, SRBEND1_A>);
impl SRBEND1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBEND1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBEND1_A {
        match self.bits {
            false => SRBEND1_A::_0,
            true => SRBEND1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBEND1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBEND1_A::_1
    }
}
impl core::ops::Deref for SRBEND1_R {
    type Target = crate::FieldReader<bool, SRBEND1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CACHE SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRBEND_A {
    #[doc = "0: System CACHE RAM BIST is active"]
    _0 = 0,
    #[doc = "1: System CACHE RAM BIST test finish"]
    _1 = 1,
}
impl From<CRBEND_A> for bool {
    #[inline(always)]
    fn from(variant: CRBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBEND` reader - CACHE SRAM BIST Test Finish"]
pub struct CRBEND_R(crate::FieldReader<bool, CRBEND_A>);
impl CRBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRBEND_A {
        match self.bits {
            false => CRBEND_A::_0,
            true => CRBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRBEND_A::_1
    }
}
impl core::ops::Deref for CRBEND_R {
    type Target = crate::FieldReader<bool, CRBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CAN SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANBEND_A {
    #[doc = "0: CAN SRAM BIST is active"]
    _0 = 0,
    #[doc = "1: CAN SRAM BIST test finish"]
    _1 = 1,
}
impl From<CANBEND_A> for bool {
    #[inline(always)]
    fn from(variant: CANBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CANBEND` reader - CAN SRAM BIST Test Finish"]
pub struct CANBEND_R(crate::FieldReader<bool, CANBEND_A>);
impl CANBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CANBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANBEND_A {
        match self.bits {
            false => CANBEND_A::_0,
            true => CANBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CANBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CANBEND_A::_1
    }
}
impl core::ops::Deref for CANBEND_R {
    type Target = crate::FieldReader<bool, CANBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB SRAM BIST Test Finish\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBEND_A {
    #[doc = "0: USB SRAM BIST is active"]
    _0 = 0,
    #[doc = "1: USB SRAM BIST test finish"]
    _1 = 1,
}
impl From<USBBEND_A> for bool {
    #[inline(always)]
    fn from(variant: USBBEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBEND` reader - USB SRAM BIST Test Finish"]
pub struct USBBEND_R(crate::FieldReader<bool, USBBEND_A>);
impl USBBEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBEND_A {
        match self.bits {
            false => USBBEND_A::_0,
            true => USBBEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBEND_A::_1
    }
}
impl core::ops::Deref for USBBEND_R {
    type Target = crate::FieldReader<bool, USBBEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1st System SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn srbistef0(&self) -> SRBISTEF0_R {
        SRBISTEF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 2nd System SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn srbistef1(&self) -> SRBISTEF1_R {
        SRBISTEF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CACHE SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn crbistef(&self) -> CRBISTEF_R {
        CRBISTEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CAN SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn canbef(&self) -> CANBEF_R {
        CANBEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB SRAM BIST Fail Flag"]
    #[inline(always)]
    pub fn usbbef(&self) -> USBBEF_R {
        USBBEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1st SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn srbend0(&self) -> SRBEND0_R {
        SRBEND0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 2nd SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn srbend1(&self) -> SRBEND1_R {
        SRBEND1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CACHE SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn crbend(&self) -> CRBEND_R {
        CRBEND_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CAN SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn canbend(&self) -> CANBEND_R {
        CANBEND_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USB SRAM BIST Test Finish"]
    #[inline(always)]
    pub fn usbbend(&self) -> USBBEND_R {
        USBBEND_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "System SRAM BIST Test Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_biststs](index.html) module"]
pub struct SYS_SRAM_BISTSTS_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BISTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_biststs::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BISTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_SRAM_BISTSTS to value 0"]
impl crate::Resettable for SYS_SRAM_BISTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
